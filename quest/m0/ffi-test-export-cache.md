# [S] Check the ffi tests' exports through mbx

## Goal

Under mbx's shim, the exporter that ffi's tests start checks each fixture through mbx's cache, so `just ffi test` from a cold scratch directory serves the fixtures' dependencies from the store. A measurement shows the hit, such as the check's own `mbx[cache]` line or `mbx explain --last` for it.

## Plan

Found 2026-11-27 while moving ffi's test recipes to nextest: mbx runs the test processes with the `PATH` from before its shim, which holds neither the shim's session directory nor `RUST2_CARGO_SHIM`. The recipe's runner unsets `$CARGO`, as [#1055](https://github.com/kixel-ai/rust2/pull/1055) meant, but the exporter then finds the toolchain's own cargo on that `PATH`, so its check bypasses mbx under `cargo test` on main and under `cargo nextest run` alike. An `execve` trace of `deref_shadowing` with the export's target directory emptied shows the exporter running `rust-default-.../bin/cargo` in both, and no shim. #1055 measured the hit with the exporter run outside the tests.

Without the recipe's runner, nextest under mbx keeps the session directory first on the test's `PATH`, with `MBX_NEXTEST_RUN` set, but `$CARGO` names the toolchain's cargo, and the exporter, with `$CARGO` removed, still ran the toolchain's cargo. Decided 2026-11-27 (maintainer, /quest-run): ✅ the tests put `RUST2_CARGO_SHIM` first on the exporter's `PATH`, then drop ffi/justfile's `env -u CARGO` runner if it no longer does anything (rejected: leave it to the implementer; ask mbx upstream to keep its session directory on test processes' `PATH`).
