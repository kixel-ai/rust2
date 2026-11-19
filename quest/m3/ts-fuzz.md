# [S] Fuzz the translation against wasm

## Goal

A coverage-guided fuzzer (such as Jazzer.js) drives the codec fixture's `run(&[u8])`
through the translation, with its wasm32 build as the oracle in the same process
(`ts/tests/differential.ts`), and a nightly salt varies the differential tests' per-case
seeds, printing it so a failure reproduces.

## Plan

Decided with the maintainer on 2026-11-18, in the per-language audit: ✅ kept, in m3 under
the gate rule (rejected: dropping it). Coverage over the runtime reaches edges such as the
switch to two-word integers at 2^53.

Public API: none.

## Required

- [rust2-wasm](/quest/m2/wasm/README.md) - the gate decides whether translation continues
