# [M] A slice's chunks as mutable arrays

## Goal

[rust2-ts](/ts/README.md) translates `slice::as_chunks_mut` and `as_rchunks_mut`, with
what is written through a chunk, by element or whole, landing in the slice, proven by
differential tests against native Rust at `u8`, at another element, and at one that drops.

## Plan

`Vec::from` of a slice and the shared `as_chunks` and `as_rchunks` translate since
[moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510), each chunk a copy, which a
shared borrow can't tell apart. The mutable forms are refused as unsupported calls.

A `&mut [[T; N]]` aliases a flat slice, which JS can't do with arrays of arrays. A view
per chunk makes a write by element land, but a whole write into the outer slice, by
`swap`, `reverse`, `sort`, `mem::replace`, or an assignment, overwrites what an earlier
read of that chunk still holds, since the translator treats that read as the old value.
For values other than bytes, a view could detach to a copy of what it held before its
chunk is overwritten. A byte chunk is a `Uint8Array` over the slice's buffer, which can't
detach, so bytes need another representation or proof that no read outlives a write.

moq calls `as_chunks_mut::<N>()` over bytes in moq-mux (`container/ts/export.rs`,
`import.rs`), moq-audio (`format.rs`), and moq-cli (`publish.rs`), writing each run by
element as it enumerates, zips, or filters them.

Decided with the maintainer on 2026-11-03:

- `as_chunks_mut` over bytes: each chunk can be a `Uint8Array` view over the slice's
  buffer, so element writes land, but a view can't detach. How to handle a whole write
  through the outer `&mut [[u8; N]]` (swap/sort/reverse/assign) changing an earlier
  by-value read?
  - ✅ Views; copy array reads: chunks are views, and reading a whole `[u8; N]` out of a
    view by value always copies it, so whole writes are safe everywhere. Measure the
    copy's cost on moq-mux.
  - Views; refuse whole writes.
  - Prove no read outlives a write.

Public API: the translator's output.
