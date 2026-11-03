# [XS] Whether a type needs drop

## Goal

[rust2-ts](/ts/README.md) translates `std::mem::needs_drop::<T>()` as the constant rustc
gives each instance. That includes a `T` whose JS value drops nothing, such as a `String`,
since a caller may branch on it as std's collections do. A differential test against
native Rust proves it for a `T` of each outcome.

## Plan

Found on 2026-11-02 while reproducing a comparison of two constants
([moq-dev/rust2#480](https://github.com/moq-dev/rust2/pull/480)): `needs_drop::<T>()` is an
"unsupported call to `std::mem::needs_drop::<u8>`", while `size_of::<T>()` already
translates as its value. The value is rustc's, not whether translation emits a drop.
Fold in any other per-instance constant of a type that is refused the same way.

Public API: the translator's output.
