# [L] Copy types cross by value

## Goal

A [rust2-ffi](/ffi/README.md) export of a type that is `Copy`, has no `Drop` glue, and has
private fields, such as moq's `Timestamp`, crosses as an opaque value copied across the C
ABI instead of an `Arc` handle, in every generator. Making and passing one allocates and
frees nothing on either side. [The binding benchmark](/bench/ffi/README.md)'s stamped
write then costs within a few ns of its plain write in each language, against 92 to 256 ns
more today in Kotlin, Python, Go, and Dart.

## Plan

- The C ABI emits a fixed-size, aligned struct of opaque bytes with the Rust type's size
  and alignment, read and written only by Rust glue; the language sees no fields, so
  privacy holds. Recommended over a `#[repr(C)]` mirror of the fields, which would expose
  them and break on a private layout change.
- Each language holds it as its own value type: a C struct, a C++ trivially copyable
  class, a Swift struct, a Go struct, a Kotlin value class over a `Long` or a small array,
  a Python object holding bytes, and a Dart `Struct`. Methods on it call Rust with the bytes.
- A type whose size changes is an ABI change of the export, as a field change already is.
- Prove it with the api fixture's differential tests and re-measure the bench.
