# [M] Debug of std's iterator adaptors

## Goal

[rust2-ts](/ts/README.md) translates `Debug` of std's iterator adaptors, such as `Map`,
`Filter`, `Enumerate`, `Zip`, `Chain`, `Peekable`, `Rev`, `Take`, `Skip`, and
`char::DecodeUtf16`, writing what std's derived or hand-written impl writes, pretty forms
included, over whatever iterator each wraps. Proven by differential tests against
native Rust.

## Plan

Found in [#507](https://github.com/moq-dev/rust2/pull/507): `Debug` of `DecodeUtf16`
fails as "unsupported call", and so does every adaptor's today, since
`iter_debug` in `src/lower/fmt.rs` covers only collections' iterators and `Chars`.

Each adaptor's `Debug` writes its fields in std's order, the wrapped iterator's own
`Debug` among them, and leaves out a closure as std does (`Map { iter: .. }`); some write
state as they go, as `Peekable` writes what it peeked and `DecodeUtf16` its buffered
unit. Read std's impls rather than guess them, and survey which adaptors share a shape
before choosing how the runtime writes them.

An adaptor over an iterator whose `Debug` has no mapping stays refused, at its span.

Public API: the translator's output, and the runtime functions each needs.
