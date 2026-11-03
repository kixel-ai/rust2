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

- Recording the capacity on the array where a step shrinks the length (a `peak` each pop
  raises) was measured on `push/stack`, which pushes onto a `Vec`, popping one every
  fourth push, then pops it empty: 1.24x in Node, 1.11x in Chrome, 2.3x in Bun, and 2.8x
  in Firefox, every other workload within noise. The maintainer kept the guarantee-only
  model on 2026-11-02, accepting that a deque made from a shrunk `Vec` may split
  differently from std's.
- Untried: a `Vec` as a wrapper object holding its own length and capacity over its
  array, measured on the same workload before it is weighed against that decision.

Public API: the translator's output.

## Related

- [A vector keeps the capacity its pushes grew as it shrinks](/quest/m3/ts-vec-clear-capacity.md) - the capacity std guarantees as the length drops, which the guarantee-only model must keep
