set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Check the runner, the small local fork of binstalk's fetchers, and the installer.
check:
    cargo clippy --locked --all-targets -- -D warnings
    cargo fmt --all --check
    shellcheck install.sh
    shfmt -d install.sh

# Exercise cache execution, trust policy, and the release's packages without the network.
test *args:
    cargo test --locked {{ args }}
    python3 -I test_release.py

# Run crabx from the development checkout.
run *args:
    cargo run --locked -- {{ args }}

# Format the runner.
fix:
    cargo fmt --all
