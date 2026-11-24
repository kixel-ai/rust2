# [M] Node's binding benchmark cases count the same on every run

## Goal

The [binding benchmark](/bench/ffi/README.md)'s `ts-node` program rejoins `count_langs` in
`ffi/justfile`, so Kixel Quest tracks the web binding's instruction counts beside Rust's, C's,
C++'s, moq-c's, and Python's: two hosted runs of one commit through `just ffi count run`
move no case by more than a few percent, the bar the README sets for every language.

## Plan

It left when rust2-bench replaced CodSpeed (the README's Instruction counts section has the
numbers): its async cases `ready`, `woken`, and `write` moved up to 63% between runs of one
build, alone or beside the other programs, under rust2-bench's Valgrind flags or CodSpeed's,
and had already moved 36% to 83% under CodSpeed on PRs that changed no TypeScript. Rust's
and C's programs run the same moq code and hold within 0.6%, so the variance is in what the
Node program adds over it: V8, the wasm32-wasip1 module's WASI clock, or `export.ts` itself.

Recommendations for the implementer:

- Find what grows before fixing anything: rust2-bench leaves each job's Callgrind profiles,
  costed by function, under `ffi/target/rust2-bench/jobs`. Diff `callgrind_annotate` of two
  runs' `ready` dumps to see which functions account for the difference.
- A count that depends on wall time, such as a timer or a deadline the module reads through
  WASI's clock, slows with Valgrind and moves with the host's load. Prefer a fix in the
  benchmark, such as a clock that only its cases advance, over one in the binding.

## Related

- [The harness's counts](/quest/m2/harness/counts.md) - also counts under Valgrind, with the same need for counts that hold still
