binstalk-fetchers is imported from cargo-bins/cargo-binstall at
`5885714d0a40a14b2535c6ffc051fd8e49aa9665` (0.10.53), under its included GPL-3.0-only
license. Its dependencies use crates.io instead of sibling paths. crabx compiles
the fetchers as a source module so Cargo includes the fork when packaging crabx;
`Cargo.toml.upstream` preserves the original manifest.

`binstalk-target.rs` is the same revision's private target-template adapter from
`crates/binstalk/src/helpers/target_triple.rs`, with its error type exposed directly
and its unit tests omitted. Its license is also GPL-3.0-only.

Local changes:

- Guess `{name}-v{version}` release tags, until upstream supports them.
- Expose the archive's SHA-256, resolved URL, and crate repository after downloading,
  hashing the same stream minisign checks. crabx checks attestations before installing.
- Qualify internal paths under `crate::fetchers` and compile quickinstall
  unconditionally; crabx enables its use only with `--allow-quickinstall`.
- Remove upstream's live-network test modules. Keep the release-tag regression test;
  crabx tests execution and trust offline.
- Apply the current Clippy syntax for the GitHub lookup condition.
- Wait for resolver channel capacity so a queued error cannot discard a successful candidate.

Upstream: https://github.com/cargo-bins/cargo-binstall
