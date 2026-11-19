# [XS] Refuse free-threaded CPython at import

## Goal

Importing a generated Python module under a free-threaded CPython (3.13t, 3.14t) raises
`ImportError` naming the build, instead of segfaulting. `tests/api.py` checks it under the
flake's free-threaded interpreter.

## Plan

Found by the 2026-11-18 language audit, reproduced with nixpkgs' `python314FreeThreading`
on a moq export (exit 139): `_load` (`ffi/src/python.rs:255-262`) loads the library by
explicit path through `ExtensionFileLoader`, skipping the filename tag check that keeps a
regular build out of a free-threaded interpreter, and single-phase init gets no ABI check
([PEP 793](https://peps.python.org/pep-0793/), [gh-137210](https://github.com/python/cpython/issues/137210)).
Recommended: check `sysconfig.get_config_var("Py_GIL_DISABLED")` before loading.

Decided with the maintainer on 2026-11-18: ✅ refuse now; real support is
[an abi3t build](/quest/m3/python-abi3t.md) in m3 (rejected: both in m1; the refusal
alone).

Caused by [#407](https://github.com/kixel-ai/rust2/pull/407), which added the loader with
the Python generator.

Public API: none; an import that crashed now raises.
