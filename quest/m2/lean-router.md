# [L] rust2-lean translates moq-net's production Router

## Goal

[rust2-lean](/driver/lean/README.md) translates every function `crate::route::relay` reaches in moq-net's `Router` (`rs/moq-net/src/route/relay.rs`), except `Debug` formatting, to Lean 4 that type-checks with core Lean.
The census at moq 512e3d9 translates 99 of 148 reached functions; it counts closures toward the function they are written in, and leaves out derived and `Debug` impls.
Every function of the route algebra (`Offer`, `Here`, `order`, `Claim`, `claim_order`, `precedence`, `floor_of`) translates, and so does `debounce`, and `just driver lean-moq` replays native Rust's outcomes on them with no mismatch: 100,000 random cases at the bounds of moq-net's `verify/route-algebra/` vectors, and every algebra call behind those 649 checked-in vectors.
Of the 67 refusals left (a closure's own refusal counts apart from its function's):

- 30 are `Option`'s `and_then` (13), `as_ref` (4), `or_else`, `is_none_or`, `map_or`, `filter`, and `as_mut`, `bool::then`, `Vec::is_empty`, `BTreeSet::get`, and `retain` on a `Vec` and a `BTreeMap`.
- 8 are `&mut`s through closures: closures `Option::and_then` passes a `&mut` from `get_mut`, which would return or read through it, a `retain` closure that calls a closure parameter, and a closure that inserts into a set it captures.
- 11 are iterators: an early stop over an integer range's items (4), as `ancestors(path).any(..)` makes, `BTreeMap::range` (3), whose panic on bounds out of order depends on whether the map ever held an entry, `scan`, whose closure writes its state (2), `tier`'s `impl Iterator` parameter, and a flat map over `claims_of`, which falls once `claims_of` translates.
- The rest are closures called directly (6), writing a variant's field in place (3), `format!` arguments and `vec!`'s `Box::new_uninit`, two explicit panics, two promoted constants, one recursion, and a lookup in a set of references by a value.

Each new model is checked by a differential test against native Rust: `lean-moq` for what the route algebra reaches, `lean-fixture` for the rest. Each new refusal has a fixture case.

## Plan

Decided 2026-11-27 (maintainer, /quest-run): ✅ keep this quest with the Plan below as recommended (rejected: drop it), after the Lean spike's verdict to invest in rust2-lean (rejected: adopt Aeneas; drop Lean extraction).

Landing as a sequence of PRs, each standing on its own; the first added the route algebra, closures, iterators, local traits, and the read-only `BTreeMap` and `BTreeSet` models. The second added `Duration`, kio's `Instant`, `VecDeque` as a list, `BTreeMap`/`BTreeSet` mutation through a `&mut` of a whole place (`insert`, `remove`, `pop_first`) and their remaining reads, and comparisons on any derived order. The third translated a `&mut` that `get_mut`, `entry`, or `get_or_insert_default` returns as a lens: the place it was made from, then the value at a key or in a `Some`, read with `valueAt` and written with `setAt` nested into the place's update, so value semantics hold without Aeneas's backward functions. An `Option<&mut V>` or `Entry` is a copy of the value, for a match, holding the lens a `&mut` taken out of it is; the key is the one read when the lens was made, kept in a variable unless nothing can change it. It also added `Default` (a local impl written by hand is called; a type parameter's is refused, since Lean's `Inhabited` need not match), `mem::take` and `replace`, `ok_or`, and `?` on a `Result`. The fourth modeled moq-net's `Path` as the `String` it holds, which cleared every hand-written `Ord` and `PartialEq` refusal, since `Path`'s were the only ones reached; it added `str`'s models, whose index panics name a character as `Debug` shows it from a table `lean-fixture` checks against native Rust, `for` loops and `Iterator::next` over ranges, lists, and iterators written in a translated crate, the collections' `IntoIter` sources, and `rev`, `take_while`, and `count`. The fifth modeled iterator adaptors as list functions: `filter`, `filter_map`, `flat_map`, `flatten`, `peekable` and `peek`, `find`, `find_map`, `sum`, `min`, `min_by_key`, `iter::once`, integer ranges as sources, `collect` into a `BTreeSet`, `extend` of a `Vec` and a `BTreeSet`, `BTreeSet::from` an array, `BTreeMap` methods passed by name, and a translated iterator's `next` driven to a list for `count` and the other consumers that read every item. A lazy list's item is now `Rust.Lazy`, the action that makes none, one, or a run of items, so a filter drops an item only when a consumer reaches it. A consumer that may stop early over an integer range's items is refused, since the list makes every item where Rust steps only as far as it reads. It also gave a generic parameter named like a crate-root item a `''`, which had hidden that item. Next, in order of refusals cleared:

- Model the `Option` and collection methods above.
- Compose a lens through a closure that returns one, as `get_mut(&a).and_then(|m| m.get_mut(&b))` does at 4 sites, by translating the closure to a lens step rather than a function.
- Step an integer range on demand under early-stopping consumers, which clears the 4 refusals `ancestors(path)` makes in `connect` and `watched`: a range source kept as its bounds through the adaptors, stepped one item at a time by `next`, `any`, and the like, rather than refusing every range whose bound the translation cannot see.
- `BTreeMap::range`: moq's two `hops_of` ranges never put their bounds out of order, but the model cannot tell a map that held an entry, whose out-of-order range panics, from a fresh one, whose range is empty. Recommended: a moq follow-up keying `live` and `rx_hops` so a node's hops are one lookup, rather than recording in `Rust.BTreeMap` whether it held an entry, which Lean's structural equality would then compare where Rust's does not.
- `scan`, by translating a closure that takes a `&mut` as one returning what it wrote beside its result.
- Add a `Spec` tactic to the prelude that steps through `Rust.M` binds, as Aeneas's `step*` does. kio's `Ranked.lean` port spent most of its 141 lines driving binds by hand.

`BTreeMap` and `BTreeSet` are sorted association lists over a key whose `Ord` is derived, not Mathlib's finite maps, so translations stay core-only.

Decided 2026-11-27 (maintainer, /quest-run): ✅ kio's `Instant` is modeled as the `u64` of nanoseconds kio keeps (rejected: a `Duration` since an origin, as this Plan first said): a `Duration` holds values an `Instant` cannot, so checked and saturating arithmetic would overflow at the wrong bound. Superseded by the next decision: kio's `Instant` now translates from kio's own MIR, and `lean-moq` checks kio's own methods on it.

Decided 2026-11-27 (maintainer, /quest-run): ✅ plan translating dependency crates' code now ([#1089](https://github.com/kixel-ai/rust2/pull/1089)), so hand models of dependencies such as kio's `Instant` can go (rejected: wait until a second dependency needs a hand model). kio's `Instant` is already plain safe Rust, so no moq change retires it; that quest does.

Decided 2026-11-27 (maintainer, /quest-run): ✅ iterators are lazy Lean lists (rejected: chains fused into loops, as this Plan first said). A list's items are thunks of `Rust.M` actions when a closure that may panic maps them, so a consumer meets a panic only at the item Rust's laziness meets it at, a proof reasons with `List` lemmas instead of a loop's equation, and no closure needs to capture only copies.

Decided 2026-11-27 (maintainer, /quest-run): ✅ a type parameter's `clone` translates as a copy, checked at every instantiation the crate uses and refused with a span otherwise (rejected: refuse it). moq's `Claim<A>` clones `A`.

Decided 2026-11-27 (maintainer, /quest-run): ✅ the differential tests keep the random in-bounds cases and also replay every one of `route-algebra/`'s vectors exactly, pinned to the same moq revision (rejected: random only; vectors only).

Decided 2026-11-27 (maintainer, /quest-run): ✅ model `Path` as a Lean `String` for now (rejected: extend the subset to `Path`'s unsafe UTF-8 handling and formatting now). Landed in [lower/path.rs](/driver/lean/src/lower/path.rs): its retiring moq change is a follow-up for moq's quest tree. The long-term goal is no hand-written models, and changing moq to get there is allowed: each hand model this quest adds names the moq change that would retire it, as a follow-up for moq's quest tree. `Path`'s is replacing its unsafe inline `Repr` with a safe representation, such as `Arc<str>`, once a measurement shows what the inline storage saves, so `Path`'s operations translate through the `str` models.

## Related

- [rust2-ts model epic](/quest/m2/ts-model/README.md) - shares `rust2/driver` and moq-net's `Router` as its consumer
