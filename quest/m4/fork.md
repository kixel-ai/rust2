# [XL] Waiting: a rustc and cargo fork

## Goal

Whether rust2 should carry its own rustc and cargo fork, installed as a rustup
toolchain so moq needs no code changes. This quest is the evidence ledger.
Later reports append their data points here.

## Plan

- The fork carries its changes itself rather than working through upstream.
- Measure each reason on moq's edits in the
  [build-speed benchmark](/bench/build/README.md).
- Code that follows moq's poll-first and thin-generics rules only gains
  more; it is never required.

### Reasons to fork

Each names what the fork changes, the measured gain on moq, and its source.

- **Early cutoff without positions in dependents' bodies.** Keep panic and
  debuginfo locations out of the generic and inline bodies dependents compile
  (for example, in a side table), keep the SVH and DefIndex stable when the API
  hash holds (as in [rust-lang/rust#158844](https://github.com/rust-lang/rust/issues/158844)),
  check at link that a skipped dependent's undefined symbols still resolve when
  share-generics exports change, and have cargo skip those dependents, including
  under pipelining ([rust-lang/cargo#17439](https://github.com/rust-lang/cargo/issues/17439)).
  Gain: the replay estimates a cutoff holds on about 65 of 250 non-test moq-net
  edits in dev (26%), up from a sound 18 (7%), but on only 7 of 84 async edits
  ([moq-dev/rust2#62](https://github.com/moq-dev/rust2/pull/62)). The
  estimate approximates the side table with `debug = 0` and
  `-Z location-detail=none`, which no differential test covers, so the fork's
  semantics need their own before this counts as sound. Each skip saves at most
  about 56% of the edit: the dependents' share of its critical path, minus the
  final link ([relink](/quest/m3/relink.md)).

### Not reasons

- **A drop-in `RUSTC_WRAPPER` cutoff**, declined on 2026-10-25
  ([moq-dev/rust2#70](https://github.com/moq-dev/rust2/pull/70)):
  - it is unsound on stock rustc: a skipped dependent keeps its dependency's old
    SVH, so the next crate fails with E0460, and its rmeta encodes the
    dependency's DefIndex raw, which shifts;
  - caching, GC, and dedup are already in mbx, and cargo's cross-workspace cache
    goal is in progress;
  - its sound cutoff holds on 7% of moq-net edits
    ([moq-dev/rust2#62](https://github.com/moq-dev/rust2/pull/62)).

Public API: none until the fork has a consumer.

## Waiting

- Maintainer: decide that the reasons to fork above add up to enough to pay for
  maintaining one. An agent can only gather the evidence; the cost of carrying a fork is
  the maintainer's to weigh.
