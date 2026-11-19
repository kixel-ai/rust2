# [M] Python and Dart calls that run no user code

## Goal

- Python keeps the GIL across a call only when the exporter proves the whole call reaches
  no user code, no blocking, and no foreign call: not its body, not a nested
  implementation (a derived `eq` or `hash` calls each field's, which may be hand-written,
  so `#[automatically_derived]` alone proves nothing), and not the runtime's own side
  effects, such as a borrow check that drains queued destructors through `Thread::own`.
  Candidates are `contract`, a getter of a primitive field whose borrow path drains
  nothing, and a derived impl whose fields are recursively primitives or derived the same
  way. The argument that this cannot deadlock is written beside the rule.
- Dart makes those same proven calls leaf calls (`isLeaf`), drops the per-call `_Arena`
  where a call needs none, and passes borrowed `&[u8]` and numeric slices through
  `TypedData.address` with no copy, as a direct argument of a leaf call.
- A regression test per binding makes a call that looks like a candidate but reaches user
  code (a field with a hand-written `PartialEq`, a getter after a value from another
  thread queued its drop) and shows it still releases the GIL, or is no leaf call.
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
