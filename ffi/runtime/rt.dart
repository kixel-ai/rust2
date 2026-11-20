// The runtime every generated Dart file includes, after the declarations of its C
// functions: the handles objects own, the stack a call lowers its values into, bytes and
// text across the C ABI, and the port that wakes, dropped wakers, and calls into Dart from
// other threads reach the isolate through.

/// Rust's `u64` or `usize`: an `int`'s 64 bits read unsigned, so it holds every value
/// exactly, at no cost over the `int`. Its operators compare unsigned, but as an extension
/// type it has the `int`'s own `toString`, signed: print [toBigInt] or [toRadixString].
extension type const U64.fromBits(int bits) {
  /// `value`, throwing a [RangeError] if negative.
  U64(int value) : bits = value {
    if (value < 0) throw RangeError.range(value, 0, null, 'value');
  }

  /// `value`, throwing a [RangeError] outside `u64`'s range.
  factory U64.fromBigInt(BigInt value) {
    if (value.isNegative || value.bitLength > 64) {
      throw RangeError("$value is out of range for Rust's u64");
    }
    return U64.fromBits(value.toSigned(64).toInt());
  }

  /// `u64::MAX`.
  static const max = U64.fromBits(-1);

  /// As an `int`, throwing a [RangeError] past its largest.
  int toInt() => bits >= 0 ? bits : throw RangeError("${toBigInt()} is past int's largest");

  BigInt toBigInt() => bits >= 0 ? BigInt.from(bits) : BigInt.from(bits).toUnsigned(64);

  /// Its digits in `radix`, unsigned.
  String toRadixString(int radix) {
    RangeError.checkValueInInterval(radix, 2, 36, 'radix');
    if (bits >= 0) return bits.toRadixString(radix);
    // Halved to fit `int`, so the quotient is at most one short.
    var q = ((bits >>> 1) ~/ radix) << 1;
    var r = bits - q * radix;
    if (r >= radix) {
      q++;
      r -= radix;
    }
    return '${q.toRadixString(radix)}${r.toRadixString(radix)}';
  }

  int compareTo(U64 other) => (bits ^ _sign).compareTo(other.bits ^ _sign);

  bool operator <(U64 other) => (bits ^ _sign) < (other.bits ^ _sign);
  bool operator <=(U64 other) => (bits ^ _sign) <= (other.bits ^ _sign);
  bool operator >(U64 other) => (bits ^ _sign) > (other.bits ^ _sign);
  bool operator >=(U64 other) => (bits ^ _sign) >= (other.bits ^ _sign);

  /// The sign bit, flipped to compare the bits unsigned as signed.
  static const _sign = -0x8000000000000000;
}

/// Rust's `Duration`: whole seconds, a `u64`, and the nanoseconds below one, so it holds
/// every value exactly, which a [Duration], of microseconds within `int`'s range, does not.
final class RustDuration implements Comparable<RustDuration> {
  final U64 secs;

  /// From 0 through 999999999.
  final int nanos;

  /// `secs` and `nanos`, throwing a [RangeError] unless `nanos` is below a second.
  RustDuration(this.secs, [this.nanos = 0]) {
    RangeError.checkValueInInterval(nanos, 0, 999999999, 'nanos');
  }

  const RustDuration._(this.secs, this.nanos);

  static const zero = RustDuration._(U64.fromBits(0), 0);

  /// `duration`, throwing an [ArgumentError] if negative.
  factory RustDuration.from(Duration duration) {
    final us = duration.inMicroseconds;
    if (us < 0) throw ArgumentError.value(duration, 'duration', 'is negative');
    return RustDuration._(U64.fromBits(us ~/ 1000000), us % 1000000 * 1000);
  }

  /// As a [Duration], throwing a [RangeError] where it holds part of a microsecond or is
  /// past the longest, 9223372036854.775807 seconds.
  Duration toDuration() {
    final s = secs.bits;
    if (nanos % 1000 != 0 || s < 0 || s > 9223372036854 || s == 9223372036854 && nanos > 775807000) {
      throw RangeError('$this is no Duration');
    }
    return Duration(microseconds: s * 1000000 + nanos ~/ 1000);
  }

  @override
  bool operator ==(Object other) => other is RustDuration && other.secs == secs && other.nanos == nanos;

  @override
  int get hashCode => Object.hash(secs.bits, nanos);

  @override
  int compareTo(RustDuration other) {
    final bySecs = secs.compareTo(other.secs);
    return bySecs != 0 ? bySecs : nanos.compareTo(other.nanos);
  }

  @override
  String toString() => '${secs.toRadixString(10)}.${nanos.toString().padLeft(9, '0')}s';

  // ignore: unused_element
  void _put(_Duration c) {
    c.secs = secs.bits;
    c.nanos = nanos;
  }

  // ignore: unused_element
  static RustDuration _of(_Duration c) => RustDuration._(U64.fromBits(c.secs), c.nanos);
}

/// Rust's `PathBuf`: the path's bytes as the OS holds them, which on Unix may be any, so
/// it holds every path exactly, which a [String] does not. On Windows they are WTF-8.
final class PathBuf {
  final typed.Uint8List _bytes;

  /// `path`'s UTF-8, throwing an [ArgumentError] where it holds a lone surrogate.
  PathBuf(String path) : _bytes = _utf8(path);

  /// The path of the OS's [bytes], copied, throwing an [ArgumentError] where one is not a
  /// byte or, on Windows, they are not WTF-8.
  PathBuf.fromBytes(List<int> bytes) : _bytes = _osPath(bytes);

  // ignore: unused_element
  PathBuf._(this._bytes);

  /// Its bytes, which never change.
  typed.Uint8List get bytes => _bytes.asUnmodifiableView();

  /// As a [String], throwing a [FormatException] where it is not UTF-8.
  String toFilePath() => convert.utf8.decode(_bytes);

  @override
  bool operator ==(Object other) {
    if (other is! PathBuf || other._bytes.length != _bytes.length) return false;
    for (var i = 0; i < _bytes.length; i++) {
      if (other._bytes[i] != _bytes[i]) return false;
    }
    return true;
  }

  @override
  int get hashCode => Object.hashAll(_bytes);

  /// Its text, as Rust's `Path::display` shows it: what is not UTF-8 is replaced.
  @override
  String toString() => convert.utf8.decode(_bytes, allowMalformed: true);
}

/// Rust's `IpAddr`: an [Ipv4Addr] or an [Ipv6Addr].
sealed class IpAddr {
  const IpAddr._();

  /// `address`'s IP, throwing an [ArgumentError] for a Unix address, or an IPv6 one with a
  /// scope, which Rust's has none of.
  factory IpAddr.from(io.InternetAddress address) => switch (address.type) {
    io.InternetAddressType.IPv4 => Ipv4Addr.from(address),
    io.InternetAddressType.IPv6 => Ipv6Addr.from(address),
    _ => throw ArgumentError.value(address, 'address', 'is no IP address'),
  };

  /// Its bytes in network order: 4 of IPv4, or 16 of IPv6.
  typed.Uint8List get octets;

  io.InternetAddress toInternetAddress() => io.InternetAddress.fromRawAddress(octets);

  @override
  String toString() => toInternetAddress().address;

  // ignore: unused_element
  static void _put(_IpAddr c, IpAddr v) {
    switch (v) {
      case Ipv4Addr():
        c.v6 = false;
        v._put(c.ip.v4);
      case Ipv6Addr():
        c.v6 = true;
        v._put(c.ip.v6);
    }
  }

  // ignore: unused_element
  static IpAddr _of(_IpAddr c) => c.v6 ? Ipv6Addr._of(c.ip.v6) : Ipv4Addr._of(c.ip.v4);
}

/// The octets of `list`, which must be `n`, each from 0 through 255.
typed.Uint8List _ipOctets(List<int> list, int n) {
  if (list.length != n || list.any((b) => b < 0 || b > 255)) {
    throw ArgumentError.value(list, 'octets', 'are not $n bytes');
  }
  return typed.Uint8List.fromList(list);
}

/// The IP of `address` of the family `type`, throwing an [ArgumentError] for another.
typed.Uint8List _ipRaw(io.InternetAddress address, io.InternetAddressType type) {
  if (address.type != type) throw ArgumentError.value(address, 'address', 'is no $type address');
  // A scope shows only in its text.
  if (address.address.contains('%')) {
    throw ArgumentError.value(address, 'address', "has a scope, which Rust's IPv6 address has none of");
  }
  return address.rawAddress;
}

/// Rust's `Ipv4Addr`.
final class Ipv4Addr extends IpAddr {
  /// Its octets, the first highest.
  final int _bits;

  const Ipv4Addr._(this._bits) : super._();

  /// The address of `octets`, throwing an [ArgumentError] unless they are 4 bytes.
  factory Ipv4Addr.fromOctets(List<int> octets) => Ipv4Addr._(_ipWord(_ipOctets(octets, 4), 0, 4));

  /// `address`, throwing an [ArgumentError] unless it is IPv4.
  factory Ipv4Addr.from(io.InternetAddress address) =>
      Ipv4Addr._(_ipWord(_ipRaw(address, io.InternetAddressType.IPv4), 0, 4));

  @override
  typed.Uint8List get octets => typed.Uint8List(4)..buffer.asByteData().setUint32(0, _bits);

  @override
  bool operator ==(Object other) => other is Ipv4Addr && other._bits == _bits;

  @override
  int get hashCode => _bits.hashCode;

  void _put(_Ipv4Addr c) {
    final o = c.octets;
    for (var i = 0; i < 4; i++) {
      o[i] = _bits >>> (24 - 8 * i) & 0xff;
    }
  }

  static Ipv4Addr _of(_Ipv4Addr c) {
    final o = c.octets;
    return Ipv4Addr._(o[0] << 24 | o[1] << 16 | o[2] << 8 | o[3]);
  }
}

/// The big-endian word of `bytes`' `n` from `at`.
int _ipWord(typed.Uint8List bytes, int at, int n) {
  var word = 0;
  for (var i = at; i < at + n; i++) {
    word = word << 8 | bytes[i];
  }
  return word;
}

/// Rust's `Ipv6Addr`.
final class Ipv6Addr extends IpAddr {
  /// Its first 8 octets and its last, each word's first highest.
  final int _hi, _lo;

  const Ipv6Addr._(this._hi, this._lo) : super._();

  factory Ipv6Addr._bytes(typed.Uint8List bytes) => Ipv6Addr._(_ipWord(bytes, 0, 8), _ipWord(bytes, 8, 8));

  /// The address of `octets`, throwing an [ArgumentError] unless they are 16 bytes.
  factory Ipv6Addr.fromOctets(List<int> octets) => Ipv6Addr._bytes(_ipOctets(octets, 16));

  /// `address`, throwing an [ArgumentError] unless it is IPv6 with no scope.
  factory Ipv6Addr.from(io.InternetAddress address) => Ipv6Addr._bytes(_ipRaw(address, io.InternetAddressType.IPv6));

  @override
  typed.Uint8List get octets => typed.Uint8List(16)
    ..buffer.asByteData().setUint64(0, _hi)
    ..buffer.asByteData().setUint64(8, _lo);

  @override
  bool operator ==(Object other) => other is Ipv6Addr && other._hi == _hi && other._lo == _lo;

  @override
  int get hashCode => Object.hash(_hi, _lo);

  void _put(_Ipv6Addr c) {
    final o = c.octets;
    for (var i = 0; i < 8; i++) {
      o[i] = _hi >>> (56 - 8 * i) & 0xff;
      o[i + 8] = _lo >>> (56 - 8 * i) & 0xff;
    }
  }

  static Ipv6Addr _of(_Ipv6Addr c) {
    final o = c.octets;
    var hi = 0, lo = 0;
    for (var i = 0; i < 8; i++) {
      hi = hi << 8 | o[i];
      lo = lo << 8 | o[i + 8];
    }
    return Ipv6Addr._(hi, lo);
  }
}

/// Rust's `SocketAddr`: a [SocketAddrV4] or a [SocketAddrV6].
sealed class SocketAddr {
  const SocketAddr._();

  /// `ip` at `port`, which IPv6 has with no flow label or scope, throwing a [RangeError]
  /// for a port past 65535.
  factory SocketAddr(IpAddr ip, int port) => switch (ip) {
    Ipv4Addr() => SocketAddrV4(ip, port),
    Ipv6Addr() => SocketAddrV6(ip, port),
  };

  IpAddr get ip;
  int get port;

  // ignore: unused_element
  static void _put(_SocketAddr c, SocketAddr v) {
    switch (v) {
      case SocketAddrV4():
        c.v6 = false;
        v._put(c.addr.v4);
      case SocketAddrV6():
        c.v6 = true;
        v._put(c.addr.v6);
    }
  }

  // ignore: unused_element
  static SocketAddr _of(_SocketAddr c) => c.v6 ? SocketAddrV6._of(c.addr.v6) : SocketAddrV4._of(c.addr.v4);
}

/// Rust's `SocketAddrV4`.
final class SocketAddrV4 extends SocketAddr {
  @override
  final Ipv4Addr ip;
  @override
  final int port;

  /// `ip` at `port`, throwing a [RangeError] for a port past 65535.
  SocketAddrV4(this.ip, this.port) : super._() {
    RangeError.checkValueInInterval(port, 0, 0xffff, 'port');
  }

  const SocketAddrV4._(this.ip, this.port) : super._();

  @override
  bool operator ==(Object other) => other is SocketAddrV4 && other.ip == ip && other.port == port;

  @override
  int get hashCode => Object.hash(ip, port);

  @override
  String toString() => '$ip:$port';

  void _put(_SocketAddrV4 c) {
    ip._put(c.ip);
    c.port = port;
  }

  static SocketAddrV4 _of(_SocketAddrV4 c) => SocketAddrV4._(Ipv4Addr._of(c.ip), c.port);
}

/// Rust's `SocketAddrV6`.
final class SocketAddrV6 extends SocketAddr {
  @override
  final Ipv6Addr ip;
  @override
  final int port;

  /// Its flow label.
  final int flowinfo;
  final int scopeId;

  /// `ip` at `port`, throwing a [RangeError] for a port past 65535, or a flow label or
  /// scope past `u32`'s range.
  SocketAddrV6(this.ip, this.port, {this.flowinfo = 0, this.scopeId = 0}) : super._() {
    RangeError.checkValueInInterval(port, 0, 0xffff, 'port');
    RangeError.checkValueInInterval(flowinfo, 0, 0xffffffff, 'flowinfo');
    RangeError.checkValueInInterval(scopeId, 0, 0xffffffff, 'scopeId');
  }

  const SocketAddrV6._(this.ip, this.port, this.flowinfo, this.scopeId) : super._();

  @override
  bool operator ==(Object other) =>
      other is SocketAddrV6 &&
      other.ip == ip &&
      other.port == port &&
      other.flowinfo == flowinfo &&
      other.scopeId == scopeId;

  @override
  int get hashCode => Object.hash(ip, port, flowinfo, scopeId);

  @override
  String toString() => scopeId == 0 ? '[$ip]:$port' : '[$ip%$scopeId]:$port';

  void _put(_SocketAddrV6 c) {
    ip._put(c.ip);
    c.port = port;
    c.flowinfo = flowinfo;
    c.scopeId = scopeId;
  }

  static SocketAddrV6 _of(_SocketAddrV6 c) => SocketAddrV6._(Ipv6Addr._of(c.ip), c.port, c.flowinfo, c.scopeId);
}

/// A copy of `bytes`, throwing an [ArgumentError] where one is not a byte or, on Windows,
/// they are not WTF-8.
typed.Uint8List _osPath(List<int> bytes) {
  if (bytes is! typed.Uint8List && bytes.any((b) => b < 0 || b > 255)) {
    throw ArgumentError.value(bytes, 'bytes', 'are not all bytes');
  }
  final copy = typed.Uint8List.fromList(bytes);
  if (io.Platform.isWindows && !_wtf8(copy)) {
    throw ArgumentError.value(bytes, 'bytes', 'is not WTF-8, so names no Windows path');
  }
  return copy;
}

/// Whether `bytes` are WTF-8, as Rust encodes a Windows path: UTF-8, but for a surrogate
/// alone, encoded as a scalar would be, though never a lead then a trail.
bool _wtf8(typed.Uint8List bytes) {
  var at = 0;
  var lead = false;
  while (at < bytes.length) {
    final b = bytes[at];
    final (n, min) = switch (b) {
      < 0x80 => (1, 0),
      >= 0xc2 && <= 0xdf => (2, 0x80),
      >= 0xe0 && <= 0xef => (3, 0x800),
      >= 0xf0 && <= 0xf4 => (4, 0x10000),
      _ => (0, 0),
    };
    if (n == 0) return false;
    if (at + n > bytes.length) return false;
    var c = n == 1 ? b : b & (0x7f >> n);
    for (var k = 1; k < n; k++) {
      final next = bytes[at + k];
      if (next & 0xc0 != 0x80) return false;
      c = (c << 6) | (next & 0x3f);
    }
    if (c < min || c > 0x10ffff) return false;
    final surrogate = c >= 0xd800 && c <= 0xdfff;
    if (surrogate && c >= 0xdc00 && lead) return false;
    lead = surrogate && c < 0xdc00;
    at += n;
  }
  return true;
}

/// `text`'s UTF-8, throwing an [ArgumentError] where it holds a lone surrogate, which no
/// Rust string does: Dart's encoder would replace it.
typed.Uint8List _utf8(String text) {
  final n = text.length;
  for (var i = 0; i < n; i++) {
    final unit = text.codeUnitAt(i);
    if (unit & 0xf800 != 0xd800) continue;
    // A lead followed by a trail is one scalar.
    if (unit < 0xdc00 && i + 1 < n && text.codeUnitAt(i + 1) & 0xfc00 == 0xdc00) {
      i++;
      continue;
    }
    throw ArgumentError.value(text, 'text', 'holds a lone surrogate at $i, which no Rust string does');
  }
  return convert.utf8.encode(text);
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

/// The future of `call`, whose start left the future in Rust that pended: polled again
/// here with a waker, which its first poll had no use for.
// ignore: unused_element
Cancelable<T> _call<T>(_Call<T> call) {
  final future = call._done.future;
  _n.calls[future] = call;
  call._begin();
  return Cancelable._(future);
}

/// What `next` yields, one item a call: a stream that calls `start` only while its one
/// listener listens and is not paused, and awaits each call before the next.
///
/// It reads nothing ahead, so a loop left early leaves the next item to the next call. A
/// pause or cancel drops the pending call's future in Rust, as dropping it there would, so
/// it loses an item only where `next` does when dropped. `None` ends it, and an error is
/// its last event before it ends.
// ignore: unused_element
dasync.Stream<T> _stream<T extends Object>(_Call<T?>? Function(_Pull<T>) start) {
  final pull = _Pull<T>._(start);
  // Synchronous, so a pause or cancel in the listener's handler lands before the next
  // call: each event is added from the event loop, never inside a listener's own call.
  pull._out = dasync.StreamController<T>(
    sync: true,
    onListen: pull._later,
    onPause: pull._drop,
    onResume: pull._later,
    onCancel: pull._drop,
  );
  return pull._out.stream;
}

/// What a [_Call]'s poll that finished hands its outcome to, once Rust freed its future.
abstract interface class _Sink<T> {
  void _settle(_Call<T> call);
}

/// A stream's calls of `next`, one at a time.
final class _Pull<T extends Object> implements _Sink<T?> {
  /// Its `next`: the call it left pending in Rust, or null once it finished, having
  /// handed this its outcome.
  final _Call<T?>? Function(_Pull<T>) _start;
  late final dasync.StreamController<T> _out;

  /// The call pending in Rust, if any.
  _Call<T?>? _pending;

  /// Whether [_pull] runs, which then adds what a call that finished meanwhile left.
  bool _pulling = false;

  /// An outcome not yet added: 1 for a value, 2 for an error, as [_Call]'s.
  int _state = 0;
  Object? _value;
  StackTrace? _stack;

  _Pull._(this._start);

  /// A finished start's value, kept until the start returns.
  // ignore: unused_element
  void _ready(T? value) {
    _state = 1;
    _value = value;
  }

  /// A finished start's error, kept until the start returns.
  // ignore: unused_element
  void _error(Object error) {
    _state = 2;
    _value = error;
  }

  /// Pull from the event loop, not inside a listen or resume, which add no event.
  void _later() => dasync.scheduleMicrotask(_pull);

  /// Drop the pending call's future, losing nothing of this stream's own.
  void _drop() {
    final pending = _pending;
    _pending = null;
    pending?.cancel();
  }

  @override
  void _settle(_Call<T?> call) {
    // A call a pause or cancel dropped while Rust polled it, as from a closure Rust called,
    // that finished in that poll: a pause keeps its outcome for the resume, and a cancel
    // leaves no listener to take it.
    final dropped = !identical(call, _pending);
    if (dropped && !_out.hasListener) return;
    _pending = null;
    _state = call._settled;
    _value = call._value;
    _stack = call._stack;
    if (!_pulling && !dropped && _add()) _pull();
  }

  void _pull() {
    if (_pulling) return;
    _pulling = true;
    try {
      while (_pending == null && _out.hasListener && !_out.isPaused && !_out.isClosed) {
        // An outcome left, by a start that finished or a call that finished as a pause
        // dropped it, which is added before the next call.
        if (_state != 0) {
          if (!_add()) return;
          continue;
        }
        final _Call<T?>? call;
        try {
          call = _start(this);
        } catch (e, s) {
          // A check that refused the call, before Rust ran.
          _end(e, s);
          return;
        }
        if (call != null) {
          // A pause or cancel from a callback Rust called in the start, before the call
          // was pending for it to drop.
          if (!_out.hasListener || _out.isPaused || _out.isClosed) {
            call.cancel();
            return;
          }
          _pending = call;
          call._sink = this;
          call._begin();
          // Still pending: its wake settles it.
          if (identical(call, _pending)) return;
        }
      }
    } finally {
      _pulling = false;
    }
  }

  /// Add the outcome left, if any: whether the stream goes on.
  bool _add() {
    final (state, value, stack) = (_state, _value, _stack);
    _state = 0;
    _value = null;
    _stack = null;
    switch (state) {
      case 0:
        return true;
      case 2:
        _end(value!, stack);
        return false;
      default:
        if (value == null) {
          _out.close().ignore();
          return false;
        }
        _out.add(value as T);
        return true;
    }
  }

  void _end(Object error, StackTrace? stack) {
    _out.addError(error, stack);
    _out.close().ignore();
  }
}

/// A [Cancelable]'s future in Rust, from when its start pended until it finishes.
final class _Call<T> {
  /// The future in Rust, until it finishes.
  ffi.Pointer<ffi.Void> _f;
  final void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>) _free;

  /// Poll once, completing this if it finished: whether it did.
  final bool Function(ffi.Pointer<ffi.Void>, _Waker, _Call<T>) _poll;

  final _done = dasync.Completer<T>();
  _Waker? _waker;
  int _token = 0;
  bool _polling = false;
  bool _cancelled = false;

  /// A stream's, which takes its outcome in place of [_done], once Rust freed the future:
  /// 1 for a value, 2 for an error, in [_value], with its [_stack].
  _Sink<T>? _sink;
  int _settled = 0;
  Object? _value;
  StackTrace? _stack;

  _Call._(this._free, this._poll, this._f);

  /// Poll it the first time with a waker, which its start had no use for.
  void _begin() {
    _token = _n.listen(_wake);
    _waker = _n.waker(_token);
    _step(_waker!);
  }

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
      _sink?._settle(this);
    } else if (_cancelled) {
      _cancel();
    }
  }

  /// Complete it with what Rust returned, from its poll.
  // ignore: unused_element
  void _ready(T value) {
    if (_sink == null) return _done.complete(value);
    _settled = 1;
    _value = value;
  }

  /// Complete it with the error Rust returned, from its poll, or the poll's own.
  void _error(Object error, [StackTrace? stack]) {
    final sink = _sink;
    if (sink == null) return _done.completeError(error, stack);
    _settled = 2;
    _value = error;
    _stack = stack;
    // A poll that threw freed the future already.
    if (_f == ffi.nullptr) sink._settle(this);
  }

  void _finish() {
    _free(_n.home, _f);
    _f = ffi.nullptr;
    _n.calls.remove(_done.future);
    if (_token != 0) {
      _n.forget(_token);
      _token = 0;
    }
  }

  void _cancel() {
    _finish();
    // Awaiting it is optional once cancelled.
    _done.future.ignore();
    _done.completeError(const Cancelled());
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

final class _Ipv4Addr extends ffi.Struct {
  @ffi.Array(4)
  external ffi.Array<ffi.Uint8> octets;
}

final class _Ipv6Addr extends ffi.Struct {
  @ffi.Array(16)
  external ffi.Array<ffi.Uint8> octets;
}

final class _Ip extends ffi.Union {
  external _Ipv4Addr v4;
  external _Ipv6Addr v6;
}

final class _IpAddr extends ffi.Struct {
  @ffi.Bool()
  external bool v6;
  external _Ip ip;
}

final class _SocketAddrV4 extends ffi.Struct {
  external _Ipv4Addr ip;
  @ffi.Uint16()
  external int port;
}

final class _SocketAddrV6 extends ffi.Struct {
  external _Ipv6Addr ip;
  @ffi.Uint16()
  external int port;
  @ffi.Uint32()
  external int flowinfo;
  @ffi.Uint32()
  external int scopeId;
}

final class _Socket extends ffi.Union {
  external _SocketAddrV4 v4;
  external _SocketAddrV6 v6;
}

final class _SocketAddr extends ffi.Struct {
  @ffi.Bool()
  external bool v6;
  external _Socket addr;
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
  /// [SeekFrom.start], `offset` and the position are a `u64`'s 64 bits, as [U64.bits] holds
  /// them, so one past `i64`'s range is negative.
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
    _n.setBuf(err, _n.copied(e.toString(), message: true));
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
    _n.setBuf(err, _n.copied(e.toString(), message: true));
    return false;
  } catch (e, s) {
    _n.abort("a reader's seek", e, s);
  }
}

/// A waker Rust returned: calling it wakes through Rust, and `close()` drops it, rather
/// than the garbage collector once it finds this unreachable. Each serves the isolate
/// while it waits, as an object's call and free do, since either may run Rust that waits
/// on a thread calling into Dart.
final class Waker extends _Object {
  final ffi.Pointer<ffi.Void> _wake;
  final ffi.Pointer<ffi.Void> _dropFn;
  final ffi.NativeFinalizer _finalizer;

  Waker._(_Waker raw)
      : _wake = raw.wake,
        _dropFn = raw.drop,
        _finalizer = _n.finalizer(raw.drop),
        super(raw.data, true, false) {
    _finalizer.attach(this, _raw, detach: this);
  }

  @override
  String get _type => 'Waker';

  @override
  void _free() {
    _finalizer.detach(this);
    _dartWakerCall(_n.home, _dropFn, _raw);
  }

  /// Wake the task through Rust: a call after [close] throws [StateError].
  void call() {
    _dartWakerCall(_n.home, _wake, _hold());
    _release();
  }
}

/// A callback's table and data, as each trait's struct is.
final class _Callback extends ffi.Struct {
  external ffi.Pointer<ffi.Void> vtable;
  external ffi.Pointer<ffi.Void> data;
}

/// What an object owns: its handle, or one Rust lends for a call, which Rust frees. Each
/// call holds it, so the handle is freed only once no call does. An owned one of a value
/// Rust checks is Rust's to account: [close] drops the value as Rust's own drop does,
/// failing while a call borrows it, and frees the handle with it unless a call holds it.
/// Any other: [close] frees it then, throwing while a call holds it. A call after either
/// throws. One returned waits in the isolate's [_Nursery], where a close frees it without
/// the garbage collector, which frees it only once pushed out unclosed.
abstract base class _Object implements ffi.Finalizable {
  final ffi.Pointer<ffi.Void> _raw;

  /// The owner's hold until closed, and one for each call.
  int _holds = 1;
  bool _closed = false;
  final bool _owned;

  /// Whether Rust checks its borrows, so its close drops the value through Rust.
  final bool _checked;

  /// The garbage collector's hold of an owned one's handle, from when the nursery pushed it
  /// out until freed.
  // ignore: unused_field
  ffi.Pointer<ffi.Void> _own = ffi.nullptr;

  /// Its slot in the nursery while there, else -1.
  int _slot = -1;

  _Object(this._raw, bool owned, bool checked) : _owned = owned, _checked = owned && checked;

  String get _type;

  /// Free an owned one's handle, and take back the garbage collector's hold of it.
  void _free() {}

  /// Give the garbage collector a hold of the handle, as the nursery pushes it out open.
  void _register() {}

  /// Drop the value of one Rust checks, then, if `free`, free its handle and take back the
  /// garbage collector's hold of it, in one crossing: throwing [StateError], changing
  /// nothing, if a call borrows it.
  void _close(bool free) {}

  ffi.Pointer<ffi.Void> _hold() {
    if (_closed) throw StateError('$_type is closed');
    _holds++;
    return _raw;
  }

  /// The handle for a leaf call, which needs no hold: no Dart runs to close it meanwhile.
  // ignore: unused_element
  ffi.Pointer<ffi.Void> get _live {
    if (_closed) throw StateError('$_type is closed');
    return _raw;
  }

  void _release() {
    if (--_holds == 0 && _owned) {
      _leave();
      _free();
    }
  }

  /// Take it out of the nursery, returning whether it was there: before its free, which
  /// may run Dart that returns objects into the nursery meanwhile.
  bool _leave() {
    final slot = _slot;
    if (slot < 0) return false;
    _slot = -1;
    _n.nursery.leave(slot);
    return true;
  }

  /// Close it as [_Object] says; again, it does nothing.
  void close() {
    if (_closed) return;
    if (_checked) {
      // Closed before Rust drops the value, which may run Dart that reaches it meanwhile.
      _closed = true;
      // A call holding it, from a callback or one waiting on a worker, may not have
      // borrowed the value in Rust yet: the handle outlives its hold, so Rust's check
      // refuses it.
      final alone = _holds == 1;
      final nursed = alone && _leave();
      try {
        _close(alone);
      } catch (_) {
        _closed = false;
        // Still open, and out of the nursery, so the garbage collector's.
        if (nursed) _register();
        rethrow;
      }
      if (alone) {
        _holds = 0;
      } else {
        _release();
      }
      return;
    }
    if (_holds != 1) throw StateError('$_type: a handle is already borrowed');
    _closed = true;
    _release();
  }
}

/// How many of the latest objects returned the nursery holds: one dropped unclosed is freed
/// only once this many more were returned, so it bounds what a dropped object keeps of
/// Rust's.
const _nurseryLen = 64;

/// The latest objects returned, oldest at [_next] once full: a close takes one out without
/// the garbage collector's hold of its handle, which costs more than the rest of a return
/// and close together, and one pushed out still open takes that hold then. Rust keeps a copy
/// of each slot's handle and the C function freeing it, and frees what is still in them once
/// the garbage collector finds the nursery unreachable, as when the isolate exits.
final class _Nursery {
  final List<_Object?> _held = List<_Object?>.filled(_nurseryLen, null);
  int _next = 0;

  /// What Rust keeps: the length, then each slot's handle, then each one's free function,
  /// null while the slot is out.
  late final ffi.Pointer<ffi.Pointer<ffi.Void>> _slots = _dartNursery(this, _nurseryLen);

  /// Hold `object`, owned, whose handle `free` frees.
  void push(_Object object, ffi.Pointer<ffi.Void> free) {
    final i = _next;
    _next = i + 1 == _nurseryLen ? 0 : i + 1;
    final old = _held[i];
    if (old != null) {
      old._slot = -1;
      old._register();
    }
    _held[i] = object;
    object._slot = i;
    final slots = _slots;
    slots[1 + i] = object._raw;
    slots[1 + _nurseryLen + i] = free;
  }

  void leave(int slot) {
    _held[slot] = null;
    _slots[1 + slot] = ffi.nullptr;
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
      bytes(dst, _utf8(text), take);
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

  /// Where a call that can fail a check writes the failure: one for the isolate, since the
  /// glue writes it on the isolate's thread as the call returns, once it served every call
  /// queued meanwhile, and the call reads it before any other can run, and a failure
  /// clears it.
  late final ffi.Pointer<ffi.Int32> status = _dartAlloc(4).cast<ffi.Int32>()..value = 0;

  /// The latest objects returned.
  late final _Nursery nursery = _Nursery();
  late final isolate.RawReceivePort _port;
  late final ffi.Pointer<ffi.Void> home;

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
  /// hands Rust. A lone surrogate throws, or in a `message` is replaced.
  _Buf copied(String text, {bool message = false}) {
    final a = _Arena();
    try {
      final view = a<_Buf>();
      if (message) {
        a.bytes(view.ref, convert.utf8.encode(text), false);
      } else {
        a.string(view.ref, text, false);
      }
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

  /// Abort with what a call into Dart threw, which Rust has nowhere to unwind to.
  Never abort(String what, Object error, StackTrace stack) => fail('$what: a call into Dart threw $error\n$stack');

  /// Abort the process with `text`.
  Never fail(String text) {
    final a = _Arena();
    final buf = a<_Buf>();
    a.bytes(buf.ref, convert.utf8.encode(text), false);
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

  int u8(int v) => v >= 0 && v <= 0xff ? v : throw RangeError.range(v, 0, 0xff);
  int u16(int v) => v >= 0 && v <= 0xffff ? v : throw RangeError.range(v, 0, 0xffff);
  int u32(int v) => v >= 0 && v <= 0xffffffff ? v : throw RangeError.range(v, 0, 0xffffffff);
  int i8(int v) => v >= -0x80 && v <= 0x7f ? v : throw RangeError.range(v, -0x80, 0x7f);
  int i16(int v) => v >= -0x8000 && v <= 0x7fff ? v : throw RangeError.range(v, -0x8000, 0x7fff);
  int i32(int v) =>
      v >= -0x80000000 && v <= 0x7fffffff ? v : throw RangeError.range(v, -0x80000000, 0x7fffffff);

  /// A `NonZero`'s integer, which Rust's type holds only if it is not zero.
  int nonZero(int v) => v != 0 ? v : throw ArgumentError.value(v, 'NonZero', 'a NonZero is zero');

  /// Whether the library's target has 64-bit words, else 32-bit ones.
  final bool _wide = ffi.sizeOf<ffi.Size>() == 8;

  /// A `usize`, which on a 32-bit target holds only 32 bits.
  int usize(U64 v) => _wide || v.bits >>> 32 == 0 ? v.bits : throw RangeError("${v.toBigInt()} is past this target's usize");

  /// An `isize`, which on a 32-bit target holds only 32 bits.
  int isize(int v) => _wide ? v : i32(v);
}

/// What Rust calls on the isolate's thread once it drops an implementation.
void _dropped(int token) => _n._drops.remove(token)?.call();

/// What frees the owner of bytes lent to a list.
final _ownerFree = ffi.Native.addressOf<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>(_dartOwnerFree);
