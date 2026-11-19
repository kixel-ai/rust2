# [M] Carry rustdoc into every binding properly

## Goal

Every binding's generated docs read as that language's docs:

- Rustdoc intra-doc links, such as ``[`Self::abort`]`` and ``[`Error::Timeout`]``, become
  links to the binding's own names (KDoc, DocC, docstrings, godoc, dartdoc, TSDoc,
  Doxygen), from the name tables the API lock already holds.
- `# Errors` and `# Panics` sections become `@throws`, `Raises:`, `- Throws:`, and their
  equivalents, naming the binding's error type.
- Rust code examples are left out of the binding's docs, not shown as Rust.
- `#[deprecated]` carries into each language's deprecation (`@Deprecated`, `@available`,
  a `DeprecationWarning`, `// Deprecated:`, `@Deprecated(...)`, `@deprecated`,
  `[[deprecated]]`), so consumers see it before the API lock enforces the removal.

## Plan

Found by the 2026-11-18 language audit: rustdoc goes into every language verbatim with
correct escaping (`ffi/src/collect.rs:6396`, `ffi/src/kotlin.rs:3952`,
`ffi/src/python.rs:2111`), but links stay unresolved, and nothing in `ffi/src` reads
`#[deprecated]`.

Decided with the maintainer on 2026-11-18: ✅ in this epic, since deprecations and doc
links are part of the generated API moq adopts (rejected: after the pin).

Public API: generated docs and deprecation attributes in every binding.

## Required

- [API surface lock](/quest/m1/ffi-export/api-surface.md) - its CI check shows this PR's break in every language's rendered API
