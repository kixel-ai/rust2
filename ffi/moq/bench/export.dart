// bench/export.c's cases through the generated Dart, so the two differ by what dart:ffi and
// the Dart wrapper add. Each round runs twice, the first to warm the JIT.

import 'dart:typed_data';

import 'package:moq/moq.dart' as moq;

const groups = 10000;
const frames = 100;
const n = groups * frames;
const sizes = [100, 1024, 16384, 65536, 1048576];

/// Nanoseconds per call `watch` has run for, over `calls` calls.
double per(Stopwatch watch, [int calls = n]) => watch.elapsedTicks * 1e9 / watch.frequency / calls;

/// What the loops sum, kept so the compiler cannot drop the calls.
var sink = 0;

/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
/// byte touched, in about 64 MiB and at least 300 frames: megabytes a second.
Future<double> throughput(moq.TrackProducer track, moq.TrackSubscriber subscriber, moq.Timestamp at, int size) async {
  final count = ((64 << 20) ~/ (size * frames)).clamp(3, 2000);
  final frame = Uint8List(size);
  final watch = Stopwatch()..start();
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
  return count * frames * size / (watch.elapsedTicks / watch.frequency) / 1e6;
}

Future<Map<String, double>> round() async {
  final out = <String, double>{};
  final broadcast = moq.BroadcastInfo().produce();
  final micro = moq.Timescale.MICRO;
  final track = broadcast.createTrack('bench', moq.TrackInfo.default_().withTimescale(micro));
  micro.close();
  final frame = Uint8List(100);

  var watch = Stopwatch()..start();
  for (var i = 0; i < n; i++) {
    if (track.isClosed()) sink++;
  }
  out['call'] = per(watch);

  final demanded = track.appendGroup();
  final demand = demanded.demand();
  watch = Stopwatch()..start();
  for (var i = 0; i < n; i++) {
    sink += demand.sequence();
  }
  out['getter'] = per(watch);
  demand.close();
  demanded.finish();
  demanded.close();

  watch = Stopwatch()..start();
  for (var i = 0; i < n; i++) {
    if (track.largest() != null) sink++;
  }
  out['record'] = per(watch);

  watch = Stopwatch()..start();
  for (var i = 0; i < n; i++) {
    track.info().close();
  }
  out['object'] = per(watch);

  // Each frame stamped through a fresh timestamp object, as a caller holding an int must.
  var micros = 0;
  watch = Stopwatch()..start();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    for (var f = 0; f < frames; f++) {
      final at = moq.Timestamp.fromMicros(micros++);
      group.writeFrame(at, frame);
      at.close();
    }
    group.finish();
    group.close();
  }
  out['stamped'] = per(watch);

  final at = moq.Timestamp.fromMicros(micros);
  watch = Stopwatch()..start();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
    }
    group.finish();
    group.close();
  }
  out['write'] = per(watch);

  final consumer = track.consume();
  final subscriber = await consumer.subscribe(null);
  consumer.close();

  // Each frame written, then read through a call whose future is ready at its first poll,
  // so it never waits for a wake.
  moq.FrameFrame? last;
  watch = Stopwatch()..start();
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
  out['ready'] = per(watch);

  // Each read called before its frame is written, so it pends, and the write's wake
  // reaches the isolate's port, whose event loop polls it again.
  watch = Stopwatch()..start();
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
  out['woken'] = per(watch);

  for (final size in sizes) {
    out['mbps:$size'] = await throughput(track, subscriber, at, size);
  }
  subscriber.close();

  // The last frame's 100 bytes, copied into a list, and one of them read.
  watch = Stopwatch()..start();
  for (var i = 0; i < n; i++) {
    sink += last!.payloadGet[0];
  }
  out['payload'] = per(watch);
  last!.close();
  at.close();
  track.close();
  broadcast.close();
  return out;
}

Future<void> main() async {
  await round();
  for (final MapEntry(:key, :value) in (await round()).entries) {
    print('$key\t$value');
  }
}
