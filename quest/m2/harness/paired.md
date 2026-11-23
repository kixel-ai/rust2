# [L] Paired base-vs-head wall-clock timing

## Goal

The harness times a `#[bench]` function's wall-clock cost by running the base and head builds interleaved in one session, so host load hits both equally.
It reports each commit's value as a Kixel Quest `kixel-metrics` sample, the base's naming its commit, and prints the head/base ratio with a confidence interval.

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): ✅ paired timing (rejected: Criterion-style statistics over one commit, whose values host load moves). Our hosts are busy and shared, so only a paired comparison is trustworthy on them.

Recommendations for the implementer:

- tango-bench 0.9 loads the base build as a shared library into the head's process and alternates them; study it before choosing between that and two processes.
- moq's Nightly does paired comparisons with scripts today (`bench/run.sh`, its `--criterion` path with `BENCH_BASE`, and moq's `just bench <base>` quest). Measure against it, so moq's migration can delete them.
- Instruction counts don't need pairing: they are the same on every run. This mode is for the wall-clock effects counts miss, such as syscalls, I/O, and contention.

## Required

- [Counts](/quest/m2/harness/counts.md) - the `#[bench]` API this times
