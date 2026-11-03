// The inputs `separated`'s cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `separated::separated` of every form and order of reads, over data whose bytes its bounds
 * make separators of alone, in runs, and at either end, and over one element and none.
 */
const separatedOp = op("separated::separated");
const separated = [[], [0], [0x90], [0, 0, 0], [5, 0x90, 0, 0x7f, 0, 0xff, 3], [0x80, 1, 2, 0], [0xff, 0x7f, 0x80, 0x81]].flatMap((data) =>
	Array.from({ length: 256 }, (_, kind) => Uint8Array.of(...separatedOp, kind, ...data)),
);

export const cases: Uint8Array[] = [...separated];
