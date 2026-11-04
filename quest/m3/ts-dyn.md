# [S] Trait objects' TypeId

## Goal

[rust2-ts](/ts/README.md) translates `Any::type_id` and `TypeId`, of a trait object and of
`TypeId::of`, with Rust's equality, order, and hash, proven by differential tests
against native Rust, so a consumer neither rewrites nor binds them.

## Plan

Trait objects of every other trait translate, std's and mapped crates' included, each
std type's class calling the mapping of its impl. Decided with the maintainer on
2026-10-27: a stock crate translates with no annotations or config, so trait objects move
from never to not yet.

`TypeId` has no JS value yet: the class of the value's own type stands for its type in a
downcast, but `TypeId::of` of a type no object holds has none, and a `&mut dyn Any` of a
value JS copies is of a second class. Decided with the maintainer on 2026-11-04:

- ✅ An `rs.u128` of rustc's `type_id_hash` of the type, which std's `TypeId` is at run
  time, so equality, order, hash, and `Debug` reuse the `u128`'s; each object's class
  returns its type's as a constant. Its value is native's only where the crate's metadata
  hash is, so tests compare equality and order within a run, not `Debug`.
- Rejected: the type's path as a string, whose order and `Debug` are not native's; the
  class itself, which still needs a value for order and hash, and a class for a type no
  object holds.

Public API: the translator's output.
