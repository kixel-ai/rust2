# [M] Export ordering

## Goal

A type's `PartialOrd` and `Ord` impls export in every binding: C gets a `_cmp` returning
-1, 0, or 1, and `_partial_cmp` adding "unordered"; C++ gets `<` and, from C++20, `<=>`;
each other language its own comparison protocol (`Comparable`, `__lt__` and friends,
`compareTo`, `cmp.Compare`-shaped functions). The `api` fixture covers a total and a
partial order, and moq-net's `Timestamp` orders from every binding.

## Plan

Found by the 2026-11-18 language audit: `ir::Traits` (`ffi/src/ir.rs:404`) has no
ordering, so C and C++ cannot order moq's frame timestamps, whose `PartialOrd` is
hand-written (`rs/moq-net/src/model/time.rs:426` in moq-dev/moq).

Decided with the maintainer on 2026-11-18: ✅ kept, outside the exporter epic since it only
adds API (rejected: dropping it). Recommended: a hand-written `PartialOrd` that is not a
total order exports only the partial comparison, never `Comparable`.

Public API: added comparison in every binding.
