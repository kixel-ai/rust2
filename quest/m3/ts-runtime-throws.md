# [M] Throw only where Rust panics

## Goal

Code [rust2-ts](/ts/README.md) generates fails at run time only where the Rust source
does: no `rust2-ts:` error the runtime throws where native Rust would go on. Each such
construct translates exactly, or is refused at translation with a span. Proven by
differential tests against native Rust for every construct that now translates.

## Plan

Decided with the maintainer on 2026-11-04: "we should not panic when Rust would not
panic", and runtime throws that copy Rust's behaviour are fine. A throw that guards Rust
UB stays, as an abort worded like std's `unsafe precondition(s) violated` check.

Decided with the maintainer on 2026-11-07, in
[moq-dev/rust2#661](https://github.com/moq-dev/rust2/pull/661), for the throws that fire
only because translation can't see where a pointer came from:

- ✅ Refuse at translation, with a span, where the body doesn't show the pointer fits:
  `rs.arcErase` (done in #661) and `alloc`'s bytes.
- ✅ Keep as loud run-time checks: `ptrUnerase`, `offset_from` across arrays,
  `AtIndex.reach`, and raw slices of a field's byte, which AGENTS.md allows under m3's
  [unsafe policy](/quest/m3/README.md).
  Two pointers made from one reference, one through a branch or loop, may each be in
  an array of one of its own; keep the loud check rather than trace joins (decided
  2026-11-07 in [#621](https://github.com/moq-dev/rust2/pull/621)).
- Rejected: refusing all of them at translation, which refuses pointer arithmetic on any
  pointer parameter; keeping all as run-time checks.

Decided with the maintainer on 2026-11-07, in the quest audit: ✅ `Bytes::slice_ref` of
bytes from other storage is refused at translation, since it is safe code and AGENTS.md
allows a run-time check only for an `unsafe` pointer. Rejected: keeping its run-time
check, as #661 had, and widening AGENTS.md's exception to safe code with no exact answer
while a `&str` stays a JS string.

What remains:

- `alloc`'s bytes: a byte of memory from `alloc` read or written, or the memory cast to a
  type other than the one it holds, throws in `rs/alloc.ts` when the byte pointer reached
  there from elsewhere. Refuse at the `alloc` or `realloc` call whose byte pointer the
  body doesn't show going only to a typed cast, `dealloc`, `realloc`, or the
  `NonNull`/`Option` steps [`alloc_root`](/ts/src/lower/mir/alloc.rs) already follows, so
  no untyped pointer leaves the body; and a `realloc` of memory the body cast from one type
  whose result it casts to another. This needs a use analysis of that pointer, which the
  translator doesn't have yet. The throws in `rs/alloc.ts` are then translator guards.
- `ptr::eq` of an array `as_chunks` split from one flat slice, which std's callback gets a
  copy of, answers `false` where Rust answers `true`, silently (moved here 2026-11-07 in
  [#608](https://github.com/moq-dev/rust2/pull/608)). A copy that remembers the slice and
  offset it came from, compared through that, is exact; it builds on the `rs.ArraySlice`
  [#662](https://github.com/moq-dev/rust2/pull/662) left, where `rs.arraySliceItems` still
  copies.
- `Bytes::slice_ref` of bytes from other storage throws in `bytesSliceRef`
  (`ts/runtime/rs/bytes.ts`), since Rust may share what JS copied, such as a
  `&'static str`'s bytes that `as_bytes` encodes anew. Refuse the call whose `subset` the
  body doesn't show borrowed from that same `Bytes`, through `Deref`, `as_ref`, or a slice
  of one; the throw is then a translator guard, and `ts/README.md` says so.
- `bytes`' panics in a hook program: `BYTES` in [`caller.rs`](/ts/src/lower/caller.rs)
  finds each by searching the source of the program's `bytes` for text, so a
  `--remap-path-prefix` of a directory inside `bytes` fails translation of a hook program.
  Take each panic's location from `bytes`' compiled code for the function that panics, as
  [`tokio.rs`](/ts/src/lower/tokio.rs) does for tokio's oneshot `poll`, so rustc names the
  file as native does under any remap, and delete the source search (decided with the
  maintainer on 2026-11-07, from [#661](https://github.com/moq-dev/rust2/pull/661)'s
  follow-ups).

Public API: the translator's output, and which crates translate.

## Related

- [Array references from slices and cells](/quest/m3/ts-array-ref-conversions.md) - also changes how an array taken from a flat slice is represented, on the same `rs.ArraySlice`
