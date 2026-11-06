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
JS keeps in two arrays, `Bytes::slice_ref` of other storage, `alloc`'s bytes read as
values, ranges stepped without their type, and fmt widths written as a placeholder.
`git grep 'rust2-ts: ' ts/runtime` lists them; classify each before fixing. The pointer
comparisons [#584](https://github.com/moq-dev/rust2/pull/584) refuses at translation are
in scope too.

Public API: the translator's output, and which crates translate.

## Related

- [Unsafe code the translator can model](/quest/m3/ts-unsafe.md) - the raw-pointer model several of these throws guard
