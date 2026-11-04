# [S] Iterators into a crate's own collections

## Goal

[rust2-ts](/ts/README.md) translates std's iterator adaptors where a crate's own type is
the collection: `collect` into a type with a `FromIterator` impl, and `flat_map` and
`flatten` over a type with an `IntoIterator` impl, proven by differential tests against
native Rust, including drops and laziness.

## Plan

moq-pattern's `Patterns` reaches all three seven times at the pin, from moq-net's
`auth.rs` and `model/origin.rs` and from `Patterns::rebase` itself. The crate's impls
translate already; what is missing is std's adaptor calling them.
