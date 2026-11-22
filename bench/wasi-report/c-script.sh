#!/usr/bin/env bash
# The api fixture's C script over the C ABI, compiled for wasm32-wasip1 with the glue and
# run under `wasmtime run`, against native Rust's transcript on the same target. Run from
# the repo root in the dev shell, after `just ffi test --test fixtures -- --exact api_c`.
set -euo pipefail
api=ffi/target/tmp/api
out=.tmp/wasi-c
mkdir -p "$out"
sysroot=$("$RUST2_TS_TOOLCHAIN/bin/rustc" --print sysroot)/lib/rustlib/wasm32-wasip1/lib/self-contained
(cd ffi && PATH=$RUST2_TS_TOOLCHAIN/bin:$PATH cargo build --quiet --locked --manifest-path "../$api/out/Cargo.toml" \
	--target-dir ../.tmp/glue-wasi --target wasm32-wasip1)
(cd ffi && PATH=$RUST2_TS_TOOLCHAIN/bin:$PATH cargo build --quiet --locked --package api-fixture --example transcript \
	--features doubled --target wasm32-wasip1 --target-dir ../.tmp/transcript-wasm)
# The C test's two 64-bit assumptions, neither in the library: isize's bounds as 64-bit
# literals, and a reader whose position is a size_t. Its stdout unbuffered, since its
# host-only checks abort after the transcript.
mkdir -p "$out/transcript"
sed 's/((ptrdiff_t)(-9223372036854775807LL - 1))/PTRDIFF_MIN/; s/((ptrdiff_t)9223372036854775807LL)/PTRDIFF_MAX/' \
	"$api/c-transcript/transcript.c" > "$out/transcript/transcript.c"
sed -e 's/^\tsize_t at;$/\tuint64_t at;/; s/m->at = (size_t)(uint64_t)offset;/m->at = (uint64_t)offset;/' \
	-e 's/^int main(int argc, char \*\*argv) {$/&\n\tsetvbuf(stdout, NULL, _IONBF, 0);/' \
	ffi/tests/api.c > "$out/api.c"
# zig compiles; Rust's own wasi-libc links, since zig's spins in pthread_key_delete, which
# std's thread-local destructors call.
nix shell nixpkgs#zig nixpkgs#lld nixpkgs#wasmtime --command bash -c "
	zig cc -target wasm32-wasi -std=c11 -O1 -I $api/out/include -I $out/transcript -c $out/api.c -o $out/api.o
	wasm-ld -o $out/api.wasm $sysroot/crt1-command.o $out/api.o .tmp/glue-wasi/wasm32-wasip1/debug/libapi.a $sysroot/libc.a
	wasmtime run .tmp/transcript-wasm/wasm32-wasip1/debug/examples/transcript.wasm > $out/native.txt
	wasmtime run $out/api.wasm > $out/c.txt 2>/dev/null || true
	for c in statuses moved borrowed utf8 finished enum bound array future started start lent 'lent moved' null returned reader outlived 'lent shared' zero; do
		printf '%s: ' \"\$c\"; wasmtime run $out/api.wasm \"\$c\" 2>&1 >/dev/null | head -1
	done"
diff "$out/native.txt" <(head -n "$(wc -l < "$out/native.txt")" "$out/c.txt") && echo "the transcript matches native Rust's on wasm32-wasip1"
