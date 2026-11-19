# [L] Bound instance inference by design

## Goal

`rust2-ffi --census` and `--out` on js-sys 0.3.107's facade finish, as they do on every
other crate of the corpus, and js-sys leaves `KNOWN` in `ffi/tests/fixtures/corpus.rs`.

## Plan

Found by [the corpus](/ffi/tests/fixtures/corpus.rs) on 2026-11-19: js-sys's census ran
two hours without finishing. `Collector::infer` repeats until no round adds an instance,
and each round names every instance's members anew. js-sys's generic types (`Array<T>`,
`Promise<T>`, and the like) are bounded by a trait every JS type implements, so each round
nests one more generic type inside another: 0, 909, 1579, then 2478 instances, naming 0.4M,
5M, 30M, then 94M types, the last round taking four minutes. Nothing bounds the depth but
the count of distinct generic types, since only an instance holding its own type is
refused.

Recommended: bound what a round may add by what the API names rather than by a constant,
such as inferring an instance only where an export's own signature, a type's default, or
a bound's implementor names it, and not where only another inferred instance's member
does; measure moq's census before and after, since its instances must not change. Also
worth measuring: `held_forms` walks every named type on every round, which a set of the
types it has seen would cut.

Public API: none, unless the bound leaves out instances some export names today.
