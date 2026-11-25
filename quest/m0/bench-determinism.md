# [M] Every counted job repeats its count

## Goal

rust2-bench runs every counted job with deterministic randomness and a deterministic clock, and launches it with a fixed environment and fixed-length paths, so a benchmark's instruction count is the same for the same code across runs, checkouts, and host load.
The seed and clock enter the fingerprint, so the change starts new Kixel Quest series instead of reading as regressions, and a binary that would bypass them is refused.
Done when all of moq's counted benchmarks (1,284 on 2026-11-24) agree exactly across two full runs in two checkouts, on a loaded host.

## Plan

Decided 2026-11-24 (maintainer, `/quest-run` pick on [moq-dev/moq#9574](https://github.com/moq-dev/moq/pull/9574)): ✅ the fixed seed and clock live in rust2-bench, one change covering every benchmark on any host (rejected: a paused clock in each moq benchmark, which can't reach the HashMap seeds inside library code).
This supersedes, for randomness and the clock, the non-goal in [kixel-ai/rust2#1010](https://github.com/kixel-ai/rust2/pull/1010)'s `bench-compare-one-path` that left noise inside a benchmark to each repository; whichever lands second aligns that line.

What #9574 measured on rust2-bench at kixel-ai/rust2@6ead6d56:

- The measured code takes the same path every run; what varies is glibc's allocator state going into the measured iteration, so any change to earlier work moves the count.
- Rust's per-process random HashMap seed is one cause. A fixed-seed `getrandom` stream made all of moq's `stats`, `retirement`, and `announce` counts repeat, raising full-load agreement from 527 to 900 of 1,284. A constant value hangs QUIC connection-ID generation, so it must be a seeded stream, not a constant.
- Host clock reads are the other: tokio's real clock, or kio's fallback to std outside a runtime. A fake clock advancing 1 µs per read made all 1,284 counts agree exactly across two runs at 32 jobs on a host already at load 32. 53 benchmarks shifted more than 1% from current counts (worst `auth/waiting`, -26%).
- Both were test shims loaded into the benchmark processes; the implementation is open.
- Location: #9574 makes moq's binaries byte-identical across checkouts and `CARGO_HOME`s. What's left is that each count gets the host's whole environment, whose size shifts the stack by up to 12 instructions.

Once this ships, moq bumps rust2, counts every benchmark twice in two checkouts, then drops `--jobs 1` from its `codspeed.yml` and its wall-clock caveats (moq's `quest/m0/repeatable-instruction-counts.md`).

## Related

- [`bench-compare-one-path`](https://github.com/kixel-ai/rust2/blob/quest-plan/bench-followups/quest/m0/bench-compare-one-path.md), planned in kixel-ai/rust2#1010 - the location half for `compare`
