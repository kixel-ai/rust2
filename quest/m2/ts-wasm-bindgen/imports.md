# [XL] Translate wasm-bindgen imports

## Goal

[rust2-ts](/ts/README.md) translates every wasm-bindgen import web-sys and js-sys declare
into a direct JS call, and refuses any other wasm-bindgen attribute, intrinsic, or item at
translation with its span. wasm-bindgen, with wasm-bindgen-futures, is mapped into the
runtime. moq-net's wasm32 clock translates through web-async and wasmtimer, so
`ts/moq/model.toml` drops its three `model::clock` binds, proven by a differential test
against a native wasm32 build.

## Plan

The epic's [Plan](/quest/m2/ts-wasm-bindgen/README.md#plan) holds the settled decisions:
the attribute set, the runtime mapping, and the first consumer. How translation reads each
import's JS shape is open until the [spike](/quest/m2/ts-wasm-bindgen/spike.md) reports.
Its PR rewrites this Plan, and splits the quest if the spike finds the work outgrows one PR.

Docs: [well-known crates](/ts/README.md#well-known-crates) gains wasm-bindgen's mapping
and the import attributes translation supports.

Public API: the translator's output for a crate calling wasm-bindgen imports.

## Required

- [Spike: how rust2-ts reads wasm-bindgen imports](/quest/m2/ts-wasm-bindgen/spike.md) - picks the mechanism and rescopes this quest
