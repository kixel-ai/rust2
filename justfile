set shell := ["bash", "-euo", "pipefail", "-c"]

# The TypeScript driver, its own workspace on its own nightly: `just ts <recipe>`.
mod ts

# The C exporter, its own workspace on the TypeScript driver's nightly: `just ffi <recipe>`.
mod ffi

# The Go translator, its own workspace on the TypeScript driver's nightly: `just go <recipe>`.
mod go

# The drivers' shared front end and rust2-lint, its own workspace on the TypeScript driver's nightly: `just driver <recipe>`.
mod driver

# Run a Rust CLI from a release: `just crabx <recipe>`.
mod crabx

# List the available development commands.
default:
    @just --list

# Lint what the branch changed since BASE: the merge group's base, or origin/main locally.
check $BASE="":
    sh/dispatch.sh check "$BASE"

# Run what the merge queue runs: lint, then test, what the branch changed since BASE.
ci $BASE="":
    sh/dispatch.sh ci "$BASE"

# Print the modules `just ci` selects for the branch's changes since BASE.
ci-select $BASE="":
    sh/dispatch.sh ci "$BASE" --query

# Lint and test every module, as the nightly does.
gate:
    sh/dispatch.sh ci --all

# Test what the branch changed since BASE.
test $BASE="":
    sh/dispatch.sh test "$BASE"

# Measure Rust build time on the pinned corpus; see bench/build/README.md.
bench *args:
    bench/build/run.sh {{ args }}

# Apply formatters without changing program behavior.
fix:
    shfmt -w bench/*/*.sh sh/*.sh
    taplo format
    nixfmt flake.nix
    just crabx fix
    just ts fix
    just ffi fix
    just go fix
    just driver fix
