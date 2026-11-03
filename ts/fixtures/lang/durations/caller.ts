// The inputs `durations`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** A `Duration`'s bytes as `durations` takes them: its seconds, then its nanoseconds. */
function duration(secs: bigint, nanos: number): number[] {
	const bytes = new Uint8Array(12);
	const view = new DataView(bytes.buffer);
	view.setBigUint64(0, secs);
	view.setUint32(8, nanos);
	return [...bytes];
}

/**
 * `durations` where seconds leave a safe integer or reach `u64::MAX`, nanoseconds carry,
 * a sum carries them past it, and a factor makes the nanoseconds' product or a
 * quotient's remainder pass 2^53.
 */
const edges = [0n, 1n, (1n << 53n) - 1n, 1n << 53n, (1n << 64n) - 1n].flatMap((secs) => [0, 1, 999_999_999].map((nanos) => duration(secs, nanos)));
const factors = [0, 1, 3, 1000, 9_007_199, 1_000_000_000, 0xffffffff].map((n) => [n >>> 24, (n >>> 16) & 0xff, (n >>> 8) & 0xff, n & 0xff]);
const durationsOp = op("durations::durations");
const durations = [
	...[0n, 1n, 1n << 32n, (1n << 53n) - 1n, 1n << 53n, (1n << 64n) - 2n, (1n << 64n) - 1n].flatMap((secs) =>
		[0, 1, 999_999_999, 1_000_000_000, 0xffffffff].flatMap((nanos) => [0, 1].map((kind) => Uint8Array.of(...durationsOp, kind, ...duration(secs, nanos)))),
	),
	...edges.flatMap((a) => edges.flatMap((b) => factors.flatMap((n) => [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13].map((kind) => Uint8Array.of(...durationsOp, kind, ...a, ...b, ...n))))),
	// Each side of where minutes, hours, and a `u128` of nanoseconds pass `u64::MAX` seconds.
	...[2n ** 64n / 60n, 2n ** 64n / 3600n].flatMap((units) => [units, units + 1n].map((n) => wide(n << 64n))),
	...[(7n << 64n) + 999_999_999n, 2n ** 64n * 1_000_000_000n - 1n, 2n ** 64n * 1_000_000_000n, 2n ** 128n - 1n].map((n) => wide(n)),
];

/** `durations`' larger units, of a `u128` whose high half is also its `u64`. */
function wide(n: bigint): Uint8Array {
	const bytes = new Uint8Array(19);
	bytes.set([...durationsOp, 12]);
	const view = new DataView(bytes.buffer);
	view.setBigUint64(3, n >> 64n);
	view.setBigUint64(11, n & 0xffff_ffff_ffff_ffffn);
	return bytes;
}

export const cases: Uint8Array[] = [...durations];
