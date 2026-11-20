# [S] Python: a pub name starting with an underscore passes pyright strict

## Goal

A Rust `pub` item whose name starts with `_`, such as `pub fn _deprecated`, generates a
Python module that pyright's strict mode accepts, and the api fixture exports one.

## Plan

Found on [kixel-ai/rust2#900](https://github.com/kixel-ai/rust2/pull/900): the module
declares each sync function as a stub under `if _typing.TYPE_CHECKING:` and binds it from
the library in the `else:` branch, so pyright reports a stub named `_x_` as
`reportUnusedFunction`, a private function nothing accesses.

Recommendation: list every export in the module's `__all__`, which pyright counts as an
access and which makes `from api import *` reach the Rust `pub` items whose names start
with `_` too, as `pub` says they should.
