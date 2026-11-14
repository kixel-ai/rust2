# [S] Python annotations survive a method named as a builtin

## Goal

A method named as a Python builtin, such as `bytes`, `int`, or `list`, no longer shadows that builtin in the annotations of later methods in its class: the stub of `Clash` with a `bytes()` method beside `joined(lent: &[u8])` passes `mypy --strict`.

## Plan

- Found by kixel-ai/rust2#777, whose fixture renamed its `Clash::bytes` to `raw` to step around it.
- ✅ Keep this quest (rejected: drop): any crate with a method named like a Python builtin hits it. Maintainer, 2026-11-14.
- ✅ Generated Python refers to a shadowed builtin through an import alias such as `_bytes`, as `_array` is imported (rejected: `builtins.bytes`). Maintainer, 2026-11-14.
- The module header in `ffi/src/python.rs` already imports `_bytes`, `_int`, `_list`, `_float`, `_str`, `_dict`, and `_tuple` from `builtins` for runtime code, but the annotation types (`py_ty` and its kin) still emit bare `bytes`, `int`, `list[...]`. Switch every builtin an annotation names to its alias, adding any missing one (such as `bool` or `object`) to the imports and the locked names, then restore a `bytes` method on a fixture type and type-check it.
