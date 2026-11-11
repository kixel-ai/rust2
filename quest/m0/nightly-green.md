# [M] The nightly passes

## Goal

Every job of the scheduled `nightly.yml` run passes on `main`, each failure fixed at its
cause, and a regression in a suite only the nightly runs cannot go unseen for a week again.

## Plan

Found in the 2026-11-09 quest audit: the nightly has failed every scheduled run since
2026-10-29, with no quest owning it. Decided with the maintainer on 2026-11-09: ✅ m0,
after the UniFFI fork's deletion removed the `uniffi-*` jobs, so this fixes only what
remains. Rejected: m1.

What failed on 2026-11-09 (run
[42655190735](https://github.com/moq-dev/rust2/actions/runs/42655190735), at `f7a0c279`):

- `Gate` and `TypeScript package`: rust2-ts's `bindgen` fixture differs from native in 1
  of 5073 cases, a NaN's payload through the variadic import (`max_of` in
  `ts/fixtures/bindgen/src/harness.rs`: native `18446472823932715226`, translated
  `9221120237041090560`). "Encoder in Chromium" runs after that step, so the WebCodecs
  encoder proof [#681](https://github.com/moq-dev/rust2/pull/681) added has never run in
  CI. Reproduce first; the fix is in the translation or the import's glue, not in making
  the harness blind to NaN payloads, unless JS cannot keep one through a call, which the
  PR then shows.
- "Export to Kotlin on Android" at API 35: `the GC frees lent bytes nothing reaches`, for
  at least three nights.
- `just ffi moq` likely fails on `main` since
  [#668](https://github.com/moq-dev/rust2/pull/668): the exporter now refuses every
  whole-crate `pub use` (`ffi/src/collect.rs`), and `ffi/moq/ffi.toml` excludes nine of
  the eleven at the pin but not hang's and moq-tokio's `pub use moq_net;`. Not yet run in
  CI, since no nightly ran after the merge. [The pin bump](/quest/m1/ffi-refusals/moq-pin.md)
  removes them if it lands first.

Open: what keeps it green. Recommended: the nightly's failure opens or updates one GitHub
issue the maintainer sees, and `just ffi moq`'s export step joins `just ci` when `ffi/`
or the pin changes, if it fits the gate's budget; measure it and report.

Public API: none.
