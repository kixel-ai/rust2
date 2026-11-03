// The inputs `ints`' cases name, which run with the generated ones.

import { be, edges, op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * The 64- and 128-bit cases over operands that random bytes rarely make: every pair of
 * values next to a power of two.
 */
function ints(): Uint8Array[] {
	const [u64, u128, i64, i128] = ["ints::u64_op", "ints::u128_op", "ints::i64_op", "ints::i128_op"].map(op);
	const [neg, cast, convert] = ["ints::neg", "ints::cast", "ints::convert"].map(op);
	const cases: Uint8Array[] = [];
	const values = edges();
	// Shift amounts in and past each limb, and past the width, which panics.
	const shifts = [0, 1, 7, 31, 32, 33, 63, 64, 65, 95, 96, 127, 128, 200];
	let i = 0;
	for (const x of values) {
		for (const y of values) {
			// A 64-bit case's operands are `x` and `y`. A 128-bit case's are `x:y` and `u:v`,
			// whose high halves are equal a third of the time, so the low half decides.
			const u = i % 3 === 0 ? x : values[(i * 7) % values.length];
			const v = values[(i * 13 + 5) % values.length];
			const shift = shifts[i++ % shifts.length];
			for (let kind = 0; kind < 31; kind++) {
				for (const o of [u64, i64]) cases.push(Uint8Array.of(...o, kind, ...be(x), ...be(y), shift));
				for (const o of [u128, i128]) cases.push(Uint8Array.of(...o, kind, ...be(x), ...be(y), ...be(u), ...be(v), shift));
			}
			// `clamp` between two values of the sweep, whose third operand follows the shift's place.
			for (const o of [u64, i64]) cases.push(Uint8Array.of(...o, 26, ...be(x), ...be(y), ...be(u)));
			for (const o of [u128, i128]) cases.push(Uint8Array.of(...o, 26, ...be(x), ...be(y), ...be(u), ...be(v), ...be(y), ...be(x)));
			// Negating an `i64` and an `i128`.
			cases.push(Uint8Array.of(...neg, 3, ...be(x)), Uint8Array.of(...neg, 4, ...be(x), ...be(y)));
			// `x * y`, `checked_mul`, `saturating_mul`, and `wrapping_mul` at 128 bits, extended in
			// each type's sign: a product of two numbers, past 2^53, multiplies their halves.
			const signs = (v: bigint) => (v >> 63n === 1n ? 0xffffffffffffffffn : 0n);
			for (const kind of [2, 13, 24, 25]) {
				cases.push(Uint8Array.of(...u128, kind, ...be(0n), ...be(x), ...be(0n), ...be(y)));
				cases.push(Uint8Array.of(...i128, kind, ...be(signs(x)), ...be(x), ...be(signs(y)), ...be(y)));
			}
			// `x:y / d`, `x:y % d`, `div_ceil`, and `is_multiple_of` at 128 bits by a divisor
			// below 2^32 in magnitude, which divides limbs by a number, extended in each type's
			// sign: -1 overflows `i128::MIN`.
			for (const d of [3n, 1_000_000n, 0xffffffffn, -1n, -0xffffffffn]) {
				for (const kind of [3, 4, 27, 28]) {
					for (const o of [u128, i128]) cases.push(Uint8Array.of(...o, kind, ...be(x), ...be(y), ...be(signs(d & 0xffffffffffffffffn)), ...be(d & 0xffffffffffffffffn)));
				}
			}
		}
		// Conversions, and halves joined back.
		cases.push(Uint8Array.of(...convert, 0, ...be(x), ...be(x)));
		for (const y of [0n, 1n, 1n << 32n, 1n << 63n, 0xffffffffffffffffn]) {
			for (const from of [3, 4, 9, 10]) {
				// A cast to every type.
				for (let to = 0; to < 12; to++) cases.push(Uint8Array.of(...cast, from, to, ...be(x), ...be(y)));
			}
		}
	}
	return cases;
}

/**
 * The first eight cases, each integer type's operators, over full-width values: each
 * `u64` boundary 2^k - 1, 2^k, and 2^k + 1, as each operand and with each selector.
 */
function boundaries(): Uint8Array[] {
	const cases: Uint8Array[] = [];
	for (const name of ["u8_op", "u16_op", "u32_op", "u64_op", "u128_op", "usize_op", "i8_op", "i16_op"]) {
		for (let k = 0n; k <= 64n; k++) {
			for (const delta of [-1n, 0n, 1n]) {
				const value = be(((1n << k) + delta) & 0xffffffffffffffffn);
				for (const selector of [0, 1, 2, 3, 4, 5, 6]) cases.push(Uint8Array.of(...op(`ints::${name}`), selector, ...value, ...value, ...value));
			}
		}
	}
	return cases;
}

export const cases: Uint8Array[] = [...ints(), ...boundaries()];
