# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on, and what every agent pays for on
each run: the checks, a red nightly, and code no consumer uses.

## Required

- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
- [The nightly passes](/quest/m0/nightly-green.md) - red since 2026-10-29: the wasm-bindgen fixture's NaN, Kotlin on Android API 35, and `just ffi moq`
