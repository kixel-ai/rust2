# [M] Read a fused chain's items in a for loop as parts

## Goal

A `for` loop over a fused iterator chain reads each item's parts where its body reads
them, with no tuple made per item and one end-of-chain test, so `iter/enumerate-for`
approaches hand-written in Chromium and Firefox.

## Plan

After [#779](https://github.com/kixel-ai/rust2/pull/779) a `for` loop over a fused chain
still reads each item through `fused_next` in `ts/src/lower/mir/fuse.rs`, which makes the
whole `Option<(usize, &u8)>` item, an array for an `enumerate`'s or a `zip`'s pair, and
`rs.DONE` at the end, which the loop's own `match` then tests again before its body reads
the fields. On 2026-11-13 `iter/enumerate-for` ran 6.44 ns per byte in Chromium against
2.40 hand-written, and 13.7 in Firefox against 4.18 (base 10.5 and 19.3).

- Let the `Some` arm's field reads of the item local, `_n[0]` and `_n[1]`, take the parts
  `read` already holds, as the stepped iterators in `ts/src/lower/mir/step.rs` keep their
  items unmade, and branch to the loop's exit where the chain ends instead of testing
  `rs.DONE` twice.
- Inline the chain's one-expression closures there too, as the consumer loop does, once the
  loop's reads and the chain's state are in one scope.
- Keep the differential fixture `iters::fused` covering early `break`, a `zip` whose
  second side ends first, and closures with effects; re-run `iter/` through `--route`.

Public API: none.

The maintainer settled on 2026-11-13 to ✅ keep this quest (rejected: drop), and that the
rust2-wasm gate's measurement in `quest/m2/wasm/routes.md` ✅ doesn't wait for it, reporting
the `iter/zip-rev-filter-count` row as the runtime's (rejected: wait for both follow-ups).
