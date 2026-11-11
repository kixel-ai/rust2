# The runtime every generated Python module includes: the awaitable a future that pends
# is, the queue each event loop reads its wakes from, `Waker`, what runs an awaitable Rust
# polls, and `Mut`. Its names start
# with `_` but `Waker` and `Mut`, so no exported name clashes with them.


class _Future(_n.Future, _typing.Generic[_T]):
	"""A Rust future its first poll left pending: awaiting it polls it on the running
	event loop until it finishes. Cancelling the awaiting task frees it, which cancels it in
	Rust and loses nothing, as does dropping it unawaited. Like a coroutine, it is awaited
	once: a second `await` raises `RuntimeError`."""

	__slots__ = ()

	def __await__(self) -> _typing.Generator[_typing.Any, None, _T]:
		loop = _get_running_loop()
		w = _wakeup(loop)
		token = self.claim()
		try:
			while True:
				r = self.poll(w.queue)
				if r is not _PENDING:
					return _typing.cast(_T, r)
				parked = w.parked[token] = loop.create_future()
				yield from parked
		finally:
			w.parked.pop(token, None)
			self.close()


class _Wakeup:
	"""One event loop's wakes: Rust queues a token and writes a byte to a socket the loop
	reads, from any thread and without the GIL, and the loop resumes what each names. A wake
	during a call on the loop's own thread writes nothing: `_resume` dispatches it."""

	__slots__ = ("queue", "parked", "wakers", "pump", "__weakref__")

	def __init__(self, loop: _asyncio.AbstractEventLoop) -> None:
		self.queue = _n.Queue(loop)
		# The future each parked poll waits on, and each `Waker`'s callback, by token.
		self.parked: dict[int, _asyncio.Future[None]] = {}
		self.wakers: dict[int, _typing.Callable[[], object]] = {}
		self.pump: _asyncio.Task[None] | None = None
		fd = self.queue.fileno()
		try:
			loop.add_reader(fd, self.dispatch, True)
		except NotImplementedError:
			# A proactor loop, Windows' default, reads only through its own calls.
			sock = _socket.fromfd(fd, _socket.AF_INET if _sys.platform == "win32" else _socket.AF_UNIX, _socket.SOCK_STREAM)
			sock.setblocking(False)
			self.pump = loop.create_task(self._pump(loop, sock))

	async def _pump(self, loop: _asyncio.AbstractEventLoop, sock: _socket.socket) -> None:
		with sock:
			while True:
				await loop.sock_recv(sock, 64)
				self.dispatch(True)

	def dispatch(self, read: bool) -> None:
		loop: _asyncio.AbstractEventLoop | None = None
		for token in self.queue.take(read):
			if token < 0:
				self.wakers.pop(-token, None)
				continue
			parked = self.parked.pop(token, None)
			if parked is not None and not parked.done():
				parked.set_result(None)
			wake = self.wakers.get(token)
			if wake is not None:
				if loop is None:
					loop = _get_running_loop()
				loop.call_soon(wake)


_wakeups: _weakref.WeakKeyDictionary[_asyncio.AbstractEventLoop, _Wakeup] = _weakref.WeakKeyDictionary()


def _resume() -> _n.Queue | None:
	"""Called once a call on the running loop's thread returns having woken some of its
	futures or wakers: schedule their dispatch as the loop's next callback, never inside
	the call, and return the queue it takes from."""
	loop = _get_running_loop()
	w = _wakeups.get(loop)
	if w is None:
		return None
	loop.call_soon(w.dispatch, False)
	return w.queue


def _wakeup(loop: _asyncio.AbstractEventLoop) -> _Wakeup:
	w = _wakeups.get(loop)
	if w is None:
		w = _wakeups[loop] = _Wakeup(loop)
	return w


class Waker:
	"""Wakes a future's poller: Rust may wake it from any thread, any number of times, and
	each wake calls `wake` on the event loop it was made on, never inside Rust's wake. Wakes
	that land before the loop gets to them call it once."""

	__slots__ = ("_node",)

	def __init__(self, wake: _typing.Callable[[], object], loop: _asyncio.AbstractEventLoop | None = None) -> None:
		w = _wakeup(_get_running_loop() if loop is None else loop)
		self._node = _n.Node(w.queue)
		w.wakers[self._node.token()] = wake


class _Pollable:
	"""An awaitable Rust polls as a future, from any thread: its first poll starts it as a
	task on the event loop running where it was passed, never inside Rust's poll, and the
	task calls `ended` once it finishes. Rust closes it once it drops it, which cancels the
	task, or closes a coroutine never started."""

	__slots__ = ("_awaitable", "_loop", "_task", "_closed")

	def __init__(self, awaitable: _typing.Awaitable[None]) -> None:
		self._loop = _get_running_loop()
		self._awaitable = awaitable
		self._task: _asyncio.Future[None] | None = None
		self._closed = False

	def start(self, ended: _typing.Callable[[_asyncio.Future[None]], object]) -> None:
		self._loop.call_soon_threadsafe(self._run, ended)

	def _run(self, ended: _typing.Callable[[_asyncio.Future[None]], object]) -> None:
		if self._closed:
			return
		self._task = _asyncio.ensure_future(self._awaitable, loop=self._loop)
		self._task.add_done_callback(ended)

	def close(self) -> None:
		if not self._loop.is_closed():
			self._loop.call_soon_threadsafe(self._cancel)
		else:
			self._cancel()

	def _cancel(self) -> None:
		self._closed = True
		if self._task is not None:
			self._task.cancel()
		elif _asyncio.iscoroutine(self._awaitable):
			self._awaitable.close()


class Mut(_typing.Generic[_T]):
	"""A value a call takes `&mut`, which Rust writes back into `value` once it returns,
	errors included."""

	__slots__ = ("value",)

	def __init__(self, value: _T) -> None:
		self.value = value


class _Const:
	"""A constant a class names, made anew each time it is read, as Rust makes it."""

	__slots__ = ("get",)

	def __init__(self, get: _typing.Callable[[], object]) -> None:
		self.get = get

	def __get__(self, obj: object, cls: object = None) -> object:
		return self.get()
