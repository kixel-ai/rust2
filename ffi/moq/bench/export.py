# bench/export.c's cases through the generated Python, so the two differ by what CPython
# and the Python wrapper add. Each round runs twice, the first to warm CPython's caches.
# Each case prints its wall time and the process's CPU time.

import asyncio
import time

import moq

GROUPS = 1000
FRAMES = 100
N = GROUPS * FRAMES
SIZES = [100, 1024, 16384, 65536, 1048576]


def now():
	"""Wall and process CPU nanoseconds."""
	return time.perf_counter_ns(), time.process_time_ns()


def per(start, n=N):
	"""Wall and CPU nanoseconds per call since `start`, over `n` calls."""
	wall, cpu = now()
	return (wall - start[0]) / n, (cpu - start[1]) / n


async def throughput(track, subscriber, at, size):
	"""Groups of 100 frames of `size` bytes written and each read back, its payload's first
	byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU
	second."""
	groups = min(max((64 << 20) // (size * FRAMES), 3), 2000)
	frame = bytes(size)
	total = 0
	start = now()
	for _ in range(groups):
		group = track.append_group()
		reader = await subscriber.recv_group()
		for _ in range(FRAMES):
			group.write_frame(at, frame)
			read = await reader.read_frame()
			with read.payload_get as payload:
				total += payload[0]
		group.finish()
	wall, cpu = per(start, groups * FRAMES * size)
	return 1e3 / wall, 1e3 / cpu


def round(loop):
	broadcast = moq.BroadcastInfo.new().produce()
	track = broadcast.create_track("bench", moq.TrackInfo.default().with_timescale(moq.Timescale.MICRO))
	frame = bytes(100)
	out = {}

	start = now()
	for _ in range(N):
		track.is_closed()
	out["call"] = per(start)

	# A bound method called in a loop, as bench/ffi/README.md timed BoltFFI's
	# `group.sequence()`. BoltFFI holds the GIL through the call, where the export releases
	# it so Rust may call Python back from another thread.
	demanded = track.append_group()
	sequence = demanded.demand().sequence
	start = now()
	for _ in range(N):
		sequence()
	out["getter"] = per(start)
	demanded.finish()

	start = now()
	for _ in range(N):
		track.largest()
	out["record"] = per(start)

	start = now()
	for _ in range(N):
		track.info()
	out["object"] = per(start)

	# Each frame stamped through a fresh timestamp object, as a caller holding an int must.
	micros = 0
	start = now()
	for _ in range(GROUPS):
		group = track.append_group()
		for _ in range(FRAMES):
			group.write_frame(moq.Timestamp.from_micros(micros), frame)
			micros += 1
		group.finish()
	out["stamped"] = per(start)

	at = moq.Timestamp.from_micros(micros)
	start = now()
	for _ in range(GROUPS):
		group = track.append_group()
		for _ in range(FRAMES):
			group.write_frame(at, frame)
		group.finish()
	out["write"] = per(start)

	async def reads():
		subscriber = await track.consume().subscribe(None)

		# Each frame written, then read through an awaitable ready at its first poll, so it
		# never suspends.
		last = None
		start = now()
		for _ in range(GROUPS):
			group = track.append_group()
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				group.write_frame(at, frame)
				last = await reader.read_frame()
			group.finish()
		out["ready"] = per(start)

		# Each read awaited before its frame is written, so it pends, and the write's wake
		# resumes it through the loop.
		start = now()
		for _ in range(GROUPS):
			group = track.append_group()
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				pending = asyncio.ensure_future(reader.read_frame())
				await asyncio.sleep(0)
				group.write_frame(at, frame)
				await pending
			group.finish()
		out["woken"] = per(start)

		for size in SIZES:
			out[f"mbps:{size}"] = await throughput(track, subscriber, at, size)
		return last

	last = loop.run_until_complete(reads())

	# The last frame's 100 bytes, lent without a copy, then released.
	start = now()
	for _ in range(N):
		with last.payload_get as payload:
			payload[0]
	out["payload"] = per(start)
	return out


loop = asyncio.new_event_loop()
round(loop)
for case, (wall, cpu) in round(loop).items():
	print(f"{case}\t{wall}\t{cpu}")
loop.close()
