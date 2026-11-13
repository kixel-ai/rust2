# [S] Spike: census the most-downloaded crates

## Goal

A report of what [rust2-ffi](/ffi/README.md)'s `--census` says about the few hundred
most-downloaded library crates on crates.io, exported as they are: how many export whole,
and every refusal ranked by construct across all of them. "Stock crates as they are" is
the claim no other binding generator makes, and it is tested only on moq.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a spike, in m3, since no
consumer waits on it.

- A crate's own root is the facade: export its `pub` API with no stand-in.
- Go: a few constructs explain most refusals, and those become the exporter's roadmap
  past moq. No-go: most refusals are generics and borrows refused by design; then
  `ffi/README.md` says the exporter targets facades written for it.
- Expect noise: the pinned nightly will fail to build some crates, and crates with C
  dependencies need their toolchains. Count those apart from refusals.
- Downloaded crates are untrusted code whose build scripts run: build them in a sandbox
  with no credentials and no network after the fetch.
- The report lands in the PR; nothing else does.

Public API: none.
