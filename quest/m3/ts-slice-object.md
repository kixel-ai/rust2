# [M] Parts of arrays without arrayView

## Goal

[rust2-ts](/ts/README.md) holds a slice of arrays as an `rs.ArraySlice` of the slice each array
is, with no `Proxy`. What remains is the runtime's `arrayView`, a `Proxy` JS array over
part of another, which `rs.sliceView` makes where a `&mut [T; N]` to part of an array is
taken whole as a JS array: by `mem::replace` and `mem::swap` of an element of a slice of
arrays, `Cell::from_mut` of one, std's mappings given one to write through, and a raw
pointer's `as_mut` or `Box::from_raw`. Each takes the slice where it is instead, or a
copy written back as `rs.arraySliceBy` writes arrays back, and `arrayView` and its `View` traps
go. Proven by differential tests against native Rust.

## Plan

✅ Decided with the maintainer on 2026-11-06 in
[moq-dev/rust2#608](https://github.com/moq-dev/rust2/pull/608), which made `rs.ArraySlice`:

1. How a `&mut [[T; N]]` holds its arrays:
   - ✅ (a) A runtime type for every slice of arrays, nested or flat, whose element `i` is
     the slice of array `i`, which replaces the `Runs` and `ByteRuns` `Proxy` views and
     `arrayView`.
   - (b) Bytes only: `as_chunks_mut` makes eager `subarray` views, keeping the `Proxy`
     path for other elements.
   - (c) Keep the `Proxy` views and drop the item.
2. `slice::from_mut` of a `&mut u8`: kept, deferred to
   [a quest of its own](/quest/m3/ts-byte-slice-place.md).
3. Comparing `*const [T]` of values JS copies: moved to
   [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md).

✅ Decided with the maintainer on 2026-11-07 at the round-3 design checkpoint of
[moq-dev/rust2#608](https://github.com/moq-dev/rust2/pull/608):

4. Conversions that copied arrays where Rust borrows them: ✅ continue, with local fixes
   (rejected: merge as is; `rs.ArraySlice` only for what `as_chunks` splits, keeping
   `rs.Slice<T[]>` for other slices of arrays; drop). Where each array of an array slice is
   a JS array of its own, `rs.arraySliceItems` slices the list of them rather than copying
   each, so a std callback's `ptr::eq` sees the array it was lent, and `join` reads its
   array separator without copying it, since it clones it anyway. The arrays `as_chunks`
   splits from one flat slice have none of their own, so a callback still gets a copy:
   that `ptr::eq` gap moved to [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md).
5. The type's name: ✅ `rs.ArraySlice`, as `rs.ByteSlice` and `rs.RawSlice` are named,
   with its helpers `rs.arraySlice*` (rejected: `rs.Slices`, `rs.Chunks`, keep `rs.Runs`).

Decided with the maintainer on 2026-11-03: Rust types the runtime can't hold as std's do
get a runtime wrapper, converted at API boundaries, rather than a JS native type
([#555](https://github.com/moq-dev/rust2/pull/555) made `Vec` one).

`rs.replaceIn` and `rs.cellIn` of a JS array write a whole value into it, and copy its
elements to a depth where a write lands in them; a slice's form needs the same, as
`rs.arraySliceReplace` and `rs.cellOfArraySlice` do for arrays. A raw pointer to an array of arrays
that std's mapping gives back as a reference is refused today.

Public API: the translator's output and what a hand-written caller passes for a slice of
arrays.
