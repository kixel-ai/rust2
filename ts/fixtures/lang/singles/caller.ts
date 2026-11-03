// The inputs `singles`' cases name, which run with the generated ones.

import { op as pick } from "./harness.ts";
import { mulAdds, op, picked, ulpsWithin, wides } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** A single's bits, big-endian, as `singles` takes them. */
function single(x: number | bigint): number[] {
	const bytes = new Uint8Array(4);
	const view = new DataView(bytes.buffer);
	if (typeof x === "bigint") view.setUint32(0, Number(x));
	else view.setFloat32(0, x);
	return [...bytes];
}

/**
 * `singles`' operands: each side of a half, of 2^23 and 2^24, where each integer type
 * saturates, where a single's range and its normals end, the bounds of `Debug`'s
 * exponent, infinities, and NaNs, one negative, one with a payload, and one signaling.
 */
const singlesValues: (number | bigint)[] = [
	0, -0, 0.5, -0.5, 1.5, 2.5, -2.5, 0.49999997, 1, -1, 3, 0.1, 1 / 3, 2 ** 23 - 0.5, 2 ** 23, 2 ** 24, 2 ** 24 + 2, 255.5, -128.5,
	2147483520, 2 ** 31, -(2 ** 31), 4294967040, 2 ** 32, 2 ** 63, -(2 ** 63), 2 ** 64, 2 ** 127, 1e-9, 0.99999994, 1.0000001, 1e-4, 1e16,
	1e-40, 2 ** -149, 2 ** -126, 3.4028234663852886e38, -3.4028234663852886e38, Infinity, -Infinity, NaN, 0xffc00000n, 0x7fc00123n, 0x7f800001n,
];
const singlesOp = op("singles::singles");
const singlesCases = [
	...Array.from({ length: 10 }, (_, kind) => singlesValues.flatMap((a) => singlesValues.map((b) => Uint8Array.of(...singlesOp, kind, ...single(a), ...single(b), 1)))).flat(),
	// `from_secs_f32`, `mul_f32`, and `div_f32` panicking, and `Duration::MAX.mul_f32`.
	...[18, 28, 38].flatMap((kind) => singlesValues.map((a) => Uint8Array.of(...singlesOp, kind, ...single(a), ...single(a), 3))),
	// Integers whose `f32` rounds once where through an `f64` it would round twice: past
	// 2^53, a bit just past halfway between two singles.
	...wides.concat([(1n << 60n) + (1n << 36n) + 1n, (1n << 64n) - (1n << 39n) - 1n, (1n << 127n) + (1n << 103n) + 1n, (1n << 24n) + 1n]).map((v) => {
		const bytes = new Uint8Array(16);
		new DataView(bytes.buffer).setBigUint64(0, BigInt.asUintN(64, v >> 64n));
		new DataView(bytes.buffer).setBigUint64(8, BigInt.asUintN(64, v));
		return Uint8Array.of(...singlesOp, 2, ...bytes);
	}),
];

/** `singles::formats`' singles: ties at each precision, carries through nines, and the shortest digits at each end. */
const singlesFormatted: (number | bigint)[] = [
	...singlesValues,
	0.125, 0.375, 3.5, 9.5, 99.5, 0.95, 9.995, 1.25, 1.35, 2.675, 9.999999e-5, 1e15, 9999999e9, 1e-7, 123456.79, 16777215, 7.006492e-46, 1.17549421e-38, 8388609,
];
const singlesFormatsOp = op("singles::formats");
const singlesParsesOp = op("singles::parses");
const singlesFormats = [
	...singlesFormatted.flatMap((a) => Array.from({ length: 12 }, (_, p) => Uint8Array.of(...singlesFormatsOp, 0, ...single(a), p))),
	...singlesFormatted.flatMap((a) => [1, 3, 4].map((kind) => Uint8Array.of(...singlesFormatsOp, kind, ...single(a), 0))),
	...singlesFormatted.flatMap((a) => [[1, 2], [2, 1], [NaN, 1], [1, NaN], [-0, 0], [0, -0]].map(([b, c]) => Uint8Array.of(...singlesFormatsOp, 2, ...single(a), ...single(b), ...single(c)))),
	...singlesValues.map((a) => Uint8Array.of(...singlesParsesOp, 2, ...single(a))),
];

/**
 * `singles::parses`' texts: std's grammar, past each end of a single's range, and
 * decimals within a hair of halfway between two singles, which rounding through the
 * nearest double would round twice.
 */
const singlesParsed = [
	"", "+", "-", ".", "e5", "1e", "1.e5", ".5", "+1", "1_000", " 1", "1e40", "-1e40", "1e-50", "-0", "inf", "-infinity", "NaN", "nan1",
	"3.4028235e38", "3.4028236e38", "3.40282357e38", "3.4028235677973366e38", "3.4028235677973367e38", "1.4e-45", "7e-46",
	"7.006492321624085354618647916449580656401e-46", "7.006492321624085354618647916449580656402e-46", "1.1754942e-38", "1.17549435e-38",
	"16777217", "16777217.000000000001", "16777218.99999999999", "16777219", "1.00000005960464477539062499", "1.000000059604644775390625",
	"1.00000005960464477539062501", "1.0000000596046448", "0.1", "1e10", "1234567e-10", "12345678e-10", "9007199254740993",
	`1${"0".repeat(60)}e-60`, `0.${"0".repeat(60)}1e61`, "1e65535", "1e-65536",
].map((text) => Uint8Array.of(...singlesParsesOp, 0, ...new TextEncoder().encode(text)));

const singlesMulAdds = mulAdds(3.4028234663852886e38, 2 ** -149, 2 ** -23, op("singles::mul_adds"), single);

const singlesAt = pick("singles::singles");

export function within(input: Uint8Array, native: Uint8Array, translated: Uint8Array): boolean {
	return picked(input) === singlesAt && input[2] % 10 === 7 && ulpsWithin(4, native, translated);
}

export const cases: Uint8Array[] = [...singlesCases, ...singlesFormats, ...singlesParsed, ...singlesMulAdds];
