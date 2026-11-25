# [M] A count does not move with its job's paths or environment

## Goal

rust2-bench launches every counted job so its initial stack is the same whatever the length of its checkout's path and the size of the host's environment, so a benchmark's instruction count is the same for the same code across checkouts and CI runs on hosts of one fingerprint.
Done when all of moq's counted benchmarks agree exactly between two checkouts whose paths differ in length, and between a run on `main` and one on a branch with a longer name, as a Kixel Quest bisect step runs, once on an idle host and once on a host loaded to its core count.

## Plan

[kixel-ai/rust2#1023](https://github.com/kixel-ai/rust2/pull/1023) gave every counted process a fixed-seed stream for randomness and a deterministic clock, the half of this quest decided 2026-11-24 (maintainer, `/quest-run` pick on [moq-dev/moq#9574](https://github.com/moq-dev/moq/pull/9574)).
What it measured on moq at [moq-dev/moq#9574](https://github.com/moq-dev/moq/pull/9574)'s head, on a shared 24-core host, no run of it idle:

- Runs at load 24 to 40 agree on all 1,284 counts: two in one checkout, and two in checkouts whose paths have the same length.
- Across two checkouts whose paths differ in length (`.tmp/moq-a` and `.tmp/moq-second-checkout`), 288 of 1,284 agree, the worst 10.5% apart (`decode_clock_drain::backward/4096B[10000]`), though every benchmark binary is byte-identical, as moq-dev/moq#9574 made them.
  So what moves them is the length of the paths in each job's arguments and environment (the executable's path, `CARGO_MANIFEST_DIR`, the library path, `PWD`), which moves the initial stack, and everything the benchmark lays out on it.
  A filler variable of 1, 36, and 61 bytes moved `cmaf-interleaved-import::4tracks[1runs-1bytes-1024chunk]` between 5,071,493 and 5,071,501, so the environment's size alone does it too.
- Unmeasured, but the same mechanism: a CI run's environment varies too, such as `GITHUB_REF` between the nightly on `main` and a bisect step on `kixel-bisect/<run>/<commit>`, so a bisect step may read a different count than the nightly did for the same commit.
- Not yet explained: one run that started at load 6 disagreed with two later ones at load 20 to 40 on 30 counts, all moq-mux's (`containers`, `ts_programs`, `ts_video`), by at most 0.024% (`cmaf-interleaved-import::4tracks[1runs-1bytes-7484chunk]`, 1,236 instructions). Natively, that benchmark runs one thread and waits only with a zero timeout, and its binary has no `rdtsc`, so neither the clock nor a timeout explains it; find the cause before calling this done.

How this splits from [`bench-compare-one-path`](https://github.com/kixel-ai/rust2/blob/quest-plan/bench-followups/quest/m0/bench-compare-one-path.md), planned in [kixel-ai/rust2#1010](https://github.com/kixel-ai/rust2/pull/1010): that quest counts `compare`'s base and head from one directory, which removes their path difference along with any the base's build compiles in, and it rejected neutralizing each cause for `compare`. `run` across checkouts and CI runs has no one directory to share, so its launch needs this either way. Recommended: keep both; this quest fixes every job's launch, `run`'s and `compare`'s alike, and bench-compare-one-path keeps the build-side paths.

## Waiting

- Maintainer: how a job's launch stops depending on paths and the environment. Options:
  1. (Recommended) Pad: rust2-bench adds one filler variable to each job's environment that brings its arguments and environment to a fixed total size, so the initial stack pointer is the same; refuse a launch larger than that size. Simplest, and it covers CI's variables too. First measure that a fixed stack pointer is enough when the strings themselves still sit at different offsets, with the two checkouts above.
  2. Fixed-length paths: launch each job through a fixed-length link to the workspace, with its paths rewritten onto it, as the same-length checkouts above did by hand. Exact for paths, but CI's variables still vary, so it needs (1) as well.
  3. A canonical environment: launch with Cargo's variables and a fixed few of the host's. Changes what a benchmark sees, and still needs (1) or (2) for the paths Cargo sets.

## Related

- [`bench-compare-one-path`](https://github.com/kixel-ai/rust2/blob/quest-plan/bench-followups/quest/m0/bench-compare-one-path.md), planned in [kixel-ai/rust2#1010](https://github.com/kixel-ai/rust2/pull/1010) - one directory for `compare`'s two sides
