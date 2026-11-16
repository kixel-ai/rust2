# [L] The exporter's web target off WASI, in browsers, on the benchmark, and refusing what the web lacks

## Goal

rust2-ffi's web target ([`ffi/src/web.rs`](/ffi/src/web.rs), documented in
[the README](/ffi/README.md#typescript-web)) runs the `api` fixture's script in Node, its
transcript matching native Rust's on `wasm32-wasip1`. What remains:

- The web module ships nothing of WASI, as [the epic](/quest/m2/wasm/README.md) decided on
  2026-11-09: its clock, randomness, and output reach web APIs through the binding's own
  `rust2_*` imports, and `rt.ts` implements no `wasi_snapshot_preview1` import. Today the
  binding builds on `wasm32-wasip1`'s std, and `rt.ts` implements its six imports
  (`clock_time_get`, `environ_get`, `environ_sizes_get`, `fd_write`, `proc_exit`,
  `sched_yield`) with web APIs.

- The same script runs headless in Chrome and Firefox from the dev shell (`pkgs.chromium`,
  `pkgs.firefox`), in `just ffi test`, with its futures and its `grown` case checked as in
  Node. Deno runs it too, or the [epic's](/quest/m2/wasm/README.md) Goal drops Deno.
- [The binding benchmark](/bench/ffi/README.md) gains a TypeScript column, in Node and a
  browser, as every generator is measured. Its stand-in must build for wasm32, which
  moq-tokio does not: export the crates of it that do, and say which. The column joins
  CodSpeed under the A/A rule of [the benchmark's README](/bench/ffi/README.md#codspeed).
  Its payload rows find the crossover for bytes Rust returns: below it they are copied out
  and freed at once, as today; above it they are lent, a view over Rust's buffer kept alive
  until a `FinalizationRegistry` frees it, as Dart lends past 64 KiB.
- What has no web API (the file system, sockets, a process) is refused at export, at the
  span of the call that reaches it. Today the module imports it, and `load` refuses the
  module, naming the import: loud, but at load, not at export. That needs every reachable
  body's MIR, which rust2-ts reads through its own MIR sysroot (`ts/src/profile.toml`,
  `ts/src/cli.rs`) and the exporter does not read at all: size it first, and split it out
  if it is most of the work, since [WASI](/quest/m3/wasi.md) needs the same.
- A crate that imports browser APIs through wasm-bindgen (web-sys) is refused at the
  import's span, until a consumer needs both. Today its module imports
  `__wbindgen_placeholder__`, which `load` refuses.
- A trait method or closure returning an object or a `Vec` from JavaScript to Rust has a
  web form: the generator refuses one today, and the api fixture has none.

## Plan

Decided with the maintainer on 2026-11-09: ✅ web and WASI are separate targets, and the
web's uses web APIs wherever one exists ("like performance.now()"). Rejected: one
`wasm32-wasip1` module for both, whose imports the binding satisfies, as this quest first
recommended. [The WASI target](/quest/m3/wasi.md) is its own quest.

Decided with the maintainer on 2026-11-12, in the quest audit
([the epic](/quest/m2/wasm/README.md) records the reasons): ✅ the binding's API is the
translator's shape, a module of functions per type. ✅ This quest adds the TypeScript
column to the binding benchmark.

Decided by the implementer on 2026-11-15, under the maintainer's standing instruction to
take the recommendation on each open decision, in
[#847](https://github.com/kixel-ai/rust2/pull/847), which landed the generator:

- ✅ A 64-bit integer crosses as the translator's `rt.u64`, a number while safe and limbs
  past it, converted from the wasm `bigint` at the boundary: 107 ns for a `u64` getter on
  Node 24 against 98 ns returning the `bigint` as it is. Rejected: `bigint`, which
  `ts/README.md` measured 8 to 65% slower on the codec, and which the translator's route
  does not share.
- ✅ For this slice, the clock and randomness come from `wasm32-wasip1`'s std, whose imports the binding
  implements with `performance.now`, `crypto.getRandomValues`, and the console: 24 KB over
  `wasm32-unknown-unknown`'s module on the api fixture (3%), 9.5 KB gzipped. Rejected:
  `web-time` or js-sys in the crate, a source change that brings wasm-bindgen back; a
  patched std through `-Zbuild-std`; waiting for a web std upstream, which has none.
- ✅ Bytes Rust returns are copied out and freed at once, so no view outlives a growth of
  memory. Rejected for now: an object of a pointer and length making its view at each
  read, which keeps Rust's buffer alive until a `FinalizationRegistry` frees it. A copy
  costs V8's `ArrayBuffer` allocation, 33 ns at 16 bytes but 0.8 µs at 1 KiB, against 0.2 µs
  for the registry: measure the crossover on the benchmark's payload rows and lend above
  it, as Dart lends past 64 KiB.

Decided with the maintainer on 2026-11-16, picking
[#847](https://github.com/kixel-ai/rust2/pull/847) to merge:

- ✅ Merge the first slice on `wasm32-wasip1`'s std, its WASI imports implemented with web
  APIs. Rejected: retargeting to `wasm32-unknown-unknown` before merging; skipping the
  slice. The 2026-11-09 decision, no WASI runtime or polyfill in the web bundle, stands as
  the goal, so this quest moves the web target off `wasm32-wasip1`.
- ✅ The copied-or-lent crossover for returned bytes is this quest's, measured on the
  benchmark column's payload rows. Rejected: a quest of its own; dropping it.

Recommended for moving off `wasm32-wasip1`: size it before choosing a route. On
`wasm32-unknown-unknown` the std's `Instant::now` and `SystemTime::now` panic and its
output is discarded, so the route must give std a clock, randomness, and output through
`rust2_*` imports without a source change or a consumer's config. #847's rejected options
(`web-time` or js-sys, a std patched through `-Zbuild-std`, a web std upstream) are the
starting survey; re-measure each against the 24 KB the WASI std costs today.

Recommended for what remains: split the MIR refusal into a quest of its own shared with
[WASI](/quest/m3/wasi.md) once the exporter's MIR reading is sized, and run the browsers through Playwright's or the dev shell's
headless binaries with a static server of the script's directory, never the user's
signed-in Chrome.
