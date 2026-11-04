# [M] rs.Slice

## Goal

[rust2-ts](/ts/README.md) translates `&[T]`, `&mut [T]`, and `Box<[T]>` of values other
than bytes as `rs.Slice`, its array, start, and length, as `rs.ByteSlice` is for bytes.
What remains:

- A `&[T; N]` and `&mut [T; N]` of other values is an `rs.Slice` too, as `&[u8; N]` is an
  `rs.ByteSlice`, so the run `first_chunk_mut`, `split_first_chunk_mut`, an array subslice
  pattern, and `array::from_mut` lend is where the slice holds it, rather than the
  runtime's `arrayView` `Proxy`.
- Pointers to a `[T; N]` and to elements JS copies compare where they point, as `ptr::eq`
  does natively, lifting the refusal of
  [moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510) and of comparing two
  `rs.Place`s of one element.
- `slice::from_mut` and `array::from_mut` of a `&mut u8` translate, writing the byte
  through the slice where the caller holds it.

Each is proven by differential tests against native Rust.

## Plan

Decided with the maintainer on 2026-11-03: Rust types the runtime can't hold as std's do
get a runtime wrapper, converted at API boundaries, rather than a JS native type
([#555](https://github.com/moq-dev/rust2/pull/555) made `Vec` one).

A `[T; N]` value stays a JS array. A reference to one would be an `rs.Slice` of it, as
`Pointee::Bytes` makes a reference to a `[u8; N]` an `rs.ByteSlice`, generalized to any
element. Then `as_chunks_mut`'s `Runs` and `ByteRuns` views and `arrayView` go, along with
the detaching copies they need before a whole write.

Pointer identity then compares an `rs.AtIndex` by its array and index, and an `rs.Slice`
by its data, start, and length, as `rs.ByteSlice` already does.

A slice of one over a `&mut u8` was found fragile when it meant a one-byte buffer for each
local; a slice whose storage can be an `rs.Place` may avoid that. Measure what each form
costs a hot `&mut u8`, such as `read_exact(slice::from_mut(&mut byte))`.

Public API: the translator's output and what a hand-written caller passes for a
reference to an array.

## Related

- [The runtime's namespace is rs](/quest/m3/ts-rs-rename.md) - runs in parallel
- [rs.String](/quest/m3/ts-string-capacity.md) - the other native mapping becoming a wrapper
