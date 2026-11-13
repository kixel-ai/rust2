# [M] The API lock compares every language's names

## Goal

`rust2-ffi --baseline` also fails the export where a binding's own name for a kept item
changes: a Kotlin, Swift, Go, Dart, Python, or C++ class, function, or member name that a
clash rule renames because another item was added. Each failure names the language, the
item, and the old and new name, at the item's span.

## Plan

The lock ([ffi/README.md](/ffi/README.md#the-api-lock)) compares the IR and C's names, so a
C caller never calls a renamed item. The languages rename on their own: `abi::apart` names
`audio::encoder::Finish` `AudioEncoderFinish` alone, but `AudioEncoder_Finish` once
`audio::EncoderFinish` is added beside it, in every language over `abi::classes`, and each
generator's `claim` gives a later member a trailing `_`. Neither reaches C, so the lock passes while a Kotlin or Swift caller stops compiling,
or, with equal signatures, calls the new item.

- Each generator returns its name table, item key to name, beside its output, and the
  lock writes each into `api.json` and compares it as it compares C's.
- A generator renames through `abi::apart`, `abi::claim`, `abi::unique`, or
  `abi::distinct`, so the table can come from those calls rather than a second walk.

Prove it on the lock fixture with a pair of edits that rename a class and a member in one
language and no C name.
