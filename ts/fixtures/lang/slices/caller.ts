// The inputs `slices`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `subslices` of each kind over zero to four items, at every pair of bounds up to one past
 * the end, so parts are empty, at the end, backwards, and out of bounds.
 */
const subslicesOp = op("slices::subslices");
const subslices = [0, 1, 2, 3, 4].flatMap((n) =>
	Array.from({ length: 8 * (n + 2) ** 2 }, (_, i) => {
		const [kind, a, b] = [i % 8, Math.floor(i / 8) % (n + 2), Math.floor(i / 8 / (n + 2))];
		return Uint8Array.of(...subslicesOp, kind, a, b, ...[0x31, 0xc2, 0x07, 0xfe].slice(0, n));
	}),
);

/**
 * `ranges` of each kind, and each kind of `Bound` at the end, over zero to four items, at
 * every pair of bounds up to one past the end and at `usize::MAX`.
 */
const rangesOp = op("slices::ranges");
const ranges = [0, 1, 2, 3, 4].flatMap((n) => {
	const ends = [...Array(n + 2).keys(), 0xff];
	return Array.from({ length: 24 * ends.length ** 2 }, (_, i) => {
		const [kind, a, b] = [i % 24, ends[Math.floor(i / 24) % ends.length], ends[Math.floor(i / 24 / ends.length)]];
		return Uint8Array.of(...rangesOp, kind, a, b, ...[0x31, 0xc2, 0x07, 0xfe].slice(0, n));
	});
});

/**
 * `edits` of each kind over zero to four items, at every three bounds up to one past the
 * end, so parts are empty, at the end, backwards, and out of bounds.
 */
const editsSlicesOp = op("slices::edits");
const sliceEdits = [0, 1, 2, 3, 4].flatMap((n) =>
	Array.from({ length: 8 * (n + 2) ** 3 }, (_, i) => {
		const [kind, a, b, c] = [i % 8, Math.floor(i / 8) % (n + 2), Math.floor(i / 8 / (n + 2)) % (n + 2), Math.floor(i / 8 / (n + 2) ** 2)];
		return Uint8Array.of(...editsSlicesOp, kind, a, b, c, ...[0x31, 0xc2, 0x07, 0xfe].slice(0, n));
	}),
);

export const cases: Uint8Array[] = [...subslices, ...ranges, ...sliceEdits];
