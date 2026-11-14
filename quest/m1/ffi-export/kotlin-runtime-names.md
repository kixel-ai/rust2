# [S] Kotlin runtime helpers take underscore-prefixed names

## Goal

The Kotlin runtime's helper classes (`Reader`, `Handle`, `Ready`, ...) take `_`-prefixed names, so no exported type is renamed with a trailing `_` to dodge them and `TAKEN` in `ffi/src/kotlin.rs` no longer lists them.

## Plan

- ✅ keep (maintainer, 2026-11-13, when landing kixel-ai/rust2#750 with the reservation as is).
- Builds on kixel-ai/rust2#777, which does the same for `_Native` and its `_<Name>` import aliases; follow its naming.
- Rename each helper, drop it from `TAKEN`, and add a fixture exporting a type named like a former helper to prove it keeps its name.

## Required

- [Variant names](/quest/m1/ffi-export/kotlin-variant-names.md) - kixel-ai/rust2#777's `_`-prefixed runtime names this extends
