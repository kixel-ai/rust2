# [M] A ready async call in one crossing

## Goal

A [rust2-ffi](/ffi/README.md) async call whose future is ready at its first poll costs one
crossing and no future handle: one C function makes the future, polls it once, and writes
the result. A call that suspends allocates once. [The binding benchmark](/bench/ffi/README.md)'s
ready read then costs each language close to its write, against 155 ns more in C, 315 in
Swift, and 571 in Go today.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a child of the exporter
epic, since it changes every future's C signature and so lands before moq pins.

What a ready call costs today, in `ffi/runtime/rt.rs` and `ffi/src/glue.rs`:

- Four allocations: `Future::new` boxes the future and then the `Future` around it, each
  result is a boxed `Done` closure, and the first poll wraps even the all-null waker in an
  `Arc`.
- Three crossings from Go and Swift: the call, the poll, and the free
  (`ffi/runtime/ffi_cgo.go`, `ffi/src/go.rs`). Kotlin and Python already fuse the call and
  first poll inside their Rust glue.

Recommended, in this order:

- A null waker lifts to `Waker::noop()`, with no `Arc`. No contract change.
- One typed allocation per future, with a poll per export that lowers the result
  directly, so the `Done` box and the second box go.
- `<fn>_start(args, outs, err, &future)` makes the future and polls it once. Ready: it
  writes the result and hands back no handle. Pending: it hands back the handle, and
  `<fn>_poll` and `<fn>_future_free` stay as they are.

Each generator's native future keeps the rules [the epic](/quest/m1/ffi-export/README.md)
sets: the result arrives in the poll that finishes the future, so a cancel loses no frame,
and a wake between `PENDING` and parking is latched. Re-run each language's forced-window
cancel test, and re-measure the bench's ready and woken reads.

Public API: every async export's C functions and the contract hash (breaking), and each
binding's future wrapper.
