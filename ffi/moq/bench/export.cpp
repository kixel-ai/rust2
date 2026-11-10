// bench/export.c's cases through the generated C++ header, so the two differ by what the
// C++ wrapper adds. Futures are polled by hand with a waker that does nothing.

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <optional>
#include <vector>

#include "moq.hpp"

namespace {

constexpr int groups = 10000;
constexpr int frames = 100;
constexpr int n = groups * frames;
constexpr std::size_t sizes[] = {100, 1024, 16384, 65536, 1048576};

// What the loops sum, kept so the compiler cannot drop the calls.
volatile unsigned sink;

using steady = std::chrono::steady_clock;

// Nanoseconds per call since `start`, over `calls` calls.
double per(steady::time_point start, double calls = n) {
	std::chrono::duration<double, std::nano> elapsed = steady::now() - start;
	return elapsed.count() / calls;
}

template <class T>
T must(std::optional<T> value, const char *what) {
	if (!value) {
		std::fprintf(stderr, "bench: %s\n", what);
		std::exit(1);
	}
	return std::move(*value);
}

template <class T, class E>
T must(moq::result<T, E> value, const char *what) {
	if (!value) {
		std::fprintf(stderr, "bench: %s\n", what);
		std::exit(1);
	}
	return std::move(*value);
}

template <class E>
void must(moq::result<void, E> value, const char *what) {
	if (!value) {
		std::fprintf(stderr, "bench: %s\n", what);
		std::exit(1);
	}
}

const moq::waker &noop() {
	static const moq::waker waker([]() {});
	return waker;
}

// The output of a future ready at its first poll, which must be some value.
template <class F>
auto ready(F future, const char *what) {
	return must(must(must(future.poll(noop()), what), what), what);
}

// Groups of 100 frames of `size` bytes written and each read back, its payload's first byte
// touched, in about 64 MiB and at least 300 frames: megabytes a second.
double throughput(moq::track_Producer &track, moq::track_Subscriber &subscriber, const moq::Timestamp &at, std::size_t size, unsigned &sum) {
	std::size_t count = std::clamp<std::size_t>((64u << 20) / (size * frames), 3, 2000);
	std::vector<std::uint8_t> payload(size);
	auto start = steady::now();
	for (std::size_t g = 0; g < count; g++) {
		auto group = must(track.append_group(), "append_group");
		auto reader = ready(subscriber.recv_group(), "recv_group");
		for (int f = 0; f < frames; f++) {
			must(group.write_frame(&at, payload), "write_frame");
			auto read = ready(reader.read_frame(), "read_frame");
			sum += read.payload_get().data()[0];
		}
		must(std::move(group).finish(), "finish");
	}
	std::chrono::duration<double> elapsed = steady::now() - start;
	return static_cast<double>(count * frames * size) / elapsed.count() / 1e6;
}

void round(bool print) {
	moq::broadcast_Producer broadcast = moq::broadcast_Info::new_().produce();
	moq::Timescale micro = moq::Timescale::MICRO();
	auto track = must(broadcast.create_track("bench", moq::track_Info::default_().with_timescale(&micro)), "create_track");

	std::uint8_t bytes[100] = {0};
	moq::view frame(bytes, sizeof bytes);
	unsigned sum = 0;

	auto start = steady::now();
	for (int i = 0; i < n; i++) {
		sum += track.is_closed();
	}
	double call = per(start);

	double getter;
	{
		auto demanded = must(track.append_group(), "append_group");
		auto demand = demanded.demand();
		start = steady::now();
		for (int i = 0; i < n; i++) {
			sum += static_cast<unsigned>(demand.sequence());
		}
		getter = per(start);
		must(std::move(demanded).finish(), "finish");
	}

	start = steady::now();
	for (int i = 0; i < n; i++) {
		sum += track.largest().has_value();
	}
	double record = per(start);

	start = steady::now();
	for (int i = 0; i < n; i++) {
		(void)track.info();
	}
	double object = per(start);

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	std::uint64_t micros = 0;
	start = steady::now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		for (int f = 0; f < frames; f++) {
			auto at = must(moq::Timestamp::from_micros(micros++), "from_micros");
			must(group.write_frame(&at, frame), "write_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	double stamped = per(start);

	auto at = must(moq::Timestamp::from_micros(micros), "from_micros");
	start = steady::now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		for (int f = 0; f < frames; f++) {
			must(group.write_frame(&at, frame), "write_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	double write = per(start);

	auto subscriber = must(must(track.consume().subscribe(std::nullopt).poll(noop()), "subscribe"), "subscribe");

	// Each frame written, then read through a new future ready at its first poll.
	std::optional<moq::frame_Frame> last;
	start = steady::now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		auto reader = ready(subscriber.recv_group(), "recv_group");
		for (int f = 0; f < frames; f++) {
			must(group.write_frame(&at, frame), "write_frame");
			last = ready(reader.read_frame(), "read_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	double read = per(start);

	// Each read polled before its frame is written, so it pends, then polled again.
	start = steady::now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		auto reader = ready(subscriber.recv_group(), "recv_group");
		for (int f = 0; f < frames; f++) {
			auto reading = reader.read_frame();
			if (reading.poll(noop())) {
				std::fprintf(stderr, "bench: woken read_frame ready\n");
				std::exit(1);
			}
			must(group.write_frame(&at, frame), "write_frame");
			(void)must(must(must(reading.poll(noop()), "woken read_frame"), "woken read_frame"), "woken read_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	double woken = per(start);

	// The last frame's 100 bytes, owned without a copy, then freed.
	start = steady::now();
	for (int i = 0; i < n; i++) {
		sum += last->payload_get().data()[0];
	}
	double payload = per(start);

	double mbps[std::size(sizes)];
	for (std::size_t i = 0; i < std::size(sizes); i++) {
		mbps[i] = throughput(track, subscriber, at, sizes[i], sum);
	}
	sink = sum;

	if (print) {
		std::printf("call\t%f\ngetter\t%f\nrecord\t%f\nobject\t%f\nstamped\t%f\nwrite\t%f\nready\t%f\nwoken\t%f\npayload\t%f\n",
		            call, getter, record, object, stamped, write, read, woken, payload);
		for (std::size_t i = 0; i < std::size(sizes); i++) {
			std::printf("mbps:%zu\t%f\n", sizes[i], mbps[i]);
		}
	}
}

} // namespace

int main() {
	round(false);
	round(true);
	return 0;
}
