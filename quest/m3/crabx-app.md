# [M] A GitHub App that sets up crabx's pipeline

## Goal

Installing the App on a repository opens the PR that `crabx init` would make, so a
maintainer opts in with one click and one merge.

## Plan

Decided in [the crabx epic](/quest/m2/crabx/README.md#plan): ✅ the App comes after the
workflow and `crabx init`, in m3. It needs a small webhook server and secrets, the only
hosting this project would run.

The PR adds what `crabx init` writes: [`crabx/src/binaries.yml`](/crabx/src/binaries.yml)
with the repository's default branch filled in.

Decided 2026-11-18 (maintainer, /quest-run): ✅ wait for crabx's first publish, since nothing
can install what the App sets up until then (rejected: start now; delete the App).

## Required

- [First publish](/quest/m2/crabx/first-publish.md) - crabx 0.1.0 on crates.io and npm, so
  the pipeline the App opens a PR for can run
