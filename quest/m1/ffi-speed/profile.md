# [S] The glue crate builds with the consumer's release profile

## Goal

The crate [rust2-ffi](/ffi/README.md) generates builds with the profiles of the workspace
it exports, so a consumer that ships fat LTO and `strip`, as moq-ffi does, gets them with
no flag. Today the generated manifest is its own workspace root with the consumer's
`[patch]` carried over and no `[profile]` (`ffi/src/glue.rs`), so every export builds
with cargo's default `release`, and [the binding benchmark](/bench/ffi/README.md) measures
that rather than what moq ships.

## Plan

- Carry the exported workspace's `[profile.*]` tables into the generated manifest, as
  `[patch]` is carried. A per-package override names packages by the consumer's spec, so
  carry only the ones that resolve in the generated workspace and refuse the rest with
  the manifest's span.
- Do not force `panic = "abort"`: it changes what a crate that catches unwinds does.
- Report the bench's calls and the libraries' sizes under moq's profile, and how much
  longer the export's build takes.

Public API: none; an export's build now follows the consumer's profile.
