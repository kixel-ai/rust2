# [L] Spike: rust2-lean extracts pure Rust state machines to Lean 4 from MIR

## Goal

A prototype `rust2-lean` backend on `rust2/driver` translates a documented pure subset of post-borrowcheck Rust MIR (structs, enums, fixed-width integers with overflow semantics, `Option`/`Result`/`Poll`, slices/`Vec`/`BTreeMap`, and value-style or non-aliasing `&mut self` step functions) into idiomatic Lean 4, refusing unsupported constructs with source spans and reporting them via `--census`. The spike benchmarks `rust2-lean` head-to-head against Aeneas (Charon MIR -> Lean 4) on the exact same proof obligations of the two pure machines in `kixel-ai/kio`'s [`lab/pure-machine`](https://github.com/kixel-ai/kio/tree/8013885f8c9f53cfe010c0f50d618f530464776c/lab/pure-machine) at the pinned revision `8013885f8c9f53cfe010c0f50d618f530464776c` (`src/warm_settle.rs`'s `WarmSettle` routing machine and `src/send_credit.rs`'s `SendCredit` session). From each extractor it proves at least one nontrivial protocol property directly on the extracted Lean 4 code, including that `SendCredit`'s `step` preserves `WithinCredit`. Verus is a third reference point: it proves that same `WithinCredit` property on the same pinned machine, in the Rust itself with no extraction step. The spike separately reports `--census` and extraction coverage on `moq-net`'s production `Router` (`rs/moq-net/src/route/relay.rs`). One comparison table covers all three on:

- subset coverage and `--census` refusals on real `moq` / `kio` code (`std` collections, iterators, closures, borrows), and for Verus the constructs it refuses on the pinned machine;
- generated Lean 4 readability and proof lines per line of Rust (measuring AI-assisted proof effort), with Verus's proof and annotation lines for `WithinCredit` in the same column;
- toolchain weight and end-to-end check time: extract + `lake build` for the two extractors (pure Rust + Lean 4 vs. Aeneas's OCaml + Charon nightly), and `cargo verus verify` for Verus;
- semantic fidelity verified with differential tests (`#eval` / compiled Lean vs. native Rust);
- whether the same MIR front end can also emit a TLA+ skeleton cheaply.

It ends with a go/no-go verdict: either adopt Aeneas, invest in `rust2-lean` as a real `rust2` target, prove on the Rust with Verus instead, or drop Lean extraction.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` started in `moq-dev/moq` on provable state-machine designs for `kio`):

- ✅ Spike `rustc`-driver MIR extraction to Lean 4 (`rust2-lean`) alongside `kixel-ai/kio`'s state-machine spikes (rejected: skipping MIR extraction, relying on Aeneas alone without comparing). [kixel-ai/kio#26](https://github.com/kixel-ai/kio/pull/26) has since compared kio's spikes: it kept the pure `StateMachine` with its BFS checker, Kani, and Aeneas in `lab/pure-machine`, kept the adversarial `kio::clock::sim` schedules, and deleted `#[kio::reactor]`, whose wakers and locks no extractor takes anyway.
- ✅ Home: `kixel-ai/rust2`, milestone `m2` (research with named consumers: `moq-dev/moq`'s routing and `kixel-ai/kio`'s pure `StateMachine`), reusing `rust2/driver` and its nightly `std`-from-source MIR and `--census` machinery (rejected: duplicating a `rustc_private` driver inside `kixel-ai/kio`, deferring as a note only).
- ✅ Benchmark target: head-to-head comparison against Aeneas on the exact same pinned revisions and proof obligations of the pure `WarmSettle` routing fixture (where TLC takes 20 minutes on 6 relays) and `kio`'s pure `SendCredit` fixture, with production `Router` (`rs/moq-net/src/route/relay.rs`, where `route-algebra/` today maintains a separate hand-written Agda proof checked by 656 test vectors) reported separately for `--census` and extraction coverage (rejected: `SendCredit` alone, `Reconnect` / `Handover`).

Decided 2026-11-27 (maintainer, `/quest-plan` in `kixel-ai/rust2`, following [kixel-ai/kio#26](https://github.com/kixel-ai/kio/pull/26)'s follow-ups):

- ✅ Both extractors read `kixel-ai/kio`'s `lab/pure-machine` at `8013885f8c9f53cfe010c0f50d618f530464776c`, kio's main when this was planned (rejected: moq's `quest/m9/kio-state-machines/pure-machine.md`, deleted at the kio repo split, whose machines landed in kio as [kixel-ai/kio#9](https://github.com/kixel-ai/kio/pull/9)). A pin keeps the comparison like-for-like while kio's machine moves on.
- ✅ Drop the Goal bullet on checking `#[kio::reactor]` action isolation (rejected: keeping it). kio#26 deleted the macro.
- ✅ Add Verus as a third reference point beside Aeneas and `rust2-lean`, proving one property (`SendCredit`'s `WithinCredit`) on the same pinned machine, with its proof lines and check time in the same comparison table (rejected: comparing only Aeneas and `rust2-lean`; dropping the spike). kio already runs Verus in CI (`just verify`, proving `src/log.rs`), and it proves on the Rust with no extraction, so it is the baseline both extractors must beat on proof effort.

Implementation guidance:

- Keep the spike scoped strictly to value-style / single-owner `&mut self` state machines (`fn step(&mut self, event: Event) -> Vec<Effect>` or `fn step(state: State, event: Event) -> (State, Vec<Effect>)`): do not re-implement Aeneas's general aliasing/prophecy LLBC theory. Refuse reborrows, interior mutability, and `unsafe` with clear source spans, following `rust2`'s `AGENTS.md` contract.
- Reuse `rust2/driver` and `rust2-ts`'s MIR reachability and `std`-from-source setup rather than building a second compiler front end. Keep generated Lean 4 small and proof-friendly (plain `structure` / `inductive` / `def`, explicit overflow/panic monad only where Rust can panic, or total functions when proven panic-free).
- Pin Lean 4 through the Nix flake (or a separate flake shell like `moq`'s `nix/route-proof.nix`) so `just ci` does not slow down for unrelated PRs. Match kio's `lab/pure-machine/proofs/lean-toolchain` (`leanprover/lean4:v4.35.0` at the pin) so the theorem statements carry over unchanged.
- The Aeneas side starts from what kio already has at the pin: `lab/pure-machine/prove.sh` (Charon + Aeneas, committed `proofs/PureMachine/{Types,Funs}.lean`) and `proofs/PureMachine/Ranked.lean`, a proved `WarmSettle` ranking lemma (133 non-comment lines for a 45-line function). Reuse those theorem statements for `rust2-lean`. kio#9's Aeneas friction table (matching an enum read through an array, `while` loops reading fields, `Option` equality, `Vec::truncate`) lists what `rust2-lean` must accept to beat Aeneas on coverage.
- The shared obligation for all three is that a successful `Session::step` from a state satisfying `Session::well_formed` and `Session::within_credit` keeps `within_credit`. Verus cannot read the pinned source unannotated: put its contracts in `#[cfg_attr(verus_keep_ghost, ...)]` attributes and its lemmas in `verus!` blocks, as kio's `src/log.rs` does, on a copy whose executable code is otherwise identical to the pin, and count those annotations as proof lines. Pin Verus to kio's `nix/verus.nix` release. Any construct Verus refuses on the machine goes in the coverage row, not a rewrite.
- Report production `Router` `--census` and extraction coverage in its own section so the three-way comparison stays like-for-like.

## Related

- [rust2-ts model epic](/quest/m2/ts-model/README.md) - shares `rust2/driver` and `moq-net`'s model subset
