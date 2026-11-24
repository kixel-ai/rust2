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

The variants and runner wiring are implemented. Remaining work runs after the
workflow lands at `main`, the only ref the runner group admits:

- Dispatch `bench.yml` at `main` and publish its bare-metal reference report in
  `bench/build/README.md`, including retired instructions and A/A noise controls.
- Compare `jobs-frontend=32` against nightly on the 32-CPU host.
- Measure `mostly-unused` against nightly alongside the other levers. Keep the
  existing toolchain pin.
- Report which winning, cost-free settings moq should use in `.cargo/config.toml`.
  Make no recommendation until the measurements support it.
- Verify the real runner dispatch obtains the host lease and counts instructions.
  The runner hook excludes queue builds and land fixers for the whole job.

Public API: none.

## Hosts

- linux - the `nixos` runner's performance counters
