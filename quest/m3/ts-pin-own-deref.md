# [S] A Pin of a crate's own Deref type type-checks

## Goal

[rust2-ts](/ts/README.md) translates a `Pin<P>` whose `P` is a crate's own `Deref` type,
such as `Pin::new(OwnPtr(Cell::new(0)))` with `old.get()` called through it, into
TypeScript that type-checks, its deref going through `P`'s own `Deref` impl, proven by
differential tests against native Rust. Today `tsc` rejects it (TS2739), since `Pin<P>`'s
deref is typed as `P` itself.

## Plan

Found on [#621](https://github.com/moq-dev/rust2/pull/621) while checking a review finding.
It is safe Rust, where m3's [unsafe policy](/quest/m3/README.md) puts translation effort,
though no consumer at the moq pin reaches it.

Decided with the maintainer on 2026-11-07: ✅ plan it, in m3 (from #621's follow-ups).

Public API: the translator's output.
