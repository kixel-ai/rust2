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

Decided on 2026-11-10, taken unattended under the maintainer's standing instruction to
take the recommended option, so it may be revisited: how a shared binding treats an item
only some of its targets have. ✅ (b) export every target's items: on a target that lacks
one, calling it raises the language's own error, and a value holding a field it lacks is
refused; only an item whose signature differs between targets is refused at export, at its
span, naming both. Rejected: (a) refuse such items, which would refuse moq's
`tokio::Error`, `tokio::listen::Config`, and `tokio::tls::Error` in Kotlin, Dart, and Go
and need moq source changes, against the 2026-11-09 rule that rust2 asks moq for none;
(c) one binding per target, which the JVM, Dart, and Go cannot load under one name.
