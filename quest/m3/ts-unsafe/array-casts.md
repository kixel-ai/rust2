# [S] A slice's pointer cast to an array's

## Goal

[rust2-ts](/ts/README.md) translates `*(s as *const [u8] as *const [u8; N])` in a crate's
own code, and `std::io::IoSlice` (`IoSlice::new`, `Buf::chunks_vectored`), proven by
differential tests against native Rust. Today the cast is refused as ``a cast from a
pointer to `[u8]` to one to `[u8; 2]`, which reads the bytes of one type as another``
(`ts/src/lower/pointer.rs`), and `IoSlice` as an unsupported type
(`ts/src/lower/mod.rs`, `ts/src/lower/dynamic.rs`).

## Plan

The bytes keep their type, so the cast reinterprets nothing. `bytes`' own `get_u16` and
wider already map to the runtime since [#447](https://github.com/moq-dev/rust2/pull/447),
and `<[u8]>::get` and `get_mut` of a range translate. No consumer at the moq pin.

Public API: the translator's output.
