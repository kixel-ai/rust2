# [S] No exclude hides what exports

## Goal

`ffi/moq/ffi.toml` excludes only what the export refuses: the nightly fails when an
exclude names an item that now exports or names nothing, so the list shrinks as the
exporter grows and `just ffi moq` builds everything that maps.

## Plan

Found by the 2026-11-12 audit, comparing the file against the 2026-11-11 nightly census's
"config that exports what maps"
([run 43038758163](https://github.com/moq-dev/rust2/actions/runs/43038758163)): 28 of the
100 excludes name items that export, such as `Path::from_static`, the
`Timestamp::as_*` conversions, `mux::Source`'s methods, and `varint::encode_quic`, so
`just ffi moq` never builds what
[#675](https://github.com/moq-dev/rust2/pull/675) and
[#678](https://github.com/moq-dev/rust2/pull/678) made exportable. At moq's `main`, 11
more name nothing exported ([the pin bump](/quest/m1/ffi-refusals/moq-pin.md)). Measure
both again first.

- Drop every exclude the census does not need.
- The census already prints the config that exports what maps; the nightly's moq job
  fails when `ffi/moq/ffi.toml` differs from it.
- The file goes with [the epic](/quest/m1/ffi-refusals/README.md); until then it is
  exact.

Public API: none.
