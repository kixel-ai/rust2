# [L] Failed checks raise

## Goal

A run-time check a [rust2-ffi](/ffi/README.md) binding fails, for a rule its language
cannot express, raises the language's own error and leaves the process running: a borrow
Rust would refuse (a `&mut self` call while another call holds the handle), a call on a
moved-out or closed handle, an object lent for a call used past it, or an item called on a
target that lacks it. Kotlin, Python, Dart, and Swift throw, and Go returns an `error`. C
and C++ abort with the message, and a Rust panic aborts everywhere. Each handle's borrows
are counted once per call, in Rust, and a type no export can misuse pays for no check.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ checks raise and panics
abort. Rejected: raising panics too, which costs unwind tables and landing pads in every
export and a poisoned state for an object that panicked mid-call; aborting on every failed
check, as today. The check runs before any Rust code does, so nothing is corrupted when it
fails. AGENTS.md carries the rule.

Decided with the maintainer on 2026-11-12, in the quest audit:

- ✅ The one accounting is the runtime's, in `ffi/runtime/rt.rs`: a failed check returns a
  status and the binding raises from it. Rejected: the accounting in each binding, as this
  quest first planned, which ports the runtime's state machine (shared, exclusive, moved,
  other thread, lent) to five runtimes and m3's three, and needs unchecked C entry points,
  since Swift and Go call the C header directly; building both in Go first to measure.
- ✅ The exporter decides at export time which types need a check. A type no export
  borrows exclusively or moves skips the accounting on its owned handles; lent handles
  keep it. `ffi/src/ir.rs` already computes most of this for an object's `shared` flag.

What the code does today:

- `rt.rs` aborts through `fail` at 34 sites, reachable from safe multithreaded Kotlin, Go,
  and Python. Every handle argument pays a compare-and-swap in and a `fetch_sub` out, which
  is the whole of C's 10 ns u64 getter against Rust's 0.4
  ([the binding benchmark](/bench/ffi/README.md)).
- Kotlin and Go keep a hold count of their own beside the runtime's, so a call costs four
  atomic operations per handle. Swift keeps one only for its classes since
  [#697](https://github.com/kixel-ai/rust2/pull/697), and traps on a closed handle
  (`rt.swift`'s `hold`). The bindings' counts go; the runtime's status replaces them.
- [#700](https://github.com/kixel-ai/rust2/pull/700) added a fourth check, an item called
  on a target the library is not built for, which traps in Swift (`ffi/src/swift.rs`) and
  panics in Go (`ffi/src/go.rs`). AGENTS.md lists three; add it there with this change.
- Swift's `async throws` and throwing functions already exist for `Result`; a call that
  can fail only a check becomes throwing too, which changes its signature. Go already
  returns `rt.ErrClosed` from every call that takes a handle.
- A foreign exception inside a callback or lent closure still aborts, since Rust has
  nowhere to unwind it to.
- `ffi/README.md` says the C++ header throws nothing, so C++ aborts as C does.

Measure a call before and after on the benchmark's handle call and u64 getter rows. A
checked call may keep the runtime's two atomics; a type that needs no check should reach
Rust's own cost plus the crossing.

Public API: which generated functions throw or return an error (breaking in Swift), the
error types the runtimes add, and the status the C functions return.

## Related

- [Copy types cross by value](/quest/m1/ffi-export/copy-values.md) - landing it first leaves only thrown errors as Swift classes, so fewer signatures turn throwing
- [Cheaper returned-object ownership](/quest/m1/ffi-speed/object-ownership.md) - rewrites the same handle word, so its baselines move with this
