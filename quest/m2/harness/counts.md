# [L] Benchmarks as attributed functions, counted under Valgrind

## Goal

A benchmark is a plain function marked `#[bench]`, its arguments and sizes declared in the attribute and its setup passed in as parameters, registered at link time, with no closures.
The harness counts each one's instructions under Valgrind, absorbing [the runner](/bench/runner/README.md)'s parallel runs, filter, `kixel-metrics` output, and local compare, so a benchmark written for it needs neither Criterion nor CodSpeed's crates.
This repository's own Rust benchmarks, starting with `ffi/moq/bench/export.rs`, move to it.

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): ✅ attribute registration, in the style Divan pioneered (rejected: Criterion-like closures and groups, `c.bench_function(name, |b| b.iter(...))`, which migrate by renaming but keep the callback-heavy shape). Agents migrate moq's and rind's benchmarks mechanically either way, and attributes follow the rule against callback parameters.

Recommendations for the implementer:

- Setup and teardown stay outside the measured region, as the Callgrind markers already allow; a parameter's value is built before the benchmark's `START_INSTRUMENTATION`.
- Keep the runner's CLI and its output; the harness is a second way to write benchmarks it runs, not a second tool. Once the last consumer moves, the CodSpeed marker path can go.
- The non-Rust harnesses (C, Node, Python) keep emitting the same markers through the instrument hooks; only Rust benchmarks change here.

Public API: a new crate and its attribute macro.
