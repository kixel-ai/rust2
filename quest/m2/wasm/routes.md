# [M] The binding benchmark's cases through the translation

## Goal

[The binding benchmark](/bench/ffi/README.md)'s cases, which cross handles, records,
objects, reads, and payloads, run through rust2-ts's translation of the same moq API too,
beside rust2-wasm's TypeScript column there, in Node, Chromium, and Firefox, each case's
result matching the export's first. With [the TS benchmark](/ts/README.md#bench)'s routes,
this is the measurement [the gate](/quest/m2/wasm/README.md) reads.

## Plan

Decided with the maintainer on 2026-11-09: this is the measurement the gate reads.

- [#860](https://github.com/kixel-ai/rust2/pull/860) gave `just ts bench` its rust2-wasm and
  `wasm2js` routes, each route's build time and cold start, and the `wasm-checked` profile,
  so the codec's and the handoff's routes run the same checks. What is left needs moq's
  model translated: the cases call `TrackProducer`, `GroupProducer`, `GroupConsumer`,
  `FrameFrame`, and `Timestamp` (`ffi/moq/bench/export.ts`), which no translation reaches
  until [the model epic](/quest/m2/ts-model/README.md) lands.
- Translate with the model fixture's roots (`ts/moq/model.toml`) and port `export.ts`'s
  cases onto the translation's API, which [the epic](/quest/m2/wasm/README.md)'s 2026-11-12
  decision made the binding's too, so the port renames imports rather than rewriting calls.
- Report the translation as one more column in `bench/ffi/report.py`'s TypeScript table.
  The translation keeps the overflow checks the export's release build drops, so the
  report says which checks each column runs, as `just ts bench`'s does.

Public API: none.

## Required

- [Translate moq-net's model](/quest/m2/ts-model/README.md) - the translation reaches the handles, groups, and frames the cases call
