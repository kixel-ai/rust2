# [S] The build benchmark on bare metal, with two more levers

## Goal

[The build-speed benchmark](/bench/build/README.md)'s reference report comes from the
bare-metal `nixos` runner, so its "instructions vs base" column holds numbers, and it
measures two levers already in the pinned nightly that it does not yet: cargo's
`hint-mostly-unused` profile setting on dependencies, and `-Zembed-metadata`.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ in m3, with the other
build-speed levers.

- `bench.yml` runs on hosted `ubuntu-24.04`, which exposes no performance counters, so
  instructions read `n/a` on every row, though `bench/build/README.md` calls them the
  signal on a busy machine.
- On its 4 vCPUs the `--jobs-frontend=8` variant measured oversubscription (check +3.1%,
  build +5.1%, CPU +15.4%), not a verdict for a many-core host. Run it again with jobs
  matched to the cores.
- The queue runner is the same host: schedule the benchmark so it never runs beside a
  queue build, and say how.
- `-Zprofile-hint-mostly-unused` alone changes nothing: it only allows the
  `hint-mostly-unused` profile setting, which the variant must also set, for the
  dependencies and not moq's own crates.
- Add each lever as a variant in [corpus.toml](/bench/build/corpus.toml). The audit
  diffed `rustc -Z help` between the 2026-10-22 pin and the 2026-11-12 nightly and found
  no new build-speed flag, so no pin bump is needed for these.
- Build speed keeps only what costs nothing: a lever that wins is a line for moq's
  `.cargo/config.toml`, reported in the README, not a tool.

Public API: none.

## Hosts

- linux - the `nixos` runner's performance counters
