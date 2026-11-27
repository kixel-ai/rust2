#!/usr/bin/env bash
# Translate kio's pure machines with rust2-lean, fail if the translation differs from
# KioPureMachine.lean here, check it and Ranked.lean's proof with Lean, then replay native
# Rust's outcomes on it (lean_diff.rs, Diff.lean).
#
# Usage: nix develop .#lean --command just driver lean-kio [--write]
# With --write, replace the committed translation instead of failing on a difference.
set -euo pipefail

# kixel-ai/kio#26's head, whose lab/pure-machine kio's prove.sh translates with Aeneas.
rev=073df54a961b802b8524c437e8957277490f9003

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
driver=$(cd "$here/../.." && pwd)
work=$driver/target/lean-kio
kio=$work/kio
mkdir -p "$work"
if [ "$(git -C "$kio" rev-parse HEAD 2>/dev/null)" != "$rev" ]; then
	rm -rf "$kio"
	git clone --quiet --filter=blob:none https://github.com/kixel-ai/kio "$kio"
	git -C "$kio" checkout --quiet "$rev"
fi

timed() {
	local start=$SECONDS
	"$@"
	echo "$(( SECONDS - start )) s: $1 ${2:-}" >&2
}

cargo build --quiet --locked --release --manifest-path "$driver/Cargo.toml" --package rust2-lean
rm -rf "$work/out"
# prove.sh's roots, and the relay's step for the differential test.
timed "$driver/target/release/rust2-lean" --out "$work/out" --manifest-path "$kio/lab/pure-machine/Cargo.toml" \
	--start-from 'crate::send_credit::Session::step' \
	--start-from 'crate::send_credit::Session::new' \
	--start-from 'crate::send_credit::Session::within_credit' \
	--start-from 'crate::send_credit::Session::well_formed' \
	--start-from 'crate::warm_settle::Relay::next_pulls' \
	--start-from 'crate::warm_settle::best' \
	--start-from 'crate::warm_settle::Relay::step'
if [ "${1:-}" = --write ]; then
	cp "$work/out/KioPureMachine.lean" "$here/KioPureMachine.lean"
else
	diff -u "$here/KioPureMachine.lean" "$work/out/KioPureMachine.lean"
fi

cd "$work/out"
cp "$here/Ranked.lean" "$here/Diff.lean" .
export LEAN_PATH=.
timed lean -o Rust.olean Rust.lean
timed lean -o KioPureMachine.olean KioPureMachine.lean
timed lean Ranked.lean

# Native Rust's outcomes, with the overflow checks the translation keeps.
cp "$here/lean_diff.rs" "$kio/lab/pure-machine/examples/lean_diff.rs"
CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true \
	cargo run --quiet --release --manifest-path "$kio/Cargo.toml" -p kio-pure-machine --example lean_diff >cases.txt
timed lean --run Diff.lean cases.txt

# The fixture test's expected translation, which `cargo test` compares but cannot check.
timed lean "$driver/lean/tests/fixture/Fixture.lean"
