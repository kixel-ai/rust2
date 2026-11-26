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
target directory rebuilds as slowly from a warm store, is affected. Then fix each affected run
by having its caller unset `$CARGO` under mbx's shim, as ffi/justfile's `exporter` runner now
does, and having the tests clear it.

Decided 2026-11-26 (maintainer, /quest-run): fix it in each caller (b), plus raise (c) in
[jdx/mr-boxington#1015](https://github.com/jdx/mr-boxington/discussions/1015)'s discussion
alongside. Rejected: (b) only; (a) the driver keying on mbx's undocumented
`MBX_CARGO_SHIM_CHAIN`; (c) only. Unverified: a consumer running our tool as a cargo
subcommand under mbx probably skips the cache the same way, which only (c) would fix.
