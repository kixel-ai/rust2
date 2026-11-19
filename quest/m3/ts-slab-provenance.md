# [S] offset_from tells apart byte allocations that share the slab

## Goal

`rs.ptrOffsetFrom` of pointers into two different byte allocations fails with its
rust2-ts error, as [m3's unsafe policy](/quest/m3/README.md) (item 5) requires, even when
both came from `rs.bytes`' shared `ArrayBuffer`.

## Plan

Found while making the string codec allocate from `rs.bytes`
([#877](https://github.com/kixel-ai/rust2/pull/877)): `ptrOffsetFrom` treats two views
of one `ArrayBuffer` as one allocation (`ts/runtime/rs.ts:507`), so `offset_from` of two
`Vec<u8>`s' pointers on `main` returns their distance in the slab, -8 for two fresh
4-byte vectors, instead of failing. #877 gave `str::as_ptr` an array of its own, so it
fails loud again; byte vectors and slices still do not.

Recommendation: record each slab allocation's extent where `rs.bytes` hands it out, such
as a `WeakMap` from the allocation's view to it, and have `ptrOffsetFrom` accept views of
one buffer only within one extent. A view made by `subarray` of an allocation inherits its
extent. The check is on `unsafe` paths only, so its cost stays off the hot ones.

Public API: none.
