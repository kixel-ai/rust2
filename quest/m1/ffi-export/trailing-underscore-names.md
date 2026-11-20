# [M] Names that differ by trailing underscores export in every binding

## Goal

A crate whose names in one scope differ only by trailing underscores, such as fields `new`
and `new_`, methods `delete` and `delete_`, or variants `Delete` and `Delete_`, exports in
every language: each binding names them apart, as C and C++ do, rather than refusing the
whole export.

## Plan

Found by [cpp-name-collisions](https://github.com/kixel-ai/rust2/pull/914), whose fixture
could not carry its pairs: each of these refused the api fixture's export with "two
members of ... are both ... which rust2-ffi should keep apart", with no span.

- Kotlin: record fields `new`/`new_` and a callback's methods `delete`/`delete_`
  (camel case drops the `_`).
- Swift: plain enum variants `Delete`/`Delete_`.
- Go: a data enum payload's fields `new`/`new_`.
- Python: a field `default` beside `Default`'s `default`.
- Dart: a field `debug` beside `Debug`'s `debug`.

Recommended: give each its own language's rule the way C++'s `named` does in
`ffi/src/cpp.rs` (a name its language spells as Rust does keeps it, and the rest take a
suffix until free, so order never renames one), then add the pairs to the api fixture's
`clash` module with a call per language. The refusal that stays as a backstop names the
second item's span.
