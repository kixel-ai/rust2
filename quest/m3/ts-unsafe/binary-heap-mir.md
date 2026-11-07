# [M] BinaryHeap from std's MIR

## Goal

[rust2-ts](/ts/README.md) translates `BinaryHeap` and `PeekMut` from std's MIR, whose
`Hole` moves elements through raw pointers into the `Vec`'s storage, replacing
`rs.BinaryHeap` and `rs.PeekMut`, the hand-written port of std's sifts that
[#252](https://github.com/moq-dev/rust2/pull/252) mapped them to
(`ts/src/lower/collections.rs`). The lang fixture's `orders` differential case proves it
still matches std's comparisons, pop order, and drops.

## Plan

No consumer at the moq pin; the port works but must track std by hand. The B-tree port of
[#577](https://github.com/moq-dev/rust2/pull/577) is the precedent.

Public API: the translator's output.

## Required

- [A Vec's spare capacity](/quest/m3/ts-unsafe/vec-spare-capacity.md) - std's `PeekMut` calls `set_len`
