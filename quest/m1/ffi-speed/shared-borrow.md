# [L] Shared borrows that don't contend

## Goal

Threads calling `&self` methods on one shared handle cost about what they cost on a handle
each, in [the binding benchmark](/bench/ffi/README.md)'s concurrent cases, while every
borrow, move, and close stays checked.
Today each shared borrow is a compare-and-swap and a `fetch_sub` on the handle's one
`state`, so on 4 threads C's u64 getter costs 107 ns against 4.1 on a handle each; Kotlin's
and Go's hold counts, and a Swift class's, add a second contended atomic on the same object
(Kotlin 378 ns, Go 252).

## Plan

- Maintainer decision (2026-11-13): ✅ keep this follow-up (rejected: drop).

Recommended: a reader-biased lock in the style of BRAVO (Dice and Kogan, USENIX ATC 2019)
for handles of `Sync` values in `ffi/runtime/rt.rs`. A shared borrow publishes the handle
in a slot of a global table hashed by thread and handle, and checks the handle's bias
flag; an exclusive borrow, a move, or a free clears the bias and waits for the table's
slots naming the handle to empty, then falls back to the count. A slot collision falls
back to the count too. The uncontended cost stays one atomic store and one load, so the
single-thread rows must not rise.
The language hold counts then need the same treatment, or folding into the C ABI's check,
which [checks-raise](/quest/m1/ffi-export/checks-raise.md) may already do.
Rejected for now: per-handle sharded counters, which cost each handle a cache line per
shard.

Prove it with the benchmark's shared and own rows, CPU first, at the load beside them,
and a model check (loom or a TLA+ spec) that no exclusive borrow, move, or free overlaps a
shared borrow.

