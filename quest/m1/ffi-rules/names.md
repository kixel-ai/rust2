# [M] Every binding records each public name in api.json

## Goal

`api.json`'s `names` table holds every name each language's binding gives an item: record
fields, enum variants, data-variant payloads, tuple structs, error wrappers, and trait
members such as `clone`, `default`, and `Display`'s, besides the functions, types, and
constants it holds now. The transcript's emitters in `ffi/tests/fixtures/transcript` read
them from the table and drop their copies of each generator's naming rules.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#885](https://github.com/kixel-ai/rust2/pull/885): ✅ kept as recommended (rejected: dropping it).

Found by [#885](https://github.com/kixel-ai/rust2/pull/885), which writes each language's
`tests/api.*` script from the transcript. Each of its eight emitters repeats some of its
generator's naming rules for what the table leaves out: camel or snake case, reserved-word
escapes, numbered duplicates, upper-case enum values, and which `Pair`/`Triple` a tuple
becomes. Go and Dart record variants; C, C++, Kotlin, Python, Swift, and TypeScript do not.
A copy that drifts from its generator fails the api test rather than hiding, so this is
cleanup, not a correctness gap.

Recommended: record each name where the generator decides it, through `lock::function` and
its siblings, keyed as `field <type key>::<field>`, `variant <type key>::<variant>`, and
`trait <type key>::<trait member>`, so a later rename shows in the lock too.

Public API: none.
