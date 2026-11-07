# [M] Parts of arrays without arrayView

## Goal

[rust2-ts](/ts/README.md) holds a slice of arrays as an `rs.Runs` of the slice each array
is, with no `Proxy`. What remains is the runtime's `arrayView`, a `Proxy` JS array over
part of another, which `rs.sliceView` makes where a `&mut [T; N]` to part of an array is
taken whole as a JS array: by `mem::replace` and `mem::swap` of an element of a slice of
arrays, `Cell::from_mut` of one, std's mappings given one to write through, and a raw
pointer's `as_mut` or `Box::from_raw`. Each takes the slice where it is instead, or a
copy written back as `rs.runsBy` writes runs back, and `arrayView` and its `View` traps
go. Proven by differential tests against native Rust.

## Plan

✅ Decided with the maintainer on 2026-11-06 in
[moq-dev/rust2#608](https://github.com/moq-dev/rust2/pull/608), which made `rs.Runs`:

1. How a `&mut [[T; N]]` holds its runs:
   - ✅ (a) A runtime type for every slice of arrays, nested or flat, whose element `i` is
     the slice of run `i`, which replaces the `Runs` and `ByteRuns` `Proxy` views and
     `arrayView`.
   - (b) Bytes only: `as_chunks_mut` makes eager `subarray` views, keeping the `Proxy`
     path for other elements.
   - (c) Keep the `Proxy` views and drop the item.
2. `slice::from_mut` of a `&mut u8`: kept, deferred to
   [a quest of its own](/quest/m3/ts-byte-slice-place.md).
3. Comparing `*const [T]` of values JS copies: moved to
   [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md).

Decided with the maintainer on 2026-11-03: Rust types the runtime can't hold as std's do
get a runtime wrapper, converted at API boundaries, rather than a JS native type
([#555](https://github.com/moq-dev/rust2/pull/555) made `Vec` one).

`rs.replaceIn` and `rs.cellIn` of a JS array write a whole value into it, and copy its
elements to a depth where a write lands in them; a slice's form needs the same, as
`rs.runsReplace` and `rs.cellOfRuns` do for runs. A raw pointer to an array of arrays
that std's mapping gives back as a reference is refused today.

Public API: the translator's output.

## Related

- [rs.String](/quest/m3/ts-string-capacity.md) - the other native mapping becoming a wrapper
