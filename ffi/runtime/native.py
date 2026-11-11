# The extension `_load` loads, as type checkers see it: the runtime's types, and after
# them each glue function the Python calls by name. Every generated Python module
# declares it under `_typing.TYPE_CHECKING`, in place of the module.


class _n:
	PENDING: object

	class Object:
		__slots__ = ()

	class Future:
		__slots__ = ()

		def poll(self, queue: _n.Queue, /) -> object: ...

		def claim(self) -> int: ...

		def close(self) -> None: ...

	class Queue:
		def __init__(self, loop: _asyncio.AbstractEventLoop, /) -> None: ...

		def fileno(self) -> int: ...

		def take(self, read: bool, /) -> list[int]: ...

	class Node:
		def __init__(self, queue: _n.Queue, /) -> None: ...

		def token(self) -> int: ...

		def wake(self) -> None: ...

	class Returned:
		def wake(self) -> None: ...
