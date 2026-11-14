# [M] Fuse iterator chains read from the back

## Goal

A chain of `map`, `filter`, `enumerate`, and `zip` that is reversed, or read by
`next_back` or `rfold`, fuses into its consumer as a forward chain does, so
`iter/zip-rev-filter-count` approaches hand-written in Chromium and Firefox.

## Plan

[#779](https://github.com/kixel-ai/rust2/pull/779) fuses chains read forward and leaves
`rev` of an adaptor to the runtime; a `rev` of a leaf already fuses as a leaf. On
2026-11-13 `iter/zip-rev-filter-count` ran 18.7 ns per byte in Chromium against 1.34
hand-written, and 34.2 in Firefox against 1.26.

- Add a back reader per node in `ts/src/lower/mir/fuse.rs`: a leaf's `nextBack`, a
  `map`'s closure of its inner's, a `filter`'s `rfind`, an `enumerate`'s index counted down
  from its length as the read starts, and a `zip`'s std `next_back`: by index from the back
  for `TrustedRandomAccess` sides, first trimming the longer side to the shorter, reading
  each trimmed item only from a side that may have side effects, as `TrustedZipIter`'s
  `nextBack` and `trim` do in `ts/runtime/rs/iter.ts`.
- Match the runtime's `RevIter` readers: `folded` passes through, `find` is the inner's
  `rfind`, and `fold` the inner's `rfold`.
- Add differential cases to `iters::fused` for each node reversed, unequal `zip` sides
  with effects, and early exits; re-run `iter/` through `--route`.

Public API: none.
