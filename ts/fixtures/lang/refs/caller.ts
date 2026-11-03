// The inputs `refs`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `refs::compared` of a split and the bytes either side of it, whose remainders by 3 and 5
 * make the bytes and floats: sides that are equal, empty, or a prefix of the other, and
 * floats with a NaN before, after, or where they differ, and a `-0.0` against `0.0`.
 */
const placeCompares = [
	[2, 1, 2, 1, 2],
	[0],
	[0, 1],
	[1, 1, 1, 2],
	[2, 1, 2, 1],
	[2, 0, 3, 0, 3],
	[2, 0, 3, 1, 3],
	[1, 3, 3, 1],
	[2, 3, 0, 3, 1],
	[1, 4, 0],
].map(([split, ...data]) => Uint8Array.of(...op("refs::compared"), split, ...data));

export const cases: Uint8Array[] = [...placeCompares];
