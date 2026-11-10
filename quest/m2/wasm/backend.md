# [L] The exporter's wasm32 target

## Goal

`rust2-ffi --target wasm32-unknown-unknown` writes, beside the glue crate, a TypeScript
binding over the `.wasm` that `cargo build` makes of it: ES modules a bundler takes as
is, typed as the other languages' bindings are, with no wasm-bindgen and no tool run on
the `.wasm` afterwards. The `api` fixture's script runs through it in Node, Chrome, and
Firefox, its transcript matching native Rust's line for line, with its futures checked as
the other languages' are.

## Plan

Decided with the maintainer on 2026-11-09: planned in
[the rust2-wasm epic](/quest/m2/wasm/README.md). The rules every generator follows
([the exporter epic](/quest/m1/ffi-export/README.md)) apply: the contract check at load,
bulk payloads, native futures, lossless types, and checks that raise.

Recommended design, from the audit's survey, for the implementer to confirm by
measurement on [the benchmark](/quest/m2/wasm/routes.md):

- Records are read from and written to linear memory at the layouts rustc computed for
  the target, with no serialization and no JS object per field until one is read.
- Bytes Rust lends are a `Uint8Array` view for the call, and a copy where kept, since a
  view detaches when memory grows. Bytes passed in are copied once into memory the call
  frees. Text is one `TextDecoder` or `TextEncoder` call; JS string builtins do not help
  UTF-8 held in linear memory.
- A handle is an integer in a class with `[Symbol.dispose]`, and a `FinalizationRegistry`
  frees one never disposed, as the other languages' garbage collectors do.
- A future is a `Promise`: the binding polls once at the call, and an imported `wake`
  schedules the next poll on a microtask, never inside the wake. Cancelling frees the
  future and loses nothing. JSPI is not needed, since no Rust code blocks on JS.
- One thread: no atomics, no shared memory, no COOP or COEP headers. A type that is not
  `Send` exports with no check ([not-send](/quest/m1/ffi-export/not-send.md)).
- The runtime's `fail` writes its message through an import, since wasm32 has no stderr.
- A struct passed by value is passed indirectly in wasm's C ABI, which the binding must
  match from the target's layouts, not the host's.

Open, for the maintainer once the implementer has the facts:

- What a reachable call into std's parts that wasm32-unknown-unknown lacks does:
  `Instant::now`, `SystemTime::now`, threads, sockets, and the file system compile there
  and panic when called. Recommended: refuse at export, with the span of the call, any
  exported item whose body reaches one, as rust2-ts already reads every body's MIR;
  rejected for now: binding them to JS (a clock over `performance.now`), which is a
  platform shim the crate did not ask for.
- How a crate that itself imports browser APIs through wasm-bindgen (web-sys) exports.
  Recommended: refuse it at the import's span until a consumer needs both.

Public API: the TypeScript binding's shape, and `--target wasm32-unknown-unknown`.

## Required

- [Export for a target](/quest/m1/ffi-export/target.md) - the exporter checks and lays out the API for wasm32, not the host
- [Lossless binding types](/quest/m1/ffi-export/lossless.md) - the types a new binding copies are the exact ones
- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - the one accounting a new binding copies raises
