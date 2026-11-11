# [M] Export moq for every target it ships

## Goal

Nightly exports moq's stand-in facade for each target moq ships into one binding, as
[rust2-ffi](/ffi/README.md#targets) shares a binding across targets, and fails on an item
whose definition differs between two of them. Each target's API is staged with `--api`
on a host with its C toolchain and merged with `--with` on Linux, whose export then builds
and runs `just ffi moq` as today.

## Plan

moq ships one binding per language across targets
([moq's release workflows](https://github.com/moq-dev/moq/tree/main/.github/workflows)):

- Kotlin: three Android ABIs, Linux x86_64 and aarch64, macOS, and Windows.
- Dart: those, plus Intel macOS and three iOS targets.
- Go: Linux x86_64 and aarch64, macOS, and Windows.
- Swift: iOS, the iOS simulator, and macOS.
- Python wheels and the C library ship per target, so they need nothing more.

So the union is the three Android ABIs (in `nix develop .#android`, which has the NDK),
Linux x86_64 and aarch64, macOS on aarch64 and x86_64 and the iOS device and simulator
targets (on a macOS runner), and Windows MSVC (on a Windows runner): moq's `aws-lc-sys`
and `ring` compile C in their build scripts even for a check, so each target stages where
its toolchain is. Each staging job uploads its JSON as an artifact, and the Linux job
downloads them all.

At moq `ffcb7cd9`, the stand-in facade's items that differ by target are all in
moq-tokio: `tokio::Error::Unix`, `tokio::listen::Config::unix`, and the `tokio::unix`
module exist only on unix, and `tokio::tls::Error::AndroidInit`, `tokio::tls::init_android`,
and the `jni` re-export only on Android. Each is an item or a variant only some targets
have, which the binding serves, so none should be refused: `tokio::listen::Config` holds a
`pub(crate)` field, so it exports as an object, whose `unix` field's accessors are only
unix's, rather than as a record whose fields differ, which the export refuses.

## Hosts

- linux
- macos - the Apple targets' checks need Apple's SDK
- windows - the MSVC target's check needs MSVC
