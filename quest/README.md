# rust2 questline

## Goal

Keep the repository's living work organized as visible, versioned quests,
grouped into milestones ordered by priority.

## Plan

m0 holds urgent fixes and what every run pays for. m1 is the work with a consumer
waiting on it now: the exporter moq switches its bindings to, and
[removing CodSpeed](/quest/m1/codspeed/README.md) from moq, rind, and here. m2 is research with a named
consumer: rust2-wasm, moq's browser route, the TypeScript translator, and an experiment
that has to beat it. m3 holds work that waits on evidence or has no consumer yet:
upstream's build-speed levers, which the [build-speed benchmark](/bench/build/README.md)
measures, the translator's features that wait on [the gate](/quest/m2/wasm/README.md),
the binding targets planned ahead of a consumer, and what waits on moq. m4 holds long-term
research.

crabx, prebuilt Rust CLIs without a toolchain, lives in [kixel-ai/crabx](https://github.com/kixel-ai/crabx), with its quests.

Decided with the maintainer on 2026-11-09, in the quest audit, the goal: one Rust code
base callable from any language or framework, with no manual wrappers or annotations.
Speed ranks above API ergonomics, Rust's correctness is kept wherever the target can hold
it, and what is unsupported fails the export, not the run. The
[exporter](/ffi/README.md) is the product, WebAssembly its browser
target, the translator an experiment against that, and build speed a side track that
keeps only what costs nothing.

Decided with the maintainer on 2026-11-23, in kixel-ai/kixel.quest's `/quest-plan`
([kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)): benchmarking
joins the charter. [rust2-bench](/bench/runner/README.md) counts benchmarks' instructions
under stock Valgrind for moq, rind, and here, replacing CodSpeed, and
[a harness](/bench/runner/harness/README.md) replaces Criterion for instruction counts,
though not yet for wall-clock timing.

Decided with the maintainer on 2026-10-23, when rust2 was planned from moq
([moq-dev/moq#6578](https://github.com/moq-dev/moq/pull/6578)):

- One repository with three projects and one setup, copied from moq's reusable
  parts: the nix flake and direnv, just, the quest tree, and agent docs and
  hooks.
- moq is the first consumer of each project, and pins rust2 as a flake input.
- The first quest that adds a crate also adds the Cargo workspace (moq's
  workspace lints, including a reason on every suppression) and the cargo
  steps of `just check`.
