# [S] Every driver check reaches the cache

## Goal

Every check a rust2 driver runs in CI compiles through mbx, so a merge group reads the
crates it checks from main's bundle: none runs the cargo that `$CARGO` names past mbx's shim.

## Plan

[#1050](https://github.com/kixel-ai/rust2/pull/1050) found that `rust2_driver::cargo` runs
`$CARGO` with `RUSTC_WRAPPER` empty, counting on mbx's shim to set its own, but `cargo run`
and `cargo test` set `$CARGO` to the cargo the shim chains to, so a driver they start checks
with no cache. It fixed ffi/justfile's exporter runs only. The rest still start a driver that
way: ts/justfile's and go/justfile's `cargo run`, ffi/release/package.py's `cargo run` of the
exporter, and ffi's tests, whose exporter inherits `cargo test`'s `$CARGO`.

First measure each: a run that prints no `mbx[cache]` line for its check, or whose check's
target directory rebuilds as slowly from a warm store, is affected. Recommend fixing it in
the driver rather than each caller: run `cargo` from `PATH` when `$CARGO` is the cargo
`MBX_CARGO_SHIM_CHAIN` names, or ask mbx in
[jdx/mr-boxington#1015](https://github.com/jdx/mr-boxington/discussions/1015)'s discussion to
reach a `$CARGO` child itself.

## Waiting

- Maintainer: where to fix it. Options: (a) the driver detects mbx's chained cargo, keyed on
  mbx's undocumented `MBX_CARGO_SHIM_CHAIN`; (b) each caller runs the built driver directly,
  as ffi/justfile now does, and the tests clear `$CARGO`; (c) ask mbx to cover `$CARGO`
  children. Recommend (b) now, since it needs nothing undocumented, and (c) alongside.
