# [M] Opaque padding from the rustc the glue builds with

## Goal

A [rust2-ffi](/ffi/README.md) export zeroes an opaque value's padding from a table only
when the table describes the layout the glue is compiled with. Today `collect.rs` reads
each target's padding from the exporter's own nightly `rustc`, while the glue builds with
the `rustc` cargo resolved, and Rust promises no `repr(Rust)` layout across compilers. A
compiler that orders a type's fields differently makes `lower_<T>` zero a live field on
every write, silently. That mismatch fails the glue's build instead.

## Plan

- The maintainer settled on 2026-11-13: ✅ keep the quest (rejected: drop). How the table
  is tied to the glue's layout: ✅ decide later (rejected for now: pin the glue's `rustc`;
  padding-free types only). The decision stays under Waiting.
- First step, before the maintainer picks: measure which of moq's opaque values have
  padding, and report it on the decision.
- Pinning would make every consumer build the glue with rust2's nightly, which today only
  the exporter needs.
- The glue can't name a private field, so it can't check an offset itself; the table's
  provenance has to be pinned instead.
- Recommended: the export records the commit hash of the `rustc` cargo resolved, the one
  it already runs for a build script's probe, beside each table, and the glue's build
  script refuses any other `rustc` with an error naming both. The table itself then has
  to come from that `rustc`'s layout, such as its `-Zprint-type-sizes` field offsets under
  `RUSTC_BOOTSTRAP`, rather than the exporter's nightly.
- Prove it with a test that builds the glue with a `rustc` whose hash differs from the
  recorded one and expects the refusal, and the api fixture's `Gap` padding test still
  passing with the matching one.
- The API lock leaves padding out: it is no part of what a binding's caller compiles
  against, so a change of it is no break.

## Waiting

- Maintainer: how the table is tied to the glue's layout. Options: pin the glue's `rustc`
  to the one whose layout the export read (recommended: exact, and fails loudly at
  build); compare only each target's size and alignment exactly (rejected: misses a
  reorder of the same size); stop zeroing padding (rejected: stale Rust bytes reach
  languages that serialize a value); cross as raw bytes only types whose leaf field sizes
  add up to their size, so no padding exists under any field order, assert that size in
  the glue, and cross padded types as objects (handles): sound on any `rustc` with no
  table and no pin, but padded types such as `Instant` pay a handle. Measure moq's padded
  opaque values first; the maintainer picks after. An agent cannot pick, since pinning the glue's
  `rustc` constrains every consumer's toolchain.
