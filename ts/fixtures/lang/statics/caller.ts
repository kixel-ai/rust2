// The inputs `statics`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `statics::onces` scripts: a `Once` forced, called, and forced again, which drops the
 * function it never runs, each static's, each `call_once` and `call_once_force` run again
 * from within itself, and crossbeam-epoch's `OnceLock` of each kind.
 */
const onces = [[1, 0, 1, 7], [0, 6, 1, 2, 3, 4, 5, 11], [2, 14, 8], [9], [3, 15], [4, 17, 23, 29, 4]].map((bytes) => Uint8Array.of(...op("statics::onces"), 0, ...bytes));

/** `statics::lazies` scripts: each lazy cell and lock printed before it is forced, after, and from within its own function. */
const lazies = [[0, 1, 2, 3, 4, 5, 4, 5, 0]].map((bytes) => Uint8Array.of(...op("statics::lazies"), 0, ...bytes));

/** `statics::writes` scripts: each way of writing a lazy cell and lock, with neither, either, or both forced first. */
const writes = [Array.from({ length: 56 }, (_, i) => i)].map((bytes) => Uint8Array.of(...op("statics::writes"), 0, ...bytes));

export const cases: Uint8Array[] = [...onces, ...lazies, ...writes];
