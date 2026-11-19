# [M] Python: an exception hierarchy, weak references, and a faster import

## Goal

- Each exported error is one base exception with a subclass per variant, so a caller can
  write `except ErrorException.Timeout`, as PyO3 and UniFFI packages do.
- Objects and records can be weakly referenced: `weakref.ref(obj)` works, through
  `__weaklistoffset__` in `Py_tp_members` for objects and a weakref slot for records.
- Records no longer go through `@dataclass` at import: the generator writes `__init__`,
  `__eq__`, `__repr__`, `__match_args__`, and `__dataclass_fields__`, so
  `dataclasses.replace` and `fields` keep working. Report `import moq`'s time before and
  after.
- An await under trio, or any loop the binding does not support, raises an error naming
  asyncio instead of an unrelated weakref `TypeError`.

## Plan

Found by the 2026-11-18 language audit: all 25 of moq's `*Exception` classes subclass
`Exception` directly; `weakref.ref(moq.WaiterList.new())` raises `TypeError`; profiling
`import moq` on CPython 3.14.10 put 58 of 91 ms in processing 163 `@dataclass`
declarations.

Decided with the maintainer on 2026-11-18: ✅ breaking changes land in this epic before
moq pins, as soon as possible (rejected: after the pin).

Public API: breaking for code that catches a variant's exception by its old class.
