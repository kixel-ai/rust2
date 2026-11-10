rust2 holds Rust tools that take Rust somewhere else: to any language through an exporter of a crate's `pub` API, which replaces a fork of UniFFI, to TypeScript through an own rustc driver, and to faster builds by measuring upstream's levers.
Each project lives in its own top-level directory; the [quest tree](quest/README.md) says what each is doing now.

# Required

- An unsupported construct or malformed input is a compile error with a source span, never a warning, a silent fallback, or a runtime failure. Generated code panics only where the Rust source does.
  One exception is an `unsafe` pointer from elsewhere whose origin translation can't see, such as a parameter's: a loud run-time check may stand in for the refusal, under m3's [unsafe policy](quest/m3/README.md), since refusing every such pointer would refuse pointer arithmetic on any parameter.
  The other is a binding, which checks at run time only the Rust rules its language cannot express: borrow exclusivity, use after a move or close, and the thread of a value that is not `Send`. A failed check raises the language's own error, or aborts where it has none; a Rust panic aborts.
- Generated output must match the source's semantics exactly: integer widths and overflow, drop order, laziness, and error paths. Prove it with differential tests against native Rust, not by inspection.
- A binding's types are lossless: every value of the Rust type crosses and returns exactly. A native type stands in only where it is exact; otherwise the runtime's own type crosses, with checked conversions to and from the idiomatic one. Speed ranks above ergonomics.
- Everything is configless, with opinionated names and defaults. Translators take stock crates as they are: no annotations, and no config beyond true platform boundaries. Each config key or source change a consumer needs is a gap to close.
- A vanity rename, if ever supported, is opt-in and lives inline in source next to the item, never in a config file.
- moq is the first consumer of every project here; a binding target may be planned before any consumer asks for it.
- Prefer upstream and maintained crates over hand-rolling. Measure before claiming an improvement over them.

# Development

PRs target `main`.

```bash
just check   # Lint what the branch changed, including the quest tree
just test    # Test what the branch changed
just ci      # Both, as the merge queue runs them; run before every push
just fix     # Auto-fix formatting
```

Quests: before any work, run `quest context` and follow it, unless this session already holds its rules.
