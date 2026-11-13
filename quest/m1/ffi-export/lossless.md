# [L] Lossless binding types

## Goal

Every type a [rust2-ffi](/ffi/README.md) binding takes or returns holds every value of its
Rust type exactly: a value passed in and returned is the same value, and no conversion
truncates, clamps, panics, traps, or aborts. A native type stands in only where it is
exact; otherwise the runtime's own type crosses, with checked conversions to and from the
idiomatic one that fail with the language's error. Proven in every language's `api` test
by the edge values of each type crossing both ways.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ lossless always, with
helpers that convert to and from the idiomatic type, checked. Rejected: idiomatic types
whose silent losses become errors, which still fail at run time on values that do not fit;
today's mappings. AGENTS.md carries the rule.

What the audit found, each a place native Rust would have gone on:

- `Duration`: Python's `timedelta` and Dart's `Duration` truncate to the microsecond and
  clamp (`ffi/runtime/python.rs`, `rt.dart`); Go's `time.Duration` panics on a negative
  one or past 292 years (`ffi_cgo.go`); Swift traps on a negative `Swift.Duration` or one
  past `Duration::MAX` (`rt.swift`); Kotlin's maps past `i64` seconds to `INFINITE`
  and back to `i64::MAX` seconds (`rt.kt`). Seconds and nanoseconds, as std holds them,
  cross exactly. moq's switch also asks that Go stop panicking on a negative one and that
  a duration field default to zero
  ([moq's quest](https://github.com/moq-dev/moq/blob/main/quest/m3/ffi-exporter.md)).
- Socket addresses: Kotlin throws and Go panics on an IPv6 flow label, and Go on a zone it
  cannot look up; `SocketAddrV4` given an IPv6 address aborts in C and C++, panics in Go,
  and throws in Kotlin and Dart. The runtime's own `SocketAddr`, as Swift and Dart have,
  and a C struct per family, hold each exactly.
- Paths: one that is not UTF-8 throws in Kotlin, traps in Swift, and aborts in Dart; Go
  and Python already keep its bytes.
- Text: a Go `string` or C++ view that is not UTF-8 aborts the process (`rt.rs`), and an
  invalid `char` aborts from Kotlin's `Int`, Go's `rune`, and C++'s `char32_t`. A value
  the language's own type allows and Rust's does not is checked in the binding, which
  raises.
- Integers: a Dart `u64` past `i64` reads negative; a Kotlin `usize` past a 32-bit
  target's aborts (`ffi/runtime/jni.rs`'s `narrow`) since
  [#689](https://github.com/kixel-ai/rust2/pull/689); and a Go `*big.Int` outside `u128`
  panics (`ffi_cgo.go`).
- `Hash`: Dart's `hashCode` throws on a `PartialEq` type and Kotlin's returns 0; the IR
  carries no `Hash`. Export it where the type implements it.

Each mapping reports its cost on [the binding benchmark](/bench/ffi/README.md); none
should add a crossing or an allocation, and several remove one. Where a language's range
is the limit and no exact type exists at a call's speed, such as a Python `int` past
`u64` raising `OverflowError`, the raise stays: it rejects a value Rust's type cannot hold.

Public API: every generated API's types for durations, addresses, paths, and text
(breaking), and the runtimes' conversion helpers.

## Required

- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - how a binding raises, which the checked conversions and invalid text use
