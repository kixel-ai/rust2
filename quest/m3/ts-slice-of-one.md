# [S] Waiting: a slice of one byte behind any &mut

## Goal

Condition: a consumer calls `slice::from_mut` or `array::from_mut` of a `&mut u8`, which
[rust2-ts](/ts/README.md) refuses with a span until then. Check: `just ts census` of the
consumer, or its sources for either call over a byte. Then both translate for every
`&mut u8` that consumer passes, proven by a differential test against native Rust that
writes the byte through the slice and reads it where the caller holds it.

## Plan

Decided with the maintainer on 2026-11-03: wait for a consumer. A `&mut [u8]` is an
`rt.ByteSlice`, which can only view a `Uint8Array`, while a `&mut u8` is an `rt.Place`
that reads and writes a JS number wherever it is. Giving a local or a field a one-byte
buffer of its own was found fragile: that representation would have to cross
parameters, fields, and map entries.

- A `&mut u8` parameter reaching `from_mut`: neither option was picked, and the question
  waits for the consumer.
  - Refuse at the caller, by a whole-program analysis.
  - Fall back at run time.
- `u8` struct fields:
  - ✅ Refuse fields for now.

`read_exact(slice::from_mut(&mut byte))` of a local is the common use; measure what each
form costs a hot `&mut u8`.

Public API: the translator's output.
