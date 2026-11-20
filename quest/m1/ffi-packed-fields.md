# [S] Read a packed record's fields without a reference

## Goal

A `#[repr(packed)]` record's field binds: the glue that reads one, as Python's field
getters do, builds, so redox_syscall's `FloatRegisters` binds in every language.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#919](https://github.com/kixel-ai/rust2/pull/919): ✅ kept as planned (rejected: dropping it).

Found by the corpus on 2026-11-20, binding redox_syscall without the items its census
refuses: the Python glue's getter of `FloatRegisters::fcw` is `&(&a_self).fcw`, which rustc
refuses as a reference to a packed struct's field (E0793).

Recommended: reduce it to a packed record in the api fixture, then read a packed record's
field by value, `{ a_self.fcw }`, which a packed struct's fields allow since they are
`Copy`, wherever the glue now borrows one.

Public API: none.
