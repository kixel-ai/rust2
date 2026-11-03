# [M] clone_from through containers of values that drop

## Goal

[rust2-ts](/ts/README.md) translates `clone_from` and `clone_into` where std's clones
element by element over values that drop, each clone and drop in std's order, proven by
differential tests against native Rust.

## Plan

Found writing the array clones of the `lang/clones` fixture: an array's `clone_from` of
elements that drop translates, each element cloned over its own, but where std's
`clone_from` forwards to an inner one these refuse as "whose `clone_from` is its own":

- `Option<T>::clone_from` of two `Some`s, which is `T`'s, as for an `Option<[T; N]>` or
  an `Option<Vec<T>>`.
- `Vec<T>::clone_from`, which clones over the elements it keeps, truncates, and extends.
- An element that is itself an array, as `[[T; N]; M]::clone_from`, and a slice of arrays'
  `clone_from_slice` and `fill`, which clone each inner element over its own.

`*self = source.clone()` would clone all before dropping any, which is not std's order.
No consumer calls them yet; they are planned here as part of std.

Public API: the translator's output.
