# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on.

## Required

- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
- [Gate moq-net's translation](/quest/m0/ts-moq-gate.md) - `just ci` translates moq-net at the pin, so a change that breaks it fails in the queue
