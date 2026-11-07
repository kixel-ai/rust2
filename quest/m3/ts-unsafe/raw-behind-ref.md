# [M] A raw pointer behind a reference

## Goal

[rust2-ts](/ts/README.md) translates `&raw const *r` and `ptr::read(r)` where `r` is a `&T`
to a value JS copies or a library object, so the pointer is where the reference came
from, proven by differential tests against native Rust. Today it is refused: ``a raw
pointer to `T` behind a reference, which is the object and not where it is kept``
(`ts/src/lower/mir/rvalue.rs`).

## Plan

The reference is the value, not where it is kept, so the lowering needs the place the
reference was made from. Since [#576](https://github.com/moq-dev/rust2/pull/576) and
[#584](https://github.com/moq-dev/rust2/pull/584), a slice's reference knows its
`{data, start}`, which is that place for elements.

At the moq pin it refuses 6 times in the model and 8 in moq-net, all smallvec's `Drain`
(`lib.rs:383`, `397`) and `retain` (`1733`, `1740`), plus moq-net's own
`ietf/parameters.rs:14,40` num_enum tag reads, which are also byte reinterpretation and
stay refused.

Public API: the translator's output.
