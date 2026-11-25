# [XS] The build benchmark on bare metal, with a verdict on mold and wild

## Goal

[The build-speed benchmark](/bench/build/README.md) settles whether the mold and wild
linkers cut wall time on moq's `check`, `build`, and edits, from a run on the bare-metal
`nixos` runner that no other work shares.

## Plan

The 2026-11-24 dispatch at `main`
([run 45450542634](https://github.com/kixel-ai/rust2/actions/runs/45450542634)) filled the
instructions column, measured `mostly-unused` (worse), and repeated the other levers; its
report is in the README. Its `moq stable` job, the only one with mold and wild, ran on a
loaded host: its CPU time per instruction was 1.8 times the nightly job's, and its
baseline `edit-hub` had a 4.95 s standard deviation. Instructions cannot judge a parallel
linker, so the verdict needs wall time from a quiet host.

- Dispatch `bench.yml` at `main` while no agent session, `pullfrog-*` container, or landing
  check runs on `nixos` (the ci-lease excludes the queue containers, land fixers, and the
  Nix daemon only), and check that the stable job's baseline `check` takes about 137 CPU-s
  for 438G instructions, as the nightly job's did. Replace the README's stable rows and its
  loaded-host note, give the stable rows their own run link and date while the current run
  link keeps the nightly and ra-jit rows, and say
  wild's `edit-hub` gain (-37.7% and -17.8% wall in two noisy runs) with its noise from
  the controls.
- If wild or mold wins outside the controls' wall noise, it is a candidate line for moq's
  `.cargo/config.toml`, not yet a recommendation: `nixos` is an x86-64 Ryzen and moq's CI
  runs on aarch64, so the win needs an aarch64 measurement first. No other lever is
  cost-free and clear of the noise.

Public API: none.

## Waiting

- Maintainer: how to get a quiet host for the stable job, which waits 1 to 2 hours for the
  lease and runs 2 hours more. Options: (a) pause agent work on `nixos` for a dispatch;
  (b) extend the ci-lease in dotfiles to freeze or exclude agent sessions and the
  `pullfrog-*` containers while a bench job holds it; (c) drop the verdict, since moq's CI
  runs on aarch64 and the instructions show no linker gain. Recommendation: (a), since
  (b) stalls every agent whenever a bench runs and (c) leaves wild's two hints open.

## Hosts

- linux - the `nixos` runner's performance counters
