# [S] Small shapes export

## Goal

The census at the moq pin refuses no item of moq's own for a small shape: 26 items at moq
`ffcb7cd9` use `u128` (`Timestamp::as_nanos`), an array, a const generic, unit outside a
return, `Result` outside a return, `!`, or an enum without variants. The count includes
the objects and futures that are not `Send`, which
[not-send](/quest/m1/ffi-export/not-send.md) owns since 2026-11-09.

## Plan

Each is its own small mapping. Changing moq is a non-goal: what has no C shape yet gets
one here or in the quest that owns it, never a request for moq to change its signature.

Decided with the maintainer on 2026-11-09:

- ✅ `u128` maps by rule as `<prefix>_u128`, two `uint64_t` halves in C, with each
  language's exact big integer above it: Python `int`, Kotlin `BigInteger`, Dart `BigInt`,
  Go `*big.Int`, Swift a `U128` struct until its deployment target allows `UInt128`, and
  C++ the C struct. Rejected: leaving it out. Clears `Timestamp::as_scale`, `as_millis`,
  `as_micros`, and `as_nanos`. `i128` stays refused: no consumer, and its sign is a
  conversion of its own in each language.
- ✅ Not `Send` (`clock::Entered`, `clock::Rng`, `catalog::Guard`) belongs to
  [not-send](/quest/m1/ffi-export/not-send.md)'s thread check.
- ✅ Const generics land here once [generics](/quest/m1/ffi-export/refusals/generics.md)
  does, since both edit instance inference. Rejected: a separate quest.

✅ A `()` parameter is passed by nothing in C, clearing `container::Producer<C>::set`'s four
instances, and an enum without variants is an object no export returns, which the bindings
leave out.

Left at moq `ffcb7cd9`, outside the moq-tokio items narrowing the stand-in drops (the
arrays, `Result` and `!` off a return, and `worker::Group::finished`'s future):

- const generics: `group::Producer::write_frames` and `group::Consumer::read_frames` over
  `frame::Buffer<const N: usize = 8>`. The IR keeps only type arguments; a const argument
  in `Type`, inferred at its default as a type parameter's is, exports `Buffer<8>` and the
  methods at it.

## Required

- [Generics](/quest/m1/ffi-export/refusals/generics.md) - instance inference, which const
  arguments extend
