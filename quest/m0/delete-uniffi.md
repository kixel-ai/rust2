# [M] Delete the UniFFI fork

## Goal

`uniffi/` and everything that exists only for it are gone: the root Cargo workspace, which
holds only the fork (`sh/dispatch.sh`), the flake's `uniffi*` packages and `.#uniffi`
shell, the nightly `uniffi-*` jobs, and the [binding benchmark](/bench/ffi/README.md)'s
harness, which builds moq-ffi against the fork (`run.sh`, its `fork`, `control`, and `base`
variants, the ubrn browser runner, `pins.toml`, and `bench-ffi.yml`). The benchmark's
README keeps what was measured: the BoltFFI comparison and the fork's generated sizes.
`just ci` and the nightly report their cost before and after.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ delete it now. moq decided
on 2026-11-02 never to build on it ([m1](/quest/m1/README.md#plan)), so its consumers here
were only the benchmark's baseline and CI: 1,426 of the repository's 3,596 tracked files
(118k lines), about 7 minutes of each nightly gate, and nightly jobs of 46 minutes
(fixtures), 11 (install), 14 (C and C++), and 100 to 125 (the benchmark on Linux and
macOS). `UniFFI fixtures` has failed every scheduled run since 2026-11-02, and the
benchmark's Swift job since then too. Rejected: keeping it until moq switches, with a
retirement quest; keeping it as is.

- The baseline no longer matched what moq ships anyway: moq's generator pins moved on
  (go `.2`, dart `.7`, cpp `.7`) from the ones `uniffi/README.md` lists.
- Docs the deletion makes stale change with it: the root `README.md`'s fork bullet,
  nix command, and license note, `AGENTS.md`'s first line ("to any language through a fork
  of UniFFI"), `quest/README.md`'s Plan, and `ffi/README.md`'s first paragraph.
- Rename [the Swift-on-arm wait](/quest/m2/uniffi-swift-arm.md) with `quest mv` to a name
  without `uniffi`, and rewrite its Goal around rust2-ffi's skips, which are all it then
  holds; `ffi/tests/fixtures.rs`, `ffi/justfile`, and `ffi/README.md` cite its path.
- The exporter's own numbers come from
  [its benchmark quest](/quest/m1/ffi-export/bench.md), which rebuilds the harness on
  rust2-ffi.
- moq's `quest/m3/ffi-exporter.md` links quests here that are long deleted
  (`quest/m1/uniffi.md`, `ffi-bytes.md`, `kt-ffi-calls.md`, `ffi-poll-protocol.md`) and
  carries fork-era requirements; report them in the PR for moq's tree to fix.

Public API: the flake's `uniffi*` packages and `.#uniffi` shell are removed (breaking for
anyone pinning them; no consumer does).
