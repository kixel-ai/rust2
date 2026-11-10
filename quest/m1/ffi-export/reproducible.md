# [S] The glue builds what the export checked

## Goal

The crate `rust2-ffi --out` writes builds against exactly the dependency versions the
export checked, on the consumer's own toolchain, or the export fails with a span: no
difference between the check and the build shows up as a glue compile error or a run-time
abort.

## Plan

Found in the 2026-11-09 quest audit:

- `--out` writes no `Cargo.lock`; only `ffi/justfile` copies one. A build that resolves a
  newer dependency can reach the wildcard arm the glue emits for a `#[non_exhaustive]`
  enum, which panics and so aborts (`ffi/src/glue.rs`). Write a lockfile beside the
  manifest, as the manifest already carries its `[patch]`es: resolved for the generated
  manifest, which adds dependencies of its own (`smallvec`, `pyo3-ffi`), with every
  package the check resolved at the version it checked, or the export fails. The recipes
  and `ffi/README.md` then build it `--locked`.
- An enum's discriminants are collected as `i128` (`ffi/src/collect.rs`) and lifted as
  `i32` (`glue.rs`, `c.rs`), with no refusal for one out of range, so a wide `repr`
  fails when the glue builds. Refuse it at export, at the variant's span.
- The export checks the whole graph on the pinned nightly (`ffi/rust-toolchain.toml`),
  and the glue then builds on the consumer's toolchain, which the generated manifest does
  not name. A build script that detects nightly, as `rustversion` and `autocfg` users do,
  can give the check an item the stable build lacks. Add a fixture with such an item and
  report what happens; recommended: refuse an exported item whose `cfg` depends on the
  release channel, and give the generated manifest the `rust-version` the facade declares.

Public API: the generated crate's `Cargo.lock` and `rust-version`.
