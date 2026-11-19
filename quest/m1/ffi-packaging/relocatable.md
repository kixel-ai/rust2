# [S] A release rebuilds the same from any checkout path

## Goal

The generated crate's version hashes its `Cargo.toml`, which names the facade by absolute
path, and cargo hashes that version into every symbol: two checkouts of one commit at
different paths make different libraries. The exporter writes the dependency relative to
its output, so [a rebuild](/ffi/release/README.md#reproducible-builds) at any path makes the
same bytes, and `reproduce` in `bindings-test.yml` builds the second copy elsewhere.

## Plan

Found by [the reproducible-releases quest](https://github.com/kixel-ai/rust2/pull/874):
`glue.rs`'s `generate` writes `path = {facade.dir}`. Measured there: two exports of the api
fixture from copies at different paths differ in the version's hash and in every symbol
hash; the same path with another export directory is byte for byte equal on Linux and for the wheels.
The Go module's Windows GNU library is not: `dlltool` names its `kernel32` import members'
symbols (`_head_D__a_rust2_..._export_target_go_...`) after the path of the target
directory, which `--remap-path-prefix` does not reach (and a random rustc temporary directory in it,
which the builder zeroes), so `reproduce` rebuilds from a new
export at the same path. Find a target directory path that is the same everywhere (or a
rustc that remaps it) so the CI rebuild can move the export again.

Fall back to the absolute path where no relative one exists, such as another Windows drive.
Check `cli.rs`'s other absolute manifest paths (its "each path absolute" note) for the same
dependence.

Public API: none.

## Related

- [Release every language the exporter generates](/quest/m1/ffi-packaging/README.md) - the epic whose builds this makes checkable by anyone
