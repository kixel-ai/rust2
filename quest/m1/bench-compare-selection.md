# [S] rust2-bench compare selects targets per side

## Goal

`rust2-bench compare <base>` works when the base lacks a package or bench target the Cargo arguments name: each side builds only the `-p` and `--bench` selectors its own revision has, prints each it dropped, and compares the benchmarks both sides have, as it already does for benchmarks only one side has.
A selector neither side has still fails.

Non-goal: changing `run`, which builds one revision.

## Plan

Found in [moq-dev/moq#9555](https://github.com/moq-dev/moq/pull/9555): moq's `sh/rs/instructions.sh` passes every package with `-p`, so a compare against a base before a new crate fails that side's build. The runner's README documents the limit today.

Decided 2026-11-24 (maintainer, `/quest-plan`): ✅ filter each side's selectors by that revision's `cargo metadata` (rejected: run a selection command from each revision's own tree, a new mechanism every consumer would supply). m1, since it widens what compare reaches rather than fixing a wrong answer.

Recommendations for the implementer:

- Resolve the selectors the way Cargo does for the forms consumers pass (`-p name`, `--package=name`, `--bench name`), and refuse any form the filter can't resolve rather than passing it through unfiltered.
- Update the runner README's Compare section, which documents the current limit.

Public API: `compare` accepts a selection one side lacks. Wire: none.

## Related

- [One directory for compare](/quest/m0/bench-compare-one-path.md) - also changes how compare builds each side
