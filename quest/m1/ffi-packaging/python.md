# [M] Python: typed and source distributions

## Goal

Each wheel carries `py.typed`, so mypy and pyright check callers against the installed
package, and the builder releases an sdist of the generated crate that builds a wheel on
a target no prebuilt wheel covers, such as musllinux, Windows on ARM, and Intel macOS. The
builder checks an install from the sdist against the transcript.

## Plan

Found by the 2026-11-18 language audit: the wheel holds only `__init__.py`, the library,
and metadata (`ffi/release/package.py:185-211`), so type checkers skip it; moq's maturin
release ships an sdist and tests building from it, which the switch would lose.

Public API: the distributions released.
