# [M] Deterministic simulated cycles in sim (--cache-sim=yes --branch-sim=yes)

## Goal

`swole sim` runs Valgrind Callgrind with `--cache-sim=yes --branch-sim=yes`, parses L1/LL cache misses and branch mispredicts alongside `Ir`, computes a deterministic estimated-cycles metric (matching CodSpeed's simulation model), reports `Ir`, cache/branch misses, and estimated cycles in tables, the full report artifact, and bounded `metrics.json` headline series, and hashes the simulation configuration into the fingerprint so memory-layout and branch regressions surface with exact repeatability on single-threaded benchmarks and repeat-sample spread on multithreaded ones.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include deterministic cache and branch simulation in `swole sim` (the CodSpeed simulation replacement). Instruction count alone misses cache-unfriendly data layouts and branch misprediction regressions; Callgrind's cache and branch simulator catches them at ~2–3× Valgrind's base runtime.

Recommendations for the implementer:

- Pass `--cache-sim=yes --branch-sim=yes` (with explicit pinned cache geometry flags, e.g. `--I1=... --D1=... --LL=...`, so Valgrind never varies its simulated cache sizes by host CPUID) and hash those exact geometry flags into `Fingerprint`.
- Parse `Ir`, `Dr`, `Dw`, `I1mr`, `D1mr`, `D1mw`, `ILmr`, `DLmr`, `DLmw`, `Bc`, `Bcm`, `Bi`, and `Bim` from Callgrind's `events:` and `totals:` lines (and per-function costs so [per-function attribution](/quest/m2/swole/callgrind-attribution.md) can also attribute cycle regressions to the offending function).
- Document the exact estimated-cycles formula (e.g. `Ir + L1_penalty * (I1mr + D1mr + D1mw) + LL_penalty * (ILmr + DLmr + DLmw) + branch_penalty * (Bcm + Bim)`) with standard hardware-grounded constants, emit headline `instructions` and `estimated_cycles` series to `metrics.json` within Kixel Quest's 1 MiB limit, and place detailed cache/branch breakdowns in the full report artifact.
- Reserve exact 0%-variance claims for single-threaded workloads that demonstrate repeatability; for multithreaded benchmarks (where thread scheduling can shift counts by up to 2.57% in `moq`), detect multi-thread execution and report variability from repeated samples instead of flagging scheduling noise as a regression.
- Verify with differential fixtures: two single-threaded functions with nearly identical instruction counts where one accesses memory sequentially and the other strides across cache lines (or predicts vs mispredicts branches), proving `estimated_cycles` clearly separates them with 0% run-to-run variance.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - uses `swole sim` and the unified fingerprint

## Hosts

- linux - Valgrind Callgrind cache and branch simulation
