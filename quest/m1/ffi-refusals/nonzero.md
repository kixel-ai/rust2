# [S] Export NonZero

## Goal

[rust2-ffi](/ffi/README.md) exports a `std::num::NonZero<T>` of each integer type, in
every position an integer exports, instead of refusing it as a foreign type. At moq's
`main` on 2026-11-12 it is 15 of the 36 errors that fail the export, such as
`Client::with_peer_hop` ([the pin bump](/quest/m1/ffi-refusals/moq-pin.md)).

## Plan

Recommended, by the rule [lossless](/quest/m1/ffi-export/lossless.md) sets for a value
the language's type allows and Rust's does not: it crosses as its integer, and a zero
passed in fails the binding's check, which raises once
[checks raise](/quest/m1/ffi-export/checks-raise.md) and aborts until then. An
`Option<NonZero<T>>` crosses the C ABI as the integer with zero for `None`, which is exact
and is its Rust layout, and stays the language's optional type in each generated API.

- Add each width, the `Option` form, and a zero passed in to the api and reject fixtures.
- Develop against a moq `main` checkout, which `just ffi census <checkout>` takes, and
  report the census before and after.

Public API: the generated type of every `NonZero` parameter, return, and field.
