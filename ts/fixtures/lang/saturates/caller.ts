// The inputs `saturates`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `saturates::*` of every kind at every width, over operands at the edges of its range and
 * across them: 0, 1, either end, the sign bit's neighbors, which make `MIN / -1` and
 * `MIN % -1`, and at 128 bits values past 2^64.
 */
function saturates(): Uint8Array[] {
	const widths: [string, number][] = [["u8s", 8], ["u16s", 16], ["u32s", 32], ["u64s", 64], ["u128s", 128], ["usizes", 32], ["i8s", 8], ["i16s", 16], ["i32s", 32], ["i64s", 64], ["i128s", 128], ["isizes", 32]];
	const cases: Uint8Array[] = [];
	for (const [name, bits] of widths) {
		const o = op(`saturates::${name}`);
		const width = BigInt(bits);
		const mask = (1n << width) - 1n;
		const sign = 1n << (width - 1n);
		const half = sign >> 1n;
		const edges = [0n, 1n, 3n, mask, mask - 1n, sign, sign - 1n, sign + 1n, half, half + 1n, mask - half];
		if (bits === 128) edges.push(1n << 64n, (1n << 64n) + 3n, 0x9e3779b97f4a7c150123456789abcdefn);
		const be = (v: bigint) => Array.from({ length: bits / 8 }, (_, i) => Number((v >> BigInt(bits - 8 - 8 * i)) & 0xffn));
		let i = 0;
		for (const a of edges) {
			for (const b of edges) {
				const c = edges[(i++ * 5 + 3) % edges.length];
				for (let kind = 0; kind < 24; kind++) cases.push(Uint8Array.of(...o, kind, ...be(a), ...be(b), ...be(c)));
			}
		}
	}
	return cases;
}

export const cases: Uint8Array[] = [...saturates()];
