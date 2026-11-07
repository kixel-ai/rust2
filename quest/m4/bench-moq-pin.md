# [S] Bump the build benchmark's moq pin past poll-first

## Goal

The `moq` corpus in [corpus.toml](/bench/build/corpus.toml) pins a moq main
after poll-first ([moq-dev/moq#7607](https://github.com/moq-dev/moq/pull/7607)),
so the benchmark measures the moq its developers edit: the hub edit rebuilds the
10 crates it rebuilds at main, not the pin's 8, and the async body edit is a
thin wrapper over a `poll_*` function, as moq's rule makes it.

## Plan

Decided with the maintainer on 2026-11-07: m4. Found by the subset-compiler
report ([moq-dev/rust2#615](https://github.com/moq-dev/rust2/pull/615)), which
measured at moq main because the pin (`f36c848`) predates poll-first.

- Land the bump on its own, as corpus.toml asks, so no report compares numbers
  across it.
- Update what names the old tree: the `sync` and `async` lines (at main,
  `Session::closed` awaits `poll_closed`), the `test` package versions, and the
  bench README's moq notes, such as the ra-jit bugs seen at the pin. At main,
  the report saw `edit-async` fall back on usage-argv's `parsed_values_given`.
- `moq-pure-crypto` is the old rev plus moq's `spike/pure-crypto` branch, so the
  bump leaves it comparing a different tree. Recommended: merge moq main into a
  new spike branch in moq-dev/moq and pin that, so the comparison still holds.
- Out of scope: [nix.toml](/bench/build/nix.toml)'s own moq pin, which the Nix
  comparison keeps for crane's dependency layer.
