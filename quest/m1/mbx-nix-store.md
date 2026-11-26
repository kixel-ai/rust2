# [S] Ask mbx to key rustc's paths under /nix/store

## Goal

A warm wasm32 `rust2-ffi` export in the dev shell is served from mbx's cache: every crate
mbx compiles over its MIR sysroot, and std's crates under `-Zbuild-std`, not compiled each
time. The crates the export's wrapper analyzes, the facade and the glue the walk checks,
compile each time.

## Plan

mbx 1.31's rustc adapter maps a path it keys to a root it knows, such as the workspace, the
target, a package's directory, cargo's registry, or `RUSTUP_HOME`, and leaves uncached a
compilation that reads any other ("absolute path has no stable cache mapping"). It maps
none under `/nix/store`, which holds the dev shell's MIR sysroot (`RUST2_TS_SYSROOT`, passed
as `--sysroot`) and the toolchain's std sources, which `-Zbuild-std` compiles. Its cc adapter
already keys `/nix/store` verbatim (`SYSTEM_ROOTS` in `mbx-cache-cc`), since a store path
names its contents and is the same on every machine that has it.

Measured 2026-11-25 on **desktop** (bench/ffi/README.md's "The export's cache"): over the
sysroot, a warm web export serves 76 host units and leaves 104 wasm32 crates unconsulted,
54 CPU seconds against 74 cold; under `-Zbuild-std`, it serves every moq crate and compiles
std's 13, 42 against 96.

Recommended ask, as a Discussion with these numbers: key rustc paths under `/nix/store`
verbatim, as the cc adapter does, covering an explicit `--sysroot` and the sources of a
package there, so a Nix toolchain's `-Zbuild-std` std and a sysroot built by a derivation
are served. jdx/mr-boxington's
[CONTRIBUTING](https://github.com/jdx/mr-boxington/blob/main/CONTRIBUTING.md) takes
proposals in Discussions, allows AI help if disclosed, and closes reports found only by
reading the source; this one is measured. No Discussion or issue asked it as of
2026-11-25. If mbx ships it, re-measure the web rows and drop the ffi README's caveat.

Decided 2026-11-25 (maintainer, pick round on [kixel-ai/rust2#1028](https://github.com/kixel-ai/rust2/pull/1028)):
✅ this quest's agent posts the ask, disclosing AI help (rejected: the maintainer posts it).
Record the Discussion link in this Plan once posted, then wait on mbx's answer under `Waiting`.

A workaround here, copying the sysroot under the target directory where mbx maps it, would
cost 120 MB a target and still leave `-Zbuild-std`'s std uncached, so it is not planned.
