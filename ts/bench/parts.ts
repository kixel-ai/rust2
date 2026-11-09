// The `parts/` workloads' hand-written route: `bench.rs`'s op 17 on a plain array, as
// TypeScript sifts a heap and writes a prefix in place, by index into the whole array.

/** Run op 17, as `bench::run` does, by `selector` for the step count and size `data` starts with. */
export function parts(selector: number, data: Uint8Array): Uint8Array {
	const padded = new Uint8Array(16);
	padded.set(data.subarray(0, 16));
	const params = new DataView(padded.buffer);
	const count = Number(params.getBigUint64(0));
	const size = Math.min(Math.max(Number(params.getBigUint64(8)), 1), 1 << 16);
	const v = Array.from({ length: size }, (_, i) => (i * 40503) % 65536);
	let sum = 0;
	if (selector === 0) {
		v.sort((a, b) => a - b);
		for (let step = 0; step < count; step++) {
			v[0] += 1 + (step % 64) * 1024;
			let i = 0;
			for (;;) {
				const l = 2 * i + 1;
				if (l >= size) break;
				const c = l + 1 < size && v[l + 1] < v[l] ? l + 1 : l;
				if (v[i] <= v[c]) break;
				[v[i], v[c]] = [v[c], v[i]];
				i = c;
			}
			sum += v[0];
		}
	} else {
		for (let step = 0; step < count; step++) {
			const k = (step % size) + 1;
			for (let j = 0; j < k; j++) v[j] += 1;
			sum += v[k - 1];
		}
	}
	const out = new Uint8Array(16);
	const view = new DataView(out.buffer);
	view.setBigUint64(0, BigInt(v[0]));
	view.setBigUint64(8, BigInt(sum));
	return out;
}
