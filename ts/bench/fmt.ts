// The `fmt/` workload's hand-written route: `bench.rs`'s op 16, as TypeScript builds a log
// line: a template literal with `padStart` and `toFixed`.

/** Run op 16, as `bench::run` does, for the count `data` starts with. */
export function format(data: Uint8Array): Uint8Array {
	const padded = new Uint8Array(8);
	padded.set(data.subarray(0, 8));
	const count = Math.min(Number(new DataView(padded.buffer).getBigUint64(0)), 1 << 16);
	let s = "";
	for (let i = 0; i < count; i++) s += `${String(i).padStart(6)}|Point { x: ${i}, y: ${-i} }|${(i / 3).toFixed(2)}|${-i}\n`;
	let sum = 0;
	for (let i = 0; i < s.length; i++) sum += s.charCodeAt(i);
	const out = new Uint8Array(16);
	const view = new DataView(out.buffer);
	view.setBigUint64(0, BigInt(s.length));
	view.setBigUint64(8, BigInt(sum));
	return out;
}
