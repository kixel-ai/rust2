# [M] Small shapes export

## Goal

The census at the moq pin refuses no item of moq's own for a small shape: 26 items at moq
`ffcb7cd9` use `u128` (`Timestamp::as_nanos`), an array, a const generic, unit outside a
return, `Result` outside a return, `!`, or an enum without variants. The count includes
the objects and futures that are not `Send`, which
[not-send](/quest/m1/ffi-export/not-send.md) owns since 2026-11-09.

## Plan

Each is its own small mapping; what has no C shape, such as an object that is not `Send`
under the C ABI's any-thread calls, keeps its refusal and goes to the maintainer as moq's
signature to change.

✅ A `()` parameter is passed by nothing in C, clearing `container::Producer<C>::set`'s four
instances, and an enum without variants is an object, which no export returns, so it leaves
with [the unholdable rule](/quest/m1/ffi-export/refusals/unholdable.md).

Left at moq `ffcb7cd9`, outside the moq-tokio items narrowing the stand-in drops (the
arrays, `Result` and `!` off a return, and `worker::Group::finished`'s future):

- `u128`: `Timestamp::as_scale`, `as_millis`, `as_micros`, `as_nanos`. std returns it too
  (`Duration::as_nanos`), so it maps by rule: proposed as `<prefix>_u128`, two `uint64_t`
  halves in C, and each language's arbitrary-precision integer above it.
- const generics: `group::Producer::write_frames` and `group::Consumer::read_frames` over
  `frame::Buffer<const N: usize = 8>`. The IR keeps only type arguments; a const argument
  in `Type`, inferred at its default as a type parameter's is, exports `Buffer<8>` and the
  methods at it.
- not `Send`: `clock::Entered` and `clock::Rng` pin a thread, and `catalog::Guard` holds a
  `MutexGuard`; no C shape, so moq's signatures change or they stay refused.

## Waiting

- Maintainer: choose the `u128` mapping, and whether moq changes or keeps refused
  `Entered`, `Rng`, and `Guard`; check the PR's decisions are answered
