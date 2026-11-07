# [S] BufMut on a byte slice

## Goal

[rust2-ts](/ts/README.md) translates `bytes`' `impl BufMut for &mut [u8]` and
`impl BufMut for &mut [MaybeUninit<u8>]`, which write at the slice's front and advance
past what they wrote, proven by differential tests against native Rust.

## Plan

Suggested on [#611](https://github.com/moq-dev/rust2/pull/611), which mapped `Buf` for a
`VecDeque<u8>`: `mapped_self` (`ts/src/lower/stock.rs`) counts any `&[u8]` or `&mut [u8]`
as mapped, but no `Buffer` covers a mutable slice, so both impls look refused the way the
deque was. That was read from the code, not a test; reproduce it first.

Decided with the maintainer on 2026-11-07: ✅ m3 (rejected: m2, drop), since rust2 aims to
support all of std and its common crates' impls. No consumer at the moq pin.

Public API: the translator's output.
