# [M] Spike: the target language's compiler enforces Rust's rules

## Goal

A report on how many of the run-time checks [rust2-ffi](/ffi/README.md)'s Swift and C++
bindings make become compile errors when the generators emit those languages' own
ownership forms, and what that costs their callers. It generates the stricter forms for
the `api` fixture, lists each test in `ffi/tests/api.swift` and `api.cpp` that breaks a
check today and whether the stricter binding refuses it at compile time, and shows the
fixture's script rewritten against it.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ a spike in m2, whose
report decides whether the generators switch. moq's own wrapper packages sit above the
generated API, so a later break is absorbed there. Rejected: spiking it in m1 before moq
adopts the exporter; not planning it. AGENTS.md lets a binding check at run time only
what its language cannot express, so this finds where that line really is.

- Swift: a handle as a `~Copyable` type with `consuming` methods for `self` and
  `borrowing` parameters makes a call after a move a compile error, where today it traps.
  The cost is a handle that is harder to store in a collection or capture in a closure.
  (`Sendable` conformance by `Sync` is [not-send](/quest/m1/ffi-export/not-send.md)'s.)
- C++: `[[clang::lifetimebound]]` on a returned `view` and `[[clang::lifetime_capture_by]]`
  on what a callable keeps make Clang warn where a view outlives its buffer, today C++'s
  own undefined behavior; `-Wdangling` as an error is the consumer's choice. Swift
  imports the same attributes through its C++ interop
  ([safe interop](https://swift.org/documentation/cxx-interop/safe-interop/)).
- Kotlin, Go, Python, and Dart have no such form; say so in the report rather than look.

Public API: none; the report scopes any later change to the generated Swift and C++.
