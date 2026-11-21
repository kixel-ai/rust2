# [S] Read a packed record's fields without a reference

## Goal

A `#[repr(packed)]` record's field binds: the glue that reads one, as Python's field
getters do, builds, so redox_syscall's `FloatRegisters` binds in every language.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#919](https://github.com/kixel-ai/rust2/pull/919): ✅ kept as planned (rejected: dropping it).

Found by the corpus on 2026-11-20, binding redox_syscall without the items its census
refuses: the Python glue's getter of `FloatRegisters::fcw` is `&(&a_self).fcw`, which rustc
refuses as a reference to a packed struct's field (E0793).

Recommended: reduce it to a packed record in the api fixture, with a `Copy` field and one
that is not, such as a `String`, since `repr(packed)` makes no field `Copy`. Wherever the
glue now borrows a packed field, copy a `Copy` one out by value, `{ a_self.fcw }`, and read
any other unaligned into a `ManuallyDrop` to take what the getter takes from it, as its
clone, so the record still owns and drops the original. A getter that lends the field, as
`Collector::accessors` lends a non-`Clone` object field from the receiver's handle, has no
copy to lend from: it lends the field itself only where rustc allows the reference, a field
aligned to 1, and refuses any other with the field's span.

Public API: none.
