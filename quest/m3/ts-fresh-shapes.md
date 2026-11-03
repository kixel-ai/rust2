# [M] Fresh writes beside a zipped iterator that points at a part

## Goal

[rust2-ts](/ts/README.md) translates a whole write into a slot zipped with an iterator
whose pointers point at a type the slot holds in place, for concrete types and type
parameters alike, proven by differential tests against native Rust:

- `slots.iter_mut().zip(rows)` of a `Vec<[T; N]>` or `vec![row; n]`, writing `[T; N]`
  rows into `([T; N], (u8, u8))` slots, as `[u8; N]` too;
- `slots.iter_mut().zip(fill.iter().copied())` where the slot holds `fill`'s element type.

## Plan

Found by [#516](https://github.com/moq-dev/rust2/pull/516), which made a value of a type
parameter read out of `Zip::next`'s result, `copied`, and `vec![x; n]` judged fresh. The
one cause left: `Trace::fresh`'s check of std-made pointers reads a live `Zip`'s whole
type, so the `IntoIter<[T; N]>` or `Iter<T>` half, which no pointer into the slot made,
counts as pointing at the slot's `[T; N]` part. `origins` also adds each std call in the
chain, the `zip` itself among them, as memory it may have allocated.

Neither the type of a pointer nor what a type holds in place tells the halves apart:
std's code can make a pointer to a tuple's field, as `RangeBounds` for
`(Bound<T>, Bound<T>)` does. Options:

- Std's code generic over a type parameter keeps what it was given under that parameter:
  match a std call's generic signature against which arguments hold pointers, and mask the
  result's parts under parameters none of them bind. In-place `collect` and other
  specializations relate parameters through bounds, so a bound naming both must taint.
- A table of std's iterator adaptors and what each part of their result is made of, as
  `pointer::derives` is for pointer functions.

Recommend the first if it stays sound under specialization; it covers every adaptor at
once.

Public API: the translator's output.
