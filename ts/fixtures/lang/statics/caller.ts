// The inputs `statics`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `statics::onces` scripts: a `Once` forced, called, and forced again, which drops the
 * function it never runs, each static's, each `call_once` and `call_once_force` run again
 * from within itself, and crossbeam-epoch's `OnceLock` of each kind.
 */
const onces = [[1, 0, 1, 7], [0, 6, 1, 2, 3, 4, 5, 11], [2, 14, 8], [9], [3, 15], [4, 17, 23, 29, 4]].map((bytes) => Uint8Array.of(...op("statics::onces"), 0, ...bytes));

export const cases: Uint8Array[] = [...onces];
