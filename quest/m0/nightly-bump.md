# [M] Bump the rustc nightly, on a schedule

## Goal

The nightly every driver here links (`ts/rust-toolchain.toml`, which `driver/`, `ffi/`,
and `go/` symlink) moves to a current one, with `flake.lock`'s rust-overlay bumped to name
it, and a nightly canary job, which fails no one's merge, builds each driver against the
newest nightly so the next bump's breaks are known before it starts. This quest recurs:
the PR that finishes a run creates the next.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a recurring bump, in m0.
The pin is `nightly-2026-10-15` and has not moved since the repository's first commit,
`.github/dependabot.yml` covers only nix and GitHub Actions, and three quests wait on a
newer nightly: [the FMA allowance](/quest/m3/ts-fma-allowance.md),
[relink](/quest/m3/relink.md), and [TPDE](/quest/m3/tpde.md).

- Each run: bump the toolchain file and the overlay, port every driver to rustc's changed
  internals, and pass `just gate`. Differential output must not change; a fixture whose
  expected output moves is a finding to explain in the PR, not to regenerate.
- The first run adds the canary to `nightly.yml`: it overrides the toolchain to the
  newest nightly the overlay names and builds `driver/`, `ffi/`, `ts/`, and `go/`. It is
  allowed to fail, and opens no issue.
- `go/` is parked with no consumer ([its epic](/quest/m4/go/README.md)), and its roughly
  5,600 lines over rustc's internals are ported with the rest. Report what that port cost in the
  PR, so the maintainer can weigh keeping it.
- The benchmark's own nightly in `flake.nix` is a corpus change that lands on its own
  (`bench/build/corpus.toml`); it is [TPDE](/quest/m3/tpde.md)'s to bump, not this
  quest's.
- Recurrence: the finishing PR re-creates this file at the same path with a `Scheduled`
  date six weeks later, one Rust release cycle, and carries forward what the run learned:
  which internals moved and what the canary showed. Chosen by the audit without asking;
  the period is the maintainer's to change. Nothing ends the recurrence while a driver
  here links `rustc_private`.

Public API: none.
