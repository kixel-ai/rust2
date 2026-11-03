# [S] A vector keeps the capacity its pushes grew as it shrinks

## Goal

[rust2-ts](/ts/README.md) keeps a `Vec`'s capacity through `clear`, `truncate`, `pop`, and
`drain` when the capacity came only from pushes, as std guarantees: `clear` and `truncate`
document that they leave the capacity unchanged, and a `Vec` never shrinks on its own.
Differential tests against native Rust prove it.

## Plan

Found in [moq-dev/rust2#522](https://github.com/moq-dev/rust2/pull/522), whose fix covers
only a `Vec` made from a deque's buffer. A `Vec` whose capacity is left implicit, grown by
pushes or made exact by `vecExact`, reads it back through `capOf` from its current length,
so the capacity falls with the length. Six pushes of 1280-byte elements, then `clear`:
native Rust reports a capacity of 8, TS reports 0.

The guarantee-only model the maintainer chose on 2026-11-01 still has to keep this, since
it is one of std's guarantees, not the exact capacity
[that quest](/quest/m3/ts-vec-exact-capacity.md) leaves open. The `capacity` fixture
misses it because its `Known` checks capacity only against a floor that `with_capacity` or
`reserve` set, and those already hold their capacity. Its cases need a check that the
capacity never drops below what it was before a step that shortens the vector.

Recommendation: record the implicit capacity once, when a step first drops the length
below it, so later steps grow from it. Measure on `push/stack` across engines first. A
record made on every pop cost 1.24x to 2.8x there in that quest's measurement.

Public API: the translator's output.

## Related

- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - std's exact capacity, beyond its guarantees, so a deque made from a shrunk `Vec` splits its slices where std's does
