# [S] Swift's one-thread getter back to its cost before the biased borrow

## Goal

Swift's u64 getter on one thread, on a noncopyable `GroupDemand`, costs what it did before
the reader-biased shared borrow: 3.8 ns in
[the binding benchmark](/bench/ffi/README.md#concurrent-calls), against 4.6 since.
C's and C++'s getters call the same Rust export and did not rise, and the shared and own
rows keep what the biased borrow won.

## Plan

- The Swift side of the call is byte-identical before and after. What rose is the Rust
  export's inlined fast path: it now loads the bias, keeps a ticket in a callee-saved
  register, and tests the ticket's listed bit at release, saving three registers where it
  saved two. Tried: outlining all but the count (5.6 to 4.4 ns), dropping the bias load
  (no change), and `#[cold]` on the outlined path (worse, 4.6).
- Recommended next: pack the bias into the borrow's `state` word, as the bindings' hold
  counts now do, so the fast path is one load and compare-and-swap again and the release
  one `fetch_sub`; the loom model in `ffi/tests/borrow.rs` follows it. Otherwise, find why
  Swift's call through its shared thunk pays for the longer path when C's loop does not.
- Prove it with the getter row and the shared and own rows, CPU first, at the load beside
  them.
