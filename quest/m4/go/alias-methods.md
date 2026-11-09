# [M] rust2-go exports methods reached only through a specialized alias

## Goal

[rust2-go](/go/README.md) translates a private generic type's `pub` inherent methods that
Go callers reach only through a specialized alias, such as `pub type Byte = Value<u8>`
with `impl Value<u8> { pub fn new }`, rather than refusing the alias at its span.

## Plan

Decided with the maintainer on 2026-11-07: m4 (from
[moq-dev/rust2#614](https://github.com/moq-dev/rust2/pull/614)'s follow-ups). #614's
review rounds 2 to 4 each found an edge case in rooting these, so it narrowed to the
refusal ([its design change](https://github.com/moq-dev/rust2/pull/614#issuecomment-6700047644)).
What those rounds found, each of which this quest handles:

- **Two aliases of one specialization** (`Byte` and `Octet` both `Value<u8>`) share one
  method instance: translate it once and give each alias its own exported wrapper
  (`ByteNew`, `OctetNew`).
- **Impl applicability:** only the impls that apply to the alias's arguments are its, so
  `impl Value<u16>` is not `Byte`'s, and `impl<T: Copy> Value<T>` is only where `T: Copy`
  holds. Decide it through rustc's trait solver (unify the impl's self type with the
  alias's and check its where clauses), not by comparing arguments.
- An alias generic over some of the type's parameters (`pub type Pair<T> = Value<(T, u8)>`)
  roots no generic method, as #614 refuses a generic `pub` function.

Fixtures: `go/fixtures/items` exports two aliases of one specialization and one of another,
with impls that apply to each, to neither, and under a bound, checked by differential
and by exported names; reject loses its specialized-alias case.

Public API: what a translation exports.

## Waiting

- Go consumers: one needs moq in pure Go, which [the epic](/quest/m4/go/README.md) is
  parked until; check with the maintainer before starting.
