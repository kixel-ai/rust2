# Exporter speed and proof, with no API change

## Goal

[rust2-ffi](/ffi/README.md)'s bindings cost less and are proven harder, in ways that change
no generated API: each child cuts a cost [the binding benchmark](/bench/ffi/README.md) or
a measurement in its own Goal shows, or adds a proof the hand-written fixtures cannot give.
Nothing here holds [moq's pin](/quest/m1/ffi-export/ship.md) open.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ this epic, beside
[the exporter epic](/quest/m1/ffi-export/README.md), which keeps only what breaks the C
ABI or a generated API. [Kotlin records](/quest/m1/ffi-speed/kotlin-records.md),
and [returned-object ownership](/quest/m1/ffi-speed/object-ownership.md) moved here from it.
Rejected: keeping work that changes no API there, where it held the pin open.

Each child reports its numbers before and after in its PR, on the benchmark where it has a
row.
