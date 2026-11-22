# [L] Free-threaded CPython through abi3t

## Goal

The Python binding loads in free-threaded CPython 3.15 and later as an `abi3.abi3t`
extension ([PEP 803](https://peps.python.org/pep-0803/)) whose module slots declare
`Py_mod_gil = Py_MOD_GIL_NOT_USED`, so importing it leaves the GIL disabled, and the
release builder ships that wheel. `tests/api.py` asserts `sys._is_gil_enabled()` is false
after the import. `tests/api.py` runs threaded under 3.15t, and under a ThreadSanitizer-built
interpreter nightly.

## Plan

Decided with the maintainer on 2026-11-18: ✅ m3, since no consumer asks, after the
refusal fixes the crash (rejected: both in m1; no quest). The refusal is in; it checks the
extension suffixes for the free-threaded tag in `_load` (`ffi/src/python.rs`) and
`api_free_threaded` (`ffi/tests/fixtures.rs`) tests it, so this quest narrows both to
pre-3.15 free-threaded builds, which an abi3t library cannot load in.

It needs the `PyModExport` entry point with a `Py_mod_abi` slot ([PEP 793](https://peps.python.org/pep-0793/)),
which also brings the multi-phase init subinterpreters need; per-object data through
PEP 697 instead of embedding the object header, as `Handle` does
(`ffi/runtime/python.rs:1847`); and atomics or critical sections in place of the
`Gil<T>` statics and `Handle.holds` and `closed`. pyo3-ffi supports abi3t
([PyO3 guide](https://pyo3.rs/main/building-and-distribution)).

Public API: the wheels released; the Python API is unchanged.

