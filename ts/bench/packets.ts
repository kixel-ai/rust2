// The `packets/` workloads' hand-written route: `bench.rs`'s op 15, as TypeScript numbers TS
// packets: each read and written at its offset in one `Uint8Array`, and a copy of one a
// `slice` of it.

const SIZE = 188;

/** Run op 15, as `bench::run` does, for the count `data` starts with. */
export function packets(selector: number, data: Uint8Array): Uint8Array {
	const padded = new Uint8Array(8);
	padded.set(data.subarray(0, 8));
	const count = Math.min(Number(new DataView(padded.buffer).getBigUint64(0)), 1 << 12);
	const bytes = new Uint8Array(count * SIZE);
	for (let i = 0; i < count; i++) {
		const at = i * SIZE;
		bytes[at] = 0x47;
		bytes[at + 2] = i % 3;
		bytes[at + 3] = i % 5 === 0 ? 0x20 : 0x10;
		bytes[at + SIZE - 1] = i;
	}
	const counters = new Uint8Array(3);
	let sum = 0;
	for (let at = 0; at < bytes.length; at += SIZE) {
		if (selector === 1) sum += bytes.slice(at, at + SIZE)[SIZE - 1];
		const pid = ((bytes[at + 1] & 0x1f) << 8) | bytes[at + 2];
		const next = pid % 3;
		let cc: number;
		if (bytes[at + 3] & 0x10) {
			cc = counters[next];
			counters[next] = (cc + 1) & 0x0f;
		} else {
			cc = (counters[next] - 1) & 0x0f;
		}
		bytes[at + 3] = (bytes[at + 3] & 0xf0) | cc;
		sum += cc;
	}
	const out = new Uint8Array(8);
	new DataView(out.buffer).setBigUint64(0, BigInt(sum));
	return out;
}
