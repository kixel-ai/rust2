// The inputs `wraps`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `wraps::*` of every kind at every width, over operands at the edges of its range: 0, 1,
 * either end, the sign bit's neighbors, which make `MIN / -1`, and at 128 bits values past
 * 2^64. Shift amounts are at, past, and many times past the width, which std masks.
 */
function wraps(): Uint8Array[] {
	const widths: [string, number][] = [["u8s", 8], ["u16s", 16], ["u32s", 32], ["u64s", 64], ["u128s", 128], ["usizes", 32], ["i8s", 8], ["i16s", 16], ["i32s", 32], ["i64s", 64], ["i128s", 128], ["isizes", 32]];
	const cases: Uint8Array[] = [];
	for (const [name, bits] of widths) {
		const o = op(`wraps::${name}`);
		const width = BigInt(bits);
		const mask = (1n << width) - 1n;
		const sign = 1n << (width - 1n);
		const edges = [0n, 1n, 3n, mask, mask - 1n, sign, sign - 1n, sign + 1n];
		if (bits === 128) edges.push(1n << 64n, (1n << 64n) + 3n, 0x9e3779b97f4a7c150123456789abcdefn);
		const be = (v: bigint) => Array.from({ length: bits / 8 }, (_, i) => Number((v >> BigInt(bits - 8 - 8 * i)) & 0xffn));
		// The amount's low byte and its high byte, which reads as `high << 24`.
		const shifts = [[0, 0], [1, 0], [bits - 1, 0], [bits, 0], [bits + 1, 0], [2 * bits + 3, 0], [5, 1]];
		let i = 0;
		for (const a of edges) {
			for (const b of edges) {
				const c = edges[(i * 5 + 3) % edges.length];
				const [low, high] = shifts[i++ % shifts.length];
				for (let kind = 0; kind < 24; kind++) cases.push(Uint8Array.of(...o, kind, ...be(a), ...be(b), ...be(c), low, high));
			}
		}
	}
	return cases;
}

export const cases: Uint8Array[] = [...wraps()];
