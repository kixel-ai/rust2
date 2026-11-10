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

Decided with the maintainer on 2026-11-09, in the quest audit:

- ✅ WASM ships, and TypeScript must beat it: rust2-ts is "an experiment to see if it can be
  an improvement over WASM", and wasm-bindgen "is kind of annoying to work with". This
  supersedes moq's 2026-10-12 "WASM is transitional and JS-only is the final goal"
  ([moq's quest](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-js.md)) and
  [the model epic](/quest/m2/ts-model/README.md)'s 2026-11-01 "the translated model is
  moq's browser route". Rejected: JS-only staying the goal, with rust2-wasm only a
  baseline; pausing rust2-ts until rust2-wasm is measured.
- ✅ m2, with [per-target export](/quest/m1/ffi-export/target.md) in m1. Rejected: all of
  it in m1; all of it in m2.
- ✅ Until the gate reports, translator work is only what the gate needs: the model's
  remaining refusals and the benchmark. [m3](/quest/m3/README.md#plan) records what waits.
  Rejected: continuing every translator quest in parallel; freezing the model's work too.

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

## Required

- [The exporter's wasm32 target](/quest/m2/wasm/backend.md) - the glue crate builds for wasm32 with a generated TypeScript binding, proven on the api fixture
- [Every route on the browser benchmark](/quest/m2/wasm/routes.md) - `just ts bench` runs rust2-wasm's export and a `wasm2js` control beside the translation
