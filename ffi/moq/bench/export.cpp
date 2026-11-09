// bench/export.c's frame writes through the generated C++ header, so the two differ by
// what the C++ wrapper adds; then the read path, each read a future polled once ready, and
// a frame's payload read.

#include <chrono>
#include <cstdio>
#include <optional>

#include "moq.hpp"

namespace {

constexpr int groups = 10000;
constexpr int frames = 100;

// Nanoseconds per frame since `start`.
double per_frame(std::chrono::steady_clock::time_point start) {
	std::chrono::duration<double, std::nano> elapsed = std::chrono::steady_clock::now() - start;
	return elapsed.count() / (groups * frames);
}

} // namespace

int main() {
	moq::broadcast_Producer broadcast = moq::broadcast_Info::new_().produce();
	moq::Timescale micro = moq::Timescale::MICRO();
	auto track = broadcast.create_track("bench", moq::track_Info::default_().with_timescale(&micro));
	if (!track) {
		return 1;
	}

	std::uint8_t payload[100] = {0};
	moq::view frame(payload, sizeof payload);
	std::uint64_t micros = 0;

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	auto start = std::chrono::steady_clock::now();
	for (int g = 0; g < groups; g++) {
		moq::group_Producer group = std::move(*track->append_group());
		for (int f = 0; f < frames; f++) {
			moq::Timestamp at = std::move(*moq::Timestamp::from_micros(micros++));
			if (!group.write_frame(&at, frame)) {
				return 2;
			}
		}
		(void)std::move(group).finish();
	}
	double stamped = per_frame(start);

	// The same, reusing one timestamp handle: what the handle costs.
	moq::Timestamp at = std::move(*moq::Timestamp::from_micros(micros));
	start = std::chrono::steady_clock::now();
	for (int g = 0; g < groups; g++) {
		moq::group_Producer group = std::move(*track->append_group());
		for (int f = 0; f < frames; f++) {
			if (!group.write_frame(&at, frame)) {
				return 3;
			}
		}
		(void)std::move(group).finish();
	}
	double reused = per_frame(start);

	// The floor: a borrow of a handle and a bool back.
	start = std::chrono::steady_clock::now();
	int closed = 0;
	for (int i = 0; i < groups * frames; i++) {
		closed += track->is_closed();
	}
	double call = per_frame(start);

	// Reads: each frame written, then read through a new future polled once, ready.
	moq::waker waker([]() {});
	moq::track_Consumer consumer = track->consume();
	auto subscribed = consumer.subscribe(std::nullopt).poll(waker);
	if (!subscribed || !*subscribed) {
		return 4;
	}
	moq::track_Subscriber subscriber = std::move(**subscribed);
	std::optional<moq::frame_Frame> last;
	start = std::chrono::steady_clock::now();
	for (int g = 0; g < groups; g++) {
		moq::group_Producer group = std::move(*track->append_group());
		auto received = subscriber.recv_group().poll(waker);
		if (!received || !*received || !received->value()) {
			return 5;
		}
		moq::group_Consumer reader = std::move(*received->value());
		for (int f = 0; f < frames; f++) {
			if (!group.write_frame(&at, frame)) {
				return 6;
			}
			auto read = reader.read_frame().poll(waker);
			if (!read || !*read || !read->value()) {
				return 7;
			}
			if (g == groups - 1 && f == frames - 1) {
				last = std::move(*read->value());
			}
		}
		(void)std::move(group).finish();
	}
	double read = per_frame(start);

	// Payload reads: the last frame's 100 bytes, owned without a copy, then freed.
	start = std::chrono::steady_clock::now();
	for (int i = 0; i < groups * frames; i++) {
		closed += last->payload_get().data()[0];
	}
	double lent = per_frame(start);

	std::printf(
	    "export (C++): %.0f ns per stamped 100 B frame, %.0f reusing the timestamp, %.1f ns per handle call, %.0f ns per frame written and read, %.0f ns per payload read%s\n",
	    stamped, reused, call, read, lent, closed ? "!" : "");
	return 0;
}
