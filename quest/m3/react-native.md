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
- Open: whether rust2-wasm can serve React Native instead. Check first whether Hermes
  runs WebAssembly, which the audit did not verify; ubrn generating JSI C++ suggests it
  does not. If it does, measure it as [the Node quest](/quest/m3/node.md) does for Node,
  and write the JSI module only if it wins or wasm cannot run the library (moq's native
  QUIC needs sockets).

Public API: the generated module's TypeScript API.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - the C ABI, its lossless types, and its checks are settled before a new generator copies them
- [The exporter's web target](/quest/m2/wasm/backend.md) - the TypeScript API this matches, and the route it is measured against
