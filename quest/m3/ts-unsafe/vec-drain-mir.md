# [M] Vec::drain from std's MIR

## Goal

[rust2-ts](/ts/README.md) translates `Vec::drain` from std's MIR, whose `Drain` reads its
range out and moves the tail back through raw pointers into the `Vec`'s storage,
replacing `rs.vecDrain` and `rs.byteVecDrain`, the runtime port that
[the slice gaps' PR](https://github.com/moq-dev/rust2/pull/259) mapped it to
(`ts/src/lower/library.rs`). The lang fixture's `slices::drains` differential case proves
it still matches std's items, drops, and what a forgotten drain leaves.

## Plan

moq calls it (`client.rs:821`, `priority.rs:216`) and the port works, so this is fidelity
to std rather than a blocker: the port must track std by hand. The B-tree port of
[#577](https://github.com/moq-dev/rust2/pull/577) is the precedent. Measure the TS
bench's drain-heavy workloads against the port.

Public API: the translator's output.

## Required

- [A Vec's spare capacity](/quest/m3/ts-unsafe/vec-spare-capacity.md) - std's `Drain` calls `set_len`
