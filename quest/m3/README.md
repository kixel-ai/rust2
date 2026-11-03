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
- [Trait objects](/quest/m3/ts-dyn.md) - `dyn Error`'s downcasts, `dyn FusedIterator`, and `Arc<dyn Iterator>` translate, so a consumer neither rewrites nor binds them
- [A trait object in a struct's last field](/quest/m3/ts-dyn-tail.md) - `Rc<RefCell<dyn Trait>>` and the like translate, the last field as the object
- [bytes' Chain as a Buf object](/quest/m3/ts-buf-chain.md) - `Chain::copy_to_bytes` across both halves translates, and with it a `Chain` as a `dyn Buf`
- [A Buf object's size](/quest/m3/ts-dyn-buf-size.md) - a `dyn Buf` object holds only the methods some call dispatches to
- [by_ref of a mutable reference to an iterator](/quest/m3/ts-by-ref-place.md) - `Iterator::by_ref` of a `&mut I` in a local translates
- [mem::take as a function value](/quest/m3/ts-take-value.md) - `mem::take`, `replace`, and `swap` passed as a function value translate
- [Debug of a RefCell](/quest/m3/ts-refcell-debug.md) - `{:?}` of a `RefCell`, `Ref`, and `RefMut` translates as std's writes them
- [Placeholders written to a crate's own fmt::Write](/quest/m3/ts-fmt-calls.md) - `write!` with placeholders into a crate's own writer translates, its `write_str` seeing each call std makes
- [Waiting: drop the lang fixture's mul_add allowance](/quest/m3/ts-fma-allowance.md) - the differential fails on any `mul_add` difference again once wasm32's `fma` rounds exactly
- [Waiting: type-check in process with tsrs](/quest/m3/ts-tsrs.md) - rust2-ts checks bindings through tsrs instead of `tsc` once it is a stable library
- [Drop order in std's folds and iterators](/quest/m3/ts-drop-order.md) - the values that drop still refused in std's consumers and collects drop as std's do
- [A slice of one value behind any &mut](/quest/m3/ts-slice-of-one.md) - `slice::from_mut` and `array::from_mut` of a `&mut u8`, or of an object no place names, translate
- [std's calls of a closure that have no mapping](/quest/m3/ts-closure-calls.md) - `try_fold`, `partition`, `repeat_with`, `sort_by_cached_key`, and `LazyCell::new` translate, dropping what their closures own as std's do
- [Waiting: a slice object for a mutable part of other values](/quest/m3/ts-slice-object.md) - waits on a moq profile showing the `Proxy` view's traps matter, then decides on an `rt.ByteSlice`-like form
- [Port the workflow guard to kixelated/quest](/quest/m3/quest-workflow-guard.md) - quest's environment guard parses YAML as this repository's does
- [Folds from the back, and a crate's own folds](/quest/m3/ts-iter-back-folds.md) - `rfold` and `try_rfold` translate, and so does a crate's iterator that overrides `fold` or `nth_back`
- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - a deque made from a `Vec` splits its slices where std's does
- [A B-tree's searches call Ord as std's do](/quest/m3/ts-btree-search-order.md) - a B-tree's lookups and inserts call a key's own `Ord` in std's order and number
- [Rc and Arc of a slice](/quest/m3/ts-shared-slices.md) - `Rc<[T]>` and `Arc<[T]>` from a `Vec`, a slice, or an iterator translate
- [Adding to a String](/quest/m3/ts-string-add.md) - `String + &str` translates as `+=` does
- [The rest of a vector's methods](/quest/m3/ts-vec-rest.md) - `append`, `resize`, `split_off`, `try_reserve`, and `Vec::from(VecDeque)` translate
- [A reversed zip's per-pair cost](/quest/m3/ts-zip-rev-overhead.md) - a reversed by-index zip costs per pair what a forward one does, or the gap is explained
- [Whole writes into a lazy cell or thread-local key](/quest/m3/ts-lazy-writes.md) - a whole write through a `&mut` to a `LazyCell`, `LazyLock`, or `LocalKey` keeps its state
- [Type errors at the Rust that made them](/quest/m3/ts-span-precision.md) - a `tsc` error on output names the tail expression or call argument that made it
- [An empty iterator of mutable references](/quest/m3/ts-empty-places.md) - `iter::empty::<&mut T>()` given to `zip` or `chain` translates
- [An Option of numbers in the result slot](/quest/m3/ts-option-slot.md) - `varint_opt`'s `Option` comes back through the slot with no object per call
- [Dropping a crate's generic type inside another](/quest/m3/ts-nested-generic-drop.md) - an `LruCache` holding a `LinkedHashMap` drops as rustc's glue does
- [Fresh writes through a zip, a copy, and a repeat](/quest/m3/ts-fresh-shapes.md) - zipping a `Vec<[T; N]>`, reading a `T` out of `Zip::next`, and `copied` or `vec![x; n]` over a `Copy` type translate
- [Python's wake on the loop's own thread](/quest/m3/ffi-python-wake.md) - a wake on the event loop's thread resumes the future with no socket pair
- [Python's call cost](/quest/m3/ffi-python-calls.md) - a call on an object costs what BoltFFI's does, or the gap is explained
- [No unused import in an export's generated crate](/quest/m3/ffi-unused-c-void.md) - the crate rust2-ffi generates builds without warnings when no export uses `void *`
- [A box's clone_from, and a vector's AsRef and AsMut](/quest/m3/ts-box-vec-conversions.md) - `clone_from` of a `Box<String>` and `AsRef<[T]>` of a `Vec` translate
- [A std map with a crate's own BuildHasher](/quest/m3/ts-map-own-hasher.md) - std's and hashbrown's maps built with a crate's own hasher translate for keys that are JS objects
- [A crate's own hasher that JS copies](/quest/m3/ts-copied-hasher.md) - an enum `Hasher`, whose `&mut` is a place, hashes as a struct one does
- [A reference to a closure as a string pattern](/quest/m3/ts-closure-patterns.md) - `s.contains(&f)` and `s.split(&mut f)` translate, leaving the closure to its owner to drop
