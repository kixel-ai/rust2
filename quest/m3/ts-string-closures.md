# [S] A closure that drops given to a string's edit or search

## Goal

[rust2-ts](/ts/README.md) translates a closure that owns a value that drops given to
`String::retain`, or to `str`'s searches that map as a `Pattern`, such as `contains` and
`split`, dropping it where std's does, proven by the task fixture's differential tests
against native Rust.

## Plan

Found beside [drop order](/quest/m3/ts-drop-order.md) in
[#438](https://github.com/moq-dev/rust2/pull/438). The reject fixture's `string_retained`
and `pattern_closure` reproduce them. `String::retain` borrows the string and drops its
closure as it returns. A search that returns an iterator, such as `split`, holds the
pattern until the iterator drops; one that returns a value drops it before returning.

Public API: the translator's output.
