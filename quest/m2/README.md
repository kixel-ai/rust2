# m2: research

## Goal

Research work with a named consumer that is not waiting on it yet: rust2-wasm, moq's
browser route, the TypeScript translator that has to beat it,
[crabx](/quest/m2/crabx/README.md), which runs moq's CLIs without a Rust toolchain, and
[registry publishing](/quest/m2/registry.md), which would replace moq's binding release
workflows.

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

Decided with the maintainer on 2026-11-12, in the quest audit:

- ✅ The translator gets a performance pass before the gate judges it:
  [multi-byte encodes](/quest/m2/ts-encode.md) and
  [iterator adaptors](/quest/m2/ts-iter-adaptors.md), which
  [the gate's measurement](/quest/m2/wasm/routes.md) requires, and
  [the per-call crossing report](https://github.com/kixel-ai/rust2/pull/760), which finds
  no clear varint lead in Chrome but a remaining lead on frames and async. On the 2026-11-12 nightly
  ([run 43246857664](https://github.com/kixel-ai/rust2/actions/runs/43246857664), Chromium)
  the translation ran about 5 times wasm's time on compute, geometric mean over 82 rows.
  Rejected: the spike alone, with fixes frozen behind the gate; judging the translator as
  it stands.
- ✅ [The model epic](/quest/m2/ts-model/README.md) moves to m3 with a `Required` on the
  gate, since once [#722](https://github.com/kixel-ai/rust2/pull/722) merges its only
  content is a wait on moq, which m3 holds. Not moved yet: #722 holds its branch, and
  `quest mv` refuses a claimed quest. Rejected: keeping it in m2.

Decided with the maintainer on 2026-10-26: the translator targets browsers. A native
runtime such as Node or Bun calls Rust through [rust2-ffi](/ffi/README.md), so Node and
Bun matter only as proxies for V8 and JavaScriptCore, and a speed call that splits them
from the browsers goes to the browsers.

The fork's feature quests folded into the [exporter](/quest/m1/ffi-export/README.md);
[m1](/quest/m1/README.md#plan) records why.
