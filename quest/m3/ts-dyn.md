# [M] Trait objects

## Goal

[rust2-ts](/ts/README.md) translates the trait objects it still refuses, those of std's
other traits such as `dyn Hasher` and `dyn fmt::Write`, and `Any::type_id`, with Rust's
dispatch and drop order, proven by differential tests against native Rust, so a consumer
neither rewrites nor binds them.

## Plan

Decided with the maintainer on 2026-10-27: a stock crate translates with no annotations or
config, so trait objects move from never to not yet.

- `dyn Debug` and `dyn Display` translate as functions writing their value, behind `&`,
  `&mut`, `Box`, `Arc`, and `Rc`, `dyn Future`
  as an `rs.Future`, `dyn Any` as the class a downcast checks for, `dyn Fn*` as the
  closure itself, `dyn Iterator` and its subtraits as an `rs.Iter`, behind `&`, `&mut`,
  `Box`, `Arc`, and `Rc`, and the translated
  crates' traits as an object with the trait's methods, generic over the trait's type
  parameters and the associated types its objects bind. Build the rest on those.
- `dyn Error` translates as an object of `source`, `Debug`, and `Display`, whose downcasts
  check the class of the value's own type, which every trait object of the type shares.
  Decided with the maintainer on 2026-11-02: objects of two traits of one name with a
  method of one name, for one type, stay refused, as that one class can't hold both.
  So do objects of one generic trait at two arguments for one type.
- `bytes::Buf` is an object of its methods, as the crate's own traits are, decided with the
  maintainer on 2026-11-02, so it works for any implementor.
- std's other traits each need their impls' methods on the class of a std type, which a
  mapping, not MIR, gives: `dyn Hasher` of `DefaultHasher`, `dyn fmt::Write` of `String`
  and `Formatter`. A trait with a const parameter is refused too.
- `Any::type_id` and `TypeId` have no JS value yet. The class of the value's own type
  stands for its type in a downcast, but `TypeId::of` of a type no object holds has none.

Public API: the translator's output.
