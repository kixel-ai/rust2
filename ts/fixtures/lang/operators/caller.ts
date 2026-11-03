// The inputs `operators`' cases name, which run with the generated ones.

import { double, op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `operators::*` of every integer kind at every width, over operands at the edges of its
 * range: 0, 1, either end, the sign bit's neighbors, which make `MIN / -1`, and at 128 bits
 * values past 2^64. Shift amounts are at, past, and many times past the width, and negative
 * as an `i16`. Each form takes its turn at each pair.
 */
function ints(): Uint8Array[] {
	const widths: [string, number][] = [["u8s", 8], ["u16s", 16], ["u32s", 32], ["u64s", 64], ["u128s", 128], ["usizes", 32], ["i8s", 8], ["i16s", 16], ["i32s", 32], ["i64s", 64], ["i128s", 128], ["isizes", 32]];
	const cases: Uint8Array[] = [];
	for (const [name, bits] of widths) {
		const o = op(`operators::${name}`);
		const width = BigInt(bits);
		const mask = (1n << width) - 1n;
		const sign = 1n << (width - 1n);
		const edges = [0n, 1n, 3n, mask, mask - 1n, sign, sign - 1n, sign + 1n];
		if (bits === 128) edges.push(1n << 64n, (1n << 64n) + 3n, 0x9e3779b97f4a7c150123456789abcdefn);
		const be = (v: bigint) => Array.from({ length: bits / 8 }, (_, i) => Number((v >> BigInt(bits - 8 - 8 * i)) & 0xffn));
		// The amount's low byte and its high byte, which reads as `high << 24`; `0xff`
		// reads as a negative `i16`.
		const shifts = [[0, 0], [1, 0], [bits - 1, 0], [bits, 0], [bits + 1, 0], [2 * bits + 3, 0], [5, 1], [0xff, 0x80]];
		let i = 0;
		for (const a of edges) {
			for (const b of edges) {
				const [low, high] = shifts[i % shifts.length];
				for (let kind = 0; kind < 32; kind++) cases.push(Uint8Array.of(...o, kind, (i + kind) % 6, ...be(a), ...be(b), low, high));
				i++;
			}
		}
	}
	return cases;
}

/** Signed zeros, rounding, each type's extremes and subnormals, infinities, and NaN. */
const doubles = [0, -0, 1, -1, 0.1, 1 / 3, 1.5, -2.5, 2 ** 53, 3.4028235677973366e38, 1e-40, 5e-324, Number.MAX_VALUE, Infinity, -Infinity, NaN];

/** `operators::f64s` and `f32s` of every kind, over each pair of doubles. */
function floats(): Uint8Array[] {
	const cases: Uint8Array[] = [];
	for (const name of ["f64s", "f32s"]) {
		const o = op(`operators::${name}`);
		let i = 0;
		for (const a of doubles) {
			for (const b of doubles) {
				for (let kind = 0; kind < 16; kind++) cases.push(Uint8Array.of(...o, kind, (i + kind) % 6, ...double(a), ...double(b)));
				i++;
			}
		}
	}
	return cases;
}

/** `operators::bools` of every kind, pair, and form. */
const boolsOp = op("operators::bools");
const bools = Array.from({ length: 10 * 6 * 4 }, (_, i) => Uint8Array.of(...boolsOp, i % 10, Math.floor(i / 10) % 6, Math.floor(i / 60)));

export const cases: Uint8Array[] = [...ints(), ...floats(), ...bools];
