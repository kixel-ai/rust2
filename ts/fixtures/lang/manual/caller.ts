// The inputs `uninit`'s cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** `uninit::uninits` of inputs whose first byte copies a slice of mismatched length into one of bytes and one of `u16`s. */
const copies: Uint8Array[] = [
	[0xfe, 3, 1, 2],
	[0xff, 3, 1, 2],
].map((data) => Uint8Array.of(...op("uninit::uninits"), ...data));

/** `uninit::boxed_uninits` of inputs whose first byte clones a slice of mismatched length into one of bytes and one of values that count their clones. */
const clones: Uint8Array[] = [
	[0xfc, 3, 1, 2],
	[0xfd, 3, 1, 2],
].map((data) => Uint8Array.of(...op("uninit::boxed_uninits"), ...data));

export const cases: Uint8Array[] = [...copies, ...clones];
