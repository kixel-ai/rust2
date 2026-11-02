# [L] Trait objects

## Goal

[rust2-ts](/ts/README.md) translates the trait objects of std's and mapped crates' traits
that moq still refuses (`dyn Error`, `dyn Iterator`, and `dyn bytes::Buf`), and behind any
pointer std coerces to one, with Rust's dispatch and drop order, proven by differential tests against native Rust, so a consumer
neither rewrites nor binds them.

## Plan

Decided with the maintainer on 2026-10-27: a stock crate translates with no annotations or
config, so trait objects move from never to not yet.

- `dyn Debug` and `dyn Display` translate as functions writing their value, behind `&`,
  `Box`, `Arc`, and `Rc`, `dyn Future`
  as an `rt.Future`, `dyn Any` as the class a downcast checks for, `dyn Fn*` as the
  closure itself, and the translated crates' traits as an object with the trait's methods.
  Build the rest on those.
- `Error` needs `source` and its supertraits' `Debug` and `Display` on the object, and its
  `downcast_ref` can check classes as `dyn Any`'s does; moq's are thiserror's `source`.
- `Iterator` can be an `rt.Iter`, and `bytes::Buf` the `rt.ByteSlice` it reads.
- Generic traits and associated types name their arguments in the object's type.
- `Any::type_id` and `TypeId` have no JS value yet, and an upcast to `dyn Any` would need
  the class of the value's own type.

Public API: the translator's output.
