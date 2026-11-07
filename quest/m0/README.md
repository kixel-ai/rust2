# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on.

## Required

- [TypeId snapshots alike on every host](/quest/m0/typeid-snapshots.md) - the `dyns` and `panics/hook` snapshots match on every host, so local `just ci` passes on main
- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
