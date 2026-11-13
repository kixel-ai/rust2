// The runtime every generated Dart file includes, after the declarations of its C
// functions: the handles objects own, the stack a call lowers its values into, bytes and
// text across the C ABI, and the port that wakes, dropped wakers, and calls into Dart from
// other threads reach the isolate through.

/// An IP socket address, as Rust's `SocketAddr` holds it.
final class SocketAddr {
  /// Its IP, IPv4 or IPv6.
  final io.InternetAddress address;
  final int port;

  /// IPv6's flow label, zero for IPv4.
  final int flowinfo;

  /// IPv6's scope, zero for IPv4.
  final int scopeId;

  const SocketAddr(this.address, this.port, {this.flowinfo = 0, this.scopeId = 0});

  @override
  bool operator ==(Object other) =>
      other is SocketAddr &&
      other.address == address &&
      other.port == port &&
      other.flowinfo == flowinfo &&
      other.scopeId == scopeId;

  @override
  int get hashCode => Object.hash(address, port, flowinfo, scopeId);

  @override
  String toString() => address.type == io.InternetAddressType.IPv6
      ? '[${address.address}]:$port'
      : '${address.address}:$port';
}

/// A value a call reads and writes back, as Rust's `&mut` of a value: the call leaves
/// what Rust left in it, whether it returns or throws.
final class Mut<T> {
  T value;

  Mut(this.value);
}

/// What a [Cancelable] completes with once cancelled.
final class Cancelled implements Exception {
  const Cancelled();

  @override
  String toString() => 'Cancelled';
}

/// A call into Rust's async API: a [Future] of its result, which [cancel] cancels.
///
/// The call polls Rust's future at once, and again on the isolate's event loop each time
/// Rust wakes it, from any thread: a wake is a message to the isolate, so none is lost.
/// It is the platform's own future, which an `await` takes the shortest path through,
/// and one ready at its first poll is complete already.
extension type Cancelable<T>._(Future<T> _future) implements Future<T> {
  /// Cancel the call in Rust unless it finished, which loses nothing: it then completes
  /// with [Cancelled].
  void cancel() => _n.calls[_future]?.cancel();
}

/// Poll `f` once with no waker, and if it pends, again with one: its future.
Cancelable<T> _call<T>(
  ffi.Pointer<ffi.Void> f,
  void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>) free,
  bool Function(ffi.Pointer<ffi.Void>, _Waker, _Call<T>) poll,
) {
  final call = _Call<T>._(f, free, poll);
  call._step(_n.idle);
  final first = call._first;
  if (first != null) return Cancelable._(first);
  final done = call._done = dasync.Completer<T>();
  _n.calls[done.future] = call;
  // Pending: poll again with a waker, which its first poll had no use for.
  call._token = _n.listen(call._wake);
  call._waker = _n.waker(call._token);
  call._step(call._waker!);
  return Cancelable._(done.future);
}

/// A [Cancelable]'s future in Rust, until it finishes.
final class _Call<T> {
  ffi.Pointer<ffi.Void> _f;
  final void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>) _free;

  /// Poll once, completing this if it finished: whether it did.
  final bool Function(ffi.Pointer<ffi.Void>, _Waker, _Call<T>) _poll;

  /// What its first poll finished with, which no one awaits yet.
  Future<T>? _first;

  /// What completes it once its first poll pended.
  dasync.Completer<T>? _done;
  _Waker? _waker;
  int _token = 0;
  bool _polling = false;
  bool _cancelled = false;

  _Call._(this._f, this._free, this._poll);

  void _wake() {
    if (_f != ffi.nullptr) _step(_waker!);
  }

  void _step(_Waker waker) {
    _polling = true;
    final bool ready;
    try {
      ready = _poll(_f, waker, this);
    } catch (e, s) {
      _polling = false;
      _finish();
      _error(e, s);
      return;
    }
    _polling = false;
    if (ready) {
      _finish();
    } else if (_cancelled) {
      _cancel();
    }
  }

  /// Complete it with what Rust returned, from its poll: synchronously at the first, as
  /// nothing listens yet.
  void _ready(T value) {
    final done = _done;
    if (done == null) {
      _first = (dasync.Completer<T>.sync()..complete(value)).future;
    } else {
      done.complete(value);
    }
  }

  /// Complete it with the error Rust returned, from its poll: never synchronously, since a
  /// future that fails with no listener reports its error as uncaught.
  void _error(Object error, [StackTrace? stack]) {
    final done = _done;
    if (done == null) {
      _first = Future<T>.error(error, stack);
    } else {
      done.completeError(error, stack);
    }
  }

  void _finish() {
    _free(_n.home, _f);
    _f = ffi.nullptr;
    final done = _done;
    if (done != null) _n.calls.remove(done.future);
    if (_token != 0) {
      _n.forget(_token);
      _token = 0;
    }
  }

  void _cancel() {
    _finish();
    // Awaiting it is optional once cancelled.
    _done!.future.ignore();
    _done!.completeError(const Cancelled());
  }

  void cancel() {
    if (_f == ffi.nullptr || _cancelled) return;
    _cancelled = true;
    if (!_polling) _cancel();
  }
}

final class _Buf extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;
  @ffi.Size()
  external int len;
  external ffi.Pointer<ffi.Void> owner;
}

final class _U128 extends ffi.Struct {
  @ffi.Uint64()
  external int lo;
  @ffi.Uint64()
  external int hi;
}

final class _Duration extends ffi.Struct {
  @ffi.Uint64()
  external int secs;
  @ffi.Uint32()
  external int nanos;
}

final class _Addr extends ffi.Struct {
  @ffi.Array(16)
  external ffi.Array<ffi.Uint8> ip;
  @ffi.Uint16()
  external int port;
  @ffi.Bool()
  external bool v6;
  @ffi.Uint32()
  external int flowinfo;
  @ffi.Uint32()
  external int scope_id;
}

final class _Waker extends ffi.Struct {
  external ffi.Pointer<ffi.Void> wake;
  external ffi.Pointer<ffi.Void> drop;
  external ffi.Pointer<ffi.Void> data;
}

/// A future Rust polls, which Dart runs.
final class _ForeignFuture extends ffi.Struct {
  external ffi.Pointer<ffi.Void> poll;
  external ffi.Pointer<ffi.Void> free;
  external ffi.Pointer<ffi.Void> data;
}

/// A reader Rust reads and seeks during a call, which Dart implements.
final class _ForeignReader extends ffi.Struct {
  external ffi.Pointer<ffi.Void> read;
  external ffi.Pointer<ffi.Void> seek;
  external ffi.Pointer<ffi.Void> data;
}

/// Where a [SeekableReader.seek] counts its offset from, as Rust's `SeekFrom` does.
enum SeekFrom { start, current, end }

/// Bytes Rust reads and seeks, as its `Read + Seek`: lent for the call it is passed to,
/// which Rust calls only during that call, on the isolate's thread. An [Exception] either
/// method throws reaches Rust as an `io::Error` of its text; anything else it throws
/// aborts.
abstract interface class SeekableReader {
  /// `file`, read and seeked through its synchronous methods.
  factory SeekableReader.file(io.RandomAccessFile file) = _FileReader;

  /// Read up to `buffer.length` bytes into `buffer`, returning how many: 0 only at the end.
  int readInto(typed.Uint8List buffer);

  /// Move to `offset` from `from`, returning the position from the start. From
  /// [SeekFrom.start], `offset` and the position are a `u64`'s 64 bits, as every `u64` in
  /// Dart is, so one past `i64`'s range is negative.
  int seek(int offset, SeekFrom from);
}

final class _FileReader implements SeekableReader {
  final io.RandomAccessFile _file;

  _FileReader(this._file);

  @override
  int readInto(typed.Uint8List buffer) => _file.readIntoSync(buffer);

  @override
  int seek(int offset, SeekFrom from) {
    // A `u64` past `i64`'s range, as a file takes no position past it.
    if (from == SeekFrom.start && offset < 0) {
      throw io.FileSystemException('a seek past the largest position a file takes', _file.path);
    }
    final base = switch (from) {
      SeekFrom.start => 0,
      SeekFrom.current => _file.positionSync(),
      SeekFrom.end => _file.lengthSync(),
    };
    final at = base + offset;
    // As Rust's `File` refuses one.
    if (at < 0) throw io.FileSystemException('a seek before the start', _file.path);
    _file.setPositionSync(at);
    return at;
  }
}

/// What Rust's read of a [SeekableReader] calls, on the isolate's thread. The reader fills
/// a list of Dart's own, copied into Rust's buffer after: a view of Rust's buffer would
/// outlive the read in a reader that kept it, and Rust frees the buffer when it likes.
bool _readerRead(
  ffi.Pointer<ffi.Void> data,
  ffi.Pointer<ffi.Uint8> buf,
  int len,
  ffi.Pointer<ffi.Size> n,
  ffi.Pointer<_Buf> err,
) {
  try {
    final owned = typed.Uint8List(len);
    final got = (_n.lent[data.address] as SeekableReader).readInto(owned);
    // Rust refuses a count past `len`, which no copy then reaches.
    if (got > 0 && got <= len) buf.asTypedList(len).setRange(0, got, owned);
    n.value = got;
    return true;
  } on Exception catch (e) {
    _n.setBuf(err, _n.copied(e.toString()));
    return false;
  } catch (e, s) {
    _n.abort("a reader's read", e, s);
  }
}

/// What Rust's seek of a [SeekableReader] calls, on the isolate's thread.
bool _readerSeek(
  ffi.Pointer<ffi.Void> data,
  int whence,
  int offset,
  ffi.Pointer<ffi.Uint64> position,
  ffi.Pointer<_Buf> err,
) {
  try {
    position.value = (_n.lent[data.address] as SeekableReader).seek(offset, SeekFrom.values[whence]);
    return true;
  } on Exception catch (e) {
    _n.setBuf(err, _n.copied(e.toString()));
    return false;
  } catch (e, s) {
    _n.abort("a reader's seek", e, s);
  }
}

/// A waker Rust returned, which its own drop releases once the garbage collector finds
/// this unreachable.
final class _Returned implements ffi.Finalizable {
  final void Function(ffi.Pointer<ffi.Void>) _wake;
  final ffi.Pointer<ffi.Void> _data;

  _Returned(_Waker raw)
      : _wake = raw.wake.cast<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>().asFunction(),
        _data = raw.data {
    _n.finalizer(raw.drop).attach(this, _data);
  }

  void call() => _wake(_data);
}

/// A callback's table and data, as each trait's struct is.
final class _Callback extends ffi.Struct {
  external ffi.Pointer<ffi.Void> vtable;
  external ffi.Pointer<ffi.Void> data;
}

/// What an object owns: its handle, freed once closed and no call holds it, or once the
/// garbage collector finds it unreachable. A lent one's Rust frees.
abstract base class _Object implements ffi.Finalizable {
  final ffi.Pointer<ffi.Void> _raw;

  /// The owner's hold until closed, and one for each call.
  int _holds = 1;
  bool _closed = false;
  final bool _owned;

  /// The garbage collector's hold of an owned one's handle, until freed.
  ffi.Pointer<ffi.Void> _own = ffi.nullptr;

  _Object(this._raw, this._owned);

  String get _type;

  /// Free the handle and detach its finalizer.
  void _free();

  ffi.Pointer<ffi.Void> _hold() {
    if (_closed) throw StateError('$_type is closed');
    _holds++;
    return _raw;
  }

  void _release() {
    if (--_holds == 0 && _owned) _free();
  }

  /// Free the handle once no call holds it, rather than once the garbage collector finds
  /// this unreachable: a call after it throws [StateError].
  void close() {
    if (_closed) return;
    _closed = true;
    _release();
  }
}

/// Native memory each call lowers its values into, as a stack: a call takes what it
/// needs from the top and gives it back once it returns, so a call Rust makes into Dart
/// meanwhile takes and gives back above it.
final class _Stack implements ffi.Allocator {
  static const _size = 1 << 16;
  final ffi.Pointer<ffi.Uint8> _base = _dartAlloc(_size).cast();
  int _top = 0;

  /// What did not fit, freed as the stack falls back below it.
  final List<ffi.Pointer<ffi.Void>> _big = [];

  /// Where the stack stands, to [reset] it to.
  int mark() => _top | (_big.length << 32);

  void reset(int mark) {
    _top = mark & 0xffffffff;
    final big = mark >> 32;
    while (_big.length > big) {
      _dartFree(_big.removeLast());
    }
  }

  @override
  ffi.Pointer<T> allocate<T extends ffi.NativeType>(int byteCount, {int? alignment}) {
    final align = alignment ?? 8;
    final at = (_top + align - 1) & ~(align - 1);
    if (at + byteCount <= _size) {
      _top = at + byteCount;
      return (_base + at).cast();
    }
    final big = _dartAlloc(byteCount == 0 ? 1 : byteCount);
    _big.add(big);
    return big.cast();
  }

  @override
  void free(ffi.Pointer<ffi.NativeType> pointer) {}
}

/// What one call lowers into and holds: its share of the stack, the objects it holds,
/// and the closures it lends, all given back once it returns.
final class _Arena implements ffi.Allocator {
  final _Stack _stack;
  final int _mark;
  _Object? _held;
  List<_Object>? _more;
  List<int>? _lent;

  /// The buffers it gave Rust to take, freed unless the call took them.
  List<_Buf>? _given;

  /// The lent bytes it passed a view of, kept reachable until the call returns.
  List<typed.Uint8List>? _viewed;

  _Arena() : _stack = _n.stack, _mark = _n.stack.mark();

  @override
  ffi.Pointer<T> allocate<T extends ffi.NativeType>(int byteCount, {int? alignment}) =>
      _stack.allocate(byteCount, alignment: alignment);

  @override
  void free(ffi.Pointer<ffi.NativeType> pointer) {}

  /// `object`'s handle, held until the call returns: a closed one throws.
  ffi.Pointer<ffi.Void> hold(_Object object) {
    final raw = object._hold();
    if (_held == null) {
      _held = object;
    } else {
      (_more ??= []).add(object);
    }
    return raw;
  }

  ffi.Pointer<ffi.Void> holdOrNull(_Object? object) => object == null ? ffi.nullptr : hold(object);

  /// `f` lent to Rust for the call as the relay data that calls `trampoline` with it.
  ffi.Pointer<ffi.Void> lend(Object f, ffi.Pointer<ffi.Void> trampoline) {
    final token = _n.next();
    _n.lent[token] = f;
    final relay = _dartRelay(_n.home, trampoline, token);
    (_lent ??= []).addAll([token, relay.address]);
    return relay;
  }

  /// Note that the call returned, so Rust took each buffer it was given.
  void took() => _given = null;

  /// Give back what the call took.
  void end() {
    final given = _given;
    if (given != null) {
      for (final buf in given) {
        _bufFree(buf);
      }
    }
    _viewed = null;
    final lent = _lent;
    if (lent != null) {
      for (var i = 0; i < lent.length; i += 2) {
        _n.lent.remove(lent[i]);
        _dartRelayFree(ffi.Pointer.fromAddress(lent[i + 1]));
      }
    }
    _held?._release();
    final more = _more;
    if (more != null) {
      for (final object in more) {
        object._release();
      }
    }
    _stack.reset(_mark);
  }

  /// `bytes`, never a null pointer, so an empty one is told apart from none. Where Rust
  /// [take]s them, a buffer it owns: a second owner of bytes Rust lent, else from
  /// [_takeFrom] bytes a copy into a buffer of its own. Else a view: of bytes Rust lent, in
  /// place, else of a copy here, which Rust copies again where it keeps them.
  void bytes(_Buf dst, typed.Uint8List? bytes, bool take) {
    if (bytes == null) {
      dst.ptr = ffi.nullptr;
      dst.len = 0;
      dst.owner = ffi.nullptr;
      return;
    }
    // Only a list longer than [_lend] can be Rust's.
    final lent = bytes.length > _lend ? _n.lentBytes[bytes] : null;
    if (lent != null) {
      dst.ptr = ffi.Pointer.fromAddress(lent.ptr);
      dst.len = bytes.length;
      dst.owner = ffi.nullptr;
      // Reachable through the call, so its finalizer cannot free the owner under Rust,
      // even while [_bufClone] shares it.
      (_viewed ??= []).add(bytes);
      if (take) {
        dst.owner = ffi.Pointer.fromAddress(lent.owner);
        _give(dst, _bufClone(dst));
      }
      return;
    }
    if (take && bytes.length >= _takeFrom) {
      final buf = _dartBuf(bytes.length);
      buf.ptr.asTypedList(bytes.length).setAll(0, bytes);
      _give(dst, buf);
      return;
    }
    final ptr = this<ffi.Uint8>(bytes.isEmpty ? 1 : bytes.length);
    ptr.asTypedList(bytes.length).setAll(0, bytes);
    dst.ptr = ptr;
    dst.len = bytes.length;
    dst.owner = ffi.nullptr;
  }

  /// `text`'s UTF-8, copied: into a buffer Rust owns where it [take]s it, from [_takeFrom]
  /// bytes.
  void string(_Buf dst, String? text, bool take) {
    if (text == null) {
      bytes(dst, null, take);
      return;
    }
    final units = text.codeUnits;
    final n = units.length;
    // ASCII, the common case, needs no encoder.
    var ascii = true;
    for (var i = 0; i < n; i++) {
      if (units[i] >= 0x80) {
        ascii = false;
        break;
      }
    }
    if (!ascii) {
      bytes(dst, convert.utf8.encode(text), take);
      return;
    }
    if (take && n >= _takeFrom) {
      final buf = _dartBuf(n);
      buf.ptr.asTypedList(n).setAll(0, units);
      _give(dst, buf);
      return;
    }
    final ptr = this<ffi.Uint8>(n == 0 ? 1 : n);
    ptr.asTypedList(n).setAll(0, units);
    dst.ptr = ptr;
    dst.len = n;
    dst.owner = ffi.nullptr;
  }
}

/// Dart's own bytes Rust takes from this long are copied into a buffer Rust allocates, and
/// shorter ones onto the arena's stack, which Rust copies again: on the binding benchmark,
/// the call to allocate Rust's buffer costs more than the second copy of 100 bytes, and
/// less than that of 1 KiB.
const _takeFrom = 1024;

extension on _Arena {
  /// Write `buf`, Rust's own, to `dst`, freed unless the call takes it.
  void _give(_Buf dst, _Buf buf) {
    dst.ptr = buf.ptr;
    dst.len = buf.len;
    dst.owner = buf.owner;
    (_given ??= []).add(dst);
  }
}

/// Where a list of bytes Rust lent reads them, and their owner.
final class _Lent {
  final int ptr;
  final int owner;

  _Lent(this.ptr, this.owner);
}

/// The isolate's share of the runtime: checked against the library before any other
/// call, then its stack, its port, and what Rust holds of its Dart values by token.
final _n = _Native();

final class _Native {
  late final _Stack stack = _Stack();
  late final isolate.RawReceivePort _port;
  late final ffi.Pointer<ffi.Void> home;
  final _Waker idle = ffi.Struct.create<_Waker>();

  /// What a token's wake calls, and its drop.
  final Map<int, void Function()> _wakes = {};
  final Map<int, void Function()> _drops = {};

  /// The closures lent for a call, and the implementations Rust holds, by token.
  final Map<int, Object> lent = {};

  /// Each list of bytes Rust lent, which passes back to Rust in place.
  final lentBytes = Expando<_Lent>();

  /// The calls pending in Rust, by their future.
  final Map<Future<Object?>, _Call<Object?>> calls = {};
  int _token = 0;

  /// The futures pending and the wakers Rust holds, which keep the isolate alive.
  int _keep = 0;

  _Native() {
    if (!_contract()) {
      throw StateError('$_file and the library are from different builds');
    }
    if (!_dartInit(ffi.NativeApi.initializeApiDLData)) {
      throw StateError("$_file: the library was built for another major version of Dart's API");
    }
    _port = isolate.RawReceivePort(_receive, _file)..keepIsolateAlive = false;
    home = _dartHome(_port.sendPort.nativePort, ffi.Pointer.fromFunction<ffi.Void Function(ffi.Int64)>(_dropped));
  }

  /// Make sure the contract was checked: only a call that touches nothing else of the
  /// runtime calls it.
  void check() {}

  int next() => ++_token;

  void _receive(Object? message) {
    final m = message as int;
    final token = m >> 2;
    switch (m & 3) {
      case 0:
        _wakes[token]?.call();
      case 1:
        _drops.remove(token)?.call();
      default:
        _dartRun(home);
    }
  }

  void _hold(int by) {
    _keep += by;
    _port.keepIsolateAlive = _keep > 0;
  }

  /// A token whose wakes call `wake` until forgotten, keeping the isolate alive.
  int listen(void Function() wake) {
    final token = next();
    _wakes[token] = wake;
    _hold(1);
    return token;
  }

  void forget(int token) {
    if (_wakes.remove(token) != null) _hold(-1);
  }

  /// The waker of `token`, whose wakes reach the isolate.
  _Waker waker(int token) => _dartWaker(home, token);

  /// The finalizer calling each C drop function, by its address.
  final Map<int, ffi.NativeFinalizer> _finalizers = {};

  ffi.NativeFinalizer finalizer(ffi.Pointer<ffi.Void> drop) =>
      _finalizers.putIfAbsent(drop.address, () => ffi.NativeFinalizer(drop.cast()));

  /// A waker Rust returned, as a function waking it through Rust until the garbage
  /// collector drops it.
  void Function() returned(_Waker raw) => _Returned(raw).call;

  /// `wake` as a waker Rust holds until it drops it, calling it on the isolate's event
  /// loop.
  _Waker wakerOf(void Function() wake) {
    final token = listen(wake);
    _drops[token] = () => forget(token);
    return waker(token);
  }

  /// `future` as a future Rust polls until it completes. Rust's free cancels it if it is a
  /// [Cancelable] still running, whose [Cancelled] then completes it. Any other error
  /// aborts, as Rust has nowhere to raise it.
  _ForeignFuture future(Future<void> future) {
    final token = next();
    final f = _dartFuture(home, token);
    final data = f.data;
    _drops[token] = () => calls[future]?.cancel();
    future.then<void>(
      (_) => _dartFutureDone(data, false),
      onError: (Object e, StackTrace s) {
        // Rust's free shows here at once, before its message reaches the isolate.
        if (!_dartFutureDone(data, true)) abort('a future given to Rust', e, s);
      },
    );
    return f;
  }

  /// The trait `what`'s two tables, made once: Rust calls through the first, whose `free`
  /// tells the isolate it dropped an implementation, and whose relays call the second's
  /// trampolines on the isolate's thread.
  (ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>, String) tables(
    String what,
    List<ffi.Pointer<ffi.Void>> relays,
    List<ffi.Pointer<ffi.Void>> trampolines,
  ) {
    final n = relays.length + 1;
    final ptr = _dartAlloc(2 * n * ffi.sizeOf<ffi.Pointer<ffi.Void>>()).cast<ffi.Pointer<ffi.Void>>();
    ptr[0] = relayDrop.cast();
    ptr[n] = ffi.nullptr;
    for (var i = 0; i < relays.length; i++) {
      ptr[i + 1] = relays[i];
      ptr[n + i + 1] = trampolines[i];
    }
    return (ptr.cast(), (ptr + n).cast(), what);
  }

  /// `implementation` given to Rust through `tables`, until Rust drops it, which calls its
  /// `close`.
  _Callback give(
    (ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>, String) tables,
    Object implementation,
    void Function() close,
  ) {
    final token = next();
    lent[token] = implementation;
    _drops[token] = () {
      lent.remove(token);
      try {
        close();
      } catch (e, s) {
        abort('${tables.$3}::close', e, s);
      }
    };
    return ffi.Struct.create<_Callback>()
      ..vtable = tables.$1
      ..data = _dartRelay(home, tables.$2, token);
  }

  /// What Rust calls once it drops an implementation or a kept closure.
  ffi.Pointer<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>> get relayDrop =>
      ffi.Native.addressOf<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(_dartRelayDrop);

  /// `f` given to Rust as the relay data that calls `trampoline` with it, which Rust keeps
  /// until it frees it with [relayDrop]: Dart holds `f` until then.
  ffi.Pointer<ffi.Void> keep(Object f, ffi.Pointer<ffi.Void> trampoline) {
    final token = next();
    lent[token] = f;
    _drops[token] = () => lent.remove(token);
    return _dartRelay(home, trampoline, token);
  }

  /// The table of a [SeekableReader]'s trampolines, `read`'s then `seek`'s, made once.
  late final ffi.Pointer<ffi.Void> _readers = () {
    final table = _dartAlloc(2 * ffi.sizeOf<ffi.Pointer<ffi.Void>>()).cast<ffi.Pointer<ffi.Void>>();
    table[0] = ffi.Pointer.fromFunction<
      ffi.Bool Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Size>, ffi.Pointer<_Buf>)
    >(_readerRead, false).cast();
    table[1] = ffi.Pointer.fromFunction<
      ffi.Bool Function(ffi.Pointer<ffi.Void>, ffi.Int, ffi.Int64, ffi.Pointer<ffi.Uint64>, ffi.Pointer<_Buf>)
    >(_readerSeek, false).cast();
    return table.cast<ffi.Void>();
  }();

  /// `reader` lent to Rust for the call of the arena `a`.
  _ForeignReader reader(_Arena a, SeekableReader reader) => ffi.Struct.create<_ForeignReader>()
    ..read = ffi.Native.addressOf<
      ffi.NativeFunction<
        ffi.Bool Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Size>, ffi.Pointer<_Buf>)
      >
    >(_dartReaderRead).cast()
    ..seek = ffi.Native.addressOf<
      ffi.NativeFunction<
        ffi.Bool Function(ffi.Pointer<ffi.Void>, ffi.Int, ffi.Int64, ffi.Pointer<ffi.Uint64>, ffi.Pointer<_Buf>)
      >
    >(_dartReaderSeek).cast()
    ..data = a.lend(reader, _readers);

  /// `text` as a buffer Rust owns, as `buf_copy` makes it: what a closure or a reader
  /// hands Rust.
  _Buf copied(String text) {
    final a = _Arena();
    try {
      final view = a<_Buf>();
      a.string(view.ref, text, false);
      return _bufCopy(view.ref);
    } finally {
      a.end();
    }
  }

  void setBuf(ffi.Pointer<_Buf> dst, _Buf src) {
    dst.ref.ptr = src.ptr;
    dst.ref.len = src.len;
    dst.ref.owner = src.owner;
  }

  /// A handle Rust takes, of a clone of `object`'s value: what a closure returns of a
  /// `Copy` object, which Rust copies out of it and frees.
  ffi.Pointer<ffi.Void> handOver(
    _Object object,
    ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>) clone,
  ) {
    final raw = object._hold();
    try {
      return clone(home, raw);
    } finally {
      object._release();
    }
  }

  /// Abort with what a call into Dart threw, which Rust has nowhere to unwind to.
  Never abort(String what, Object error, StackTrace stack) => fail('$what: a call into Dart threw $error\n$stack');

  /// Abort the process with `text`.
  Never fail(String text) {
    final a = _Arena();
    final buf = a<_Buf>();
    a.string(buf.ref, text, false);
    _dartAbort(buf.ref);
    throw StateError('unreachable');
  }

  /// Bytes Rust returned: copied up to [_lend] bytes, and past that lent to a list freed
  /// with it and every view of it.
  typed.Uint8List bytes(_Buf buf) {
    final len = buf.len;
    if (len <= _lend) {
      final copy = typed.Uint8List(len);
      if (len > 0) copy.setAll(0, buf.ptr.asTypedList(len));
      _bufFree(buf);
      return copy;
    }
    final list = buf.ptr.asTypedList(len, finalizer: _ownerFree, token: buf.owner).asUnmodifiableView();
    lentBytes[list] = _Lent(buf.ptr.address, buf.owner.address);
    return list;
  }

  typed.Uint8List? bytesOrNull(_Buf buf) => buf.ptr == ffi.nullptr ? null : bytes(buf);

  /// Bytes Rust lends a closure or an implementation for its call, copied.
  typed.Uint8List bytesLent(_Buf buf) =>
      buf.len == 0 ? typed.Uint8List(0) : typed.Uint8List.fromList(buf.ptr.asTypedList(buf.len));

  typed.Uint8List? bytesLentOrNull(_Buf buf) => buf.ptr == ffi.nullptr ? null : bytesLent(buf);

  /// Text Rust returned, copied, then freed.
  String string(_Buf buf) {
    final text = stringLent(buf);
    _bufFree(buf);
    return text;
  }

  String? stringOrNull(_Buf buf) => buf.ptr == ffi.nullptr ? null : string(buf);

  /// Text Rust lends, which Rust made UTF-8, copied.
  String stringLent(_Buf buf) => buf.len == 0 ? '' : convert.utf8.decode(buf.ptr.asTypedList(buf.len));

  String? stringLentOrNull(_Buf buf) => buf.ptr == ffi.nullptr ? null : stringLent(buf);

  /// A path Rust returned, copied, then freed.
  String path(_Buf buf) {
    try {
      return pathLent(buf);
    } finally {
      _bufFree(buf);
    }
  }

  String? pathOrNull(_Buf buf) => buf.ptr == ffi.nullptr ? null : path(buf);

  /// A path from Rust, copied, which on Unix may hold bytes no `String` holds: those
  /// abort, since a throw would leave the rest of a list or record being lifted unfreed.
  String pathLent(_Buf buf) {
    if (buf.len == 0) return '';
    try {
      return const convert.Utf8Decoder(allowMalformed: false).convert(buf.ptr.asTypedList(buf.len));
    } on FormatException {
      fail('a path from Rust is not UTF-8');
    }
  }

  String? pathLentOrNull(_Buf buf) => buf.ptr == ffi.nullptr ? null : pathLent(buf);

  /// A `char`, one Unicode scalar.
  int char(String text) {
    final runes = text.runes;
    // A lone surrogate is one rune, but no Unicode scalar.
    if (runes.length != 1 || runes.first >= 0xd800 && runes.first <= 0xdfff) {
      throw ArgumentError.value(text, 'char', 'a char is one Unicode scalar');
    }
    return runes.first;
  }

  String scalar(int code) => String.fromCharCode(code);

  void u128(_U128 dst, BigInt value) {
    if (value.isNegative || value.bitLength > 128) {
      throw ArgumentError.value(value, 'value', "out of range for Rust's u128");
    }
    // Each half as the int that holds its 64 bits.
    dst.lo = value.toSigned(64).toInt();
    dst.hi = (value >> 64).toSigned(64).toInt();
  }

  BigInt u128Of(_U128 c) =>
      BigInt.from(c.hi).toUnsigned(64) << 64 | BigInt.from(c.lo).toUnsigned(64);

  /// Rust's `Duration::MAX`, and any longer than Dart's longest, is that.
  static const _longest = Duration(microseconds: 0x7fffffffffffffff);

  void duration(_Duration dst, Duration value) {
    if (value.isNegative) {
      throw ArgumentError.value(value, 'duration', 'a negative Duration has no Rust Duration');
    }
    if (value == _longest) {
      dst.secs = -1;
      dst.nanos = 999999999;
      return;
    }
    final us = value.inMicroseconds;
    dst.secs = us ~/ 1000000;
    dst.nanos = (us % 1000000) * 1000;
  }

  /// A Rust `Duration`, truncated to the microsecond.
  Duration durationOf(_Duration c) {
    final secs = c.secs;
    // Past Dart's longest, 9223372036854.775807 seconds, read as `u64`: negative is past
    // `i64`.
    if (secs < 0 || secs > 9223372036854 || secs == 9223372036854 && c.nanos ~/ 1000 >= 775807) {
      return _longest;
    }
    return Duration(microseconds: secs * 1000000 + c.nanos ~/ 1000);
  }

  void addr(_Addr dst, SocketAddr value, String family) {
    final ip = value.address.rawAddress;
    final v6 = value.address.type == io.InternetAddressType.IPv6;
    if (family == 'V4' && v6 || family == 'V6' && !v6) {
      throw ArgumentError.value(
        value,
        'addr',
        'an IP${v6 ? 'v6' : 'v4'} address has no Rust SocketAddr$family',
      );
    }
    for (var i = 0; i < 16; i++) {
      dst.ip[i] = i < ip.length ? ip[i] : 0;
    }
    dst.port = u16(value.port);
    dst.v6 = v6;
    dst.flowinfo = u32(value.flowinfo);
    dst.scope_id = u32(value.scopeId);
  }

  SocketAddr addrOf(_Addr c) {
    final n = c.v6 ? 16 : 4;
    final ip = typed.Uint8List(n);
    for (var i = 0; i < n; i++) {
      ip[i] = c.ip[i];
    }
    return SocketAddr(
      io.InternetAddress.fromRawAddress(ip),
      c.port,
      flowinfo: c.flowinfo,
      scopeId: c.scope_id,
    );
  }

  int u8(int v) => v >= 0 && v <= 0xff ? v : throw RangeError.range(v, 0, 0xff);
  int u16(int v) => v >= 0 && v <= 0xffff ? v : throw RangeError.range(v, 0, 0xffff);
  int u32(int v) => v >= 0 && v <= 0xffffffff ? v : throw RangeError.range(v, 0, 0xffffffff);
  int i8(int v) => v >= -0x80 && v <= 0x7f ? v : throw RangeError.range(v, -0x80, 0x7f);
  int i16(int v) => v >= -0x8000 && v <= 0x7fff ? v : throw RangeError.range(v, -0x8000, 0x7fff);
  int i32(int v) =>
      v >= -0x80000000 && v <= 0x7fffffff ? v : throw RangeError.range(v, -0x80000000, 0x7fffffff);
}

/// What Rust calls on the isolate's thread once it drops an implementation.
void _dropped(int token) => _n._drops.remove(token)?.call();

/// What frees the owner of bytes lent to a list.
final _ownerFree = ffi.Native.addressOf<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(_dartOwnerFree);
