# [S] A vector keeps the capacity its pushes grew as it shrinks

## Goal

[rust2-ts](/ts/README.md) keeps a `Vec`'s capacity through every step that drops its
length, as std guarantees, when the capacity came only from pushes. That covers `clear`,
`truncate`, `pop`, `drain`, `retain`, `dedup`, `resize` down, `split_off`, `remove`, and
`swap_remove`. Differential tests against native Rust prove it. A measured wrapper
decides how, or whether the divergence is documented instead.

## Plan

Decided with the maintainer on 2026-11-03:

> std documents that `clear`/`truncate` don't change a Vec's capacity, but the
> guarantee-only model reports 0 or 1 afterwards. What should the quest aim for?
>
> - ✅ Try a wrapper first: measure ts-vec-exact-capacity's untried idea, a Vec object
>   holding its own length and capacity over its array, on push/stack and the other TS
>   benchmarks, against the 1.1x-2.8x of peak tracking, before choosing.
> - Fix it and accept the measured cost.
> - Document the divergence and drop the quest.

This conflicts with the 2026-11-02 decision recorded in
[that quest](/quest/m3/ts-vec-exact-capacity.md) to keep only std's capacity guarantees,
since `clear` and `truncate` keeping the capacity is itself documented. So the first step
is a measurement: build the wrapper and time it on `push/stack` and the other TS
benchmarks in Node, Chrome, Bun, and Firefox, against peak tracking's 1.24x, 1.11x, 2.3x,
and 2.8x on `push/stack`. Then bring the numbers back to choose between the wrapper,
fixing it at the measured cost, and documenting the divergence. A wrapper that holds its
own capacity would give std's exact capacity too, so this quest may merge with
[that quest](/quest/m3/ts-vec-exact-capacity.md) once measured.

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

Public API: the translator's output.

## Related

- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - std's exact capacity, beyond its guarantees, so a deque made from a shrunk `Vec` splits its slices where std's does
