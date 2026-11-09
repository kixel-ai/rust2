# [M] rust2-go exports a type's trait-impl methods

## Goal

Go callers can call a `pub` type's trait-impl methods that
[rust2-go](/go/README.md) translates, such as `Display::fmt`, `Debug::fmt`, and
`From::from`, by a naming rule that keeps every one apart from inherent methods, other
impls of the same trait, and other roots.

## Plan

Decided with the maintainer on 2026-11-07: m4 (from
[moq-dev/rust2#614](https://github.com/moq-dev/rust2/pull/614)'s follow-ups). #614 exports
only `pub` inherent methods (`Counter::new` as `CounterNew`); trait impls are translated
only as callers reach them, under internal names.

- Which impls: those whose self type is an exported type and whose trait is `pub`, std's
  included. A blanket impl (`impl<T: Display> ToString for T`) exports nothing of its
  own, and a generic method stays refused, as a generic inherent one is.
- Recommended for std's formatting and comparison traits: Go's conventional methods, as
  rust2-ffi's Go generator names them ([ffi/README.md](/ffi/README.md#go)): `Display` is
  `String()`, so the type is a `fmt.Stringer`, `Debug` is `GoString()`, and `PartialEq`
  `Equal(other)`. Go callers cannot build a runtime `Formatter`, so `fmt` is exported
  through these, not as itself.
- Open: every other trait's method, as a package function named from the type, the
  trait, and the method, with a generic trait's arguments in the name, since a type can
  implement `From` many times (`From<u8>` and `From<u16>`). Claim each as roots are
  claimed, so a clash fails at both spans.
- [moq-dev/rust2#599](https://github.com/moq-dev/rust2/pull/599) gave rust2-ffi's Go
  exports a Go package per Rust crate, with a function named as a type `New` and that
  name or a trailing `_`, and [moq-dev/rust2#658](https://github.com/moq-dev/rust2/pull/658)
  keeps two types whose names would match apart with a `_` where their paths split. Pick
  a rule that holds if rust2-go follows, so a type's trait methods need not be renamed
  again.

Fixtures: `go/fixtures/items` gives an exported type `Display`, `Debug`, `PartialEq`, and
two `From` impls, called from Go and checked by differential.

Public API: what a translation exports.

## Waiting

- Go consumers: one needs moq in pure Go, which [the epic](/quest/m4/go/README.md) is
  parked until; check with the maintainer before starting.
