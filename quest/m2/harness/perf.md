# [M] Native instruction counts from perf counters

## Goal

The harness counts a `#[bench]` function's user-space instructions (`instructions:u`) through Linux perf counters at native speed, for quick base-vs-head compares on one host, without Valgrind's 20 to 50 times slowdown.
A host that exposes no counters fails the run, saying so.

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): ✅ perf counters beside Valgrind (rejected: Valgrind only).

Recommendations for the implementer:

- Keep these counts local: never write them to `kixel-metrics`. GitHub's hosted runners expose no counters, so CI's series stay on Valgrind. Like Valgrind's, these counts compare only between builds run on the same host, since code that picks a path by CPU feature, such as rind's AES and SHA or moq's SIMD, takes another path elsewhere.
- Report a compare only when base and head ran on the same host, and refuse multiplexed counters, as moq's `SESSION_ALLOCS` mode does, rather than scale an undercount.
- Threads make these counts noisy; say so in the output for a benchmark that spawns any.

## Required

- [Counts](/quest/m2/harness/counts.md) - the `#[bench]` API this measures
