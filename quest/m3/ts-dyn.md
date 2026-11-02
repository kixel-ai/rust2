# [L] Trait objects

## Goal

[rust2-ts](/ts/README.md) translates the trait objects of std's and mapped crates' traits
that moq still refuses (`dyn Iterator` and `dyn bytes::Buf`), and behind any
pointer std coerces to one, such as `Arc<dyn Debug>`, with Rust's dispatch and drop order, proven by differential tests against native Rust, so a consumer
neither rewrites nor binds them.

## Plan

Decided with the maintainer on 2026-10-27: a stock crate translates with no annotations or
config, so trait objects move from never to not yet.

- `&dyn Debug` and `&dyn Display` translate as functions writing their value, `dyn Future`
  as an `rt.Future`, `dyn Any` as the class a downcast checks for, `dyn Fn*` as the
  closure itself, and the translated crates' traits as an object with the trait's methods.
  Build the rest on those.
- `dyn Error` translates as an object of `source`, `Debug`, and `Display`. Its downcasts
  remain: checking the class of `Error.of$T` as `dyn Any`'s do would miss a value upcast
  from the crate's own trait over `Error`, whose object is that trait's class, so a
  downcast needs the class of the value's own type, as an upcast to `dyn Any` does.
- `Iterator` can be an `rt.Iter`, and `bytes::Buf` the `rt.ByteSlice` it reads.
- `Arc<dyn Debug>` is refused where it is built, found by
  [#247](https://github.com/moq-dev/rust2/pull/247) on 2026-10-29: `Arc`, `Rc`, and `Box`
  of a trait object take the same object as a `&dyn`, behind the pointer's own counts.
- Generic traits and associated types name their arguments in the object's type.
- `Any::type_id` and `TypeId` have no JS value yet, and an upcast to `dyn Any` would need
  the class of the value's own type.

Public API: the translator's output.
