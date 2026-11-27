# [L] rust2-lean translates moq-net's production Router

## Goal

[rust2-lean](/driver/lean/README.md) translates every function `crate::route::relay` reaches in moq-net's `Router` (`rs/moq-net/src/route/relay.rs`), except `Debug` formatting, to Lean 4 that type-checks with core Lean.
Today's census at moq 512e3d9 translates 58 of 225 reached functions. Of its 168 refusals:

- 117 are types or functions with no Lean model: `Formatter` (22), `BTreeMap` (19), `cmp::Reverse` (14), `BTreeSet` (10), slice and `Vec` iterators, `Arc<[T]>`, `Ord::cmp`, `Default`, `mem::take`, and `!`.
- 34 are `&mut` borrows the subset refuses: 20 calls that return one, such as `BTreeMap::get_mut`, and 14 reborrows.
- The rest are closures, one recursion, and 8 `PartialEq` impls that are not structural.

Each new model is checked by a differential test against native Rust, as the kio comparison's is. moq's `route-algebra/` test vectors are the first fixture.

## Plan

Decided 2026-11-27 (maintainer, /quest-run): ✅ keep this quest with the Plan below as recommended (rejected: drop it), after the Lean spike's verdict to invest in rust2-lean (rejected: adopt Aeneas; drop Lean extraction).


- Model `BTreeMap` and `BTreeSet` as sorted association lists over a `DecidableLT` key, not Mathlib's finite maps, so translations stay core-only.
- Translate a `&mut` returned by `get_mut` or `entry` as a lens, a getter plus setter pair. This keeps value semantics without Aeneas's backward functions.
- Fuse an iterator chain whose closures capture only copies into a loop, as rust2-ts fuses them. Keep refusing other closures.
- Add a `Spec` tactic to the prelude that steps through `Rust.M` binds, as Aeneas's `step*` does. kio's `Ranked.lean` port spent most of its 141 lines driving binds by hand.

## Related

- [rust2-ts model epic](/quest/m2/ts-model/README.md) - shares `rust2/driver` and moq-net's `Router` as its consumer
