// The inputs `lengths`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `lengths::lengths` at each length, with `0xff` among the rows `copied` copies, which
 * clones none, and as the value each `vec![x; n]` clones, which panics, in a body generic
 * over a type and in one over none.
 */
const lengthsOp = op("lengths::lengths");
export const cases: Uint8Array[] = [0, 1, 2, 3].flatMap((kind) =>
	[[0xff, 0, 0, 0xff], [0, 0xff, 0], [0, 0, 0xff]].map((data) => Uint8Array.of(...lengthsOp, kind, ...data)),
);
