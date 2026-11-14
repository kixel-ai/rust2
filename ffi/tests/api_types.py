# A typed caller of the api fixture's Python, which tests/fixtures.rs's `api_py_types`
# type-checks alongside it but never runs. Each list it passes in holds one alternative of
# a widened element type, or is no `list` at all, as a caller's own may be.

import array
import asyncio
import io

import api

v4s: list[tuple[str, int]] = [("127.0.0.1", 8080)]
ports: array.array[int] = api.ports(v4s)
ports = api.ports(addr for addr in v4s)

rows: list[array.array[int]] = [array.array("I", [1, 2]), array.array("I", [3])]
sums: array.array[int] = api.sums(rows)
sums = api.sums(memoryview(row) for row in rows)
floats: array.array[float] = api.boxed_floats(array.array("d", [0.5]))

names: tuple[str, ...] = ("a", "b")
peer = api.Peer(v4s[0], names)
first: str = peer.names[0]

# A reader is any binary file, or a class of its own with readinto and seek.
with open(__file__, "rb") as opened:
	tail: tuple[memoryview, int] = api.tail(opened, 4)
tail = api.tail(io.BytesIO(b"abc"), 1)


class Reader:
	def readinto(self, buffer: memoryview, /) -> int:
		return 0

	def seek(self, offset: int, whence: int = 0, /) -> int:
		return 0


tail = api.tail(Reader(), 1)


# Closures returning what Rust owns, or raising the exception of their error.
def drive(now: api.Moment, waiter: api.Waiter) -> api.Moment | None:
	if now.millis():
		raise api.StringException("stopped")
	return now.after(5)


start = api.Moment.new(0, 1000)
assert start is not None
stopped: str = asyncio.run(api.drive(start, drive))
made: str = api.made(3, lambda n: f"{n} times")
spawned: int = api.Spawned.new(lambda: 7).join()
polled: None = asyncio.run(api.Waiter.poll_future(asyncio.sleep(0)))
