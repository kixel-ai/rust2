// The inputs `locks`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `locks` scripts: writes through guards and reads, `get_mut`, and a lock matched.
 */
const locks = [[0, 1, 5, 6, 2, 7, 3, 8, 3, 0, 1], [4, 0, 4, 1], [2, 9, 2, 10, 4, 1], [3, 3, 4, 0]].map(
	(bytes) => Uint8Array.of(...op("locks::locks"), 0, ...bytes),
);

/** `locks::mapped` scripts: writes and reads through mapped and filtered guards. */
const mappedLocks = [[0x04, 0x01, 0x02, 0x86, 0x05, 0x0a, 0x03, 0x02, 0x80, 0x01, 0x03]].map((bytes) =>
	Uint8Array.of(...op("locks::mapped"), 0, ...bytes),
);

/**
 * `locks::relock` with each pick: a mutex locked again while a guard holds it, which
 * panics, and a read-write lock read or written again while a guard, mapped or not, holds
 * it, which aborts but for a read under a read.
 */
const relocks = Array.from({ length: 16 }, (_, pick) => Uint8Array.of(...op("locks::relock"), 0, pick));

/** `locks::mutex_mapped` scripts: writes and reads through mapped and filtered mutex guards. */
const mutexMaps = [[0x04, 0x01, 0x02, 0x86, 0xf5, 0x0a, 0x03, 0x02, 0xe1, 0x80, 0x07, 0x02]].map((bytes) =>
	Uint8Array.of(...op("locks::mutex_mapped"), 0, ...bytes),
);

/** `locks::printed` scripts: each lock's guards printed, empty and not. */
const printed = [[], [0, 1, 25, 200]].map((bytes) => Uint8Array.of(...op("locks::printed"), 0, ...bytes));

/** `locks::downgraded` scripts: write guards downgraded, each lock's released in either order, and whole byte arrays mapped. */
const downgrades = [[0x00, 0x01, 0x11, 0x02, 0xf2, 0xe2, 0x03, 0x04, 0x13, 0x05]].map((bytes) =>
	Uint8Array.of(...op("locks::downgraded"), 0, ...bytes),
);

/** `locks::rwlocks` scripts: `get_mut` and `into_inner` among writes and reads. */
const rwlocks = [[6, 7, 0, 6, 2, 14, 15, 1]].map((bytes) => Uint8Array.of(...op("locks::rwlocks"), 0, ...bytes));

/** `locks::shown` scripts: each lock printed, free, read, held, and written, nested, and locked while it writes what it holds. */
const shown = [[0, 6, 11, 2, 3, 8, 4, 125, 7]].map((bytes) => Uint8Array.of(...op("locks::shown"), 0, ...bytes));

/** `locks::guarded` scripts: writes through a crate's own guards, held and temporary, and one that puts what it lends as it drops. */
const guarded = [[1, 0, 1, 3, 2, 8, 9, 4, 10, 16, 22, 5, 11, 255, 5]].map((bytes) =>
	Uint8Array.of(...op("locks::guarded"), 0, ...bytes),
);

export const cases: Uint8Array[] = [
	...locks,
	...mappedLocks,
	...relocks,
	...mutexMaps,
	...printed,
	...downgrades,
	...rwlocks,
	...shown,
	...guarded,
];
