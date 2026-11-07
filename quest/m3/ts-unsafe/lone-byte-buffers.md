# [M] A byte local whose address is taken, in a buffer of its own

## Goal

[rust2-ts](/ts/README.md) keeps a `u8` local whose raw pointer the body takes (`&raw const`
or `&raw mut` of it) in a `Uint8Array(1)`, so raw slices, mutable slices, and casts to
`*const [u8; N]` of it are views of that buffer, as for any other byte, proven by
differential tests against native Rust. Today JS keeps such a byte in no array of bytes,
so `stray` and `lone_byte_array` (`ts/src/lower/mir/origin.rs`) refuse, with `a raw or
mutable slice of a byte in no array of bytes`, each route the body shows, and the runtime
throws the same where it doesn't (`ptrByteSlice`, `RawSlice.byteArray` in
`ts/runtime/rs.ts`), or copies the byte for a shared slice (`Array::Copy`).

## Plan

Decided with the maintainer on 2026-11-07: m3. Planned on review of
[#624](https://github.com/moq-dev/rust2/pull/624), whose body-visible refusal of a cast of
an empty raw slice of a lone byte to `*const [u8; N]` kept missing routes (`&raw *s`,
`NonNull::from`, `NonNull::new(p).unwrap()`); this fixes the class at its root.

- Which locals: those whose `&raw` the body takes, decided per body before lowering, so
  every read and write of the local is `buf[0]` and a place of it is the `rs.AtIndex` of
  element 0.
- Then the lone-byte refusals and the `Array::Copy` case go for those locals. A byte in a
  field of an object JS keeps, or one behind a reference from elsewhere, stays in no array:
  keep refusing or throwing there, and say so in the README's pointer row.

Public API: the translator's output.

## Related

- [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md) - lists the run-time throws this removes for locals
