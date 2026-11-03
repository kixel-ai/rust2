// The inputs `sorts`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `sorts` cases of each length std's thresholds part, of each run of keys; `disorders`
 * cases of each length, kind of item, and sort; and `btrees` cases of each length.
 */
const sortCases = [
	...Array.from({ length: 20 * 5 }, (_, i) => Uint8Array.of(...op("sorts::sorts"), i % 20, Math.floor(i / 20), 7, 1, 3)),
	...Array.from({ length: 160 }, (_, kind) => Uint8Array.of(...op("sorts::disorders"), kind, kind % 5, 0x5a, 0xc3, 0x0f, 7)),
	...Array.from({ length: 20 * 5 }, (_, i) => Uint8Array.of(...op("sorts::btrees"), i % 20, Math.floor(i / 20), 2, 9)),
];

export const cases: Uint8Array[] = [...sortCases];
