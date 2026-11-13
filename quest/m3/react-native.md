# [L] React Native bindings

## Goal

[rust2-ffi](/ffi/README.md) writes a React Native module for the crate: TypeScript over a
JSI module in C++ that calls the generated C++ header, so a call from Hermes reaches Rust
with no bridge serialization, on iOS and Android. The `api` fixture's script runs through
it on an Android emulator and an iOS simulator, nightly.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ planned in m3 before a
consumer asks ([m3](/quest/m3/README.md#plan)).

- Prior art: uniffi-bindgen-react-native (ubrn) generates JSI C++ and TypeScript from
  UniFFI's annotations; its releases trail UniFFI's (no release read UniFFI 0.32 by
  2026-11-09).
- It builds on what exists: `rt.hpp`'s futures and executors for the JS thread's
  scheduling, and the TypeScript API [rust2-wasm](/quest/m2/wasm/backend.md) generates,
  which this should match so one application's code calls either.
- The JS runtime is single-threaded while Rust wakes from any thread, so a wake posts to
  the runtime's `CallInvoker`, never polling inside the wake, as every other binding's does.
- rust2-wasm cannot serve React Native: Hermes runs no WebAssembly. Checked by the
  2026-11-12 audit: [facebook/hermes#429](https://github.com/facebook/hermes/issues/429)
  is still open, and Hermes's default branch has no WebAssembly source in its VM. So the
  JSI module is the route.
- It follows every generator's rules ([the exporter epic](/quest/m1/ffi-export/README.md)):
  the contract check at load, bulk payloads, native futures, a `ffi/moq/moq.*` run, and a
  column on [the binding benchmark](/bench/ffi/README.md).
- Open: every other target runs its `tests/api.*` in the merge gate, and this Goal tests
  only nightly, on an emulator and a simulator. A host-side test needs Hermes in the
  flake, which nixpkgs does not package. Recommended: accept nightly only, and say so in
  the Goal. The maintainer confirms before this starts.
- The generated API is the one [the gate](/quest/m2/wasm/README.md) settled on
  2026-11-12: a module of functions per type, not classes.

Public API: the generated module's TypeScript API.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - the C ABI, its lossless types, and its checks are settled before a new generator copies them
- [The exporter's web target](/quest/m2/wasm/backend.md) - the TypeScript API this matches, and the route it is measured against
