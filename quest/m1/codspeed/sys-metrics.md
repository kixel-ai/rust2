# [M] System metrics beside each wall-clock time

## Goal

Every benchmark `rust2-bench time` times also records its user and system CPU time, its voluntary and involuntary context switches, its allocations, and its peak RSS, each as its own measure in the table and in `metrics.json` beside its wall time, alone and paired with a base.
It lands before moq's and rind's rewrites to `#[bench]` (moq-dev/moq's `quest/m9/bench-harness.md` and kixel-ai/rind's `quest/m2/bench/harness.md`), so their benches are written once.

Non-goals: these measures under a count, which Valgrind and perf counters already make exact.

## Plan

Decided 2026-11-25 (maintainer, pick round on [kixel-ai/rust2#1033](https://github.com/kixel-ai/rust2/pull/1033)): ✅ keep this quest (rejected: drop it). moq's and rind's rewrite quests wait on it.

Decided 2026-11-25 (maintainer, in [kixel-ai/rust2#1033](https://github.com/kixel-ai/rust2/pull/1033)): ✅ CPU time, context switches, allocations, and peak RSS (rejected: CPU time and context switches only; adding them inside #1033). The maintainer: "system level metrics seem quite useful".

- CPU time and context switches come from `getrusage(RUSAGE_SELF)` read around each sample, so a server or runtime thread's busy time counts, and each is a median per call like the wall time, paired by per-pair ratios with a base. moq's ~20 self-timed `iter_custom` sites that measure thread CPU or a server's busy time can then become plain bodies; self-timing (`Iterations` returning `Duration`) stays only for those that time a slice of one thread.
  A count can be 0 on both sides of a pair, as context switches often are, which leaves that pair no ratio: say how a pair compares then. Recommended: such a measure's change is the ratio of the two sides' medians, with no interval; with a base median of 0, both measured values stay in the table and `metrics.json`, and the change is left out.
  A benchmark with `before`/`after` steps runs them between calls inside the sample, so its reads go around each call instead, or the steps' CPU counts too; say which and measure the read's cost.
- Allocations count through a global allocator the harness defines, wrapping `System`, with cheap per-thread counters summed at each sample's end: the count and the bytes allocated per call. A bench that sets its own `#[global_allocator]`, or links a crate that does, must fail to compile loudly: rustc already refuses two in one binary, so check its error names the harness's, and say in the harness README how to drop the other. Measure what counting costs on wall time with an A/A of the fixture counting and not, and record it in the PR.
- Peak RSS: recommended, the process's own `ru_maxrss`. Each benchmark already runs in a process of its own, one per side when paired, so the fresh-process condition already holds at no cost, and RSS covers what the heap misses: mappings, stacks, and the allocator's retention. It includes the setup's peak, which is the regression moq's large fixtures (relays of 10k paths) would show; a body's transient peak below the setup's stays hidden. Rejected: the allocator's peak live bytes, which sees only the heap. It is one value per process, so a pair reports the two sides' values and their ratio, with no interval. Linux reports `ru_maxrss` in KiB and macOS in bytes.
- Measure names, recommended: `user time`, `system time`, `voluntary context switches`, `involuntary context switches`, `allocations`, `allocated bytes`, and `peak RSS`, each with the `(<host>)` label the wall time has, since a runtime sized by the CPU count allocates and switches by it too. Units `ns`, `count`, and `bytes`.
- The harness's sample line grows these fields. A base whose harness predates them still pairs its wall time, with the new measures timed as none on its side, so the first runs after it lands still report, as a base's CodSpeed benchmarks do now.

## Related

- [The epic](/quest/m1/codspeed/README.md) - the rewrites this lands ahead of
