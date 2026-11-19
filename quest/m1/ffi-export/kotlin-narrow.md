# [S] Kotlin: a usize too wide for a 32-bit target throws

## Goal

On a 32-bit Android ABI (armv7, x86), Kotlin passing a `ULong` past `usize::MAX`, or a
`Long` outside `isize`'s range, throws `IllegalArgumentException`, as every other value
Rust's type cannot hold does, and leaves the call unmade. Today the glue aborts the
process. This covers a parameter, a field of a record crossing in, and a closure's return.
`tests/api.kt`'s `lossless()` checks each one on the 32-bit emulator that the nightly runs.

## Plan

Found by the checkers quest's review
([kixel-ai/rust2#891](https://github.com/kixel-ai/rust2/pull/891)). Kotlin crosses
`usize` as a 64-bit `Long` on every target, since one binding serves them all. The glue
narrows it with `rt::narrow` (`ffi/runtime/jni.rs:1241`), which calls `rt::fail`. The
repository's rules allow an abort only where the language has no error of its own.

Recommended: Kotlin checks the width before the call, against the library's pointer width
read once at load, as it already checks a `Duration`'s nanoseconds. Narrowing inside the
glue cannot unwind out of a nested `Wire::read`. The other bindings use a native type the
width of the target's `usize`, so they need nothing.

Public API: none. A behavior change: an abort becomes an exception.
