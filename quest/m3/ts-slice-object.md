# [L] rs.Slice

## Goal

[rust2-ts](/ts/README.md) translates `&[T]`, `&mut [T]`, `[T; N]`, and `Box<[T]>` of values
other than bytes as a runtime type of its array, start, and length, as `rs.ByteSlice` is
for bytes, rather than a JS array and the `Proxy` view a mutable part lends today:

- A mutable part, such as `&mut v[a..b]`, `split_at_mut`, or `chunks_mut` lends, splits in
  O(1) and reads at near a plain array's speed, proven by the lang fixture's differential
  tests and a benchmark against the view.
- Pointers to a `[T; N]` of other values, including those `first_chunk`, `last_chunk`,
  `split_first_chunk`, `as_chunks`, and `as_rchunks` lend, compare where they point, as
  `ptr::eq` does natively, lifting the refusal of
  [moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510).
- `slice::from_mut` and `array::from_mut` translate, of a `&mut u8` too, writing the
  value through the slice where the caller holds it.

Hand-written callers convert at the boundary with helpers, as for `rs.Vec`. Each is proven
by differential tests against native Rust.

## Plan

Decided with the maintainer on 2026-11-03: Rust types the runtime can't hold as std's do
get a runtime wrapper, converted at API boundaries, rather than a JS native type
([#555](https://github.com/moq-dev/rust2/pull/555) made `Vec` one). A native type, aliased
or branded, stays where it holds all of std's semantics with no extra allocation, as a
branded `number` does for an integer and a `Uint8Array` for a `[u8; N]`; prefer one if it
holds them, but capacity and the like usually need a wrapper. This quest absorbs the two
that waited: on a moq profile of the `Proxy` view, and on a consumer of `slice::from_mut`
of a `&mut u8`. Neither wait holds now.

A slice of one over a `&mut u8` was found fragile when it meant a one-byte buffer for each
local; a slice whose storage can be an `rs.Place` may avoid that. Measure what each form
costs a hot `&mut u8`, such as `read_exact(slice::from_mut(&mut byte))`.

Earlier findings:

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

Public API: the translator's output and what a hand-written caller passes for a slice.

## Related

- [The runtime's namespace is rs](/quest/m3/ts-rs-rename.md) - runs in parallel
- [rs.String](/quest/m3/ts-string-capacity.md) - the other native mapping becoming a wrapper
