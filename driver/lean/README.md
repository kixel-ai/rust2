# rust2-lean

Translates a pure subset of a crate's Rust to Lean 4 from rustc's MIR after borrow checking, so a property of the Rust is proved on Lean that reads like it.
It is a spike on the drivers' front end; the kio comparison below is what it was measured on.

```bash
just driver lean --out lean --start-from 'crate::machine::Session::step'   # in the crate
just driver lean --census census.txt --start-from 'crate::route' --manifest-path a/Cargo.toml
```

`--out` writes `Rust.lean`, the runtime every translation imports ([lean/Rust.lean](lean/Rust.lean)), and `<Crate>.lean`: every function a `--start-from` path names or prefixes, and every local function and type they reach.
A construct outside the subset is a compile error at its span, and so is a `--start-from` path that names no function. A method is named under its type's module, wherever its `impl` is written.
`--census` lists every refusal with its span instead, then how many reached functions translated, by kind.
Only core Lean is needed, with no Mathlib or other package.

## Subset

Accepted:

- Structs and enums with derived or no `PartialEq`, tuples, `bool`, and fixed-width integers; `usize` and `isize` are 64-bit.
- `[T; N]` with a const-generic `N`, as `Vector T N.toNat`; `Vec<T>` and slices as `Array T`; `Option`; `Result<T, E>` as `Except E T`.
- Generic type and const parameters.
- `if`, `match`, `let`-`else`, `while`, `loop`, `break`, early `return`, and `?` on `Option`.
- `&T` as the value it borrows, and `&mut` of a whole place, written back where the borrow ends: a function's `&mut` parameters come back after its result.
- From `core`, `alloc`, and `std` alone, matched by crate: `Vec::new`, `push`, `len`, `extend_from_slice`, `truncate`, slice indexing and `len`, `Option::is_some` and `is_none`, `Clone::clone`, unsigned `checked_add` and `checked_sub`, and panics.

Refused at the span: recursion, closures and iterators, interior mutability, raw pointers, `Drop` impls, unions, calls that return a `&mut`, a reborrow kept in a local, `PartialEq` that is not structural, a `clone` that may not copy, and any `std` type or function without a model, such as `BTreeMap`.

## Translation

- A function that can panic runs in `Rust.M`, `ExceptT String Option`: a panic is `throw` with a fixed message naming its kind, and `none` is a loop that never ends. One that cannot is plain Lean. A panic's payload is not kept: a proof needs only where Rust panics, so the differential test checks that both panic, not what they say.
- Arithmetic that Rust's overflow checks guard is `+?`, `-?`, and `*?`, which panic as Rust does; the file's header says whether the build checked overflow.
- An index is `unless i < n do throw "index out of bounds"`, then `v[i.toNat]!`, which the check makes total.
- Each loop is a function of its own over what it carries, a `partial_fixpoint`, so a proof of termination is induction on its equation, separate from the translation.
- Names follow the Rust path: `send_credit::Session::step` is `send_credit.Session.step`, and a method whose name a field or Lean already takes gets a `'`.

## kio comparison

[compare/](compare) runs the translation on kio's pure `SendCredit` and `WarmSettle` machines, at the revision kio's own Aeneas proof uses: `nix develop .#lean --command just driver lean-kio`.

- It fails if the translation differs from [compare/KioPureMachine.lean](compare/KioPureMachine.lean); `--write` replaces it.
- [compare/Ranked.lean](compare/Ranked.lean) proves kio's `options_rank_below` on it, with the statement kio's Aeneas proof uses.
- [compare/lean_diff.rs](compare/lean_diff.rs) records native Rust's outcomes on every reachable state of kio's seven `SendCredit` configs, sampled, and on random relays and events, panics included. [compare/Diff.lean](compare/Diff.lean) replays them on the translation and fails on any difference.
