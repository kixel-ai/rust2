# [S] Option fields of a generic type at a unit instance

## Goal

[rust2-ts](/ts/README.md) translates a generic type's `Option<T>` or `Poll<T>` field at an
instance whose `T` is itself `undefined` or pending in JS, such as moq-net's `Idle<()>`
(`util/idle.rs`'s `parked`) and tokio's `oneshot::Inner<()>`, proven by differential
tests against native Rust. At the moq pin it refuses 16 times in the model and 24 in
moq-net whole.

## Plan

The type's definition types the field `T | undefined`, which loses `Some(())` against
`None` at `T = ()`; the field needs `rs.None` (or `rs.OuterPending`) where an instance
can already be the bare value. Decide it per field from the instances the crate uses, not
per type, and keep the bare form for every other instance.

Public API: the translator's output.
