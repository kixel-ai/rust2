// The inputs `zeros`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** `zeros::zeros` of each case, on up to three bytes. */
const zerosOp = op("zeros::zeros");
const zeros = Array.from({ length: 7 * 4 }, (_, i) => Uint8Array.of(...zerosOp, i % 7, ...Array.from({ length: Math.floor(i / 7) }, (_, b) => b)));

/** `zeros::writes` of each case, on up to three bytes: the last past the slice it swaps in. */
const writesOp = op("zeros::writes");
const writes = Array.from({ length: 5 * 4 }, (_, i) => Uint8Array.of(...writesOp, i % 5, ...Array.from({ length: Math.floor(i / 5) }, (_, b) => b)));

export const cases: Uint8Array[] = [...zeros, ...writes];
