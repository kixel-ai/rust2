# [M] Failed checks raise

## Goal

A run-time check a [rust2-ffi](/ffi/README.md) binding fails raises the language's own
error and leaves the process running, everywhere the binding has a caller to raise to, and
each handle's borrows are counted once per call, in Rust: the bindings keep no hold count
of their own beside the runtime's.

## Plan

Decided with the maintainer on 2026-11-09 and 2026-11-12, in the quest audits: checks
raise and panics abort; the one accounting is the runtime's, in `ffi/runtime/rt.rs`, whose
failed check returns a status the binding raises from; and the exporter decides at export
time which types need a check.

What the code does now:

- A function whose handles can fail a check takes a trailing `<prefix>_status *status`;
  NULL aborts with the message, as C++ passes. Kotlin throws `IllegalStateException`,
  Python `ValueError` or `RuntimeError`, Dart `StateError`, Swift `CheckError`, and Go
  returns an error matching `rt.ErrClosed` or `rt.ErrBorrowed`. An item off the target
  throws in Swift and returns `errors.ErrUnsupported` in Go. A type no export borrows
  exclusively, moves, or lends, and that is `Sync`, is borrowed uncounted (`Object::peek`).
- Still aborting where a caller exists: Swift's `description`, `debugDescription`, `==`,
  and `clone()` of a checked object, which its protocols cannot let throw; Go's `String()`
  and `GoString()`; a record's or data enum's trait functions (`_eq`, `_to_string`,
  `_debug`) whose value holds a checked handle; and Swift's `Arena.hold`, which traps on a
  closed class handle.
- Kotlin, Go, Python, and Dart keep a hold count of their own beside the runtime's, so
  close waits for running calls; a checked call costs four atomic operations per handle.
  Dropping them needs the Rust allocation freed only once the binding's GC releases the
  object, with close dropping the value instead: a moved-out state the runtime already
  has.

Measure the bench's handle call and u64 getter rows before and after, on an idle host.

Public API: the bindings' close semantics, if close stops waiting for a running call.

## Waiting

- Maintainer: whether close in Kotlin, Go, Python, and Dart drops the value at once and leaves the
  handle's memory to the GC, failing a running call's later uses with `moved`, so the
  bindings' hold counts go. Recommended: yes; it is the runtime's existing `TAKEN` state.

## Related

- [Copy types cross by value](/quest/m1/ffi-export/copy-values.md) - landing it first leaves only thrown errors as Swift classes
- [Cheaper returned-object ownership](/quest/m1/ffi-speed/object-ownership.md) - rewrites the same handle word, so its baselines move with this
