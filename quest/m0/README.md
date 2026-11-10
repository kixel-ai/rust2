# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on, and what every agent pays for on
each run: the checks, a red nightly, and code no consumer uses.

## Required

- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
- [Delete the UniFFI fork](/quest/m0/delete-uniffi.md) - `uniffi/`, its workspace, flake packages, nightly jobs, and the benchmark harness built on it are gone
- [The nightly passes](/quest/m0/nightly-green.md) - red since 2026-10-29: the wasm-bindgen fixture's NaN, Kotlin on Android API 35, and `just ffi moq`
- [Trim the build-speed benchmark](/quest/m0/bench-trim.md) - it runs on a pin change, not nightly, without the nix job and the `pure` corpora
