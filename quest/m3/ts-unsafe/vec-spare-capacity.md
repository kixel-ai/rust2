# [M] A Vec's spare capacity

## Goal

[rust2-ts](/ts/README.md) translates writes into a `Vec<T>`'s spare capacity, through
`as_mut_ptr().add(len)` or `spare_capacity_mut`, committed by `set_len`, for any `T`,
proven by differential tests against native Rust. Today `set_len` is refused but for a
byte `Vec`'s (`unsupported call to std::vec::Vec::<u32>::set_len`, `ts/src/lower/body.rs`),
and a write past the length throws `a read or write through a pointer past the array JS
keeps it in` (`ts/runtime/rs/collections.ts`) where Rust defines it.

## Plan

`rs.Vec` holds std's capacity since [#555](https://github.com/moq-dev/rust2/pull/555);
the spare slots need to live outside the array's length until `set_len` takes them, as a
byte `Vec`'s do since [#568](https://github.com/moq-dev/rust2/pull/568). Removing the
run-time throw is also a gap of [Throw only where Rust panics](/quest/m3/ts-runtime-throws.md);
a read of a slot never written stays an abort, since Rust leaves it undefined.

No consumer at the moq pin. [Vec::drain](/quest/m3/ts-unsafe/vec-drain-mir.md) and
[BinaryHeap](/quest/m3/ts-unsafe/binary-heap-mir.md) from std's MIR need it.

Public API: the translator's output.
