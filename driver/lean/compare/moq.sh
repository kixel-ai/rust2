#!/usr/bin/env bash
# Translate moq-net's route algebra with rust2-lean, fail if the translation differs from
# MoqNet.lean here, check it with Lean, then replay native Rust's outcomes on it
# (moq_diff.rs, MoqDiff.lean).
#
# Usage: nix develop .#lean --command just driver lean-moq [--write]
# With --write, replace the committed translation instead of failing on a difference.
set -euo pipefail

# The moq-dev/moq revision the Router census was taken at.
rev=512e3d95a54c7c9cb8940ec3d1fc3948b5fa2159

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
driver=$(cd "$here/../.." && pwd)
work=$driver/target/lean-moq
moq=$work/moq
net=$moq/rs/moq-net
mkdir -p "$work"
if [ "$(git -C "$moq" rev-parse HEAD 2>/dev/null)" != "$rev" ]; then
	rm -rf "$moq"
	git clone --quiet --filter=blob:none https://github.com/moq-dev/moq "$moq"
	git -C "$moq" checkout --quiet "$rev"
fi
# Undo the last run's test hook, so the translation reads moq-net as it is.
git -C "$moq" checkout --quiet -- rs/moq-net/src/route/tests/mod.rs
rm -f "$net/src/route/tests/lean_diff.rs"

timed() {
	local start=$SECONDS
	"$@"
	echo "$(( SECONDS - start )) s: $1 ${2:-}" >&2
}

cargo build --quiet --locked --release --manifest-path "$driver/Cargo.toml" --package rust2-lean
rm -rf "$work/out"
# The functions the route algebra's proof binds (moq-net's route/tests/algebra.rs).
timed "$driver/target/release/rust2-lean" --out "$work/out" --manifest-path "$net/Cargo.toml" \
	--start-from 'crate::route::relay::Offer' \
	--start-from 'crate::route::relay::Here' \
	--start-from 'crate::route::relay::order' \
	--start-from 'crate::route::relay::Claim' \
	--start-from 'crate::route::relay::claim_order' \
	--start-from 'crate::route::relay::precedence' \
	--start-from 'crate::route::relay::floor_of'
if [ "${1:-}" = --write ]; then
	cp "$work/out/MoqNet.lean" "$here/MoqNet.lean"
else
	diff -u "$here/MoqNet.lean" "$work/out/MoqNet.lean"
fi

# Native Rust's outcomes, from a test in moq-net's route::tests, since what it calls is
# crate-private; the test profile keeps the overflow checks the translation keeps.
cp "$here/moq_diff.rs" "$net/src/route/tests/lean_diff.rs"
echo 'mod lean_diff;' >>"$net/src/route/tests/mod.rs"
LEAN_DIFF_OUT=$work/out/cases.txt timed cargo test --quiet --manifest-path "$moq/Cargo.toml" -p moq-net --lib \
	route::tests::lean_diff -- --ignored >/dev/null
git -C "$moq" checkout --quiet -- rs/moq-net/src/route/tests/mod.rs
rm "$net/src/route/tests/lean_diff.rs"

cd "$work/out"
cp "$here/MoqDiff.lean" .
export LEAN_PATH=.
timed lean -o Rust.olean Rust.lean
timed lean -o MoqNet.olean MoqNet.lean
timed lean --run MoqDiff.lean cases.txt
