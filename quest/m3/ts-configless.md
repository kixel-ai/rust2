# [XL] rust2-ts with no ts.toml

## Goal

[rust2-ts](/ts/README.md) translates a crate's whole `pub` API with no `ts.toml`: the
package is the crate it runs on, every `pub` item is a root, a `pub` type's `Display` and
`Debug` come along with it, and no item is bound to hand-written TypeScript.

## Plan

Decided with the maintainer on 2026-11-04: every tool is configless, with opinionated
names and defaults.

- ✅ `roots`, `package`, and `printable` go: the `pub` API is the roots, and printing
  follows the impls. This waits on moq-net translating whole, which the
  [model epic](/quest/m2/ts-model/README.md) drives toward.
- ✅ Each `bind` is a gap to close, not a platform boundary: a bound item becomes a
  translation, or a mapping of that crate built into the runtime as std's are (moq's
  model binds DEFLATE, its clock, and its task sets). Rejected: keeping `bind` as the one
  config key.

Fixtures that name a few roots become crates whose `pub` API is those roots. Split
into an epic if the binds alone outgrow one PR.

Public API: the command line, and what a translation exports.

## Required

- [moq-net's model in TypeScript](/quest/m2/ts-model/README.md) - the model translates with no refusals
