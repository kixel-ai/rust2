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
- [The capacity of a BytesMut](/quest/m3/ts-bytes-capacity.md) - `BytesMut::capacity` and what reads it translate with `bytes`' values
- [Trait objects](/quest/m3/ts-dyn.md) - std's other traits, generic traits, traits with an associated type, and `Any::type_id` translate, so a consumer neither rewrites nor binds them
- [Placeholders written to a crate's own fmt::Write](/quest/m3/ts-fmt-calls.md) - `write!` with placeholders into a crate's own writer translates, its `write_str` seeing each call std makes
- [Waiting: drop the lang fixture's mul_add allowance](/quest/m3/ts-fma-allowance.md) - the differential fails on any `mul_add` difference again once wasm32's `fma` rounds exactly
- [Waiting: type-check in process with tsrs](/quest/m3/ts-tsrs.md) - rust2-ts checks bindings through tsrs instead of `tsc` once it is a stable library
- [A slice of one byte behind any &mut](/quest/m3/ts-slice-of-one.md) - `slice::from_mut` and `array::from_mut` of a `&mut u8` translate
- [Debug of std's iterator adaptors](/quest/m3/ts-iter-adaptor-debug.md) - `Debug` of `Map`, `Zip`, `Peekable`, `DecodeUtf16`, and the other adaptors writes what std's does
- [Waiting: a slice object for a mutable part of other values](/quest/m3/ts-slice-object.md) - waits on a moq profile showing the `Proxy` view's traps matter, then decides on an `rt.ByteSlice`-like form
- [Port the workflow guard to kixelated/quest](/quest/m3/quest-workflow-guard.md) - quest's environment guard parses YAML as this repository's does
- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - a deque made from a `Vec` splits its slices where std's does
- [A B-tree's searches call Ord as std's do](/quest/m3/ts-btree-search-order.md) - a B-tree's lookups and inserts call a key's own `Ord` in std's order and number
- [A slice's chunks as mutable arrays](/quest/m3/ts-slice-conversions.md) - `as_chunks_mut` and `as_rchunks_mut` translate, writes landing in the slice
- [Repeated values that drop](/quest/m3/ts-vec-from-elem-drops.md) - `vec![x; n]` translates where `x` drops, cloned and dropped as std's `from_elem` does
- [A vector's capacity where clone_from writes it in a container](/quest/m3/ts-clone-from-capacity.md) - a `Vec` an array, slice, `Option`, or list clones over keeps its capacity as std's does
- [Comparing pointers to arrays of other values than bytes](/quest/m3/ts-array-ref-identity.md) - `ptr::eq` of pointers to a `[T; N]` of non-bytes translates, as a slice lends them too
- [An Arc's pointer](/quest/m3/ts-arc-pointers.md) - `Arc::as_ptr` and `Rc::as_ptr` translate, comparable by `ptr::addr_eq`
- [The rest of a vector's methods](/quest/m3/ts-vec-rest.md) - `try_reserve`, a deque's rotations and searches, and `Vec::splice` translate
- [A reversed zip's per-pair cost](/quest/m3/ts-zip-rev-overhead.md) - a reversed by-index zip costs per pair what a forward one does, or the gap is explained
- [Type errors at the Rust that made them](/quest/m3/ts-span-precision.md) - a `tsc` error on output names the tail expression or call argument that made it
- [Fresh writes beside a zipped iterator that points at a part](/quest/m3/ts-fresh-shapes.md) - zipping slots with a `Vec<[T; N]>`, `vec![row; n]`, or `copied` of what the slot holds translates
- [Fresh writes through a vector's own accessors](/quest/m3/ts-fresh-vec-access.md) - a whole write into a vector's element stays fresh through a shared borrow, `first_mut`, a by-value array `for`, and a byte-holding collect
- [Python's wake on the loop's own thread](/quest/m3/ffi-python-wake.md) - a wake on the event loop's thread resumes the future with no socket pair
- [Python's call cost](/quest/m3/ffi-python-calls.md) - a call on an object costs what BoltFFI's does, or the gap is explained
- [A std map with a crate's own BuildHasher](/quest/m3/ts-map-own-hasher.md) - std's and hashbrown's maps built with a crate's own hasher translate for keys that are JS objects
