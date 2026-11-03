// The inputs `bounds`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `bounds` with each kind of bound at each end, over every pair of values around and past
 * four items, so ranges are empty, backwards, equal, and out of bounds, and of no items.
 */
const boundsOp = op("bounds::bounds");
const bounds = Array.from({ length: 6 * 9 * 36 }, (_, n) => {
	const [kind, ends, at] = [n % 6, Math.floor(n / 6) % 9, Math.floor(n / 54)];
	const [a, b] = [at % 6, Math.floor(at / 6)];
	return [Uint8Array.of(...boundsOp, kind, ends % 3, Math.floor(ends / 3), a, b, 1, 3, 2, 5), Uint8Array.of(...boundsOp, kind, ends % 3, Math.floor(ends / 3), a, b)];
}).flat();

export const cases: Uint8Array[] = [...bounds];
