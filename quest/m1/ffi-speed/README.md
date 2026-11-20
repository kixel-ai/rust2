# Cut the bindings' crossing costs

## Goal

The per-call costs [the binding benchmark](/bench/ffi/README.md) shows above C's, where the
2026-11-18 language audit found a known lever, are cut and measured. It completes when
every child has merged.

## Plan

Decided with the maintainer on 2026-11-18, in the per-language audit: ✅ all of the speed
findings are kept (rejected: any subset). The zero-copy buffer findings change generated
APIs, so they are children of [the exporter epic](/quest/m1/ffi-export/README.md) instead:
[C++](/quest/m1/ffi-export/cpp-idioms.md), Swift (landed), and
Kotlin (landed). This epic changes no generated API and
does not hold moq's pin.

Considered and rejected by the audit: Go's `#cgo noescape` and `nocallback`, since
dropping a Go-implemented trait or closure calls back into Go from almost any call;
purego, which still goes through `runtime.cgocall`; and cross-language LTO in C, worth
about 1 ns against Clang and LLVM version coupling.
