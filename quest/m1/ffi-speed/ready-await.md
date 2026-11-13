# [M] A cheaper ready await in Swift, Go, and Dart

## Goal

A [rust2-ffi](/ffi/README.md) Swift, Go, or Dart await of a future ready at its first
poll costs close to C's, once [returned-object ownership](/quest/m1/ffi-speed/object-ownership.md)
is out of the way. Since `<fn>_start` made a ready call one crossing and one allocation,
[the binding benchmark](/bench/ffi/README.md)'s ready read costs C 126 ns over its write,
Rust 93, and Swift 289, Go 414, and Dart 522. Less the returned frame's ownership, Swift's
await still adds about 150 ns over C's, Go's 100, and Dart's 180.

## Plan

- Profile each language's ready path from the generated call to the result, after
  object-ownership lands so its share is gone: Swift's `async` entry and `Native.start`,
  Go's escaped out-pointers and error check, and Dart's `_Call` and `_Arena`.
- Fix what dominates in the generator or runtime, with no API change.
- Re-measure the ready read interleaved with the commit before, CPU first.

## Required

- [Returned-object ownership](/quest/m1/ffi-speed/object-ownership.md) - removes Go's and Dart's ownership share of the ready read, so the profile shows only the await's
