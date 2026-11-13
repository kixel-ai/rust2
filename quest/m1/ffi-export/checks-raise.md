# [M] Failed checks raise

## Goal

A run-time check a [rust2-ffi](/ffi/README.md) binding fails raises the language's own
error and leaves the process running, everywhere the binding has a caller to raise to, and
each handle's borrows are counted once per call, in Rust: the bindings keep no hold count
of their own beside the runtime's, and closing or moving out of an object costs no more
than the count it replaced.

## Plan

Decided with the maintainer on 2026-11-09 and 2026-11-12, in the quest audits: checks
raise and panics abort; the one accounting is the runtime's, in `ffi/runtime/rt.rs`, whose
failed check returns a status the binding raises from; the exporter decides at export
time which types need a check; and close in Kotlin, Go, Python, and Dart is Rust-like,
taking the value through the runtime's `TAKEN`, so a call after it raises `moved` and a
close racing a call raises in whichever loses.

[#730](https://github.com/kixel-ai/rust2/pull/730) does all of that. Every place with a
caller raises, Swift's `description`, `==`, and `clone()` included; what a closure returns
still aborts on a failed check, since Rust, its caller, has no error to take.

What is left is the handle itself. With no count of its own, a binding cannot tell when a
call on another thread has read the handle but not yet borrowed it in Rust, so the handle
a close or a move empties is freed only once the GC finds its object unreachable. Go
leaves its `runtime.AddCleanup` registered and Kotlin its phantom reference, and the
[bench](/bench/ffi/README.md#per-handle-baseline) measures it: Go's object return went from
183 to 691 ns (2,080 CPU) and its stamped write from 429 to 1,061; Kotlin's stamped write
from 310 to 613 (2,165 CPU). The getter gained 8 ns in Kotlin and 6 in Go. Python frees by
reference count and Dart's finalizer costs no more, so neither lost.

Public API: none beyond #730's.

## Waiting

- Maintainer: how Go and Kotlin free a handle a close or a move emptied. Recommended: each
  keeps a count for a checked type only to free the handle once no call holds it, with
  close staying Rust-like; it gives back the getter's 6 to 8 ns to win back 300 to 500 ns
  per object returned or stamped, which moq pays per frame. The other way is to keep #730
  as it is and look for a cheaper deferred free under
  [object-ownership](/quest/m1/ffi-speed/object-ownership.md).

## Related

- [Copy types cross by value](/quest/m1/ffi-export/copy-values.md) - landing it first leaves only thrown errors as Swift classes, and the stamped write without a `Timestamp` object
