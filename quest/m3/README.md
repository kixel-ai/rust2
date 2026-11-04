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
- [The spare capacity of a BytesMut](/quest/m3/ts-bytes-capacity.md) - `BytesMut::spare_capacity_mut` translates, its writes reaching the storage
- [Trait objects](/quest/m3/ts-dyn.md) - std's other traits, generic traits, traits with an associated type, and `Any::type_id` translate, so a consumer neither rewrites nor binds them
- [Placeholders written to a crate's own fmt::Write](/quest/m3/ts-fmt-calls.md) - `write!` with placeholders into a crate's own writer translates, its `write_str` seeing each call std makes
- [Waiting: drop the lang fixture's mul_add allowance](/quest/m3/ts-fma-allowance.md) - the differential fails on any `mul_add` difference again once wasm32's `fma` rounds exactly
- [Waiting: type-check in process with tsrs](/quest/m3/ts-tsrs.md) - rust2-ts checks bindings through tsrs instead of `tsc` once it is a stable library
- [Waiting: a slice of one byte behind any &mut](/quest/m3/ts-slice-of-one.md) - `slice::from_mut` and `array::from_mut` of a `&mut u8` translate once a consumer calls them
- [Waiting: a slice object for a mutable part of other values](/quest/m3/ts-slice-object.md) - waits on a moq profile showing the `Proxy` view's traps matter, then decides on an `rt.ByteSlice`-like form
- [Port the workflow guard to kixelated/quest](/quest/m3/quest-workflow-guard.md) - quest's environment guard parses YAML as this repository's does
- [Debug of a str's searches](/quest/m3/ts-iter-source-debug.md) - `Debug` of `str`'s `Split` and its forms, `Matches`, `Lines`, and `SplitWhitespace` writes their searcher's state as std's does
- [A string's exact capacity](/quest/m3/ts-string-capacity.md) - a `String` holds std's capacity, so a program making a `Bytes` of one reads its capacity
- [A B-tree's searches call Ord as std's do](/quest/m3/ts-btree-search-order.md) - a B-tree's lookups and inserts call a key's own `Ord` in std's order and number
- [A panic hook](/quest/m3/ts-panic-hook.md) - `set_hook`, `take_hook`, and `PanicHookInfo` translate, every panic running the hook
- [A string Cow's variant](/quest/m3/ts-cow-str-variant.md) - matching a `Cow<str>`'s `Borrowed` or `Owned`, and `to_mut`'s change of it, translate as std's
- [An Arc's raw pointer to a value JS copies](/quest/m3/ts-arc-raw-place.md) - `from_raw` of a pointer remade from a reference to an `Arc`'s number, and an `Arc<AtomicBool>`'s raw pointer cast to `*const ()`, translate
