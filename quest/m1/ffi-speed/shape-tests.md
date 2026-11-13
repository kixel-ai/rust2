# [M] Generated type shapes round-trip in every language

## Goal

A generator samples type shapes from [rust2-ffi](/ffi/README.md)'s own IR, in every
position a type can take (parameter, return, field, `Option`, `Vec`, closure, future), and
each language's test checks `digest(echo(make(seed)))` against the value native Rust
computes. A shape a generator mishandles, or one that reaches one of its `unreachable!`
arms, fails the test in that language.

## Plan

Found by the 2026-11-12 audit. Every language already matches native Rust's transcript on
the api fixture (`ffi/tests/fixtures.rs`), but the fixture covers each rule once and each
script is ported by hand, while the generators hold 92 `unreachable!` sites keyed on type
shapes and a new rule touches 19 to 38 files.

- Sample `ir::Type` trees to a bounded depth with a fixed seed, and emit a fixture crate
  of `make_N(seed)`, `echo_N(v)`, and `digest_N(&v)` per shape.
- Each language's driver is a generated flat list of `digest(echo(make(s))) == K`, with
  `K` computed natively, so no driver holds type-specific code.
- The sampled set runs nightly; a small fixed subset joins the gate, and the PR reports
  what it adds to `just ffi test`.
- A shape a generator refuses by design must fail the export, not the test: the sampler
  asks the exporter which shapes export.

Public API: none.
