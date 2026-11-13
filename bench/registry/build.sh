#!/usr/bin/env bash
# Export once, build packages, and retain the native oracle with the consumer scripts.
set -euo pipefail
export PATH="${RUST2_CARGO_SHIM:+$RUST2_CARGO_SHIM:}${RUST2_TS_TOOLCHAIN:?run in nix develop}/bin:$PATH"
cd "$(dirname "$0")/../.."
host=$(rustc -vV | sed -n 's/^host: //p')
just ffi run --target "$host" --target wasm32-unknown-unknown --config fixtures/api/ffi.toml --manifest-path fixtures/api/Cargo.toml --out target/registry-export -- --features doubled
cargo build --locked --release --manifest-path ffi/target/registry-export/Cargo.toml --features python
python3 bench/registry/package.py wheel
cargo run --quiet --locked --manifest-path ffi/Cargo.toml --package api-fixture --example transcript --features doubled >ffi/target/registry/native.txt
cp ffi/tests/api.py ffi/target/registry/consumer.py
if [[ $(uname) == Darwin ]]; then
    # Build separately so Python symbols cannot enter the Swift library.
    cargo build --locked --release --manifest-path ffi/target/registry-export/Cargo.toml --no-default-features
    python3 bench/registry/package.py swift
    cp ffi/tests/api.swift ffi/target/registry/consumer.swift
fi
