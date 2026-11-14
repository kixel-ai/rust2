# [L] The binding benchmark on CodSpeed

## Goal

Every single-threaded case of [the binding benchmark](/bench/ffi/README.md), in every language,
reports to
[CodSpeed](https://app.codspeed.io/kixel-ai) under its CPU simulation, so main keeps a history
and each PR that can move a binding's cost gets a comment with its change per case. Today the
numbers exist only in the nightly's summary, on a loaded host, and each
[ffi-speed](/quest/m1/ffi-speed/README.md) child copies its before and after by hand.

## Plan

Decided with the maintainer on 2026-11-12, in `/quest-plan`:

- ✅ Informational (rejected: a required merge gate, nightly history only). CodSpeed's check is
  not required and no threshold blocks a merge, as no rust2 benchmark sets one.
- ✅ The binding benchmark only (rejected: the build benchmark, whose runs take hours and need a
  reset between rounds that `codspeed exec` lacks; the ts benchmark, since CodSpeed drives no
  browser and its Node simulation turns off V8's optimizing tiers; the parked go benchmark).
- ✅ CPU simulation on hosted `ubuntu-24.04` (rejected: walltime on CodSpeed's macro runners;
  walltime on `nixos`, which CodSpeed's tools don't install on, and which hosts the queue
  runner; adding the memory instrument). The repository is public, so hosted runners and
  CodSpeed cost nothing. Simulation counts no system calls, so the woken read's wake is
  undercounted; the nightly report keeps wall and CPU time.
- ✅ Every language, A/A-checked (rejected: compiled languages and Python only; the Rust side
  only, through divan on the C ABI). Each program, Rust's floor and moq-c included, calls
  CodSpeed's [instrument-hooks](https://github.com/CodSpeedHQ/instrument-hooks) through its
  own FFI around each case, one benchmark per language and case, such as `kotlin::write`.
  Run the same commit twice: a language whose numbers move between the two leaves CodSpeed,
  with both runs' numbers in the PR and the reason in the README. JVM, Go, and Dart runtimes
  under Valgrind are the likely movers.
- ✅ Runs on pushes to main, its baseline, and on PRs that change what `just ffi bench`
  reads (rejected: every PR; main and dispatch only): `ffi/`, `bench/ffi/`, the flake, the
  workflow itself, and what `ffi/` takes from `ts/`, the moq pin `ts/justfile` reads from
  `ts/fixtures/model/Cargo.toml` and `ts/rust-toolchain.toml`, which
  `ffi/rust-toolchain.toml` links to. It stays out of the merge gate, in its own workflow,
  with `id-token: write` for CodSpeed's OIDC upload scoped to that job.

Guidance:

- The CodSpeed GitHub App is installed on `kixel-ai`; enable rust2 at
  [app.codspeed.io/kixel-ai](https://app.codspeed.io/kixel-ai) if it isn't, with
  `agent-browser`, and set PR comments to post on change.
- A case is one run under simulation, so drop the program's warm-up repeat only where the
  hooks wrap it, and keep the nightly's wall and CPU tables as they are.
- Running `nix develop` itself under Valgrind costs minutes: enter the bench shell first and
  run CodSpeed's CLI inside it, or show the action's wrapping costs nothing.
- The contention cases, `shared:*` and `own:*`, stay out: simulation runs one thread at a
  time, so it would report a contention number that measures no contention.
- A column added later, such as [the TypeScript one](/quest/m2/wasm/backend.md), joins under
  the same A/A rule.
- Update [the benchmark's README](/bench/ffi/README.md) and the
  [epic's Plan](/quest/m1/ffi-speed/README.md), whose children then point at the CodSpeed
  comment instead of copying numbers.

Public API: none.

## Hosts

- linux - CodSpeed's simulation runs only on Linux
