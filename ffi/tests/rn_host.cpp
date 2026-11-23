// A host for React Native's binding off a device: Hermes, as React Native runs it, with
// the JSI module installed as `globalThis.__rust2_<prefix>`, an event loop that runs what
// the module posts and `setTimeout`'s timers, `performance.now` as React Native's, and
// `print`, `process`, and `TextDecoder` stand-ins, `process.cpuUsage` among them as Node's.
// It runs one bundled script, exiting with its `process.exit` code, or 1 if it throws or
// leaves a promise rejected, then destroys the runtime, as a reload does. With `--reload`
// first, it runs the script again in a new runtime once the first exits cleanly.
//
// Build it with the module's `cpp/<prefix>_jsi.cpp`, defining RUST2_PREFIX_NS as its
// namespace, RUST2_GLOBAL as its global, and RUST2_HEADER as its header's name.

#include <chrono>
#include <condition_variable>
#include <cstdio>
#include <fstream>
#include <functional>
#include <map>
#include <memory>
#include <mutex>
#include <sstream>
#include <string>
#include <vector>

#include <sys/resource.h>

#include <hermes/hermes.h>
#include <jsi/jsi.h>

#include RUST2_HEADER

namespace jsi = facebook::jsi;
using job_t = std::function<void(jsi::Runtime &)>;

namespace {

std::mutex mutex;
std::condition_variable cv;
std::vector<job_t> jobs;
// Whether the runtime that jobs run in still takes them: its teardown drops the rest, as
// React Native's does its `CallInvoker`'s.
bool open = false;

std::multimap<std::chrono::steady_clock::time_point, std::pair<double, std::shared_ptr<jsi::Function>>> timers;
double next_timer = 1;

int exit_code = -1;

void post(job_t job) {
	std::unique_lock<std::mutex> lock(mutex);
	if (!open) {
		// Dropped outside the lock, as the job may hold what posts another.
		lock.unlock();
		return;
	}
	jobs.push_back(std::move(job));
	cv.notify_one();
}

void fn(jsi::Runtime &rt, jsi::Object &on, const char *name, unsigned argc, jsi::HostFunctionType f) {
	on.setProperty(rt, name, jsi::Function::createFromHostFunction(rt, jsi::PropNameID::forAscii(rt, name), argc, std::move(f)));
}

const char *prelude = R"(
globalThis.process = {
	argv: ["hermes", "api.js", ...__argv],
	env: {},
	exit(code) { __exit(code); throw new Error("__exited"); },
	stderr: { write(text) { __eprint(text); return true; } },
	cpuUsage: __cpu,
};
globalThis.performance = { now: __now };
globalThis.console = { log: (...a) => print(a.join(" ")), error: (...a) => __eprint(a.join(" ") + "\n") };
// UTF-8 as WHATWG's decoder reads it, replacing what is not.
globalThis.TextDecoder = class TextDecoder {
	decode(bytes) { return __decode(bytes === undefined ? new Uint8Array(0) : new Uint8Array(bytes.buffer ?? bytes, bytes.byteOffset ?? 0, bytes.byteLength)); }
};
globalThis.__rejected = [];
)";

// Run `src` in a new runtime with `args` as its arguments, then destroy the runtime, with
// what it posted and its timers first: the exit code of the script.
int run(const std::string &name, const std::string &src, const std::vector<std::string> &args) {
	exit_code = -1;
	next_timer = 1;
	{
		std::lock_guard<std::mutex> lock(mutex);
		open = true;
	}
	auto config = hermes::vm::RuntimeConfig::Builder().withMicrotaskQueue(true).build();
	std::unique_ptr<facebook::hermes::HermesRuntime> runtime = facebook::hermes::makeHermesRuntime(config);
	{
		jsi::Runtime &rt = *runtime;
		jsi::Object global = rt.global();

		jsi::Array argv(rt, args.size());
		for (size_t i = 0; i < args.size(); i++) {
			argv.setValueAtIndex(rt, i, jsi::String::createFromUtf8(rt, args[i]));
		}
		global.setProperty(rt, "__argv", argv);
		fn(rt, global, "print", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, size_t n) {
			std::string s = n > 0 ? a[0].toString(rt).utf8(rt) : "";
			std::fwrite(s.data(), 1, s.size(), stdout);
			std::fputc('\n', stdout);
			std::fflush(stdout);
			return jsi::Value::undefined();
		});
		fn(rt, global, "__eprint", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, size_t n) {
			std::string s = n > 0 ? a[0].toString(rt).utf8(rt) : "";
			std::fwrite(s.data(), 1, s.size(), stderr);
			return jsi::Value::undefined();
		});
		fn(rt, global, "__exit", 1, [](jsi::Runtime &, const jsi::Value &, const jsi::Value *a, size_t n) {
			if (exit_code < 0) {
				exit_code = n > 0 && a[0].isNumber() ? static_cast<int>(a[0].getNumber()) : 0;
			}
			return jsi::Value::undefined();
		});
		fn(rt, global, "__now", 0, [](jsi::Runtime &, const jsi::Value &, const jsi::Value *, size_t) {
			auto since = std::chrono::steady_clock::now().time_since_epoch();
			return jsi::Value(std::chrono::duration<double, std::milli>(since).count());
		});
		// The process's CPU time in microseconds, every thread's, as Node's `process.cpuUsage`.
		fn(rt, global, "__cpu", 0, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *, size_t) {
			rusage usage{};
			getrusage(RUSAGE_SELF, &usage);
			auto micros = [](const timeval &t) { return static_cast<double>(t.tv_sec) * 1e6 + static_cast<double>(t.tv_usec); };
			jsi::Object out(rt);
			out.setProperty(rt, "user", micros(usage.ru_utime));
			out.setProperty(rt, "system", micros(usage.ru_stime));
			return out;
		});
		fn(rt, global, "setTimeout", 2, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, size_t n) {
			auto f = std::make_shared<jsi::Function>(a[0].getObject(rt).getFunction(rt));
			double ms = n > 1 && a[1].isNumber() ? a[1].getNumber() : 0;
			double id = next_timer++;
			timers.emplace(std::chrono::steady_clock::now() + std::chrono::microseconds(static_cast<long long>(ms * 1000)),
			               std::make_pair(id, f));
			return jsi::Value(id);
		});
		fn(rt, global, "clearTimeout", 1, [](jsi::Runtime &, const jsi::Value &, const jsi::Value *a, size_t n) {
			for (auto it = timers.begin(); n > 0 && it != timers.end(); ++it) {
				if (a[0].isNumber() && it->second.first == a[0].getNumber()) {
					timers.erase(it);
					break;
				}
			}
			return jsi::Value::undefined();
		});
		global.setProperty(rt, RUST2_GLOBAL, RUST2_PREFIX_NS::installer(rt, post));
		// Decoding through the module's own lossy decoder, which `rt.ts` installs.
		fn(rt, global, "__decode", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, size_t) {
			jsi::ArrayBuffer ab = a[0].getObject(rt).getPropertyAsObject(rt, "buffer").getArrayBuffer(rt);
			auto off = static_cast<size_t>(a[0].getObject(rt).getProperty(rt, "byteOffset").getNumber());
			auto len = static_cast<size_t>(a[0].getObject(rt).getProperty(rt, "byteLength").getNumber());
			// Lossless for the UTF-8 a test decodes.
			return jsi::Value(jsi::String::createFromUtf8(rt, ab.data(rt) + off, len));
		});

		auto guard = [&](const std::function<void()> &f) {
			try {
				f();
			} catch (const jsi::JSError &e) {
				if (e.getMessage() != "__exited") {
					std::fprintf(stderr, "uncaught: %s\n", e.getStack().c_str());
					if (exit_code < 0) {
						exit_code = 1;
					}
				}
			}
		};
		guard([&] { rt.evaluateJavaScript(std::make_shared<jsi::StringBuffer>(prelude), "prelude.js"); });
		guard([&] { rt.evaluateJavaScript(std::make_shared<jsi::StringBuffer>(src), name); });
		guard([&] { rt.drainMicrotasks(); });
		// The event loop: what the module posted, then timers due, until nothing is left or the
		// script exits.
		while (exit_code < 0) {
			std::vector<job_t> ready;
			{
				std::unique_lock<std::mutex> lock(mutex);
				auto wait = timers.empty() ? std::chrono::steady_clock::now() + std::chrono::seconds(10) : timers.begin()->first;
				if (jobs.empty()) {
					cv.wait_until(lock, wait, [] { return !jobs.empty(); });
				}
				ready.swap(jobs);
			}
			for (auto &job : ready) {
				guard([&] { job(rt); });
				guard([&] { rt.drainMicrotasks(); });
			}
			// One timer a turn, so a timer that sets another never starves what was posted.
			if (exit_code < 0 && !timers.empty() && timers.begin()->first <= std::chrono::steady_clock::now()) {
				auto f = timers.begin()->second.second;
				timers.erase(timers.begin());
				guard([&] { f->call(rt); });
				guard([&] { rt.drainMicrotasks(); });
			}
			if (ready.empty() && timers.empty()) {
				std::unique_lock<std::mutex> lock(mutex);
				if (jobs.empty() && !cv.wait_for(lock, std::chrono::seconds(10), [] { return !jobs.empty(); })) {
					break;
				}
			}
		}
		std::vector<job_t> dropped;
		{
			std::lock_guard<std::mutex> lock(mutex);
			open = false;
			dropped.swap(jobs);
		}
		dropped.clear();
		timers.clear();
	}
	runtime.reset();
	return exit_code < 0 ? 0 : exit_code;
}

} // namespace

int main(int argc, char **argv) {
	bool reload = argc > 1 && std::string(argv[1]) == "--reload";
	int first = reload ? 2 : 1;
	if (argc <= first) {
		std::fprintf(stderr, "usage: rn_host [--reload] <bundle.js> [args...]\n");
		return 2;
	}
	std::ifstream in(argv[first], std::ios::binary);
	std::stringstream src;
	src << in.rdbuf();
	std::vector<std::string> args(argv + first + 1, argv + argc);
	int code = run(argv[first], src.str(), args);
	if (reload && code == 0) {
		code = run(argv[first], src.str(), args);
	}
	std::fflush(stdout);
	// Rust's threads may still run, such as a session's runtime.
	std::_Exit(code);
}
