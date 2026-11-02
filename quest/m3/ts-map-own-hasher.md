# [M] A std map with a crate's own BuildHasher

## Goal

[rust2-ts](/ts/README.md) translates std's `HashMap` and `HashSet`, and hashbrown's, built
with a crate's own `BuildHasher`, for keys that are JS objects too, proven by differential
tests against native Rust with a hasher that logs what it is given.

## Plan

Found by [#424](https://github.com/moq-dev/rust2/pull/424), which translates hashing
through a crate's own `Hasher` but leaves this refused with "hashing by the hasher of
`_`". A key that is a JS primitive already translates, and never reaches the hasher.

- The runtime finds a JS object key by its `Hash` into the collection's own `Mixer`. A
  crate's own key `Hash` is generic over the hasher, so the map's instance takes the
  crate's hasher type, not the runtime's.
- Open question: whether the runtime must call the crate's hasher at all. std leaves
  how often a map hashes unspecified, and the README already treats it as the runtime's,
  but native calls it at least once per insert or lookup, which a logging hasher sees.
  Decide that before choosing between hashing through an `rt.OwnHasher` and hashing
  around it.
- No consumer yet; planned here because rust2 aims to support all of std.

Public API: the translator's output.

## Related

- [A crate's own hasher that JS copies](/quest/m3/ts-copied-hasher.md) - the other gap #424 left
