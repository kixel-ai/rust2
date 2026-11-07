# [M] A raw pointer into a nested array

## Goal

[rust2-ts](/ts/README.md) translates a raw pointer to an element of a nested array, such as
`&raw mut (*p)[1][0]`, today refused as not supported yet. It points at that element in
its inner array, so reads, writes, and arithmetic within the inner array match native Rust,
proven by differential tests.

## Plan

Suggested on [#639](https://github.com/moq-dev/rust2/pull/639). An element pointer is
`rs.AtIndex`, `{ array, index }`, into the array JS holds it in; for a nested array that is
the inner array. Arithmetic that crosses from one inner array into the next, which Rust
allows within the outer allocation, stays refused with a span unless the plan finds a
cheap exact way; say which in the PR.

Decided with the maintainer on 2026-11-07: ✅ m3, in this epic (from #639's follow-ups), ✅
arithmetic within the inner array (rejected: crossing inner arrays as a requirement). No
consumer at the moq pin.

Public API: the translator's output.
