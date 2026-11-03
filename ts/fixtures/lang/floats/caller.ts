// The inputs `floats`' cases name, which run with the generated ones.

import { mulAddsWithin } from "./fma.ts";
import { op as pick } from "./harness.ts";
import { double, mulAdds, op, picked, ulpsWithin, wides } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `floats`' operands: each side of a half, of 2^52 and 2^53, where each integer type
 * saturates, of a nanosecond and `u64::MAX` seconds, subnormals, infinities, and NaNs,
 * one of them negative and one with a payload.
 */
const doubles: (number | bigint)[] = [
	0, -0, 0.5, -0.5, 1.5, 2.5, -2.5, 0.49999999999999994, 1, -1, 3, 0.1, 1 / 3, 2 ** 52 - 0.5, 2 ** 52, 2 ** 53, 2 ** 53 + 2,
	255.5, -128.5, 2147483647.5, -2147483648.5, 4294967295.9, 2 ** 63, -(2 ** 63), -(2 ** 63) - 2048, 2 ** 64, 2 ** 64 - 2048,
	2 ** 127, 2 ** 128, -(2 ** 128), 1e-9, 0.5e-9, 1.5e-9, 0.999999999, 0.9999999995, 1.0000000005, 4.656612873077393e-10,
	1e-310, 5e-324, 2.2250738585072014e-308, Number.MAX_VALUE, -Number.MAX_VALUE, Infinity, -Infinity, NaN,
	0xfff8000000000000n, 0x7ff0000000000001n,
];

const floatsOp = op("floats::floats");

const floats = [
	...Array.from({ length: 10 }, (_, kind) => doubles.flatMap((a) => doubles.map((b) => Uint8Array.of(...floatsOp, kind, ...double(a), ...double(b), 1)))).flat(),
	// `from_secs_f64`, `mul_f64`, and `div_f64` panicking, and `Duration::MAX.mul_f64`.
	...[18, 28, 38].flatMap((kind) => doubles.map((a) => Uint8Array.of(...floatsOp, kind, ...double(a), ...double(a), 3))),
	...wides.map((v) => {
		const bytes = new Uint8Array(16);
		new DataView(bytes.buffer).setBigUint64(0, BigInt.asUintN(64, v >> 64n));
		new DataView(bytes.buffer).setBigUint64(8, BigInt.asUintN(64, v));
		return Uint8Array.of(...floatsOp, 2, ...bytes);
	}),
];

/**
 * `floats::formats`' doubles: ties at each precision, which std rounds half to even on the
 * exact value, carries through nines, each side of where `Debug` takes an exponent, the
 * shortest digits that are not the nearest ones, and the extremes.
 */
const formatted: (number | bigint)[] = [
	...doubles,
	0.125, 0.375, 0.625, 0.875, 0.0625, 3.5, 4.5, 9.5, 99.5, 999.5, 0.95, 9.995, 1.25, 1.35, 2.675, 1e-4, 9.999999999999999e-5,
	1e16, 9999999999999998, 1e21, 1e22, 1e23, 5e-324, 2.225073858507201e-308, 123456789.125, 1.5e300, -1.5e-300, 2 ** -1022 * 1.5,
	0.1 + 0.2, 100, 1e15, 1e-7, 0.000123, -9.5, -0.5,
];
const formatsOp = op("floats::formats");
const formats = [
	...formatted.flatMap((a) => Array.from({ length: 22 }, (_, p) => Uint8Array.of(...formatsOp, 0, ...double(a), p))),
	...formatted.flatMap((a) => [1, 3, 4].map((kind) => Uint8Array.of(...formatsOp, kind, ...double(a), 0))),
	// Seeds for `parses`' decimals, a signaling NaN's bits among them, which a seed keeps.
	...doubles.map((a) => Uint8Array.of(...op("floats::parses"), 2, ...double(a))),
	...formatted.flatMap((a) => [[1, 2], [2, 1], [NaN, 1], [1, NaN], [-0, 0], [0, -0]].map(([b, c]) => Uint8Array.of(...formatsOp, 2, ...double(a), ...double(b), ...double(c)))),
];

/**
 * `floats::parses`' texts: std's grammar at each edge, past each end of a double's range,
 * and digits that round half to even, or just off it past 19 digits.
 */
const parsed = [
	"", "+", "-", ".", "+.", "-.", "e5", ".e5", "1e", "1e+", "1e-", "1.e5", ".5", "5.", "-.5e-3", "+1", "++1", "+-1", "1_000", "0x10", " 1",
	"1 ", "1,5", "1.5.5", "1e5e5", "1E+05", "1e400", "-1e400", "1e-400", "1e99999999999999999999", "0e99999999999", "-0", "+0.0e-5",
	"0000.0000", "inf", "-inf", "+inf", "infinity", "INFINITY", "InFiNiTy", "infin", "in", "infinityy", "nan", "NaN", "-nan", "+NAN",
	"nan1", "Infinity", "i\u0307nf", "\u0131nf", "2.2250738585072011e-308", "2.2250738585072012e-308", "4.9406564584124654e-324",
	"2.4703282292062327e-324", "2.4703282292062328e-324", "1.7976931348623157e308", "1.7976931348623158e308", "1.7976931348623159e308",
	"9007199254740993", "9007199254740993.0000000000000000001", "9007199254740992.9999999999999999999", "123456789012345678901234567890",
	"0.1000000000000000055511151231257827021181583404541015625", "1e23", "8.98846567431158e307", `0.${"0".repeat(400)}1e400`,
	`1${"0".repeat(800)}e-800`, `${"9".repeat(800)}e-1100`, "1e65535", `1e-65536`, `0.${"0".repeat(70000)}1e70000`,
].map((text) => Uint8Array.of(...op("floats::parses"), 0, ...new TextEncoder().encode(text)));

const floatsMulAdds = mulAdds(Number.MAX_VALUE, 5e-324, 2 ** -52, op("floats::mul_adds"), double);

const [floatsAt, mulAddsAt] = [pick("floats::floats"), pick("floats::mul_adds")];

export function within(input: Uint8Array, native: Uint8Array, translated: Uint8Array): boolean {
	const at = picked(input);
	if (at === mulAddsAt) return mulAddsWithin(native, translated);
	return at === floatsAt && input[2] % 10 === 7 && ulpsWithin(8, native, translated);
}

export const cases: Uint8Array[] = [...floats, ...formats, ...parsed, ...floatsMulAdds];
