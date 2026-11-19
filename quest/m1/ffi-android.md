# [S] Android: 16 KB pages and abort messages

## Goal

- The generated crate's build script links Android libraries with
  `-Wl,-z,max-page-size=16384`, a platform boundary needing no config, and `just ffi
  android` checks the built `.so`'s alignment with `readelf` and runs on a 16 KB page
  emulator image beside today's.
- A failed run-time check or panic that aborts on Android sets bionic's abort message
  (`android_set_abort_message`) and writes it to logcat, so it shows in the tombstone,
  logcat, `ApplicationExitInfo`, and Play vitals instead of vanishing with stderr.

## Plan

Found by the 2026-11-18 language audit. rustc does not set `max-page-size` for Android, so
alignment depends on the consumer's NDK: r28 and later align to 16 KB, r27 does not
([page sizes](https://developer.android.com/guide/practices/page-sizes)). From 2027-02-01,
Google Play rejects updates targeting API 35 and later whose 64-bit libraries are not
16 KB aligned. `rt::fail` is `eprintln!` then `abort()` (`ffi/runtime/rt.rs:33-36`), and
Android sends stderr nowhere.

Public API: none.

## Hosts

- linux - the Android emulator needs KVM
