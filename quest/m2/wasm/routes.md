# [M] Every route on the browser benchmark

## Goal

`just ts bench` ([the TS benchmark](/ts/README.md#bench)) times each workload through two
more routes, in Chrome, Firefox, Node, and Bun: the fixture's `pub` API exported by
[rust2-wasm](/quest/m2/wasm/backend.md), called through its generated binding, and the
same `.wasm` lowered to JavaScript by Binaryen's `wasm2js`. Each route's output must match
native Rust's first, and the report gives each route's shipped bytes, raw and gzipped,
with its `.wasm` or runtime included.

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
- Add moq's `js/net/bench/frames.ts` workload, a group stream read into a group, once the
  model translates; `ts/bench/frames.ts` is the `bytes/` route, not moq's.

Public API: none.

## Required

- [The exporter's web target](/quest/m2/wasm/backend.md) - the binding the new route calls through
