# m2: research

## Goal

Research work with a named consumer that is not waiting on it yet: rust2-wasm, moq's
browser route, and the TypeScript translator that has to beat it.

## Plan

Decided with the maintainer on 2026-10-23: the translator measures before it
builds. The benchmark and census come first; each spike's report rescopes or
deletes the quests that follow it.

Decided with the maintainer on 2026-10-24: the translator's output ranks correctness,
then speed, then the size of the consumer's final tree-shaken, minified JS; readability
is not a goal, and types must be solid at the API boundary ([design](/ts/README.md#design)).

Decided with the maintainer on 2026-11-09, in the quest audit: WASM ships, and the
translator must beat it. [rust2-wasm](/quest/m2/wasm/README.md) is the browser route and
holds the gate; until it reports, translator work is only what the gate needs.

Decided with the maintainer on 2026-10-26: the translator targets browsers. A native
runtime such as Node or Bun calls Rust through [rust2-ffi](/ffi/README.md), so Node and
Bun matter only as proxies for V8 and JavaScriptCore, and a speed call that splits them
from the browsers goes to the browsers.

The fork's feature quests folded into the [exporter](/quest/m1/ffi-export/README.md);
[m1](/quest/m1/README.md#plan) records why.

## Required

- [rust2-wasm](/quest/m2/wasm/README.md) - a crate's `pub` API in the browser through the exporter's wasm32 target, and the gate the translator has to pass
- [Translate moq-net's model to TypeScript](/quest/m2/ts-model/README.md) - the model translates with kio and moq-pattern, for the gate to compare
- [Waiting: Swift 6.4 in nixpkgs-unstable](/quest/m2/uniffi-swift-arm.md) - the nightly UniFFI fixtures move back to arm
