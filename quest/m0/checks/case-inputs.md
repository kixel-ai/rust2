# [S] A case's inputs stay as cases are added

## Goal

Each case of a differential harness gets generated inputs of its own, seeded from the
case's name, so adding, removing, or reordering a case changes no other case's inputs, and
a new case fails only on its own inputs.

## Plan

Decided with the maintainer on 2026-11-02: m0, since every PR adding a case can now fail on
another case's inputs, costing a fixer run and another gate run.

`ts/tests/differential.ts` draws its random inputs from one seeded stream, and a harness
picks the case by leading bytes modulo its case count: `stock`'s first byte, `lang`'s first
two, and each of `task`'s steps' first. Adding a case shifts which case each random input
reaches, and the full-width values reach only the first eight ops. In
[#436](https://github.com/moq-dev/rust2/pull/436), adding a `stock` case sent the input
`421f00ff...` to `buf::write_all`, which hit the `rt.BytesMut` reserve refusal on a
zero-capacity `take` that no input had reached before. Generate each case's inputs from a
stream seeded by its name, behind the `ops` a harness exports, as `lang`'s caller already
exports one per case.

Public API: none.
