# [M] Drop order in std's folds and iterators

## Goal

[rust2-ts](/ts/README.md) translates the values that drop which it still refuses
inside std's iterators, each dropping where std's does, in std's order,
proven by the task fixture's differential tests against native Rust. None appears in moq's census at the pinned revision; rust2 aims to support
all of std.

## Plan

Moved from the m2 drop quest on 2026-10-27, once moq-net, its model, and kio had no
drop refusal left ([#139](https://github.com/moq-dev/rust2/pull/139)). `DebugList::entries`,
ranges of bounds that drop, a drain or a B-tree's `range` of a range that drops, a hash
map's `into_keys` and `into_values`, and the closures `for_each` and `fold` own landed in
[#307](https://github.com/moq-dev/rust2/pull/307), with the task fixture's `inside` step.
The closures `reduce`, `max_by`, `min_by`, their `_by_key` forms, `is_sorted_by_key`, and
a map's entries' `or_insert_with`, `or_insert_with_key`, and `and_modify` own landed in
[#438](https://github.com/moq-dev/rust2/pull/438), with its `released` step.
A `skip` of a zip std reads by index, a zip collected in place, and a `flatten` whose
inner iterators hold items once they end landed in
[#446](https://github.com/moq-dev/rust2/pull/446), with its `indexed` and
`flattened` steps.
Each left is refused with a span where std's order is one JS's can't follow yet:

- A B-tree's `collect` of entries that drop, sorted and deduplicated by the crate's own
  impls, as `v.into_iter().collect::<BTreeSet<O>>()` of an `O` whose `Ord`, `PartialOrd`,
  or `PartialEq` is its own: one that panics or disagrees with itself leaves equal keys
  in the order std's sort does, which decides which of them
  drop.

Each `rt.Iter` has a `fold` and `rfold` that drop what it holds where std's do, and the
by-value consumers std folds go through it: build on those rather than `rt.drop`.

Public API: the translator's output for values that drop in std's iterators.
