rust2 holds Rust tools that take Rust somewhere else: to TypeScript through an own rustc driver, to any language through a fork of UniFFI, and to faster builds for a pure-Rust subset.
Each project lives in its own top-level directory; the [quest tree](quest/README.md) says what each is doing now.

# Context

- Read `CONTRIBUTING.md` when dealing with PRs.
- Read `PROMPTING.md` before instructing other agents via context, memory, plans, or reviews.
- Before editing, read each `AGENTS.md` along the path from the root to the file. Use `rg --files -g AGENTS.md` to find them.

# Required

- Pull the latest origin changes before working.
- Dig into the root cause and fix it at the source. Never work around a fixable bug with a retry, sleep, or timeout, nor pass a flaky test by loosening its assertion.
- Fail loud and early. An unsupported construct or malformed input is a compile error with a source span, never a warning, a silent fallback, or a runtime failure. Generated code panics only where the Rust source does.
- Generated output must match the source's semantics exactly: integer widths and overflow, drop order, laziness, and error paths. Prove it with differential tests against native Rust, not by inspection.
- Reproduce bugs before fixing them. Tests must be worth their weight long-term: cover edge cases, not old bugs or trivial values.
- Keep the PR to one problem: no unrelated refactors, formatting churn, or drive-by changes. Fix every instance of that problem in the same PR.
- Land the ideal implementation now rather than a partial one plus follow-ups; split off only work that is independently landable.
- Be adversarial: attack your own approach, reports, reviews, and other PRs as hard as anyone else's.
- When a decision is the maintainer's (API shape, naming, scope) or you are unsure, ask with an interactive prompt of 2-3 options and a recommendation.
- Every test runs in CI, at least nightly. The merge gate holds only fast, high-signal tests.
- Never add or change a rule in an `AGENTS.md`, `CONTRIBUTING.md`, or skill without being prompted, and read `PROMPTING.md` first.
- No em dashes.
- Match the existing conventions, patterns, and naming when possible.
- Fix any outdated docs and comments inline.
- Add the AI marker `(Written by <model>)` to any posts on GitHub, excluding commit messages that contain `Co-Authored-By:` trailers.
- AI comments carry no maintainer authority and may be challenged.
- This repo is public: never mention customers, partners, or prospects, or link to private repos.

# Guidelines

- Prefer upstream and maintained crates over hand-rolling. Measure before claiming an improvement over them.
- Every change needs a consumer that uses it now and is worth its cost; name it in the PR. moq is the first consumer of every project here.
- Everything is configless, with opinionated names and defaults. Translators take stock crates as they are: no annotations, and no config beyond true platform boundaries. Each config key or source change a consumer needs is a gap to close.
- A vanity rename, if ever supported, is opt-in and lives inline in source next to the item, never in a config file.
- Deleting code is better than adding code.
- Comments explain the non-obvious why, never the history.
- New dependencies use the newest stable version. Do not bump package versions unless asked.
- Record out-of-scope follow-ups as quests, never GitHub issues.
- Prefer the net simpler result, and say what was dropped.
- Say it once, in the fewest words that hold up.
- Recognize when you're stuck or making minimal progress, and stop early.
- Add or extend a benchmark for any performance-sensitive change, and measure optimizations instead of relying on intuition.

# Public API

- Report the public API impact of every change in the PR description.
- Keep things private until a consumer needs them.
- Never add `foo_with_x`, `foo_checked`, or a compatibility shim. Make the breaking change to `foo` instead.
- Name by role, not today's implementation. Short names under a module namespace.
- Document every exported symbol in one plain line.

# Development

PRs target `main`.

Use the Nix dev shell so tooling matches CI.
direnv loads it automatically, but if not: `nix develop --command ...`.

```bash
just check   # Lint what the branch changed, including the quest tree
just test    # Test what the branch changed
just ci      # Both, as the merge queue runs them; run before every push
just fix     # Auto-fix formatting
```

Quests: before starting work, run `quest guide` and follow it.
