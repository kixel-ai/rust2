// The inputs `methods`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `methods::*` of every kind at every width, over operands at the edges of its range:
 * both ends and their neighbors, the sign bit's, small values, and where they reach,
 * either side of 2^53 and 2^64 and their negations. Each meets 0, 1, -1, 2, 10, and
 * either end, which make `MIN / -1`, `MIN % -1`, and a zero divisor or base, and
 * exponents and shifts in, at, and past the width.
 */
function methods(): Uint8Array[] {
	const widths: [string, number, boolean][] = [["u8s", 8, false], ["u16s", 16, false], ["u32s", 32, false], ["u64s", 64, false], ["u128s", 128, false], ["usizes", 32, false], ["i8s", 8, true], ["i16s", 16, true], ["i32s", 32, true], ["i64s", 64, true], ["i128s", 128, true], ["isizes", 32, true]];
	// The kinds that read the `u32`, and the others, which read `b`.
	const exponents = new Set([3, 4, 7, 8, 21, 22, 23, 38, 39, 40, 45, 46, 47]);
	const cases: Uint8Array[] = [];
	for (const [name, bits, signed] of widths) {
		const o = op(`methods::${name}`);
		const width = BigInt(bits);
		const min = signed ? -(1n << (width - 1n)) : 0n;
		const max = (1n << (signed ? width - 1n : width)) - 1n;
		const sign = 1n << (width - 1n);
		const values = [min, min + 1n, min + 2n, -2n, -1n, 0n, 1n, 2n, 3n, 10n, 99n, max - 1n, max, sign - 1n, sign, sign + 1n];
		for (const k of [53n, 64n]) {
			if (k < width) values.push(...[-1n, 0n, 1n].flatMap((d) => [(1n << k) + d, -(1n << k) + d]));
		}
		const edges = [...new Set(values.filter((v) => v >= min && v <= max))];
		const others = [0n, 1n, -1n, 2n, 10n, min, max].filter((v) => v >= min && v <= max);
		const shifts = [0, 1, 2, bits - 1, bits, bits + 1, 2 * bits, 3 * bits - 1, 0x80000000, 0xffffffff];
		const be = (v: bigint) => Array.from({ length: bits / 8 }, (_, i) => Number((v >> BigInt(bits - 8 - 8 * i)) & 0xffn));
		const u32 = (n: number) => [n >>> 24, (n >>> 16) & 0xff, (n >>> 8) & 0xff, n & 0xff];
		let i = 0;
		for (let kind = 0; kind < 51; kind++) {
			for (const a of edges) {
				if (exponents.has(kind)) {
					for (const n of shifts) cases.push(Uint8Array.of(...o, kind, ...be(a), ...be(others[i++ % others.length]), ...u32(n)));
				} else {
					for (const b of others) cases.push(Uint8Array.of(...o, kind, ...be(a), ...be(b), ...u32(shifts[i++ % shifts.length])));
				}
			}
		}
	}
	return cases;
}

export const cases: Uint8Array[] = [...methods()];
