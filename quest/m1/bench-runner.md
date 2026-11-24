# [M] Count benchmark instructions with stock Valgrind, without CodSpeed

## Goal

rust2 ships a benchmark runner, as a flake package and installable with `cargo install --git` for repositories without Nix, that builds a repository's benchmarks once and runs each benchmark in its own stock Valgrind (Callgrind) process, in parallel across the host's cores.
It writes each benchmark's instruction count (`Ir`) as a Kixel Quest `kixel-metrics` `metrics.json`, takes a list of benchmark names, one per line, to run only those, and compares a base commit against the working tree locally.

rust2's own [binding benchmark](/bench/ffi/README.md) moves to it in the same PR: `.github/workflows/codspeed.yml` drops the CodSpeed Action, its pull request trigger, its per-push runs, and `id-token: write`, and runs nightly on `main` plus on `workflow_dispatch` with a `benchmarks` input, uploading `kixel-metrics`.
The PR also adds benchmarking to rust2's charter: the first line of `AGENTS.md`, `README.md`, and the [questline](/quest/README.md)'s Plan.

Non-goals: wall-clock or memory measurement, macOS and Windows, and changing how benchmarks are written, which [the harness](/quest/m2/harness/README.md) does.

## Plan

Decided 2026-11-23 (maintainer, `/quest-plan` in kixel-ai/kixel.quest, [kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113), "is there anything like codspeed that we could run locally instead of using them as a 3rd party provider?"):

- ✅ Replace CodSpeed in moq-dev/moq, kixel-ai/rind, and here with stock Valgrind through our own runner, reporting only to Kixel Quest and running on any Linux host, hosted CI, self-hosted, or an agent's own (rejected: moq only for now; keeping CodSpeed). Why: no third-party service in the loop, no upload limits (the reason moq and rind split their benchmarks into shards), no OIDC token, and an agent measures a performance claim locally, comparing base and head on its own host. CodSpeed's CLI also supports only Ubuntu and Debian, not our NixOS and Fedora hosts.
- ✅ Our own thin runner over the markers the benchmarks already emit, here in rust2 (rejected: rewriting the benchmarks to Gungraun 0.20, about 875 KB of them in moq alone, which also can't cover this repository's C, Node, and Python harness; only our own harness). The benchmarks keep CodSpeed's open-source compatibility crates and instrument hooks as ordinary libraries until [the harness](/quest/m2/harness/README.md) replaces them.
- ✅ No benchmark runs on pull requests, since none gates (rejected: per-PR base-vs-head tables in the job summary, which would replace CodSpeed's PR comments here). Agents compare locally.
- ✅ Nightly runs on `main` plus Kixel Quest's bisect, which dispatches the workflow with the regressed benchmarks' names ([kixel-ai/kixel.quest#113](https://github.com/kixel-ai/kixel.quest/pull/113)'s targeted bisect) (rejected: a coalesced run on each landing). The maintainer: "Bisecting is always going to be cheaper because it's logN and can target a specific benchmark."
- ✅ GitHub-hosted runners, free for these public repositories (rejected: self-hosted, whose cores agents and moq's queue share).

The markers: CodSpeed's instrument hooks issue Callgrind client requests around each benchmark, `ZERO_STATS` and `START_INSTRUMENTATION` before it, then `STOP_INSTRUMENTATION` and `DUMP_STATS_AT(uri)`. Checked 2026-11-23 with nixpkgs' Valgrind 3.27.1: run with `--instr-atstart=no`, it writes one dump per benchmark, its `desc: Trigger: Client Request: <uri>` naming it and its `totals:` line holding the cost, plus a `Program termination` dump. `ADD_OBJ_SKIP` and `ADD_DESC` exist only in CodSpeed's Valgrind fork; stock Valgrind warns about each and goes on.

Recommendations for the implementer:

- Pin Valgrind through the flake, so an upgrade, which can shift counts, is a visible pin bump that starts new series through the fingerprint.
- Build the benchmarks without the `cargo-codspeed` binary: set what it sets for the compatibility crates' instrumented mode yourself.
- Read `totals:`, not `summary:`: both CodSpeed's fork and stock Valgrind write `summary: 0` on each part. Fail on a dump with an unknown trigger, a benchmark without `totals`, or one with zero `Ir`; [moq-dev/moq#9472](https://github.com/moq-dev/moq/pull/9472)'s `sh/gh/bench-report.py` parses the same dumps and its tests list those cases.
- A benchmark name the filter lists but no benchmark matches fails the run, so a bisect step never reads a suspect as clean.
- Check that the instrument hooks pick their Valgrind instrument with no CodSpeed runner present: they may look for its FIFO first.
- Counts compare only within one environment: Valgrind presents a CPU derived from the host's, so a binary that picks a path by CPU feature (glibc's string functions, Rust's runtime detection), or one built for another architecture, counts differently. Hosted `ubuntu-24.04` mixes Intel and AMD hosts, and [CodSpeed measured](https://codspeed.io/blog/why-glibc-faster-github-actions) different counts from identical binaries across them. So the runner fingerprints its environment, the architecture, the CPUID Valgrind presents (read by a probe run under the same Valgrind), the effective `GLIBC_TUNABLES`, which can change the CPU features glibc picks under the same CPUID, and the Valgrind version, and names it in each measure, so counts from two environments never share a Kixel Quest series and a hardware change starts a new series instead of reading as a regression. A local compare refuses base and head measured under different fingerprints, and a bisect step that lands on another fingerprint leaves the suspect's series unmeasured at that commit, which Kixel Quest's bisect must treat as a step to retry, not a clean one; check that it does. Measure on hosted runners how many fingerprints occur; if more than one does, propose pinning one, such as with `GLIBC_TUNABLES` hardware-capability masks set by the runner, rather than splitting every series.
- moq's repeat runs differed by up to 2.57% on multi-threaded cases; measure the spread here over repeated runs and report it in the PR.

Public API: a new CLI. Wire: none.

## Related

- [The build-speed benchmark on bare metal](/quest/m3/bench-metal.md) - what remains of it, a verdict on mold and wild from a quiet `nixos` runner
