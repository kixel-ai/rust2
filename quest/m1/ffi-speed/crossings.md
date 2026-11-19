# [M] Python and Dart calls that run no user code

## Goal

- Python keeps the GIL across calls where no user code can run: scalar `pub` field
  getters, derived `eq` and `hash` (detected by `#[automatically_derived]`), and
  `contract`, with the argument that this cannot deadlock written beside the rule.
- Dart makes those same calls leaf calls (`isLeaf`), drops the per-call `_Arena` where a
  call needs none, and passes borrowed `&[u8]` and numeric slices through
  `TypedData.address` with no copy, as a direct argument of a leaf call.
- The binding benchmark gains a case for Dart's thread hop: while Rust holds any closure
  or trait implementation from an isolate, every call goes to a worker thread
  (`ffi/runtime/dart.rs:287-333`).

Each lands only where the benchmark shows it faster.

## Plan

Found by the 2026-11-18 language audit. `rt::call` releases and retakes the GIL around
every call (`ffi/runtime/python.rs:228-253`); the benchmark puts Python's getter 37 ns over
C's, an estimated 10 to 15 of it the GIL round trip. `isLeaf` appears nowhere in
`ffi/src/dart.rs`, and every call allocates an `_Arena` (`rt.dart:961`).

Watch [dart-lang/sdk#64368](https://github.com/dart-lang/sdk/issues/64368): `.address` into
a leaf call that returns a struct is miscompiled, so avoid that combination or refuse it.
`NativeCallable.isolateGroupBound` would remove the thread hop, but Dart 3.14 put it behind
`--experimental-shared-data`; do not adopt it while it is experimental.

Public API: none.
