# [XS] Debug of a RefCell or a Cell

## Goal

[rust2-ts](/ts/README.md) translates `{:?}` of a `RefCell` and its `Ref` and `RefMut`,
writing `RefCell { value: .. }` and `<borrowed>` while mutably borrowed as std's does, and
of a `Cell` of a `Copy` value, writing `Cell { value: .. }`, proven by a differential test
against native Rust.

## Plan

Found writing [`Box<dyn Debug>`](https://github.com/moq-dev/rust2/pull/445): it is an
"unsupported call to `<RefCell<Vec<u8>> as Debug>::fmt`". `Cell`'s, found writing
[`&mut dyn Debug`](https://github.com/moq-dev/rust2/pull/453), is the same; the
maintainer chose on 2026-11-02 to cover both here.

Public API: the translator's output.
