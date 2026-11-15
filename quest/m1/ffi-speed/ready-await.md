# [M] A cheaper ready await in Swift, Go, and Dart

## Goal

A [rust2-ffi](/ffi/README.md) Swift, Go, or Dart await of a future ready at its first
poll costs close to C's, apart from the returned object's ownership, which
kixel-ai/rust2#838 cut to a nursery of the last 64 returned objects. Since
`<fn>_start` made a ready call one crossing and one allocation, [the binding benchmark](/bench/ffi/README.md)'s ready read costs C 126 ns over its write,
Rust 93, and Swift 289, Go 414, and Dart 522. Less the returned frame's ownership, Swift's
await still adds about 150 ns over C's, Go's 100, and Dart's 180.

## Plan

- Profile each language's ready path from the generated call to the result, setting the
  returned frame's ownership apart (its split is in
  [the benchmark's README](/bench/ffi/README.md#what-a-call-spends), item 2): Swift's
  `async` entry and `Native.start`, Go's escaped out-pointers and error check, and Dart's
  `_Call` and `_Arena`.
- Fix what dominates in the generator or runtime, with no API change.
- Re-measure the ready read interleaved with the commit before, CPU first.
