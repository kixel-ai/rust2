# [S] Cow's clone_from, Reverse's clone, and Option::as_slice

## Goal

[rust2-ts](/ts/README.md) translates `Cow<[T]>::clone_from`, `Clone` of `cmp::Reverse`,
and `Option::as_slice`, each as std's behaves, proven by differential tests against
native Rust.

## Plan

Found while keeping a `Vec`'s capacity through `clone_from`
([#533](https://github.com/moq-dev/rust2/pull/533)): each is refused with its span today.

- `Cow<[T]>::clone_from` of an owned `Cow` over an owned one keeps the target's buffer,
  as `Vec::clone_from` does, so its capacity shows the difference from a fresh clone.
- `Reverse::clone` clones what it wraps, which may be a value that drops or one JS
  shares, such as a `Vec`.
- `Option::as_slice` lends the payload as a slice of zero or one element; whether a
  payload JS copies needs a place to lend it from, as `slice::from_ref` does, is open.

No consumer calls them yet; they are planned as part of std.

Public API: the translator's output.
