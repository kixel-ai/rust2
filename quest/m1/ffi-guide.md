# [S] Onboard a crate that is not moq

## Goal

Each refusal kind carries a `help:` note saying how to change the facade, such as
`HashMap` to `BTreeMap` or a borrowed return to an owned one, and `ffi/README.md` links a
short guide to writing a facade for the exporter: what exports, what refuses, and how to
read the census.

## Plan

Found by the 2026-11-18 language audit: refusals are rustc diagnostics at their spans but
say only what is refused; the README's scope note says a stock crate needs a facade, and
nothing shows how to write one. The [corpus](/quest/m1/ffi-proofs/corpus.md)'s failures
are the examples to write the notes against. The "exclude it in the config" suffix goes
with [the config](/quest/m1/ffi-refusals/no-config.md), not here.

Public API: refusal messages.
