# [M] Exports that mbx can cache

## Goal

A warm `rust2-ffi` export, native or web, re-checks only the crates whose inputs changed:
every crate but the ones the driver analyzes compiles as plain rustc, which mbx caches
across checkouts and CI. The merge queue's `ffi` module and `bench/ffi`'s export rows
report the change, cold and warm.

## Plan

Measured on `nixos` on 2026-11-22: rust2 is about 3% of the host's CPU (its CI container
44 of 1,526 CPU-hours over 8.9 days), so this is about latency, for rust2's queue and for
every consumer that exports. The queue's median merge group took 9 minutes over its last
29 runs. In the traced [run](https://github.com/kixel-ai/rust2/actions/runs/44988506053)
`ffi` was nearly all of it: the moq stand-in's two exports checked moq for 2m53s (native)
and 2m00s (web), and the binding tests took about 3.5 minutes.

The cause: the driver makes itself cargo's `RUSTC_WRAPPER` (`driver/src/cargo.rs`), and
mbx then caches nothing ("RUSTC_WRAPPER is already set to .../rust2-ffi; deferring to it,
so this build is not cached"). The queue's `git clean -ffdx` also deletes `ffi/target`, so
every queue export starts cold. Locally, the `api` fixture's export took 51 s wall and
62 CPU-s cold, and 5.8 s once cargo's own target directory was kept.

Decided with the maintainer on 2026-11-22:

- ✅ The goal is cacheable exports (rejected: the whole queue's latency, with the binding
  tests; running bench-metal's rustc levers now; nothing for rust2). moq's own `just check`
  and `just test`, 211 of the 213 gated CPU-hours on `nixos` that week, are another
  agent's work in moq's tree.
- ✅ The driver wraps only the crates it analyzes; every other crate gets the leaf flags
  (`ffi/src/leaf.rs`'s `-Zalways-encode-mir -Coverflow-checks=on -Zmir-opt-level=1`)
  through cargo's rustflags and compiles as plain rustc. mbx 1.30 caches beside a
  `RUSTC_WORKSPACE_WRAPPER`, deferring only for workspace members. Rejected: keeping the
  export's target directory across queue runs, which helps only rust2's queue runner;
  waiting on mbx to cache through a custom wrapper, which would also cache a compilation
  whose side effect is the export.

Recommended:

- Pass the flags as profile rustflags (`-Zprofile-rustflags`, on the nightly the exporter
  already needs), which add to a consumer's own `RUSTFLAGS` as the driver's appended flags
  do now, where `build.rustflags` and `target.<triple>.rustflags` lose to them. Cargo then
  fingerprints the flags itself, so the `rust2-ffi-leaf` line the driver prints into
  `rustc -vV` can go. Keep them off build scripts and proc macros, as `leaf::flags` does.
- The moq stand-in is its own workspace with moq's crates as path dependencies, so a
  workspace wrapper reaches only the facade. A crate exported from inside a larger
  workspace, as moq's real `moq-ffi` is, leaves its siblings uncached under one:
  measure how much that costs, for [the mbx ask](/quest/m1/mbx-wrapper-delegate.md).
- The web export also checks std from the dev shell's MIR sysroot, or from source with
  `-Zbuild-std`; check that both of its passes, the check and the walk of the glue crate,
  hit the cache too, or say in the PR why one cannot.
- Validate with mbx's own report (`mbx explain`) on a second export from an emptied target
  directory, beside wall and CPU time from `bench/ffi/measure.py`, and against the queue's
  `ffi` time before and after.

Public API: none; the generated bindings must not change, which `just ffi surface --check`
shows.
