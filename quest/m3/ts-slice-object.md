# [M] rs.Slice

## Goal

[rust2-ts](/ts/README.md) translates slices and references to arrays as `rs.Slice` and
`rs.ByteSlice`, their array, start, and length. What remains:

- A `&mut [[T; N]]` that `as_chunks_mut` and `as_rchunks_mut` lend holds its runs where
  the slice holds their elements, rather than as the runtime's `Runs` and `ByteRuns`
  `Proxy` views and `arrayView`, along with the detaching copies they need before a
  whole write.
- `slice::from_mut` and `array::from_mut` of a `&mut u8` translate, writing the byte
  through the slice where the caller holds it.
- Pointers to a `[T]` of values JS copies compare where they point, as pointers to a
  `[T; N]` and to its elements do.

Each is proven by differential tests against native Rust.

## Plan

Decided with the maintainer on 2026-11-03: Rust types the runtime can't hold as std's do
get a runtime wrapper, converted at API boundaries, rather than a JS native type
([#555](https://github.com/moq-dev/rust2/pull/555) made `Vec` one).

A slice of arrays holds each array as a JS array of its own, so a run of a flat slice has
no array to be. One option is a slice of runs over the flat array, whose element `i` is
the `rs.Slice` of run `i`, which a `&mut [[T; N]]` would be wherever its runs alias
another slice's elements.

A slice of one over a `&mut u8` was found fragile when it meant a one-byte buffer for each
local; an `rs.ByteSlice` whose storage can be an `rs.Place` may avoid that. Measure what
each form costs a hot `&mut u8`, such as `read_exact(slice::from_mut(&mut byte))`.

An `rs.RawSlice` compares by `rs.placeSame` of its first element and its length.

Public API: the translator's output and what a hand-written caller passes for a slice of
arrays.

## Related

- [rs.String](/quest/m3/ts-string-capacity.md) - the other native mapping becoming a wrapper
