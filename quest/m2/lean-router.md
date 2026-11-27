# [L] rust2-lean translates moq-net's production Router

## Goal

[rust2-lean](/driver/lean/README.md) translates every function `crate::route::relay` reaches in moq-net's `Router` (`rs/moq-net/src/route/relay.rs`), except `Debug` formatting, to Lean 4 that type-checks with core Lean.
The census at moq 512e3d9 translates 34 of 144 reached functions; it counts closures toward the function they are written in, and leaves out derived and `Debug` impls.
Every function of the route algebra (`Offer`, `Here`, `order`, `Claim`, `claim_order`, `precedence`, `floor_of`) translates, and `just driver lean-moq` replays native Rust's outcomes on it with no mismatch: 100,000 random cases at the bounds of moq-net's `verify/route-algebra/` vectors, and every algebra call behind those 649 checked-in vectors.
Of the 192 refusals left:

- 70 are `std::time::Duration`, and 2 `kio::clock::Instant`.
- 20 are calls that return a `&mut`, mostly `entry(..).or_default()` and `get_mut(..).expect(..)`, and 18 more `&mut` borrows the subset refuses.
- 9 are `VecDeque<Edit>`, and about 20 more are `BTreeMap` and `BTreeSet` functions with no model: `get_mut`, `remove`, `pop_first`, `get`, `get_key_value`, `values`, `keys`, `range`, `iter`, `first`, and `Index`.
- 17 are hand-written `Ord` and `PartialEq` impls (`Path`, `Gone`, and others) where a structural one is required.
- The rest are `Path`'s string internals (`str` indexing, `len`, `is_empty`, `moq_pattern::Parts`), `format!` arguments, tuple `PartialOrd::lt` and `gt`, `Iterator::scan`, `count`, `flatten`, and `flat_map`, `mem::take`, `Option::and_then`, closures called directly, and one recursion.

Each new model is checked by a differential test against native Rust: `lean-moq` for what the route algebra reaches, `lean-fixture` for the rest. Each new refusal has a fixture case.

## Plan

Decided 2026-11-27 (maintainer, /quest-run): ✅ keep this quest with the Plan below as recommended (rejected: drop it), after the Lean spike's verdict to invest in rust2-lean (rejected: adopt Aeneas; drop Lean extraction).

Landing as a sequence of PRs, each standing on its own; the first added the route algebra, closures, iterators, local traits, and the read-only `BTreeMap` and `BTreeSet` models. Next, in order of refusals cleared:

- Model `Duration` as its seconds and nanoseconds, with Rust's panics on overflow, and `kio::clock::Instant` as a `Duration` since an origin.
- Translate a `&mut` returned by `get_mut` or `entry` as a lens, a getter plus setter pair. This keeps value semantics without Aeneas's backward functions. Add `BTreeMap`/`BTreeSet` mutation (`insert`, `remove`, `pop_first`) with it, and `VecDeque` as a list.
- Translate a hand-written `Ord` or `PartialEq` as a call to the local impl, through an instance built from it, instead of requiring a derived one.
- Model `Path` as a Lean `String` in the runtime, diffed against native Rust like every other model.
- Add a `Spec` tactic to the prelude that steps through `Rust.M` binds, as Aeneas's `step*` does. kio's `Ranked.lean` port spent most of its 141 lines driving binds by hand.

`BTreeMap` and `BTreeSet` are sorted association lists over a key whose `Ord` is derived, not Mathlib's finite maps, so translations stay core-only.

Decided 2026-11-27 (maintainer, /quest-run): ✅ iterators are lazy Lean lists (rejected: chains fused into loops, as this Plan first said). A list's items are `Rust.M` actions when a closure that may panic maps them, so a consumer meets a panic only at the item Rust's laziness meets it at, a proof reasons with `List` lemmas instead of a loop's equation, and no closure needs to capture only copies.

Decided 2026-11-27 (maintainer, /quest-run): ✅ a type parameter's `clone` translates as a copy, checked at every instantiation the crate uses and refused with a span otherwise (rejected: refuse it). moq's `Claim<A>` clones `A`.

Decided 2026-11-27 (maintainer, /quest-run): ✅ the differential tests keep the random in-bounds cases and also replay every one of `route-algebra/`'s vectors exactly, pinned to the same moq revision (rejected: random only; vectors only).

Decided 2026-11-27 (maintainer, /quest-run): ✅ model `Path` as a Lean `String` for now (rejected: extend the subset to `Path`'s unsafe UTF-8 handling and formatting now). The long-term goal is no hand-written models, and changing moq to get there is allowed: each hand model this quest adds names the moq change that would retire it, as a follow-up for moq's quest tree. `Path`'s is replacing its unsafe inline `Repr` with a safe representation, such as `Arc<str>`, once a measurement shows what the inline storage saves, so `Path`'s operations translate through the `str` models.

## Related

- [rust2-ts model epic](/quest/m2/ts-model/README.md) - shares `rust2/driver` and moq-net's `Router` as its consumer
