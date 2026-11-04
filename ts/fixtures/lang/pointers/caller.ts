// The inputs `pointers`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `pointers::cells` scripts: each operation on every cell, a number wrapping past its top, a
 * struct assigned through `get` and written over, and cells replaced or made and taken out.
 */
const unsafeCells = [
	[0, 248, 1, 9, 2, 10, 3, 11, 4, 12, 5, 13, 6, 14, 7, 15],
	[4, 12, 4, 5, 253, 7, 255, 0],
	[3, 251, 6, 3, 2, 2, 254],
	[],
].map((bytes) => Uint8Array.of(...op("pointers::cells"), 0, ...bytes));

export const cases: Uint8Array[] = [...unsafeCells];
