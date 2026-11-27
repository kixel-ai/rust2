# [M] Measure dev dynamic linking and mbx unconsulted cascades

## Goal

Measure in `rust2`'s [build-speed benchmark](/bench/build/README.md) whether dynamic linking for dev test builds (`workspace-dylib` / `-C prefer-dynamic`) and mitigating `mbx`'s `unconsulted` cascades (`private-dependency` and `no-prediction`) cut incremental edit-test CPU on `moq` by >30%, and either adopt the winning configuration or file concrete upstream `mbx` improvements with benchmarks.

Non-goals: duplicating [relink](/quest/m3/relink.md), [TPDE](/quest/m3/tpde.md), or [ffi-export-cache](/quest/m0/ffi-export-cache.md); shipping a complex build tool that breaks `rust2`'s "keeps only what costs nothing" rule.

## Plan

Decided 2026-11-26 (maintainer, `/quest-plan` after 24h multi-host CPU audit):
- ✅ Place in `m3`, matching `rust2/quest/README.md` ("m3 holds work that waits on evidence or has no consumer yet: upstream's build-speed levers, which the build-speed benchmark measures").

Baseline (2026-11-26, 2.4h `mbx` session audit on `fedora` and `nixos`):
- `unconsulted` actions accounted for **73% of compiler CPU on `fedora`** (338.6 of 464 CPU-min) and **72% on `nixos`** (498.7 of 692 CPU-min):
  - `no-prediction`: 169.9 CPU-min in 2.4h on `fedora` (ad-hoc agent `cargo check --workspace --tests` runs changing `--emit` (`metadata` vs `link,metadata`) and feature unification, leaving up to 478 crates unconsulted in a single build).
  - `private-dependency`: 154.7 CPU-min in 2.4h on `fedora` (`learned_incremental = true` marks an edited hub crate like `moq-net` private and cascades unconsulted compiles to all 41 downstream crates).
- Linking large test binaries over `moq-net` and its 41 dependents repeatedly copies and links the same unchanged dependency rlibs.

Approach:
1. **Dynamic linking lever (`bench/build/`):** Benchmark `-C prefer-dynamic` and a thin dev `dylib` hub crate (splitting `moq-net`'s unchanged third-party closure into a shared library for dev test builds only) on the `bench/build` corpus (`moq` incremental 1-line edit + test link), measuring wall time, CPU seconds, peak RSS, and `mbx` cache hit rate.
2. **`mbx` `unconsulted` cascade diagnosis:** Measure on the same corpus:
   - Toggling or tuning `learned_incremental` thresholds (`private-dependency` cascade when editing a hub crate with >N dependents), comparing warm single-worktree vs multi-worktree/cross-host CPU.
   - Identifying why ad-hoc `cargo check` / `cargo test` flags trigger `no-prediction` storms and whether a minimal `mbx` manifest fix or canonical cargo-flag guard eliminates them.
3. Record results in `bench/build/README.md`, keep only levers that pay for themselves cleanly, and file upstream `jdx/mr-boxington` issues/PRs (with maintainer approval) or follow-up adoption quests for winners.

## Related

- [relink](/quest/m3/relink.md) - upstream incremental linking lever
- [TPDE](/quest/m3/tpde.md) - upstream fast codegen backend lever
- [ffi-export-cache](/quest/m0/ffi-export-cache.md) - exporter caching in `ffi`
