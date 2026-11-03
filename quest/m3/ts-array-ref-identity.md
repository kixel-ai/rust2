# [L] Comparing pointers to arrays of other values than bytes

## Goal

[rust2-ts](/ts/README.md) compares pointers to a `[T; N]` of other values than bytes by
where they point, as `ptr::eq` does natively, including those a slice lends by
`first_chunk`, `last_chunk`, `split_first_chunk`, `as_chunks`, or `as_rchunks`, proven by
differential tests against native Rust.

## Plan

Refused since [moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510): each of
those lends a copy of the run, as a shared subslice of such values is a copy, so two loans
of one run compared unequal by `===` while Rust's compare equal. A run of bytes is an
`rt.ByteSlice` or a `Uint8Array` view of the slice's buffer and compares where it points
already.

Lending one array per run, kept by the array beneath, holds only until a shared subslice
copies its elements; the run's identity would need to survive those copies too. An exact
design then needs all of:

- Every shared subslice of such values, made by the runtime's slice functions, iterators
  (`chunks`, `windows`, `split`), a `VecDeque`'s, `from_raw_parts`, or translated code,
  noting the array beneath and its offset.
- One object per (array, offset, `N`) for every loan, shared or mutable, so a
  `first_chunk_mut` pointer compares equal to a `first_chunk` one. That is a view of the
  array, and [moq-dev/rust2#529](https://github.com/moq-dev/rust2/pull/529)'s runs, which
  detach a run's view before a whole write, would have to keep it instead.
- A run covering a whole `[T; N]` being that array, but not a `Vec`'s, whose length
  changes.
- `as_flattened`'s runs being the arrays it flattened.

Measured on [moq-dev/rust2#558](https://github.com/moq-dev/rust2/pull/558), per shared
subslice of 8 of 16 values: 21.9 ns in Node 24 and 12.6 ns in Bun 1.4 as a copy; noting
its origin adds 5 and 18 ns as a symbol property, 390 and 100 ns in a `WeakMap`; a
runtime switch left off adds 1.6 and 6.4 ns.

A slice object, `{ s, start, length }` as [the slice-object quest](/quest/m3/ts-slice-object.md)
weighs for `&[T]`, carries the array and offset without any of the above.

Public API: the translator's output.
