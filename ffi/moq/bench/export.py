# bench/export.kt's frame writes and reads through the generated Python, so the two differ
# by what CPython and the Python wrapper add, with the calls bench/ffi/README.md compares
# BoltFFI's Python on. Each round runs twice, the first to warm CPython's caches.

import asyncio
import time

import moq

GROUPS = 1000
FRAMES = 100


def per_frame(start):
	"""Nanoseconds per frame since `start`."""
	return (time.perf_counter_ns() - start) / (GROUPS * FRAMES)


def round(loop):
	broadcast = moq.BroadcastInfo.new().produce()
	track = broadcast.create_track("bench", moq.TrackInfo.default().with_timescale(moq.Timescale.MICRO))

	# A frame of 100 zero bytes, as bench/export.c writes.
	frame = bytes(100)
	micros = 0

	# Each frame stamped through a fresh timestamp object, as a caller holding an int must.
	start = time.perf_counter_ns()
	for _ in range(GROUPS):
		group = track.append_group()
		for _ in range(FRAMES):
			group.write_frame(moq.Timestamp.from_micros(micros), frame)
			micros += 1
		group.finish()
	stamped = per_frame(start)

	# The same, reusing one timestamp object: what the object costs.
	at = moq.Timestamp.from_micros(micros)
	start = time.perf_counter_ns()
	for _ in range(GROUPS):
		group = track.append_group()
		for _ in range(FRAMES):
			group.write_frame(at, frame)
		group.finish()
	reused = per_frame(start)

	# The floor: a hold of a handle and a bool back.
	start = time.perf_counter_ns()
	closed = 0
	for _ in range(GROUPS * FRAMES):
		if track.is_closed():
			closed += 1
	call = per_frame(start)

	# A u64 getter, timed as bench/ffi/README.md timed BoltFFI's `group.sequence()`: a bound
	# method called in a loop. BoltFFI holds the GIL through the call, where the export
	# releases it so Rust may call Python back from another thread, at about 40 ns a call.
	sequence = track.append_group().demand().sequence
	start = time.perf_counter_ns()
	for _ in range(GROUPS * FRAMES):
		sequence()
	getter = per_frame(start)

	# A record returned: the track's info, its fields made as Python objects.
	start = time.perf_counter_ns()
	for _ in range(GROUPS * FRAMES):
		track.info()
	info = per_frame(start)

	# Reads: each frame written, then read through an awaitable ready at its first poll,
	# so it never suspends.
	async def ready():
		subscriber = await track.consume().subscribe(None)
		last = None
		start = time.perf_counter_ns()
		for _ in range(GROUPS):
			group = track.append_group()
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				group.write_frame(at, frame)
				last = await reader.read_frame()
			group.finish()
		return per_frame(start), last

	read, last = loop.run_until_complete(ready())

	# Woken reads: each read awaited before its frame is written, so it pends, and the
	# write's wake resumes it through the loop.
	async def woken():
		subscriber = await track.consume().subscribe(None)
		start = time.perf_counter_ns()
		for _ in range(GROUPS // 10):
			group = track.append_group()
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				pending = asyncio.ensure_future(reader.read_frame())
				await asyncio.sleep(0)
				group.write_frame(at, frame)
				await pending
			group.finish()
		return per_frame(start) * 10

	wake = loop.run_until_complete(woken())

	# A handle call inside the loop the woken reads ran on, once nothing waits on it.
	async def looped():
		start = time.perf_counter_ns()
		for _ in range(GROUPS * FRAMES):
			track.is_closed()
		return per_frame(start)

	in_loop = loop.run_until_complete(looped())

	# Payload reads: the last frame's 100 bytes, lent without a copy, then released.
	start = time.perf_counter_ns()
	for _ in range(GROUPS * FRAMES):
		with last.payload_get as payload:
			closed += payload[0]
	lent = per_frame(start)

	bang = "!" if closed != 0 else ""
	return (
		f"export (Python): {stamped:.0f} ns per stamped 100 B frame, {reused:.0f} reusing the timestamp, "
		f"{call:.1f} ns per handle call ({in_loop:.1f} in a loop), {getter:.1f} ns per u64 getter, {info:.0f} ns per record return, {read:.0f} ns per frame written and read, "
		f"{wake:.0f} ns per frame written and read after a wake, {lent:.0f} ns per payload read{bang}"
	)


loop = asyncio.new_event_loop()
round(loop)
print(round(loop))
loop.close()
