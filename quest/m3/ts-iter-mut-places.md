# [M] Writes through iter_mut without a Place per item

## Goal

[rust2-ts](/ts/README.md) translates a loop over `iter_mut()` of an `rs.Slice` or `rs.Vec`
of copy values, such as `for x in part.iter_mut() { *x += 1 }`, as indexed writes into the
slice's array, rather than an `rs.Place` allocated for each item by `rs.iterSliceMut`.
Proven by differential tests against native Rust, and measured by the codec bench's
`parts/prefix-writes`, 15-25x off its hand-written plain-array route today.

## Plan

A `Place` per item is the general form, needed when an item's reference escapes the loop
body. Where the lowering sees every use of the item is a read or write through it, the
iterator can yield the index instead.

## Related

- [Parts of arrays without arrayView](/quest/m3/ts-slice-object.md) - the slices these loops index
