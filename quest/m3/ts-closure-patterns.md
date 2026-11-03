# [S] A reference to a closure as a string pattern

## Goal

[rust2-ts](/ts/README.md) translates a `str` search given a reference to a closure as its
pattern, `s.contains(&f)` or `s.split(&mut f)`. The search calls the closure as std's does
and never drops it, which leaves the drop to its owner. Differential tests against native
Rust prove it, covering closures that own a value that drops and searches that return a
value and an iterator.

## Plan

Found in [#471](https://github.com/moq-dev/rust2/pull/471), which made a search drop a
closure it owns. A reference to one is refused as a pattern other than a `char`, a
`&str`, a predicate, or chars, as the README's list of what is refused says. std takes
`&F` for an `F: Fn` and `&mut F` for an `F: FnMut`, as each implements `FnMut`.

The closure drop logic that #471 extended already tells a lent closure from an owned one,
and wraps a lent one in `rt.lent` for an iterator that would drop what it holds. A
`&mut` to a function JS copies is that function's place. Watch that a lent closure is
neither dropped by the search nor by the iterator it returns, and drop the refusal from the
README once it translates.

Public API: the translator's output.
