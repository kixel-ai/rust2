# [S] Python annotations survive a method named as a builtin

## Goal

A method named as a Python builtin, such as `bytes`, `int`, or `list`, no longer shadows that builtin in the annotations of later methods in its class: the stub of `Clash` with a `bytes()` method beside `joined(lent: &[u8])` passes `mypy --strict`.

## Plan

- Found by kixel-ai/rust2#777, whose fixture renamed its `Clash::bytes` to `raw` to step around it.
- Qualify builtin annotations through `builtins.` (or a `_bytes`-style alias, matching how `_array` is imported), then restore a `bytes` method on a fixture type and type-check it.
