# [S] A benchmark harness to replace Criterion

## Goal

rust2 ships a Rust benchmark harness that replaces Criterion and CodSpeed's compatibility crates in moq-dev/moq, kixel-ai/rind, and here: benchmarks are plain functions marked `#[bench]`, measured by instruction count under Valgrind for Kixel Quest, by native perf counters for quick local compares, and by paired base-vs-head wall-clock timing.

What stays here once every child has merged: the harness's docs and its line in rust2's charter beside the runner's. Paired timing, and moq's and rind's move to the harness, belong to [the CodSpeed epic](/quest/m1/codspeed/README.md).

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): ✅ plan a Criterion replacement here, beside [the runner](/bench/runner/README.md) (rejected: Gungraun for counts, which leaves every benchmark written twice, once for counts and once for Criterion's wall-clock; keeping Criterion). The maintainer has "never been a huge fan" of Criterion, and Divan, the nicer API, has had no release since April 2025.

m2: moq and rind are the named consumers, and neither waits on it, since the runner already serves their benchmarks as they are written.

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2): ✅ paired timing moves to [the CodSpeed epic](/quest/m1/codspeed/README.md) in m1, where moq's and rind's rewrites to `#[bench]` wait on it, and those rewrites are planned in their own trees now rather than after this epic's last child. Perf counters stay here, since no rewrite needs them.
