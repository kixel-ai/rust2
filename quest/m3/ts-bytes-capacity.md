# [S] The spare capacity of a BytesMut

## Goal

[rust2-ts](/ts/README.md) translates `BytesMut::spare_capacity_mut`, with writes through
the `&mut [MaybeUninit<u8>]` it lends reaching the storage, proven by differential tests
against native Rust.

## Plan

`rt.BytesMut`'s capacity is `bytes`' since its storage counts its handles, but a
`[MaybeUninit<u8>]` is a JS array apart from the storage, so a write through it, and a
second call's read of that write, wouldn't reach it. Make a slice of `MaybeUninit<u8>` an
`rt.ByteSlice`, as a `[u8]` is, which `Vec<u8>::spare_capacity_mut` would take too. moq
calls neither.

Public API: the translator's output.
