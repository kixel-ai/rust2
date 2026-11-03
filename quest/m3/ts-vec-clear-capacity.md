# [S] A vector keeps the capacity its pushes grew as it shrinks

## Goal

[rust2-ts](/ts/README.md) keeps a `Vec`'s capacity through every step that drops its
length, as std guarantees, when the capacity came only from pushes. That covers `clear`,
`truncate`, `pop`, `drain`, `retain`, `dedup`, `resize` down, `split_off`, `remove`, and
`swap_remove`. Differential tests against native Rust prove it.

## Plan

std documents that `clear` and `truncate` leave the capacity unchanged, and that a `Vec`
never shrinks on its own. So the guarantee-only model the maintainer chose on 2026-11-01
must keep this capacity. It is a guarantee, not the exact capacity
[that quest](/quest/m3/ts-vec-exact-capacity.md) leaves open.

A `Vec` whose capacity is implicit, grown by pushes or made exact by `vecExact`, reads it
back through `capOf` from its current length, so the capacity falls with the length.
Two paths already keep it: [moq-dev/rust2#522](https://github.com/moq-dev/rust2/pull/522)
fixed a `Vec` made from a deque's buffer, and
[moq-dev/rust2#527](https://github.com/moq-dev/rust2/pull/527) fixed `clone_from`, which
records the capacity before its own truncate. `rt.vecTruncate` and the other steps still
lose it.

Reproduction on #527's head: a `Vec<[u64; 160]>` gets six pushes, then one step, then a
read of `capacity()`. Native Rust reports 8 after every step. TS reports:

| step | TS |
| --- | --- |
| `clear`, `pop` until empty, `drain(..)`, `retain` of none, `resize(0, _)` | 0 |
| `truncate(1)`, `dedup` to one | 1 |
| `split_off(2)` | 2 |
| `remove`, `swap_remove` | 8, only because five elements still imply 8 |
| `clone_from` of one element | 8 |

The `capacity` fixture misses this because its `Known` checks capacity only against a
floor that `with_capacity` or `reserve` set, and those already hold their capacity. Its
cases need a check that the capacity never drops below what it was before a step that
shortens the vector.

Recommendation: record the implicit capacity once, when a step first drops the length
below it, so later steps grow from it. Measure on `push/stack` across engines first. A
record made on every pop cost 1.24x to 2.8x there in that quest's measurement.

Public API: the translator's output.

## Related

- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - std's exact capacity, beyond its guarantees, so a deque made from a shrunk `Vec` splits its slices where std's does
- `quest/m3/ts-clone-from-capacity.md`, added in [moq-dev/rust2#527](https://github.com/moq-dev/rust2/pull/527) - the capacity of a `Vec` that `clone_from` writes over inside a container; link it by path once #527 lands
