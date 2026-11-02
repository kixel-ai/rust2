# [XS] Debug of a RefCell

## Goal

[rust2-ts](/ts/README.md) translates `{:?}` of a `RefCell` and its `Ref` and `RefMut`,
writing `RefCell { value: .. }` and `<borrowed>` while mutably borrowed as std's does,
proven by a differential test against native Rust.

## Plan

Found writing [`Box<dyn Debug>`](https://github.com/moq-dev/rust2/pull/445): it is an
"unsupported call to `<RefCell<Vec<u8>> as Debug>::fmt`".

Public API: the translator's output.
