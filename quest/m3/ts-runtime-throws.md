# [L] Throw only where Rust panics

## Goal

Code [rust2-ts](/ts/README.md) generates fails at run time only where the Rust source
does: no `rust2-ts:` error the runtime throws where native Rust would go on. Each such
construct translates exactly, through a slower path if it must, or is refused at
translation with a span. Proven by differential tests against native Rust for every
construct that now translates.

## Plan

Decided with the maintainer on 2026-11-04: "we should not panic when Rust would not
panic", and runtime throws that copy Rust's behaviour are fine.

- ✅ A slower exact path, no flag: where one exists it is the translation, with the fast
  path kept where the translator can prove it applies; refuse at translation only where
  none exists. Rejected: refusing with no fallback; an inline opt-in.
- ✅ A throw that guards Rust UB (a read through a null or dangling pointer, `from_raw` of
  a value already dropped, freeing memory `alloc` did not give) stays, as an abort worded
  like std's `unsafe precondition(s) violated` check, since Rust defines nothing there.

The rest are gaps. At `main` they include a panic its hook cannot locate
([#556](https://github.com/moq-dev/rust2/pull/556)), a pointer cast to `()` that is not an
`Arc`'s or `Rc`'s (`rs.arcErase`, [#562](https://github.com/moq-dev/rust2/pull/562),
[#565](https://github.com/moq-dev/rust2/pull/565)), `offset_from` of one Rust allocation
JS keeps in two arrays (such as two pointers made from one reference, one of them through
a branch or loop, which JS keeps each in an array of one of its own; decided 2026-11-07 in
[#621](https://github.com/moq-dev/rust2/pull/621) to throw rather than trace joins), a
pointer moved past the array JS keeps it in (`AtIndex.reach`, such as one kept in a field),
`Bytes::slice_ref` of other storage, `alloc`'s bytes read as values, ranges stepped
without their type, fmt widths written as a placeholder, a read
through a substring's `str::as_ptr` outside its own bytes but inside its text's
(`s[..1].as_ptr().add(1)`, `s[1..].as_ptr().wrapping_sub(1)`), and a raw or mutable slice
of a byte in a field (a struct's, a tuple's, a closure's capture, or one an async body keeps
across an `.await`), or a cast of one to `*const [u8; N]`: a field's byte has no buffer of
its own, unlike a `u8` local whose `&raw` the body takes, so the fix is a `Uint8Array(1)`
for each `u8` field whose address is taken. A `u8` local whose address only a call takes,
as `ptr::from_mut(&mut b)` does, has none either.
The substring reads are UB under Stacked Borrows and allowed under Tree Borrows, so they
throw until Rust settles its memory model (decided 2026-11-07 in
[#634](https://github.com/moq-dev/rust2/pull/634)); if Rust rules them defined, a
substring's pointer carries its text's encoding and offset.
`git grep 'rust2-ts: ' ts/runtime` lists them; classify each before fixing. The pointer
comparisons [#584](https://github.com/moq-dev/rust2/pull/584) refuses at translation, where
JS may make two `rs.Place`s for one place, are out of scope: addresses were their route, and
addresses stay refused (decided 2026-11-07 in [the model epic](/quest/m2/ts-model/README.md)).

✅ Decided with the maintainer on 2026-11-06 in
[moq-dev/rust2#608](https://github.com/moq-dev/rust2/pull/608): comparing `*const [T]` of
values JS copies moved here from rs.Slice's quest. The options
were to wait on rs.String ([moq-dev/rust2#574](https://github.com/moq-dev/rust2/pull/574)) and run only if `&str` shares
its bytes, or to move it here with the other comparisons left refused (picked). Its only
consumer in moq-net is form_urlencoded 1.2.2's `src/lib.rs:417`,
`debug_assert!(ptr::eq(s.as_bytes(), vec))`, of the `&str` that
`String::from_utf8_lossy(&vec)` borrowed. Under
[moq-dev/rust2#574](https://github.com/moq-dev/rust2/pull/574)'s decision a `&str` stays a
JS string, so `as_bytes` makes fresh bytes and today's refusal is exact; comparing by array
and offset would answer `false` where Rust answers `true`. Look for an exact slower path.

✅ Decided with the maintainer on 2026-11-07 in
[moq-dev/rust2#608](https://github.com/moq-dev/rust2/pull/608): `ptr::eq` of an array that
`as_chunks` split from one flat slice, as std lends it to a callback, moved here. Such an
array has no JS array of its own, so `rs.arraySliceItems` gives std a copy of it, and
`ptr::eq` with the array lent elsewhere answers `false` where Rust answers `true`. Arrays
that are each a JS array of their own keep their identity. Look for an exact slower path
with the `*const [T]` comparisons above.

Public API: the translator's output, and which crates translate.
