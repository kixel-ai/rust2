# [S] The stock fixture's target dir race

## Goal

The ts `stock` fixture never fails setting up its build, as it did once in the land fixer
with `writing .../CACHEDIR.TAG: No such file or directory`, and passed on a rerun.

## Plan

Found by the land fixer on 2026-11-02, running #391. Cargo writes `CACHEDIR.TAG` when it
creates a target dir, so the likely cause is two tests, or a test and a recipe, sharing one
target dir while another removes or recreates it. Reproduce it under parallel runs, and fix
the race at its source, never by retrying.

Public API: none.
