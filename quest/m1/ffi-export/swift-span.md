# [M] Swift: lend bytes and numeric slices as Span

## Goal

The Swift binding lends bytes and numeric slices to closures as `Span`/`RawSpan` with
no copy, takes `&[u8]` and numeric `&[T]` parameters as `borrowing Span`, and gives
`Bytes` a `span` accessor, so a frame's payload crosses into Swift without the copy
`lift_lent` makes today (`ffi/src/swift.rs:683-688`). The compiler, not a run-time check,
keeps a lent span from escaping.

## Plan

Found by the 2026-11-18 language audit. Swift 6.2's `Span` back-deploys to iOS 12.2 and
macOS 10.14.4 ([SE-0456](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0456-stdlib-span-properties.md),
[back-deployment](https://forums.swift.org/t/using-span-on-pre-26-apple-os-versions/80513));
`Array.span` needs OS 26, so building spans must not rely on it.

Decided with the maintainer on 2026-11-18: ✅ breaking changes land in this epic before
moq pins, as soon as possible (rejected: after the pin).

First check that generated code can build a span without the experimental Lifetimes
flags; if it cannot, report what it would take and stop. Measure the lent-bytes cases on
[the binding benchmark](/bench/ffi/README.md) before and after.

Public API: breaking for Swift callers of closures that receive lent bytes, and of
functions taking byte or numeric slices.

## Required

- [API surface lock](/quest/m1/ffi-export/api-surface.md) - its CI check shows this PR's break in every language's rendered API
