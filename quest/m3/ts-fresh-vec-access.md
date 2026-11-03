# [M] Fresh writes through a vector's own accessors

## Goal

[rust2-ts](/ts/README.md) translates a whole write into a vector's element, then a raw
pointer into a part of it, where the vector is reached through std's own accessors, proven
by differential tests against native Rust. Each is refused today with "a raw pointer to
`(u8, u8)` in ..., which the crate may put another object in place of", for concrete types
too:

- a shared borrow of the vector, or of the one it was collected from, such as `v.len()` or
  `other.iter().copied().collect()`: `Trace::origins` gives up on a local any shared borrow
  takes, as if one could change it;
- `v.first_mut()` and `v.get_mut(i)`: a std call given a `&mut [T]` counts as replacing
  `T`'s parts, as a slice is no object `fills_fresh` judges;
- a `for` over an array by value: `array::IntoIter::next` is given the `&mut` to an
  iterator holding the elements in place, which counts as replacing them;
- a vector collected from a std iterator, of a type that holds a byte in place: a `Vec`'s
  buffer pointer is `Unique<u8>`, so it reads as pointing at that byte.

## Plan

Found by [#516](https://github.com/moq-dev/rust2/pull/516), whose tests step around each.
They are separate causes; reproduce each and fix the one that is a single cause first. A
`Vec`'s buffer typed by its element would make `points_to` exact, but a heap buffer is no
part of another value, so check it refuses nothing new.

Public API: the translator's output.

## Related

- [Fresh writes beside a zipped iterator that points at a part](/quest/m3/ts-fresh-shapes.md) - the zip half of #516's leftovers
