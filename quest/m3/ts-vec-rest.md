# [S] A vector's try_reserve

## Goal

[rust2-ts](/ts/README.md) translates `try_reserve` and `try_reserve_exact` of a `Vec` and a
`VecDeque`, proven by differential tests against native Rust.

## Plan

Each refuses with its span: std's bodies reach the allocator. They need a
`TryReserveError` of the runtime's with std's `Debug`, `Display`, `Clone`, `Eq`, and
`Error`. `CapacityOverflow` is where `reserve` panics with "capacity overflow" today. Its
`AllocError` is the allocator's, which JS has none of: an array reserves no memory up
front, and only a byte vector's `Uint8Array` can fail to allocate. Whether one is ever
returned, or the call refused where it could be, is open.

`Vec::spare_capacity_mut` stays refused: its length is a capacity the runtime holds only
as std guarantees it, by the model decided with the maintainer on 2026-11-01.

No consumer calls them yet; they are planned here as part of std.

Public API: the translator's output.
