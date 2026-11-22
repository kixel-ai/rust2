# [S] Measure the cached export

## Goal

Once mbx caches the export's flags, the merge queue's `ffi` module and `bench/ffi`'s export
rows report a warm `rust2-ffi` export, native and web, beside a cold one, and the cost of a
facade's sibling workspace crates, which mbx leaves to the wrapper, is known.

## Plan

The export now wraps only the workspace's own crates (`RUSTC_WORKSPACE_WRAPPER`) and is
cargo's `RUSTC` for the rest, each with the consumer's rustflags and then the export's in
`CARGO_ENCODED_RUSTFLAGS`, so mbx sees every flag; `tests/fixtures.rs`'s `cached` checks
the command lines a cache gets. mbx 1.30 still bypasses every target crate for the
export's `-Z` flags, which [the mbx ask](/quest/m0/mbx-export-flags.md) closes, and serves
only build scripts and proc macros (`bench/ffi/README.md`'s "The export's cache").

Then, from the queue's `ffi` time before and after over its merge groups, and
`bench/ffi/measure.py` with `mbx explain` on a second export from an emptied target:

- Both passes of the web export, the check and the walk of the glue crate, served from
  the cache, with std from the dev shell's MIR sysroot and from `-Zbuild-std`, or say why
  one cannot be.
- A warm row in `bench/ffi`'s export table, which measures uncached exports today.
- A crate exported from inside a larger workspace, as moq's real `moq-ffi` is, whose
  siblings mbx leaves uncached under the wrapper: how much they cost, for
  [the mbx ask](/quest/m1/mbx-wrapper-delegate.md).

## Required

- [The mbx flags](/quest/m0/mbx-export-flags.md) - an mbx release that caches a crate
  carrying the export's `-Z` flags
