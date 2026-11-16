# [M] A free elsewhere drains on an idle Python loop or C++ executor

## Goal

A [rust2-ffi](/ffi/README.md) value that is not `Send`, freed on another thread, drops
promptly on its owner thread even while that thread's Python event loop or C++ executor
calls nothing into Rust. Today the drop waits in the owner's queue for its next call or
its exit ([kixel-ai/rust2#844](https://github.com/kixel-ai/rust2/pull/844)), so an idle
loop holds what was freed, such as a closed session's sockets, for as long as it stays
idle. The Goal's measurement is that delay: unbounded today, one loop turn after.

## Plan

Decided with the maintainer on 2026-11-16, picking
[kixel-ai/rust2#844](https://github.com/kixel-ai/rust2/pull/844) to merge: ✅ schedule a
drain on Python's wake queue and on a C++ executor when a free elsewhere queues one.
Rejected: waiting for the owner's next call or exit, as #844 does.

- `rt/thread.rs`'s owner record gains an optional wake the binding sets, which the queue
  calls once when it goes from empty to non-empty. Kotlin, Swift, Go, and Dart already
  send such values to a home, which drains on each wake, so they set none.
- Python: the owner's wake queues a drain token on the wake queue its futures use, with
  the same byte to the loop's socket, and the loop's dispatch drains on the loop thread.
  The binding sets the wake when a value that is not `Send` is first made on a thread
  with a running loop.
- C++: an executor may run its job on any thread, so a drain posted to one runs only if
  it lands on the owner. Recommended: post the drain job to the executor of the owner's
  latest `then` or `via`, and drain if the job runs on the owner thread, else leave the
  queue for the next call. Rejected: a generated `api::drain()` for the caller to
  schedule, which is new API for a case the runtime can mostly cover.
- Prove it in `api.py` and `api.cpp`: free a `Local` on another thread while the owner's
  loop is idle, and observe its drop with no further call into Rust.
