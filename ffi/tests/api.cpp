// The api fixture's C++ test: the script fixtures/api/examples/transcript.rs runs
// natively, through the C++ tests/fixtures/transcript writes from it, transcript.hpp, which
// calls the functions in `hook` here that transcript.rs defines. tests/fixtures.rs compares
// the transcripts.
//
// With an argument, it instead checks the futures' semantics, who owns what, or what the
// transcript says only in Rust, or breaks a contract of the ABI, which must abort.

#include <algorithm>
#include <array>
#include <chrono>
#include <cmath>
#include <condition_variable>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <functional>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <mutex>
#include <optional>
#include <string>
#include <string_view>
#include <thread>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>
#if __has_include(<version>)
#include <version>
#endif
#ifdef __cpp_lib_span
#include <span>
#endif
#ifdef __cpp_lib_ranges
#include <ranges>
#endif

#include "api.hpp"

#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#endif

#ifdef __linux__
#include <sys/prctl.h>
#endif

namespace {

// The functions transcript.rs defines, each C++'s own: what tests its idiom, such as its
// futures and readers, or prints a value no two bindings shape alike.
namespace hook {

// Print a returned buffer.
void print(const char *before, const api::buf &buf, const char *after) {
	std::printf("%s%.*s%s", before, static_cast<int>(buf.size()), reinterpret_cast<const char *>(buf.data()), after);
}

const char *yes(bool b) {
	return b ? "true" : "false";
}

unsigned long long ull(std::uint64_t v) {
	return static_cast<unsigned long long>(v);
}

// The Log callback: keeps even levels.
class Print final : public api::Log {
public:
	bool log(std::uint8_t level, api::view line) const override {
		std::printf("log %u %.*s\n", level, static_cast<int>(line.size()), reinterpret_cast<const char *>(line.data()));
		return level % 2 == 0;
	}
	~Print() override {
		std::printf("freed\n");
	}
};

// Lowercase hex of `bytes`, two digits each.
std::string hex(api::view bytes) {
	std::string out;
	char digits[3];
	for (std::uint8_t b : bytes) {
		std::snprintf(digits, sizeof digits, "%02x", b);
		out += digits;
	}
	return out;
}

// A path's bytes as Rust's `escape_ascii` prints them.
std::string escaped(api::view path) {
	std::string out;
	for (std::uint8_t b : path) {
		switch (b) {
		case '\t':
			out += "\\t";
			break;
		case '\r':
			out += "\\r";
			break;
		case '\n':
			out += "\\n";
			break;
		case '\\':
		case '\'':
		case '"':
			out += '\\';
			out += static_cast<char>(b);
			break;
		default:
			if (b >= 0x20 && b < 0x7f) {
				out += static_cast<char>(b);
			} else {
				char escape[5];
				std::snprintf(escape, sizeof escape, "\\x%02x", b);
				out += escape;
			}
		}
	}
	return out;
}

std::string path_hex(api::view path) {
	return hex(path);
}

// A duration as its seconds and nine digits of nanoseconds.
std::string seconds(api::duration d) {
	char out[40];
	std::snprintf(out, sizeof out, "%llu.%09u", ull(d.secs), d.nanos);
	return out;
}

// A duration in whole milliseconds.
std::uint64_t millis(api::duration d) {
	return d.secs * 1000 + d.nanos / 1000000;
}

std::uint32_t code(char32_t c) {
	return static_cast<std::uint32_t>(c);
}

std::vector<api::tls_Codec> reversed(std::vector<api::tls_Codec> list) {
	std::reverse(list.begin(), list.end());
	return list;
}

std::vector<std::uint64_t> f64_bits(const std::vector<double> &floats) {
	std::vector<std::uint64_t> out;
	for (double f : floats) {
		std::uint64_t bits;
		std::memcpy(&bits, &f, sizeof bits);
		out.push_back(bits);
	}
	return out;
}

// An address's bytes in hex, so the length shows its family.
std::string ip_hex(const api::ip_addr &ip) {
	return ip.v6 ? hex(api::view(ip.ip.v6.octets, sizeof ip.ip.v6.octets)) : hex(api::view(ip.ip.v4.octets, sizeof ip.ip.v4.octets));
}

std::string ip_hex(const api::ipv4_addr &v4) {
	api::ip_addr ip{};
	ip.ip.v4 = v4;
	return ip_hex(ip);
}

std::string ip_hex(const api::ipv6_addr &v6) {
	api::ip_addr ip{};
	ip.v6 = true;
	ip.ip.v6 = v6;
	return ip_hex(ip);
}

// A socket address as its IP's bytes in hex, its port, flow label, and scope, each IPv6's
// alone, apart by colons.
std::string socket_hex(const api::socket_addr &addr) {
	if (addr.v6) {
		const api::socket_addr_v6 &v6 = addr.addr.v6;
		return hex(api::view(v6.ip.octets, sizeof v6.ip.octets)) + ":" + std::to_string(v6.port) + ":" +
		       std::to_string(v6.flowinfo) + ":" + std::to_string(v6.scope_id);
	}
	return hex(api::view(addr.addr.v4.ip.octets, sizeof addr.addr.v4.ip.octets)) + ":" + std::to_string(addr.addr.v4.port);
}

std::string socket_hex(const api::socket_addr_v4 &v4) {
	api::socket_addr addr{};
	addr.addr.v4 = v4;
	return socket_hex(addr);
}

std::string socket_hex(const api::socket_addr_v6 &v6) {
	api::socket_addr addr{};
	addr.v6 = true;
	addr.addr.v6 = v6;
	return socket_hex(addr);
}

// Print a list as Rust's `{:?}` does, each item as `show` spells it.
template <typename T, typename F>
void print_list(const std::vector<T> &list, F show) {
	std::printf("[");
	for (size_t i = 0; i < list.size(); i++) {
		std::printf(i ? ", %s" : "%s", show(list[i]).c_str());
	}
	std::printf("]");
}

// Print each of `n`'s lists, its floats as their bits, as transcript.rs does.
void numbers(std::string_view what, const api::Numbers &n) {
	auto line = [&](const char *name, const auto &list, auto show) {
		std::printf("%.*s %s ", static_cast<int>(what.size()), what.data(), name);
		print_list(list, show);
		std::printf("\n");
	};
	auto num = [](auto v) { return std::to_string(v); };
	line("u16", n.u16s, num);
	line("u32", n.u32s, num);
	line("u64", n.u64s, num);
	line("i8", n.i8s, [](int8_t v) { return std::to_string(int(v)); });
	line("i16", n.i16s, num);
	line("i32", n.i32s, num);
	line("i64", n.i64s, num);
	line("f32", n.f32s, [](float v) {
		std::uint32_t bits;
		std::memcpy(&bits, &v, sizeof bits);
		return std::to_string(bits);
	});
	line("f64", n.f64s, [](double v) {
		std::uint64_t bits;
		std::memcpy(&bits, &v, sizeof bits);
		return std::to_string(bits);
	});
	if (n.some) {
		std::printf("%.*s some Some(", static_cast<int>(what.size()), what.data());
		print_list(*n.some, num);
		std::printf(")\n");
	} else {
		std::printf("%.*s some None\n", static_cast<int>(what.size()), what.data());
	}
}

// A path no Rust string spells: on Unix, bytes that are not UTF-8; on Windows, a lone
// surrogate, which is WTF-8's `ed a0 80`.
api::buf unspelled() {
#ifdef _WIN32
	static const std::uint8_t stray[] = {0xed, 0xa0, 0x80};
	return api::buf(api::view(stray, sizeof stray));
#else
	return api::stray();
#endif
}

// A channel's futures, polled by hand.
void channel() {
	static int wakes = 0;
	api::Channel channel = api::Channel::new_();
	api::waker waker([]() { wakes++; });
	{
		api::Channel_recv_future recv = channel.recv();
		std::printf("recv pending %s\n", yes(!recv.poll(waker).has_value()));
		channel.send("hi");
		std::printf("woken %d\n", wakes);
		if (auto payload = recv.poll(waker)) {
			print("recv ", *payload, "\n");
		}
	}
	channel.send("");
	{
		api::Channel_recv_nonempty_future nonempty = channel.recv_nonempty();
		if (auto r = nonempty.poll(waker)) {
			if (*r) {
				std::printf("nonempty %zu\n", r->value().size());
			} else {
				print("nonempty err ", api::Error_to_string(r->error()), "\n");
			}
		} else {
			std::printf("nonempty pending\n");
		}
	}
	{
		auto fill4 = api::Batch::new_();
		auto fill2 = api::pair();
		auto filled4 = api::fill_4(fill4, 9).poll(waker);
		auto filled2 = api::fill_2(fill2, 1).poll(waker);
		if (filled4 && filled2) {
			std::printf("fill");
			for (auto v : *filled4) {
				std::printf(" %u", unsigned(v));
			}
			for (auto v : *filled2) {
				std::printf(" %u", unsigned(v));
			}
			std::printf("\n");
		}
	}

	// The send happens at the call: the future is dropped unpolled.
	(void)channel.send_later("eager");
	{
		api::Channel_recv_future recv = channel.recv();
		if (auto payload = recv.poll(waker)) {
			print("recv ", *payload, "\n");
		}
	}
	// The send happens at the call, and the future borrows the channel and the tag.
	{
		api::Channel_tagged_future tagged = channel.tagged("tag");
		if (auto text = tagged.poll(waker)) {
			print("tagged ", *text, "\n");
		}
	}
	// So does a future borrowing an optional slice, which the export owns while it is pending.
	{
		api::Channel_itemized_future itemized = channel.itemized(std::vector<std::uint32_t>{1, 2, 3});
		if (auto count = itemized.poll(waker)) {
			std::printf("itemized %zu\n", *count);
		}
	}
	// Another crate's future borrowing its bytes, which the export owns while it is pending.
	{
		api::byte_len_future measuring = api::byte_len("abc");
		if (auto len = measuring.poll(waker)) {
			std::printf("byte_len %zu\n", *len);
		}
	}
	channel.send("sized");
	{
		api::Channel_recv_sized_future sized = channel.recv_sized();
		if (auto r = sized.poll(waker)) {
			print("sized ", std::get<0>(*r), " ");
			std::printf("%zu\n", std::get<1>(*r));
		}
	}
	// The waker moves into Rust, which keeps it until the send wakes it.
	channel.notify(waker);
	channel.send("rung");
	std::printf("woken %d\n", wakes);
	// A poll with no async twin is a future.
	{
		api::Channel_poll_take_future take = channel.poll_take();
		if (auto payload = take.poll(waker)) {
			print("take ", *payload, "\n");
		}
	}
	channel.send("measured");
	{
		api::Channel_poll_measure_future measure = channel.poll_measure(2);
		if (auto r = measure.poll(waker)) {
			std::printf("measure %zu\n", r->value());
		}
	}
	// A context is made of the waker, which Rust keeps until the send wakes it.
	channel.listen(waker);
	channel.send("heard");
	std::printf("woken %d\n", wakes);
	// A returned waker wakes the waiter's task, through Rust, until its last copy drops.
	api::Waiter waiter = api::Waiter::new_(waker);
	waiter.waker().wake();
	waiter.context().wake();
	std::printf("woken %d\n", wakes);
}

// A kept closure: called again at the instant it asks for, then once its waiter wakes.
void drive() {
	api::Moment began = *api::Moment::new_(0, 1000);
	int calls = 0;
	auto drive = api::drive(began, [&](const api::Moment &now, const api::Waiter &waiter) {
		using result = api::result<std::optional<api::Moment>, std::string>;
		calls++;
		if (calls == 1) {
			return result::ok(now.after(5));
		}
		if (calls == 2) {
			std::printf("drive %llu\n", ull(now.millis() - began.millis()));
			waiter.waker().wake();
			return result::ok(std::nullopt);
		}
		return result::err("stopped after " + std::to_string(calls));
	});
	api::waker waker([] {});
	std::optional<api::buf> stopped;
	while (!(stopped = drive.poll(waker))) {
	}
	print("drive ", *stopped, "\n");
}

// A reader of bytes in memory, which Rust reads and seeks.
struct memory {
	std::string bytes;
	std::size_t at = 0;

	api::result<std::size_t, std::string> read(std::uint8_t *buf, std::size_t len) {
		std::size_t n = std::min(len, bytes.size() - at);
		std::memcpy(buf, bytes.data() + at, n);
		at += n;
		return api::result<std::size_t, std::string>::ok(n);
	}
	api::result<std::uint64_t, std::string> seek(std::int64_t offset, std::ios_base::seekdir dir) {
		std::int64_t base = dir == std::ios_base::beg ? 0 : static_cast<std::int64_t>(dir == std::ios_base::cur ? at : bytes.size());
		if (base + offset < 0) {
			return api::result<std::uint64_t, std::string>::err("before the start");
		}
		at = static_cast<std::size_t>(base + offset);
		return api::result<std::uint64_t, std::string>::ok(at);
	}
};

// A reader whose reads fail, as the transcript's: each seek lands at 0.
struct broken_reader {
	api::result<std::size_t, std::string> read(std::uint8_t *, std::size_t) {
		return api::result<std::size_t, std::string>::err("broken");
	}
	api::result<std::uint64_t, std::string> seek(std::int64_t, std::ios_base::seekdir) {
		return api::result<std::uint64_t, std::string>::ok(0);
	}
};

// A reader whose reads fail, which seeks anywhere from its start, as Rust's `Cursor` does.
struct far_reader : broken_reader {
	api::result<std::uint64_t, std::string> seek_start(std::uint64_t offset) {
		return api::result<std::uint64_t, std::string>::ok(offset);
	}
};

// Print what `tail` read of `file`.
template <class R>
void tail(R &file) {
	auto read = api::tail(file, 5);
	if (read) {
		print("tail ", std::get<0>(*read), "");
		std::printf(" %llu\n", ull(std::get<1>(*read)));
	} else {
		print("tail error ", read.error(), "\n");
	}
}

// Readers over bytes, whose reads fail, and that seek past `INT64_MAX`.
void readers() {
	memory hello{"hello world"};
	tail(hello);
	broken_reader broken;
	tail(broken);
	// A seek from the start reaches the reader whole, past `INT64_MAX`, as `Cursor` takes one.
	far_reader far;
	auto sought = api::seek_to(far, UINT64_MAX);
	if (sought) {
		std::printf("sought %llu\n", ull(*sought));
	} else {
		print("sought error ", sought.error(), "\n");
	}
}

// Streams, each `next` awaited in turn, as C++ has no asynchronous iteration of its own.
void streams() {
	{
		api::Countdown countdown = api::Countdown::new_(3);
		std::printf("countdown");
		const char *sep = " ";
		while (auto count = countdown.next().get()) {
			std::printf("%s%u", sep, unsigned(*count));
		}
		std::printf("\n");
	}
	api::Countdown countdown = api::Countdown::new_(5);
	while (auto count = countdown.next().get()) {
		if (*count == 4) {
			break;
		}
	}
	std::printf("countdown after break %u\n", unsigned(*countdown.next().get()));
	api::Channel channel = api::Channel::new_();
	api::Feed feed = channel.feed();
	for (const char *first : {"a", "c"}) {
		channel.send(first);
		for (;;) {
			auto next = feed.next().get();
			if (!next) {
				print("feed error ", api::Error_to_string(next.error()), "\n");
				break;
			}
			if (!next.value()) {
				std::printf("feed end\n");
				break;
			}
			const api::buf &payload = *next.value();
			print("feed ", payload, "\n");
			channel.send(payload.str() == "a" ? "b" : payload.str() == "b" ? "!" : "");
		}
	}
}

// Futures of a value that is not `Send`, polled on this thread until each is ready.
void local_later(const api::Local &local) {
	api::waker waker([]() {});
	const api::Local *froms[] = {&local, nullptr};
	for (const api::Local *from : froms) {
		api::local_or_new_later_future later = api::local_or_new_later(from);
		auto made = later.poll(waker);
		while (!made) {
			made = later.poll(waker);
		}
		std::printf("local later%s %u\n", from ? "" : " fresh", static_cast<unsigned>(made->bump()));
	}
}

} // namespace hook

} // namespace

#include "transcript.hpp"

namespace {

// Each check that must hold, beyond the transcript.
int check(bool ok, const char *what) {
	if (!ok) {
		std::fprintf(stderr, "failed: %s\n", what);
		std::exit(1);
	}
	return 0;
}

// What the transcript's script says only in Rust: C++'s own copies, hashes, ranges, and
// operators, each checked by an assertion.
int checks() {
	// Fields named as an `operator==`'s argument might be.
	{
		api::Shadowed one{{1}, 2, 3};
		api::Shadowed two{{1}, 2, 4};
		check(one == one && !(one == two) && one != two, "a record with fields named other, a, and b compares");
	}
	// Types named as an `operator==`'s arguments might be.
	{
		api::a one{1};
		api::a two{2};
		api::b unit = api::b_One{};
		api::b two_b = api::b_Two{2};
		auto handle = api::c::new_(1);
		check(one == one && !(one == two) && unit != two_b && handle == api::c::new_(1) && !(handle == api::c::new_(2)),
			"types named a, b, and c compare");
	}
	// Names a system header defines as macros, escaped.
	check(api::EOF_ == -1 && api::WEXITSTATUS_(0x300) == 3, "names of system macros escape");
	check(api::clash_Errno::NULL_().errno_ == 0 && api::clash_Errno{5}.errno_ == 5, "members named as system macros escape");
	// A name takes a copy of the caller's buffer, which is its own again after.
	{
		api::Name name = api::Name::owned("a");
		std::string text = "c";
		name.rename(text);
		text[0] = 'x';
		check(name.as_str().str() == "c", "a renamed name holds its own copy");
	}
	// `std::hash` hashes a key as Rust's `Hash` does: equal keys alike, and these two apart.
	{
		api::Key a = api::Key::new_("a");
		api::Key b = api::Key::new_("b");
		check(a.hash() == a.hash_of(), "Key::hash is Rust's own hash");
		std::hash<api::Key> hasher;
		check(hasher(a) == hasher(api::Key::new_("a")) && hasher(a) != hasher(b), "std::hash hashes keys as Rust does");
	}
	// `Copy` objects with no C++ copy, beside bytes: a returned vector moves back in, stored or
	// not, each ticket moved and each name viewed in place.
	{
		std::vector<std::tuple<api::view, api::Ticket>> lent;
		lent.emplace_back("a", api::Ticket::new_(1));
		auto returned = api::tickets(lent);
		auto again = api::tickets(api::tickets(std::move(returned)));
		check(again.size() == 1 && std::get<0>(again[0]).str() == "a" && std::get<1>(again[0]).number() == 1,
			"returned tickets pass back");
	}
#ifdef __cpp_lib_ranges
	// A range whose end is a sentinel of another type than its begin.
	{
		std::vector<std::string> letters{"p", "q", "r"};
		auto taken = api::counters(letters | std::views::take_while([](const std::string &n) { return n != "r"; }));
		check(taken.size() == 2 && taken[1].name().str() == "q", "a sentinel range passes");
	}
#endif
	// A kept closure, called on another thread after the call that took it returned.
	check(api::Spawned::new_([] { return 7; }).join() == 7, "a kept closure runs on another thread");
	// A returned `Vec<&Token>` holds clones, which the vector drops.
	{
		api::Token twice = api::Token::new_(9);
		std::uint32_t before = api::dropped();
		{
			auto tokens = twice.twice();
			check(tokens.size() == 2 && tokens[1].id_get() == 9, "a vector of lent tokens holds clones");
		}
		check(api::dropped() == before + 2, "the vector drops its clones");
	}
	std::printf("checks ok\n");
	return 0;
}

// A queue of jobs that a test runs when it chooses: the executor futures run on.
struct queue {
	std::mutex mutex;
	std::deque<std::function<void()>> jobs;

	auto executor() {
		return [this](std::function<void()> job) {
			std::lock_guard<std::mutex> lock(mutex);
			jobs.push_back(std::move(job));
		};
	}

	// Run jobs until none are left.
	void drain() {
		for (;;) {
			std::function<void()> job;
			{
				std::lock_guard<std::mutex> lock(mutex);
				if (jobs.empty()) {
					return;
				}
				job = std::move(jobs.front());
				jobs.pop_front();
			}
			job();
		}
	}
};

// A future C++ implements, which Rust polls: ready on its `ready`th poll, keeping the
// waker of the last in what it shares with the test.
struct countdown {
	struct state {
		int ready;
		int polls = 0;
		std::optional<api::waker> waker;
		explicit state(int r) : ready(r) {}
	};
	std::shared_ptr<state> s;

	bool poll(const api::waker &w) {
		if (++s->polls >= s->ready) {
			s->waker.reset();
			return true;
		}
		s->waker = w;
		return false;
	}
};

int foreign_futures() {
	// Ready at once.
	{
		auto s = std::make_shared<countdown::state>(1);
		check(api::after(countdown{s}, 4).get() == 8 && s->polls == 1, "a future C++ implements is awaited");
		check(s.use_count() == 1, "Rust destroys a future C++ implements once done with it");
	}
	// Pending once: its wake reaches the waker Rust was polled with.
	{
		auto s = std::make_shared<countdown::state>(2);
		api::after_future after = api::after(countdown{s}, 5);
		int woken = 0;
		api::waker waker([&]() { woken++; });
		check(!after.poll(waker) && s->waker, "a future C++ implements pends");
		s->waker->wake();
		check(woken == 1, "its wake reaches Rust's poller");
		auto got = after.poll(waker);
		check(got && *got == 10 && s.use_count() == 1, "it finishes once polled again, and is destroyed");
	}
	// Never ready: Rust gives up on it, destroying it with the waker it kept.
	{
		auto s = std::make_shared<countdown::state>(1000);
		auto got = api::within(0, countdown{s}).get();
		check(!got && std::holds_alternative<api::Error_Overflow>(got.error()), "Rust gives up on a future C++ implements");
		check(s.use_count() == 1 && s->polls == 1, "and destroys it");
	}
	// A binding's future of void is one too.
	{
		api::Channel channel = api::Channel::new_();
		check(api::after(channel.send_later("passed"), 6).get() == 12, "a binding's future of void passes to Rust");
		check(channel.recv().get().str() == "passed", "the future Rust polled ran");
	}
	// Ready in time, by a second poll.
	{
		auto s = std::make_shared<countdown::state>(2);
		api::within_future within = api::within(1, countdown{s});
		api::waker waker([]() {});
		check(!within.poll(waker), "a future C++ implements pends once");
		check(within.poll(waker) && s->polls == 2, "a future C++ implements finishes in time");
	}
	// A poll of a waiter, made of each poll's waker, of a future C++ implements, which the
	// future owns and lends it pinned: destroyed once it finishes.
	{
		auto s = std::make_shared<countdown::state>(2);
		api::Waiter_poll_future_future polled = api::Waiter::poll_future(countdown{s});
		int woken = 0;
		api::waker waker([&]() { woken++; });
		check(!polled.poll(waker) && s->waker, "a future a poll lends pends");
		s->waker->wake();
		check(woken == 1 && s.use_count() == 2, "its wake reaches Rust's poller, and the future keeps it");
		check(polled.poll(waker) && s->polls == 2 && s.use_count() == 1, "it finishes once polled again, and is destroyed");
	}
	// Dropped before it finishes, it destroys the future it owns.
	{
		auto s = std::make_shared<countdown::state>(1000);
		{
			api::Waiter_poll_future_future polled = api::Waiter::poll_future(countdown{s});
			api::waker waker([]() {});
			check(!polled.poll(waker), "a future a poll lends pends");
		}
		check(s.use_count() == 1, "a dropped future destroys the future it owns");
	}
	// A kept closure, destroyed once Rust drops it: after its thread ran it.
	{
		auto kept = std::make_shared<int>(7);
		api::Spawned spawned = api::Spawned::new_([kept] { return static_cast<std::uint32_t>(*kept); });
		check(std::move(spawned).join() == 7 && kept.use_count() == 1, "Rust destroys a closure it kept once done with it");
	}
	// A kept closure the future of the call holds, destroyed with it before it finishes.
	{
		auto kept = std::make_shared<int>(0);
		api::Moment start = *api::Moment::new_(0, 1000);
		{
			auto drive = api::drive(start, [kept](const api::Moment &, const api::Waiter &) {
				return api::result<std::optional<api::Moment>, api::buf>::ok(std::nullopt);
			});
			check(kept.use_count() == 2, "Rust keeps its own copy of a closure");
			api::waker waker([]() {});
			check(!drive.poll(waker) && kept.use_count() == 2, "a future keeps the closure while it pends");
		}
		check(kept.use_count() == 1, "a dropped future destroys the closure it kept");
	}
	// A closure failing with an error any view takes, and a reader's seek failing.
	{
		api::Moment start = *api::Moment::new_(0, 1000);
		auto drive = api::drive(start, [](const api::Moment &, const api::Waiter &) {
			return api::result<std::optional<api::Moment>, const char *>::err("failed");
		});
		check(std::move(drive).get().str() == "failed", "a closure's error reaches Rust as its message");
		hook::memory empty{""};
		empty.at = 0;
		auto read = api::tail(empty, 5);
		check(read && std::get<0>(*read).empty() && std::get<1>(*read) == 0, "a reader of nothing reads nothing");
	}
	return 0;
}

int futures() {
	foreign_futures();
	api::Channel channel = api::Channel::new_();

	// Blocking: the send lands from another thread while it waits.
	{
		std::thread sender([&]() { channel.send("blocked"); });
		api::buf payload = channel.recv().get();
		sender.join();
		check(payload.str() == "blocked", "get returns the payload sent");
	}

	// On an executor: polled there, and called back there once ready.
	queue q;
	{
		std::string got;
		api::task task = channel.recv().then(q.executor(), [&](api::buf payload) { got = payload.str(); });
		q.drain();
		check(got.empty(), "then waits for a send");
		channel.send("then");
		check(got.empty(), "then never polls inside the wake");
		q.drain();
		check(got == "then", "then calls back with the payload");
	}

	// A wake that lands while the future polls, before it returns pending, is kept: the
	// future polls again rather than waiting for a wake that already happened.
	{
		std::uint32_t got = 0;
		api::task task = api::yielded(7).then(q.executor(), [&](std::uint32_t value) { got = value; });
		q.drain();
		check(got == 7, "a wake during the poll polls again");
	}

	// A value that is not Send stays on the thread that made it, whose executor polls its
	// futures; destroyed on another thread, it drops on its own, posted to the executor of
	// its thread's latest task, or else at its next call.
	{
		api::Local local = api::Local::new_();
		api::Local shared = local.share();
		local.bump();
		check(shared.bump() == 2, "a shared local counts together");
		std::uint32_t got = 0;
		api::task later = local.later().then(q.executor(), [&](std::uint32_t value) { got = value; });
		q.drain();
		check(got == 3, "a local's future runs on its thread");
		api::task doubled = api::local_later(5).then(q.executor(), [&](std::uint32_t value) { got = value; });
		q.drain();
		check(got == 10, "a future that is not Send runs on its thread");
		std::uint32_t before = api::locals_dropped();
		std::thread([&]() { api::Local gone = std::move(local); }).join();
		check(api::locals_dropped() == before, "a local destroyed elsewhere waits for its thread");
		q.drain();
		check(api::locals_dropped() == before + 1, "a local destroyed elsewhere drops on its idle executor");

		// An executor running the drain on another thread leaves it to the next call.
		queue away;
		auto kept = std::make_shared<int>(0);
		api::task there = api::yielded(7).then(away.executor(), [&got, kept](std::uint32_t value) { got = value; });
		api::Local other = shared.share();
		std::thread([&]() { api::Local gone = std::move(other); }).join();
		std::thread([&]() { away.drain(); }).join();
		check(got == 7 && api::locals_dropped() == before + 1, "a drain on another thread drops nothing of this one");
		check(shared.bump() == 4 && api::locals_dropped() == before + 2, "a local destroyed elsewhere drops on its thread");

		// Once the latest task is gone, its executor is never called again.
		there = api::task();
		check(kept.use_count() == 1, "a gone task's callback is freed, though its thread's wake outlives it");
		api::Local last = shared.share();
		std::thread([&]() { api::Local gone = std::move(last); }).join();
		check(away.jobs.empty(), "a drain is never posted to a gone task's executor");
		check(shared.bump() == 5 && api::locals_dropped() == before + 3, "a local destroyed elsewhere drops at its next call");

		// A drop queued before its thread's first task drains through that task's executor.
		std::thread([]() {
			queue mine;
			api::Local early = api::Local::new_();
			std::uint32_t was = api::locals_dropped();
			std::thread([&]() { api::Local gone = std::move(early); }).join();
			api::task first = api::yielded(1).then(mine.executor(), [](std::uint32_t) {});
			mine.drain();
			check(api::locals_dropped() == was + 1, "a drop queued before its thread's first task drains through it");
		}).join();
	}

	// Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	// for the next.
	{
		bool called = false;
		{
			api::task task = channel.recv().then(q.executor(), [&](api::buf) { called = true; });
			q.drain();
			channel.send("kept");
		}
		q.drain();
		check(!called, "a cancelled task never calls back");
		auto next = channel.recv();
		auto payload = next.poll(api::waker([]() {}));
		check(payload && payload->str() == "kept", "a cancelled receive loses nothing");
	}

	// Cancelled from its own callback, it finishes.
	{
		api::task task;
		bool called = false;
		channel.send("self");
		task = channel.recv().then(q.executor(), [&](api::buf) {
			called = true;
			task = api::task();
		});
		q.drain();
		check(called, "a task cancelled in its callback finishes");
	}

	// Destroyed while a wake is posting it, a task returns only once that post is done, so
	// its executor can go with it. This executor blocks inside the wake's post until released:
	// a destroy that returns before then has left the executor in use. Seeing that the
	// destroy waits needs a bound, so it is given 100 ms, more than a destroy that does not
	// wait ever takes.
	{
		std::mutex mutex;
		std::condition_variable cv;
		std::deque<std::function<void()>> jobs;
		int posts = 0;
		bool entered = false;
		bool released = false;
		bool destroyed = false;
		auto exec = [&](std::function<void()> job) {
			std::unique_lock<std::mutex> lock(mutex);
			jobs.push_back(std::move(job));
			// The first post is the task's own first poll; the second, the send's wake.
			if (++posts == 2) {
				entered = true;
				cv.notify_all();
				cv.wait(lock, [&]() { return released; });
			}
		};
		api::task task = channel.recv().then(exec, [](api::buf) {});
		std::function<void()> first;
		{
			std::lock_guard<std::mutex> lock(mutex);
			first = std::move(jobs.front());
			jobs.pop_front();
		}
		first();
		std::thread sender([&]() { channel.send("raced"); });
		{
			std::unique_lock<std::mutex> lock(mutex);
			cv.wait(lock, [&]() { return entered; });
		}
		std::thread destroyer([&]() {
			task = api::task();
			std::lock_guard<std::mutex> lock(mutex);
			destroyed = true;
			cv.notify_all();
		});
		bool early = false;
		{
			std::unique_lock<std::mutex> lock(mutex);
			early = cv.wait_for(lock, std::chrono::milliseconds(100), [&]() { return destroyed; });
			released = true;
		}
		cv.notify_all();
		sender.join();
		destroyer.join();
		check(!early, "destroying a task waits for a post under way");
		jobs.clear();
		check(channel.recv().get().str() == "raced", "a task destroyed mid-post loses nothing");
	}

#if defined(__cpp_impl_coroutine) && __cpp_impl_coroutine >= 201902L && __has_include(<coroutine>)
	// Awaited in a coroutine, resumed on the executor.
	{
		struct detached {
			struct promise_type {
				detached get_return_object() {
					return {std::coroutine_handle<promise_type>::from_promise(*this)};
				}
				std::suspend_never initial_suspend() noexcept {
					return {};
				}
				std::suspend_always final_suspend() noexcept {
					return {};
				}
				void return_void() {}
				void unhandled_exception() {
					std::abort();
				}
			};
			std::coroutine_handle<promise_type> handle;
		};
		std::string got;
		auto read = [&](api::Channel &ch) -> detached {
			api::buf payload = co_await ch.recv().via(q.executor());
			got = payload.str();
			std::uint32_t value = co_await api::yielded(3).via(q.executor());
			got += std::to_string(value);
		};
		detached coroutine = read(channel);
		channel.send("awaited");
		q.drain();
		check(coroutine.handle.done() && got == "awaited3", "co_await resumes with each output");
		coroutine.handle.destroy();

		// Destroyed while suspended, the coroutine cancels its receive, which loses nothing.
		got.clear();
		detached cancelled = read(channel);
		q.drain();
		channel.send("again");
		cancelled.handle.destroy();
		q.drain();
		check(got.empty(), "a destroyed coroutine never resumes");
		auto payload = channel.recv().get();
		check(payload.str() == "again", "a destroyed coroutine's receive loses nothing");
	}
#endif
	std::printf("futures ok\n");
	return 0;
}

// Who owns each handle and bytes, beyond the transcript, each checked by an assertion.
// A payload Rust returned goes back to Rust in place: moved in, shared by a copy kept
// meanwhile, and written back through `&mut`. Its bytes stay where Rust put them, and
// drop once, with their last owner.
void forwarding() {
	std::uint32_t before = api::dropped();
	api::Channel channel = api::Channel::new_();
	api::buf payload = api::token_bytes(3, 64);
	const std::uint8_t *at = payload.data();
	channel.send(std::move(payload));
	payload = channel.recv().get();
	check(payload.data() == at && payload.size() == 64, "a payload moved into Rust stays in place");
	api::buf kept = payload;
	check(kept.data() == at, "a copy of Rust's bytes shares them");
	channel.send(kept);
	check(channel.recv().get().data() != at, "bytes lent to a call are copied");
	channel.send(std::move(kept));
	api::buf shared = channel.recv().get();
	check(shared.data() == at && payload.data() == at, "a copy moved into Rust stays in place");
	api::Packet packet = api::next_packet(api::Packet{1, std::move(shared)});
	check(packet.payload.size() == 64 && api::dropped() == before, "a record carries the payload on");
	payload = api::buf();
	packet = api::Packet{};
	check(api::dropped() == before + 1, "the payload drops once, with its last owner");
	api::buf digits = api::token_bytes('0', 2);
	api::put_number(7, digits);
	check(digits.str() == "007," && api::dropped() == before + 2, "Rust takes its bytes passed for &mut");
}

int lifetimes() {
	// Copies of a type whose clone no call tells apart share one handle: a write through
	// one shows in the other, as it does between clones of an `Arc`.
	{
		std::uint32_t before = api::dropped();
		api::Gauge a = api::Gauge::new_(1);
		api::Gauge b = a;
		b.set(2);
		check(a.level() == 2, "copies of a shared type see each other's writes");
		// Moving out of one copy moves out a clone, as Rust moves out of a clone, and
		// leaves the other copy whole.
		check(std::move(b).into_level() == 2, "a shared copy moves out its value");
		check(a.level() == 2 && api::dropped() == before, "moving out of a shared copy leaves the others");
		{
			api::Gauge c = a;
			(void)c;
		}
		check(api::dropped() == before, "a released copy frees nothing while another holds it");
		// The last copy moves out the value itself.
		check(std::move(a).into_level() == 2 && api::dropped() == before + 1, "the last copy moves out the value");
	}

	// A pending future borrows the value as though it held a copy: once the copy that made
	// it goes, moving out of the last copy moves out a clone, as Rust moves out of a clone
	// while a future borrows another.
	{
		std::uint32_t before = api::dropped();
		api::Gauge a = api::Gauge::new_(4);
		// On the heap, where GCC 16 sees no use of an uninitialized `optional`.
		auto made = std::make_unique<api::Gauge>(a);
		api::Gauge_later_future later = made->later();
		api::waker waker([]() {});
		check(!later.poll(waker), "a future pends");
		made.reset();
		check(std::move(a).into_level() == 4 && api::dropped() == before, "the last copy moves out a clone while a future borrows it");
		check(std::move(later).get() == 4 && api::dropped() == before + 1, "a future reads the value it borrows, and frees it once it finishes");
	}

	// Copies of a type whose clone is a value of its own, such as a lock behind a `Box`,
	// keep apart: copying is `clone()`, which crosses into Rust, and a write through one
	// clone never shows in the other.
	{
		api::Latch a = api::Latch::default_();
		api::Latch b = a.clone();
		check(b.set(1) && !a.get(), "a clone's write stays its own");
		check(a.set(2) && a.get() == 2u && b.get() == 1u, "each clone holds its own value");
	}

	// A copy released on one thread while a call runs on another: the call's copy keeps
	// the handle, and the last copy frees it, once, on whichever thread drops it.
	{
		std::uint32_t before = api::dropped();
		std::optional<api::Gauge> mine(api::Gauge::new_(7));
		std::mutex mutex;
		std::condition_variable cv;
		bool started = false;
		bool released = false;
		bool read = true;
		std::thread caller([&, theirs = *mine]() {
			{
				std::lock_guard<std::mutex> lock(mutex);
				started = true;
			}
			cv.notify_all();
			for (;;) {
				bool last;
				{
					std::lock_guard<std::mutex> lock(mutex);
					last = released;
				}
				read = read && theirs.level() == 7;
				if (last) {
					return;
				}
			}
		});
		{
			std::unique_lock<std::mutex> lock(mutex);
			cv.wait(lock, [&]() { return started; });
		}
		mine.reset();
		{
			std::lock_guard<std::mutex> lock(mutex);
			released = true;
		}
		caller.join();
		check(read, "a call races a copy's release unharmed");
		check(api::dropped() == before + 1, "the last copy frees the value once");
	}

	// A copy of a lent object lasts only its call, as the lent object does.
	{
		api::Gauge gauge = api::Gauge::new_(3);
		check(api::watch(gauge, [](const api::Gauge &lent) {
			api::Gauge copy = lent;
			return copy.level();
		}) == 3, "a copy of a lent object reads it during the call");
	}

	// Returned bytes own Rust's buffer, so they outlive the object they came from; a view
	// or span of them lasts only as long as they do.
	{
		api::buf payload;
		{
			api::Message message = api::Message::new_(std::nullopt, "kept");
			payload = message.payload_get();
		}
		check(payload.str() == "kept", "bytes outlive the object they came from");
		std::uint32_t before = api::dropped();
		{
			api::buf bytes = api::token_bytes(9, 3);
			check(api::dropped() == before && bytes.size() == 3 && bytes.data()[2] == 9, "bytes hold their owner");
#ifdef __cpp_lib_span
			// From C++20, a view is a contiguous range: a span of it, and a view of a span.
			std::span<const std::uint8_t> span = api::view(bytes);
			check(span.size() == 3 && span[0] == 9, "a view converts to a span");
			check(api::len(span) == 3, "a span converts to a view");
#endif
		}
		check(api::dropped() == before + 1, "bytes free their owner");
	}

	// Numbers lent in place, from wherever the caller holds them, with no copy on C++'s side.
	{
		const std::uint32_t array[] = {1, 2, 3};
		std::vector<std::uint32_t> vector(array, array + 3);
		std::array<std::uint32_t, 3> std_array = {1, 2, 3};
		const std::uint32_t *samples = vector.data();
		check(api::sum_u32(array) == 6, "a C array is a slice");
		check(api::sum_u32(vector) == 6 && api::sum_u32(std_array) == 6, "a contiguous range is a slice");
		check(api::sum_u32({samples, 2}) == 3, "a pointer and a count are a slice");
		check(api::sum_u32({}) == 0 && api::sum_u32({nullptr, 0}) == 0, "an empty slice is one");
		const std::int32_t owned[] = {1, 2};
		check(api::sum_lists(owned, std::nullopt, std::vector<std::int32_t>{4}) == 7 &&
		          api::sum_lists({}, api::slice<std::int32_t>(owned), std::nullopt) == 3,
		      "a slice stands in for an owned Vec and in an option");
#ifdef __cpp_lib_span
		std::span<const std::uint32_t> span = vector;
		check(api::sum_u32(span) == 6, "a span is a slice");
		std::span<const std::uint32_t> back = api::slice<std::uint32_t>(vector);
		check(back.size() == 3 && back[2] == 3, "a slice converts to a span");
#endif
	}

	forwarding();

	std::printf("lifetimes ok\n");
	return 0;
}

// Each breaks a contract, which must abort with a message naming the function. Those
// `tests/api_strict.cpp` refuses at compile time reach the object through a reference or
// pointer, whose state Clang does not track.
int broken(const char *which) {
#ifdef __linux__
	// Non-dumpable, the expected abort writes no core and never starts the host's crash
	// handler.
	if (prctl(PR_SET_DUMPABLE, 0, 0, 0, 0) != 0) {
		std::perror("prctl");
		return 1;
	}
#endif
	if (std::strcmp(which, "moved") == 0) {
		api::Counter c = api::Counter::new_("c", 0);
		api::Counter &same = c;
		api::Label label = std::move(c).into_label(std::nullopt);
		same.get();
	} else if (std::strcmp(which, "moved from") == 0) {
		api::Counter c = api::Counter::new_("c", 0);
		api::Counter &same = c;
		api::Counter d = std::move(c);
		same.get();
	} else if (std::strcmp(which, "shared moved from") == 0) {
		api::Gauge g = api::Gauge::new_(0);
		api::Gauge &same = g;
		api::Gauge h = std::move(g);
		std::move(same).into_level();
	} else if (std::strcmp(which, "shared moved out") == 0) {
		api::Gauge g = api::Gauge::new_(0);
		api::Gauge &same = g;
		std::move(g).into_level();
		std::move(same).into_level();
	} else if (std::strcmp(which, "lent copied") == 0) {
		static std::optional<api::Gauge> kept;
		api::watch(api::Gauge::new_(0), [](const api::Gauge &lent) {
			kept.emplace(lent);
			return lent.level();
		});
	} else if (std::strcmp(which, "value") == 0) {
		api::Counter c = api::Counter::new_("c", 0);
		c.sub(1).value();
	} else if (std::strcmp(which, "finished") == 0) {
		api::Channel channel = api::Channel::new_();
		channel.send("x");
		api::Channel_recv_future recv = channel.recv();
		api::waker waker([]() {});
		recv.poll(waker);
		recv.poll(waker);
	} else if (std::strcmp(which, "null view") == 0) {
		api::len(api::view(nullptr, 1));
	} else if (std::strcmp(which, "lent ledger") == 0) {
		// A ledger used while a draft holds its exclusive borrow.
		api::Ledger ledger = api::Ledger::new_();
		api::Draft draft = ledger.edit().value();
		ledger.published();
	} else if (std::strcmp(which, "outlived") == 0) {
		api::Meters meters = api::Meters::new_();
		api::Meter first = meters.first();
		meters.clear();
		first.count();
	} else if (std::strcmp(which, "lent moved") == 0) {
		api::Registry registry = api::Registry::new_(api::Counter::new_("c", 0));
		registry.edit([](api::borrow<api::Counter> counter) {
			api::Label label = std::move(*counter.operator->()).into_label(std::nullopt);
			return false;
		});
	} else if (std::strcmp(which, "overread") == 0) {
		struct overread {
			api::result<std::size_t, std::string> read(std::uint8_t *, std::size_t len) {
				return api::result<std::size_t, std::string>::ok(len + 1);
			}
			api::result<std::uint64_t, std::string> seek(std::int64_t, std::ios_base::seekdir) {
				return api::result<std::uint64_t, std::string>::ok(0);
			}
		} file;
		(void)api::tail(file, 1);
	} else if (std::strcmp(which, "lent kept") == 0) {
		static std::optional<api::Counter> kept;
		api::Registry registry = api::Registry::new_(api::Counter::new_("c", 0));
		registry.edit([](api::borrow<api::Counter> counter) {
			kept.emplace(std::move(*counter.operator->()));
			return false;
		});
	}
	return 0;
}

// A method of dep-fixture's `Area` at the api fixture's `Point`, which the crate header of
// `Point`'s crate defines, though `Area`'s class declares it.
int rehomed() {
	api::Area area = api::Area::new_({2, 3});
	std::printf("scaled by %llu\n", static_cast<unsigned long long>(area.scaled_Point({-3, 4}) / area.pixels()));
	return 0;
}

} // namespace

int main(int argc, char **argv) {
#ifdef _WIN32
	// The transcript's lines end as Rust's do, which text mode would turn into CRLF.
	_setmode(_fileno(stdout), _O_BINARY);
#endif
	if (argc > 1 && std::strcmp(argv[1], "futures") == 0) {
		return futures();
	}
	if (argc > 1 && std::strcmp(argv[1], "lifetimes") == 0) {
		return lifetimes();
	}
	if (argc > 1 && std::strcmp(argv[1], "rehomed") == 0) {
		return rehomed();
	}
	if (argc > 1 && std::strcmp(argv[1], "checks") == 0) {
		return checks();
	}
	if (argc > 1) {
		return broken(argv[1]);
	}
	transcript::script();
	return 0;
}
