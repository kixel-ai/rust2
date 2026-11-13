# [L] The exporter's web target

## Goal

rust2-ffi's web target writes, beside the glue crate, a TypeScript binding over the
`.wasm` that `cargo build` makes of it: ES modules a bundler takes as is, typed as the
other languages' bindings are, with no wasm-bindgen and no tool run on the `.wasm`
afterwards. What the module needs of its platform comes from web APIs, called directly:
its clock is `performance.now` and `Date.now`, its randomness `crypto.getRandomValues`,
and its stderr `console.error`, with no WASI runtime or polyfill in the bundle. The `api`
fixture's script runs through it in Node, Chrome, and
Firefox, its transcript matching native Rust's line for line, with its futures checked as
the other languages' are. The fixture spawns a thread (`ffi/fixtures/api/src/lib.rs`),
which wasm32 cannot, so what needs one moves to a fixture of its own that the native
languages keep running.

## Plan

Decided with the maintainer on 2026-11-09: ✅ web and WASI are separate targets, and the
web's uses web APIs wherever one exists ("like performance.now()"). Rejected: one
`wasm32-wasip1` module for both, whose imports the binding satisfies, as this quest first
recommended. [The WASI target](/quest/m3/wasi.md) is its own quest.

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
  was lent. Bytes passed in are copied once into memory the call frees. Text is one
  `TextDecoder` or `TextEncoder` call; JS string builtins do not help UTF-8 held in
  linear memory.
- A handle is an integer in an object whose only method is `[Symbol.dispose]`, and a
  `FinalizationRegistry` frees one never disposed, as the other languages' garbage
  collectors do. Measure a returned object with and without the registry: the same
  pattern is what [object ownership](/quest/m1/ffi-speed/object-ownership.md) found
  costing ten times a C return in Go and Dart.
- A future is a `Promise`: the binding polls once at the call, and an imported `wake`
  schedules the next poll on a microtask, never inside the wake. Cancelling frees the
  future and loses nothing. JSPI is not needed, since no Rust code blocks on JS.
- One thread: no atomics, no shared memory, no COOP or COEP headers. A type that is not
  `Send` exports with no check ([not-send](/quest/m1/ffi-export/not-send.md)).
- The runtime's `fail` writes its message to `console.error`.
- A struct passed by value is passed indirectly in wasm's C ABI, which the binding must
  match from the target's layouts, not the host's.

Decided with the maintainer on 2026-11-12, in the quest audit
([the epic](/quest/m2/wasm/README.md) records the reasons):

- ✅ The binding's API is the translator's shape, not a class per type: a module of
  functions per type that take the handle first, plain objects for records, and a
  `Promise` from an async function, so one caller's code runs on either route and a
  bundler drops the glue of every function it never calls. Rejected: classes with
  methods, as this quest first planned and the other languages' bindings have.
- ✅ This quest adds a TypeScript column to [the binding benchmark](/bench/ffi/README.md),
  in Node and a browser, as every generator is measured. Its stand-in must build for
  wasm32, which moq-tokio does not: export the crates of it that do, and say which.

Open, for the maintainer once the implementer has the facts:

- How a 64-bit integer crosses. A wasm `i64` reaches JavaScript as a `bigint`, which
  `ts/README.md` measured 8 to 65% slower than the translator's numbers-while-safe form on
  the codec, and both routes must share one type for a caller to swap them. Recommended:
  the translator's branded `rs.u64`, converted at the boundary, with the conversion's
  cost on the benchmark's u64 getter row.

- How a stock crate's `std::time::Instant::now` becomes `performance.now`. moq-net
  reaches it: `Timestamp::now` reads kio's clock, whose fallback is `Instant::now`. std
  for `wasm32-unknown-unknown` has no clock and no hook for one, so the call panics
  there, and getrandom and uuid refuse that target until a binary picks a backend, whose
  browser one is wasm-bindgen's (`ts/README.md`). The ways to a web clock:
  - compile against a std that imports its platform, `wasm32-wasip1`'s, and have the
    generated binding implement each import the module declares with the web API for it.
    The import names are then a detail between the generated `.wasm` and its generated
    binding, not a WASI contract, and nothing of WASI ships;
  - a web std of rustc's own: none exists. On 2026-11-12 the platform support page on
    rust-lang/rust's `master` lists no web target, only the tier-3, `no_std`
    `wasm32-none-component`, added on 2026-10-15 and so in no nightly up to the pin;
  - the crate calls `web-time` or js-sys itself, a source change AGENTS.md calls a gap, and
    one that brings wasm-bindgen back;
  - a patched std built with `-Zbuild-std`, which is hand-rolling what upstream maintains.
  Recommended: the first, measured for size against `wasm32-unknown-unknown` on the
  fixture. Whichever is chosen, what has no web API (the file system, sockets, the
  environment, a process) is refused at export, at the span of the call that reaches it,
  rather than failing when the module loads or the call runs. That needs every reachable
  body's MIR, which rust2-ts reads through its own MIR sysroot (`ts/src/profile.toml`,
  `ts/src/cli.rs`) and the exporter does not read at all today: size it before starting,
  and split it out if it is most of the work, since [WASI](/quest/m3/wasi.md) needs the
  same.
- How a crate that itself imports browser APIs through wasm-bindgen (web-sys) exports.
  Recommended: refuse it at the import's span until a consumer needs both.

Public API: the TypeScript binding's shape, and the name of the web target.

## Required

- [Lossless binding types](/quest/m1/ffi-export/lossless.md) - the types a new binding copies are the exact ones
