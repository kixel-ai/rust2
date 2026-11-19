# [L] Test the ownership protocol against a model

## Goal

Random sequences of borrow, exclusive lend, move, drop, free, poll, cancel, and calls from
another thread run against an executable model of the status each step must return,
through the C ABI under Miri and ASan, and through each binding's wrapper (close, GC, hold
counts) by a generated driver, as the shape tests generate theirs. loom models the
future and waker state (`Future` and its `polling` flag in the runtime) and
`rt/thread.rs`'s drop queue and homes.

## Plan

Found by the 2026-11-18 language audit: each runtime check is broken exactly once by a
hand-written test today, and no loom model covers the futures or `rt/thread.rs` (427
lines, 15 `unsafe`).

Recommended: `proptest-state-machine` for the C ABI sequences, the model itself the
spec; a TLA+ or Quint spec only if the model grows past what code reads clearly. Short
sequences in the gate, long ones nightly.

Public API: none.
