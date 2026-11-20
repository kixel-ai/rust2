# [S] Swift: a span property on Bytes

## Goal

`Bytes` has a `span` property, so a caller passes Rust's own bytes to a call borrowing a
slice as `f(payload.span)` rather than `payload.withSpan { f($0) }`, with no experimental
compiler feature asked of a consumer's build.

## Plan

Found by [Swift spans](https://github.com/kixel-ai/rust2/pull/902), which takes slices as
`Span` and gave `Bytes` the scoped `withSpan`. A property returning a `Span` needs
`@_lifetime(borrow self)`, which Swift 6.4 builds only under
`-enable-experimental-feature Lifetimes`. It also needs `Bytes` addressable for its
dependents (`AddressableTypes`, as Foundation's `Data.span` has), since `Data` keeps up to
14 bytes inline in the value, where a borrow may copy them.

Recommended: wait for both to leave experimental, since a consumer would otherwise add
two experimental flags to its build of the generated file, whose spelling has already
changed once (`@lifetime` to `@_lifetime` in 6.2). Rejected: those flags now.

Public API: adds `Bytes.span`; nothing breaks.

## Waiting

- swiftlang/swift-evolution: lifetime dependencies and addressable types accepted, so a
  release toolchain builds `@_lifetime` and `@_addressableForDependencies` without
  `-enable-experimental-feature`; check by compiling a getter that returns
  `_overrideLifetime(span, borrowing: self)` with plain `swiftc`.
