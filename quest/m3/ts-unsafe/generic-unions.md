# [M] Unions over a type parameter

## Goal

[rust2-ts](/ts/README.md) translates a union whose fields' layout depends on a type
parameter, such as smallvec's `SmallVecData`, by laying out each instance rather than
the generic definition, proven by differential tests against native Rust. Today it is
refused: ``a union field of `_`, whose layout is not known: the type `A` does not have a
fixed layout`` (`ts/src/lower/unions.rs`).

## Plan

Unions whose overlapping fields share a type translate since
[#201](https://github.com/moq-dev/rust2/pull/201); an instance fixes the offsets the
generic definition lacks. Smallvec (`lib.rs:840`) is the only hit at the moq pin, once
each in the model, kio, and moq-net.

Public API: the translator's output.
