# [S] rust2-go with no go.toml

## Goal

[rust2-go](/go/README.md) translates a crate's whole `pub` API with no `go.toml`: the
package is the crate's name and every `pub` item is a root.

## Plan

Decided with the maintainer on 2026-11-04: every tool is configless, with opinionated
names and defaults. Follow [rust2-ts's](/quest/m3/ts-configless.md) choices.

Public API: the command line, and what a translation exports.
