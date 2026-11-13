# rust2-wasm: a crate's pub API in the browser

## Goal

[rust2-ffi](/ffi/README.md) exports a crate's `pub` API to WebAssembly with a generated
TypeScript binding, by the rules its other languages follow: no attributes, no config, no
wasm-bindgen, and no step after `cargo build`. The same package loads in browsers, Node,
Bun, and Deno. It is moq's browser route, and the baseline
[rust2-ts](/ts/README.md) has to beat.

What stays here once every child has merged, the gate: moq-net runs in a browser through
rust2-wasm, and one report compares the same `pub` API through rust2-wasm, through
rust2-ts's translation, and hand-written (`@moq/net`), on CPU time, shipped bytes, and
startup, in Chrome and Firefox. The maintainer decides from it whether translation
continues, and for which crates.

## Plan

The [per-item crossing spike](https://github.com/kixel-ai/rust2/pull/760) finds no clear
varint lead in Chrome, with frames and async still favoring wasm and Firefox medians
favoring wasm throughout; it recommends no backend design or priority change, since
the hand-written byte boundary and per-call task setup do not measure generated bindings.

Decided with the maintainer on 2026-11-09, in the quest audit:

- ✅ WASM ships, and TypeScript must beat it: rust2-ts is "an experiment to see if it can be
  an improvement over WASM", and wasm-bindgen "is kind of annoying to work with". This
  supersedes moq's 2026-10-12 "WASM is transitional and JS-only is the final goal"
  ([moq's quest](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-js.md)) and
  [the model epic](/quest/m2/ts-model/README.md)'s 2026-11-01 "the translated model is
  moq's browser route". Rejected: JS-only staying the goal, with rust2-wasm only a
  baseline; pausing rust2-ts until rust2-wasm is measured.
- ✅ Web and WASI are separate targets. [The web target](/quest/m2/wasm/backend.md) calls
  web APIs directly, `performance.now` for its clock, and ships nothing of WASI;
  [the WASI target](/quest/m3/wasi.md) serves hosts with no JavaScript, in m3 as the
  other targets planned ahead of a consumer are. Rejected: one `wasm32-wasip1` module for
  both, with the binding implementing its imports.
- ✅ m2, with per-target export in m1, which landed in
  [#689](https://github.com/kixel-ai/rust2/pull/689) and
  [#700](https://github.com/kixel-ai/rust2/pull/700). Rejected: all of it in m1; all of it
  in m2.
- ✅ Until the gate reports, translator work is only what the gate needs: the model's
  remaining refusals and the benchmark. [m3](/quest/m3/README.md#plan) records what waits.
  Rejected: continuing every translator quest in parallel; freezing the model's work too.

Decided with the maintainer on 2026-11-12, in the quest audit:

- ✅ The gate measures what it names. The audit found the benchmark could not: every
  route was one `run(&[u8]) -> Vec<u8>` call per sample, so no handle, record, or future
  crossed and the boundary was paid once; startup was timed nowhere; and the wasm route
  was built release at `opt-level = "z"` while the translation came from the dev profile,
  with overflow checks and debug assertions on. So
  [the backend](/quest/m2/wasm/backend.md) adds a TypeScript column to
  [the binding benchmark](/bench/ffi/README.md), whose cases cross handles, records,
  objects, reads, and payloads, and [the measurement](/quest/m2/wasm/routes.md) adds the
  translation of the same cases, a cold start, and matching build profiles. Rejected:
  startup and profiles only, with compute-only workloads; rewording the gate to what the
  benchmark measured.
- ✅ One TypeScript API for both routes, the translator's: a module of functions per type,
  called as `Decoder.slice(dec, 4)`, plain objects for records, an object with
  `[Symbol.dispose]` only for what drops, which in wasm is the handle, and a `Promise`
  from a root async function. `ts/README.md`'s design measured free functions, classes,
  and a class facade within noise of each other on speed, and free functions 13 to 21%
  smaller gzipped than classes on its two callers (8.3 and 7.5 KB against 9.5), since no
  bundler drops a class's unused methods; the facade was the largest. A consumer then swaps routes per
  crate with no source change. Rejected: the binding's classes for both, as the other
  languages' bindings have; a surface per route.
- Open, with no owner yet: the gate's own deliverable, moq-net running in a browser
  through rust2-wasm, needs a transport adapter over WebTransport, and
  [the backend](/quest/m2/wasm/backend.md) recommends refusing a crate that imports
  browser APIs. moq's tree plans none of it. Plan it with moq's tree before the gate's
  last child lands.
- Deno is in the Goal and tested by no child, and is not in the flake. The backend tests
  it or the Goal drops it.

The gate sets no threshold, as the other benchmarks set none: rust2-go's "within 2x of
cgo" was the maintainer's call from its report too. It compares every workload all routes
run, the codec's today and the model's once it translates, so it need not wait on moq's
[model rewrite](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-model-subset.md).

Why this shape, from the audit's survey of 2026's tools:

- wasm-bindgen is maintained (0.2.130 on 2026-10-30), so the case against it is its
  design: an attribute on every export, a CLI locked to the crate's version, and glue per
  value. Diplomat already runs a C ABI over `wasm32-unknown-unknown` with generated JS.
- Not the Component Model: it is phase 1 with no browser engine behind it, runs in
  browsers only through `jco transpile`, and its sync calls cost about 3.5 times a plain
  one today. The exporter's IR could emit WIT later; nothing plans it.
- This is also how [m3's unsafe policy](/quest/m3/README.md) compiles an `unsafe`-heavy
  crate behind its public API (rule 4).

moq's [`rs2ts-js`](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-js.md) and
[`rs2ts-wasm2js`](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-wasm2js.md) still
record the superseded decision; they are moq's tree's to update.
