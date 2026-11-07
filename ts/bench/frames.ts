// The `bytes/` workloads' hand-written route: `bench.rs`'s op 18 with `subarray`, as
// moq's TypeScript hands a frame's bytes to its consumers, sharing the group's buffer
// with no count of its handles.

/** Run op 18, as `bench::run` does, for the frame count and size `data` starts with. */
export function frames(data: Uint8Array): Uint8Array {
	const padded = new Uint8Array(16);
	padded.set(data.subarray(0, 16));
	const params = new DataView(padded.buffer);
	const count = Math.min(Number(params.getBigUint64(0)), 1 << 16);
	const size = Math.min(Math.max(Number(params.getBigUint64(8)), 1), 1 << 10);
	let group = new Uint8Array(count * size).fill(0x5a);
	let sum = 0;
	let unique = 0;
	for (let i = 0; i < count; i++) {
		const frame = group.subarray(0, size);
		group = group.subarray(size);
		const rest = frame.subarray(1);
		sum += frame[0] + rest.length;
		// The group the last frame took whole is empty, which `bytes` makes static, of no storage.
		if (group.length > 0) unique++;
	}
	const out = new DataView(new ArrayBuffer(16));
	out.setBigUint64(0, BigInt(sum));
	out.setBigUint64(8, BigInt(unique));
	return new Uint8Array(out.buffer);
}
