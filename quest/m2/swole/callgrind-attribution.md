# [S] Per-function attribution from Callgrind dumps in every sim compare

## Goal

`swole sim <base>` (and bisect output) parses per-function costs from the Callgrind dumps it already produces and prints the top shifted functions beside each regressed or improved benchmark (such as `+12.4% in moq_net::frame::decode`), so every regression arrives with its location at zero extra runtime.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include deterministic Callgrind per-function attribution in the `swole` epic. The Callgrind dumps already hold per-function inclusive and self costs for every benchmark; parsing them costs milliseconds and turns a bare percentage change into an actionable diagnosis.

Recommendations for the implementer:

- Extend `callgrind.rs` to parse function definitions (`fn=(id) name`, `ob=(id)`, `fl=(id)`), self costs, and call-entry inclusive costs (`cfn=...`, `calls=...`), handling Callgrind's name compression (`(id)`) and demangling Rust symbols (`rustc-demangle`).
- Diff per-function self and inclusive deltas between `<base>` and head for each benchmark that moved, ranking the largest contributors to the net delta and attributing added/removed functions cleanly.
- Keep output concise and readable in Markdown: show the top 3–5 functions explaining the majority of each changed benchmark's delta, omitting unchanged benchmarks.
- Verify with unit tests on synthetic multi-function Callgrind profiles (compressed IDs, inlined/called frames, added/removed symbols) and an end-to-end integration test comparing two commits where one helper function does extra work and confirming the compare output names that exact function.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - hooks into `swole sim <base>` reporting
