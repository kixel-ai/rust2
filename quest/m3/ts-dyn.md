# [L] Trait objects

## Goal

[rust2-ts](/ts/README.md) translates the trait objects of std's and mapped crates' traits
that moq still refuses (`dyn Error` and `dyn Iterator`), and `dyn FusedIterator`, behind
any pointer std coerces to one, such as `Arc<dyn Debug>`, `&mut dyn Debug`, and
`Arc<dyn Iterator>`, with Rust's dispatch and drop order, proven by differential tests against native Rust, so a consumer
neither rewrites nor binds them.

## Plan

Decided with the maintainer on 2026-10-27: a stock crate translates with no annotations or
config, so trait objects move from never to not yet.

- `&dyn Debug` and `&dyn Display` translate as functions writing their value, `dyn Future`
  as an `rt.Future`, `dyn Any` as the class a downcast checks for, `dyn Fn*` as the
  closure itself, and the translated crates' traits as an object with the trait's methods.
  Build the rest on those.
- `Error` needs `source` and its supertraits' `Debug` and `Display` on the object, and its
  `downcast_ref` can check classes as `dyn Any`'s does; moq's are thiserror's `source`.
- `Iterator` can be an `rt.Iter`.
- `bytes::Buf` is an object of its methods, as the crate's own traits are, decided with the
  maintainer on 2026-11-02, so it works for any implementor.
- `Arc<dyn Debug>` is refused where it is built, found by
  [#247](https://github.com/moq-dev/rust2/pull/247) on 2026-10-29: `Arc`, `Rc`, and `Box`
  of a trait object take the same object as a `&dyn`, behind the pointer's own counts.
- Generic traits and associated types name their arguments in the object's type.
- `Any::type_id` and `TypeId` have no JS value yet, and an upcast to `dyn Any` would need
  the class of the value's own type.

Public API: the translator's output.
