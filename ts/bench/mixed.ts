// The `int/` workloads' hand-written route: `bench.rs`'s op 23 with `bigint` wrapped to 64
// bits, as TypeScript keeps a hash of that width.

/** Run op 23, as `bench::run` does: the xor of every round's result. */
export function mixed(selector: number, data: Uint8Array): Uint8Array {
	const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
	const [count, seed] = [0, 1].map((i) => view.getBigUint64(i * 8));
	const wrap = (v: bigint) => BigInt.asUintN(64, v);
	let h = seed;
	let sum = 0n;
	for (let i = 0n; i < count; i++) {
		if (selector === 0) {
			h = wrap(h + 0x9e3779b97f4a7c15n);
			let z = wrap((h ^ (h >> 30n)) * 0xbf58476d1ce4e5b9n);
			z = wrap((z ^ (z >> 27n)) * 0x94d049bb133111ebn);
			sum ^= z ^ (z >> 31n);
		} else {
			h = wrap(wrap(h * 0x9e3779b97f4a7c15n) + i);
			sum ^= wrap(h / 0x100000001n + (h % 0xffffffffffffffc5n));
		}
	}
	const out = new Uint8Array(8);
	new DataView(out.buffer).setBigUint64(0, sum);
	return out;
}
