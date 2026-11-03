rust2 holds Rust tools that take Rust somewhere else: to TypeScript through an own rustc driver, to any language through a fork of UniFFI, and to faster builds for a pure-Rust subset.
Each project lives in its own top-level directory; the [quest tree](quest/README.md) says what each is doing now.

# Required

- An unsupported construct or malformed input is an error with a source span, never a warning or a silent fallback.
- Generated output must match the source's semantics exactly: integer widths and overflow, drop order, laziness, and error paths. Prove it with differential tests against native Rust, not by inspection.
- Translators take stock crates as they are: no annotations, and no config beyond true platform boundaries. Each config key or source change a consumer needs is a gap to close.
- moq is the first consumer of every project here.
- Prefer upstream and maintained crates over hand-rolling. Measure before claiming an improvement over them.
- A breaking change adds an entry under `## Unreleased` in the affected package's `MIGRATE.md`.

# Development

PRs target `main`.

```bash
just check   # Lint what the branch changed, including the quest tree
just test    # Test what the branch changed
just ci      # Both, as the merge queue runs them; run before every push
just fix     # Auto-fix formatting
```

Quests: before starting work, run `quest context` and `quest guide`, and follow both.
