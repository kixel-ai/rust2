# [M] A release rebuilds the same from any checkout path

## Goal

A [rebuild](/ffi/release/README.md#reproducible-builds) of a commit makes the same bytes
at any checkout path, and `reproduce` in `bindings-test.yml` builds its second copy of the
source and the export elsewhere.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#874](https://github.com/kixel-ai/rust2/pull/874): ✅ kept the relative dependency (rejected: dropping it).

Done: the generated crate's manifest names the facade, each `[patch]` path, and each
path crate it reaches relative to its export, so its version hash, and with it every
symbol of the glue crate, is the same from any checkout. Two exports of one commit at
other paths are byte for byte equal while each keeps its position relative to its crate
(`export_is_relocatable`); an export at another depth writes another relative path, so
another hash. It stays absolute where no
relative path reads right: an export whose own path holds a symlink (cargo joins `..`
lexically), or on another Windows drive from the crate.

Still differing, measured on the api fixture from two checkouts at other paths: the
facade's own symbols. Cargo hashes a path package outside the workspace root into its
metadata by its absolute path, so the glue's `[workspace]` root, the export, cannot make
the facade's hash relative whatever the manifest says. Nothing in the manifest, `RUSTFLAGS`,
or `--remap-path-prefix` reaches it. So `reproduce` can still only move the export, as it
could before.

The Windows GNU Go library is the same kind: `dlltool` names its `kernel32` import
members' symbols (`_head_D__a_rust2_..._export_target_go_...`) after the path of the target
directory (and a random rustc temporary directory in it, which the builder zeroes).

Recommendation: the builder builds through one fixed spelling. `package.py build` links
a fixed-name directory (a symlink, a junction on Windows) to the common ancestor of the
crate's workspace and the export, and runs cargo on the manifest, the target directories,
and the remap prefixes through it, never resolved. Cargo then hashes the same facade path
and dlltool the same target directory wherever the checkout is. `reproduce` then moves both
the source and the export for its second copy, keeping the export's depth below the common
ancestor. Only CI shows it on macOS and Windows, which
the linked directory and `Path.resolve()` calls in `package.py` could break: try it there.
The alternative for the dlltool path alone is a target directory under a fixed absolute
root, which leaves the facade's hash.

## Related

- [Release every language the exporter generates](/quest/m1/ffi-packaging/README.md) - the epic whose builds this makes checkable by anyone
