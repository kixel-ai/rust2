# [S] Trim the build-speed benchmark

## Goal

The [build-speed benchmark](/bench/build/README.md) costs CI only when its result can
change: `bench.yml` runs when a pin or the benchmark itself changes and on manual
dispatch, not nightly; the `Bench (moq nix)` job and `bench/build/nix.*` are deleted, and
so are the `pure` corpora (regex, wasmparser) and their job, with every table they
produced kept in the README.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ trim it, in m0 since it
is CI cost. Build speed is outside rust2's goal as the maintainer restated it, any Rust
crate callable from any language, so the track keeps only what costs nothing. Rejected:
keeping the nightly schedule and fixing the nix job; removing the track.

- The nightly `bench.yml` costs about 3.5 runner-hours on inputs that are all pinned (the
  moq revision, the nightly, ra-jit), and a pin change already runs it on its PR.
- `Bench (moq nix)` has failed every scheduled run since 2026-11-02, and the choice it
  informed was made on 2026-10-26 (`bench/build/README.md`).
- The `pure` corpora were built for the subset compiler, closed on 2026-11-06
  ([m4](/quest/m4/README.md#plan)).
- What stays: the four quests that wait on upstream
  ([Cranelift](/quest/m3/cranelift-unwind.md), [relink](/quest/m3/relink.md),
  [TPDE](/quest/m4/tpde.md), [the fork ledger](/quest/m4/fork.md)), the benchmark they
  measure with, and rust2-lint, which moq's CI plans to adopt.

Public API: none.
