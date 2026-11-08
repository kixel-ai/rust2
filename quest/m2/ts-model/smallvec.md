# [S] smallvec maps into the runtime

## Goal

[rust2-ts](/ts/README.md) maps smallvec's `SmallVec<A>` to the runtime's `rs.Vec`, as it
maps hashbrown's tables to std's, so the model's uses of smallvec translate with no
`unsafe` from its source. The model census at the moq pin loses smallvec's 4 refusals
(its `SmallVecData` union over a type parameter and its raw parts), and a fixture proves
every method the model calls, and smallvec's other common safe API, by differential tests
against native Rust.

## Plan

Decided with the maintainer on 2026-11-07, in the quest audit: ✅ map smallvec into the
runtime under rule 2 of m3's [unsafe policy](/quest/m3/README.md), which wraps a common
crate's `unsafe` at its safe API. Rejected: moq dropping smallvec from the model's path,
which the [model epic](/quest/m2/ts-model/README.md) had planned.

- Add smallvec to the stock crates in `ts/src/lower/stock.rs`, storing a `SmallVec` as
  an `rs.Vec` plus its own state where smallvec's observable behaviour differs from
  `Vec`'s, so every method answers as smallvec does at the pinned version:
  - `spilled`, `capacity`, and `inline_size` follow smallvec's own code for each
    method, not the length: `with_capacity(8)` of a `[u8; 4]` is spilled at capacity 8
    while `new()` is not, a `truncate` keeps a spilled allocation, `grow` and `try_grow`
    to the inline size move it back, `into_inner` of a spilled one is its `Err`, and
    `inline_size` of zero-sized elements is `usize::MAX`. Port each transition from
    smallvec's source at the pin rather than from this list.
  - Drop order follows smallvec's code, not `Vec`'s mapping: its `truncate`, and `clear`
    through it, drop the removed elements from the end, where the runtime's `vecTruncate`
    drops them forwards.
  - The fixture's differential covers reservation, growth past the inline size,
    truncation, `shrink_to_fit`, `grow` and `try_grow` back inline, `into_inner` of an
    inline and a spilled one, zero-sized elements, and a destructor log for `truncate`,
    `clear`, `drain`, and the `SmallVec`'s own drop.
- Methods with no `Vec` twin (`grow`, `try_grow`, `into_inner`, `from_buf_and_len`) map
  where they are safe; `from_raw_parts` and the other `unsafe` methods are refused with a
  span.
- List the mapping in `ts/README.md`'s table beside hashbrown's.

Public API: the translator's output for a crate using smallvec.
