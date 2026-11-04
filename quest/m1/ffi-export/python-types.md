# [S] Generated Python type-checks

## Goal

The Python that [rust2-ffi](/ffi/README.md) generates passes mypy and pyright, checked in
CI on the api fixture and moq's export, so a typed consumer imports it
with no ignores.

## Plan

On `main` it fails both, at least on: classes based on the untyped `_n.Object`, enum
variants assigned as class attributes, `__hash__ = None` on unhashable classes, and the
extension loader's module spec. Fix the generator, not the checks; a construct no stub can
type is a gap in the generator's output, not a reason to ignore it.

Public API: the generated Python's types, not its behaviour.
