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

So the union is the three Android ABIs (in `nix develop .#android`, whose NDK is the
caller's and which adds only the aarch64 and x86_64 Android targets today, so this quest
adds `armv7-linux-androideabi`), Linux x86_64 and aarch64, macOS on aarch64 and x86_64 and the iOS device and simulator
targets (on a macOS runner), and Windows MSVC (on a Windows runner): moq's `aws-lc-sys`
and `ring` compile C in their build scripts even for a check, so each target stages where
its toolchain is. Each staging job uploads its JSON as an artifact, and the Linux job
downloads them all.

At moq `ffcb7cd9`, the stand-in facade's items that differ by target are all in
moq-tokio: `tokio::Error::Unix` and `tokio::listen::Config::unix` exist only on unix, and
`tokio::tls::Error::AndroidInit` and `tokio::tls::init_android` only on Android. The
narrowed stand-in (`ffi/justfile`) re-exports neither moq-tokio's `unix` module nor its
`jni`. Each is an item or a variant only some targets have, which the binding serves:
`tokio::listen::Config` holds a `pub(crate)` field, so it exports as an object, whose
`unix` field's accessors are only unix's, rather than as a record whose fields differ,
which the export refuses. `ffi/moq/ffi.toml` still excludes `tokio::listen::Config::unix`.

Open, found by the 2026-11-12 audit: `init_android(env: &mut jni::Env, context: JObject)`
and `Error::AndroidInit(jni::errors::Error)` name the `jni` crate's types, which no rule
in `ffi/README.md` exports. Recommended: the Kotlin binding's JNI glue calls
`init_android` when the library loads, with the `JNIEnv` and context it already holds,
and the variant's payload crosses as its message, as other foreign errors do.

## Hosts

- linux
- macos - the Apple targets' checks need Apple's SDK
- windows - the MSVC target's check needs MSVC
