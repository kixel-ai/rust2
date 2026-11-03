# [S] An Arc's pointer

## Goal

[rust2-ts](/ts/README.md) translates `Arc::as_ptr` and `Rc::as_ptr`, of a sized value and
of a slice, and `ptr::addr_eq` of the pointers they give, proven by a differential test
against native Rust that `make_mut` keeps the allocation only when no other owner shares it.

## Plan

Found writing the `lang/shares` fixture for `Rc<[T]>`: `Rc::as_ptr` and `ptr::addr_eq` are
refused as unsupported calls. An `rt.Arc` is its allocation, so its pointer may be the
`rt.Arc` itself or a place of its value; the choice must keep what `ptr::eq` and
`addr_eq` already compare. No consumer calls them yet; they are planned here as part of std.

Public API: the translator's output.
