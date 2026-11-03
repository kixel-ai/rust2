# [S] A boxed iterator folds as std's box does

## Goal

[rust2-ts](/ts/README.md) reads a `Box<I>` of a sized iterator, owned or behind a `&mut`,
as std's `Iterator` for `Box<I>` does: by `I`'s own `next`, `size_hint`, `nth`, `last`,
`next_back`, `nth_back`, `len`, and `is_empty`, and by `next` or `next_back` for every
other consumer, proven by the lang fixture's differential tests against native Rust.

## Plan

Found on 2026-11-03 probing what [#437](https://github.com/moq-dev/rust2/pull/437) left:
the translation erases the box and folds what it holds by its own `fold`, where std's
defaults read the box by `next`. A boxed `Chain` of mapped `vec::IntoIter`s matches native
across `count`, `fold`, `for_each`, `find`, `position`, `max_by_key`, `rev().fold`, and
`skip(1).count()`, since `Chain`'s folds read and drop in `next`'s order. A box of a `skip`
of a zip std reads by index does not: `Box::new(a.map(f).zip(b.map(g)).skip(1)).count()`
makes and drops the skipped pair whole, as `Skip`'s fold does, where native skips it a
side at a time through the zip's `nth`.

`rt.iterDyn` already reads a `dyn Iterator` this way, so a box of a sized iterator can be
one too, with `last` forwarded to the iterator's own, as std's `BoxIter` specializes it.

Public API: the translator's output.
