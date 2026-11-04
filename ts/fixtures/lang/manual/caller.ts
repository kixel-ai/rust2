// The inputs `uninit`'s cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** `uninit::uninits` of inputs whose first byte copies a slice of mismatched length into one of bytes and one of `u16`s. */
export const cases: Uint8Array[] = [
	[0xfe, 3, 1, 2],
	[0xff, 3, 1, 2],
].map((data) => Uint8Array.of(...op("uninit::uninits"), ...data));
