# [M] Language names keep module paths apart

## Goal

Every [rust2-ffi](/ffi/README.md) generator gives two items with distinct C names distinct
names of its own, so a facade never re-exports an item under a second name to dodge a
clash, and `just ffi standin` drops `ConnectTargetError`.

## Plan

C keeps each path segment's case, but Kotlin, Swift, Python, Go, and Dart join a type's
segments in Pascal case, which loses where they split: moq-tokio's `ConnectError` and
`connect::Error` are both `TokioConnectError`, so the stand-in re-exports the second as
`ConnectTargetError`. Go's package functions share the namespace with types, so
`clock::wall` takes a trailing `_` beside `clock::Wall`. Each generator either nests a
facade module as its language's namespace (Kotlin and Swift nested objects, Python
submodules, Dart prefixed imports) or fails the export at both spans, as C does on a
residual clash. Measure which moq would hit before choosing.

Public API: the generated names of items in a facade module, in every language but C and
C++.
