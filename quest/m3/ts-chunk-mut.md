# [S] The chunk a BufMut lends

## Goal

[rust2-ts](/ts/README.md) translates `BufMut::chunk_mut` of a `BytesMut` and a `Vec<u8>`,
and the writes of the `bytes::buf::UninitSlice` it lends, so a buffer filled through its
chunk and taken in by `advance_mut` holds what was written, proven by differential tests
against native Rust.

## Plan

`spare_capacity_mut` is an `rs.ByteSlice` of the storage past the length, and
`advance_mut` maps. An `UninitSlice` is `#[repr(transparent)]` over `[MaybeUninit<u8>]`,
so it can be the same `rs.ByteSlice`; `chunk_mut` reserves 64 bytes where there is no
room, as the runtime's `bytesMutRoom` and `byteVecRoom` already model. A generic
`B: BufMut`, the crate's own included, reaches it, as it is a required method. moq calls
neither.
