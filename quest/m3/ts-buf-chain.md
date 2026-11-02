# [S] bytes' Chain as a Buf object

## Goal

[rust2-ts](/ts/README.md) translates `copy_to_bytes` of `bytes`' `Chain` across both
halves, and with it a `Chain` coerced to `dyn Buf`, whose object holds every method,
proven by a differential test against native Rust.

## Plan

Found writing `dyn Buf`: `Chain::copy_to_bytes` puts `&mut A` and a `Take<&mut B>` into a
fresh `BytesMut::with_capacity(len)`, refused as "`BufMut::put` of `&mut Bytes`" and as a
`put` that picks its methods by a capacity nothing counts. A buffer just made with a
capacity and never split has the capacity it asked for, so `put` into it can read the
source chunk by chunk, as the crate's own `copy_to_bytes` does.

Public API: the translator's output.
