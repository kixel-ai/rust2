# [S] Numeric slices on the binding benchmark

## Goal

[The binding benchmark](/bench/ffi/README.md) times a slice of 1,000 `u32`s passed into
Rust and a `Vec` of them returned, in every language, so the bulk crossing that
[kixel-ai/rust2#743](https://github.com/kixel-ai/rust2/pull/743) gave numeric slices is
measured beside bytes.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a child of the exporter
epic.

#743 made every `&[T]`, `Vec<T>`, and `Box<[T]>` of a fixed-width integer or float cross
as one block, and each language hold it as its primitive array. The benchmark exports
only moq's stand-in facade, whose API has no numeric slice, so the case needs a function
to call.

`usize` and `isize` items still cross item by item, unless
[copy-values](/quest/m1/ffi-export/copy-values.md)'s per-target layout makes one form
exact on every target.

## Waiting

- Maintainer: where the benchmark's numeric-slice function lives. Options: a `bench`
  module the `just ffi standin` recipe adds to the stand-in facade, which the census and
  `just ffi moq` would then also export (recommended: one function, the least setup); a
  second facade the benchmark exports beside moq's; or
  waiting until moq's API has a numeric slice. An agent cannot pick this, since it
  changes what the stand-in exports.

## Related

- [Lossless binding types](/quest/m1/ffi-export/lossless.md) - the exactness rule each array type meets
