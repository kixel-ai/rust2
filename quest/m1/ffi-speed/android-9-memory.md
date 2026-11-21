# [M] Find what grows the Kotlin benchmark's native memory on Android 9

## Goal

`export.kt`, [the binding benchmark](/bench/ffi/README.md#kotlin-on-art)'s Kotlin, finishes
on an Android 9 (API 28) emulator as it does on API 30 and 35, or the binding's part in what
stops it is fixed, with a check that would have caught it.

## Plan

Found by the ART measurement on 2026-11-20. On an x86_64 API 28 emulator with 2 GB of RAM,
`dalvikvm -Xusejit:true -Xmx256m` running the R8-shrunk `export.kt` grows its resident
memory from 632 MB at 20 s to 1.5 GB at 100 s, past the heap's cap, so the growth is native;
the device then thrashes and its system server restarts. It does so with and without
`@FastNative`, and on main's generator too. `just ffi android`, the api fixture, passes
there. Bisect by case first: the bench's loops make and close millions of handles, frames,
and timestamps, so a leak per call, such as a JNI reference ART 9 keeps or a handle freed
only by the collector, is the first suspect. `just ffi bench-android` runs it on the device
adb reaches, so start an API 28 emulator and make it the only device, or name it in
`ANDROID_SERIAL`.

Public API: none expected.

## Hosts

- linux - the Android emulator needs KVM
