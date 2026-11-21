# [S] The runtime's loom models catch what #941's gap hid, and a throwing Kotlin destructor frees once

## Goal

Every loom model in [`ffi/tests/loom/rt.rs`](/ffi/tests/loom/rt.rs), the futures and threads
models, explores the interleavings loom's DPOR skips, and fails on a mutant of each
synchronization point it covers, as `ffi/tests/hold.rs` does since
[#941](https://github.com/kixel-ai/rust2/pull/941). A checked Kotlin object whose destructor
throws after its close commits frees its native handle exactly once, proven by a fixture.

## Plan

Decided with the maintainer on 2026-11-21: ✅ one small m1 quest for both (rejected: two
separate quests; skipping both), since both check code that just landed for correctness.

The loom gap, found in #941: loom 0.7.3 keeps one "last access" per atomic for DPOR, and a
thread's own `load` overwrites it, so a `load` after another thread's `load` never races the
`compare_exchange` that follows, and DPOR never schedules the other thread's access between
them. `hold.rs` works around it with `Atom`, whose every access first does a relaxed
`fetch_add(0)` on one shared gate, and proves it with `Flaw` mutants that `flaws_are_caught`
must catch. `rt.rs` uses loom atomics directly, so it may have the same gap. Recommendation:
reuse `hold.rs`'s gate and `Flaw` pattern rather than a second mechanism, and keep the
preemption bound, paying the time as #941 did.

The Kotlin path, from [#942](https://github.com/kixel-ai/rust2/pull/942): when an object's
destructor throws after Rust commits the close, the Rust glue frees the native handle once.
Only review checked that path, since no fixture has a checked Kotlin object whose destructor
throws. Recommendation: add one to the api fixture and assert the free happens once and the
exception reaches the caller.
