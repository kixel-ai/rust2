# [M] A vector's exact capacity as it shrinks

## Goal

[rust2-ts](/ts/README.md) holds a `Vec`'s exact capacity through every step, as it now
does a `VecDeque`'s, so a deque made from a `Vec` splits its slices where std's does,
proven by differential tests against native Rust.

## Plan

Found splitting a deque's slices where std's does
([moq-dev/rust2#443](https://github.com/moq-dev/rust2/pull/443)): a deque now holds std's
exact capacity, but one made from a `Vec`, by `From<Vec<T>>` or by `collect`, takes the
capacity the runtime models for the `Vec`, which holds only std's guarantees (decided
2026-11-01) and so recomputes from its length after a pop, `truncate`, or `drain`. A
`Vec<u8>` already holds its exact capacity as its buffer's length.

- Record the capacity where a step shrinks the length, and measure its cost on pops with
  the codec's [TS benchmark](/ts/README.md#bench).
- Needs the maintainer's call against the 2026-11-01 decision.

Public API: the translator's output.
