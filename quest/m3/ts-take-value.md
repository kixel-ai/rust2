# [XS] mem::take as a function value

## Goal

[rust2-ts](/ts/README.md) translates `std::mem::take`, `replace`, and `swap` passed as a
function value, such as `v.iter_mut().map(std::mem::take)`, as their calls do, proven by a
differential test against native Rust.

## Plan

Found writing [`dyn Iterator`](https://github.com/moq-dev/rust2/pull/437): the call maps,
but the function value is an "unsupported call to `std::mem::take::<Vec<u8>>`".

Public API: the translator's output.
