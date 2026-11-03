# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on.

## Required

- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
- [Translating moq-net panics on a dyn Buf](/quest/m0/ts-moq-dyn-panic.md) - `just ts moq` and `just ts bench` translate moq-net at the pin again
