# [M] Opaque values cross with zeroed padding

## Goal

A [rust2-ffi](/ffi/README.md) opaque value, a `Copy` type with private fields held as its
bytes, crosses with every padding byte zero, so no stale Rust stack byte reaches the memory
of the language holding it. Today a padded value, such as std's `Instant` on 64-bit Linux
(an `i64` and a `u32`, then 4 bytes of padding), copies its uninitialized padding into the C
struct: C and C++ see it as any struct's padding, and Kotlin's `LongArray`, Go's `Bytes_`,
Dart's `int`s, and Python's `Value` hold it as ordinary bytes. No API reads them, and no
Rust undefined behavior remains, since the codec holds them as `MaybeUninit`, but they are
readable in the language's own process memory.

## Plan

- The collector records, per target, the byte ranges a value's fields initialize, from
  rustc's layout of its fields, through structs, tuples, and arrays. The glue cannot name
  private fields, so it reads the ranges for the library's own target from a table.
- The glue lowers a value into a zeroed C struct, copying only those ranges with an
  untyped copy, which keeps a reference's provenance. Every lowering of a value goes
  through it: returns, outs, fields, options, lists, and closure arguments.
- Prove it with a padded fixture value, checked in each language to hold zeros where
  Rust has padding, and re-measure the bench's stamped write.

## Waiting

- Maintainer: a value whose initialized bytes depend on its data, such as one holding an
  enum with fields, whose payload's padding differs by variant. Recommended: such a type
  stays an object, an `Arc` handle, as before values crossed by value. Rejected: refusing
  it, which would refuse a type that exports today; masking by variant, which needs the
  discriminant read through Rust on each lowering.

## Related

- [Copy types that change cross by value](/quest/m1/ffi-export/copy-values.md) - lowers values the same way
