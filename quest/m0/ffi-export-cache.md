# [S] Measure the cached export

## Goal

The merge queue's `ffi` module and `bench/ffi`'s export
rows report a warm `rust2-ffi` export, native and web, beside a cold one, and the cost of a
facade's sibling workspace crates, which mbx leaves to the wrapper, is known.

## Plan

The export now wraps only the workspace's own crates (`RUSTC_WORKSPACE_WRAPPER`) and is
cargo's `RUSTC` for the rest, each with the consumer's rustflags and then the export's in
`CARGO_ENCODED_RUSTFLAGS`, so mbx sees every flag; `tests/fixtures.rs`'s `cached` checks
the command lines a cache gets. mbx 1.31 keys the export's `-Z` flags
([jdx/mr-boxington#1003](https://github.com/jdx/mr-boxington/issues/1003)), and the queue
pins it. Measured 2026-11-25 on **desktop** with a release rust2-ffi, a second export from
an emptied target (`bench/ffi/README.md`'s "The export's cache"):

- Native: every crate but the facade served, 332 hits.
- Web over the dev shell's MIR sysroot: the check serves only build scripts and proc
  macros (76 hits); its 104 wasm32 crates are neither looked up nor stored, since mbx maps
  no `--sysroot` under `/nix/store` ("absolute path has no stable cache mapping").
- Web over `-Zbuild-std`: every moq crate served (196 hits), and std's 13 crates, whose
  sources are under `/nix/store` too, compiled.
- Both web runs: the walk of the glue crate, a second cargo, bypassed both of its crates.

Then, from the queue's `ffi` time before and after over its merge groups, both on
GitHub-hosted runners since [#991](https://github.com/kixel-ai/rust2/pull/991) moved the
queue off the host's persistent mbx store, and `bench/ffi/measure.py` with `mbx explain` on
a second export from an emptied target:

- Both passes of the web export, the check and the walk of the glue crate, served from
  the cache, with std from the dev shell's MIR sysroot and from `-Zbuild-std`, or say why
  one cannot be.
- A warm row in `bench/ffi`'s export table, which measures uncached exports today.
- A crate exported from inside a larger workspace, as moq's real `moq-ffi` is, whose
  siblings mbx leaves uncached under the wrapper: how much they cost, for
  [the mbx ask](/quest/m1/mbx-wrapper-delegate.md).
