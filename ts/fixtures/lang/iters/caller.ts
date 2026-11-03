// The inputs `iters`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `iters::reducers` of no items, one, one repeated, which `next_if_eq` takes, and keys
 * that all tie.
 */
const reducers = [[], [9], [9, 9], [8, 4, 12, 0x40, 0x80]].map((bytes) => Uint8Array.of(...op("iters::reducers"), 0, ...bytes));

/**
 * `iters::peeked_others` of no items, one, a run that matches the first throughout, and
 * items that each match one comparison and miss the next.
 */
const peekedOthers = [[], [3], [4, 4, 4, 4], [0, 1, 2, 3, 4, 5, 6], [1, 0, 0, 2, 5, 9]].map((bytes) =>
	Uint8Array.of(...op("iters::peeked_others"), 0, ...bytes),
);

/**
 * `iters::range_froms` of each case, each way its zips read the range, from each distance
 * below its type's last value and from 250, on up to nine bytes: each length its zips
 * panic at.
 */
const rangeFromsOp = op("iters::range_froms");
const rangeFroms = Array.from({ length: 16 * 4 * 4 * 10 }, (_, i) => {
	const kind = (i % 16) | ((Math.floor(i / 16) % 4) << 6);
	const a = [0, 1, 3, 250][Math.floor(i / 64) % 4];
	const data = Array.from({ length: Math.floor(i / 256) }, (_, j) => (j === 0 ? a : (j * 37) & 0xff));
	return Uint8Array.of(...rangeFromsOp, kind, ...data);
});

/** `iters::range_methods` of each type's table, at each pair of its first eight values. */
const rangeMethodsOp = op("iters::range_methods");
const rangeMethods = Array.from({ length: 19 * 64 }, (_, i) => Uint8Array.of(...rangeMethodsOp, i % 19, Math.floor(i / 19) % 8, Math.floor(i / 152)));

/**
 * `iters::compares` of each comparison, shape, and pair, over no items, one, a run up to
 * 255, which `start..` panics past and a sum overflows at, NaNs, and repeats.
 */
const comparesOp = op("iters::compares");
const compares = [[], [5], [1, 2, 3], [250, 251, 252, 253, 254, 255], [0xff, 3, 0xff], [0, 1, 2, 3, 4, 5], [7, 7, 7, 7, 7, 7, 7, 7, 7]].flatMap((data) =>
	Array.from({ length: 256 }, (_, kind) => Uint8Array.of(...comparesOp, kind, ...data)),
);

export const cases: Uint8Array[] = [...reducers, ...peekedOthers, ...rangeFroms, ...rangeMethods, ...compares];
