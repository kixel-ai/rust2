# [S] A crate's own hasher that JS copies

## Goal

[rust2-ts](/ts/README.md) hashes into a crate's own `Hasher` whose JS value is a copy, so
a `&mut` to it is an `rt.Place`: an enum, such as one choosing between two hashers, or a
binding that Rust copies. std's `Hash` impls, its provided `Hasher` methods, and
`BuildHasher::hash_one` call it as they call a struct hasher, proven by differential tests
against native Rust with an enum hasher that logs each call.

## Plan

Found by [#424](https://github.com/moq-dev/rust2/pull/424), which hashes through a
crate's own hasher by wrapping its `&mut` in an `rt.OwnHasher`, and refuses one JS copies
with "hashing into `_`, a `Hasher` JS copies, which is not supported yet".

- The wrapper holds whatever the `&mut` is, so a place may pass through as it is. Each
  method the impl writes then takes the place, as its translated body expects.
- `hash_one` builds the hasher as a value, so it needs a place of its own to hand over,
  and must still drop the hasher before the value.
- No consumer yet; planned here because rust2 aims to support all of std.

Public API: the translator's output.
