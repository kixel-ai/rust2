# [S] A slice of one byte behind any &mut

## Goal

[rust2-ts](/ts/README.md) translates `slice::from_mut` and `array::from_mut` of every
`&mut u8`, proven by a differential test against native Rust that writes the byte through
the slice and reads it where the caller holds it: a local, a struct's field, and an
element of a `Vec<u8>` or a `[u8; N]`, each also through a parameter.

## Plan

Found writing [a slice of one object](https://github.com/moq-dev/rust2/pull/502): both
are still refused for a `u8`. A `&mut [u8]` is an `rt.ByteSlice`, which can only view a
`Uint8Array`, while a `&mut u8` is an `rt.Place` that reads and writes a JS number
wherever it is, through closures even into bytes. Translated code writes a byte slice's
`Uint8Array` directly, so nothing can carry the slice's writes back to a local or a field.

Open question: how a `&mut u8` knows the bytes it may point into. A place into bytes
could be an `rt.AtIndex` of the `Uint8Array`, which a slice views as `ptrByteSlice` does;
a local or a field still holds a number, so it would need a `Uint8Array` of its own
wherever its `&mut` may reach such a slice. `read_exact(slice::from_mut(&mut byte))` of a
local is the common use; measure what each costs a hot `&mut u8`.

Public API: the translator's output.
