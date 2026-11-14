// bench/export.c's cases through the generated C++ header, so the two differ by what the
// C++ wrapper adds. Futures are polled by hand with a waker that does nothing. Each case
// prints its wall time and the process's CPU time.

#include <algorithm>
#include <atomic>
#include <chrono>
#include <ctime>
#include <cstdio>
#include <cstdlib>
#include <optional>
#include <thread>
#include <vector>

#include "moq.hpp"

namespace {

constexpr int groups = 10000;
constexpr int frames = 100;
constexpr int n = groups * frames;
constexpr int threads = 4;
constexpr std::size_t sizes[] = {100, 1024, 16384, 65536, 1048576};

// What the loops sum, kept so the compiler cannot drop the calls.
volatile unsigned sink;

// Wall and CPU nanoseconds: a point, or a span per call.
struct span {
	double wall, cpu;
};

double clock_ns(clockid_t clock) {
	timespec ts;
	clock_gettime(clock, &ts);
	return static_cast<double>(ts.tv_sec) * 1e9 + static_cast<double>(ts.tv_nsec);
}

span now() {
	return {clock_ns(CLOCK_MONOTONIC), clock_ns(CLOCK_PROCESS_CPUTIME_ID)};
}

// Nanoseconds per call since `start`, over `calls` calls.
span per(span start, double calls = n) {
	span end = now();
	return {(end.wall - start.wall) / calls, (end.cpu - start.cpu) / calls};
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

// `call` made n times on each of `threads` threads at once, thread i on handles[i]: wall
// nanoseconds per call on one thread, and CPU nanoseconds per call.
template <class T, class F>
span concurrent(const T *const (&handles)[threads], F call, unsigned &sum) {
	std::atomic<int> ready{0};
	std::atomic<bool> go{false};
	unsigned sums[threads] = {};
	std::vector<std::thread> workers;
	for (int i = 0; i < threads; i++) {
		workers.emplace_back([&, i] {
			unsigned local = 0;
			ready.fetch_add(1);
			while (!go.load()) {
			}
			for (int j = 0; j < n; j++) {
				local += call(*handles[i]);
			}
			sums[i] = local;
		});
	}
	while (ready.load() < threads) {
	}
	span start = now();
	go.store(true);
	for (auto &worker : workers) {
		worker.join();
	}
	span end = now();
	for (unsigned s : sums) {
		sum += s;
	}
	return {(end.wall - start.wall) / n, (end.cpu - start.cpu) / (static_cast<double>(n) * threads)};
}

// Groups of 100 frames of `size` bytes written and each read back, its payload's first byte
// touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
span throughput(moq::track_Producer &track, moq::track_Subscriber &subscriber, moq::Timestamp at, std::size_t size, unsigned &sum) {
	std::size_t count = std::clamp<std::size_t>((64u << 20) / (size * frames), 3, 2000);
	std::vector<std::uint8_t> payload(size);
	auto start = now();
	for (std::size_t g = 0; g < count; g++) {
		auto group = must(track.append_group(), "append_group");
		auto reader = ready(subscriber.recv_group(), "recv_group");
		for (int f = 0; f < frames; f++) {
			must(group.write_frame(at, payload), "write_frame");
			auto read = ready(reader.read_frame(), "read_frame");
			sum += read.payload_get().data()[0];
		}
		must(std::move(group).finish(), "finish");
	}
	span ns = per(start, static_cast<double>(count * frames * size));
	return {1e3 / ns.wall, 1e3 / ns.cpu};
}

void round(bool print) {
	moq::broadcast_Producer broadcast = moq::broadcast_Info::new_().produce();
	moq::Timescale micro = moq::Timescale::MICRO();
	auto track = must(broadcast.create_track("bench", moq::track_Info::default_().with_timescale(micro)), "create_track");

	std::uint8_t bytes[100] = {0};
	moq::view frame(bytes, sizeof bytes);
	unsigned sum = 0;

	auto start = now();
	for (int i = 0; i < n; i++) {
		sum += track.is_closed();
	}
	span call = per(start);

	span getter, shared_call, own_call, shared_getter, own_getter;
	{
		auto demanded = must(track.append_group(), "append_group");
		auto demand = demanded.demand();
		start = now();
		for (int i = 0; i < n; i++) {
			sum += static_cast<unsigned>(demand.sequence());
		}
		getter = per(start);

		// The handle call and the getter from `threads` threads: on one shared handle, then
		// on a clone each, a handle of its own to the same value.
		auto is_closed = [](const moq::track_Producer &t) { return static_cast<unsigned>(t.is_closed()); };
		auto sequence = [](const moq::group_Demand &d) { return static_cast<unsigned>(d.sequence()); };
		std::vector<moq::track_Producer> tracks;
		std::vector<moq::group_Demand> demands;
		for (int i = 0; i < threads; i++) {
			tracks.push_back(track.clone());
			demands.push_back(demand.clone());
		}
		const moq::track_Producer *shared_tracks[threads], *own_tracks[threads];
		const moq::group_Demand *shared_demands[threads], *own_demands[threads];
		for (int i = 0; i < threads; i++) {
			shared_tracks[i] = &track;
			own_tracks[i] = &tracks[i];
			shared_demands[i] = &demand;
			own_demands[i] = &demands[i];
		}
		shared_call = concurrent(shared_tracks, is_closed, sum);
		own_call = concurrent(own_tracks, is_closed, sum);
		shared_getter = concurrent(shared_demands, sequence, sum);
		own_getter = concurrent(own_demands, sequence, sum);
		must(std::move(demanded).finish(), "finish");
	}

	start = now();
	for (int i = 0; i < n; i++) {
		sum += track.largest().has_value();
	}
	span record = per(start);

	start = now();
	for (int i = 0; i < n; i++) {
		(void)track.info();
	}
	span object = per(start);

	// Each frame stamped through a fresh timestamp, as a caller holding a u64 must.
	std::uint64_t micros = 0;
	start = now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		for (int f = 0; f < frames; f++) {
			auto at = must(moq::Timestamp::from_micros(micros++), "from_micros");
			must(group.write_frame(at, frame), "write_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	span stamped = per(start);

	auto at = must(moq::Timestamp::from_micros(micros), "from_micros");
	start = now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		for (int f = 0; f < frames; f++) {
			must(group.write_frame(at, frame), "write_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	span write = per(start);

	auto subscriber = must(must(track.consume().subscribe(std::nullopt).poll(noop()), "subscribe"), "subscribe");

	// Each frame written, then read through a new future ready at its first poll.
	std::optional<moq::frame_Frame> last;
	start = now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		auto reader = ready(subscriber.recv_group(), "recv_group");
		for (int f = 0; f < frames; f++) {
			must(group.write_frame(at, frame), "write_frame");
			last = ready(reader.read_frame(), "read_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	span read = per(start);

	// Each read polled before its frame is written, so it pends, then polled again.
	start = now();
	for (int g = 0; g < groups; g++) {
		auto group = must(track.append_group(), "append_group");
		auto reader = ready(subscriber.recv_group(), "recv_group");
		for (int f = 0; f < frames; f++) {
			auto reading = reader.read_frame();
			if (reading.poll(noop())) {
				std::fprintf(stderr, "bench: woken read_frame ready\n");
				std::exit(1);
			}
			must(group.write_frame(at, frame), "write_frame");
			(void)must(must(must(reading.poll(noop()), "woken read_frame"), "woken read_frame"), "woken read_frame");
		}
		must(std::move(group).finish(), "finish");
	}
	span woken = per(start);

	// The last frame's 100 bytes, owned without a copy, then freed.
	start = now();
	for (int i = 0; i < n; i++) {
		sum += last->payload_get().data()[0];
	}
	span payload = per(start);

	span mbps[std::size(sizes)];
	for (std::size_t i = 0; i < std::size(sizes); i++) {
		mbps[i] = throughput(track, subscriber, at, sizes[i], sum);
	}
	sink = sum;

	if (print) {
		const char *names[] = {"call", "getter", "shared:call", "own:call", "shared:getter", "own:getter", "record", "object", "stamped", "write", "ready", "woken", "payload"};
		span values[] = {call, getter, shared_call, own_call, shared_getter, own_getter, record, object, stamped, write, read, woken, payload};
		for (std::size_t i = 0; i < std::size(names); i++) {
			std::printf("%s\t%f\t%f\n", names[i], values[i].wall, values[i].cpu);
		}
		for (std::size_t i = 0; i < std::size(sizes); i++) {
			std::printf("mbps:%zu\t%f\t%f\n", sizes[i], mbps[i].wall, mbps[i].cpu);
		}
	}
}

} // namespace

int main() {
	round(false);
	round(true);
	return 0;
}
