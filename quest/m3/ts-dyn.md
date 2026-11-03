# [M] Trait objects

## Goal

[rust2-ts](/ts/README.md) translates `dyn Error`'s downcasts, the last of std's trait
objects it refuses, with Rust's answers for a value upcast from the crate's own trait,
proven by differential tests against native Rust, so a consumer neither rewrites nor
binds them.

## Plan

Decided with the maintainer on 2026-10-27: a stock crate translates with no annotations or
config, so trait objects move from never to not yet.

- `dyn Debug` and `dyn Display` translate as functions writing their value, behind `&`,
  `&mut`, `Box`, `Arc`, and `Rc`, `dyn Future`
  as an `rt.Future`, `dyn Any` as the class a downcast checks for, `dyn Fn*` as the
  closure itself, `dyn Iterator` and its subtraits as an `rt.Iter`, behind `&`, `&mut`,
  `Box`, `Arc`, and `Rc`, and the translated
  crates' traits as an object with the trait's methods. Build the rest on those.
- `dyn Error` translates as an object of `source`, `Debug`, and `Display`. Its downcasts
  remain: checking the class of `Error.of$T` as `dyn Any`'s do would miss a value upcast
  from the crate's own trait over `Error`, whose object is that trait's class, so a
  downcast needs the class of the value's own type, as an upcast to `dyn Any` does.
- `bytes::Buf` is an object of its methods, as the crate's own traits are, decided with the
  maintainer on 2026-11-02, so it works for any implementor.
- Generic traits and associated types name their arguments in the object's type.
- `Any::type_id` and `TypeId` have no JS value yet, and an upcast to `dyn Any` would need
  the class of the value's own type.

Public API: the translator's output.
