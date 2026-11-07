# [M] Fields typed by an associated type

## Goal

[rust2-ts](/ts/README.md) translates a generic struct whose field is typed by an
associated type of its parameter, such as smallvec's `Drain<'a, T: Array>` holding
`<T as Array>::Item`, by resolving the projection at each instance, proven by differential
tests against native Rust.

## Plan

Suggested on [#600](https://github.com/moq-dev/rust2/pull/600) (rangemap): it is 1 of
smallvec's 10 model refusals at the moq pin. The rest are `unsafe` that the
[ts-unsafe epic](/quest/m3/ts-unsafe/README.md) owns, and moq drops or binds smallvec, so
this is a language gap rather than a consumer's.

Decided with the maintainer on 2026-11-07: ✅ m3 (rejected: m2, drop), as a core language
gap, not a smallvec-specific one.

Public API: the translator's output.
