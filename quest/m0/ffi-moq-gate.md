# [M] The moq export gates exporter merges

## Goal

A change to [rust2-ffi](/ffi/README.md) cannot merge while it breaks moq's export: the
`ffi` module of `just ci` runs a cut-down `just ffi moq`, as the `ts` module already runs
`just ts moq` before its tests (`sh/dispatch.sh`'s `test_ts`). The PR says how long it
adds to the gate, warm and cold.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ in m0. Today the
exporter's check on its first consumer runs only nightly, where it was red on 2026-11-12,
while the experiment's runs on every merge.

- The nightly's full export step takes about 13m40s, too slow for the gate. Find the
  cheapest step that still fails on a real break. Recommended: export the stand-in facade
  to C, `cargo check` the generated crate, and compile the header and `ffi/moq/moq.c`
  against it without linking the library. The other languages' `ffi/moq/moq.*` stay
  nightly.
- Measure on the queue runner before choosing, and report the gate's time with and
  without it.

Public API: none.
