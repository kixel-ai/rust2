# Exporter speed and proof, with no API change

## Goal

[rust2-ffi](/ffi/README.md)'s bindings cost less and are proven harder, in ways that change
no generated API: each child cuts a cost [the binding benchmark](/bench/ffi/README.md) or
a measurement in its own Goal shows, measures a cost no benchmark shows yet, or adds a proof
the hand-written fixtures cannot give.
Nothing here holds [moq's pin](/quest/m1/ffi-export/ship.md) open.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ this epic, beside
[the exporter epic](/quest/m1/ffi-export/README.md), which keeps only what breaks the C
ABI or a generated API. Kotlin records,
[returned-object ownership](/quest/m1/ffi-speed/object-ownership.md), and the woken read
moved here from it. Rejected: keeping them there, where three quests that change no API held
the pin open.

Each child's PR shows its change on the benchmark where it has a row through CodSpeed's
comment on the PR, as [the benchmark's README](/bench/ffi/README.md#codspeed) says, and
copies numbers by hand only for what CodSpeed does not count: wall and CPU time, the
contention cases, a language that left CodSpeed, and a measurement of its own.

Decided with the maintainer on 2026-11-12, in `/quest-plan`: ✅ the benchmark's own
measurement work joins this epic, as CodSpeed,
the export's cost (since done), and concurrent calls, since they serve its PRs.
Rejected: standalone m1 quests; holding the last two in m3.
