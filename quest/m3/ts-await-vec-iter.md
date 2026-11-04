# [S] A vector's iterator held across an await

## Goal

[rust2-ts](/ts/README.md) translates an `async` block that holds a `vec::IntoIter` across
an `.await`, such as `for n in v { f(n).await }`, proven by a differential test against
native Rust in the lang fixture's `futures`.

## Plan

Found by [#550](https://github.com/moq-dev/rust2/pull/550). Translation emits TS that
`tsc` rejects: the future's class types the saved iterator as `rt.Iter<u8>`, but the
value stored is an object literal with an `s` field, so the driver reports
`Object literal may only specify known properties, and 's' does not exist in type
'Iter<u8>'` at the `for`. A slice's `iter()` held the same way translates, so the
coroutine's field likely takes the iterator's type from one lowering and its value from
another.

Public API: the translator's output.
