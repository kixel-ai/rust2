# [S] Onboard a crate that is not moq

## Goal

Each refusal kind carries a `help:` note saying how to change the facade, such as
`HashMap` to `BTreeMap` or a borrowed return to an owned one, and `ffi/README.md` links a
short guide to writing a facade for the exporter: what exports, what refuses, and how to
read the census.

## Plan

Found by the 2026-11-18 language audit: refusals are rustc diagnostics at their spans but
say only what is refused; the README's scope note says a stock crate needs a facade, and
nothing shows how to write one. The nightly corpus's refusals (`just ffi corpus`, whose
report lists each crate's) are the examples to write the notes against. The "exclude it in the config" suffix went
with the config, in [#903](https://github.com/kixel-ai/rust2/pull/903).

Public API: refusal messages.
