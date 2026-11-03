# [S] A map built with BuildHasherDefault

## Goal

[rust2-ts](/ts/README.md) translates `BuildHasherDefault<H>`, std's `BuildHasher` of a
`Hasher` that is `Default`, as a map's hasher, as `rustc-hash`'s `FxHashMap` and
`nohash-hasher`'s `IntMap` build theirs: making one, `Default`, `Clone`, `build_hasher`,
and `hash_one` through a crate's own `H`, proven by differential tests against native Rust.

## Plan

Found while landing maps that hold a crate's own `BuildHasher`, which refuse
`BuildHasherDefault` as "unsupported type `std::hash::BuildHasherDefault` from another
crate".

- It holds nothing but a `PhantomData`, so it may be what such a type is, as `PhantomData`
  is, and its `build_hasher` is `H::default()`.
- A map already holds any `BuildHasher` but `RandomState` and hashes keys around it.
- No consumer yet; planned here because rust2 aims to support all of std.

Public API: the translator's output.
