// The inputs `chars`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** `chars::unicode` of every block of 4096 scalars, by method and as function values, so every char is tried. */
const blocks = [0, 1].flatMap((kind) => Array.from({ length: 0x110 }, (_, block) => Uint8Array.of(...op("chars::unicode"), kind, block >> 8, block & 0xff)));

/** `chars::encodes` into a buffer of each length, of the chars at each UTF-8 and UTF-16 width, which `chars.rs` always tries. */
const encodes = [0, 1, 2, 3, 4].map((len) => Uint8Array.of(...op("chars::encodes"), 2, len));

/**
 * `chars::decodes` of units that pair, a leading surrogate before another, before a
 * non-surrogate, and at the end, and a trailing surrogate alone.
 */
const decodes = [
	[0xd83d, 0xde00, 0x61],
	[0xd800, 0xd800, 0xdc00],
	[0xd800, 0x41, 0xdfff],
	[0xdc00, 0xd7ff, 0xe000, 0xdbff],
	[0xdbff, 0xdfff, 0xd800],
].flatMap((units) => [0, 1].map((kind) => Uint8Array.of(...op("chars::decodes"), kind, ...units.flatMap((u) => [u >> 8, u & 0xff]))));

/** `chars::converts` at each radix's edge, and past it, where `from_digit` and `is_digit` panic. */
const converts = [0, 1, 2, 10, 16, 35, 36, 37].flatMap((radix) => [0, 1].map((kind) => Uint8Array.of(...op("chars::converts"), kind, radix)));

export const cases: Uint8Array[] = [...blocks, ...encodes, ...decodes, ...converts];
