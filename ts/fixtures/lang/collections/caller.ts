// The inputs `collections`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `collections::deque_ranges` of each form of range, over no items and over items whose
 * front wraps past the buffer's end, at bounds empty, at either end, past it, and backwards.
 */
const dequeRangesOp = op("collections::deque_ranges");
const dequeRanges = [0, 0x18, 0xa8].flatMap((high) =>
	Array.from({ length: 8 }, (_, form) =>
		[[0, 0], [0, 3], [2, 2], [1, 4], [5, 5], [3, 1], [6, 7]].flatMap(([a, b]) => [[], [7, 8, 9]].map((items) => Uint8Array.of(...dequeRangesOp, high | form, a, b, ...items))),
	).flat(),
);

/**
 * `collections::set_ops` of one key against 30, which std's B-tree looks up in the larger
 * rather than merging, of each key type: a key below the rest, at either end, between, and
 * above. Then `Nibble`s equal to another set's but for their high bits, of hash sets as
 * large as each other and one larger, where std's picks which set's to yield by size.
 */
const setOpsOp = op("collections::set_ops");
const setOps = [
	...[15, 16, 17, 18, 19].flatMap((kind) => [0x20, 0x21, 0x25, 0x3e, 0x3f].map((single) => Uint8Array.of(...setOpsOp, kind, single, ...Array.from({ length: 30 }, (_, i) => 0x41 + i)))),
	...[
		[0x01, 0x02, 0x13, 0x21, 0x32, 0x44],
		[0x01, 0x02, 0x13, 0x21, 0x32, 0x44, 0x55],
		[0x01, 0x02, 0x13, 0x55, 0x21, 0x32, 0x44],
	].map((data) => Uint8Array.of(...setOpsOp, 3, ...data)),
];

/**
 * `collections::unlike_eqs` of every pair of positions where `eq` and `ne` disagree and each
 * length the right side may lose, over no items, one, two, a wrapped deque's four, and nine,
 * whose positions are all within it.
 */
const unlikeEqsOp = op("collections::unlike_eqs");
const unlikeEqs = [[], [5], [1, 2], [0, 7, 2, 9], [3, 3, 3, 3, 3, 3, 3, 3, 3]].flatMap((bytes) =>
	Array.from({ length: 128 }, (_, kind) => Uint8Array.of(...unlikeEqsOp, kind, ...bytes)),
);

/** `collections::own_nes` of the same positions, lengths, and items as `unlike_eqs`. */
const ownNesOp = op("collections::own_nes");
const ownNes = unlikeEqs.map((input) => Uint8Array.of(...ownNesOp, ...input.subarray(2)));

/** `collections::unlike_compares` of the same positions, lengths, and items as `unlike_eqs`. */
const unlikeComparesOp = op("collections::unlike_compares");
const unlikeCompares = unlikeEqs.map((input) => Uint8Array.of(...unlikeComparesOp, ...input.subarray(2)));

/**
 * `collections::ref_orders` of no items, one, repeats, and runs whose remainders by 2, 3,
 * 4, and 5 agree and disagree.
 */
const refOrdersOp = op("collections::ref_orders");
const refOrders = [[], [5], [3, 3, 3], [1, 2], [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], [7, 2, 9, 4, 4, 11, 15, 6], [12, 6, 0, 30, 60]].map((data) => Uint8Array.of(...refOrdersOp, 0, ...data));

export const cases: Uint8Array[] = [...dequeRanges, ...setOps, ...unlikeEqs, ...ownNes, ...unlikeCompares, ...refOrders];
