# [M] Kotlin: a closure's usize too wide for a 32-bit target throws

## Goal

On a 32-bit Android ABI (armv7, x86), a Kotlin closure or callback that returns a `ULong`
past `usize::MAX`, or a `Long` outside `isize`'s range, raises `IllegalArgumentException`
at the Kotlin call that reached the closure, as an out-of-range argument does, instead of
aborting the process. The binding checks the width in Kotlin, against the library's
pointer width read once at load, never in the Rust glue. `tests/api.kt`'s `lossless()`
checks it on the 32-bit emulator that the nightly runs.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#891](https://github.com/kixel-ai/rust2/pull/891): ✅ kept, and the binding checks the width in Kotlin against the library's pointer width, read once at load (rejected: checking it in the Rust glue; dropping the quest).

Found by the checkers quest's review
([kixel-ai/rust2#891](https://github.com/kixel-ai/rust2/pull/891)). Kotlin crosses
`usize` as a 64-bit `Long` on every target, since one binding serves them all.

The check itself is in place: `to_signed` (`ffi/src/kotlin.rs`) wraps every `usize` and
`isize` that Kotlin hands over, whether a parameter, a record field, an encoded value, or
a closure's return, in `_Native.__usize` or `__isize` (`ffi/runtime/rt.kt`). Those helpers
check the value against the library's `usize::MAX`, read once at load through
`__usize_max`, and throw `__Invalid`, an `IllegalArgumentException`, before the value
crosses. So an argument or field out of range already throws and leaves the call unmade,
and `rt::narrow` (`ffi/runtime/jni.rs`) never fails for a value Kotlin passes. Keep it
that way: no width check moves into the Rust glue.

Left: carrying a closure return's exception out. There `__usize` throws inside the upcall,
and `rt::upcall`'s `rt::threw` aborts on the pending exception, as it does for anything a
closure with no `Result` return throws. The exception has to cross the Rust frames between
the upcall and the outer Kotlin call. One way: the upcall leaves the exception pending,
returns a zeroed value, and marks the thread so that each Rust frame between it and the
outermost JNI entry returns at once; that entry then lets JNI rethrow. Without unwinding,
the Rust code in between would keep running on a value the closure never returned, which
breaks the source's semantics. Scope it to the width check's `__Invalid`: any other throw
from a closure keeps aborting, as the repository's rules allow where Rust has no error
path. Sized M, not S, for that unwinding.

Public API: none. A behavior change: an abort becomes an exception.
