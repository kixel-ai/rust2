# [L] Spike: rust2-lean extracts pure Rust state machines to Lean 4 from MIR

## Goal

A prototype `rust2-lean` backend on `rust2/driver` translates a documented pure subset of post-borrowcheck Rust MIR (structs, enums, fixed-width integers with overflow semantics, `Option`/`Result`/`Poll`, slices/`Vec`/`BTreeMap`, and value-style or non-aliasing `&mut self` step functions) into idiomatic Lean 4, refusing unsupported constructs with source spans and reporting them via `--census`. The spike benchmarks `rust2-lean` head-to-head against Aeneas (Charon MIR -> Lean 4) on the exact same pinned revisions and proof obligations of two pure fixtures (the `WarmSettle` routing machine and `kixel-ai/kio`'s pure `SendCredit` state machine), proving at least one nontrivial protocol property directly on the extracted Lean 4 code from each extractor (such as `WarmSettle` ranking/convergence or `SendCredit`'s `WithinCredit`), and separately reports `--census` and extraction coverage on `moq-net`'s production `Router` (`rs/moq-net/src/route/relay.rs`). It compares both extractors on:

- subset coverage and `--census` refusals on real `moq` / `kio` code (`std` collections, iterators, closures, borrows);
- generated Lean 4 readability and proof lines per line of Rust (measuring AI-assisted proof effort);
- toolchain weight and end-to-end extract + `lake build` check time (pure Rust + Lean 4 vs. Aeneas's OCaml + Charon nightly);
- semantic fidelity verified with differential tests (`#eval` / compiled Lean vs. native Rust);
- whether the same MIR front end can also check `#[kio::reactor]` action isolation or emit a TLA+ skeleton cheaply.

It ends with a go/no-go verdict: either adopt Aeneas, invest in `rust2-lean` as a real `rust2` target, or drop Lean extraction.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` started in `moq-dev/moq` on provable state-machine designs for `kio`):

- ✅ Spike `rustc`-driver MIR extraction to Lean 4 (`rust2-lean`) alongside `kixel-ai/kio`'s three state-machine spikes (pure `StateMachine` + BFS/Kani/Aeneas, `#[kio::reactor]`, and adversarial `kio::clock::sim` + `kio::Tasks`) (rejected: skipping MIR extraction, relying on Aeneas alone without comparing).
- ✅ Home: `kixel-ai/rust2`, milestone `m2` (research with named consumers: `moq-dev/moq`'s routing and `kixel-ai/kio`'s pure `StateMachine`), reusing `rust2/driver` and its nightly `std`-from-source MIR and `--census` machinery (rejected: duplicating a `rustc_private` driver inside `kixel-ai/kio`, deferring as a note only).
- ✅ Benchmark target: head-to-head comparison against Aeneas on the exact same pinned revisions and proof obligations of the pure `WarmSettle` routing fixture (where TLC takes 20 minutes on 6 relays) and `kio`'s pure `SendCredit` fixture, with production `Router` (`rs/moq-net/src/route/relay.rs`, where `route-algebra/` today maintains a separate hand-written Agda proof checked by 656 test vectors) reported separately for `--census` and extraction coverage (rejected: `SendCredit` alone, `Reconnect` / `Handover`).

Implementation guidance:

- Keep the spike scoped strictly to value-style / single-owner `&mut self` state machines (`fn step(&mut self, event: Event) -> Vec<Effect>` or `fn step(state: State, event: Event) -> (State, Vec<Effect>)`): do not re-implement Aeneas's general aliasing/prophecy LLBC theory. Refuse reborrows, interior mutability, and `unsafe` with clear source spans, following `rust2`'s `AGENTS.md` contract.
- Reuse `rust2/driver` and `rust2-ts`'s MIR reachability and `std`-from-source setup rather than building a second compiler front end. Keep generated Lean 4 small and proof-friendly (plain `structure` / `inductive` / `def`, explicit overflow/panic monad only where Rust can panic, or total functions when proven panic-free).
- Pin Lean 4 through the Nix flake (or a separate flake shell like `moq`'s `nix/route-proof.nix`) so `just ci` does not slow down for unrelated PRs.
- Run both `rust2-lean` and Aeneas on the identical pinned revisions of the pure `WarmSettle` and `SendCredit` fixtures with the same Lean theorem statements (coordinating with `kixel-ai/kio`'s `pure-machine` spike, planned in `moq-dev/moq`'s `quest/m9/kio-state-machines/pure-machine.md` and carried to `kixel-ai/kio` at its repo split); report production `Router` `--census` and extraction coverage in its own section so the Aeneas comparison stays like-for-like.

## Related

- [rust2-ts model epic](/quest/m2/ts-model/README.md) - shares `rust2/driver` and `moq-net`'s model subset

## Waiting

- moq-dev/moq: the pure-machine spike (`quest/m9/kio-state-machines/pure-machine.md`, carried to
  `kixel-ai/kio` at its repo split) lands the pure `SendCredit` and `WarmSettle` Rust machines this
  spike extracts, and its Aeneas friction table. On 2026-11-25 moq has only `WarmSettle.tla` and
  that quest is unstarted. Check: the quest is gone from moq's main (or kio's), and the two
  machines exist at a revision both extractors can pin.
