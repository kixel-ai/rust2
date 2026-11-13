# [S] Spike: wasm's lead when each item crosses

## Goal

A throwaway route on `just ts bench` calls compiled Rust once per item instead of once
per batch, on three workloads (varint, frames, and an async handoff), through the
existing hand-written boundary. Its report says whether wasm's lead over the translation,
about 5 times on compute in the 2026-11-12 nightly, survives a call per item, before
[the backend](/quest/m2/wasm/backend.md) `[L]` is built to find out.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ spike first.

- Today the wasm route is timed with its input already in linear memory and its batch
  copies timed apart, so a per-call crossing shows nowhere.
- Reuse the benchmark's routes (`--route name=dir`); nothing here lands but the report.
  Put the numbers in the PR, in Chrome and Firefox, and one line in
  [the gate](/quest/m2/wasm/README.md)'s Plan saying what they change, if anything, about
  the backend's design or its priority.
- The result may differ from rust2-wasm's generated binding, which is why it is a spike
  and not the measurement.

Public API: none.
