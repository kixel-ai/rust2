# [S] Comparing pointers to arrays of other values than bytes

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
copies its elements; the run's identity would need to survive those copies too.

Public API: the translator's output.
