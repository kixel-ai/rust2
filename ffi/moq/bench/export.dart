// bench/export.c's cases through the generated Dart, so the two differ by what dart:ffi and
// the Dart wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the VM's own threads included, or under
// CodSpeed, runs once for its CPU simulation to count. Its argument is
// moq/bench/codspeed.h's shared library.

import 'dart:async';
import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'dart:typed_data';

import 'package:moq/moq.dart' as moq;

const groups = 10000;
const frames = 100;
const n = groups * frames;
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
Span per(Span start, [num calls = n]) {
  final end = now();
  return Span((end.wall - start.wall) / calls, (end.cpu - start.cpu) / calls);
}

/// moq/bench/codspeed.h's functions, from the shared library main opens.
late final bool Function(ffi.Pointer<ffi.Char>) _codspeedInit;
late final void Function() _codspeedStart;
late final void Function(ffi.Pointer<ffi.Char>) _codspeedStop;

/// A case's start: now, and CodSpeed counting from here.
Span begin() {
  final start = now();
  _codspeedStart();
  return start;
}

/// The case `name`'s nanoseconds per call since `start`, over `calls` calls.
Span end(Span start, String name, [num calls = n]) {
  final c = _cString(name);
  _codspeedStop(c);
  _free(c.cast());
  return per(start, calls);
}

final _malloc = _process
    .lookupFunction<ffi.Pointer<ffi.Void> Function(ffi.Size), ffi.Pointer<ffi.Void> Function(int)>('malloc');
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

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
Future<Span> throughput(moq.TrackProducer track, moq.TrackSubscriber subscriber, moq.Timestamp at, int size) async {
  final count = ((64 << 20) ~/ (size * frames)).clamp(3, 2000);
  final frame = Uint8List(size);
  final name = 'throughput[$size]';
  final start = begin();
  for (var g = 0; g < count; g++) {
    final group = track.appendGroup();
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
      final read = (await reader.readFrame())!;
      sink += read.payloadGet[0];
      read.close();
    }
    reader.close();
    group.finish();
    group.close();
  }
  final ns = end(start, name, count * frames * size);
  return Span(1e3 / ns.wall, 1e3 / ns.cpu);
}

Future<Map<String, Span>> round() async {
  final out = <String, Span>{};
  final broadcast = moq.BroadcastInfo().produce();
  final track = broadcast.createTrack('bench', moq.TrackInfo.default_().withTimescale(moq.Timescale.MICRO));
  final frame = Uint8List(100);

  var start = begin();
  for (var i = 0; i < n; i++) {
    if (track.isClosed()) sink++;
  }
  out['call'] = end(start, 'call');

  final demanded = track.appendGroup();
  final demand = demanded.demand();
  start = begin();
  for (var i = 0; i < n; i++) {
    sink += demand.sequence();
  }
  out['getter'] = end(start, 'getter');
  demand.close();
  demanded.finish();
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

  // Each frame stamped through a fresh timestamp, as a caller holding an int must.
  var micros = 0;
  start = begin();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    for (var f = 0; f < frames; f++) {
      group.writeFrame(moq.Timestamp.fromMicros(micros++), frame);
    }
    group.finish();
    group.close();
  }
  out['stamped'] = end(start, 'stamped');

  final at = moq.Timestamp.fromMicros(micros);
  start = begin();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
    }
    group.finish();
    group.close();
  }
  out['write'] = end(start, 'write');

  final consumer = track.consume();
  final subscriber = await consumer.subscribe(null);
  consumer.close();

  // Each frame written, then read through a call whose future is ready at its first poll,
  // so it never waits for a wake.
  moq.FrameFrame? last;
  start = begin();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
      last?.close();
      last = (await reader.readFrame())!;
    }
    reader.close();
    group.finish();
    group.close();
  }
  out['ready'] = end(start, 'ready');

  // Each read called before its frame is written, so it pends, and the write's wake
  // reaches the isolate's port, whose event loop polls it again.
  start = begin();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      final pending = reader.readFrame();
      group.writeFrame(at, frame);
      (await pending)!.close();
    }
    reader.close();
    group.finish();
    group.close();
  }
  out['woken'] = end(start, 'woken');

  // One writer future and one reader; one outstanding permit bounds the stream.
  start = begin();
  final permits = StreamController<void>();
  Future<void> writeLoop() async {
    final next = StreamIterator<void>(permits.stream);
    for (var g = 0; g < groups; g++) {
      final group = track.appendGroup();
      for (var f = 0; f < frames; f++) {
        if (!await next.moveNext()) throw StateError('missing permit');
        group.writeFrame(at, frame);
      }
      if (!await next.moveNext()) throw StateError('missing group acknowledgement');
      group.finish();
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

  for (final size in sizes) {
    out['mbps:$size'] = await throughput(track, subscriber, at, size);
  }
  subscriber.close();

  // The last frame's 100 bytes, copied into a list, and one of them read.
  start = begin();
  for (var i = 0; i < n; i++) {
    sink += last!.payloadGet[0];
  }
  out['payload'] = end(start, 'payload');
  last!.close();
  track.close();
  broadcast.close();
  return out;
}

Future<void> main(List<String> args) async {
  final codspeed = ffi.DynamicLibrary.open(args.single);
  _codspeedInit = codspeed.lookupFunction<ffi.Bool Function(ffi.Pointer<ffi.Char>), bool Function(ffi.Pointer<ffi.Char>)>(
    'codspeed_init',
  );
  _codspeedStart = codspeed.lookupFunction<ffi.Void Function(), void Function()>('codspeed_start');
  _codspeedStop = codspeed.lookupFunction<ffi.Void Function(ffi.Pointer<ffi.Char>), void Function(ffi.Pointer<ffi.Char>)>(
    'codspeed_stop',
  );
  final bench = _cString('ffi/moq/bench/export.dart::dart');
  final measured = _codspeedInit(bench);
  _free(bench.cast());
  // Under CodSpeed, one round, which prints nothing: its simulation counts each case.
  if (measured) {
    await round();
    return;
  }
  await round();
  for (final MapEntry(:key, :value) in (await round()).entries) {
    print('$key\t${value.wall}\t${value.cpu}');
  }
}
