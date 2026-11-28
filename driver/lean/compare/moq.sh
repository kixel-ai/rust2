#!/usr/bin/env bash
# Translate moq-net's route algebra, `Path`, and kio's `Instant` with rust2-lean, fail if the
# translation differs from MoqNet.lean, MoqPattern.lean, and Kio.lean here, check it with Lean, then replay
# native Rust's outcomes on it (moq_diff.rs, MoqDiff.lean): random cases at the algebra's
# bounds, then every algebra call behind the route proof's checked-in vectors
# (moq_vectors.rs).
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
tests=$net/src/route/tests
# Undo the last run's test hooks, so the translation reads moq-net as it is.
unhook() {
	git -C "$moq" checkout --quiet -- rs/moq-net/src/route/tests/mod.rs
	rm -f "$tests/lean_diff.rs" "$tests/lean_vectors.rs"
}
unhook

timed() {
	local start=$SECONDS
	"$@"
	echo "$(( SECONDS - start )) s: $1 ${2:-}" >&2
}

cargo build --quiet --locked --release --manifest-path "$driver/Cargo.toml" --package rust2-lean
rm -rf "$work/out"
# The functions the route algebra's proof binds (moq-net's route/tests/algebra.rs),
# `debounce`, the reader's time arithmetic, `Path`'s model and the methods that read it, which
# reach moq-pattern's segment walk, and the clock arithmetic of kio's `Instant`, which
# translates from kio's own source.
instant=kio::clock::instant::Instant
timed "$driver/target/release/rust2-lean" --out "$work/out" --manifest-path "$net/Cargo.toml" \
	--start-from 'crate::route::relay::Offer' \
	--start-from 'crate::route::relay::Here' \
	--start-from 'crate::route::relay::order' \
	--start-from 'crate::route::relay::Claim' \
	--start-from 'crate::route::relay::claim_order' \
	--start-from 'crate::route::relay::precedence' \
	--start-from 'crate::route::relay::floor_of' \
	--start-from 'crate::route::relay::debounce' \
	--start-from 'crate::path::Path::head' \
	--start-from 'crate::path::Path::has_prefix' \
	--start-from 'crate::path::Path::is_empty' \
	--start-from "$instant::checked_add" \
	--start-from "$instant::checked_sub" \
	--start-from "$instant::saturating_add" \
	--start-from "$instant::saturating_sub" \
	--start-from "$instant::checked_duration_since" \
	--start-from "$instant::saturating_duration_since" \
	--start-from "$instant::duration_since" \
	--start-from "$instant::sub"
for module in MoqNet MoqPattern Kio; do
	if [ "${1:-}" = --write ]; then
		cp "$work/out/$module.lean" "$here/$module.lean"
	else
		diff -u "$here/$module.lean" "$work/out/$module.lean"
	fi
done

# Native Rust's outcomes, from tests in moq-net's route::tests, since what they call is
# crate-private; the test profile keeps the overflow checks the translation keeps.
cp "$here/moq_diff.rs" "$tests/lean_diff.rs"
# The vectors' generators in algebra.rs, with each algebra call renamed to one that records
# it (moq_vectors.rs). A call left unrenamed would go unrecorded, so none may remain.
calls='claim_order, floor_of, order, precedence,'
[ "$(grep -c "$calls" "$tests/algebra.rs")" = 1 ] || { echo "algebra.rs imports the algebra differently" >&2; exit 1; }
sed -e "s/$calls//" -e 's/Offer::own(/Offer::rec_own(/g' \
	-e 's/\.\(onward\|arrive\|over\|exported\|adopted\|lowered\)(/.rec_\1(/g' \
	"$tests/algebra.rs" >"$tests/lean_vectors.rs"
if grep -nE '(Offer::own|[.](onward|arrive|over|exported|adopted|lowered))\(' "$tests/lean_vectors.rs"; then
	echo "an algebra call above was not renamed" >&2
	exit 1
fi
cat "$here/moq_vectors.rs" >>"$tests/lean_vectors.rs"
printf 'mod lean_diff;\nmod lean_vectors;\n' >>"$tests/mod.rs"
LEAN_DIFF_OUT=$work/out/cases.txt LEAN_VECTORS_OUT=$work/out/vectors.txt timed cargo test --quiet \
	--manifest-path "$moq/Cargo.toml" -p moq-net --lib route::tests::lean_ -- --ignored --test-threads 1 --nocapture
unhook

cd "$work/out"
cp "$here/MoqDiff.lean" .
export LEAN_PATH=.
timed lean -o Rust.olean Rust.lean
timed lean -o Kio.olean Kio.lean
timed lean -o MoqPattern.olean MoqPattern.lean
timed lean -o MoqNet.olean MoqNet.lean
timed lean --run MoqDiff.lean cases.txt
timed lean --run MoqDiff.lean vectors.txt
