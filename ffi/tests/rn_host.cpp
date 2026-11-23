// A host for React Native's binding off a device: Hermes, as React Native runs it, with
// the JSI module installed as `globalThis.__rust2_<prefix>`, an event loop that runs what
// the module posts and `setTimeout`'s timers, and `print`, `process`, and `TextDecoder`
// stand-ins. It runs one bundled script, exiting with its `process.exit` code, or 1 if it
// throws or leaves a promise rejected.
//
// Build it with the module's `cpp/<prefix>_jsi.cpp`, defining RUST2_PREFIX_NS as its
// namespace and RUST2_GLOBAL as its global.

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

#include <hermes/hermes.h>
#include <jsi/jsi.h>

#include "api_jsi.h"

namespace jsi = facebook::jsi;
using job_t = std::function<void(jsi::Runtime &)>;

namespace {

std::mutex mutex;
std::condition_variable cv;
std::vector<job_t> jobs;

std::multimap<std::chrono::steady_clock::time_point, std::pair<double, std::shared_ptr<jsi::Function>>> timers;
double next_timer = 1;

int exit_code = -1;

void post(job_t job) {
	std::lock_guard<std::mutex> lock(mutex);
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
};
globalThis.console = { log: (...a) => print(a.join(" ")), error: (...a) => __eprint(a.join(" ") + "\n") };
// UTF-8 as WHATWG's decoder reads it, replacing what is not.
globalThis.TextDecoder = class TextDecoder {
	decode(bytes) { return __decode(bytes === undefined ? new Uint8Array(0) : new Uint8Array(bytes.buffer ?? bytes, bytes.byteOffset ?? 0, bytes.byteLength)); }
};
globalThis.__rejected = [];
)";

} // namespace

int main(int argc, char **argv) {
	if (argc < 2) {
		std::fprintf(stderr, "usage: rn_host <bundle.js> [args...]\n");
		return 2;
	}
	std::ifstream in(argv[1], std::ios::binary);
	std::stringstream src;
	src << in.rdbuf();

	auto config = hermes::vm::RuntimeConfig::Builder().withMicrotaskQueue(true).build();
	std::unique_ptr<facebook::hermes::HermesRuntime> runtime = facebook::hermes::makeHermesRuntime(config);
	jsi::Runtime &rt = *runtime;
	jsi::Object global = rt.global();

	jsi::Array args(rt, argc - 2);
	for (int i = 2; i < argc; i++) {
		args.setValueAtIndex(rt, i - 2, jsi::String::createFromUtf8(rt, argv[i]));
	}
	global.setProperty(rt, "__argv", args);
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
	fn(rt, global, "__decode", 1, [](jsi::Runtime &rt, const jsi::Value &, const jsi::Value *a, size_t n) {
		jsi::ArrayBuffer ab = a[0].getObject(rt).getPropertyAsObject(rt, "buffer").getArrayBuffer(rt);
		auto off = static_cast<size_t>(a[0].getObject(rt).getProperty(rt, "byteOffset").getNumber());
		auto len = static_cast<size_t>(a[0].getObject(rt).getProperty(rt, "byteLength").getNumber());
		// Lossless for the UTF-8 a test decodes.
		return jsi::Value(jsi::String::createFromUtf8(rt, ab.data(rt) + off, len));
	});

	int code = 0;
	auto run = [&](const std::function<void()> &f) {
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
	run([&] { rt.evaluateJavaScript(std::make_shared<jsi::StringBuffer>(prelude), "prelude.js"); });
	run([&] { rt.evaluateJavaScript(std::make_shared<jsi::StringBuffer>(src.str()), argv[1]); });
	run([&] { rt.drainMicrotasks(); });
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
			run([&] { job(rt); });
			run([&] { rt.drainMicrotasks(); });
		}
		// One timer a turn, so a timer that sets another never starves what was posted.
		if (exit_code < 0 && !timers.empty() && timers.begin()->first <= std::chrono::steady_clock::now()) {
			auto f = timers.begin()->second.second;
			timers.erase(timers.begin());
			run([&] { f->call(rt); });
			run([&] { rt.drainMicrotasks(); });
		}
		if (ready.empty() && timers.empty()) {
			std::unique_lock<std::mutex> lock(mutex);
			if (jobs.empty() && !cv.wait_for(lock, std::chrono::seconds(10), [] { return !jobs.empty(); })) {
				break;
			}
		}
	}
	code = exit_code < 0 ? 0 : exit_code;
	std::fflush(stdout);
	// What the module holds of the runtime goes with it, as a reload's does.
	std::_Exit(code);
}
