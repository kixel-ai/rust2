rust2 holds Rust tools that take Rust somewhere else: to any language through an exporter of a crate's `pub` API, to TypeScript through an own rustc driver, and to faster builds by measuring upstream's levers. Its benchmark runner moved to [kixel-ai/swole](https://github.com/kixel-ai/swole).
Each project lives in its own top-level directory; the [quest tree](quest/README.md) says what each is doing now.

# Required

- An unsupported construct or malformed input is a compile error with a source span, never a warning, a silent fallback, or a runtime failure. Generated code panics only where the Rust source does.
  One exception is an `unsafe` pointer from elsewhere whose origin translation can't see, such as a parameter's: a loud run-time check may stand in for the refusal, under m3's [unsafe policy](quest/m3/README.md), since refusing every such pointer would refuse pointer arithmetic on any parameter.
  The other is a binding, which checks at run time only the Rust rules its language cannot express: borrow exclusivity, use after a move or close, the thread of a value that is not `Send`, and a call of an item the library's target lacks. A failed check raises the language's own error, or aborts where it has none; a Rust panic aborts.
- Generated output must match the source's semantics exactly: integer widths and overflow, drop order, laziness, and error paths. Prove it with differential tests against native Rust, not by inspection.
- A binding's types are lossless: every value of the Rust type crosses and returns exactly. A native type stands in only where converting it to the Rust type can never fail, such as `bool`, or a byte array for `Vec<u8>` in most languages. Otherwise the runtime's own type crosses, such as `rs.NonZeroU16`, or a string where the language's own can hold a lone surrogate, with helpers converting it to and from the idiomatic one. The caller checks a value once, at that conversion, rather than in every call it passes through, and a value Rust cannot hold fails there, never in a call. Speed ranks above ergonomics.
- Everything is configless, with opinionated names and defaults. Translators take stock crates as they are: no annotations, and no config beyond true platform boundaries. Each config key or source change a consumer needs is a gap to close.
- A vanity rename, if ever supported, is opt-in and lives inline in source next to the item, never in a config file.
- moq is the first consumer of every project here; a binding target may be planned before any consumer asks for it.

# Development

PRs target `main`.
Run `just ci` before every push.
The dev shell points `core.hooksPath` at `.githooks`, whose `pre-push` hook runs it on the push's diff and refuses a push from a tree that differs from the commit pushed, so run it yourself whenever the hook is not active in the checkout: unless `git config core.hooksPath` prints `.githooks` and the branch has `.githooks/pre-push`.
`--no-verify` skips the hook; the merge queue then catches what it would have.
Outside the dev shell, run `just` as `nix develop --command just ...`: the shell's tools on `PATH` alone lack its variables (`PKG_CONFIG_PATH`, `NIX_LDFLAGS`, ...).

Review rounds run one local reviewer, `quest review --harness codex`, or `--harness claude` when Codex changed the branch last.

Quests: before any work, run `quest context` and follow it, unless this session already holds its rules.
