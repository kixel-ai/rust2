# [XS] moq adopts rust2-lint, or it goes

## Goal

The report `rust2-lint --report` gives on moq decides the lint's fate: moq's tree gains a
quest to adopt the lint, or the lint, its flake package, its fixtures, and its line in the
root `README.md` are deleted.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a report mode, then a
run on moq that decides. The report mode and the attribution of a std or dependency
generic to its local caller landed with this quest's first PR, which holds the moq run.

On moq `main` at 6b80d19 (`build -p moq-net -p moq-relay`), no generic passes 9% of a
crate's codegen, and the heaviest in moq's own crates is `moq_stats::aggregate::advance`
at 3%. moq-net's top entries are std's BTreeMap internals, each 1-2% across 80-165 key
and value types.

## Waiting

- Maintainer: adopt or delete. Recommended: delete, since the report names no generic
  in moq's own crates worth fixing. Adopting adds a quest in moq's tree instead.
