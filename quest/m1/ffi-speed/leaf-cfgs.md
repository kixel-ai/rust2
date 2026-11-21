# [M] Prove leaves against the build's target cfgs

## Goal

A build whose target `cfg`s differ from the check's, as `RUSTFLAGS` make them with
`-C target-cpu`, `-C target-feature`, or `--cfg`, makes no leaf call, as one under another
profile makes none since [#943](https://github.com/kixel-ai/rust2/pull/943). So does one whose
build scripts read another `DEBUG`, which a check always sets `false`.

## Plan

Found in #943, which proves leaves against the release profile and has the glue's build
script compare `PROFILE`, `OPT_LEVEL`, and debug assertions with the check's, beside the
panic strategy. A body under `cfg(target_feature = "avx2")`, or under a `cfg` a build script
derives from `DEBUG` or a target `cfg`, may differ between the export's environment and the
build's, which today refuse only another target triple.

Recommended: the build script compares its `CARGO_CFG_*`, less `CARGO_CFG_FEATURE`, with the
target's `cfg`s the export records: those rustc sets for the target under the check's
rustflags, as the wrapper can ask rustc with the args every crate of the target shares, not
the facade's own `--cfg`s from its features and build script. `DEBUG` is no `cfg`, and the
check never sees the build's, so the export records the `debug` setting of the profile it
resolved instead, from the manifest `PROFILE`'s resolution in `ffi/src/cli.rs` reads.
