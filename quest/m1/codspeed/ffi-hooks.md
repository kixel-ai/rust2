# [S] The binding benchmark brackets its cases with Valgrind's own client requests

## Goal

The binding benchmark's programs bracket each counted case with stock Valgrind's `callgrind.h` client requests instead of CodSpeed's instrument-hooks, so the flake drops its `instrument-hooks` input and `CODSPEED_INSTRUMENT_HOOKS`, and every count and name stays the same.

## Plan

Recommendations for the implementer:

- Rewrite `ffi/moq/bench/codspeed.c` over `<valgrind/callgrind.h>` and `RUNNING_ON_VALGRIND`, issuing the same `ZERO_STATS`, `START_INSTRUMENTATION`, `STOP_INSTRUMENTATION`, and `DUMP_STATS_AT <name>` sequence rust2-bench-harness does, and rename the files, the `codspeed_*` functions, the `codspeed-hooks` recipe, `target/codspeed`, and `RUST2_CODSPEED` after what they are, such as `hooks`.
- Take the header from the bench shell's pinned Valgrind, so a Valgrind bump moves both together.
- Prove it with `just ffi count run` before and after on one host: every case's name the same and its count within the noise `bench/ffi/README.md` records for that language.
- The cutover is a boundary for `just ffi count compare`: the base's build needs `CODSPEED_INSTRUMENT_HOOKS`, which the new bench shell lacks, and its programs read `RUST2_CODSPEED`, which the new count recipe no longer exports, so a compare against a base before it fails or counts the base wrong. Say so in `bench/ffi/README.md`, where a comparison across it counts each revision with `just ffi count run` in its own revision's bench shell and compares the named results.
- Update `bench/ffi/README.md`'s section on the hooks.

Public API: none. Wire: none.
