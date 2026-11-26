# [M] Scaling and complexity regression detection across args

## Goal

`swole` fits asymptotic scaling across numeric `args = [...]` sweeps (distinguishing `O(1)`, `O(log n)`, `O(n)`, `O(n log n)`, and `O(n²)`) on instruction counts, simulated cycles, allocations, and wall-clock time, and flags complexity-class regressions in `compare` even when a single benchmark size barely moved.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Defer to m3 (rejected: including in the initial m2 `swole` epic; dropping). Easy to layer once multi-metric runs land, and no consumer blocks on it yet. Carried into `kixel-ai/swole`'s quest tree when [Move rust2-bench to kixel-ai/swole](/quest/m1/swole-repo.md) executes.

Recommendations for the implementer:

- Group cases sharing the same base benchmark name whose arguments parse as positive numbers (`[n]`) with at least 3 distinct sizes spanning at least an order of magnitude.
- Fit candidate complexity models (`O(1)`, `O(log n)`, `O(n)`, `O(n log n)`, `O(n²)`) via least squares / log-log slope + BIC/AIC penalty so simpler models win ties, and flag a regression when the best-fit class worsens between `<base>` and head (e.g. `O(n)` → `O(n log n)` or `O(n²)`).
- Verify with synthetic `#[bench(args = [100, 1_000, 10_000])]` fixtures for each complexity class and a compare fixture that flips a linear scan into a quadratic loop.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - reads multi-metric outputs across parameterized cases
