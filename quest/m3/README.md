# m3: deferred

## Goal

Work that waits on evidence or has no consumer yet: the build-speed tracks,
each a measurement that decides whether its follow-up lands, and the parts of
std the translators cover before a consumer calls them.

## Plan

Decided with the maintainer on 2026-10-27: rust2 aims to support all of std, so a
generic std gap is planned here even when moq never calls it.

## Required

- [Retrial Cranelift on moq's tests](/quest/m3/cranelift-unwind.md) - waits on upstream: moq's tests pass under Cranelift, so its CI can weigh it again
- [Relink without rebuilding](/quest/m3/relink.md) - measure upstream's skip of dependency-only rebuilds on moq's edits once it reaches nightly
- [Unsafe code the translator can model](/quest/m3/ts-unsafe.md) - a dependency using only `unsafe` the translator can model translates from source
- [Waiting: drop the lang fixture's mul_add allowance](/quest/m3/ts-fma-allowance.md) - the differential fails on any `mul_add` difference again once wasm32's `fma` rounds exactly
- [Waiting: type-check in process with tsrs](/quest/m3/ts-tsrs.md) - rust2-ts checks bindings through tsrs instead of `tsc` once it is a stable library
- [rs.Slice](/quest/m3/ts-slice-object.md) - references to arrays of other values are a runtime slice too, and pointers compare by where they point
- [Writes through iter_mut without a Place per item](/quest/m3/ts-iter-mut-places.md) - a loop over `iter_mut()` of copy values writes the slice's array by index
- [rs.String](/quest/m3/ts-string-capacity.md) - `String`, `Box<str>`, and `&str` are a runtime type holding std's capacity
- [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md) - no runtime `rust2-ts:` error where native Rust goes on: an exact slower path, or a refusal at translation
- [rust2-ts with no ts.toml](/quest/m3/ts-configless.md) - a crate's whole `pub` API translates with no config and no bound items
- [Buf on a deque of bytes](/quest/m3/ts-vecdeque-buf.md) - `bytes`' `Buf` for a deque of bytes translates
- [std's panic messages hold std's capacity](/quest/m3/ts-panic-payload-capacity.md) - a hook reads native's `String` capacity from std's own formatted panics
- [A B-tree drops as std's does](/quest/m3/ts-btree-drop-order.md) - keys and values drop node by node, in std's order
