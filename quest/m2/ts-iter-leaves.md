# [M] Read a fused chain's slice and range leaves in place

## Goal

A fused chain's slice, `Vec`, and integer range leaves are read in its loop as an index
and a bound, not through their `rs.Iter`'s `next` and `nextBack`, so `iter/` chains
approach hand-written in Chromium and Firefox.

## Plan

The fused readers ([#814](https://github.com/kixel-ai/rust2/pull/814)) keep each leaf an
`rs.Iter`. On 2026-11-14 `iter/zip-rev-filter-count` ran 11.7 in Chromium against 2.98
hand-written, and `iter/zip-filter-count` 7.08 against 0.50: each item calls the leaf's
method, and `Range`'s `next` and `nextBack` step through the generic `step` and `lt`,
which check for wide integers on each call.

- Bind a leaf `iterSlice` or `vecIntoIter` makes as its array, front, and back, and a
  `Range` of an integer no wider than 2^53 as its start and end, in `bind` in
  `ts/src/lower/mir/fuse.rs`; read, trim, and measure them as `SliceIter` and `Range` do.
- Keep a leaf whose items drop, or a wide range, an `rs.Iter`.
- Re-run `iter/` through `--route`.

Public API: none.
