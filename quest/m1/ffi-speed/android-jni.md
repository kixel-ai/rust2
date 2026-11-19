# [M] Measure Kotlin on ART, then try @FastNative

## Goal

[The binding benchmark](/bench/ffi/README.md) reports the Kotlin rows on ART, on the
Android emulator the nightly already runs, beside HotSpot's. Then a spike registers the
JNI functions with `RegisterNatives` from the binding's first-call `load()` and marks
short, non-blocking calls `@FastNative`; it lands only where ART measures it faster, and
otherwise its numbers go in the PR and the README.

## Plan

Found by the 2026-11-18 language audit. Every Kotlin number today is HotSpot's: the getter
costs 24 ns wall against BoltFFI's 5.8. Android lists `@FastNative` near 35 ns against a
regular JNI call ([JNI tips](https://developer.android.com/ndk/guides/jni-tips)).

- `@FastNative` needs `RegisterNatives` on Android 8 to 11, where the glue uses `Java_`
  name lookup today (`ffi/src/kotlin.rs:2280`); registering also drops the exported
  `Java_` symbols, without a `JNI_OnLoad`.
- It is public SDK API only from API 34, so the JVM build needs a stub annotation.
- It blocks the GC for the call, so it suits only calls that never block or call back.
- `@CriticalNative` does not fit: it has no `JNIEnv` to throw a failed check, and
  Android 7, which moq's API 24 floor includes, does not support it.

Public API: none.

## Hosts

- linux - the Android emulator needs KVM
