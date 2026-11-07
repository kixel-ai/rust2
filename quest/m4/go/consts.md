# [M] rust2-go exports pub constants and statics

## Goal

[rust2-go](/go/README.md) exports every `pub` constant and static, associated ones
included, as Go package-level declarations named by their export path, so a crate that
has any translates with no config. Today each is refused at its span, which blocks the whole crate.

## Plan

Decided with the maintainer on 2026-11-07: m4 (from
[moq-dev/rust2#614](https://github.com/moq-dev/rust2/pull/614)'s follow-ups). #614
refuses them in `Translator::root_const` because exporting them is new codegen.

- A constant evaluates once (`const_eval_poly`) and lowers through `lower/consts.rs`,
  as an operand's value already does.
- A static is one place, as Rust's is: translated code reads and Go callers see the same
  Go variable. Translation refuses any reference into a static today
  (`lower/consts.rs`, "pointing into a static"), so this quest translates statics, not
  only their export.
- Open, recommended: a constant of a Go basic type (an integer, `bool`, or `&str`) is a
  Go `const`, so Go callers cannot reassign what Rust makes immutable; any other is a
  `var`. A `var` holding a `pub static` lets Go callers write it, which Rust forbids
  outside `unsafe`; document that, since Go has no read-only variable.
- Open: the name of an upper-case constant. #614's rule joins a path's words in camel
  case, which keeps `LIMIT` but makes `MAX_LEN` `MAXLEN` and `Counter::MAX` `CounterMAX`;
  rust2-ffi's Go keeps a constant's upper case (`LIMIT`, [ffi/README.md](/ffi/README.md#go)).
  Recommended: camel case each word (`MaxLen`, `CounterMax`), as Go names constants.
- A `static mut`, and a static of a type with interior mutability that translation
  refuses (atomics), stay refused at their span.
- An associated constant of a generic type is refused, as a generic `pub` function is.

Fixtures: `go/fixtures/items` gains a `pub const`, an associated constant, and a
`pub static` read by Go and by translated code, checked by differential; reject loses its
`pub const` case.

Public API: what a translation exports.
