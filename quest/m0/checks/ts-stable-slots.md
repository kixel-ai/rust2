# [S] A result slot's name stays as translation reaches more

## Goal

Reaching another function that shares a result slot renames neither the slot nor any line
that reads or writes it, and `stable_names` covers it on the `names` fixture.

## Plan

Decided with the maintainer on 2026-11-02: m0, since unstable names churn snapshots and
conflict merges, which this epic exists to cut.

Found while moving `stable_names` to its own fixture on 2026-11-02. A slot of `Ok` parts is
named for the first function in source order among those translation reaches that share
it, so a root reaching an earlier one renames it: with `read` reached, `readOut`; once
`lead`, declared above it with the same payload type, is reached too, every `readOut` in
`read` and its callers becomes `leadOut`. `place_slot` in `ts/src/lower/mir/scalar.rs` picks
the name. Name it by what the slot holds, or by every function of the crate that could
share it as the import census does, rather than by what translation reached.

Public API: the translator's output.
