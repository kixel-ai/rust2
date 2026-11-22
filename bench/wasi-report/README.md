# WASI report harness

What [kixel-ai/rust2#967](https://github.com/kixel-ai/rust2/pull/967) ran, kept in one
commit of its branch and removed before it merged, since no WASI target ships.

- `c-script.sh`: the api fixture's C script over the C ABI, linked with the glue into one
  wasm32-wasip1 module and run under `wasmtime run`, against native Rust's transcript.
- `build.sh`, then `bench.sh` and `bench-wazero.sh`: `lib/`'s call shapes, mirroring
  bench/ffi's cases, through the exporter's C ABI in a core module, through a component
  (`guest/`, its WIT in `guest/wit/wb.wit`), and in Rust, from a wasmtime 51 host
  (`host/`) and a wazero 1.12 host (`wazero/`). `rounds.txt` and `rounds-wazero.txt` are
  the five rounds the PR reports, with the load before each.
