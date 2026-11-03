# [S] A crate's own collect, eq, partial_cmp, and by_ref

## Goal

[rust2-ts](/ts/README.md) translates a crate's own iterator that overrides `collect`, `eq`,
`partial_cmp`, or `by_ref`, which std calls of the iterator in place of another method. Each
runs where native's does, as many times and in the same order, proven by differential tests
against native Rust.

## Plan

Left by the PR that translated a crate's own `find`, `rfind`, `find_map`, `for_each`,
`reduce`, and `all`. Each is refused today as "overriding `_`, which std calls in place of
another method":

- std's `FromIterator` of `Box<[T]>`, `Rc<[T]>`, `Arc<[T]>`, `BTreeMap`, `BTreeSet`, and
  `BinaryHeap` calls the iterator's `collect` for a `Vec` first.
- `ne` calls the iterator's `eq`, and `lt`, `le`, `gt`, and `ge` its `partial_cmp`.
- `Vec::splice` calls its replacement's `by_ref` as the splice drops.
- Unlike the overrides taking a closure, these are generic over another iterator or a
  collection, so one instance per iterator, as `rt.iterFrom`'s `rt.Own` record holds, doesn't
  fit. A call of the crate's instance for that type where std's mapping reaches it may.
- Adaptor constructors such as `map`, and nightly-only overrides such as `try_fold`, stay
  refused: an override of either can't return without diverging or a feature.

Public API: the translator's output.
