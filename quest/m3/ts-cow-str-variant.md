# [L] A string Cow's variant

## Goal

[rust2-ts](/ts/README.md) translates matching `Cow::Borrowed` and `Cow::Owned` of a
`Cow<str>`, and `into_owned` and `to_mut` keep std's variant: `to_mut` of a borrowed one
makes it owned. Differential tests against native Rust observe the variant wherever std
decides it, including `String::from_utf8_lossy`, which borrows valid input and owns
replaced text. `Cow::is_borrowed` and `is_owned` are unstable and stay refused.

## Plan

Decided with the maintainer on 2026-11-03: plan this, found while translating
`from_utf8_lossy` in [#537](https://github.com/moq-dev/rust2/pull/537).

Today a `Cow<str>` is a plain JS string ([#105](https://github.com/moq-dev/rust2/pull/105)),
so it keeps no variant. Code that reads one is refused with `library::cow_variant`, at a
match or `if let` of its discriminant (`mir/place.rs` and `mir/rvalue.rs`), at a read of a
variant's field (`mir/place.rs`), and at `is_borrowed` and `is_owned` (`library.rs`). The
`reject` fixture's `strs.rs` holds these cases. `to_mut` is the same place and
`into_owned` the same text, which is right only while nothing reads the variant.

Representation options:

- A wrapper only where some match observes the variant: a whole-program pass finds every
  `Cow<str>` whose variant some match or field read can see, and only those values become
  a `{ borrowed, text }`-like object. Every other `Cow<str>` stays a string. It costs
  nothing where nothing observes, but values flow through generics, fields, collections,
  and calls, so the pass has to follow each one and refuse where it can't.
- A wrapper everywhere: every `Cow<str>` is an object. It is simple and uniform, but costs
  an allocation and an indirection on every use, and changes `Display`, `==`, hashing,
  keys, `Deref`, and the conversions the plain string gets for free.
- Others found along the way, such as a variant kept beside the string only in the
  locals and fields that need it.

Measure each candidate against the plain string on the TS bench (`just ts bench`) and
on moq's translated codec (`just ts moq`) before choosing. Ask the maintainer if no
option is free where nothing observes the variant.

`Cow<[T]>` already keeps its variant (`rt.Borrowed`, or a `ByteSlice` against a
`ByteVec`), which may suggest a shape. The tests belong in the `lang/cows` and `lang/utf8`
fixtures. The `reject` fixture's `Cow<str>` cases move to them.
