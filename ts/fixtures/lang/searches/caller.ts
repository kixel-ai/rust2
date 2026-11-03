// The inputs `searches`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `searches::searches` of each search and pattern, by chars, needles, and counts its first
 * byte picks: an empty needle, one, two, and three chars, and counts from 0 to 4. Of no
 * text, a separator alone, twice, and at either end, runs a two-char needle overlaps in,
 * chars of each UTF-8 width, and digits the closure matches, read from either end.
 */
const searchesOp = op("searches::searches");
const searchesCases = [0x00, 0x12, 0x22, 0x31, 0x4e, 0x57].flatMap((pick) =>
	[[], [0], [0, 0], [0, 2], [2, 0], [0, 2, 0], [2, 2, 2], [2, 2, 2, 2], [14, 7, 14], [0x80, 2, 0x80], [0x82, 0x8e, 0x07, 0x83]].flatMap((text) =>
		Array.from({ length: 85 }, (_, kind) => Uint8Array.of(...searchesOp, kind, pick, ...text)),
	),
);

/**
 * `searches::texts` of each iterator and the trims, of no text, a line end of each kind
 * alone, between lines and doubled, a `\r` without a `\n`, and whitespace of each UTF-8
 * width between the chars JS's `\s` alone counts, read from either end.
 */
const textsOp = op("searches::texts");
const textsCases = [[], [0], [1, 0], [2, 1, 0, 2], [2, 0, 0, 2], [0, 2], [2, 1], [1, 1, 0], [3, 2, 3], [6, 7, 8, 9, 2, 10, 2, 11, 12, 13, 2, 14, 15, 4, 5], [0x83, 2, 0x8d, 0x0a, 0x84]].flatMap(
	(text) => Array.from({ length: 4 }, (_, kind) => Uint8Array.of(...textsOp, kind, ...text)),
);

export const cases: Uint8Array[] = [...searchesCases, ...textsCases];
