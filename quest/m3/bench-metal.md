# [S] The build benchmark on bare metal, with a dependency hint

## Goal

[The build-speed benchmark](/bench/build/README.md)'s reference report comes from the
bare-metal `nixos` runner, so its "instructions vs base" column holds numbers, and it
measures Cargo's `hint-mostly-unused` profile setting on dependencies.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ in m3, with the other
build-speed levers.

Decided with the maintainer on 2026-11-22, in `/quest-plan` for the follow-up of
[kixel-ai/dotfiles#227](https://github.com/kixel-ai/dotfiles/pull/227), which adds the
`rust2-bench` runner on `nixos` that no queue build runs beside: ✅ the change that moves
`bench.yml` onto `[self-hosted, rust2-bench]` adds a comment beside its `on:` saying it
must never gain a `workflow_call` trigger, since the runner group admits `bench.yml`'s jobs
whichever workflow calls it, so a caller could run its own checkout on the bench host
(rejected: a CI check that fails on the trigger, which the same PR could delete; rejected:
drop it).

Decided with the maintainer on 2026-11-23 in
[kixel-ai/rust2#764](https://github.com/kixel-ai/rust2/pull/764#issuecomment-7112509978):
✅ split setup from measurements (rejected: deferring the setup PR; loosening the
runner's main-only trust). Land the workflow and variant setup first; keep this
quest open for a later measurement and report PR from a `bench.yml` dispatch at
`main`. The runner provisioning wait cleared with dotfiles PR 227.

Decided 2026-11-23 (maintainer, /quest-run): ✅ drop the `embed-metadata` variant
(rejected: replacing it with a Cargo-configured `unstable.embed-metadata=true`
opt-out comparison). Nightly Cargo already defaults to `-Zembed-metadata=no`
since rust-lang/cargo#17267, before the 2026-10-22 pin; moq has nothing to adopt.
`mostly-unused` stays.

The variants and runner wiring are implemented, and the first dispatch at `main`
([run 45323643680](https://github.com/kixel-ai/rust2/actions/runs/45323643680)) is
published in `bench/build/README.md`: the host lease held for each job, the `jobs-frontend=32`
and `mostly-unused` comparisons, and the A/A controls. Its instructions column read
`n/a`: hyperfine 2 reads no counters under `--shell`, which the harness used (the runner's
counters work), and the harness now runs bash as the benchmarked program instead.
Its wall-time noise was 5 to 9% RMS on the controls, above the hosted runners' 1.3 to 1.9%.

Decided 2026-11-24 on `/quest-run`'s recommendation while the maintainer was away, not
the maintainer's decision: ✅ land the `--shell none` fix and the wall and CPU report now
(rejected: holding the PR until instruction counts exist); ✅ don't recommend
`jobs-frontend` for moq's CI, which is shared and CPU-bound, at most an opt-in for local
development, and revisit it with the instruction counts (rejected: recommending it in moq's
`.cargo/config.toml`); ✅ investigate the wall noise only if wall time still matters after
the instruction rerun (rejected: investigating now).

Remaining, after that change lands at `main`:

- Dispatch `bench.yml` at `main` again and check that every row of the new report holds
  instructions. Replace the README's reference table with it, its noise from the controls'
  instructions, and the mold and wild comparison the first run could not settle.
- If the controls' wall time is still as noisy, find what shares the host that the lease
  does not exclude (the CARLA demo, agent sessions, the Nix daemon) and either exclude it
  from the lease or say it in the README.
- Report which winning, cost-free settings moq should use in `.cargo/config.toml`.
  Make no recommendation until the instructions support it; `jobs-frontend=32` is the only
  lever outside the first run's noise, and it trades CPU time for wall time.

Public API: none.

## Hosts

- linux - the `nixos` runner's performance counters
