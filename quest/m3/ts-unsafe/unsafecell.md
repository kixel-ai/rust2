# [XS] UnsafeCell's raw accessors

## Goal

[rust2-ts](/ts/README.md) translates `UnsafeCell::raw_get`, `UnsafeCell::from_mut`, and
`UnsafeCell::replace`, proven by differential tests against native Rust. Today each is
refused as an unsupported call, and `raw_get(&c)` also as a raw pointer behind a
reference.

## Plan

`UnsafeCell::get` already lends the cell where it is an `rs.Place`. No consumer at the moq
pin.

Public API: the translator's output.
