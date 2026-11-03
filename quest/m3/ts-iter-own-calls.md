# [M] A crate's own find and for_each

## Goal

[rust2-ts](/ts/README.md) translates a crate's own iterator that overrides a provided
method std's adaptors and consumers call on it other than those `rt.iterFrom` already
calls (`size_hint`, `next_back`, `len`, `nth`, `nth_back`, `fold`, `rfold`, `count`, and
`last`), such as `find`, `rfind`, `find_map`, or `for_each`. Each runs where native's does,
as many times and in the same order, proven by differential tests against native Rust.

## Plan

Left by the PR that finished folds from the back. Each is refused today as "overriding
`_`, which std's adaptors call in place of `next` or beside it":

- `Filter::next` and `SkipWhile::next` call their iterator's `find`, `FilterMap::next` its
  `find_map`, and `Fuse`, `Chain`, and `Rev` forward `find` and `rfind`. The runtime's
  adaptors read their iterator by `next` there, which is the same for std's own iterators
  but skips a crate's override.
- `for_each` is what std's `Extend` impls for hash maps, `String`, and others call of the
  iterator they are given.
- Each takes a closure, as `fold` does: the crate's `fold` translates for one
  `Box<dyn FnMut>` accumulator-agnostic instance, which these could share.
- Overrides std never calls in place of another, such as `max` or `position`, only matter
  where the crate calls them directly. Check which std calls each before lifting its
  refusal.
- `try_fold`, `try_rfold`, `advance_by`, and `advance_back_by` can be overridden only on
  nightly, through unstable traits and features.
- `lang`'s `ends` fixture's `Overrides` iterator logs each method it is called through, and
  can extend to these.

Public API: the translator's output.
