# [S] Kotlin: a closure's usize too wide for a 32-bit target throws

## Goal

On a 32-bit Android ABI (armv7, x86), a Kotlin closure or callback that returns a `ULong`
past `usize::MAX`, or a `Long` outside `isize`'s range, raises `IllegalArgumentException`
at the Kotlin call that reached the closure, as an out-of-range argument does, instead of
aborting the process. `tests/api.kt`'s `lossless()` checks it on the 32-bit emulator that
the nightly runs.

## Plan

Found by the checkers quest's review
([kixel-ai/rust2#891](https://github.com/kixel-ai/rust2/pull/891)). Kotlin crosses
`usize` as a 64-bit `Long` on every target, since one binding serves them all.

Already done: `to_signed` (`ffi/src/kotlin.rs`) wraps every `usize` and `isize` that
Kotlin hands over, whether a parameter, a record field, an encoded value, or a closure's
return, in `_Native.__usize` or `__isize` (`ffi/runtime/rt.kt`). Those helpers check the
value against the library's `usize::MAX`, which is read once at load through
`__usize_max`, and throw `__Invalid`, an `IllegalArgumentException`, before the call
crosses. So an argument or field out of range already throws and leaves the call unmade,
and `rt::narrow` (`ffi/runtime/jni.rs`) never fails for a value Kotlin passes.

Left: a closure's return. There `__usize` throws inside the upcall, and `rt::upcall`'s
`rt::threw` aborts on the pending exception, as it does for anything a closure with no
`Result` return throws. Another width check cannot fix this. The exception has to cross
the Rust frames between the upcall and the outer Kotlin call. One way: the upcall leaves
the exception pending, returns a zeroed value, and marks the thread so that each Rust
frame between it and the outermost JNI entry returns at once. That entry then lets JNI
rethrow. Without unwinding, the Rust code in between would keep running on a value the
closure never returned. That breaks the source's semantics, which is why abort is today's
behavior.

Public API: none. A behavior change: an abort becomes an exception.

## Waiting

- Maintainer: keep or drop this quest. Every closure that throws aborts today, because a
  Rust caller of a closure with no `Result` return has no error path, and the repository's
  rules allow an abort where the language has no error of its own. An out-of-range
  return is one more such throw. Keeping it means unwinding a Kotlin exception through
  Rust frames, a far larger change than `[S]`, and it should then cover every closure
  throw. Recommended: drop it, since the abort follows the rules and only a closure
  returning a usize too wide for a 32-bit target reaches it.
