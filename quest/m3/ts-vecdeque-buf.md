# [S] Buf on a deque of bytes

## Goal

[rust2-ts](/ts/README.md) translates `bytes`' `impl Buf for VecDeque<u8>`, whose
`chunk` is the deque's front slice, proven by differential tests against native Rust
across a wrapped ring.

## Plan

Today it is refused at translation. Its chunks follow the ring's split, so the runtime's
deque must expose its two slices as std's `as_slices` does.

Public API: the translator's output.
