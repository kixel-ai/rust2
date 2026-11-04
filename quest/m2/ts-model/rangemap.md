# [M] rangemap

## Goal

[rust2-ts](/ts/README.md) translates rangemap as it is, as moq-net's model uses it,
proven by differential tests against native Rust. At the pin it refuses 11 times:

- `BTreeMap::range` by a type other than the key, which the key borrows as, ordered by
  the crate's own `Ord` (`RangeStartWrapper` keys searched by a `RangeEndWrapper`).
- std's `btree_map::Iter` and `Range` held in a field of the crate's own iterators.

## Plan

The runtime's B-tree calls `Ord::cmp` as std's does, `key.cmp(k)` node by node, so a
search by a borrowed form calls that form's `Ord` where std's would. A field of std's iterator type is the runtime's `rs.Iter`, as a local
of that type already is.
