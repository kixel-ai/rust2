# [M] Numeric slices cross in bulk

## Goal

A `&[T]`, `Vec<T>`, or `Box<[T]>` of an integer or float other than `u8` crosses a
[rust2-ffi](/ffi/README.md) binding as one block, as bytes already do: borrowed in place
on the way in, one copy or none on the way out, with no per-item call, box, or closure.
Each language holds it as its primitive array type: `IntArray` and its kin in Kotlin,
`Uint32List` and its kin in Dart, the buffer protocol in Python, and a pinned `[]T` in Go.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a child of the exporter
epic, since it changes generated types in Kotlin, Dart, and Python.

Today only `u8` is bulk. `ffi/src/glue.rs`'s `lift` collects any other list item by item
into a fresh collection, Kotlin's type is a boxed `List<UInt>`, Dart's a `List<int>`,
Python's a `list`, and Go lowers each item through a closure (`ffi/runtime/ffi_cgo.go`).
The api fixture has no numeric slice parameter, so the path is untested.

- A primitive array type stands in only where it is exact, as
  [lossless](/quest/m1/ffi-export/lossless.md) requires: Kotlin's unsigned arrays for
  unsigned items, and a Dart typed list whose element width matches. Where a language has
  none that is exact, such as a 128-bit item, the list stays as it is.
- `usize` and `isize` items differ in width by target, so they stay item by item unless
  a per-target layout, as the staged API records for a `Copy` value's size, makes one
  form exact on every target.
- Add numeric slice and vector parameters, returns, and fields to the api fixture, with
  each type's edge values, and a bench case for a slice of 1,000 `u32`s in and out.

Public API: the generated type of every numeric slice and vector in Kotlin, Dart, and
Python (breaking).

## Related

- [Lossless binding types](/quest/m1/ffi-export/lossless.md) - the exactness rule each array type must meet
