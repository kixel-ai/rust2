// bench/export.c's frame writes and reads through the generated Dart, so the two differ by
// what dart:ffi and the Dart wrapper add. Each round runs twice, the first to warm the JIT.

import 'dart:async';
import 'dart:typed_data';

import 'package:moq/moq.dart' as moq;

const groups = 10000;
const frames = 100;

/// Nanoseconds per frame `watch` has run for.
double perFrame(Stopwatch watch) => watch.elapsedTicks * 1e9 / watch.frequency / (groups * frames);

Future<String> round() async {
  final broadcast = moq.BroadcastInfo().produce();
  final micro = moq.Timescale.MICRO;
  final track = broadcast.createTrack('bench', moq.TrackInfo.default_().withTimescale(micro));
  micro.close();

  // A frame of 100 zero bytes, as bench/export.c writes.
  final frame = Uint8List(100);
  var micros = 0;

  // Each frame stamped through a fresh timestamp object, as a caller holding an int must.
  var watch = Stopwatch()..start();
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
  final stamped = perFrame(watch);

  // The same, reusing one timestamp object: what the object costs.
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
  final reused = perFrame(watch);

  // The floor: a hold of a handle and a bool back.
  var closed = 0;
  watch = Stopwatch()..start();
  for (var i = 0; i < groups * frames; i++) {
    if (track.isClosed()) closed++;
  }
  final call = perFrame(watch);

  // Reads: each frame written, then read through a call whose future is ready at its first
  // poll, so it never waits for a wake.
  final consumer = track.consume();
  final subscriber = await consumer.subscribe(null);
  consumer.close();
  moq.FrameFrame? last;
  watch = Stopwatch()..start();
  for (var g = 0; g < groups; g++) {
    final group = track.appendGroup();
    final reader = (await subscriber.recvGroup())!;
    for (var f = 0; f < frames; f++) {
      group.writeFrame(at, frame);
      final read = (await reader.readFrame())!;
      if (g == groups - 1 && f == frames - 1) {
        last = read;
      } else {
        read.close();
      }
    }
    reader.close();
    group.finish();
    group.close();
  }
  final read = perFrame(watch);

  // Woken reads: each read called before its frame is written, so it pends, and the write's
  // wake reaches the isolate's port, whose event loop polls it again.
  watch = Stopwatch()..start();
  for (var g = 0; g < groups ~/ 10; g++) {
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
  final wake = perFrame(watch) * 10;

  // And one 4 KiB frame.
  final group = track.appendGroup();
  final reader = (await subscriber.recvGroup())!;
  group.writeFrame(at, Uint8List(4096));
  final big = (await reader.readFrame())!;
  reader.close();
  group.close();
  subscriber.close();

  // Payload reads: each frame's bytes, as a list, both sizes copied at the call.
  double payload(moq.FrameFrame frame) {
    final watch = Stopwatch()..start();
    for (var i = 0; i < groups * frames; i++) {
      closed += frame.payloadGet[0];
    }
    return perFrame(watch);
  }

  final small = payload(last!);
  final large = payload(big);
  last.close();
  big.close();
  at.close();
  track.close();
  broadcast.close();

  final bang = closed != 0 ? '!' : '';
  return 'export (Dart): ${stamped.toStringAsFixed(0)} ns per stamped 100 B frame, '
      '${reused.toStringAsFixed(0)} reusing the timestamp, ${call.toStringAsFixed(1)} ns per handle call, '
      '${read.toStringAsFixed(0)} ns per frame written and read, ${wake.toStringAsFixed(0)} ns per frame written '
      'and read after a wake, ${small.toStringAsFixed(0)} ns per 100 B payload read, '
      '${large.toStringAsFixed(0)} per 4 KiB$bang';
}

Future<void> main() async {
  await round();
  print(await round());
}
