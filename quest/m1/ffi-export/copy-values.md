# [L] Copy types cross by value

## Goal

A [rust2-ffi](/ffi/README.md) export of a type that is `Copy` and has private fields, such
as moq's `Timestamp`, crosses as an opaque value copied across the C ABI instead of an
`Arc` handle, in every generator. Making and passing one allocates and frees nothing on
either side. [The binding benchmark](/bench/ffi/README.md)'s stamped write then costs
within a few ns of its plain write in each language, against 140 to 256 ns of CPU time
more today in Kotlin, Python, Go, and Dart.

## Plan

- The C ABI emits a fixed-size, aligned struct of opaque bytes with the Rust type's size
  and alignment, read and written only by Rust glue; the language sees no fields, so
  privacy holds. Recommended over a `#[repr(C)]` mirror of the fields, which would expose
  them and break on a private layout change.
- Each language holds it as its own value type: a C struct, a C++ trivially copyable
  class, a Swift struct, a Go struct, a Kotlin value class, a Python object holding bytes,
  and a Dart `Struct`. Methods on it call Rust with the bytes.
- A type whose size changes is an ABI change of the export, as a field change already is.
- Prove it with the api fixture's differential tests and re-measure the bench.

Decided with the maintainer on 2026-11-12, in the quest audit, for a size that differs by
target: since [#700](https://github.com/kixel-ai/rust2/pull/700) one binding serves every
target, moq ships 32-bit `armv7-linux-androideabi` beside 64-bit ones, and Kotlin, Python,
and Dart share one source across them.

- ✅ The merge picks the representation from the type's size on every target. At most 8
  bytes on all of them: an inline word, such as a Kotlin value class over a `Long`, with no
  allocation. Otherwise: bytes sized by the library when it loads. Rejected: always sized
  at load, which allocates a small array per value in Kotlin where a `Long` is free;
  refusing a type whose layout differs between targets, which would make moq change the
  type, against "rust2 asks moq for no source changes".
- The IR carries no type's layout today: `ffi/src/collect.rs` reads a scalar's size and
  the target's pointer width, and nothing records a struct's size or alignment. The
  staged API (`--api`) gains each such type's size and alignment per target, and the merge
  reads them.

What this replaces, from [#710](https://github.com/kixel-ai/rust2/pull/710): the C ABI's
`<T>_copy`, the IR's `Object.copy`, and Swift's class for a `Copy` object.

Public API: how every generated API holds a `Copy` type (breaking), and the staged API's
layout fields.

## Related

- [Cheaper returned-object ownership](/quest/m1/ffi-speed/object-ownership.md) - the other half of Go's and Dart's stamped write
