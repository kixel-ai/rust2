# [M] The rest of a vector's methods

## Goal

[rust2-ts](/ts/README.md) translates the `Vec` and `VecDeque` methods still refused, each
holding what std guarantees of its capacity, proven by differential tests against native
Rust.

## Plan

Found by a probe calling each stable method: these have no mapping, and std's bodies move
elements through raw pointers, so each refuses with its span.

- `try_reserve` and `try_reserve_exact`, of a `Vec` and a `VecDeque`, which need a
  `TryReserveError` of the runtime's with std's `Debug`, `Display`, `Eq`, and `Error`. Its
  `AllocError` is the allocator's, which JS has none of, so whether one is ever returned,
  or the call refused where it could be, is open.
- `VecDeque::rotate_left`, `rotate_right`, `swap`, `make_contiguous`, `binary_search`,
  `binary_search_by`, `binary_search_by_key`, and `partition_point`, each placing its
  items where std's does, which `as_slices` shows.
- `Vec::splice`, `extract_if` with its `Debug` (`retained`, `remainder`, and
  `skipped_tail`), `into_flattened`, `leak`, and `spare_capacity_mut`; the last two hand
  out memory past the length, which may stay refused.

A vector's capacity follows only std's guarantees, at least its length and what
`with_capacity` and `reserve` ask for, and doubles as it grows (decided with the
maintainer on 2026-11-01). No consumer calls them yet; they are planned here as part of
std.

Public API: the translator's output.
