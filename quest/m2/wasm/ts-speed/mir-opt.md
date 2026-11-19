# [S] rustc's MIR passes before translation

## Goal

rust2-ts reads MIR with rustc's value numbering (GVN), scalar replacement (SROA), and jump
threading enabled, inlining still off, where the differential tests pass and the benchmark
shows a gain; each pass left off says why.

## Plan

The profile reads MIR at the dev opt level (`ts/src/profile.toml:12`), and no commit tried
these passes. `expected/bench.ts:554` rebuilds a `NAMES` array to read its length, and
`:562` rebuilds an array literal on every loop pass. Release rustc runs these passes, and
they are where [Rustlantis](https://dl.acm.org/doi/10.1145/3689780) found miscompiles, so
the differential tests decide.

Public API: none.
