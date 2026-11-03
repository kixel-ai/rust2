# [M] A string's exact capacity

## Goal

[rust2-ts](/ts/README.md) holds a `String`'s exact capacity as std's does, through
`with_capacity`, `reserve`, `reserve_exact`, push growth, `shrink_to_fit`, `shrink_to`,
`format!`'s estimate, and `into_bytes`, `Vec::from`, and `Bytes::from`, which take it.
Differential tests against native Rust prove it, and the refusal below is lifted.

## Plan

Decided with the maintainer on 2026-11-03
([moq-dev/rust2#532](https://github.com/moq-dev/rust2/pull/532)):

> A `Bytes` of a `String`'s bytes is unique, and truncates to nothing, by the `String`'s
> capacity, which JS keeps none of. How should what reads it behave?
>
> - Throw a `rust2-ts:` error at runtime.
> - Refuse `From<String> for Bytes` at translation.
> - Treat a `String`'s capacity as its length.
> - ✅ Track String capacity, as its own quest. Until then, refuse at translation.

A `String` is a JS string, so `capacity` is refused and `with_capacity`, `reserve`, and
`shrink_to_fit` do nothing. A `Vec<u8>` or a `Bytes` of its bytes gets a buffer as long
as its UTF-8, whose capacity std's differs from after `with_capacity`, `format!`, or a
push that grew it. So where any translated crate makes a `Bytes` or a `Vec<u8>` of a
`String`'s bytes, every crate is refused what reads a `Bytes`' capacity (`is_unique`,
`try_into_mut`, `truncate`, `clear`, `From` into a `BytesMut` or a `Vec<u8>`, and
`BufMut::put` into a `BytesMut`), since the bytes may reach any of them. Lifting that
needs the capacity on the `String` itself.

Measure first: a `String` as a wrapper object holding its text and capacity, and a side
table, such as a `WeakMap` from an object, against plain JS strings, on `just ts bench`
and the moq codec in Node, Chrome, Bun, and Firefox, before choosing. A string's text is
a primitive, so a side table needs an object to key on, and a wrapper changes every
string operation's output; weigh both against how few strings ever read a capacity.
[The pushed capacity quest](/quest/m3/ts-vec-clear-capacity.md) measures a `Vec`
wrapper the same way, so share the harness and compare the two.

Public API: the translator's output.

## Related

- [A vector keeps the capacity its pushes grew as it shrinks](/quest/m3/ts-vec-clear-capacity.md) - measures a `Vec` wrapper holding its own capacity
- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - a `Vec`'s exact capacity, which a `String`'s bytes become
