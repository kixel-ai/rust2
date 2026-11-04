# [L] C names and crates with no config

## Goal

[rust2-ffi](/ffi/README.md) names every exported item by a collision-free rule and takes
its crate set and prefix from Cargo alone, so `ffi.toml` loses `package`, `prefix`,
`[crates.*]`, and every `rename`, leaving only `exclude` for
[the refusals](/quest/m1/ffi-export/refusals.md) to remove.

## Plan

Decided with the maintainer on 2026-11-04, after
[#390](https://github.com/moq-dev/rust2/pull/390) added moq-specific renames: the exporter
takes an arbitrary crate with no config, and a vanity rename, if ever supported, lives
inline in source next to the item.

- Naming rule: keep Rust's case. ✅ `moq_session_Stats` for the type and
  `moq_Session_stats` for the method, so a crate that follows Rust's naming conventions
  never collides. A residual clash, possible only with unconventional names, fails the
  export with both spans. Rejected: snake case with `__` before members; mangled
  (`moq_7session5Stats`), which is provably free but unreadable in C.
  Case alone does not separate a field's getter and setter from a method of the same
  name, which conventional Rust allows: moq's `Backoff::initial` field and method, and
  the reject fixture's `Limit::max` setter beside `set_max`. The rule must name field
  accessors apart from methods too, with no rename.
- Items of different crates are named by their path in the facade, so moq-tokio's
  `Client` and moq-net's need the facade to re-export them under distinct paths.
- Crate set: one facade crate. ✅ The exporter exports one crate's `pub` API, following
  its `pub use` re-exports, so moq-ffi re-exports what it ships, in moq's tree. Rejected:
  crates listed on the command line, which only moves the list into a script.
- Prefix: the facade's Cargo lib name. ✅ `[lib] name` defaults to the package name;
  moq-ffi sets it to `moq` in its own Cargo.toml for `moq_`. Rejected: the package name,
  which would give `moq_ffi_`.

Every generator derives its names from the C ones, so each changes with them; regenerate
moq's export and the stand-in (`just ffi standin` becomes the facade's `pub use` list) and
update `ffi/README.md`'s naming and config sections.

Public API: every C name the exporter emits, and the `ffi.toml` keys it reads.
