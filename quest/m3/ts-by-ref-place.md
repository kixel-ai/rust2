# [S] by_ref of a mutable reference to an iterator

## Goal

[rust2-ts](/ts/README.md) translates `Iterator::by_ref` of a `&mut I` held in a local,
such as `(&mut it).by_ref()` where `it: &mut I`, advancing the iterator it points at as
std's does, proven by a differential test against native Rust.

## Plan

Found writing [`dyn Iterator`](https://github.com/moq-dev/rust2/pull/437): it fails as
"`Iterator::by_ref` with a mutable reference to a value JS copies, which its mapping would
copy", since the `&mut I` itself is an `rt.Place`.

Public API: the translator's output.
