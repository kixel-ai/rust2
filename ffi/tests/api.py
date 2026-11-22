# The api fixture's Python test: the script fixtures/api/examples/transcript.rs runs
# natively, through the Python tests/fixtures/transcript writes from it, which calls the
# functions here that transcript.rs defines. tests/fixtures.rs compares the transcripts.
#
# With an argument, it instead checks the futures' semantics, what frees what nobody
# closed, the values only Python's types bound, how Python calls into the extension, or
# what Python code expects of a package, under a debug CPython that all of those leak
# nothing, or breaks a contract, which must fail.

import array
import asyncio
import contextlib
import copy
import dataclasses
import datetime
import gc
import io
import ipaddress
import math
import os
import struct
import sys
import sysconfig
import threading
import weakref

import api

# A free-threaded CPython runs the module with the GIL off: one that turned it back on to
# import it would run every thread of `threads` one at a time.
if sysconfig.get_config_var("Py_GIL_DISABLED"):
	assert not sys._is_gil_enabled(), "importing api turned the GIL on"

# The transcript's lines end and encode as Rust's do, which Windows' text mode and code
# page would turn into CRLF and the locale's encoding.
sys.stdout.reconfigure(encoding="utf-8", newline="\n")


class Print(api.Log):
	"""The Log callback: keeps even levels."""

	def log(self, level, line):
		print(f"log {level} {line}")
		return level % 2 == 0

	# Rust closes it once it drops the implementation.
	def close(self):
		print("freed")


class Eraser(api.ClashEraser):
	"""An eraser whose methods differ only by a trailing underscore."""

	def delete_(self, new_, new):
		return new_ * 10 + new

	def delete(self):
		return 3


class Count(api.ClashCount):
	"""A count named with a trailing underscore in Rust, whose C names join no `__`."""

	def next(self):
		return 4


class Edits(api.Undo):
	"""The Undo callback: each method returns its own digit."""

	def delete(self, n):
		return n - 1

	def free(self):
		return 2

	def type(self):
		return 3


def bound(b):
	match b.kind:
		case api.Bound.INCLUDED:
			return f"Included({b.value})"
		case api.Bound.EXCLUDED:
			return f"Excluded({b.value})"
		case api.Bound.UNBOUNDED:
			return "Unbounded"


def flag(value):
	"""A bool as Rust prints it."""
	return "true" if value else "false"


def data(view):
	"""The bytes a memoryview lends, releasing it."""
	with view:
		return bytes(view)


def text(view):
	return data(view).decode()


def debugs(values):
	return "[" + ", ".join(repr(v) for v in values) + "]"


def quoted(values):
	return "[" + ", ".join(f'"{v}"' for v in values) + "]"


def escaped(path):
	"""A path's bytes as Rust's `escape_ascii` prints them."""
	out = ""
	for b in os.fsencode(path):
		match b:
			case 0x09:
				out += "\\t"
			case 0x0D:
				out += "\\r"
			case 0x0A:
				out += "\\n"
			case 0x5C | 0x27 | 0x22:
				out += "\\" + chr(b)
			case b if 0x20 <= b < 0x7F:
				out += chr(b)
			case b:
				out += f"\\x{b:02x}"
	return out


def bits(floats, code):
	"""An array of floats' bits, as Rust's `to_bits` gives them."""
	return array.array(code, floats.tobytes()).tolist()


def numbers(what, n):
	"""Print each of `n`'s lists, its floats as their bits, as transcript.rs does."""
	for name, items in [
		("u16", n.u16s),
		("u32", n.u32s),
		("u64", n.u64s),
		("i8", n.i8s),
		("i16", n.i16s),
		("i32", n.i32s),
		("i64", n.i64s),
	]:
		print(f"{what} {name} {items.tolist()}")
	print(f"{what} f32 {bits(n.f32s, 'I')}")
	print(f"{what} f64 {bits(n.f64s, 'Q')}")
	print(f"{what} some {'None' if n.some is None else f'Some({n.some.tolist()})'}")


def millis(duration):
	"""A duration in whole milliseconds."""
	return duration.secs * 1000 + duration.nanos // 1_000_000


def socket_hex(addr):
	"""A socket address as the native script's socket_hex prints it."""
	ip = ipaddress.ip_address(addr[0]).packed.hex()
	return ":".join([ip] + [str(part) for part in addr[1:]])


def hex(data):
	return bytes(data).hex()


def path_hex(path):
	return os.fsencode(path).hex()


def seconds(duration):
	return f"{duration.secs}.{duration.nanos:09}"


def code(c):
	return ord(c)


def reversed(items):
	return items[::-1]


def f64_bits(floats):
	return bits(floats, "Q")


def ip_hex(ip):
	return ip.packed.hex()


def unspelled():
	"""A path no Rust string spells: on Unix, bytes that are not UTF-8; on Windows, a
	lone surrogate."""
	if os.name == "nt":
		return "\ud800"
	return api.stray()


def check(ok, what):
	if not ok:
		print(f"failed: {what}", file=sys.stderr)
		sys.exit(1)


def step(loop):
	"""Run the loop until what is ready has run."""
	loop.run_until_complete(asyncio.sleep(0))


def until(loop, done):
	"""Run the loop until `done()`."""

	async def wait():
		while not done():
			await asyncio.sleep(0)

	loop.run_until_complete(asyncio.wait_for(wait(), 5))



def script():
	import transcript

	transcript.script()


def channel():
	"""A channel's futures, on a loop the script steps."""
	loop = asyncio.new_event_loop()
	channel = api.Channel.new()
	wakes = 0

	def wake():
		nonlocal wakes
		wakes += 1

	waker = api.Waker(wake, loop)
	# On a loop the script steps, so it sees the receive pend, and its wake resume it.
	got = None

	async def recv():
		nonlocal got
		got = await channel.recv()

	task = loop.create_task(recv())
	step(loop)
	print(f"recv pending {flag(got is None)}")
	channel.send(b"hi")
	check(got is None, "recv never resumes inside the wake")
	loop.run_until_complete(task)
	wakes += 1
	print(f"woken {wakes}")
	print(f"recv {text(got)}")

	async def block():
		channel.send(b"")
		try:
			with await channel.recv_nonempty() as payload:
				print(f"nonempty {payload.nbytes}")
		except api.ErrorException as e:
			print(f"nonempty err {e}")
		filled = await api.fill_4(api.Batch.new(), 9) + await api.fill_2(api.pair(), 1)
		print("fill " + " ".join(map(str, filled)))
		# The send happens at the call: the future is dropped unawaited.
		channel.send_later(b"eager")
		print(f"recv {text(await channel.recv())}")
		# The send happens at the call, and the future borrows the channel and the tag.
		print(f"tagged {await channel.tagged('tag')}")
		print(f"itemized {await channel.itemized(array.array('I', [1, 2, 3]))}")
		print(f"byte_len {await api.byte_len(b'abc')}")
		channel.send(b"sized")
		payload, size = await channel.recv_sized()
		print(f"sized {text(payload)} {size}")

	loop.run_until_complete(block())
	# The waker moves into Rust, which keeps it until the send wakes it.
	channel.notify(waker)
	channel.send(b"rung")
	until(loop, lambda: wakes == 2)
	print(f"woken {wakes}")

	async def polls():
		# A poll with no async twin is a future.
		print(f"take {text(await channel.poll_take())}")
		channel.send(b"measured")
		print(f"measure {await channel.poll_measure(2)}")

	loop.run_until_complete(polls())
	# A context is made of the waker, which Rust keeps until the send wakes it.
	channel.listen(waker)
	channel.send(b"heard")
	until(loop, lambda: wakes == 3)
	print(f"woken {wakes}")
	# A returned waker wakes the waiter's task through Rust until it is dropped.
	# Wakes before the loop gets to them call back once, so it runs between them.
	waiter = api.Waiter.new(waker)
	waiter.waker().wake()
	until(loop, lambda: wakes == 4)
	returned = waiter.context()
	returned.wake()
	until(loop, lambda: wakes == 5)
	print(f"woken {wakes}")
	# Passed back, a returned waker is Rust's own again.
	channel.notify(returned)
	del waiter, returned
	channel.send(b"again")
	until(loop, lambda: wakes == 6)
	channel.clear()
	loop.close()


def drive():
	"""A kept closure: called again at the instant it asks for, then once its waiter wakes."""
	start = api.Moment.new(0, 1000)
	calls = 0

	def kept(now, waiter):
		nonlocal calls
		calls += 1
		match calls:
			case 1:
				return now.after(5)
			case 2:
				print(f"drive {now.millis() - start.millis()}")
				waiter.waker().wake()
				return None
			case _:
				raise api.StringException(f"stopped after {calls}")

	print(f"drive {asyncio.run(api.drive(start, kept))}")


def readers():
	"""Readers over bytes, whose reads fail, and that seek past `i64::MAX`."""
	for file in (io.BytesIO(b"hello world"), Broken()):
		try:
			tail, at = api.tail(file, 5)
			print(f"tail {text(tail)} {at}")
		except api.StringException as e:
			print(f"tail error {e}")
	# A seek from the start reaches the reader whole, past `i64::MAX`, as `Cursor` takes one.
	try:
		print(f"sought {api.seek_to(Far(), 2**64 - 1)}")
	except api.StringException as e:
		print(f"sought error {e}")


def streams():
	"""Streams, each iterated by `async for`: to its end, left early, and to an error, past
	which the stream goes on."""

	async def run():
		counts = [str(count) async for count in api.Countdown.new(3)]
		print(f"countdown {' '.join(counts)}")
		# A loop left early takes nothing past the item it stopped at.
		countdown = api.Countdown.new(5)
		async for count in countdown:
			if count == 4:
				break
		print(f"countdown after break {await countdown.next()}")
		# Each payload sends the next, which `!` fails and an empty one ends.
		channel = api.Channel.new()
		feed = channel.feed()
		for first in ("a", "c"):
			channel.send(first.encode())
			try:
				async for payload in feed:
					got = text(payload)
					print(f"feed {got}")
					channel.send({"a": b"b", "b": b"!"}.get(got, b""))
				print("feed end")
			except api.ErrorException as e:
				print(f"feed error {e}")

	asyncio.run(run())


async def streamed():
	"""A stream's iteration cancelled while its `next` pends loses nothing: the payload
	sent after it is the next one `next` returns."""
	channel = api.Channel.new()
	feed = channel.feed()
	got = []

	async def iterate():
		async for payload in feed:
			got.append(text(payload))

	task = asyncio.ensure_future(iterate())
	await asyncio.sleep(0)
	check(not task.done(), "a stream's iteration waits for a payload")
	task.cancel()
	try:
		await task
		check(False, "a cancelled iteration raises")
	except asyncio.CancelledError:
		pass
	channel.send(b"kept")
	check(got == [], "a cancelled iteration took nothing")
	payload = await asyncio.wait_for(feed.next(), 5)
	check(payload is not None and text(payload) == "kept", "a cancelled iteration loses nothing")
	check(aiter(feed) is feed, "a stream iterates itself")


def local_later(local):
	"""Futures of a value that is not `Send`, run on this thread."""
	loop = asyncio.new_event_loop()
	print(f"local later {loop.run_until_complete(api.local_or_new_later(local)).bump()}")
	print(f"local later fresh {loop.run_until_complete(api.local_or_new_later(None)).bump()}")
	loop.close()


def checks():
	"""What the transcript's script says only in Rust: Python's own errors and equality."""
	try:
		api.Moment()
		check(False, "Python makes no opaque value")
	except TypeError:
		pass
	# A draft borrows its ledger exclusively: the ledger refuses any other use meanwhile.
	ledger = api.Ledger.new()
	draft = ledger.edit()
	try:
		ledger.published()
		check(False, "a lent ledger refuses a read")
	except RuntimeError as e:
		check(str(e) == "api_Ledger_published: a handle is borrowed exclusively", repr(e))
	del draft
	# Each counter moves in, so a later call raises.
	counters = api.counters(["x", "y"])
	api.worth([counters[0]], [counters[1]])
	for counter in counters:
		try:
			counter.get()
			check(False, "a counter moved in is closed")
		except ValueError:
			pass
	# A record compares a buffer passed in by its numbers, as Rust does, not by its own `==`.
	held = [api.Shadowed(Elementwise("i", items), 1, 2) for items in [[1, 2], [1, 2], [1, 3]]]
	check(held[0] == held[1] and not held[0] == held[2], "a record compares a buffer by its numbers")
	# A kept closure, called on another thread after the call that took it returned.
	check(api.Spawned.new(lambda: 7).join() == 7, "a kept closure runs on another thread")


class Elementwise(array.array):
	"""A buffer whose `==` compares item by item into an object of no truth, as numpy's does.
	A native buffer, since CPython before 3.12 takes no `__buffer__` written in Python."""

	def __eq__(self, other):
		return NoTruth()


class NoTruth:
	def __bool__(self):
		raise ValueError("the truth value of an array is ambiguous")


class Broken:
	"""A reader whose reads fail, as the transcript's: each seek lands at 0."""

	def readinto(self, buffer):
		raise OSError("broken")

	def seek(self, offset, whence=0):
		return 0


class Far:
	"""A reader of nothing, which seeks anywhere from its start, as Rust's `Cursor` does."""

	def readinto(self, buffer: memoryview, /) -> int:
		return 0

	def seek(self, offset: int, whence: int = 0, /) -> int:
		return offset


def futures():
	"""The futures' semantics beyond the transcript, each checked by an assertion."""
	channel = api.Channel.new()

	# The send lands from another thread while the receive waits.
	async def threaded():
		sender = threading.Thread(target=lambda: channel.send(b"blocked"))
		sender.start()
		payload = await asyncio.wait_for(channel.recv(), 5)
		sender.join()
		check(text(payload) == "blocked", "recv returns the payload another thread sent")

	asyncio.run(threaded())

	loop = asyncio.new_event_loop()

	# Resumed on its loop, never inside the wake.
	got = None

	async def recv():
		nonlocal got
		got = text(await channel.recv())

	task = loop.create_task(recv())
	step(loop)
	check(got is None, "recv waits for a send")
	channel.send(b"then")
	check(got is None, "recv never polls inside the wake")
	loop.run_until_complete(asyncio.wait_for(task, 5))
	check(got == "then", "recv resumes with the payload")

	# A wake that lands while the future polls, before it returns pending, is kept: the
	# future polls again rather than waiting for a wake that already happened.
	check(loop.run_until_complete(asyncio.wait_for(api.yielded(7), 5)) == 7, "a wake during the poll polls again")
	check(loop.run_until_complete(api.corner_later()) == (1, 1), "a future's borrowed tuple returns whole")
	# A future's error borrowing its argument crosses as its message, as `parse_cert`'s does.
	check(loop.run_until_complete(api.parse_later("cert")) == 4, "a future with a borrowing error returns Ok")
	try:
		loop.run_until_complete(api.parse_later(""))
		check(False, "a future's borrowing error raises")
	except api.StringException as e:
		check(e.error == "malformed: ", "a future's borrowing error is its message")
	# A future returning a `Cow` that borrows its text owns it before the future ends.
	check(loop.run_until_complete(api.unescaped_later("a\\b")) == "ab", "a future's owned Cow crosses")
	check(loop.run_until_complete(api.unescaped_later("plain")) == "plain", "a future's borrowed Cow crosses")
	check(loop.run_until_complete(api.unescaped_maybe("c\\d")) == "cd", "a future's optional Cow crosses")
	check(loop.run_until_complete(api.unescaped_maybe(None)) is None, "a future's absent Cow crosses")

	# A value that is not Send stays on the thread that made it: a use elsewhere raises, and
	# its future polls on its thread alone. Freed elsewhere, it drops there at its next call.
	local = api.Local.new()
	shared = local.share()
	local.bump()
	check(shared.bump() == 2, "a shared local counts together")
	check(loop.run_until_complete(local.later()) == 3, "a local's future runs on its thread")
	check(loop.run_until_complete(api.local_later(5)) == 10, "a future that is not Send runs on its thread")
	raised = []

	def elsewhere():
		try:
			local.bump()
		except RuntimeError as e:
			raised.append(str(e))

	thread = threading.Thread(target=elsewhere)
	thread.start()
	thread.join()
	check(raised == ["api_Local_bump: a value that is not Send is used on another thread than the one that made it"], "a local used elsewhere raises")
	before = api.locals_dropped()
	held = [local]
	del local
	thread = threading.Thread(target=held.clear)
	thread.start()
	thread.join()
	check(api.locals_dropped() == before, "a local freed elsewhere waits for its thread")
	check(shared.bump() == 4 and api.locals_dropped() == before + 1, "a local freed elsewhere drops on its thread")

	# Made while a loop runs, one freed elsewhere drops on the loop's next turn, though the
	# loop calls nothing into Rust meanwhile.
	# One made with no loop running and freed elsewhere waits, until a value made while one
	# runs gives the thread its wake.
	early = [api.Local.new()]
	before = api.locals_dropped()
	thread = threading.Thread(target=early.clear)
	thread.start()
	thread.join()
	check(api.locals_dropped() == before, "a local freed elsewhere with no wake waits")

	async def idle() -> None:
		before = api.locals_dropped()
		held = [api.Local.new()]
		await asyncio.sleep(0)
		await asyncio.sleep(0)
		check(api.locals_dropped() == before + 1, "a drop queued before its thread's wake drains once it is set")
		before += 1
		freed = loop.create_future()

		def free() -> None:
			held.clear()
			loop.call_soon_threadsafe(freed.set_result, None)

		thread = threading.Thread(target=free)
		thread.start()
		await freed
		thread.join()
		check(api.locals_dropped() == before + 1, "a local freed elsewhere drops on its idle loop")

	loop.run_until_complete(idle())

	# Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	# for the next.
	called = False

	async def cancelled():
		nonlocal called
		await channel.recv()
		called = True

	task = loop.create_task(cancelled())
	step(loop)
	channel.send(b"kept")
	task.cancel()
	until(loop, task.done)
	check(not called and task.cancelled(), "a cancelled receive never resumes")
	payload = loop.run_until_complete(asyncio.wait_for(channel.recv(), 5))
	check(text(payload) == "kept", "a cancelled receive loses nothing")

	# An object whose last reference goes while a call awaits on it is freed once the
	# call ends.
	other = api.Channel.new()
	sender = other.clone()
	task = loop.create_task(other.recv())
	step(loop)
	del other
	sender.send(b"held")
	got = loop.run_until_complete(asyncio.wait_for(task, 5))
	check(text(got) == "held", "the call awaiting on an object holds it")

	# An awaitable from Rust is awaited once, as a coroutine is.
	later = channel.send_later(b"once")

	async def wait(awaitable):
		return await awaitable

	loop.run_until_complete(asyncio.wait_for(wait(later), 5))
	try:
		loop.run_until_complete(wait(later))
		check(False, "a second await raises")
	except RuntimeError:
		pass
	check(text(loop.run_until_complete(asyncio.wait_for(channel.recv(), 5))) == "once", "the send landed once")

	# Python run while a closing future is freed, as by its drop's callback, finds it ended.
	class Polls(api.Log):
		def log(self, level, line):
			return True

		def close(self):
			try:
				later.poll(api._n.Queue(loop))
				check(False, "a future polled while it is freed raises")
			except ValueError:
				pass

	later = api.log_later(Polls())
	later.close()

	# A waker woken from another thread calls back on its loop, once for wakes that land
	# before the loop gets to them.
	wakes = []
	waker = api.Waker(lambda: wakes.append(threading.get_ident()), loop)
	channel.notify(waker)
	sender = threading.Thread(target=lambda: channel.send(b"rung"))
	sender.start()
	sender.join()
	until(loop, lambda: wakes)
	check(wakes == [threading.get_ident()], "a waker calls back on its loop's thread")
	loop.run_until_complete(asyncio.wait_for(on_loop(), 5))
	loop.run_until_complete(asyncio.wait_for(foreign(), 5))
	loop.run_until_complete(asyncio.wait_for(polled(), 5))
	loop.run_until_complete(asyncio.wait_for(kept(), 5))
	loop.run_until_complete(asyncio.wait_for(streamed(), 5))
	loop.close()
	reading()
	print("futures ok")


async def polled():
	"""A poll of a waiter, made of each poll's waker, of an awaitable Rust owns and lends it
	pinned: closed once it finishes, or once the future is dropped first."""
	gate = asyncio.get_running_loop().create_future()
	ran = []

	async def lent():
		await gate
		ran.append("lent")

	task = asyncio.ensure_future(api.Waiter.poll_future(lent()))
	for _ in range(3):
		await asyncio.sleep(0)
	check(not task.done() and not ran, "a polled future waits on the awaitable it owns")
	gate.set_result(None)
	await task
	check(ran == ["lent"], "a polled future finishes once its awaitable does")

	cancelled = []

	async def never():
		try:
			await asyncio.get_running_loop().create_future()
		except asyncio.CancelledError:
			cancelled.append(True)
			raise

	try:
		await asyncio.wait_for(api.Waiter.poll_future(never()), 0.05)
		check(False, "a polled future that never ends times out")
	except asyncio.TimeoutError:
		pass
	for _ in range(3):
		await asyncio.sleep(0)
	check(cancelled == [True], "dropping a polled future cancels the awaitable it owns")


class Drive:
	"""drive's closure, which only ever waits, or stops at once."""

	def __init__(self, stop):
		self.stop = stop

	def __call__(self, now, waiter):
		if self.stop:
			raise api.StringException("stopped")
		return None


async def kept():
	"""Closures Rust keeps past the call, which it lets go of once it drops them."""
	start = api.Moment.new(0, 1000)
	stopping = Drive(True)
	ref = weakref.ref(stopping)
	check(await api.drive(start, stopping) == "stopped", "a kept closure's error ends its future")
	del stopping
	gc.collect()
	check(ref() is None, "a finished future lets go of its closure")

	# Dropped before it finishes, it lets go of the closure it kept. Cancelled without
	# awaiting it, as a traceback through its frame would keep the closure.
	idle = Drive(False)
	ref = weakref.ref(idle)
	task = asyncio.ensure_future(api.drive(start, idle))
	for _ in range(3):
		await asyncio.sleep(0)
	check(not task.done(), "a drive that only waits pends")
	task.cancel()
	for _ in range(3):
		await asyncio.sleep(0)
	check(task.cancelled(), "a drive is cancelled")
	del idle, task
	gc.collect()
	check(ref() is None, "a dropped future lets go of its closure")

	# Refused at the call, before Rust takes it.
	try:
		await api.drive(start, 42)  # type: ignore[arg-type]
		check(False, "a value that is not callable is refused")
	except TypeError:
		pass

	# Called on Rust's own thread, after the call that took it returned.
	threads = []

	class Task:
		def __call__(self):
			threads.append(threading.get_ident())
			return 9

	task = Task()
	ref = weakref.ref(task)
	spawned = api.Spawned.new(task)
	del task
	check(spawned.join() == 9, "a spawned closure returns to Rust")
	gc.collect()
	check(threads != [threading.get_ident()] and ref() is None, "a spawned closure runs elsewhere, then is let go")

	# What a closure returns, Rust owns: a copy of a moment.
	later = start.after(7)
	calls = 0

	def ask(now, waiter):
		nonlocal calls
		calls += 1
		if calls == 1:
			return later
		check(now == later, "Rust's copy of a returned moment is its value")
		raise api.StringException("done")

	check(await api.drive(start, ask) == "done", "a closure's returned moment is Rust's own copy")
	check(later.millis() == 7, "a returned moment stays the caller's")


def reading():
	"""Readers Rust reads and seeks during the call."""
	data = bytes(range(256)) * 64
	file = io.BytesIO(data)
	tail, at = api.tail(file, 1000)
	check(bytes(tail) == data[-1000:] and at == len(data) - 1000, "a long tail reads in several reads")
	check(file.tell() == at, "Rust's seeks move the reader")

	with open(__file__, "rb") as opened:
		size = os.fstat(opened.fileno()).st_size
		tail, at = api.tail(opened, 10)
		check(len(tail) == 10 and at == size - 10, "a file opened for reading is a reader")

	class Seeks(io.BytesIO):
		def seek(self, offset, whence=0, /):
			if whence == os.SEEK_END:
				raise OSError("no end")
			return super().seek(offset, whence)

	try:
		api.tail(Seeks(b"abc"), 1)
		check(False, "a failed seek fails the call")
	except api.StringException as e:
		check(str(e) == "no end", "a failed seek's message crosses")

	try:
		api.tail(object(), 1)  # type: ignore[arg-type]
		check(False, "a value with no readinto is refused")
	except TypeError:
		pass


async def foreign():
	"""Awaitables Rust polls as futures, each run as a task on the loop it was passed on."""
	ran = []

	async def ready():
		ran.append("ready")

	check(await api.after(ready(), 4) == 8 and ran == ["ready"], "Rust awaits a coroutine")

	# Rust waits on a coroutine that waits, which wakes it once it ends.
	gate = asyncio.get_running_loop().create_future()

	async def later():
		await gate
		ran.append("later")

	task = asyncio.ensure_future(api.after(later(), 5))
	for _ in range(3):
		await asyncio.sleep(0)
	check(not task.done() and ran == ["ready"], "Rust waits on a coroutine that waits")
	gate.set_result(None)
	check(await task == 10 and ran == ["ready", "later"], "a coroutine's end wakes Rust")

	# Given up on before it starts, a coroutine is closed, never run.
	try:
		await api.within(0, ready())
		check(False, "within gives up on a coroutine")
	except api.ErrorException as e:
		check(str(e) == "overflow", "within gives up on a coroutine")
	for _ in range(3):
		await asyncio.sleep(0)
	check(ran == ["ready", "later"], "a coroutine Rust gave up on before it started never runs")

	# Dropped while it runs, its task is cancelled.
	cancelled = []

	async def never():
		try:
			await asyncio.get_running_loop().create_future()
		except asyncio.CancelledError:
			cancelled.append(True)
			raise

	try:
		await asyncio.wait_for(api.within(1, never()), 0.05)
		check(False, "a coroutine that never ends times out")
	except asyncio.TimeoutError:
		pass
	for _ in range(3):
		await asyncio.sleep(0)
	check(cancelled == [True], "dropping a future Rust polls cancels its task")

	# Refused at the call, before Rust takes it: what is not awaitable, and a future of
	# another loop, which Rust's first poll could not start.
	try:
		await api.after(42, 1)  # type: ignore[arg-type]
		check(False, "a value that is not awaitable is refused")
	except TypeError:
		pass
	other = asyncio.new_event_loop()
	try:
		await api.after(other.create_future(), 1)
		check(False, "a future of another loop is refused")
	except ValueError:
		pass
	other.close()



async def on_loop():
	"""Wakes during a call on the running loop's own thread, which write no byte to its
	socket and dispatch once the call returns, beside wakes from other threads, which do."""
	channel = api.Channel.new()
	got = []

	async def recv():
		got.append(text(await channel.recv()))

	async def parked():
		task = asyncio.ensure_future(recv())
		await asyncio.sleep(0)
		return task

	def signaled():
		return wakeup.queue.signaled()

	async def settle():
		for _ in range(4):
			await asyncio.sleep(0)

	task = await parked()
	wakeup = api._wakeups[asyncio.get_running_loop()]
	channel.send(b"near")
	check(not got, "a wake on the loop's thread never resumes inside the call")
	check(not signaled(), "a wake on the loop's thread writes no byte")
	await task
	check(got == ["near"], "a wake on the loop's thread resumes the receive")

	task = await parked()
	sender = threading.Thread(target=lambda: channel.send(b"far"))
	sender.start()
	sender.join()
	check(signaled(), "a wake from another thread writes a byte")
	await task
	check(got == ["near", "far"], "a wake from another thread resumes the receive")

	# Before the first await, the call's poll with no waker leaves nothing to wake.
	early = channel.recv()
	channel.send(b"early")
	check(text(await early) == "early", "a send before the first await is received")

	# Cancelled after its wake, or before it, in the same step, a receive never resumes and
	# leaves the payload.
	for send_first in (True, False):
		task = await parked()
		if send_first:
			channel.send(b"kept")
			task.cancel()
		else:
			task.cancel()
			channel.send(b"kept")
		await settle()
		check(task.cancelled() and len(got) == 2, "a cancelled receive never resumes")
		check(text(await channel.recv()) == "kept", "a cancelled receive loses nothing")
	check(not signaled(), "wakes on the loop's thread wrote no byte")

	# Wakes that land before the loop gets to them call the waker once.
	wakes = []
	waker = api.Waker(lambda: wakes.append(1))
	for _ in range(2):
		channel.notify(waker)
		channel.send(b"rung")
	check(not wakes, "a waker is never called inside the call")
	check(not signaled(), "a waker's wakes on the loop's thread, with nothing parked, write no byte")
	await settle()
	check(wakes == [1], "wakes on the loop's thread call the waker once")
	await channel.recv()

	# Many receives, each woken once from the loop's thread or another, while both wake.
	channels = [api.Channel.new() for _ in range(64)]
	for _ in range(20):
		tasks = [asyncio.ensure_future(c.recv()) for c in channels]
		await asyncio.sleep(0)
		sender = threading.Thread(target=lambda: [c.send(b"far") for c in channels[1::2]])
		sender.start()
		for c in channels[::2]:
			c.send(b"near")
		payloads = await asyncio.gather(*tasks)
		sender.join()
		expected = ["near", "far"] * (len(channels) // 2)
		check([text(p) for p in payloads] == expected, "every receive resumes once with its payload")


def owned():
	"""What frees an object or bytes: its last reference going, or the cycle collector.
	Rust's bytes cross lent, until they and every view of them are released."""
	for n in range(1023, 1026):
		payload = bytes(i % 256 for i in range(n))
		packet = api.next_packet(api.Packet(1, payload))
		check(packet.seq == 2 and data(packet.payload) == payload, f"a record's {n} bytes cross both ways")
	freed = api.dropped()
	for n in range(1023, 1026):
		view = api.token_bytes(n, n)
		check(api.dropped() == freed, f"{n} bytes are lent, not copied")
		check(bytes(view) == bytes([n % 256]) * n, f"{n} bytes read back")
		view.release()
		freed += 1
		check(api.dropped() == freed, f"releasing {n} bytes frees them")
		view.release()
		check(api.dropped() == freed, f"releasing {n} bytes again frees nothing")
		try:
			bytes(view)
			check(False, f"released {n} bytes refuse a read")
		except ValueError:
			pass

	before = api.dropped()
	api.Token.new(3)
	api.token_bytes(4, 1025)
	check(api.dropped() == before + 2, "the last reference going frees at once")

	# A view kept past what it was read from keeps Rust's buffer.
	part = api.token_bytes(7, 1025)[1:]
	packet = api.next_packet(api.Packet(1, bytes(range(256)) * 5))
	payload = packet.payload
	del packet
	check(api.dropped() == before + 2, "a view keeps Rust's buffer")
	check(bytes(part) == bytes([7]) * 1024, "a slice outlives the bytes it was cut from")
	check(bytes(payload) == bytes(range(256)) * 5, "bytes outlive their record")
	del part
	check(api.dropped() == before + 3, "the last view going frees Rust's buffer")

	cycle = [api.Token.new(5), api.token_bytes(6, 1)]
	cycle.append(cycle)
	del cycle
	gc.collect()
	check(api.dropped() == before + 5, "the cycle collector frees what a cycle held")
	forwarding()
	released_mid_call()


def released_mid_call():
	"""Rust's bytes a call reads in place stay Rust's until it returns, though the view
	passed is released while the arguments are still being read."""
	kept = []

	def parts():
		before = api.dropped()
		with api.token_bytes(9, 2048) as view:
			yield view
		kept.append(api.dropped() == before)
		yield b"x"

	before = api.dropped()
	joined = bytes(api.joined(parts()))
	check(kept == [True], "a call keeps Rust's bytes it was passed a released view of")
	check(joined == bytes([9]) * 2048 + b"x", "a released view's bytes read back")
	check(api.dropped() == before + 1, "Rust's bytes drop once the call returns")
	print("owned ok")


def forwarding():
	"""Rust's bytes, read and passed back, cross in place: whole, sliced, kept while passed
	on, and in a record. The token's drop shows no copy stood in for them: it comes once,
	only when the last view of the bytes Rust put it in goes."""
	loop = asyncio.new_event_loop()
	channel = api.Channel.new()
	before = api.dropped()
	payload = api.token_bytes(3, 64)
	channel.send(payload)
	del payload
	payload = loop.run_until_complete(channel.recv())
	channel.send(payload[1:63])
	del payload
	payload = loop.run_until_complete(channel.recv())
	check(bytes(payload) == bytes([3]) * 62, "a slice of Rust's bytes crosses as itself")
	channel.send(payload)
	kept = loop.run_until_complete(channel.recv())
	del payload
	packet = api.next_packet(api.Packet(1, kept))
	del kept
	check(api.dropped() == before and len(packet.payload) == 62, "Rust's bytes cross back without a copy")
	del packet
	check(api.dropped() == before + 1, "Rust's bytes drop once, with their last view")
	# Bytes of Python's own cross copied once, and a view of Rust's only read stays theirs.
	packet = api.next_packet(api.Packet(1, bytearray(b"abc")))
	check(bytes(packet.payload) == b"abc", "a bytearray crosses copied")
	loop.close()


def values():
	"""The values Python's types bound: integers checked against their Rust type's range,
	durations and their conversions to and from `timedelta`, which raise rather than lose
	anything, a key hashing by Rust's `Hash`, a zero passed for a `NonZero`, which raises, and on Unix, a path whose bytes are not UTF-8, which crosses as `os.fsdecode` decodes it,
	and back as it came. Also a borrowed tuple, returned whole."""

	def raises(error, f, what):
		try:
			f()
			check(False, what)
		except error:
			pass

	raises(ValueError, lambda: api.ArrayU8x4(b"abc"), "3 bytes are no [u8; 4]")
	raises(OverflowError, lambda: api.widths(2**64), "an int past u64 raises")
	raises(OverflowError, lambda: api.widths(-1), "a negative int for a u64 raises")
	raises(TypeError, lambda: api.title(1), "an int is no bool")
	raises(OverflowError, lambda: api.scaled(2**128, 1), "an int past u128 raises")
	raises(OverflowError, lambda: api.scaled(-1, 1), "a negative int for a u128 raises")

	# An object reclassed in Python is still what Rust made: its methods refuse it.
	stamp = api.Stamp.from_millis(5)
	stamp.__class__ = api.Moment
	raises(TypeError, lambda: stamp.millis(), "a Stamp reclassed as a Moment is no Moment")
	counter = api.Counter.new("r", 1)
	counter.__class__ = api.Berth
	raises(TypeError, lambda: counter.at(), "a Counter reclassed as a Berth is no Berth")

	# A list of numbers is any buffer of exactly its numbers, one dimension and contiguous,
	# and returns as an array of them.
	edges = api.Numbers.edges()
	check(type(edges.u32s) is array.array and edges.u32s.typecode == "I", "a u32 list is an array of 'I'")
	check(bits(edges.reversed().reversed().f64s, "Q") == bits(edges.f64s, "Q"), "floats cross back exactly")
	check(edges.reversed().reversed().i64s == edges.i64s, "integers cross back exactly")
	check(bits(api.boxed_floats(edges.f64s), "Q") == bits(edges.f64s, "Q"), "a NaN keeps its payload")
	raises(TypeError, lambda: api.sum_u32(array.array("i", [1])), "an i32 buffer is no u32 list")
	raises(TypeError, lambda: api.sum_u32(array.array("H", [1])), "a u16 buffer is no u32 list")
	raises(TypeError, lambda: api.sum_u32(b"\0\0\0\0"), "bytes are no u32 list")
	raises(TypeError, lambda: api.sum_u32([1]), "a list is no buffer")
	square = memoryview(struct.pack("=4I", 1, 2, 3, 4)).cast("B").cast("I", [2, 2])
	raises(TypeError, lambda: api.sum_u32(square), "a 2D buffer is no list")
	raises(BufferError, lambda: api.sum_u32(memoryview(array.array("I", [1, 2, 3, 4]))[::2]), "a strided buffer")
	check(api.sum_u32(memoryview(struct.pack("=2I", 5, 6)).cast("I")) == 11, "a memoryview cast to u32s is one")
	# The same numbers under another code of their width and kind, as numpy's may be.
	code = "L" if struct.calcsize("L") == 4 else "I"
	check(api.sum_u32(memoryview(struct.pack(f"={2}I", 5, 6)).cast(code)) == 11, "a buffer of another u32 code")
	n = api.Numbers.edges()
	n.some = None
	check(n.reversed().some is None, "a missing list is None")
	n.some = array.array("i")
	check(n.reversed().some == array.array("i"), "an empty list is not None")

	# A borrowed tuple is one C value, not one out per element.
	check(api.corner(True) == (1, 1), "a borrowed tuple returns whole")
	raises(api.ErrorException, lambda: api.corner(False), "a borrowed tuple's error raises")

	td = datetime.timedelta
	longest = api.Duration(2**64 - 1, 999_999_999)
	check(api.doubled_ttl(api.Duration(1, 5)) == api.Duration(2, 10), "a duration crosses")
	check(api.doubled_ttl(api.Duration(1, 5)) in {api.Duration(2, 10)}, "a returned duration hashes as one Python makes")
	check(api.doubled_ttl(longest) == longest, "the longest crosses")
	check(api.Duration(1, 2) < api.Duration(1, 3) < api.Duration(2), "durations order as Rust's")
	check(api.Duration.from_timedelta(td(days=1, microseconds=5)) == api.Duration(86_400, 5_000), "from a timedelta")
	check(api.Duration(86_400, 5_000).to_timedelta() == td(days=1, microseconds=5), "to a timedelta")
	check(api.Duration.from_timedelta(td.max).to_timedelta() == td.max, "timedelta's max round trips")
	raises(ValueError, lambda: api.Duration.from_timedelta(td(seconds=-1)), "a negative timedelta raises")
	raises(ValueError, lambda: api.Duration(0, 1).to_timedelta(), "a part of a microsecond raises")
	raises(OverflowError, lambda: api.Duration(2**64 - 1).to_timedelta(), "past timedelta's max raises")
	raises(ValueError, lambda: api.doubled_ttl(api.Duration(0, 10**9)), "a billion nanoseconds raises")
	raises(OverflowError, lambda: api.doubled_ttl(api.Duration(2**64)), "seconds past u64 raise")
	raises(OverflowError, lambda: api.doubled_ttl(api.Duration(-1)), "negative seconds raise")
	raises(TypeError, lambda: api.doubled_ttl(td(seconds=1)), "a timedelta is no Duration")
	raises(ValueError, lambda: api.next_hop(0), "a zero NonZero raises")
	raises(ValueError, lambda: api.farther_hop(api.PairNonZeroU16(0, 1)), "a zero NonZero field raises")
	key = api.Key.new("a")
	check(len({key, api.Key.new("a"), api.Key.new("b")}) == 2, "equal keys are one set member")
	check(hash(api.Url.parse("https://a/")) == hash(api.Url.parse("https://a/")), "a URL hashes by Rust's Hash")

	# The module serves wasm32 too: what only wasm32 has raises on the host's library.
	check(api.pointer_bits(api.platform()) == api.pointer_bits(api.Platform.Native(64)) == 64, "the host is native")
	check(api.is_family(api.Family.NATIVE), "and of the native family")
	for f, what in [
		(api.wasm_only, "api_wasm_only"),
		(lambda: api.pointer_bits(api.Platform.Wasm(32)), "api_Platform.Wasm"),
		(lambda: api.is_family(api.Family.WASM), "api_Family.Wasm"),
		(api.Page.default, "api_Page_default"),
		(lambda: repr(api.Page(size=1)), "api_Page_debug"),
		(lambda: repr(api.Booth(mark=api.Mark.of(1))), "api_Booth_debug"),
	]:
		try:
			f()
			check(False, f"{what} raises")
		except NotImplementedError as e:
			check(str(e) == f"{what}: it is not on this library's target", repr(e))

	if os.name == "posix":
		path = api.stray()
		check(os.fsencode(path) == b"\xff", "a path that is not UTF-8 keeps its bytes")
		check(api.parents([path, os.fsdecode(b"\xfe/\xff")])[1] == os.fsdecode(b"\xfe"), "and crosses back")
	check(bytes(api.Clash.String("hi").bytes()) == b"hi", "a method named as a builtin runs")
	# Names that differ only by trailing underscores, each reaching its own Rust: a name Python
	# spells as Rust does keeps it, whichever comes first.
	underscored = api.ClashUnderscored(new_=1, new=2, debug=3, default_=4)
	check(api.clash_underscored(underscored, api.ClashOp.DELETE, api.ClashChange.Set(new_=5, new=6)) == 2_561_234, "names apart by trailing underscores cross")
	check(
		api.clash_underscored_next(underscored, api.ClashChange.Set(new_=5, new=6))
		== (api.ClashUnderscored(new_=2, new=3, debug=4, default_=5), api.ClashChange.Set(new_=6, new=5)),
		"names apart by trailing underscores cross back",
	)
	check(api.clash_erased(Eraser()) == 123, "a callback's methods apart by a trailing underscore")
	check(api.ClashEscaped(new=3).delete() == 3 and api.ClashEscaped(new=3).delete_(1, 2) == 213, "methods apart by a trailing underscore")
	check(api.clash_erase_() == 2 and api.clash_erase() == 1, "functions apart by a trailing underscore")
	# Items whose Rust names end in `_`, so every C name joined to them keeps one `_`.
	notch = api.ClashNotch.new(3)
	check(notch.count() == 3 and notch.clone() == notch and hash(notch.clone()) == hash(notch), "a class named with a trailing underscore")
	check(api.ClashNotch.default() < notch, "a class named with a trailing underscore orders")
	check(str(notch) == "notch 3" and repr(notch) == "Notch_ { count: 3 }", "a class named with a trailing underscore prints")
	check(len(api.clash_notches(2)) == 2, "a list of a class named with a trailing underscore")
	check(api.clash_shifted(1, api.ClashShift.Up(2)) == 3 and api.clash_shifted(1, api.ClashShift.Down()) == 0, "an enum named with a trailing underscore")
	check(api.clash_shifted(5, api.ClashShift.Tag(2)) == 3, "a variant named as the tag")
	check(api.clash_counted(Count()) == 5, "a callback named with a trailing underscore")
	check(api.ClashStore.open_().open() == 7, "a static function and a method apart by a trailing underscore")
	print("values ok")


def calls():
	"""Calls Python makes straight into the extension, with no Python frame between, as a
	`def` of the same signature takes them: by position or keyword, raising its errors,
	with its docstring and the signature `inspect` reads."""
	import inspect

	def raises(f, message):
		try:
			f()
			check(False, message)
		except TypeError as e:
			check(str(e) == message, f"{e!r} is {message!r}")

	kinds = {
		(api.Counter, "get"): "method_descriptor",
		(api.Counter, "new"): "staticmethod",
		(api.Counter, "__repr__"): "method_descriptor",
		(api.Registry, "first_get"): "getset_descriptor",
		(api.Point, "offset"): "method_descriptor",
		(api.Channel, "send_later"): "method_descriptor",
		# An `async def` stays a coroutine function.
		(api.Gauge, "later"): "function",
	}
	for (cls, name), kind in kinds.items():
		got = type(inspect.getattr_static(cls, name)).__name__
		check(got == kind, f"{cls.__name__}.{name} is a {got}")
	check(type(api.has_text).__name__ == "builtin_function_or_method", "a function is the extension's")

	c = api.Counter.new(start=2, name="c")
	check(c.add(n=3) == 5 and c.sub(1) == 4, "a method takes its arguments by keyword")
	check(api.Point(1, 2).offset(dy=3, dx=4) == api.Point(5, 5), "a record's method takes keywords")
	check(api.has_text(note=api.Note.default()) is False, "a function takes keywords")
	check(api.Registry.new(first=c).edit(edit=lambda counter: True), "a closure passes by keyword")
	loop = asyncio.new_event_loop()
	check(loop.run_until_complete(api.halved(n=4)) == 2, "an async def takes keywords")
	channel = api.Channel.new()
	loop.run_until_complete(channel.send_later(payload=b"kw"))
	check(bytes(loop.run_until_complete(channel.recv())) == b"kw", "an eager future takes keywords")
	check(loop.run_until_complete(api.either_later("ab")) == 2, "a future owns an optional borrow")
	check(loop.run_until_complete(api.either_later(None)) == 0, "a future owns a None borrow")
	two, three = api.Counter.new("two", 2), api.Counter.new("three", 3)
	check(loop.run_until_complete(two.plus_later(three)) == 5, "a future holds an optional handle")
	check(loop.run_until_complete(two.plus_later(None)) == 2, "a future holds a None handle")
	loop.close()

	# C callers may pass no argument array at all for no arguments.
	import ctypes

	no_args = ctypes.pythonapi.PyObject_CallNoArgs
	no_args.restype, no_args.argtypes = ctypes.py_object, [ctypes.py_object]
	check(no_args(api.Note.default) == api.Note.default(), "a call with no argument array")

	raises(lambda: c.add(), "Counter.add() missing 1 required positional argument: 'n'")
	raises(lambda: c.add(1, 2), "Counter.add() takes 2 positional arguments but 3 were given")
	raises(lambda: c.add(1, n=2), "Counter.add() got multiple values for argument 'n'")
	raises(lambda: c.add(m=1), "Counter.add() got an unexpected keyword argument 'm'")
	raises(lambda: c.get(1), "Counter.get() takes 1 positional argument but 2 were given")
	raises(lambda: api.Counter.new(), "Counter.new() missing 2 required positional arguments: 'name' and 'start'")
	raises(lambda: api.has_text(1, 2), "has_text() takes 1 positional argument but 2 were given")

	check(str(inspect.signature(c.add)) == "(n)", "a bound method's signature")
	check(str(inspect.signature(api.Counter.new)) == "(name, start)", "a static method's signature")
	check(api.has_text.__doc__ == "Whether `note` has text, even empty.", "a function's docstring")
	check(api.Gauge.set.__doc__ == "Set the level each clone reads.", "a method's docstring")
	check(api.Counter.get.__doc__ is None, "a method without docs has no docstring")
	check(api.Registry.first_get.__doc__ is None, "a property without docs has no docstring")

	# A deprecated function warns at each call, as Python's own do, and still runs.
	import warnings

	with warnings.catch_warnings(record=True) as caught:
		warnings.simplefilter("always")
		knob = api.knob(api.Dial(level=3, before=0))
		dial = api.Dial(level=3, before=0).max()
		turned = api.Dial(level=3, before=0).turn(2)
		# An `async def`'s warns at its await.

		async def awaits():
			return await api.level_later(api.Dial(level=4, before=0))

		later = asyncio.run(awaits())
	warned = [(w.category, str(w.message), w.filename == __file__) for w in caught]
	check(
		warned
		== [
			(DeprecationWarning, "knob() is deprecated", True),
			(DeprecationWarning, "Dial.max() is deprecated: turn it by `LIMIT` with `Dial::turn`", True),
			(DeprecationWarning, "level_later() is deprecated: read its `level`", True),
		],
		f"deprecated calls warn at the caller: {warned}",
	)
	check(knob.level == 3 and dial == api.Dial(level=api.LIMIT, before=3), "a deprecated call runs")
	check(turned == api.Dial(level=5, before=3), "a turn")
	check(later == 4, "a deprecated future runs")
	print("calls ok")


def idioms():
	"""What Python code expects of a package: an error's exception per variant, weak
	references to objects and records, records `dataclasses` takes as its own, and an await
	off asyncio's loop refused with a message naming asyncio."""

	def raised(f):
		try:
			f()
		except Exception as e:
			return e
		check(False, f"{f} raises")

	# An enum's variants, each raised as its own subclass of the error's exception.
	c = api.Counter.new("c", 5)
	e = raised(lambda: c.add(2000))
	check(type(e) is api.ErrorException.Overflow, repr(type(e)))
	check(isinstance(e, api.ErrorException) and e.error == api.Error.Overflow(), "an overflow is the error's")
	check(type(e).__qualname__ == "ErrorException.Overflow", type(e).__qualname__)
	e = raised(lambda: c.sub(20))
	check(type(e) is api.ErrorException.Underflow and e.error == api.Error.Underflow(by=15), repr(e))
	check(not isinstance(e, api.ErrorException.Overflow), "an underflow is no overflow")
	try:
		api.divide(7, 0)
		check(False, "a division by zero raises")
	except api.ErrorException.Underflow:
		check(False, "a division by zero is no underflow")
	except api.ErrorException.Overflow:
		pass
	# Python makes the same subclass of a variant, as a closure raising it to Rust does.
	check(type(api.ErrorException(api.Error.Overflow())) is api.ErrorException.Overflow, "made by its variant")
	check(type(api.ErrorException.Underflow(api.Error.Underflow(by=1))) is api.ErrorException.Underflow, "made as itself")

	# An object that is an enum, whose fields no binding reads, raises by its variant too.
	for n, variant, text in [
		(0, api.FaultException.Busy, "busy"),
		(1, api.FaultException.Lost, "lost frame"),
		(2, api.FaultException.Late, "late by 7"),
	]:
		e = raised(lambda: api.jam(n))
		check(type(e) is variant and str(e) == text, f"{n}: {e!r} {e}")
		check(type(api.FaultException(e.error)) is variant, f"{n}: made by its variant")
	check(api.jam(3) == 3, "no fault")

	# Weak references to an object, an opaque value, a record, a variant, and a range's bound.
	for make in [
		lambda: api.Counter.new("w", 1),
		lambda: api.Stamp.from_millis(5),
		lambda: api.Point(1, 2),
		lambda: api.Error.Underflow(by=1),
		lambda: api.BoundU64(api.Bound.INCLUDED, 3),
	]:
		value = make()
		gone = []
		ref = weakref.ref(value, gone.append)
		check(ref() is value, f"{value!r} is weakly referenced")
		del value
		check(ref() is None and gone == [ref], "a weak reference dies with its referent")

	# A record is a dataclass to `dataclasses`, `copy`, and `match`, as `@dataclass` made it.
	# A subclass's fields are its record's, read first through either, and a dataclass may
	# subclass a record.
	class Hidden(api.Point):
		__match_args__ = ()

	check(dataclasses.asdict(Hidden(1, 2)) == {"x": 1, "y": 2}, "a subclass's own __match_args__")

	class Sub(api.Point):
		pass

	check(dataclasses.asdict(Sub(1, 2)) == {"x": 1, "y": 2}, "a subclass's fields")

	@dataclasses.dataclass
	class Tagged(api.Weighted):
		tag: str = ""

	tagged = Tagged(api.Score.of(1), 2, "t")
	check([f.name for f in dataclasses.fields(tagged)] == ["score", "weight", "tag"], "a dataclass subclass")
	weighted = api.Weighted(tagged.score, 2)
	check(weighted == api.Weighted(tagged.score, 2) and weighted != tagged and tagged != weighted, "== only its own class")
	p = api.Point(1, 2)
	check(dataclasses.is_dataclass(p) and [f.name for f in dataclasses.fields(api.Point)] == ["x", "y"], "fields")
	check(dataclasses.replace(p, x=5) == api.Point(5, 2) and p == api.Point(1, 2), "replace")
	check(dataclasses.asdict(api.Platform.Native(64)) == {"_0": 64}, "asdict")
	if sys.version_info >= (3, 13):
		check(copy.replace(p, y=7) == api.Point(1, 7), "copy.replace")
	check(copy.copy(p) == p and copy.copy(p) is not p, "copy")
	match api.Platform.Native(64):
		case api.Platform.Native(bits):
			check(bits == 64, "matched positionally")
		case _:
			check(False, "a variant matches its class")
	check(repr(api.Platform.Native(64)) == "Platform.Native(_0=64)", repr(api.Platform.Native(64)))
	check(p != api.Point(1, 3) and p != (1, 2) and api.Point.__eq__(p, (1, 2)) is NotImplemented, "eq")
	check(api.Platform.Native(64) != api.Platform.Wasm(64), "variants differ")
	check(api.Point.__hash__ is None, "a record is unhashable, as Rust's without Hash")

	# An await or a waker off a running asyncio loop names asyncio.
	coro = api.Local.default().later()
	e = raised(lambda: coro.send(None))
	check(type(e) is RuntimeError and "asyncio" in str(e) and "trio" in str(e), str(e))
	e = raised(lambda: api.Waker(lambda: None))
	check(type(e) is RuntimeError and "asyncio" in str(e), str(e))

	# `import *` reaches every export: a `pub fn _deprecated` too, which takes a trailing `_`
	# beside the module's own `_deprecated`, as `pub fn __all__` does beside the name Python
	# reserves, past `pub fn __all___`, which keeps its own, and each constant `__getattr__`
	# makes; and no import: what the module binds without a leading `_` but its `__future__`
	# import.
	star = {}
	exec("from api import *", star)
	del star["__builtins__"]
	check(star["_deprecated_"](1) == 2, "a pub fn named with a leading underscore")
	check(star["__all____"](1) == 3, "a pub fn named as a name Python reserves")
	check(star["__all___"](1) == 4, "a pub fn named as the one Python reserves would take")
	public = {name for name in vars(api) if not name.startswith("_")} - {"annotations"}
	expected = public | {"_deprecated_", "__all___", "__all____"} | set(api._consts)
	check(set(star) == expected, f"import * reaches the exports: {sorted(set(star) ^ expected)}")
	print("idioms ok")


def leaf():
	"""Calls proven to reach no user code keep the GIL; one that only looks like them
	releases it, which a hook Rust calls from another thread, which takes the GIL, proves:
	had the call kept it, the hook would deadlock against it."""
	plain = api.Plain.new(3, True)
	plain.n_set(4)
	check(plain.n_get == 4 and plain.on_get, "a leaf getter reads what a leaf setter wrote")
	check(plain == api.Plain.new(4, True) and plain != api.Plain.new(4, False), "a leaf equality")
	check(hash(plain) == hash(api.Plain.new(4, True)), "a leaf hash")
	plain.recount(5)
	check(plain.count() == 5, "a leaf method reads what a leaf method wrote")
	check(plain.steps() == 5, "a method that loops")
	wrapped = api.Wrapped.new(3)
	wrapped.n_set(4)
	check(wrapped.n_get == 4 and wrapped.count() == 4, "a leaf through a Deref")
	labeled = api.Labeled.new("a" * 100)
	check(labeled == api.Labeled.new("a" * 100) and labeled != api.Labeled.new("b"), "a leaf equality over a string")
	check(hash(labeled) == hash(api.Labeled.new("a" * 100)), "a leaf hash over a string")
	buffer = api.Buffer.new(100)
	buffer.blank()
	check(buffer.zeroed(), "a fill the caller sized")

	hooked = []
	api.hook(lambda: hooked.append(threading.get_ident()))
	# A derived equality over a field whose own is written by hand.
	check(api.Hooked.new(1) == api.Hooked.new(1), "a hooked equality")
	check(len(hooked) == 1 and hooked[0] != threading.get_ident(), "a hooked equality releases the GIL")
	# A hand-written equality that claims to be derived.
	check(api.Forged.new(1) == api.Forged.new(1), "a forged equality")
	check(len(hooked) == 2 and hooked[1] != threading.get_ident(), "a forged equality releases the GIL")
	# A getter of a value kept on its thread, after another thread freed one of its values:
	# the getter first drops that one, whose drop runs the hook.
	pinned = api.Pinned.new(5)
	freed = [api.Pinned.new(6)]
	thread = threading.Thread(target=freed.clear)
	thread.start()
	thread.join()
	check(len(hooked) == 2, "a value freed elsewhere waits for its thread")
	check(pinned.value_get == 5 and len(hooked) == 3, "a getter that drains releases the GIL")
	# A `Send` part it lends, whose getter and equality borrow it too, so drain the same.
	part = pinned.part()
	for use in [lambda: part.value_get == 5, lambda: part == part]:
		freed = [api.Pinned.new(6)]
		thread = threading.Thread(target=freed.clear)
		thread.start()
		thread.join()
		count = len(hooked)
		check(use() and len(hooked) == count + 1, "a use of a part that drains releases the GIL")
	count = len(hooked)
	check(api.Hooked.new(1).count() == 1 and len(hooked) == count + 1, "a hooked method")
	check(hooked[-1] != threading.get_ident(), "a hooked method releases the GIL")
	guarded = api.Guarded.new(2)
	check(guarded.n_get == 2 and guarded.count() == 2, "a hooked Deref")
	check(len(hooked) == count + 3, "a hooked Deref releases the GIL")
	api.unhook()
	del part
	del pinned
	print("leaf ok")


def threads():
	"""The runtime's shared state used from many threads at once, which a free-threaded
	CPython runs in parallel: each first use, an object's calls, its borrows, values made and
	freed, and futures on a loop per thread."""
	# Fewer rounds than a counter holds, `api::LIMIT`.
	n, rounds = 8, 100
	start = threading.Barrier(n)
	meter = api.Meter.default()
	counter = api.Counter.new("shared", 0)
	added = [0] * n
	refused = [0] * n
	failures = []

	async def wakes(channel):
		# Each payload sent from another thread while the receive waits.
		for i in range(10):
			sender = threading.Thread(target=channel.send, args=(bytes([i]),))
			sender.start()
			got = await asyncio.wait_for(channel.recv(), 5)
			sender.join()
			check(data(got) == bytes([i]), "a receive gets what another thread sent")

	def run(k):
		try:
			start.wait()
			# First uses race: interned names, an enum's members, and the runtime's types.
			check([shape for _, shape, _ in api.rows([api.Point(1, 1), api.Point(1, 2)])] == [api.Shape.SQUARE, api.Shape.CIRCLE], "an enum's members")
			for _ in range(rounds):
				# A leaf on a shared object, which no call detaches from.
				meter.add(1)
				# Borrows of one object, which Rust refuses while an exclusive one runs, and an
				# exclusive one while any other does.
				try:
					check(counter.name() == "shared", "a shared borrow")
				except RuntimeError as e:
					check(str(e) == "api_Counter_name: a handle is borrowed exclusively", repr(e))
				try:
					counter.add(1)
					added[k] += 1
				except RuntimeError as e:
					check(str(e) == "api_Counter_add: a handle is already borrowed", repr(e))
					refused[k] += 1
				own = api.Counter.new(f"own{k}", k)
				check(own.get() == k and api.only(own)[0].get() == k, "an object moved through a call")
				check(api.Point(k, 1).offset(1, 1) == api.Point(k + 1, 2), "a record")
				check(data(api.boxed_tuple(bytes([k]))[0]) == bytes([k]), "bytes Rust returned")
			asyncio.run(wakes(api.Channel.new()))
			check(api.Spawned.new(lambda: k).join() == k, "a kept closure called on another thread")
		except BaseException as e:
			failures.append(e)
			raise

	workers = [threading.Thread(target=run, args=(k,)) for k in range(n)]
	for worker in workers:
		worker.start()
	for worker in workers:
		worker.join()
	check(not failures, f"every thread finished: {failures!r}")
	check(meter.count() == n * rounds, "a leaf loses no update")
	check(counter.get() == sum(added) and sum(added) + sum(refused) == n * rounds, "each exclusive borrow either ran or was refused")
	print("threads ok")


def broken(which):
	"""Each breaks a contract, which must fail with a message naming it."""
	if sys.platform == "linux":
		# Non-dumpable, an expected abort writes no core and never starts the host's crash
		# handler. PR_SET_DUMPABLE is 4.
		import ctypes

		if ctypes.CDLL(None, use_errno=True).prctl(4, 0, 0, 0, 0) != 0:
			raise OSError(ctypes.get_errno(), "prctl")
	match which:
		case "moved":
			c = api.Counter.new("c", 0)
			c.into_label(None)
			c.get()
		case "boxed moved":
			tail = api.Node.new(1, None)
			api.Node.new(2, tail)
			tail.sum()
		case "outlived":
			meters = api.Meters.new()
			first = meters.first()
			meters.clear()
			first.count()
		case "lent moved":
			api.Registry.new(api.Counter.new("c", 0)).edit(lambda counter: counter.into_label(None) and False)
		case "lent kept":
			kept = []
			api.Registry.new(api.Counter.new("c", 0)).edit(lambda counter: kept.append(counter) or False)
			kept[0].get()
		# A thread holds the lent counter while converting an argument, which waits, the GIL
		# released, until the lend has ended; the call would then reach a freed handle.
		case "lent held":
			converting, ended = threading.Event(), threading.Event()

			class Slow:
				def __index__(self):
					converting.set()
					ended.wait()
					return 1

			def edit(counter):
				edit.thread = threading.Thread(target=counter.add, args=(Slow(),))
				edit.thread.start()
				converting.wait()
				return False

			api.Registry.new(api.Counter.new("c", 0)).edit(edit)
			ended.set()
			edit.thread.join()
		# An IPv6 address where Rust takes `SocketAddrV4`.
		case "family":
			api.nexts(("::", 1, 0, 0), ("::", 1, 0, 0))
		# An IPv6 address where Rust takes `Ipv4Addr`.
		case "ip family":
			api.next_ips(ipaddress.IPv6Address("::1"), ipaddress.IPv6Address("::1"))
		# An IPv6 address with a scope, which Rust's `IpAddr` has no room for.
		case "ip scope":
			api.flip(ipaddress.IPv6Address("fe80::1%3"))
		# A future Rust polls, cancelled by its owner rather than by Rust.
		case "foreign cancelled":

			async def cancel():
				gate = asyncio.get_running_loop().create_future()
				task = asyncio.ensure_future(api.after(gate, 1))
				await asyncio.sleep(0)
				gate.cancel()
				await task

			asyncio.run(cancel())
		case "callback threw":

			class Threw(api.Log):
				def log(self, level, line):
					raise RuntimeError("a log failed")

			api.emit(Threw(), bytes([1]))
		# A kept closure raising what is not its error, on Rust's own thread.
		case "kept threw":
			api.Spawned.new(lambda: 1 // 0).join()
		# A closure returning what Rust does not take.
		case "returned":
			api.made(1, lambda n: n)  # type: ignore[arg-type, return-value]
		# A reader keeping a slice of Rust's buffer, which would write it once it is gone.
		case "reader kept":

			class Keeps(io.BytesIO):
				def readinto(self, buffer, /):
					self.kept = buffer[1:]
					return super().readinto(buffer)

			api.tail(Keeps(b"abc"), 1)
		case _:
			raise ValueError(f"no case {which}")


def leaks():
	"""The other runs' checks, looped under a debug CPython: once two rounds warm the
	interpreter, five more leave its references and memory blocks exactly as they were."""
	check(hasattr(sys, "gettotalrefcount"), "leaks runs under a debug CPython")

	def counts():
		# Printed lines wait in the stream's buffer, and attribute lookups in the
		# interpreter's caches, until they fill.
		sys.stdout.flush()
		sys._clear_internal_caches()
		gc.collect()
		return sys.gettotalrefcount(), sys.getallocatedblocks()

	def rounds(n):
		for _ in range(n):
			for run in (script, checks, futures, owned, values, calls, idioms):
				run()

	with open(os.devnull, "w") as null, contextlib.redirect_stdout(null):
		rounds(2)
		first = counts()
		# Each measurement held is counted by the next one.
		before = counts()
		rounds(5)
		after = counts()
	held = [b - f for b, f in zip(before, first)]
	grown = [a - b for a, b in zip(after, before)]
	check(grown == held, f"five rounds left the references and blocks {before} as {after}")


match sys.argv[1:]:
	case []:
		script()
	case ["leaks"]:
		leaks()
	case ["futures"]:
		futures()
	case ["owned"]:
		owned()
	case ["values"]:
		values()
	case ["calls"]:
		calls()
	case ["checks"]:
		checks()
	case ["leaf"]:
		leaf()
	case ["idioms"]:
		idioms()
	case ["threads"]:
		threads()
	case [which]:
		broken(which)
