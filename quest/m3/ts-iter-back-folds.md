# [M] Folds from the back, and a crate's own folds

## Goal

[rust2-ts](/ts/README.md) translates `DoubleEndedIterator::rfold` and `try_rfold` of any
iterator, and a crate's own iterator that overrides a provided method std's adaptors call
in place of `next` or `next_back`, such as `fold` or `nth_back`. Each runs where native's
does, as many times and in the same order, proven by differential tests against native
Rust.

## Plan

Found by [#436](https://github.com/moq-dev/rust2/pull/436), which translates a crate's own
`next_back`, `len`, and `size_hint`:

- `rfold` and `try_rfold` are an "unsupported call" for every iterator, std's included.
  The runtime already folds from the back inside its adaptors (`Iter.rfold`, `folded`),
  so the gap may be mostly the mapping.
- A crate's own `Iterator::fold` or `DoubleEndedIterator::nth_back` is refused as
  "overriding `_`, which std's adaptors call in place of `next` or beside it". Each override
  is a method std's adaptors and consumers call in place of the default: `fold` from
  `for_each`, `count`, `last`, and `Map::fold`, `nth_back` from `Rev::nth`. `rt.iterFrom`
  would call it there, as it now calls `next_back`.
- Open: whether overrides of the other provided methods (`nth`, `try_fold`, `rfold`,
  `advance_by`, `count`, `last`, and more) land here too. Each is the same problem, and
  the runtime's callers of each decide how far it reaches.
- `lang`'s `ends` fixture logs each method a crate's iterator is called through, and can
  extend to these.

Public API: the translator's output.
