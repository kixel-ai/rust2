# [XS] The exporter pins moq on its own

## Goal

[rust2-ffi](/ffi/README.md)'s moq recipes read a moq revision of their own, so the
exporter's pin moves without rust2-ts's. Today `ffi/justfile`'s `moq_rev` evaluates
`ts/justfile`'s, which reads it from `ts/fixtures/model/Cargo.toml`, so
[the bump](/quest/m1/ffi-refusals/moq-pin.md) waits on moq building for wasm32, which
only the translator needs.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ two pins. Rejected: one
pin for both.

- `ffi/justfile` holds the 40-character revision itself, starting at today's shared one,
  so nothing else changes in this PR. The binding benchmark and `go/justfile`'s bench
  recipe follow whichever pin their recipe already reads; say which in the PR.
- This PR removes the bump's `Waiting` item on `rind` building for wasm32, and moves that
  wait to wherever rust2-ts's own bump is planned, or records that none is.

Public API: none.
