# [S] Bytes from an owner that drops

## Goal

[rust2-ts](/ts/README.md) translates `Bytes::from_owner` of a type that drops, which drops
with the last `Bytes` sharing it as `bytes` drops it, proven by differential tests against
native Rust that log the drop. Today it is refused, since `rs.Shared` keeps no owner.

## Plan

moq-net's `model/frame.rs` makes a frame's bytes this way twice at the pin, from an owner
holding an `Arc`. Counting the clones of a shared storage is what `bytes` does; the
runtime needs the same count, without slowing the `Bytes` that have no owner.
