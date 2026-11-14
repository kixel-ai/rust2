# [S] Owned buffers cost a write nothing

## Goal

A 100 B write on [the binding benchmark](/bench/ffi/README.md) costs Swift and Dart no more
CPU time than it did before [kixel-ai/rust2#744](https://github.com/kixel-ai/rust2/pull/744)
made Swift's bytes `Bytes` and gave Dart's calls buffers Rust takes, measured against
`7008d1d3` at the same load.

## Plan

#744 measured the write four times against `7008d1d3` on a quiet host (load 2 to 3): Swift
179 against 173 ns and Dart 214 against 203, in every pair, while each copies a 100 B frame
as often as before. The generated Swift differs only in `Native.take(Bytes)` against
`Native.view(Data)`, so the cost is per call, not a copy.

- Profile both writes (`perf record` on `target/bench-export-swift` and the Dart program)
  against `7008d1d3`'s, and remove what the new path adds: in Swift, likely what `Bytes`
  costs to pass and unwrap; in Dart, the `take` flag's checks, `took()`, and the `_given`
  and `_viewed` bookkeeping per call.
- Re-measure the two rows interleaved with `7008d1d3`, and update the benchmark's tables.
