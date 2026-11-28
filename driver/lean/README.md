# rust2-lean

Translates a pure subset of a crate's Rust to Lean 4 from rustc's MIR after borrow checking, so a property of the Rust is proved on Lean that reads like it.
It is a spike on the drivers' front end; the kio comparison below is what it was measured on.

```bash
just driver lean --out lean --start-from 'crate::machine::Session::step'   # in the crate
just driver lean --census census.txt --start-from 'crate::route' --manifest-path a/Cargo.toml
just driver lean --out lean --start-from 'kio::clock::instant::Instant::checked_add'   # a dependency's
```

`--out` writes `Rust.lean`, the runtime every translation imports ([lean/Rust.lean](lean/Rust.lean)), and a `<Crate>.lean` for each crate reached: every function a `--start-from` path names or prefixes, and every function and type they reach outside the sysroot.
A path starts with `crate::` for the package and with a dependency's name for that dependency.
A dependency's functions are translated from its MIR under the same subset, so its own code stands in for a hand model; the sysroot's `std`, `core`, and `alloc` keep the models below.
Its file imports the files of the crates it calls, and names their items under the crate's name from the root namespace, as `_root_.kio.clock.instant.Instant`, so a module of the same name never hides them.
The translation builds every crate in its own target directory, `target/rust2-lean`, with `-Zmir-opt-level=0` and `-Zalways-encode-mir` set on the cargo profile the arguments after `--` pick, `dev` by default, so a dependency's MIR is there and a cached build without it is rebuilt. A dependency only a `--start-from` path names is loaded as if the source named it.
A construct outside the subset is a compile error at its span, in a dependency's source too, and so is a `--start-from` path that names no function. A method is named under its type's module, wherever its `impl` is written.
`--census` lists every refusal with its span instead, then how many reached functions translated, by kind, then each function and whether it translated.
A closure counts toward the function it is written in, and derived impls and `Debug` impls are not counted, since `deriving` stands in for the first and the second formats.
Only core Lean is needed, with no Mathlib or other package.

## Subset

Accepted:

- Structs and enums with derived or no `PartialEq` and `Ord`, tuples, `bool`, `char` as Lean's `Char`, matched by literal and cast as its code point, and fixed-width integers; `usize` and `isize` are 64-bit, and `u128` and `i128` are `Rust.UInt128` and `Rust.Int128` over a `BitVec 128`.
- `[T; N]` with a const-generic `N`, as `Vector T N.toNat`; `Vec<T>` and slices as `Array T`; `Option`; `Result<T, E>` as `Except E T`; `String` and `str` as Lean's `String`, which holds UTF-8 exactly; `Arc<T>` as `T`, since it is never written in place; `cmp::Reverse` as `Rust.Reverse`, whose `Ord` is reversed.
- `BTreeMap` and `BTreeSet` as `Rust.BTreeMap` and `Rust.BTreeSet`, their entries in ascending key order, for `new`, `get`, `get_key_value`, `contains_key`, `contains`, `first`, `len`, `is_empty`, `keys`, `values`, `iter`, indexing, and, through a `&mut` of a whole place, `insert`, `remove`, and `pop_first`, when the key's `Ord` is derived and a lookup's key type is the map's. `get_mut` and `entry` make a `&mut` to the value at a key, as a lens below, and an `Entry` is a copy of that value, for a match on `Vacant` and `Occupied`, `or_default`, `or_insert`, `or_insert_with`, and `VacantEntry::insert`.
- `VecDeque` as `Rust.VecDeque`, its items front first, for `new`, `push_back`, `pop_front`, `is_empty`, `len`, and `iter`.
- `str`'s `len`, `is_empty`, `as_bytes`, `find` and `starts_with` of a `char`, and `starts_with` of a `&str`, and indexing by a range, which panics where Rust's does, with its message: past the end, backwards, or inside a character, which names the character as Rust's `Debug` shows it.
- moq-net's `path::Path` as the `String` it holds ([lower/path.rs](src/lower/path.rs)), since its inline representation reads bytes back as UTF-8 through `unsafe`: `as_str`, `empty`, and `head` are written as Lean that reads the string, and every other method translates through them. Its `PartialEq`, `Ord`, and `Borrow<str>` read that string, so it compares and orders as a `String`, whose order is Rust's byte order on UTF-8, and a map keyed by it is looked up by a `str` too. A safe representation in moq-net, such as an `Arc<str>`, would retire the model.
- `std::time::Duration` as `Rust.Duration`, its seconds and nanoseconds, for `ZERO`, `MAX`, `from_secs`, `from_millis`, `from_nanos`, `as_nanos`, `as_micros`, `as_millis`, and `checked_` and `saturating_` `add`, `sub`, and `mul`. Its constants are built from their fields.
- Generic type and const parameters, with `Ord` and `PartialEq` bounds, and a local trait's bound as a Lean class, every method in `Rust.M`.
- `if`, `match`, slice patterns, `let`-`else`, `while`, `loop`, `break`, early `return`, and `?` on `Option`.
- `for` loops and `Iterator::next` over an integer `Range`, any iterator the models make, and an iterator written in a translated crate, whose `next` is called; `IntoIterator` of an array, a slice, a `Vec`, an `Option`, a `VecDeque`, a `BTreeMap`, or a `BTreeSet`, by value or by reference.
- `&T` as the value it borrows, and `&mut` of a whole place, written back where the borrow ends: a function's `&mut` parameters come back after its result.
- A `&mut` a call returns, from `get_mut`, `entry`, or `Option::get_or_insert_default`, as a lens: the place it was made from, then the value at a key or in a `Some`. It goes through `?`, `ok_or`, `expect`, `unwrap`, and a match, which read a copy of the value it points at.
- `Default::default`, `or_default`, `unwrap_or_default`, `mem::take`, and `mem::replace`, on a `std` type, a type whose `Default` is derived, or a local `Default` impl written by hand, which is called.
- Closures that only read what they capture, and functions, tuple-struct and variant constructors, and modeled `std` functions such as `Duration::from_nanos` passed by name, as arguments to the models below.
- From the sysroot's `core` and `alloc` alone: `Vec::new`, `push`, `len`, `extend_from_slice`, `truncate`, slice indexing by an integer or a range, `len`, `iter`, `windows`, `first`, `last`, and `contains`; `Iterator::map`, `any`, `all`, `copied`, `cloned`, `chain`, `enumerate`, `rev`, `take_while` by a closure that cannot panic over items that cannot, `count`, and `collect` into `Vec` or `Arc<[T]>`; `BTreeMap::into_keys` and `into_values`; `slice::get` by an integer; `Option::is_some`, `is_none`, `is_some_and`, `map`, `expect`, `unwrap`, `unwrap_or`, `ok_or`, `copied`, and `cloned`; `Result::ok`; `?` on a `Result` of the same error type; `bool::then_some`; `Ord::cmp`, `min`, and `max`, and `<`, `<=`, `>`, and `>=` through `PartialOrd`, on a type whose `Ord` is derived; `Clone::clone`; `From` between integers and from `bool`, and `TryFrom` between integers, whose `TryFromIntError` is `Unit`; integer `checked_add`, `checked_sub`, `saturating_add`, and `saturating_sub`, and `+`, `-`, and `*` through a reference; `Arc::new`; and the panics of indexing, slicing, `expect`, and overflow checks.

Refused at the span: recursion, a closure that changes what it captures or is called directly, an iterator read after `any` or `all` consumed part of it, including again in a loop, an iterator from a source the models above do not list, or through an adaptor other than those above, a `..` index that may panic, `next` on an iterator no model makes, such as a `char` range, whose stepping skips the surrogates, a `str` pattern other than a `char` (and a `&str` for `starts_with`), reading moq-net's `Path` representation outside its model, interior mutability, raw pointers, `Drop` impls, unions, a call returning a `&mut` that no lens model above makes, a function or closure returning one, a closure taking one, a value holding one set twice, since it could hold another lens on each path, a reborrow kept in a local, the `Default` of a type parameter, which Lean's `Inhabited` need not match, `PartialEq` that is not structural, an `Ord` that is not derived, or is on an enum whose discriminants do not rise in declaration order or on a `Result`, where a map orders by it, a type passed for a parameter a local trait bounds, other than the caller's own parameter with that bound, since its impl would need a Lean instance, a type passed where `Ord` is bound whose order may not match Lean's, a `clone` that may not copy (a `Clone` impl written by hand, even on a `Copy` type), a trait with associated types or generic methods, recursive types, an order on a type parameter without an `Ord` bound, a reference's `+` in a dependency, whose overflow check std inherits from the calling crate's build, which the translation sees for the package alone, a dependency's function without MIR, and any `std` type or function without a model, such as `u128::count_ones` or `Vec::last_mut`.

## Translation

- A function that can panic runs in `Rust.M`, `ExceptT String Option`: a panic is `throw` with Rust's message, and `none` is a loop that never ends. One that cannot is plain Lean. An explicit `panic!` is refused, since its formatted message has no model.
- Arithmetic that Rust's overflow checks guard is `+?`, `-?`, and `*?`, which panic as Rust does; the package file's header says whether its build checked overflow. A dependency's overflow checks are its own, read from its MIR.
- An index is `unless i < n do throw "index out of bounds"`, then `v[i.toNat]!`, which the check makes total.
- An iterator is the Lean `List` of its items, built eagerly. When a closure that may panic maps it, each item is a thunk, `Unit → Rust.M`, that makes it only when a consumer reaches it, so `any` stops before a later item's panic as Rust's laziness does.
- `next` takes the list's first item, making it then if it is a thunk, and writes the rest back, as a `&mut` argument is; on a `Range`, its bounds, it takes the start. A `for` loop is a loop calling `next`, so its body meets each item as Rust's does.
- A closure is a function of its own, `f.closure0` for `f`'s first, taking what it captures as its first argument.
- A type parameter's `clone` is a copy. Every instantiation in the package, translated or not, or in a dependency's translated function, in a function, constant, inline constant, or static, is checked: one whose type's `clone` may not copy refuses the function at that instantiation's span, and a generic caller passing its own parameter on has its instantiations checked in turn. A dependency's untranslated functions' instantiations are not seen.
- A lens reads the value at its key with `valueAt` and writes it with `setAt`, nested into each write through it, as a write to a field is: `*m.get_mut(&k).unwrap() += 1` is `m := m.setAt k (← m.valueAt k +? 1)`, after `unless (m.get k).isSome do throw ..`. Its key is the one read when it was made: a key that may change in the meantime is kept in a variable.
- Each loop is a function of its own over what it carries, a `partial_fixpoint`, so a proof of termination is induction on its equation, separate from the translation.
- Names follow the Rust path: `send_credit::Session::step` is `send_credit.Session.step`, and a method or type whose name a field or Lean already takes gets a `'`. A variable named like a crate-root module or a dependency gets a number, so it never hides the namespace.

## kio comparison

[compare/](compare) runs the translation on kio's pure `SendCredit` and `WarmSettle` machines, at the revision kio's own Aeneas proof uses: `nix develop .#lean --command just driver lean-kio`.

- It fails if the translation differs from [compare/KioPureMachine.lean](compare/KioPureMachine.lean); `--write` replaces it.
- [compare/Ranked.lean](compare/Ranked.lean) proves kio's `options_rank_below` on it, with the statement kio's Aeneas proof uses.
- [compare/lean_diff.rs](compare/lean_diff.rs) records native Rust's outcomes on every reachable state of kio's seven `SendCredit` configs, sampled, and on random relays and events, panics and their messages included. [compare/Diff.lean](compare/Diff.lean) replays them on the translation and fails on any difference.

## moq comparison

[compare/moq.sh](compare/moq.sh) runs the translation on the route algebra of moq-net's `Router` at the revision its census was taken at, the functions moq-net's `route/tests/algebra.rs` binds to its Agda proof, on `debounce`, the reader's time arithmetic, and on the `Path` methods `head`, `has_prefix`, and `is_empty`: `nix develop .#lean --command just driver lean-moq`.

- It fails if the translation differs from [compare/MoqNet.lean](compare/MoqNet.lean), [compare/MoqPattern.lean](compare/MoqPattern.lean), or [compare/Kio.lean](compare/Kio.lean); `--write` replaces them.
- [compare/moq_diff.rs](compare/moq_diff.rs), copied into moq-net's `route::tests` since what it calls is crate-private, records native Rust's outcomes on random offers, claims, and ports, with costs and path lengths at the bounds `route-algebra/`'s vectors probe, on `Path`'s order, equality, and the methods that read its model on random paths of escaped and multi-byte segments, which reach moq-pattern's segment walk ([compare/MoqPattern.lean](compare/MoqPattern.lean)), and on kio's own `Instant` methods, which it translates from kio to [compare/Kio.lean](compare/Kio.lean). [compare/MoqDiff.lean](compare/MoqDiff.lean) replays them, with an `Item` impl whose `lowered` overflows, and fails on any difference.
- [compare/moq_vectors.rs](compare/moq_vectors.rs), appended to a copy of `algebra.rs` whose algebra calls are renamed to recording ones, checks that its generators still write the proof's checked-in `Vectors.agda` and `ClaimVectors.agda` exactly, and records every algebra call behind them, which `MoqDiff.lean` replays too.

## Fixture comparison

[compare/fixture.sh](compare/fixture.sh) replays native Rust's outcomes on the fixture's `cases`, for the models the moq comparison does not reach, through its own dependencies `fixture_dep` and `kio` for the translation of a dependency: `nix develop .#lean --command just driver lean-fixture`.
The fixture's [tests/lean_diff.rs](tests/fixture/tests/lean_diff.rs) records them, and [compare/FixtureDiff.lean](compare/FixtureDiff.lean) replays them.
It also records which characters native Rust's `Debug` escapes, which the runtime's `debugEscaped` must match, since a `str` index's panic names one.
