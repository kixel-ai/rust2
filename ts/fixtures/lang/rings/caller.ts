// The inputs `rings`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `rings::slices` of each item type, from a deque made by `new`, `with_capacity` of 5, and
 * from a `Vec` of 7: pushed at both ends and popped at the front to wrap it, past a few
 * capacities, then each step by a few arguments, twice.
 */
const ringsCases = Array.from({ length: 5 * 3 * 4 * 24 * 3 }, (_, i) => {
	const [kind, first, wraps, step, arg] = [i % 5, [0, 0x15, 0x1e][Math.floor(i / 5) % 3], [1, 3, 6, 10][Math.floor(i / 15) % 4], Math.floor(i / 60) % 24, [0, 0x5b, 0xe7][Math.floor(i / 1440)]];
	const wrap = Array.from({ length: wraps }, (_, j) => [j % 3 === 2 ? 4 : 0, j * 3, 6, 0, 4, j]).flat();
	return Uint8Array.of(...op("rings::slices"), kind, first, ...wrap, step, arg, step, arg ^ 0x25);
});

/**
 * `rings::slices` remaking each wrapped deque each way and with a few skips: a clone,
 * `clone_from`, `clone_into`, its `into_iter` read in part and by `nth`, from a `Vec` as it
 * is and filtered in place, and a `Vec`'s `into_iter` read in part, collected as it is and
 * cloned; then pushed at both ends.
 */
const remadeCases = [0, 5, 0x2d, 0x37, 0x64, 0x6e, 1, 0x10, 2, 0x11, 3, 0x21, 0x0e, 0x13, 0x45, 0x5f].flatMap((arg) =>
	Array.from({ length: 5 * 3 * 4 }, (_, i) => {
		const [kind, first, wraps] = [i % 5, [0, 0x15, 0x1e][Math.floor(i / 5) % 3], [1, 3, 6, 10][Math.floor(i / 15)]];
		const wrap = Array.from({ length: wraps }, (_, j) => [j % 3 === 2 ? 4 : 0, j * 3, 6, 0, 4, j]).flat();
		return Uint8Array.of(...op("rings::slices"), kind, first, ...wrap, 22, arg, 0, 9, 4, 9);
	}),
);

export const cases: Uint8Array[] = [...ringsCases, ...remadeCases];
