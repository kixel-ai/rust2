# m3: deferred

## Goal

Work that waits on evidence or has no consumer yet: two waits on upstream's build-speed
levers, the translator's features that wait on [the gate](/quest/m2/wasm/README.md), the
binding targets planned ahead of a consumer, and what waits on moq.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit:

- ✅ Until [the gate](/quest/m2/wasm/README.md) reports, translator work is only what the
  gate needs. The quests here that no consumer calls wait on it:
  [no `ts.toml`](/quest/m3/ts-configless.md) and
  [browser APIs through wasm-bindgen](/quest/m3/ts-wasm-bindgen/README.md). This
  supersedes, until then, the 2026-10-27 decision that rust2 aims to support all of std,
  so a generic std gap was planned here even when moq never calls it. Rejected:
  continuing as before; freezing the model's work too.
- ✅ C# and Unity, a Node-API addon, and React Native are planned here before any consumer
  asks, since they are the targets a media SDK's users expect: LiveKit ships Python, Node,
  Unity, and React Native over its Rust core. Each waits on
  [the exporter epic](/quest/m1/ffi-export/README.md), so it copies the settled C ABI,
  types, and checks. Rejected: none until a consumer asks. Not planned: Java without
  Kotlin (the JNI library serves it, and Android has no FFM API), Ruby, PHP, Elixir, Lua, and game engines.

Decided with the maintainer on 2026-11-07, the unsafe policy, to stop one-off fixes for
ever rarer `unsafe`:

1. ✅ Safe Rust is the contract, and where translation effort goes.
2. ✅ `unsafe` inside std and common crates is wrapped at their safe API, as std,
   `hashbrown`, `bytes`, [tokio's channels](/ts/src/lower/tokio.rs), and
   [smallvec](/ts/src/lower/smallvec.rs) are, rather than translated.
3. ✅ Other `unsafe` translates only where it fits the translator's existing pointer forms
   (a place, an array element, a byte in a buffer). A construct needing a new pointer form
   or pointer metadata is refused with its span, unless a consumer needs it and no library
   wrapper covers it.
4. ✅ An `unsafe`-heavy crate a consumer needs is compiled to WASM behind its public API
   rather than fixed construct by construct, through
   [rust2-wasm](/quest/m2/wasm/README.md) once it exists. Mixing a translated crate with
   one compiled so is not planned until a consumer needs it.
5. ✅ Where translation can't see where an `unsafe` pointer came from, a loud run-time
   check stands in for the refusal, the exception AGENTS.md names: `ptrUnerase`,
   `offset_from` across arrays, `AtIndex.reach`, and raw slices of a field's byte
   (decided 2026-11-07 in [#661](https://github.com/moq-dev/rust2/pull/661) and
   [#621](https://github.com/moq-dev/rust2/pull/621)). Safe code gets no such check: it
   translates exactly or is refused.

Rejected: an unsafe std gap with no consumer becoming its own quest, as
`MaybeUninit::zeroed`, `Rc::new_uninit`, and `ptr::write_bytes` would have
([#647](https://github.com/moq-dev/rust2/pull/647)), or as `Vec::from_raw_parts`'s adopted
storage did ([#635](https://github.com/moq-dev/rust2/pull/635)); and a second, byte-backed
memory model for values whose address is reinterpreted, without a consumer.

## Required

- [Sessions on sans-IO QUIC](/quest/m3/session-quic.md) - bindings move UDP datagrams and drive the session's poll themselves, over moq's runtime-free QUIC driver once it lands
- [C# and Unity bindings](/quest/m3/csharp.md) - a C# generator over the C ABI that runs under .NET and Unity's IL2CPP
- [A Node-API addon](/quest/m3/node.md) - a native Node binding, kept only if it beats rust2-wasm and `node:ffi`
- [React Native bindings](/quest/m3/react-native.md) - a JSI module over the generated C++ header
- [Retrial Cranelift on moq's tests](/quest/m3/cranelift-unwind.md) - waits on upstream: moq's tests pass under Cranelift, so its CI can weigh it again
- [Relink without rebuilding](/quest/m3/relink.md) - measure upstream's skip of dependency-only rebuilds on moq's edits once it reaches nightly
- [Waiting: drop the lang fixture's mul_add allowance](/quest/m3/ts-fma-allowance.md) - the differential fails on any `mul_add` difference again once wasm32's `fma` rounds exactly
- [Waiting: type-check in process with tsrs](/quest/m3/ts-tsrs.md) - rust2-ts checks bindings through tsrs instead of `tsc` once it is a stable library
- [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md) - no runtime `rust2-ts:` error where native Rust goes on: exact, or refused at translation
- [rust2-ts with no ts.toml](/quest/m3/ts-configless.md) - a crate's whole `pub` API translates with no config and no bound items
- [Browser APIs through wasm-bindgen](/quest/m3/ts-wasm-bindgen/README.md) - web-sys and js-sys imports become direct JS calls, so idiomatic crates such as web-rs's web-codecs translate as stock crates
- [`u64` wrapping arithmetic in V8](/quest/m3/ts-u64-wrapping.md) - `rs.u64WrappingAdd` and its kin cost near a plain `+` while the value is a safe integer
