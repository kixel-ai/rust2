# rust2-lean

Translates a pure subset of a crate's Rust to Lean 4 from rustc's MIR after borrow checking, so a property of the Rust is proved on Lean that reads like it.
It is a spike on the drivers' front end; the kio comparison below is what it was measured on.

```bash
just driver lean --out lean --start-from 'crate::machine::Session::step'   # in the crate
just driver lean --census census.txt --start-from 'crate::route' --manifest-path a/Cargo.toml
```

`--out` writes `Rust.lean`, the runtime every translation imports ([lean/Rust.lean](lean/Rust.lean)), and `<Crate>.lean`: every function a `--start-from` path names or prefixes, and every local function and type they reach.
A construct outside the subset is a compile error at its span, and so is a `--start-from` path that names no function. A method is named under its type's module, wherever its `impl` is written.
`--census` lists every refusal with its span instead, then how many reached functions translated, by kind, then each function and whether it translated.
A closure counts toward the function it is written in, and derived impls and `Debug` impls are not counted, since `deriving` stands in for the first and the second formats.
Only core Lean is needed, with no Mathlib or other package.

## Subset

Accepted:

- Structs and enums with derived or no `PartialEq` and `Ord`, tuples, `bool`, and fixed-width integers; `usize` and `isize` are 64-bit.
- `[T; N]` with a const-generic `N`, as `Vector T N.toNat`; `Vec<T>` and slices as `Array T`; `Option`; `Result<T, E>` as `Except E T`; `String` and `str` as Lean's `String`, which holds UTF-8 exactly; `Arc<T>` as `T`, since it is never written in place; `cmp::Reverse` as `Rust.Reverse`, whose `Ord` is reversed.
- `BTreeMap` and `BTreeSet` as `Rust.BTreeMap` and `Rust.BTreeSet`, their entries in ascending key order, for `new`, `get`, `contains_key`, `contains`, `len`, and `is_empty`, when the key's `Ord` is derived and a lookup's key type is the map's.
- Generic type and const parameters, with `Ord` and `PartialEq` bounds, and a local trait's bound as a Lean class, every method in `Rust.M`.
- `if`, `match`, slice patterns, `let`-`else`, `while`, `loop`, `break`, early `return`, and `?` on `Option`.
- `&T` as the value it borrows, and `&mut` of a whole place, written back where the borrow ends: a function's `&mut` parameters come back after its result.
- Closures that only read what they capture, as arguments to the models below.
- From the sysroot's `core` and `alloc` alone: `Vec::new`, `push`, `len`, `extend_from_slice`, `truncate`, slice indexing by an integer or a range, `len`, `iter`, `windows`, `first`, `last`, and `contains`; `Iterator::map`, `any`, `all`, `copied`, `cloned`, `chain`, `enumerate`, and `collect` into `Vec` or `Arc<[T]>`; `Option::is_some`, `is_none`, `is_some_and`, `map`, `expect`, `unwrap`, `unwrap_or`, `copied`, and `cloned`; `bool::then_some`; `Ord::cmp`, `min`, and `max` on integers; `Clone::clone`; `From` between integers and from `bool`; integer `checked_add`, `checked_sub`, `saturating_add`, and `saturating_sub`, and `+`, `-`, and `*` through a reference; `Arc::new`; and the panics of indexing, slicing, `expect`, and overflow checks.

Refused at the span: recursion, a closure that changes what it captures or is called directly, an iterator read after `any` or `all` consumed part of it, iterator `next` and `for` loops, interior mutability, raw pointers, `Drop` impls, unions, calls that return a `&mut`, a reborrow kept in a local, `PartialEq` that is not structural, an `Ord` that is not derived where a map orders by it, a `clone` that may not copy (a `Clone` impl written by hand, even on a `Copy` type), a trait with associated types or generic methods, recursive types, and any `std` type or function without a model, such as `Duration` or `BTreeMap::get_mut`.

## Translation

- A function that can panic runs in `Rust.M`, `ExceptT String Option`: a panic is `throw` with Rust's message, and `none` is a loop that never ends. One that cannot is plain Lean. An explicit `panic!` is refused, since its formatted message has no model.
- Arithmetic that Rust's overflow checks guard is `+?`, `-?`, and `*?`, which panic as Rust does; the file's header says whether the build checked overflow.
- An index is `unless i < n do throw "index out of bounds"`, then `v[i.toNat]!`, which the check makes total.
- An iterator is the Lean `List` of its items, built eagerly. When a closure that may panic maps it, each item is the `Rust.M` action that makes it, run only when a consumer reaches it, so `any` stops before a later item's panic as Rust's laziness does.
- A closure is a function of its own, `f.closure0` for `f`'s first, taking what it captures as its first argument.
- A type parameter's `clone` is a copy. Each type the crate instantiates one with is checked to copy where it is passed, as structural equality is.
- Each loop is a function of its own over what it carries, a `partial_fixpoint`, so a proof of termination is induction on its equation, separate from the translation.
- Names follow the Rust path: `send_credit::Session::step` is `send_credit.Session.step`, and a method or type whose name a field or Lean already takes gets a `'`. A variable named like a crate-root module gets a number, so it never hides the namespace.

## kio comparison

[compare/](compare) runs the translation on kio's pure `SendCredit` and `WarmSettle` machines, at the revision kio's own Aeneas proof uses: `nix develop .#lean --command just driver lean-kio`.

- It fails if the translation differs from [compare/KioPureMachine.lean](compare/KioPureMachine.lean); `--write` replaces it.
- [compare/Ranked.lean](compare/Ranked.lean) proves kio's `options_rank_below` on it, with the statement kio's Aeneas proof uses.
- [compare/lean_diff.rs](compare/lean_diff.rs) records native Rust's outcomes on every reachable state of kio's seven `SendCredit` configs, sampled, and on random relays and events, panics and their messages included. [compare/Diff.lean](compare/Diff.lean) replays them on the translation and fails on any difference.

## moq comparison

[compare/moq.sh](compare/moq.sh) runs the translation on the route algebra of moq-net's `Router` at the revision its census was taken at, the functions moq-net's `route/tests/algebra.rs` binds to its Agda proof: `nix develop .#lean --command just driver lean-moq`.

- It fails if the translation differs from [compare/MoqNet.lean](compare/MoqNet.lean); `--write` replaces it.
- [compare/moq_diff.rs](compare/moq_diff.rs), copied into moq-net's `route::tests` since what it calls is crate-private, records native Rust's outcomes on random offers, claims, and ports, with costs and path lengths at the bounds `route-algebra/`'s vectors probe. [compare/MoqDiff.lean](compare/MoqDiff.lean) replays them, with an `Item` impl whose `lowered` overflows, and fails on any difference.

## Fixture comparison

[compare/fixture.sh](compare/fixture.sh) replays native Rust's outcomes on the fixture's `cases`, for the models the moq comparison does not reach: `nix develop .#lean --command just driver lean-fixture`.
The fixture's [tests/lean_diff.rs](tests/fixture/tests/lean_diff.rs) records them, and [compare/FixtureDiff.lean](compare/FixtureDiff.lean) replays them.
