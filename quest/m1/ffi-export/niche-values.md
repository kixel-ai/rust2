# [M] Cross a value whose niche fills its bytes as an opaque value

## Goal

A `Copy` type with private fields whose every value initializes all its bytes crosses as an
opaque value, copied across, even when a field is an enum whose dataless variant is encoded
in another variant's niche, such as `Option<NonZero<u64>>`. moq's `Timestamp`, `{ value:
u64, scale: Option<Timescale> }` at moq `1aa86d5e6`, crosses as its 16 bytes again, and the
binding benchmark's stamped write is back to the cost it had as a value.

An enum whose dataless variant leaves bytes uninitialized, such as `Option<u32>` or
`Option<(u32, NonZero<u32>)>`, whose `None` writes only the niche's 4 bytes, stays an
object.

## Plan

`full` in `ffi/src/collect.rs` takes an enum's fewest initialized bytes as its smallest
variant's fields, so `None` of `Option<NonZero<u64>>` counts as 0 bytes and `Timestamp`
falls back to an object. With a niche, though, rustc writes a dataless variant as the
niche field's invalid value, so it initializes that field's bytes. Count them: a dataless
variant initializes exactly the niche field's range, and the enum is full only when every
variant, so counted, fills the layout under every field order and at both model widths,
as `full` already requires of a struct. Take the niche's offset and size from rustc's
layout (`largest_niche`), and keep the model's 4- and 8-byte-pointer check on it.

Prove it beside `full`'s existing cases: a fixture type with an `Option<NonZero<u64>>`
field exports as an opaque value, and the reject fixtures keep `Option<u32>` and a niche
narrower than its payload as objects; a Miri run of the api fixture's C ABI copies a
`None` without reading uninitialized bytes.

Found by [kixel-ai/rust2#881](https://github.com/kixel-ai/rust2/pull/881), the bump to moq
`1aa86d5e6`, where moq's `Timestamp` gained `Option<Timescale>` for unscaled group keys and
became a handle: its one-round benchmark there puts the stamped write at 208 ns against
173 ns with one reused timestamp in C (35 ns a timestamp), and the bench README's 2026-11-10
measurements give the handle's cost in each language (21 ns in C up to 260 in Go).
