# [S] Kotlin options passed flat

## Goal

A [rust2-ffi](/ffi/README.md) Kotlin binding passes an `Option` of a record or an opaque
value as its JNI scalars, a `bool` and the value's own, as it passes one bare, instead of
encoded into a `byte[]`. moq's `group::Producer::write_frame` takes an `Option<Timestamp>`,
so [the binding benchmark](/bench/ffi/README.md)'s write allocates 208 B a call in Kotlin
(the thread's allocated bytes over the case, measured 2026-11-14), a writer and its array
among them, and costs 220 ns against C's 163. The write then allocates no more than its
frame's bytes take.

## Plan

- Generalize `flat_in` in `ffi/src/kotlin.rs` to an `Option` of a record or an opaque value,
  as `flat_out` already returns one: a `jboolean`, then the value's scalars, zero for `None`,
  within the 255 JVM slots a method takes, and encoded past them.
- Measure the write's wall time, CPU time, and allocated bytes against main's, interleaved.

