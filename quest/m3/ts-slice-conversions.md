# [S] A Vec from a slice, and a slice's chunks as arrays

## Goal

[rust2-ts](/ts/README.md) translates `Vec::from` of a `&[T]` and a `&mut [T]`, and
`slice::as_chunks`, `as_rchunks`, and `as_chunks_mut`, proven by differential tests
against native Rust at `u8`, at another element, and at one that drops.

## Plan

Found writing the `lang/shares` fixture for `Rc<[T]>`: `Vec::<u8>::from(&[u8])` and
`<[u8]>::as_chunks::<2>` are refused as unsupported calls, while `to_vec` and
`chunks_exact` translate. No consumer calls them yet; they are planned here as part of std.

Public API: the translator's output.
