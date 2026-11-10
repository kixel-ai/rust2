# [L] Share a binding across targets

## Goal

One binding serves every target its package ships to, such as one Kotlin file for each
Android ABI and the desktops, though their exports differ by `cfg`: on each target, every
item that target compiles works exactly and no other item does. Proven by a fixture whose
API differs by target, exported for the host and wasm32 into one binding and driven on the
host, and by moq's census on each target moq ships.

## Plan

`rust2-ffi --target <triple>` exports one target's API, with that target's `cfg`s and
layouts, into a crate whose build script refuses any other target; the `target` test in
`ffi/tests/fixtures.rs` proves it for the host and wasm32. moq ships one binding per
language across targets ([moq's release workflows](https://github.com/moq-dev/moq/tree/main/.github/workflows)):

- Kotlin: three Android ABIs, Linux x86_64 and aarch64, macOS, and Windows.
- Dart: those, plus Intel macOS and three iOS targets.
- Go: Linux x86_64 and aarch64, macOS, and Windows.
- Swift: iOS, the iOS simulator, and macOS.
- Python wheels and the C library ship per target, so they need nothing here.

At moq `ffcb7cd9`, the stand-in facade's items that differ by target are all in
moq-tokio: `tokio::Error::Unix`, `tokio::listen::Config::unix`, and the `tokio::unix`
module exist only on unix, and `tokio::tls::Error::AndroidInit`, `tokio::tls::init_android`,
and the `jni` re-export only on Android. Swift's targets differ in none of them; Kotlin's,
Dart's, and Go's differ in `tokio::Error`, moq's top-level error.

Each target's census has to run where its C toolchain is: moq's dependencies `aws-lc-sys`
and `ring` compile C in their build scripts even for a check, so a Linux host without the
NDK, Apple's SDK, or MSVC can census only itself.

## Waiting

- Maintainer: how a shared binding exports an item that some of its targets lack. (a)
  Refuse it, at its span, naming the targets that differ, as the quest first recommended:
  this refuses moq's `tokio::Error`, `tokio::listen::Config`, and `tokio::tls::Error` in
  Kotlin, Dart, and Go, which moq can avoid only by changing its source. (b) Export the
  union: a target that lacks an item raises the language's own error when it is called,
  never produces a variant it lacks, and refuses a value holding a field it lacks; refuse at
  compile time only an item whose shape differs between targets, such as a parameter's
  type, at its span, naming both. (c) Ship a binding per target, which the JVM, Dart, and
  Go cannot load side by side under one name. Recommendation: (b), since moq's error type
  differs by target and rust2 asks moq for no source changes.
