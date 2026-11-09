# [M] A Go API for rust2-go's exported enums

## Goal

Go callers in another package build, match, and pass a `pub enum` that
[rust2-go](/go/README.md) exports: an exported tag, a constant per variant, and a
constructor per variant, with the enum passed whole across every root. The external Go test
package `go/tests/exports`, which [#645](https://github.com/moq-dev/rust2/pull/645) added,
constructs and calls the enum's exports too, so what another package can reach is pinned,
not only spelled.

## Plan

Decided with the maintainer on 2026-11-07: m4 (from
[moq-dev/rust2#614](https://github.com/moq-dev/rust2/pull/614)'s follow-ups). #614 kept
`pub` enums as roots in today's layout, a struct of an unexported `tag` and a field per
variant with fields, which code outside the package can neither build nor match
([its design change](https://github.com/moq-dev/rust2/pull/614#issuecomment-6701655826)).

- Keep the layout translated code uses and export its parts: a tag type with a constant
  per variant (`ShapeCircle`), an exported tag, and a constructor per variant. A root
  wrapper (`Translator::wrapper`) already hands an enum Go holds as its parts across a
  root whole; every root that takes or returns an enum, methods included, goes through it.
- Open: the names of the constants and constructors, which must stay apart from an
  inherent method of the variant's name (`Shape::circle` is `ShapeCircle` today) and from
  other roots. Claim them as roots are claimed, so a clash fails at both spans.
- rust2-ffi's Go API differs: a plain enum is an `int32` type with a constant per
  variant, and one with data a sealed interface of a struct per variant
  ([ffi/README.md](/ffi/README.md#go)). Recommended: keep this quest's struct form, which
  needs no conversion at a root and keeps enums as values; record any difference a Go
  user moving off the cgo bindings would meet in `go/README.md`.

The external test package was the review's suggestion on #614
([its OpenAI review](https://github.com/moq-dev/rust2/pull/614#pullrequestreview-5809902393)),
and #645 landed it as `go/tests/exports/main.go`, run by `exports()` in
`go/tests/fixtures.rs`. Extend it to construct a `pub enum` and call `ShapesArea` and the
other roots that take or return one, so `go vet` and `go test` fail when an export is
unreachable from another package.

Public API: what a translation exports for an enum (breaking for Go code reading the
lowercase `tag`, which only the package itself could).

## Waiting

- Go consumers: one needs moq in pure Go, which [the epic](/quest/m4/go/README.md) is
  parked until; check with the maintainer before starting.
