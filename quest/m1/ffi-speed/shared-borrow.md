# [M] Swift's calls reach the C export directly

## Goal

A Swift caller of a generated method calls its C export directly, not through a thunk, so
Swift's u64 getter on one thread costs what C's does (3.4 ns in
[the binding benchmark](/bench/ffi/README.md#concurrent-calls), against 4.0 to 4.6 now)
wherever the linker places the code. The generated API stays the same.

## Plan

- Found on 2026-11-15, chasing the getter's rise from 3.8 to 4.6 ns after the
  reader-biased borrow: the borrow is at most 0.2 ns of it. Each wrapper calls one merged thunk
  (`$s3Moq4PathV3lenSuyKFTm`, from Swift's function merging) that takes the C function as
  a pointer and calls it indirectly, and the getter's cost follows that code's 64-byte
  placement: 4.0 to 4.5 ns as all code shifts 16 bytes at a time, flat as the Rust export
  alone shifts. The earlier count-only borrow costs 4.15 on the same tree, and packing the
  bias into the count's word, or threading the ticket out of the fast path, moved nothing.
- Hand-marking `GroupDemand.sequence()` `@inlinable` in the generated Swift, with
  `@usableFromInline` on `handle`, `Native`, and `Native.status`, made the bench loop call
  `moq_group_Demand_sequence` directly: 3.5 ns at every 16-byte shift of 0 to 48.
- Recommended: generate every wrapper `@inlinable`, and `@usableFromInline` on each
  internal declaration an inlined body names: handles, `Native` and its helpers, the
  runtime's hold counts in `ffi/runtime/rt.swift`. A consumer already imports the C module
  the wrappers call, so an inlined body compiles in its module. Measure the handle call's
  row too (10 ns against C's 6.4), whose class wrapper goes through the same thunk, and
  report the Swift module's size and compile time beside the rows.
- Rejected: aligning Swift's functions, which takes a compiler flag from each consumer, and
  a build setting that turns off function merging, for the same reason.
- Prove it with the single-thread rows, CPU first, at the load beside them, at each
  placement in the 64-byte period: a `.text` pad of 0, 16, 32, and 48 bytes linked ahead
  of the Swift objects.
