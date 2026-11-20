# [L] Prove leaves against the profile the library builds with

## Goal

A leaf ([`Abi::leaf`](/ffi/src/abi.rs)) is proven against the bodies the library actually
runs, in every build profile. A build whose profile differs from what the proof saw takes the
slow path in every binding instead of trusting the proof. A release build, which moq ships,
still makes its leaves leaf calls.

## Plan

Found by the review of [#911](https://github.com/kixel-ai/rust2/pull/911). The proof reads the
MIR of the export's one `cargo check`, a dev-profile check with `debug_assertions` on. The
library is built separately, usually with `--release`, as `ffi/release/package.py` and the
benchmark do. Code compiled only without debug assertions (`cfg(not(debug_assertions))`, or a
cfg a build script derives from `PROFILE` or `OPT_LEVEL`) is never seen. A scalar accessor
whose release body waits on something that waits on the GIL would then deadlock Python, or run
outside Dart's serving path. No text scan closes this gap: build scripts, proc macros, and
other crates' macros can each hide a profile-dependent cfg.

Recommended:

- Check with release settings, so the proof covers the release body. This also moves the
  API's own `cfg(debug_assertions)` items to release's side, so it is a change to what the
  whole export exports, and needs measuring.
- Have the generated crate's build script compare the profile it builds under (`PROFILE`,
  `OPT_LEVEL`, `CARGO_CFG_DEBUG_ASSERTIONS`) with the check's, which the export records. On a
  mismatch, it sets a cfg that turns every leaf back into a normal call. Python's leaf choice
  is Rust glue, so a cfg suffices. Dart's `isLeaf` is fixed in the Dart source, so it declares
  both bindings and picks one through a flag the library reports at load.

## Waiting

- Maintainer: whether the export checks with release settings rather than dev's. Options:
  release settings plus the build-time guard above (recommended: sound in every profile, and
  release builds keep their leaves); the guard alone on today's dev check (sound, but release
  builds lose every leaf); or keep the documented boundary (no work). An agent cannot take
  this: it changes which `cfg`-gated items every export exports.
