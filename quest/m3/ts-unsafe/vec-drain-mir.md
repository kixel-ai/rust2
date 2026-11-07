# [M] Vec::drain from std's MIR

## Goal

[rust2-ts](/ts/README.md) translates `Vec::drain` from std's MIR, whose `Drain` reads its
range out and moves the tail back through raw pointers into the `Vec`'s storage,
replacing `rs.vecDrain` and `rs.byteVecDrain`, the runtime port that
[the slice gaps' PR](https://github.com/moq-dev/rust2/pull/259) mapped it to
(`ts/src/lower/library.rs`). The lang fixture's `slices::drains` differential case proves
it still matches std's items, drops, and what a forgotten drain leaves.

## Plan

moq calls it (`priority.rs:216`, a `Vec<Waker>`'s full drain; `client.rs:821` drains a
`VecDeque`) and the port works, so this is fidelity to std rather than a blocker: the
port must track std by hand. The B-tree port of
[#577](https://github.com/moq-dev/rust2/pull/577) is the precedent.

A spike in [#636](https://github.com/moq-dev/rust2/pull/636), on branch
`spike/vec-drain-mir`, translated `alloc::vec::drain` and `Vec::drain` as the crate's own
code. Over elements JS copies (`u8`, `u16`, `u32`), it passes `slices::drains` against
native once [#621](https://github.com/moq-dev/rust2/pull/621) lands. It also needed:
`Global` as nothing, a mapping of `slice::range`, `slice::Iter`'s `as_slice` and `Default`,
`<*mut T>::cast_slice`, `NonNull<Vec<T>>::as_mut`, and an own iterator's `is_empty`.
It ran 3-12x slower than the port by CPU time per `slices::drains` call in Node: `set_len`
moves the range and the tail into the `Vec`'s room, which std's code then reads back
through a `Proxy` view and copies back one element at a time.

Over elements JS keeps as objects, such as the fixture's `Item` or a `Waker`,
`as_ptr().add(start)`, `offset_from`, and `slice::from_raw_parts` are refused: a pointer
to such an element is that object, which knows no array it is in
([the epic](/quest/m3/ts-unsafe/README.md), decided 2026-11-01).

Public API: the translator's output.

## Waiting

- Maintainer: whether std's `Drain` is still wanted, given it needs pointers into an array
  of objects and runs 3-12x slower. Options: drop this quest and keep the port, which
  `slices::drains` tests against native (recommended); plan a pointer model where a
  pointer into an array of objects is an `rs.AtIndex`, plus a room that keeps the slots
  past the length in the array itself, and block this quest on both; or translate it only
  for elements JS copies, which keeps the port for the rest (not recommended: two
  implementations, one still tracking std by hand). Check: this item is resolved here.
