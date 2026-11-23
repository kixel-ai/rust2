# [L] React Native bindings

## Goal

[rust2-ffi](/ffi/README.md) writes a React Native module for the crate: TypeScript over a
JSI module in C++ that calls the C ABI, so a call from Hermes reaches Rust with no bridge
serialization, on iOS and Android. The `api` fixture's script runs through
it on an Android emulator and an iOS simulator, nightly only: no merge-gate test runs it,
since nixpkgs packages no Hermes for a host-side one.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ planned in m3 before a
consumer asks ([m3](/quest/m3/README.md#plan)).

- Prior art: uniffi-bindgen-react-native (ubrn) generates JSI C++ and TypeScript from
  UniFFI's annotations; its releases trail UniFFI's (no release read UniFFI 0.32 by
  2026-11-09).
- It builds on what exists: `rt.hpp`'s futures and executors for the JS thread's
  scheduling, and the TypeScript API [rust2-wasm](/ffi/README.md#typescript-web) generates,
  which this should match so one application's code calls either.
- The JS runtime is single-threaded while Rust wakes from any thread, so a wake posts to
  the runtime's `CallInvoker`, never polling inside the wake, as every other binding's does.
- rust2-wasm cannot serve React Native: Hermes runs no WebAssembly. Checked by the
  2026-11-12 audit: [facebook/hermes#429](https://github.com/facebook/hermes/issues/429)
  is still open, and Hermes's default branch has no WebAssembly source in its VM. So the
  JSI module is the route.
- It follows every generator's rules ([every binding's](/ffi/README.md#every-binding)):
  the contract check at load, bulk payloads, native futures, a `ffi/moq/moq.*` run, and a
  column on [the binding benchmark](/bench/ffi/README.md).
- Decided with the maintainer on 2026-11-22: ✅ test nightly only, on an emulator and a
  simulator, and say so in the Goal, since nixpkgs does not package Hermes for a
  host-side gate test. Rejected: a gate test, which needs Hermes in the flake.
- The generated API is the one [the gate](/quest/m2/wasm/README.md) settled on
  2026-11-12: a module of functions per type, not classes.

Public API: the generated module's TypeScript API.

