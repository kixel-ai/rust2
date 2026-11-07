# [S] A slice of one byte's place

## Goal

[rust2-ts](/ts/README.md) translates `slice::from_mut` and `array::from_mut` of a
`&mut u8`, of a local, a field, or an element of an array of bytes alike, which it refuses
today: the slice writes the byte where the caller holds it. Proven by differential tests
against native Rust.

## Plan

✅ Decided with the maintainer on 2026-11-06 in
[moq-dev/rust2#608](https://github.com/moq-dev/rust2/pull/608): keep it, deferred to this
quest of its own, as rust2 aims to support all of std. The options were to drop it until
a consumer appears, since moq has none at its pin, or to keep it and measure the
`rs.Place`-backed `rs.ByteSlice` below (picked).

A slice of one over a `&mut u8` was found fragile when it meant a one-byte buffer for each
local; an `rs.ByteSlice` whose storage can be an `rs.Place` may avoid that. Measure what
each form costs a hot `&mut u8`, such as `read_exact(slice::from_mut(&mut byte))`, against
the `rs.ByteSlice` over a `Uint8Array` every other byte slice is.

Public API: the translator's output, and what a hand-written caller may receive as a
`rs.ByteSlice`.

## Related

- [rs.String](/quest/m3/ts-string-capacity.md) - another runtime wrapper over storage JS holds differently
