# [L] Free-threaded CPython through abi3t

## Goal

The Python binding loads in free-threaded CPython 3.15 and later as an `abi3.abi3t`
extension ([PEP 803](https://peps.python.org/pep-0803/)), and the release builder ships
that wheel. `tests/api.py` runs threaded under 3.15t, and under a ThreadSanitizer-built
interpreter nightly.

## Plan

Decided with the maintainer on 2026-11-18: ✅ m3, since no consumer asks, after
[the refusal](/quest/m1/ffi-free-threaded.md) fixes the crash (rejected: both in m1; no
quest).

It needs the `PyModExport` entry point with a `Py_mod_abi` slot ([PEP 793](https://peps.python.org/pep-0793/)),
which also brings the multi-phase init subinterpreters need; per-object data through
PEP 697 instead of embedding the object header, as `Handle` does
(`ffi/runtime/python.rs:1847`); and atomics or critical sections in place of the
`Gil<T>` statics and `Handle.holds` and `closed`. pyo3-ffi supports abi3t
([PyO3 guide](https://pyo3.rs/main/building-and-distribution)).

Public API: the wheels released; the Python API is unchanged.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - the Python binding's API is settled before its runtime is rewritten
- [Refuse free-threaded CPython](/quest/m1/ffi-free-threaded.md) - the crash is fixed first
