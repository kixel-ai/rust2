// bench/export.c's cases through the generated Dart, so the two differ by what dart:ffi and
// the Dart wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the VM's own threads included.

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

/// What the loops sum, kept so the compiler cannot drop the calls.
var sink = 0;

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
Future<Span> throughput(moq.TrackProducer track, moq.TrackSubscriber subscriber, moq.Timestamp at, int size) async {
  final count = ((64 << 20) ~/ (size * frames)).clamp(3, 2000);
  final frame = Uint8List(size);
  final start = now();
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
  final ns = per(start, count * frames * size);
  return Span(1e3 / ns.wall, 1e3 / ns.cpu);
}

Future<Map<String, Span>> round() async {
  final out = <String, Span>{};
  final broadcast = moq.BroadcastInfo().produce();
  final track = broadcast.createTrack('bench', moq.TrackInfo.default_().withTimescale(moq.Timescale.MICRO));
  final frame = Uint8List(100);

  var start = now();
  for (var i = 0; i < n; i++) {
    if (track.isClosed()) sink++;
  }
  out['call'] = per(start);

  final demanded = track.appendGroup();
  final demand = demanded.demand();
  start = now();
  for (var i = 0; i < n; i++) {
    sink += demand.sequence();
  }
  out['getter'] = per(start);
  demand.close();
  demanded.finish();
  demanded.close();

  start = now();
  for (var i = 0; i < n; i++) {
    if (track.largest() != null) sink++;
  }
  out['record'] = per(start);

  start = now();
  for (var i = 0; i < n; i++) {
    track.info().close();
  }
  out['object'] = per(start);

  // Each frame stamped through a fresh timestamp, as a caller holding an int must.
  var micros = 0;
  start = now();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    for (var f = 0; f < frames; f++) {
      group.writeFrame(moq.Timestamp.fromMicros(micros++), frame);
    }
    group.finish();
    group.close();
  }
  out['stamped'] = per(start);

  final at = moq.Timestamp.fromMicros(micros);
  start = now();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
    }
    group.finish();
    group.close();
  }
  out['write'] = per(start);

  final consumer = track.consume();
  final subscriber = await consumer.subscribe(null);
  consumer.close();

  // Each frame written, then read through a call whose future is ready at its first poll,
  // so it never waits for a wake.
  moq.FrameFrame? last;
  start = now();
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
  out['ready'] = per(start);

  // Each read called before its frame is written, so it pends, and the write's wake
  // reaches the isolate's port, whose event loop polls it again.
  start = now();
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
  out['woken'] = per(start);

  for (final size in sizes) {
    out['mbps:$size'] = await throughput(track, subscriber, at, size);
  }
  subscriber.close();

  // The last frame's 100 bytes, copied into a list, and one of them read.
  start = now();
  for (var i = 0; i < n; i++) {
    sink += last!.payloadGet[0];
  }
  out['payload'] = per(start);
  last!.close();
  track.close();
  broadcast.close();
  return out;
}

Future<void> main() async {
  await round();
  for (final MapEntry(:key, :value) in (await round()).entries) {
    print('$key\t${value.wall}\t${value.cpu}');
  }
}
