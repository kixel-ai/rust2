#!/usr/bin/env bash
# Build what bench.sh and bench-wazero.sh run, from the repo root in the dev shell: lib/
# exported by rust2-ffi as a wasm32-wasip1 core module, guest/ as wasip2 and wasip3
# components, and the wasmtime and wazero hosts.
set -euo pipefail
here=bench/wasi-report
nix build --impure -f "$here/rust-wasi.nix" -o .tmp/rust-wasi
rust=$PWD/.tmp/rust-wasi/bin
(cd ffi && just run --manifest-path "../$here/lib/Cargo.toml" --out ../.tmp/wb/core --target wasm32-wasip1)
cd "$here"
PATH=$rust:$PATH cargo build --quiet --release --manifest-path ../../.tmp/wb/core/Cargo.toml --target wasm32-wasip1 --features web --target-dir target-core
PATH=$rust:$PATH cargo build --quiet --release --manifest-path guest/Cargo.toml --target wasm32-wasip2 --target-dir target-p2
PATH=$rust:$PATH cargo build --quiet --release --manifest-path guest/Cargo.toml --target wasm32-wasip3 --features async --target-dir target-p3
PATH=$rust:$PATH cargo build --quiet --release --manifest-path host/Cargo.toml --target-dir target-host
(cd wazero && go build -o wb-wazero .)
