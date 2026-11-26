# [M] Learned A/A noise floors per host

## Goal

`swole` measures each benchmark against itself (A/A) per host fingerprint, records each benchmark's empirical spread per metric, and reports `compare` changes relative to that benchmark's measured noise floor (signal-to-noise ratio / outside A/A band) instead of fixed percentage thresholds.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Defer to m3 (rejected: including in the initial m2 `swole` epic; dropping). Needs multi-metric series history first and eliminates arbitrary acceptance thresholds once in place. Carried into `kixel-ai/swole`'s quest tree when [Move rust2-bench to kixel-ai/swole](/quest/m1/swole-repo.md) executes.

Recommendations for the implementer:

- Provide a self-calibration command (e.g. `swole calibrate` or periodic A/A runs stored by host fingerprint) that runs each benchmark against itself across repeat pairs/runs and stores robust spread estimates (MAD / 95th percentile A/A delta) per benchmark and metric.
- In `compare` tables and `metrics.json`, annotate each change with whether it exceeds the benchmark's host noise floor (e.g. `4.2× noise`), keeping deterministic zero-noise metrics (`sim` single-threaded `Ir`, allocations) flagged on any non-zero move.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - measures per-metric spread across the unified modes
