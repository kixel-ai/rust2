# [L] The exporter's wasm32 target

## Goal

`rust2-ffi --target wasm32-unknown-unknown` writes, beside the glue crate, a TypeScript
binding over the `.wasm` that `cargo build` makes of it: ES modules a bundler takes as
is, typed as the other languages' bindings are, with no wasm-bindgen and no tool run on
the `.wasm` afterwards. The `api` fixture's script runs through it in Node, Chrome, and
Firefox, its transcript matching native Rust's line for line, with its futures checked as
the other languages' are. The fixture spawns a thread (`ffi/fixtures/api/src/lib.rs`),
which wasm32 cannot, so what needs one moves to a fixture of its own that the native
languages keep running.

## Plan

Decided with the maintainer on 2026-11-09: planned in
[the rust2-wasm epic](/quest/m2/wasm/README.md). The rules every generator follows
([the exporter epic](/quest/m1/ffi-export/README.md)) apply: the contract check at load,
bulk payloads, native futures, lossless types, and checks that raise.

Recommended design, from the audit's survey, for the implementer to confirm by
measurement on [the benchmark](/quest/m2/wasm/routes.md):

- Records are read from and written to linear memory at the layouts rustc computed for
  the target, with no serialization and no JS object per field until one is read.
- A `Uint8Array` view of linear memory detaches whenever memory grows, which any call
  into Rust can cause, one a callback makes while it holds lent bytes included. So bytes
  Rust returns or lends are an object of their pointer and length that makes its view
  from the current `memory.buffer` at each read, and hands out a copy where a caller
  wants a `Uint8Array` to keep. Test a callback that grows memory while it reads what it
  was lent. Bytes passed in are copied once into memory the call frees. Text is one `TextDecoder` or `TextEncoder` call; JS string builtins do not help
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

- What std's parts that wasm32-unknown-unknown lacks do. The file system, sockets, and
  `thread::Builder::spawn` return errors there, which is the target's Rust and needs
  nothing. `Instant::now` and `SystemTime::now` panic, and moq-net reaches them:
  `Timestamp::now` reads kio's clock, whose fallback is `Instant::now`, so refusing the
  call would refuse the gate's own consumer. Options: build for `wasm32-wasip1` instead,
  whose std has a clock, randomness, and stderr through a handful of imports the
  generated binding implements (`performance.now`, `crypto.getRandomValues`), a platform
  boundary AGENTS.md allows; refuse at export, with the call's span, any exported item
  whose body reaches a call that can only panic, as rust2-ts already reads every body's
  MIR; or moq gives kio a wasm32 clock, a source change AGENTS.md calls a gap.
  Recommended: the first, if its imports stay that few and its `.wasm` no larger;
  measure both targets on the fixture before choosing.
- How a crate that itself imports browser APIs through wasm-bindgen (web-sys) exports.
  Recommended: refuse it at the import's span until a consumer needs both.

Public API: the TypeScript binding's shape, and `--target wasm32-unknown-unknown`.

## Required

- [Export for a target](/quest/m1/ffi-export/target.md) - the exporter checks and lays out the API for wasm32, not the host
- [Lossless binding types](/quest/m1/ffi-export/lossless.md) - the types a new binding copies are the exact ones
- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - the one accounting a new binding copies raises
