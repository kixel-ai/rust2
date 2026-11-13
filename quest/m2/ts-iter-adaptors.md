# [L] Lower iterator adaptors per chain

## Goal

Generated iterator chains specialize `map`, `filter`, `zip` and `enumerate`
adaptors while leaving leaf iterators in the runtime, preserving Rust semantics
and approaching hand-written performance without runtime dispatch per adaptor.

## Plan

The spike in `bench/iter-adaptors/README.md` earned a go on 2026-11-13:
`zip-map-filter-count` reached 3.41 ns/byte in Chromium versus 3.45 hand-written,
and 5.84 in Firefox versus 3.32 hand-written. One specialized chain added 25 raw
and 43 gzip bytes to the full codec bundle. The profile does not establish
megamorphic map dispatch as the cause; fusion also removes tuple allocation.

- Specialize adaptor code at each chain while retaining runtime leaf iterators.
  Do not lower core's pointer-based slice iterator MIR. Follow the existing
  Option/Result combinator lowering where useful.
- Cover all supported ways a chain is consumed, including escaped iterators,
  reverse traversal, `nth`, folds, short circuiting, exact lengths and size hints.
  Preserve closure call order, integer overflow and item/closure drops. Reuse the
  current runtime for chains whose specialization cannot preserve these rules.
- Add differential fixtures against native Rust for primitive and owned items,
  non-fused and unequal-length sources, side-effecting closures, early exits,
  forward/backward interleaving, and unwind/drop paths supported by translation.
- Re-run the iterator family through `--route` in Chromium and Firefox and
  measure bundle growth across multiple specialized chains. Include enumerate,
  which the spike did not specialize. Keep the existing benchmark checks and
  watchdog enabled. Benchmark-owned headless browser launches are authorized;
  agent-browser-only instructions apply to manual/web browser tasks.

Public API: none. Sized L for general consumption and ownership semantics;
the finished spike was S.
