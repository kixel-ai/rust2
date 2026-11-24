# A benchmark harness to replace Criterion

## Goal

rust2 ships a Rust benchmark harness that replaces Criterion and CodSpeed's compatibility crates in moq-dev/moq, kixel-ai/rind, and here: benchmarks are plain functions marked `#[bench]`, measured by instruction count under Valgrind for Kixel Quest, by native perf counters for quick local compares, and by paired base-vs-head wall-clock timing.

What stays here once every child has merged: the harness's docs, its line in rust2's charter beside the runner's, and the migration quests for moq and rind, added to each repository's own tree as follow-ups of the last child's PR.

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): ✅ plan a Criterion replacement here, beside [the runner](/bench/runner/README.md) (rejected: Gungraun for counts, which leaves every benchmark written twice, once for counts and once for Criterion's wall-clock; keeping Criterion). The maintainer has "never been a huge fan" of Criterion, and Divan, the nicer API, has had no release since April 2025.

m2: moq and rind are the named consumers, and neither waits on it, since the runner already serves their benchmarks as they are written.
