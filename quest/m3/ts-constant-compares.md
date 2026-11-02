# [S] A comparison of two constants

## Goal

[rust2-ts](/ts/README.md) translates a comparison whose sides are both constants once a
generic function is monomorphized, such as `size_of::<T>() == 1` in a function generic
over `T`, into TypeScript `tsc` accepts, proven by a fixture that runs it for a `T` of each
outcome against native Rust.

## Plan

Found writing the deque's slices fixture
([moq-dev/rust2#443](https://github.com/moq-dev/rust2/pull/443)): each instance writes
`12 === 1`, which `tsc` rejects as TS2367, two literal types with no overlap, so the
crate fails to translate. Fold the comparison to its value, or widen a side, where both are
literals.

Public API: the translator's output.
