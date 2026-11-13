# [M] Concurrent calls in the binding benchmark

## Goal

[The binding benchmark](/bench/ffi/README.md) measures calls from many threads at once, in
every language. A binding checks borrow exclusivity and a non-`Send` value's thread at run
time, and every case today runs on one thread, so what those checks cost under contention is
unmeasured.

## Plan

Decided with the maintainer on 2026-11-12, in `/quest-plan`: ✅ shared against own handle
(rejected: a shared handle only, which can't tell the checks' contention from moq's own
lock; Rust and C only, which leave out the checks each language's glue makes).

- The handle call and the u64 getter run from N threads, once on one shared handle and
  once with a handle per thread. The gap between the two is contention, and the
  single-threaded case is the floor.
- Every language that can share a handle across threads takes part. Where a language
  can't, such as Dart's isolates, or serializes threads, such as Python under the GIL, its
  column says what it does instead of a number.
- If [CodSpeed](/quest/m1/ffi-speed/codspeed.md) lands first, the new cases join it under
  its A/A rule.

Public API: none.
