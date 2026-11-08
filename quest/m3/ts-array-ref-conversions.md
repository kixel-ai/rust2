# [S] Array references from slices and cells

## Goal

[rust2-ts](/ts/README.md) translates `TryFrom<&mut [T]> for &mut [T; N]` (and its `&[T]`
twin, as `slice.try_into()` reaches them), `Cell::as_array_of_cells`, and
`convert::identity` of a `&mut [T; N]`, each lending the array where it is, proven by
differential tests against native Rust. Today each is an unsupported call.

## Plan

Suggested on [#662](https://github.com/moq-dev/rust2/pull/662), which removed `arrayView`
and lends parts of arrays where they are: its tests use `first_chunk_mut` in place of these.
`try_into` of a slice into an array reference is common in parsers. Build on #662's
`rs.ArraySlice` and slice helpers, adding no new pointer form, per m3's
[unsafe policy](/quest/m3/README.md).

Decided with the maintainer on 2026-11-07: ✅ plan it, in m3 (from #662's follow-ups).

Public API: the translator's output.
