# [S] The glue crate builds with the consumer's release profile

## Goal

The crate rust2-ffi generates builds with the profiles of the workspace it exports.

## Plan

The implementation and measurements are in [PR #752](https://github.com/kixel-ai/rust2/pull/752).
Carry workspace profiles beside patches without forcing panic behavior. Overrides
for packages outside the exported dependency graph are dropped; unknown package
specs fail at their manifest line. Complete local review after settling the decision.

## Waiting

- Maintainer: accept dropping overrides for resolved packages outside the exported dependency graph, instead of refusing them as originally planned. moq's dev profiles name packages its exported library does not build. Recommendation: accept dropping them so exporting moq's workspace succeeds.
