// The inputs `hints`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `hints::hints` of each iterator, each pair of lengths, and each of its arms, read three
 * times each way, with as many bytes as the first length's index.
 */
const hintsOp = op("hints::hints");
const hints = Array.from({ length: 24 * 8 * 8 * 8 * 2 }, (_, i) => {
	const [kind, n, m, arm, back] = [i % 24, Math.floor(i / 24) % 8, Math.floor(i / 192) % 8, Math.floor(i / 1536) % 8, Math.floor(i / 12288)];
	const steps = (arm << 5) | ((back ? 0b101 : 0b010) << 2) | 3;
	return Uint8Array.of(...hintsOp, kind, n, m, steps, ...Array.from({ length: n }, (_, b) => b * 37));
});

/** `hints::reserves` of each collection and element type, each shape, length, and limit on what it reads. */
const reservesOp = op("hints::reserves");
const reserves = Array.from({ length: 13 * 6 * 5 * 4 * 5 }, (_, i) =>
	Uint8Array.of(...reservesOp, i % 78, Math.floor(i / 78) % 5, Math.floor(i / 390) % 4, Math.floor(i / 1560)),
);

export const cases: Uint8Array[] = [...hints, ...reserves];
