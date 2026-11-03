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
- [Waiting: a slice of one byte behind any &mut](/quest/m3/ts-slice-of-one.md) - `slice::from_mut` and `array::from_mut` of a `&mut u8` translate once a consumer calls them
- [Wakers made from a RawWaker](/quest/m3/ts-raw-waker.md) - `Waker::from_raw` and the rest of the raw waker API translate, each vtable call where std makes it
- [Waiting: a slice object for a mutable part of other values](/quest/m3/ts-slice-object.md) - waits on a moq profile showing the `Proxy` view's traps matter, then decides on an `rt.ByteSlice`-like form
- [Port the workflow guard to kixelated/quest](/quest/m3/quest-workflow-guard.md) - quest's environment guard parses YAML as this repository's does
- [Debug of a str's searches](/quest/m3/ts-iter-source-debug.md) - `Debug` of `str`'s `Split` and its forms, `Matches`, `Lines`, and `SplitWhitespace` writes their searcher's state as std's does
- [A B-tree's extract_if](/quest/m3/ts-btree-extract-if.md) - `BTreeMap::extract_if` and `BTreeSet::extract_if` translate, with their `Debug`
- [A vector keeps the capacity its pushes grew as it shrinks](/quest/m3/ts-vec-clear-capacity.md) - every step that drops a pushed `Vec`'s length keeps its capacity, as std guarantees
- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - a deque made from a `Vec` splits its slices where std's does
- [A B-tree's searches call Ord as std's do](/quest/m3/ts-btree-search-order.md) - a B-tree's lookups and inserts call a key's own `Ord` in std's order and number
- [Comparing pointers to arrays of other values than bytes](/quest/m3/ts-array-ref-identity.md) - `ptr::eq` of pointers to a `[T; N]` of non-bytes translates, as a slice lends them too
- [Cow's clone_from, Reverse's clone, and Option::as_slice](/quest/m3/ts-std-stragglers.md) - three std methods still refused translate as std's behave
- [The rest of a vector's methods](/quest/m3/ts-vec-rest.md) - `try_reserve`, a deque's rotations and searches, and `Vec::splice` translate
- [Fresh writes beside a zipped iterator that points at a part](/quest/m3/ts-fresh-shapes.md) - zipping slots with a `Vec<[T; N]>`, `vec![row; n]`, or `copied` of what the slot holds translates
- [Python's wake on the loop's own thread](/quest/m3/ffi-python-wake.md) - a wake on the event loop's thread resumes the future with no socket pair
- [Python's call cost](/quest/m3/ffi-python-calls.md) - a call on an object costs what BoltFFI's does, or the gap is explained
- [A map built with BuildHasherDefault](/quest/m3/ts-build-hasher-default.md) - `FxHashMap`-style maps, of `BuildHasherDefault` over a crate's own `Hasher`, translate
- [A name or an operator argument at its own Rust span](/quest/m3/ts-argument-spans.md) - a `tsc` error on a plain-name or operator argument names that argument's Rust, not the method it is passed to
- [A panic's Location](/quest/m3/ts-panic-location.md) - `Location::caller` and `#[track_caller]` translate with native Rust's locations
- [A panic hook](/quest/m3/ts-panic-hook.md) - `set_hook`, `take_hook`, and `PanicHookInfo` translate, every panic running the hook
- [A string Cow's variant](/quest/m3/ts-cow-str-variant.md) - matching a `Cow<str>`'s `Borrowed` or `Owned`, and `to_mut`'s change of it, translate as std's
