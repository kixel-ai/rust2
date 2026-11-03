# [S] Equality of hash maps and sets

## Goal

[rust2-ts](/ts/README.md) translates `==` and `!=` of two `HashMap`s or two `HashSet`s,
std's and hashbrown's, whatever their `BuildHasher`, as std's: equal lengths, then each
entry of one found in the other with an equal value, proven by differential tests against
native Rust.

## Plan

Found while landing maps that hold a crate's own `BuildHasher`: `PartialEq::eq` of two
maps is refused as an unsupported call, for a `RandomState` map too.

- std's walks one map and looks each key up in the other, calling the value's
  `PartialEq`; which map it walks decides which value's `eq` runs as `self`.
- No consumer yet; planned here because rust2 aims to support all of std.

Public API: the translator's output.
