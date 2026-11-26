# [M] Translate random MIR programs

## Goal

[Rustlantis](https://dl.acm.org/doi/10.1145/3689780) generates random MIR programs nightly;
each one rust2-ts translates must print what native Rust prints, and a refused program is
filtered out. A report lists which MIR constructs the differential fixtures cover.

## Plan

Decided with the maintainer on 2026-11-18, in the per-language audit: ✅ kept, in m3 under
the gate rule (rejected: dropping it). It tests the control-flow structuring and temporary
folding `driver/src/cfg.rs` shares with rust2-go, which `go/fixtures/order` guards by hand.

Public API: none.

## Required

- [rust2-wasm](/quest/m2/wasm/README.md) - the gate decides whether translation continues
