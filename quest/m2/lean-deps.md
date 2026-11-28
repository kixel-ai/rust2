# [L] rust2-lean translates dependency crates' code

## Goal

[rust2-lean](/driver/lean/README.md) translates the functions and types a `--start-from` root reaches in the package's non-sysroot dependencies, such as kio, from their MIR under the same subset and refusals as the package's own, instead of matching them by path to a hand-written model.
kio's `Instant` is the first to go: `Rust.kio.Instant` and the driver's `kio_instant` special case are deleted, and `just driver lean-moq` diffs the translation of kio's own `Instant` methods against native Rust with no mismatch, as it diffs the hand model today.
A construct a dependency uses outside the subset is a compile error at the dependency's span, as it is in the package.
The sysroot (`core`, `alloc`, `std`) keeps its hand models: its collections rest on raw pointers and `unsafe` the subset refuses.

## Plan

Decided 2026-11-27 (maintainer, /quest-run, [#1085](https://github.com/kixel-ai/rust2/pull/1085)): ✅ plan this quest now, so hand models such as kio's `Instant` can go (rejected: wait until a second dependency needs a hand model). The long-term goal is no hand-written models; moq may change to get there.

Recommended, open to change by the implementer with a reason in the PR:

- Encode every non-sysroot crate's MIR at `-Zmir-opt-level=0` with `-Zalways-encode-mir` from the same rustc wrapper ([job.rs](/driver/lean/src/job.rs)) that already passes `-Zmir-opt-level=0` to the package, so a dependency's body reads as its source wrote it, as the package's does. The flags change those crates' artifacts, so they build into rust2-lean's own target directory, never the consumer's.
- Name a dependency's items under its crate's name, `kio.clock.Instant`, so they never collide with the package's, which keep their crate-relative names. Write each crate's reached items to its own `<Crate>.lean`, importing the files of the crates it calls, so a proof about kio's items is reused by every package that reaches them.
- Translate a generic dependency function once, generically, as the package's are; the clone-by-copy check then sees the package's instantiations of it, which is what reaches the translation.
- Model `u128` and `i128` in the runtime as a structure over `BitVec 128`, diffed in `lean-fixture` at their bounds: kio's `checked_add` and `checked_sub` go through `Duration::as_nanos` and `u64::try_from`. The same methods also need `Result::ok`, `?` on a `Result` in an `Option` function, a tuple struct's constructor and `Duration::from_nanos` passed as functions, and `unwrap_or_default`, each a model with a `lean-fixture` case.
- Keep a dependency's private items reachable: kio's public methods call `pub(crate)` ones, and a `pub(crate)` constant such as `Instant::MAX` is built from the value tree as a local one is.

Once kio's `Instant` translates, measure what moq-net's `Router` census refuses in its other dependencies, such as `moq_pattern::Parts` under `Path`, and record it in the PR.

## Related

- [rust2-lean translates moq-net's production Router](/quest/m2/lean-router.md) - adds the hand models this quest retires, and shares `lower.rs`
