# [S] Buf on a deque of bytes

## Goal

[rust2-ts](/ts/README.md) translates `bytes`' `impl Buf for VecDeque<u8>`, whose
`chunk` is the deque's front slice, proven by differential tests against native Rust
across a wrapped ring.

## Plan

Today it is refused at translation. The runtime already has the deque's two slices, as
std's `as_slices`, so the gap is the translator's mapping of `bytes`' impl onto them.

Public API: the translator's output.
