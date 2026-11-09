# [M] rust2-go keeps a crate's exported names off the runtime's

## Goal

A crate whose `pub` API takes a name the [rust2-go](/go/README.md) runtime declares
translates, with both under the names Go callers expect: the common
`pub type Result<T> = Result<T, Error>` exports `Result`, and so does a crate's own
`pub struct Formatter`. Today either fails as two items with one Go name.

## Plan

Decided with the maintainer on 2026-11-07: m4 (from
[moq-dev/rust2#614](https://github.com/moq-dev/rust2/pull/614)'s follow-ups).

The runtime (`go/runtime/rt.go`, `rt_fmt.go`) is copied into the crate's package, so its
exported names (`Option`, `Result`, `Ordering`, `RustPanic`, `Formatter`, `Argument`,
`Arguments`, `DebugBuilder`) share the namespace with the crate's exports. #614 reserves
them in `Translator::taken`, so a crate that takes one is refused.

Recommended: the runtime becomes its own Go package that the translation imports, as
rust2-ts imports its runtime as the `rs` namespace, so no crate name can collide with it
by construction. Go needs the importing package's module path for that, which rust2-go
can read from the `go.mod` above `--out` and refuse when there is none, keeping it
configless. Rejected: renaming the crate's item (`Result_`) on a clash, which turns a
common name ugly and needs a rule Go callers must learn; renaming the runtime's types to a
prefix no export produces, which shows in every exported signature.

[moq-dev/rust2#599](https://github.com/moq-dev/rust2/pull/599) gave rust2-ffi's Go
exports a Go package per Rust crate, since Rust's modules may name each other in a cycle
Go's imports forbid, and a runtime package `rt` they share, imported by the module path
above. If rust2-go follows, settle the import path so it serves both.

Fixtures: `go/fixtures/items` exports `pub type Result<T> = Result<T, Error>` and a type
named as another runtime export, and its Go callers use both.

Public API: what a translation exports, and the runtime's package (breaking for Go code
naming `Option` or `Result` from the translated package).

## Waiting

- Go consumers: one needs moq in pure Go, which [the epic](/quest/m4/go/README.md) is
  parked until; check with the maintainer before starting.
