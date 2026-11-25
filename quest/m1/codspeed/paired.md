# [L] Paired base-vs-head wall-clock timing

## Goal

The harness times a `#[bench]` function's wall-clock cost by running the base and head builds in pairs in one session, each pair's order randomized, so drifting or recurring host load biases neither side.
Shared-host load still widens the interval and can't be removed, only balanced.
It reports each commit's value as a Kixel Quest `kixel-metrics` sample, the base's naming its commit, and prints the head/base ratio with a confidence interval.

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): ✅ paired timing (rejected: Criterion-style statistics over one commit, whose values host load moves). Our hosts are busy and shared, so only a paired comparison is trustworthy on them.

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2): ✅ this ships before moq and rind rewrite their Criterion benches to `#[bench]`, as that rewrite's only prerequisite, so it moved from the m2 harness epic to [the CodSpeed epic](/quest/m1/codspeed/README.md) (rejected: rewrite first and stop timing until this ships; keep a Criterion copy of each bench for timing).
Done when it times what Criterion times for those two today, so their rewrites can delete Criterion:

- moq's Nightly, which times every bench, including the async ones (24 bench files build a tokio runtime and `block_on` in the body) and the 64 `iter_custom` ones, which keep an input across iterations and time only part of each, such as `rs/moq-stats/benches/producer_tick.rs`'s untimed `relay.touch()` before each timed `tick()`. Close that gap in `#[bench]` itself, such as an untimed per-iteration step on a kept input, rather than leave it to each benchmark.
- rind's native job, which times one build's rustcrypto rows against aws-lc-rs and ring on several CPUs and targets and reads Criterion's `benchmark.json`: one build timed alone, its rows compared with each other rather than with a base.

Criterion's `sample_size`, `measurement_time`, and `Throughput` settings have no counterpart: the harness picks its own sampling, configless, and a rate is the time over a size the benchmark's name already carries.

Recommendations for the implementer:

- tango-bench 0.9 loads the base build as a shared library into the head's process and alternates them; study it before choosing between that and two processes.
- moq's Nightly does paired comparisons with scripts today (`bench/run.sh`, its `--criterion` path with `BENCH_BASE`, and moq's `just bench <base>` quest). Measure against it, so moq's migration can delete them.
- Instruction counts don't need pairing, since host load doesn't move them, but multi-threaded ones vary between runs (up to 2.57% in moq), so a count comparison still checks repeat runs' spread, as [the runner](/bench/runner/README.md) measures. This mode is for the wall-clock effects counts miss, such as syscalls, I/O, and contention.
