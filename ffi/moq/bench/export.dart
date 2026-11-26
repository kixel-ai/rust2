// bench/export.c's cases through the generated Dart, so the two differ by what dart:ffi and
// the Dart wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the VM's own threads included, or under
// Valgrind, runs once for swole to count. Its argument is
// moq/bench/hooks.h's shared library.

import 'dart:async';
import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'dart:typed_data';

import 'package:moq/moq.dart' as moq;

/// A tenth under Valgrind, which counts every call exactly.
var groups = 10000;
const frames = 100;
int get n => groups * frames;
const sizes = [100, 1024, 16384, 65536, 1048576];

/// Wall and CPU nanoseconds: a point, or a span per call.
class Span {
  final double wall;
  final double cpu;
  const Span(this.wall, this.cpu);
}

final _process = ffi.DynamicLibrary.process();
final _clockGettime = _process
    .lookupFunction<ffi.Int32 Function(ffi.Int32, ffi.Pointer<ffi.Int64>), int Function(int, ffi.Pointer<ffi.Int64>)>(
      'clock_gettime',
    );
// A timespec of two 64-bit fields, as on every 64-bit target, never freed.
final _timespec = _process
    .lookupFunction<ffi.Pointer<ffi.Void> Function(ffi.Size), ffi.Pointer<ffi.Void> Function(int)>('malloc')(16)
    .cast<ffi.Int64>();
// CLOCK_PROCESS_CPUTIME_ID, which dart:io does not expose.
final _processCpu = Platform.isMacOS ? 12 : 2;
final _wall = Stopwatch()..start();

Span now() {
  if (_clockGettime(_processCpu, _timespec) != 0) throw StateError('clock_gettime');
  return Span(_wall.elapsedTicks * 1e9 / _wall.frequency, _timespec[0] * 1e9 + _timespec[1]);
}

/// Nanoseconds per call since `start`, over `calls` calls.
Span per(Span start, [num? calls]) {
  final end = now();
  calls ??= n;
  return Span((end.wall - start.wall) / calls, (end.cpu - start.cpu) / calls);
}

/// moq/bench/hooks.h's functions, from the shared library main opens.
late final bool Function(ffi.Pointer<ffi.Char>) _hooksInit;
late final void Function() _hooksStart;
late final void Function(ffi.Pointer<ffi.Char>) _hooksStop;

/// A case's start: now, and Valgrind counting from here.
Span begin() {
  final start = now();
  _hooksStart();
  return start;
}

/// The case `name`'s nanoseconds per call since `start`, over `calls` calls.
Span end(Span start, String name, [num? calls]) {
  final c = _cString(name);
  _hooksStop(c);
  _free(c.cast());
  return per(start, calls);
}

final _malloc = _process.lookupFunction<ffi.Pointer<ffi.Void> Function(ffi.Size), ffi.Pointer<ffi.Void> Function(int)>(
  'malloc',
);
final _free = _process.lookupFunction<ffi.Void Function(ffi.Pointer<ffi.Void>), void Function(ffi.Pointer<ffi.Void>)>(
  'free',
);

/// `s`, an ASCII name, as a C string to free.
ffi.Pointer<ffi.Char> _cString(String s) {
  final c = _malloc(s.length + 1).cast<ffi.Uint8>();
  for (var i = 0; i < s.length; i++) {
    c[i] = s.codeUnitAt(i);
  }
  c[s.length] = 0;
  return c.cast();
}

/// What the loops sum, kept so the compiler cannot drop the calls.
var sink = 0;

/// The round's clock in microseconds on its timed track, where each group starts at its
/// first frame's time, so the starts a track's groups are appended at strictly increase.
var micros = 0;

/// A new group's start, at which each of its frames is stamped: one conversion a group, so
/// a frame costs what it did with one timestamp for all.
moq.Timestamp groupStart() => moq.Timestamp.fromMicros(moq.U64(micros++));

/// A timed track named [name] on [broadcast]. It caches every group written to it until it
/// closes, as the broadcast's pool, which nothing polls, evicts none, so each case that writes
/// groups writes them to a track of its own, closed once it ends: a round holds one case's
/// frames at a time instead of every case's, about 2 GB.
moq.TrackProducer createTrack(moq.BroadcastProducer broadcast, String name) =>
    broadcast.createTrack(name, moq.TrackInfo.default_().withTimescale(moq.Timescale.micro));

/// A subscriber to [track].
Future<moq.TrackSubscriber> subscribe(moq.TrackProducer track) async {
  final consumer = track.consume();
  final subscriber = await consumer.subscribe(null);
  consumer.close();
  return subscriber;
}

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
Future<Span> throughput(moq.BroadcastProducer broadcast, int size) async {
  final count = ((64 << 20) ~/ (size * frames)).clamp(3, 2000);
  final frame = Uint8List(size);
  final name = 'throughput[$size]';
  final track = createTrack(broadcast, name);
  final subscriber = await subscribe(track);
  final start = begin();
  for (var g = 0; g < count; g++) {
    final at = groupStart();
    final group = track.appendGroup(at);
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
      final read = (await reader.readFrame())!;
      sink += read.payloadGet[0];
      read.close();
    }
    reader.close();
    group.finish().close();
    group.close();
  }
  final ns = end(start, name, count * frames * size);
  subscriber.close();
  track.close();
  return Span(1e3 / ns.wall, 1e3 / ns.cpu);
}

Future<Map<String, Span>> round() async {
  final out = <String, Span>{};
  final broadcast = moq.BroadcastInfo().produce();
  var track = createTrack(broadcast, 'bench');
  final frame = Uint8List(100);
  micros = 0;

  var start = begin();
  for (var i = 0; i < n; i++) {
    if (track.isClosed()) sink++;
  }
  out['call'] = end(start, 'call');

  final demanded = track.appendGroup(groupStart());
  final demand = demanded.demand();
  start = begin();
  for (var i = 0; i < n; i++) {
    sink += demand.start().bits;
  }
  out['getter'] = end(start, 'getter');
  demand.close();
  demanded.finish().close();
  demanded.close();

  start = begin();
  for (var i = 0; i < n; i++) {
    if (track.largest() != null) sink++;
  }
  out['record'] = end(start, 'record');

  start = begin();
  for (var i = 0; i < n; i++) {
    track.info().close();
  }
  out['object'] = end(start, 'object');

  // A primitive pub field read, which reaches no user code: a leaf call.
  final info = track.info();
  start = begin();
  for (var i = 0; i < n; i++) {
    sink += info.priorityGet;
  }
  out['field'] = end(start, 'field');

  // While Rust holds a closure of the isolate, which another thread may call, each call
  // but a leaf hands off to a worker thread, as the isolate's own serves what it calls back.
  final held = moq.clockDrive((_, _) => null);
  final hops = n ~/ 10;
  start = begin();
  for (var i = 0; i < hops; i++) {
    if (track.isClosed()) sink++;
  }
  out['hop:call'] = end(start, 'hop:call', hops);
  start = begin();
  for (var i = 0; i < hops; i++) {
    sink += info.priorityGet;
  }
  out['hop:field'] = end(start, 'hop:field', hops);
  held.cancel();
  await held.then<void>((_) {}, onError: (Object _) {});
  info.close();

  // Each frame stamped through a fresh timestamp, as a caller holding an int must.
  track.close();
  track = createTrack(broadcast, 'stamped');
  start = begin();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup(moq.Timestamp.fromMicros(moq.U64(micros)));
    for (var f = 0; f < frames; f++) {
      group.writeFrame(moq.Timestamp.fromMicros(moq.U64(micros++)), frame);
    }
    group.finish().close();
    group.close();
  }
  out['stamped'] = end(start, 'stamped');

  track.close();
  track = createTrack(broadcast, 'write');
  start = begin();
  for (var g = 0; g < groups; g++) {
    final at = groupStart();
    final group = track.appendGroup(at);
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
    }
    group.finish().close();
    group.close();
  }
  out['write'] = end(start, 'write');

  // Each frame written, then read through a call whose future is ready at its first poll,
  // so it never waits for a wake.
  track.close();
  track = createTrack(broadcast, 'ready');
  var subscriber = await subscribe(track);
  moq.FrameFrame? last;
  start = begin();
  for (var g = 0; g < groups; g++) {
    final at = groupStart();
    final group = track.appendGroup(at);
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
      last?.close();
      last = (await reader.readFrame())!;
    }
    reader.close();
    group.finish().close();
    group.close();
  }
  out['ready'] = end(start, 'ready');

  // Each read called before its frame is written, so it pends, and the write's wake
  // reaches the isolate's port, whose event loop polls it again.
  subscriber.close();
  track.close();
  track = createTrack(broadcast, 'woken');
  subscriber = await subscribe(track);
  start = begin();
  for (var g = 0; g < groups; g++) {
    final at = groupStart();
    final group = track.appendGroup(at);
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      final pending = reader.readFrame();
      group.writeFrame(at, frame);
      (await pending)!.close();
    }
    reader.close();
    group.finish().close();
    group.close();
  }
  out['woken'] = end(start, 'woken');

  // One writer future and one reader; one outstanding permit bounds the stream.
  subscriber.close();
  track.close();
  track = createTrack(broadcast, 'loop');
  subscriber = await subscribe(track);
  start = begin();
  final permits = StreamController<void>();
  Future<void> writeLoop() async {
    final next = StreamIterator<void>(permits.stream);
    for (var g = 0; g < groups; g++) {
      final at = groupStart();
      final group = track.appendGroup(at);
      for (var f = 0; f < frames; f++) {
        if (!await next.moveNext()) throw StateError('missing permit');
        group.writeFrame(at, frame);
      }
      if (!await next.moveNext()) throw StateError('missing group acknowledgement');
      group.finish().close();
      group.close();
    }
    await next.cancel();
  }

  final writer = writeLoop();
  for (var g = 0; g < groups; g++) {
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      permits.add(null);
      (await reader.readFrame())!.close();
    }
    reader.close();
    permits.add(null);
  }
  await writer;
  await permits.close();
  out['loop'] = end(start, 'loop');
  subscriber.close();
  track.close();

  for (final size in sizes) {
    out['mbps:$size'] = await throughput(broadcast, size);
  }

  // The last frame's 100 bytes, copied into a list, and one of them read.
  start = begin();
  for (var i = 0; i < n; i++) {
    sink += last!.payloadGet[0];
  }
  out['payload'] = end(start, 'payload');
  last!.close();
  broadcast.close();
  return out;
}

Future<void> main(List<String> args) async {
  final hooks = ffi.DynamicLibrary.open(args.single);
  _hooksInit = hooks
      .lookupFunction<ffi.Bool Function(ffi.Pointer<ffi.Char>), bool Function(ffi.Pointer<ffi.Char>)>('hooks_init');
  _hooksStart = hooks.lookupFunction<ffi.Void Function(), void Function()>('hooks_start');
  _hooksStop = hooks
      .lookupFunction<ffi.Void Function(ffi.Pointer<ffi.Char>), void Function(ffi.Pointer<ffi.Char>)>('hooks_stop');
  final bench = _cString('ffi/moq/bench/export.dart::dart');
  final measured = _hooksInit(bench);
  _free(bench.cast());
  // Under Valgrind, one round, which prints nothing: swole counts each case.
  if (measured) {
    groups ~/= 10;
    await round();
    return;
  }
  await round();
  for (final MapEntry(:key, :value) in (await round()).entries) {
    print('$key\t${value.wall}\t${value.cpu}');
  }
}
