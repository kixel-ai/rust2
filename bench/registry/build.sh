#!/usr/bin/env bash
# Export once, build packages, and retain the native oracle with the consumer scripts.
set -euo pipefail
export PATH="${RUST2_CARGO_SHIM:+$RUST2_CARGO_SHIM:}${RUST2_TS_TOOLCHAIN:?run in nix develop}/bin:$PATH"
if [[ $(uname) == Darwin ]]; then
    # Nix selects an SDK bundle; this build needs the installed Xcode and its full SDK.
    export DEVELOPER_DIR
    DEVELOPER_DIR=$(env -u DEVELOPER_DIR /usr/bin/xcode-select -p)
    export SDKROOT
    SDKROOT=$(env -u SDKROOT /usr/bin/xcrun --sdk macosx --show-sdk-path)
    # Nix's linker wrapper injects store libraries unavailable to wheel consumers.
    export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=/usr/bin/clang
    export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=/usr/bin/clang
fi
cd "$(dirname "$0")/../.."
rm -rf ffi/target/registry
host=$(rustc -vV | sed -n 's/^host: //p')
just ffi run --target "$host" --target wasm32-wasip1 --config fixtures/api/ffi.toml --manifest-path fixtures/api/Cargo.toml --out target/registry-export -- --features doubled
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
