// The `push/` workloads' hand-written route: `bench.rs`'s op 13, as TypeScript grows a
// list: `push` onto an array, for a `VecDeque` too, as JS has no deque, and `pop` for a stack.

/** Run op 13, as `bench::run` does, by `selector` for the count `data` starts with. */
export function push(selector: number, data: Uint8Array): Uint8Array {
	const padded = new Uint8Array(8);
	padded.set(data.subarray(0, 8));
	const count = Math.min(Number(new DataView(padded.buffer).getBigUint64(0)), 1 << 16);
	const v: number[] = [];
	const out = new Uint8Array(16);
	const view = new DataView(out.buffer);
	if (selector === 2) {
		let sum = 0;
		for (let i = 0; i < count; i++) {
			v.push(i);
			if (i % 4 === 3) sum = (sum + (v.pop() ?? 0)) >>> 0;
		}
		for (let x = v.pop(); x !== undefined; x = v.pop()) sum = (sum + x) >>> 0;
		view.setBigUint64(0, BigInt(v.length));
		view.setBigUint64(8, BigInt(sum));
		return out;
	}
	for (let i = 0; i < count; i++) v.push(i);
	view.setBigUint64(0, BigInt(v.length));
	view.setBigUint64(8, BigInt(v[v.length >> 1] ?? 0));
	return out;
}
