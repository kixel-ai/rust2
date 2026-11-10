# [S] The glue builds on the channel the export checked

## Goal

No item the export checks on the pinned nightly is missing when the glue builds on the
consumer's toolchain: an exported item that a build script's release-channel detection
enables, as `rustversion` and `autocfg` users do, is refused at export with a span, rather
than a glue compile error.

## Plan

The export checks the whole graph on the pinned nightly (`ffi/rust-toolchain.toml`), with
build scripts reading that nightly from `RUSTC`. The generated manifest now carries the
facade's `rust-version` and a `Cargo.lock` pinned to the check's resolution, but not the
channel.

Add a fixture whose build script emits a `cfg` on nightly and gates an exported item on
it, and report what the glue build does on stable.

## Waiting

- Maintainer: how to refuse a channel-dependent item. Options: (a) refuse an exported
  item whose `cfg_trace` names a `--cfg` a build script set, which also refuses
  target-only detection; (b) run the check's build scripts against a stable `RUSTC`, so
  they see the consumer's channel; (c) accept the gap. Recommended: (b), since it removes
  the difference instead of guessing which `cfg`s depend on it.
