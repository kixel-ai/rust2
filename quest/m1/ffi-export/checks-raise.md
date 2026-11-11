# [L] Failed checks raise

## Goal

A run-time check a [rust2-ffi](/ffi/README.md) binding fails, for a rule its language
cannot express, raises the language's own error and leaves the process running: a borrow
Rust would refuse (a `&mut self` call while another call holds the handle), a call on a
moved-out or closed handle, or an object lent for a call used past it. Kotlin, Python, Dart,
and Swift throw, and Go returns an `error`. C and C++ built without exceptions
still abort with the message, and a Rust panic aborts everywhere. Each handle's borrows
are counted once per call, not in the wrapper and again in the runtime.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ checks raise and panics
abort. Rejected: raising panics too, which costs unwind tables and landing pads in every
export and a poisoned state for an object that panicked mid-call; aborting on every failed
check, as today. The check runs before any Rust code does, so nothing is corrupted when it
fails. AGENTS.md carries the rule.

- Today `ffi/runtime/rt.rs` aborts through `fail` at 26 sites, reachable from safe
  multithreaded Kotlin, Go, and Python, while the wrappers already raise for a closed
  handle (`rt.kt`, `rt.swift`, `rt.go`). Kotlin, Swift, and Go keep their own hold count
  beside the runtime's borrow flag, so a call costs four atomic operations per handle
  where C's costs two.
- One accounting, where it can raise: the binding's. Measure a call before and after on
  [the binding benchmark](/bench/ffi/README.md), whose handle call and u64 getter rows are
  the baseline; the target is the C ABI's two atomics
  per handle or fewer.
- Swift's `async throws` and throwing functions already exist for `Result`; a call that
  can fail only a check becomes throwing too, which changes its signature. Go already
  returns `rt.ErrClosed` from every call that takes a handle.
- A foreign exception inside a callback or lent closure still aborts, since Rust has
  nowhere to unwind it to.

Public API: which generated functions throw or return an error (breaking in Swift), and
the error types the runtimes add.

