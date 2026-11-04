# [L] Waiting: a slice object for a mutable part of other values

## Goal

Decide, from a profile of moq, whether a mutable part of a slice of other values, such as
`&mut v[a..b]`, `split_at_mut`, or `chunks_mut` lends, needs a faster form than the
runtime's `Proxy` view. Waits until a profile of translated moq shows the view's traps
matter, by time spent in `sliceView`'s handler or in code reading through a view. If it
does, a `&mut [T]` becomes an object of its array, start, and length, as `rt.ByteSlice`
is for bytes, which splits in O(1) and reads at near a plain array's speed. Proven by the
lang fixture's differential tests and a benchmark against the view.

Once `&[T]` takes that form too, pointers to a `[T; N]` of other values than bytes,
including those a slice lends by `first_chunk`, `last_chunk`, `split_first_chunk`,
`as_chunks`, or `as_rchunks`, compare where they point, as `ptr::eq` does natively,
lifting the refusal of [moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510),
proven by differential tests against native Rust.

## Plan

Decided on 2026-10-30 in [moq-dev/rust2#274](https://github.com/moq-dev/rust2/pull/274):
lend the `Proxy` view, since it covers every part, including one returned or stored, and
moq's sites run per timer operation or control message, not per frame. The traps cost
per operation, in ns, Node 24, from that PR's scratch benchmark:

| shape | plain array | Proxy view | copy-back | `{ s, start, length }` |
|---|---|---|---|---|
| heap of 16, one wasmtimer sift step | 8 | 582 | 55 | 17 |
| heap of 256, one sift step | 8 | 669 | 296 | 9 |
| heap of 4096, one sift step | 7 | 3776 | 9020 | 9 |
| `parts[..1]` written | 6 | 301 | 21 | 5 |
| `parts[..4]` written | 5 | 965 | 24 | 6 |
| `parts[..32]` written | 13 | 7274 | 74 | 16 |

A tuned `Proxy` measured about the same, so the trap is the floor. Copy-back, the part
copied and written back as its borrow ends, can't lend a part that outlives its scope.

The object changes every mapping that takes a `&mut [T]`, the coercion to `&[T]`, and
what a hand-written caller passes, so weigh whether `&[T]` takes the same form.

Pointer identity waits for that form, decided on 2026-11-03 in
[moq-dev/rust2#558](https://github.com/moq-dev/rust2/pull/558). Today a shared part of
other values is a copy, so a run it lends says nothing of where the slice holds it. Without
the object, every shared part would note the array beneath and its offset, and every
loan, shared or mutable, would be one view per run, which
[moq-dev/rust2#529](https://github.com/moq-dev/rust2/pull/529)'s runs detach before a
whole write. Noting the origin costs, per shared part of 8 of 16 values, in ns:

| | Node 24 | Bun 1.4 |
|---|---|---|
| a copy, today | 21.9 | 12.6 |
| a runtime switch, off | 23.5 | 19.0 |
| the origin as a symbol property | 27.4 | 30.6 |
| the origin in a `WeakMap` | 410 | 116 |

The object carries the array and start, and compares by them as `rt.ByteSlice` does.

Public API: the translator's output and what a caller passes for a slice.
