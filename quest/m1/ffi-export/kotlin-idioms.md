# [S] Kotlin: direct ByteBuffers and plain Java callers

## Goal

- `Bytes` takes a direct `ByteBuffer`, copied once inside the call through
  `GetDirectBufferAddress`, and copies into one with `copyInto(ByteBuffer)`, so a
  MediaCodec frame crosses with one copy instead of two (`ffi/src/kotlin.rs:324-378`). No
  view of Rust's memory escapes.
- Plain Java can call the binding: each function that throws a generated error carries
  `@Throws`, so javac accepts the `catch`, and companion functions are `@JvmStatic`.

`tests/api.kt` covers both, the Java part through a small Java caller in the test.

## Plan

Found by the 2026-11-18 language audit. Errors subclass checked `Exception` with no
`@Throws` (`ffi/src/kotlin.rs:1328`) and companion functions lack `@JvmStatic`
(`ffi/src/kotlin.rs:1638`).

Decided with the maintainer on 2026-11-18: ✅ breaking changes land in this epic before
moq pins, as soon as possible (rejected: after the pin). ✅ No Java-first API: `new` as a
Java keyword, value classes, and `suspend` stay Kotlin-shaped, and
[m3](/quest/m3/README.md#plan)'s rejection of a Java target stands (rejected: reopening
it).

Public API: added `Bytes(ByteBuffer)` and `copyInto`; annotations visible to Java.
