# [M] Every route on the browser benchmark

## Goal

`just ts bench` ([the TS benchmark](/ts/README.md#bench)) times each workload through two
more routes, in Chrome, Firefox, Node, and Bun: the fixture's `pub` API exported by
[rust2-wasm](/quest/m2/wasm/backend.md), called through its generated binding, and the
same `.wasm` lowered to JavaScript by Binaryen's `wasm2js`. Each route's output must match
native Rust's first, and the report gives each route's shipped bytes, raw and gzipped,
with its `.wasm` or runtime included, its cold start, from a fresh worker to its first
result with `WebAssembly.compile` or the module's evaluation inside the time, and how long
it takes to build. [The binding benchmark](/bench/ffi/README.md)'s cases, which cross
handles, records, objects, reads, and payloads, run through the translation too, beside
the column [the backend](/quest/m2/wasm/backend.md) adds for rust2-wasm.

## Plan

Decided with the maintainer on 2026-11-09: this is the measurement
[the gate](/quest/m2/wasm/README.md) reads.

- The benchmark's existing wasm32 route stays, as compiled Rust with a hand-written
  boundary whose copies are timed apart: against it, the rust2-wasm route shows what the
  generated binding costs.
- `wasm2js` is the control no one has measured: it needs no translator and no WebAssembly
  at run time, which is what moq's
  [`rs2ts-wasm2js`](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-wasm2js.md)
  planned to try first. No published speed or size figures for it were found. If it
  matches the translation on speed and size, that is a finding about the translator, to
  report plainly.
- What is published points both ways, so only this benchmark decides: a Rust and WASM
  parser lost 2.2 to 4.6 times to a TypeScript rewrite on boundary costs
  ([OpenUI, 2026-03](https://www.openui.com/blog/rust-wasm-parser)), while Scala.js on
  Wasm won up to 5.8 times on hashing and lost on object-heavy workloads
  ([VirtusLab, 2024-07](https://virtuslab.com/blog/scala/scala-to-web-assembly/)).
- Decided with the maintainer on 2026-11-12, in the quest audit: ✅ the routes are built
  alike. Today `ts/Cargo.toml`'s `wasm-release` profile is release at `opt-level = "z"`,
  where overflow wraps and debug assertions are off, while `ts/src/cli.rs` translates
  from the dev profile, with both on. Add a wasm route built with overflow checks and
  debug assertions, or a translation with release semantics, so each pair compared runs
  the same checks; the report says which pair it compares.
- The existing benchmark crosses the boundary once per sample, since every route is one
  `run(&[u8]) -> Vec<u8>` (`ts/bench/entry.ts`), which is why the binding benchmark's
  cases join it.
- moq's `js/net/bench/frames.ts` workload is [the model epic](/quest/m2/ts-model/README.md)'s
  to add once the model translates, on every route here.
- `wasm2js` needs Binaryen, which the flake does not have yet, and it legalizes `i64`
  exports, so the generated binding may need its own entry for that route.

Public API: none.

## Required

- [The exporter's web target](/quest/m2/wasm/backend.md) - the binding the new route calls through
- [Iterator adaptors](/quest/m2/ts-iter-adaptors.md) - the spike's verdict on its second largest, and the fix if it earns one
