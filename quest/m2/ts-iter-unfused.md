# [M] Fuse the iterator chains that still run through the runtime

## Goal

`iter/any`, `iter/skip-take-fold`, and `iter/filter-collect` run as one loop over their
leaves read in place, as the fused chains do since
[#831](https://github.com/kixel-ai/rust2/pull/831), so every `iter/` row approaches
hand-written in Chromium and Firefox.

## Plan

Kept by the maintainer on 2026-11-15 ([#831 comment](https://github.com/kixel-ai/rust2/pull/831#issuecomment-6890381752)): ✅ keep (rejected: drop).

On 2026-11-14, with leaves read in place, Chromium ran `iter/any` at 2.01 against wasm's
0.43, `iter/skip-take-fold` at 10.8 against 0.245, and `iter/filter-collect` at 9.8
against 3.16; Firefox ran `iter/any` at 11.6 against 1.13. Each still calls the runtime:

- `any` straight over a slice's iterator: `fuses` in `ts/src/lower/mir/fuse.rs` takes only
  a consumer of an adaptor. Fuse a consumer of a bare leaf where it reads the leaf in place.
- `skip` and `take`: add them as nodes, each with the count std's `Skip` and `Take` keep,
  read from the back as their `nextBack` reads.
- `collect` into a `Vec` of a fused chain: a loop that pushes into the `Vec` the runtime's
  `iterToVec` would make, with its capacity from the chain's size hint.
- Re-run `iter/` through `--route`, and add each shape to `fused` in the `iters` fixture.

Public API: none.
