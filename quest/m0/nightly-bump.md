# [M] Bump the rustc nightly, on a schedule

## Goal

The nightly every driver here links (`ts/rust-toolchain.toml`, which `driver/`, `ffi/`,
and `go/` symlink) moves to a current one, with `flake.lock`'s rust-overlay bumped to name
it. This quest recurs: the PR that finishes a run creates the next.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a recurring bump, in m0,
every six weeks, one Rust release cycle. Nothing ends the recurrence while a driver here
links `rustc_private`. The period is the maintainer's to change.

- Each run: bump the toolchain file and the overlay, port every driver to rustc's changed
  internals and std's changed behavior, and pass `just gate`. Differential output must
  not change. A code snapshot that moves is a finding to explain in the PR, by its cause,
  before it is regenerated.
- Start from the nightly's "Newest nightly" job (`canary` in `nightly.yml`), which runs
  every module's checks and tests on the newest nightly rust-overlay names: its failures
  are this run's work list.
- `rustc-dev` ships the compiler's source under `lib/rustlib/rustc-src`, and `rust-src`
  ships std's; diff the old and new toolchains' trees to find what moved. `cargo fix
  --broken-code` applies most mechanical renames.
- The benchmark's own nightly in `flake.nix` is a corpus change that lands on its own
  (`bench/build/corpus.toml`); it is [TPDE](/quest/m3/tpde.md)'s to bump, not this
  quest's.
- The finishing PR re-creates this file at the same path with a `Scheduled` date six
  weeks later and carries forward what the run learned.

What the 2026-10-15 to 2026-11-12 run learned:

- rustc internals: `GenericArgsRef` is gone and `GenericArgs` is a `Copy` value with no
  slice `Index` of a range (`as_slice`, `as_tcx_slice`); `ParamEnv::caller_bounds()` is an
  accessor (`.all_clauses(tcx).iter()`, with `CallerBoundsIterator`); `LayoutData::align`
  is an `Align`; tool attributes are `Attribute::Tool(AttrItem)` (`tcx.tool_attrs`), and a
  doc comment is `AttributeExt::doc_str_and_fragment_kind`.
- Expansion and std moved more than the compiler did, and only the differential tests
  caught it: derived `Debug` passes the type's and fields' names joined by spaces and,
  past four variants, indexes a joined string (`debug_c_like_enum_write_str`);
  `vec![x; n]` calls `from_elem_global`; `panic!()`, `unreachable!()`,
  `unimplemented!()`, and `todo!()` call `*_default_msg`; `String`'s char collects reserve
  the size hint exactly, and `str::escape_default` specializes them; `Fuse` forwards
  `advance_back_by`; `EscapeDebug` drops its char once yielded; `IoSlice::advance` and
  `Ready` reword their panics; `Pattern` is an `unsafe trait`, and `Allocator` is stable.
- `go/`'s port added 41 lines and removed 35 over 7 files, all of them derived `Debug` or
  the mechanical renames, against `ts/`'s 380 and 302 over 36.

Public API: none.

## Scheduled

- 2026-12-24 - six weeks after the 2026-11-12 run
