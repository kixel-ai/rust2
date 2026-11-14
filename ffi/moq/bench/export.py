# bench/export.c's cases through the generated Python, so the two differ by what CPython
# and the Python wrapper add. Each round runs twice, the first to warm CPython's caches.
# Each case prints its wall time and the process's CPU time, or under CodSpeed, runs once
# for its CPU simulation to count. Its argument is moq/bench/codspeed.h's shared library.

import asyncio
import ctypes
import sys
import threading
import time

import moq

GROUPS = 1000
FRAMES = 100
N = GROUPS * FRAMES
THREADS = 4
SIZES = [100, 1024, 16384, 65536, 1048576]


def now():
	"""Wall and process CPU nanoseconds."""
	return time.perf_counter_ns(), time.process_time_ns()


def per(start, n=N):
	"""Wall and CPU nanoseconds per call since `start`, over `n` calls."""
	wall, cpu = now()
	return (wall - start[0]) / n, (cpu - start[1]) / n


codspeed = ctypes.CDLL(sys.argv[1])
codspeed.codspeed_init.argtypes = [ctypes.c_char_p]
codspeed.codspeed_init.restype = ctypes.c_bool
codspeed.codspeed_start.argtypes = []
codspeed.codspeed_start.restype = None
codspeed.codspeed_stop.argtypes = [ctypes.c_char_p]
codspeed.codspeed_stop.restype = None
start_case = codspeed.codspeed_start
stop_case = codspeed.codspeed_stop


def begin():
	"""A case's start: now, and CodSpeed counting from here."""
	start = now()
	start_case()
	return start


def end(start, name, n=N):
	"""The case `name`'s nanoseconds per call since `start`, over `n` calls."""
	stop_case(name)
	return per(start, n)


def concurrent(calls):
	"""Each of `calls`, a bound method, made N times on a thread of its own, all at once:
	wall nanoseconds per call on one thread, and CPU nanoseconds per call."""
	ready = threading.Barrier(len(calls) + 1)
	go = threading.Event()
	failures = []

	def work(call):
		ready.wait()
		go.wait()
		try:
			for _ in range(N):
				call()
		except BaseException as error:
			failures.append(error)

	threads = [threading.Thread(target=work, args=(call,)) for call in calls]
	for thread in threads:
		thread.start()
	ready.wait()
	begin = now()
	go.set()
	for thread in threads:
		thread.join()
	# A worker that died early would otherwise report a time for fewer than N calls.
	if failures:
		raise failures[0]
	wall, cpu = per(begin)
	return wall, cpu / len(calls)


async def throughput(track, subscriber, at, size):
	"""Groups of 100 frames of `size` bytes written and each read back, its payload's first
	byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU
	second."""
	groups = min(max((64 << 20) // (size * FRAMES), 3), 2000)
	frame = bytes(size)
	name = f"throughput[{size}]".encode()
	total = 0
	start = begin()
	for _ in range(groups):
		group = track.append_group()
		reader = await subscriber.recv_group()
		for _ in range(FRAMES):
			group.write_frame(at, frame)
			read = await reader.read_frame()
			with read.payload_get as payload:
				total += payload[0]
		group.finish()
	wall, cpu = end(start, name, groups * FRAMES * size)
	return 1e3 / wall, 1e3 / cpu


def round(loop, measured):
	broadcast = moq.BroadcastInfo.new().produce()
	track = broadcast.create_track("bench", moq.TrackInfo.default().with_timescale(moq.Timescale.MICRO))
	frame = bytes(100)
	out = {}

	start = begin()
	for _ in range(N):
		track.is_closed()
	out["call"] = end(start, b"call")

	# A bound method called in a loop, as bench/ffi/README.md timed BoltFFI's
	# `group.sequence()`. BoltFFI holds the GIL through the call, where the export releases
	# it so Rust may call Python back from another thread.
	demanded = track.append_group()
	demand = demanded.demand()
	sequence = demand.sequence
	start = begin()
	for _ in range(N):
		sequence()
	out["getter"] = end(start, b"getter")

	# The handle call and the getter from THREADS threads: on one shared handle, then on a
	# clone each, a handle of its own to the same value. CodSpeed's simulation runs one thread
	# at a time, which contends for nothing, so it skips them.
	if not measured:
		out["shared:call"] = concurrent([track.is_closed] * THREADS)
		out["own:call"] = concurrent([track.clone().is_closed for _ in range(THREADS)])
		out["shared:getter"] = concurrent([sequence] * THREADS)
		out["own:getter"] = concurrent([demand.clone().sequence for _ in range(THREADS)])
	demanded.finish()

	start = begin()
	for _ in range(N):
		track.largest()
	out["record"] = end(start, b"record")

	start = begin()
	for _ in range(N):
		track.info()
	out["object"] = end(start, b"object")

	# Each frame stamped through a fresh timestamp object, as a caller holding an int must.
	micros = 0
	start = begin()
	for _ in range(GROUPS):
		group = track.append_group()
		for _ in range(FRAMES):
			group.write_frame(moq.Timestamp.from_micros(micros), frame)
			micros += 1
		group.finish()
	out["stamped"] = end(start, b"stamped")

	at = moq.Timestamp.from_micros(micros)
	start = begin()
	for _ in range(GROUPS):
		group = track.append_group()
		for _ in range(FRAMES):
			group.write_frame(at, frame)
		group.finish()
	out["write"] = end(start, b"write")

	async def reads():
		subscriber = await track.consume().subscribe(None)

		# Each frame written, then read through an awaitable ready at its first poll, so it
		# never suspends.
		last = None
		start = begin()
		for _ in range(GROUPS):
			group = track.append_group()
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				group.write_frame(at, frame)
				last = await reader.read_frame()
			group.finish()
		out["ready"] = end(start, b"ready")

		# Each read awaited before its frame is written, so it pends, and the write's wake
		# resumes it through the loop.
		start = begin()
		for _ in range(GROUPS):
			group = track.append_group()
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				pending = asyncio.ensure_future(reader.read_frame())
				await asyncio.sleep(0)
				group.write_frame(at, frame)
				await pending
			group.finish()
		out["woken"] = end(start, b"woken")

		# One writer task, with one outstanding permit, for the whole reader loop.
		start = begin()
		permits = asyncio.Queue(maxsize=1)

		async def write_loop():
			for _ in range(GROUPS):
				group = track.append_group()
				for _ in range(FRAMES):
					await permits.get()
					group.write_frame(at, frame)
				await permits.get()
				group.finish()

		writer = asyncio.create_task(write_loop())
		for _ in range(GROUPS):
			reader = await subscriber.recv_group()
			for _ in range(FRAMES):
				permits.put_nowait(None)
				await reader.read_frame()
			permits.put_nowait(None)
		await writer
		out["loop"] = end(start, b"loop")

		for size in SIZES:
			out[f"mbps:{size}"] = await throughput(track, subscriber, at, size)
		return last

	last = loop.run_until_complete(reads())

	# The last frame's 100 bytes, lent without a copy, then released.
	start = begin()
	for _ in range(N):
		with last.payload_get as payload:
			payload[0]
	out["payload"] = end(start, b"payload")
	return out


# Under CodSpeed, one round, which prints nothing: its simulation counts each case.
measured = codspeed.codspeed_init(b"ffi/moq/bench/export.py::python")
loop = asyncio.new_event_loop()
if measured:
	round(loop, True)
else:
	round(loop, False)
	for case, (wall, cpu) in round(loop, False).items():
		print(f"{case}\t{wall}\t{cpu}")
loop.close()
