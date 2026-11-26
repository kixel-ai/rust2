# [M] Deterministic simulated cycles in sim (--cache-sim=yes --branch-sim=yes)

## Goal

`swole sim` runs Valgrind Callgrind with `--cache-sim=yes --branch-sim=yes`, parses L1/LL cache misses and branch mispredicts alongside `Ir`, computes a deterministic estimated-cycles metric (matching CodSpeed's simulation model), reports `Ir`, cache/branch misses, and estimated cycles in tables and `metrics.json`, and hashes the simulation configuration into the fingerprint so memory-layout and branch regressions surface with 0% flake.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include deterministic cache and branch simulation in `swole sim` (the CodSpeed simulation replacement). Instruction count alone misses cache-unfriendly data layouts and branch misprediction regressions; Callgrind's cache and branch simulator catches them deterministically at ~2–3× Valgrind's base runtime.

Recommendations for the implementer:

- Pass `--cache-sim=yes --branch-sim=yes` (with explicit pinned cache geometry flags, e.g. `--I1=... --D1=... --LL=...`, so Valgrind never varies its simulated cache sizes by host CPUID) and hash those exact geometry flags into `Fingerprint`.
- Parse `Ir`, `Dr`, `Dw`, `I1mr`, `D1mr`, `D1mw`, `ILmr`, `DLmr`, `DLmw`, `Bc`, `Bcm`, `Bi`, and `Bim` from Callgrind's `events:` and `totals:` lines (and per-function costs so [per-function attribution](/quest/m2/swole/callgrind-attribution.md) can also attribute cycle regressions to the offending function).
- Document the exact estimated-cycles formula (e.g. `Ir + L1_penalty * (I1mr + D1mr + D1mw) + LL_penalty * (ILmr + DLmr + DLmw) + branch_penalty * (Bcm + Bim)`) with standard hardware-grounded constants, and record `estimated_cycles` alongside `instructions` in `metrics.json`.
- Verify with differential fixtures: two functions with nearly identical instruction counts where one accesses memory sequentially and the other strides across cache lines (or predicts vs mispredicts branches), proving `estimated_cycles` clearly separates them with 0% run-to-run variance.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - uses `swole sim` and the unified fingerprint

## Hosts

- linux - Valgrind Callgrind cache and branch simulation
