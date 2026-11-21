binstalk-fetchers is imported from cargo-bins/cargo-binstall at
`f027dceb0f90c3371f26047a160578b8b4bdc877` (0.10.54, released with binstalk 0.31.0),
under its included GPL-3.0-only license. Its dependencies use crates.io instead of sibling
paths. crabx compiles the fetchers as a source module so Cargo includes the fork when
packaging crabx; `Cargo.toml.upstream` preserves the original manifest.

`binstalk-target.rs` is the same revision's private target-template adapter from
`crates/binstalk/src/helpers/target_triple.rs`, with its error type exposed directly
and its unit tests omitted. Its license is also GPL-3.0-only.

Local changes:

- Expose the archive's SHA-256, resolved URL, and crate repository after downloading,
  hashing the same stream minisign checks. crabx checks attestations before installing.
- Qualify internal paths under `crate::fetchers` and compile quickinstall
  unconditionally; crabx enables its use only with `--allow-quickinstall`.
- Remove upstream's test modules, some of which need the network or upstream's workspace
  docs. Keep a test that the default templates guess `{name}-v{version}` and
  `{name}-{version}` tags; crabx tests execution and trust offline.
- Wait for resolver channel capacity so a queued error cannot discard a successful candidate.

Upstream: https://github.com/cargo-bins/cargo-binstall
