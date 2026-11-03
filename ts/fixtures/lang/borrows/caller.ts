// The inputs `borrows`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `borrows::guards` scripts: each operation on cells empty and not, cloned while borrowed
 * shared and exclusive, and borrowed exclusively while a mapped or cloned guard holds it.
 */
const guards = [
	[0x04, 0x05, 0x06, 0x03, 0x07, 0x02, 0x01],
	[0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0b, 0x0c, 0x0d],
	[0x08, 0xe1, 0xf1],
	[0x08, 0xe6],
	[0xf6],
].map((bytes) => Uint8Array.of(...op("borrows::guards"), 0, ...bytes));

/**
 * `borrows::maps` scripts: each operation through mapped, filtered, and split guards, with
 * parts written whole and split parts dropped in either order, and each borrow again
 * while one holds the cell, which panics.
 */
const maps = [
	[0x00, 0x01, 0x02, 0x03, 0x03, 0x04, 0x05, 0x15, 0x07, 0xf1, 0xf2, 0x0b, 0x06, 0x13, 0x0c],
	[0x02, 0x22, 0x0d, 0xc6],
	[0x0a, 0xd6],
	[0xe6],
	[0x02, 0xf6],
].map((bytes) => Uint8Array.of(...op("borrows::maps"), 0, ...bytes));

/**
 * `borrows::tries` scripts: each operation through tried guards, given back beside each
 * error, printed, and each borrow again while a tried guard holds the cell, which panics.
 */
const tries = [
	[0x00, 0x00, 0x10, 0xf0, 0xe0, 0x01, 0x02, 0x05, 0x20, 0x21, 0x06, 0x03],
	[0x01, 0x10, 0x01, 0x12, 0xe2],
	[0xd3],
	[0xe3],
	[0x00, 0xf3],
].map((bytes) => Uint8Array.of(...op("borrows::tries"), 0, ...bytes));

/**
 * `borrows::leaks` scripts: guards leaked and undone, whole byte arrays mapped, and each
 * borrow again while a leak holds the cell, which panics.
 */
const leaks = [[0x00, 0x01, 0x02, 0xf1, 0xf2, 0x04, 0x05, 0x03, 0x06], [0xe3], [0x00, 0xf3]].map((bytes) =>
	Uint8Array.of(...op("borrows::leaks"), 0, ...bytes),
);

/**
 * `borrows::printed` scripts: each cell printed, free, shared, and borrowed exclusively,
 * nested, through guards, and borrowed while it writes what it holds, more than once so
 * that what each holds grows.
 */
const printed = [[0x00, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x18, 0xf1, 0x12, 0xfb, 0x24, 0x2d], []].map((bytes) =>
	Uint8Array.of(...op("borrows::printed"), 0, ...bytes),
);

export const cases: Uint8Array[] = [...guards, ...maps, ...tries, ...leaks, ...printed];
