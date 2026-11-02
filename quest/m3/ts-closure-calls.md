# [M] std's calls of a closure that have no mapping

## Goal

[rust2-ts](/ts/README.md) translates std's functions that take a closure and are refused
as "unsupported call" whatever the closure holds, each calling it and dropping what it
owns as std's does, proven by the task fixture's differential tests against native Rust.

## Plan

Found probing std's consumers for [drop order](/quest/m3/ts-drop-order.md) in
[#438](https://github.com/moq-dev/rust2/pull/438). Of a closure that owns nothing, each of
these is refused today:

- `Iterator::try_fold`, `try_for_each`, and `partition`. Folds from the back, `rfold` and
  `try_rfold`, are [#436](https://github.com/moq-dev/rust2/pull/436)'s
  [`ts-iter-back-folds`](https://github.com/moq-dev/rust2/blob/quest/m2/ts-model/iter-traits/quest/m3/ts-iter-back-folds.md).
- `iter::repeat_with`, and `Iterator::scan` of a state JS copies, which is refused as a
  `&mut` its mapping would copy.
- `slice::sort_by_cached_key`.
- `LazyCell::new`, `Cell::update`, and `RefCell::replace_with` of a value JS copies.

Each `rt.Iter` has a `fold` and `rfold` that drop what it holds where std's do: a consumer
that takes the iterator by value builds on them, as `rt.iterFold` does.

Public API: the translator's output, and the runtime's functions each needs.
