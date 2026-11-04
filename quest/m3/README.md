# m3: deferred

## Goal

Work that waits on evidence or has no consumer yet: the build-speed tracks,
each a measurement that decides whether its follow-up lands, and the parts of
std the translators cover before a consumer calls them.

## Plan

Decided with the maintainer on 2026-10-27: rust2 aims to support all of std, so a
generic std gap is planned here even when moq never calls it.

## Required

- [The runtime's namespace is rs](/quest/m3/ts-rs-rename.md) - translated code and hand-written callers import the runtime as `rs`, not `rt`
- [Retrial Cranelift on moq's tests](/quest/m3/cranelift-unwind.md) - waits on upstream: moq's tests pass under Cranelift, so its CI can weigh it again
- [Relink without rebuilding](/quest/m3/relink.md) - measure upstream's skip of dependency-only rebuilds on moq's edits once it reaches nightly
- [Unsafe code the translator can model](/quest/m3/ts-unsafe.md) - a dependency using only `unsafe` the translator can model translates from source
- [Trait objects](/quest/m3/ts-dyn.md) - std's other traits, generic traits, traits with an associated type, and `Any::type_id` translate, so a consumer neither rewrites nor binds them
- [Waiting: drop the lang fixture's mul_add allowance](/quest/m3/ts-fma-allowance.md) - the differential fails on any `mul_add` difference again once wasm32's `fma` rounds exactly
- [Waiting: type-check in process with tsrs](/quest/m3/ts-tsrs.md) - rust2-ts checks bindings through tsrs instead of `tsc` once it is a stable library
- [rs.Slice](/quest/m3/ts-slice-object.md) - slices and arrays of other values are a runtime type of array, start, and length, splitting in O(1) and comparing by where they point
- [Port the workflow guard to kixelated/quest](/quest/m3/quest-workflow-guard.md) - quest's environment guard parses YAML as this repository's does
- [rs.String](/quest/m3/ts-string-capacity.md) - `String`, `Box<str>`, and `&str` are a runtime type holding std's capacity
- [Every panic of a hooked program has its location](/quest/m3/ts-panic-sites.md) - no panic in a program that sets a hook lacks the location native code gives it
- [The chunk a BufMut lends](/quest/m3/ts-chunk-mut.md) - `BufMut::chunk_mut` and its `UninitSlice`'s writes translate, reaching the buffer's storage
- [Slices not yet written](/quest/m3/ts-uninit-slices.md) - `Box::new_uninit_slice` and `write_clone_of_slice` translate
- [Python calls skip the loop lookup](/quest/m3/ffi-python-loop-lookup.md) - a call while an asyncio loop runs costs what one outside does unless something waits
