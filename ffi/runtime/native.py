# The extension `_load` loads, as type checkers see it: the runtime's types, and after
# them each glue function the Python calls by name. Every generated Python module
# declares it under `_typing.TYPE_CHECKING`, in place of the module.


class _n:
	PENDING: _object

	class Object:
		__slots__ = ()

	class Value:
		__slots__ = ()

	class Future:
		__slots__ = ()

		def poll(self, queue: _n.Queue, /) -> _object: ...

		def claim(self) -> _int: ...

		def close(self) -> None: ...

	class Queue:
		def __init__(self, loop: _asyncio.AbstractEventLoop, /) -> None: ...

		def fileno(self) -> _int: ...

		def take(self, read: _bool, /) -> _list[_int]: ...

	class Node:
		def __init__(self, queue: _n.Queue, /) -> None: ...

		def token(self) -> _int: ...

		def wake(self) -> None: ...

	class Returned:
		def wake(self) -> None: ...
