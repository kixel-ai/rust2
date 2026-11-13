# [S] rust2-lint reports, then moq adopts it or it goes

## Goal

`rust2-lint --report` prints every generic the build instantiates past a share of its
codegen, ranked by cost, with a generic from std or a dependency attributed to the local
function that instantiates it. The report runs on moq, and this quest ends one of two
ways: moq's tree gains a quest to adopt the lint, or the lint, its flake package, and its
line in the root `README.md` are deleted.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a report mode, then a
run on moq that decides. Rejected: deleting it now; leaving it. m0 names code no consumer
uses, and no search finds `rust2-lint` or `mono_budget` in moq.

- Today the lint only fails: a generic whose copies cost more than the budget (50,000 MIR
  statements unless `--mono-budget` or an inline `rust2::mono_budget` attribute says
  otherwise) is an error at the generic. A user has no way to see what is close to the
  budget or what to fix first, and `driver/tests/mono.rs` covers only local generics.
- Check first what an over-budget generic from std or a dependency does today, and add
  the case to the tests: an error at a span the user cannot edit is a bug this quest
  fixes by pointing at the local caller.
- Run the report on moq at the pin and put its top entries in the PR, which then waits on
  the maintainer: adopt, or delete. Recommended: adopt if the report names a generic worth
  fixing in moq's own crates, otherwise delete. Record the answer here or in the deleting
  PR.

Public API: the `--report` flag.
