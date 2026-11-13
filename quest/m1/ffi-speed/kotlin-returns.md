# [S] Kotlin returns a fallible call's value directly

## Goal

A [rust2-ffi](/ffi/README.md) Kotlin call that returns a `Result` or a tuple returns its
value with no `ByteArray` on the success path, and a returned object takes no lock every
other object shares. On [the binding benchmark](/bench/ffi/README.md) Kotlin's write costs
230 ns of CPU time against C's 148, and its object return 210 against C's 25.

## Plan

Found by the 2026-11-12 audit; neither cost is another quest's:
[records](/quest/m1/ffi-speed/kotlin-records.md) covers records only, and
[object ownership](/quest/m1/ffi-speed/object-ownership.md) Go and Dart only.

- Today any call with an out pointer returns a `jbyteArray` that Kotlin re-reads through
  `Native.Reader`, on success too (`ffi/src/kotlin.rs`, `ffi/runtime/jni.rs`). Return the
  scalar or handle directly and raise the error from native through a static factory; a
  `Result<(), E>` returns null or the encoded error. This adds one R8 keep rule.
- Every owned handle links and unlinks under one process-wide `synchronized(LOCK)`
  (`ffi/runtime/rt.kt`'s `Cleanup`). Stripe the registry or use a concurrent set. The
  bench is single-threaded, so add a case that returns objects from several threads to
  show the contended cost before and after.

Public API: none; the JNI functions between the generated Kotlin and its glue change.
