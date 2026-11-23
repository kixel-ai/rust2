#!/usr/bin/env bash
# Builds the three routes' programs into $dir, then runs each `rounds` times, alternating,
# into $dir/rounds.tsv: route, round, case, wall ns, CPU ns, and the 1-minute load average.
# Run from the repository's root after `just moq-export moq-web ts-program` in ffi/, with
# NODE pointing at a Node 26 binary.
set -euo pipefail
rounds=${1:-5}
dir=$(realpath -m "${2:-.tmp/node-report}")
node=${NODE:?NODE must name a Node 26 binary}
out=$PWD/ffi/target/export/moq
inc=$(dirname "$(dirname "$(readlink -f "$node")")")/include/node
here=$(dirname "$0")
mkdir -p "$dir"
cp "$out/target/release/libmoq.so" "$dir/"
cc -O2 -std=c11 -Wall -Wextra -Werror -shared -fPIC -I "$out/include" "$here/nf.c" -L "$dir" -lmoq -Wl,-rpath,'$ORIGIN' -o "$dir/libnf.so"
cc -O2 -std=gnu11 -Wall -Wextra -Werror -Wno-unused-parameter -shared -fPIC -I "$inc" -I "$out/include" "$here/addon.c" -L "$dir" -lmoq -Wl,-rpath,'$ORIGIN' -o "$dir/addon.node"
cp "$here"/*.ts "$dir/"
wasm=ffi/target/export/moq-web/target/wasm32-wasip1/release/moq.wasm
: > "$dir/rounds.tsv"
for round in $(seq "$rounds"); do
	for route in wasm wasm-tf nf napi; do
		case $route in
			wasm) "$node" --no-warnings ffi/target/bench-ts/export.ts "$wasm" > "$dir/run.tsv" ;;
			# The module compiled by TurboFan up front, so no case runs before its tier-up.
			wasm-tf) "$node" --no-warnings --no-liftoff ffi/target/bench-ts/export.ts "$wasm" > "$dir/run.tsv" ;;
			*) "$node" --no-warnings "$dir/node.ts" "$route" "$dir" > "$dir/run.tsv" ;;
		esac
		load=$(cut -d' ' -f1 /proc/loadavg)
		awk -v route="$route" -v round="$round" -v avg="$load" -F'\t' 'NF == 3 { print route "\t" round "\t" $1 "\t" $2 "\t" $3 "\t" avg }' "$dir/run.tsv" >> "$dir/rounds.tsv"
	done
done
