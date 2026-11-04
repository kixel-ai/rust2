// A hand-written caller of a fixture's translation, as moq's TypeScript is: it runs the
// harness, which picks each case by its first two bytes. `differential.ts` runs it beside
// native Rust's, from the output directory, as it does a fixture's own `caller.ts`, which
// adds the inputs its cases name from what this module shares.

import { op as pick, run } from "./harness.ts";
import { cases } from "./lib.ts";

export { run };

/** The first two bytes, which pick the case named `name`. */
export function op(name: string): number[] {
	const at = pick(name);
	return [at >> 8, at & 0xff];
}

/** Each case's name and the bytes that pick it, through `op`, which panics for one past what two bytes pick. */
export const ops = new Map(cases().items.map(([name]): [string, Uint8Array] => [name, Uint8Array.from(op(name))]));

/** Which case the first two bytes of `input` pick. */
export function picked(input: Uint8Array): number {
	return ((input[0] << 8) | input[1]) % ops.size;
}

/** A double's bits, big-endian, as `floats` takes them. */
export function double(x: number | bigint): number[] {
	const bytes = new Uint8Array(8);
	const view = new DataView(bytes.buffer);
	if (typeof x === "bigint") view.setBigUint64(0, x);
	else view.setFloat64(0, x);
	return [...bytes];
}

/** Integers whose `f64` rounds, as `floats`' kind 2 takes them: 16 bytes of a `u128`. */
export const wides = [0n, 1n, (1n << 53n) + 1n, (1n << 53n) + 3n, (1n << 54n) + 2n, (1n << 63n) + 1025n, (1n << 64n) - 1n, (1n << 64n) - 1025n, (1n << 64n) + (1n << 11n) + 1n, (1n << 127n) + 1n, (1n << 128n) - 1n, (1n << 128n) - (1n << 74n), -(1n << 100n) - 1n];

/**
 * `mul_adds`' operands at each width: signed zeros, a product that underflows to either
 * zero, infinities and NaN, a product past the type's range whose sum is in it, subnormal
 * sums and ties, products whose error alone survives, which rounding twice loses, and a
 * product of half `c`'s last place, which only its error moves off a tie; then
 * seeds for its sweeps.
 */
export const mulAdds = (max: number, tiny: number, eps: number, op: number[], bytes: (x: number) => number[]) => [
	...[
		[0, 1, -0], [-0, 1, -0], [0, -1, 0], [-0, -0, -0], [-0, 0, -0], [1, -1, 1], [-1, 1, 1],
		[tiny, -(2 ** -30), 0], [tiny, 2 ** -30, -0], [-tiny, 2 ** -30, -0], [tiny, 0.5, 0], [tiny, 0.5, -0], [-tiny, 0.5, 0],
		[tiny, 1.5, 0], [tiny, 0.5, tiny], [tiny, -0.5, tiny], [tiny, 2 ** 52, tiny],
		[Infinity, 0, 1], [0, -Infinity, 1], [Infinity, 1, -Infinity], [Infinity, -1, -Infinity], [1, 1, Infinity], [max, 2, -Infinity], [max, -2, Infinity],
		[NaN, 1, 1], [1, NaN, 1], [1, 1, NaN], [Infinity, 1, NaN],
		[max, 2, -max], [max, -2, max], [max, 1 + eps, -max], [max, 1 + eps, 0], [max * 0.5, 2, max * eps * 0.25],
		[1 + 2 ** -27, 1 - 2 ** -27, -1], [1 + eps, 1 + eps, -1], [1 + eps, 1 - eps, -1], [0.1, 10, -1], [3, 1 / 3, -1], [1 + eps, 1 + eps, -(1 + 2 * eps)],
		[1 + 2 ** -12, 1 + 2 ** -12, tiny], [1 + 2 ** -12, 1 + 2 ** -12, -tiny], [1 + eps, 1 + eps / 2, 0], [eps / 2 + eps * eps / 2, 1 - eps / 2, 1], [eps / 2 + eps * eps / 2, 1 - eps / 2, -1],
	].map((xs) => Uint8Array.of(...op, 0, ...xs.flatMap(bytes))),
	...Array.from({ length: 400 }, (_, i) => Uint8Array.of(...op, 1, ...double(BigInt(i + 1) * 0x9e3779b97f4a7c15n & 0xffff_ffff_ffff_ffffn))),
];

/** How many ULPs from native Rust's each float of `floats`' and `singles`' kind 7 may be, which Rust leaves to libm. */
const ULPS = 2n;

/** Whether each float of `width` bytes in two outputs is within `ULPS` of the other's. */
export function ulpsWithin(width: number, native: Uint8Array, translated: Uint8Array): boolean {
	if (native.length !== translated.length) return false;
	const [a, b] = [native, translated].map((v) => new DataView(v.buffer, v.byteOffset, v.length));
	const read = (v: DataView, i: number) => (width === 8 ? v.getBigInt64(i) : BigInt(v.getInt32(i)));
	// Bits as a line through zero, so the distance between two is how many floats apart they are.
	const magnitude = (1n << BigInt(width * 8 - 1)) - 1n;
	const line = (v: bigint) => (v < 0n ? -(v & magnitude) : v);
	for (let i = 0; i < native.length; i += width) {
		const d = line(read(a, i)) - line(read(b, i));
		if (d > ULPS || d < -ULPS) return false;
	}
	return true;
}

/**
 * 64-bit operands that random bytes rarely make: 2^k - 1, 2^k, and 2^k + 1 for each k
 * where a limb, a safe number, or a sign ends, and values without that structure, whose
 * quotients and remainders are not round.
 */
export function edges(): bigint[] {
	const edges = new Set<bigint>([
		1_000_000n,
		0x9e3779b97f4a7c15n,
		0x0123456789abcdefn,
		0xfedcba9876543210n,
		// Safe integers whose halves carry when multiplied, and a negative one.
		0x1d3c5b7a9f8e6dn,
		-0x1b2a3c4d5e6f71n & 0xffffffffffffffffn,
	]);
	for (const k of [0n, 1n, 16n, 21n, 31n, 32n, 33n, 52n, 53n, 54n, 63n, 64n]) {
		for (const delta of [-1n, 0n, 1n]) edges.add(((1n << k) + delta) & 0xffffffffffffffffn);
	}
	// Small negative numbers, -2^32, and where negative safe numbers end, as signed values.
	for (const k of [1n, 32n, 53n]) {
		for (const delta of [-1n, 0n, 1n]) edges.add((-(1n << k) + delta) & 0xffffffffffffffffn);
	}
	return [...edges];
}

/** A `u64`'s bytes, big-endian. */
export function be(v: bigint): number[] {
	const bytes = new Uint8Array(8);
	new DataView(bytes.buffer).setBigUint64(0, v);
	return [...bytes];
}
