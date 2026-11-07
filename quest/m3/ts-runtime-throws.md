# [M] Throw only where Rust panics

## Goal

Code [rust2-ts](/ts/README.md) generates fails at run time only where the Rust source
does: no `rust2-ts:` error the runtime throws where native Rust would go on. Each such
construct translates exactly, or is refused at translation with a span. Proven by
differential tests against native Rust for every construct that now translates.

## Plan

Decided with the maintainer on 2026-11-04: "we should not panic when Rust would not
panic", and runtime throws that copy Rust's behaviour are fine. A throw that guards Rust
UB stays, as an abort worded like std's `unsafe precondition(s) violated` check, since
Rust defines nothing there. m3's [unsafe policy](/quest/m3/README.md) (2026-11-07)
supersedes the 2026-11-04 "slower exact path" for `unsafe` gaps: where exactness needs a
new pointer form or pointer metadata, refuse at translation with a span.

[moq-dev/rust2#661](https://github.com/moq-dev/rust2/pull/661) made the UB guards
`rs.violated` aborts (null, dangling, or moved-off reads and writes, writes through
`str::as_ptr`, `from_raw` of a pointer no `into_raw` gave or of a dropped value, `dealloc`
of memory `alloc` didn't give, a `TrustedLen` zip running out), located std's `Zip`
`unreachable!()` and tokio's oneshot panic in hook programs, and found no other panic a
hook can't locate in a forced-hook survey of the `lang` fixtures. Of `git grep 'rust2-ts: '
ts/runtime`, these are not gaps: a range's missing `Step` and a fmt width written as a
placeholder (no translation makes either), `rs.u64Of` and its kin given a number past
2^53 (a caller's input), a hook's unlocatable panic (a translator bug guard), and a substring's pointer
read outside its bytes, or a mutable slice through `str::as_ptr` (undefined under Stacked
Borrows; they throw until Rust settles its memory model, decided 2026-11-07 in
[#634](https://github.com/moq-dev/rust2/pull/634)).

What remains throws only where translation can't see a pointer's or a byte slice's
origin, since each refusal where the body shows it is already made:

- `rs.arcErase`: a pointer to a value JS copies cast to `*const ()`, which isn't an `Arc`'s
  or `Rc`'s raw pointer, so has no one place to compare as.
- `rs.ptrUnerase`: a `*const ()` cast back to a pointer to a type no cast to it was from.
  The cast itself is defined; a read through it reads one type's bytes as another's.
- `offset_from` of pointers JS keeps in two arrays, as two `repr(C)` fields or two
  `&raw` of one variable are.
- `AtIndex.reach`: a read past the array JS keeps, such as a `repr(C)` neighbour or a
  `Vec`'s old values past a `truncate`.
- A raw or mutable slice of a byte in no array of bytes, a field's (a struct's, a
  tuple's, a capture's, or one an async body keeps across an `.await`), or a local's whose
  address only a call takes, as `ptr::from_mut(&mut b)` does, or a cast of one to
  `*const [u8; N]`. A `Uint8Array(1)` per such byte stays within the byte-buffer form, but
  any `&mut u8` a callee may turn raw would need one.
- `alloc`'s bytes read or written as bytes, or cast to another type than the one it holds.
- `Bytes::slice_ref` of bytes JS copied where Rust shares them, such as a `&str`'s
  `as_bytes`, safe code: a `&str` stays a JS string
  ([#574](https://github.com/moq-dev/rust2/pull/574)), so no answer is exact.
- `ptr::eq` of an array `as_chunks` split from one flat slice, which std's callback gets a
  copy of, answers `false` where Rust answers `true`, silently (moved here 2026-11-07 in
  [#608](https://github.com/moq-dev/rust2/pull/608)). A copy that remembers the slice and
  offset it came from, compared through that, is exact; it touches the code
  [rs.Slice](/quest/m3/ts-slice-object.md) reworks, so it follows that quest.

Comparing `*const [T]` of values JS copies (moved here 2026-11-06 in
[#608](https://github.com/moq-dev/rust2/pull/608)) has no exact path: a `&str`'s
`as_bytes` encodes anew each call, so its refusal stays, the exact answer.

Public API: the translator's output, and which crates translate.

## Waiting

- Maintainer: how to treat the pointers above, which only run time sees leave the
  pointer forms. Refusing at translation every such construct the body can't show fits,
  as the unsafe policy reads, also refuses pointer arithmetic, `offset_from`, raw byte
  slices, and `*const ()` casts back on any pointer a parameter or field holds, which work
  today; whole-program provenance would refuse exactly the reaching ones, but is a large
  analysis. Options: (A) refuse each where the body doesn't show it fits, measured by the
  census of moq-net, kio, and the model; (B) keep these run-time checks as the refusal
  translation can't make, amending AGENTS.md's "never a runtime failure" for unsafe
  pointers from elsewhere; (C) whole-program provenance. Recommended: A for `arcErase` and
  `alloc`'s bytes, whose shown routes are narrow and rare; B for the rest, which A would
  refuse in most unsafe code that takes a pointer; and B for `slice_ref`, safe code with
  no exact answer, rather than a copy that would hide the panic Rust makes on bytes it
  doesn't hold. Check: this section records the choice.

## Related

- [Unsafe code the translator can model](/quest/m3/ts-unsafe/README.md) - the raw-pointer model several of these throws guard; its 2026-11-01 run-time failures now follow this quest's rule
- [rs.Slice](/quest/m3/ts-slice-object.md) - reworks the arrays `as_chunks` lends, which the `ptr::eq` gap follows
