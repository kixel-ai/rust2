// The inputs `dyns`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `dyns::dyns` too short to run, with a `Tri` of each variant, at the ends of each side and
 * count, and boxing each kind of shape.
 */
const dyns = [[], [1, 2], [3, 0, 7], [255, 1, 255, 0, 1, 2, 3, 4, 5], [0, 3, 2, 9, 255, 6, 128]].map((bytes) =>
	Uint8Array.of(...op("dyns::dyns"), 0, ...bytes),
);

/**
 * `dyns::shared` too short to run, and with `Arc`s of each type, the last count of each at
 * zero and at the top of its range.
 */
const shared = [[], [1], [0, 0], [255, 255, 0, 1, 2, 3, 4, 5], [7, 0xff, 255, 254, 253]].map((bytes) =>
	Uint8Array.of(...op("dyns::shared"), 0, ...bytes),
);

/**
 * `ids::ids` too short to run, and boxing each kind of value, the first two bytes picking
 * ids at both ends of the list.
 */
const ids = [[], [1], [0, 12, 0, 1, 2], [12, 0, 255, 254, 253, 3]].map((bytes) =>
	Uint8Array.of(...op("ids::ids"), 0, ...bytes),
);

export const cases: Uint8Array[] = [...dyns, ...shared, ...ids];
