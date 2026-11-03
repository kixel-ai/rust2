# [S] A vector's capacity where clone_from writes it in a container

## Goal

[rust2-ts](/ts/README.md) keeps a `Vec`'s capacity where std's `clone_from` writes over
it inside a container, as it does where the `Vec` is the target itself, proven by
differential tests against native Rust that read each element's `capacity()`.

## Plan

std's `Vec::clone_from` keeps its buffer, grown only if it holds fewer, and an array's,
a slice's `clone_from_slice` and `fill`, an `Option`'s, a `Vec`'s, and a list's
`clone_from` each call it on the `Vec`s they hold. Where the `Vec`'s elements drop
nothing, the translation instead writes a fresh clone with room for exactly its
elements: `[Vec::<u32>::with_capacity(8)].clone_from(&[vec![1]])` leaves a capacity of 8
in Rust and 1 in JS. Where they drop, `rt.vecCloneFrom` already keeps it, so treating
every `Vec`'s `clone_from` as its own over what it holds may close this, with a byte
form for `Vec<u8>`. No consumer reads it yet; it is planned as part of std.

Public API: the translator's output.

## Related

- [A vector's exact capacity as it shrinks](/quest/m3/ts-vec-exact-capacity.md) - other capacities std keeps
