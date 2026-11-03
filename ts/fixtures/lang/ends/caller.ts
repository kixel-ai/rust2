// The inputs `ends`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `ends::ends`: each kind of iterator, and boxes and borrows of the crate's own, through
 * every consumer, fold, and find and either end's bits, of runs empty, short, and long,
 * whose hints are exact, loose, and lying, after none and a few steps through each of its
 * methods.
 */
const endsCases = [0x00, 0x03, 0x0f, 0x15, 0x27, 0x36, 0x2f].flatMap((shape) =>
	[[], [0x01, 0x00], [0x0a, 0x0d, 0x03], [0x05, 0x06, 0x04, 0x0f]].flatMap((steps) =>
		Array.from({ length: 18 * 256 }, (_, i) => Uint8Array.of(...op("ends::ends"), i % 18, shape, Math.floor(i / 18), ...steps)),
	),
);

export const cases: Uint8Array[] = [...endsCases];
