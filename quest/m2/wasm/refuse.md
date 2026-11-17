# [L] Refuse at export what a wasm32 target lacks

## Goal

rust2-ffi refuses at export, at the span of the call that reaches it, what a wasm32 target's
host has no API for. On [the web target](/ffi/README.md#typescript-web), that is the file
system, sockets, and a process: today the module imports the WASI function std calls for
each, and `load` refuses the module, naming the import, which is loud but at load, not at
export. A crate importing browser APIs through wasm-bindgen (web-sys, js-sys) is refused
at the import's span too, until a consumer needs both: today its module imports
`__wbindgen_placeholder__`, which `load` refuses. [The WASI target](/quest/m3/wasi.md)
refuses what its host lacks the same way, such as threads and sockets on `wasm32-wasip1`.

## Plan

Split out of the web target's quest in
[#851](https://github.com/kixel-ai/rust2/pull/851), as its Plan recommended once the
exporter's MIR reading was sized: it is most of the work, and WASI needs the same.

- It needs every reachable body's MIR, which the exporter does not read: it runs `cargo
  check` of the crate under its rustc driver (`driver/`, shared with rust2-ts) and reads
  signatures alone. rust2-ts reads MIR through its profile, `-Zalways-encode-mir` on every
  crate ([`ts/src/profile.toml`](/ts/src/profile.toml), set by
  [`ts/src/cli.rs`](/ts/src/cli.rs)), and std's from its own MIR sysroot (the flake's
  `RUST2_TS_SYSROOT`, else `-Zbuild-std`). Recommended: the exporter reuses both for its
  wasm32 targets alone, so the native export costs nothing more.
- Recommended: derive what is refused from the module's imports, not a list of std paths.
  Walk the calls reachable from each export's instance, through `dyn` and function pointers
  to every implementation the crate can reach, to a foreign item imported from
  `wasi_snapshot_preview1` that the target's binding does not implement, or from
  `__wbindgen_placeholder__`, and refuse at the first call in the crate's own code, naming
  the path down to the import. A new std then needs no change here.
- Report the export's time on moq's stand-in with and without the MIR, as the benchmark's
  build cost rows do, since every wasm32 export pays it.

Public API: none; an export that reaches what its target lacks fails where its module's
`load` failed.

## Related

- [The WASI target](/quest/m3/wasi.md) - refuses what a WASI host lacks the same way
