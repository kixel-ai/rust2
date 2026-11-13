set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Check the runner and the small local fork of binstalk's fetchers.
check:
    cargo clippy --locked --all-targets -- -D warnings
    cargo fmt --all --check

# Exercise cache execution and trust policy without the network.
test *args:
    cargo test --locked {{ args }}

# Run crabx from the development checkout.
run *args:
    cargo run --locked -- {{ args }}

# Format the runner.
fix:
    cargo fmt --all
