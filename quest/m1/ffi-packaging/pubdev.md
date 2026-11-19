# [M] A pub.dev package with a build hook

## Goal

The export writes a Dart package, `pubspec.yaml` and `hook/build.dart`, and the builder
releases it to pub.dev with checksummed per-target libraries the hook downloads, building
from source inside a checkout. A clean Dart app and a Flutter app on the Android emulator
consume it and print the transcript; the Flutter run includes a hot restart, since Rust's
statics outlive the restarted isolate.

## Plan

Found by the 2026-11-18 language audit: moq already publishes `moq_ffi` with a
hand-written UniFFI hook (`dart/moq_ffi/hook/build.dart` in moq-dev/moq) that downloads
checksummed libraries and accepts a `user_defines` override. Build hooks are stable since
Dart 3.10 and Flutter 3.38 ([announcement](https://blog.dart.dev/announcing-dart-3-10-ea8b952b6088)).

Public API: the generated package layout.
