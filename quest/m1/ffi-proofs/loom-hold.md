# [S] Hold model's missed interleavings

## Goal

`ffi/tests/hold.rs`, the loom model of the hold counts Go, Kotlin, and Swift share, explores
a close or move landing between a call's hold and its release on the count, and fails on a
mutant that frees a handle a call holds there.

## Plan

Found in [#906](https://github.com/kixel-ai/rust2/pull/906): loom 0.7.3 never ran another
thread's load between a thread's `load` then `compare_exchange` and its later `fetch_sub`.
A probe, one thread doing `load`, `compare_exchange(n, n + 8)`, `fetch_sub(8)` against
another's `load`, passed `assert_eq!(n, 8)` in every execution, while the same probe without
the leading `load` failed it. So the model's calls on the count, whose fast path is that
`load` and `compare_exchange`, never meet a close mid-call, and a checked close that skipped
its `state == HELD` claim passed every model until #906 held one call up front.

Find why (loom's DPOR with SeqCst, a preemption bound, or the probe), then make every model
reach those interleavings: a newer loom, an explicit scheduling point, or calls held up front
as #906's does. Check with mutants of `hold`, `close`, `abandon`, and `drop_free` that each
model fails.
