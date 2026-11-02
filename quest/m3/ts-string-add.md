# [XS] Adding to a String

## Goal

[rust2-ts](/ts/README.md) translates `String + &str`, which moves the `String` and pushes
onto it, as `+=` already does, proven by a differential test against native Rust.

## Plan

Found on 2026-11-02 writing a scratch test for `&mut dyn Debug`: `a + &b` with `a: String`
is an "unsupported call to `<String as Add<&str>>::add`", while `+=` maps.

Public API: the translator's output.
