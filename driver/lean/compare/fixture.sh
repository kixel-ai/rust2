#!/usr/bin/env bash
# Translate the fixture's `cases` with rust2-lean, check it with Lean, then replay native
# Rust's outcomes on it (the fixture's tests/lean_diff.rs, FixtureDiff.lean), for the models
# the moq comparison does not reach.
#
# Usage: nix develop .#lean --command just driver lean-fixture
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
driver=$(cd "$here/../.." && pwd)
fixture=$driver/lean/tests/fixture
work=$driver/target/lean-fixture
mkdir -p "$work"

timed() {
	local start=$SECONDS
	"$@"
	echo "$(( SECONDS - start )) s: $1 ${2:-}" >&2
}

cargo build --quiet --locked --release --manifest-path "$driver/Cargo.toml" --package rust2-lean
rm -rf "$work/out"
CARGO_TARGET_DIR=$work/target timed "$driver/target/release/rust2-lean" --out "$work/out" \
	--manifest-path "$fixture/Cargo.toml" --start-from 'crate::cases'
# The test profile keeps the overflow checks the translation keeps.
LEAN_DIFF_OUT=$work/out/cases.txt CARGO_TARGET_DIR=$work/target timed cargo test --quiet \
	--manifest-path "$fixture/Cargo.toml" --test lean_diff -- --ignored >/dev/null

cd "$work/out"
cp "$here/FixtureDiff.lean" .
export LEAN_PATH=.
timed lean -o Rust.olean Rust.lean
# Each crate the cases reach, after the crates it calls.
for module in Kio FixtureDep Fixture; do
	timed lean -o $module.olean $module.lean
done
timed lean --run FixtureDiff.lean cases.txt
