# [S] The address of an array a slice lends

## Goal

[rust2-ts](/ts/README.md) compares a `&[T; N]` that a slice of values other than bytes
lends, by `first_chunk`, `last_chunk`, `split_first_chunk`, `as_chunks`, or `as_rchunks`,
by where it points, as `ptr::eq` does natively, or refuses the comparison with its span,
proven by differential tests against native Rust.

## Plan

Found reviewing [moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510): each of
these makes a copy of the run, so two loans of the same run compare unequal by `===`,
while Rust's compare equal. A run of bytes is an `rt.ByteSlice` or a `Uint8Array` view of
the slice's buffer and compares where it points already.

Public API: the translator's output.
