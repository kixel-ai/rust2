# [S] Debug of std's other iterators

## Goal

[rust2-ts](/ts/README.md) translates `Debug` of std's iterators that still have no
mapping, so an adaptor over one writes what std's does instead of being refused:
iterator sources such as `Once`, `Repeat`, `RepeatN`, `FromFn`, `Successors`,
`OnceWith`, and `RepeatWith`, an array's and an `Option`'s or `Result`'s iterators, and
`str`'s iterators other than `Chars` and `Bytes`. Proven by differential tests against
native Rust.

## Plan

Found while translating `Debug` of the adaptors: `char::decode_utf16([..])` refuses at
its span, as `Debug` of `array::IntoIter` has no mapping. Read each of std's impls, which
write private fields of their own (`Once { inner: IntoIter { inner: Item { opt } } }`),
and survey which share a shape before choosing how the runtime writes them, as
`iter_debug` and `adaptor_debug` in `src/lower/fmt.rs` do.
