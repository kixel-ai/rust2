# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on.

## Required

- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
- [Every lang case reachable](/quest/m0/lang-cases.md) - the `lang` differential runs every case its harness lists, `borrows::leaks` included
