// The inputs `ascii`'s cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `ascii` cases of each kind: bytes and slices on each prefix of text padded by ASCII
 * whitespace, vertical tab among it, holding a byte of each escape and class, so escapes
 * are read from both ends; slices on text of each char `ascii.rs` tries; chars on the
 * scalars either side of the surrogates and past the last.
 */
const asciiText = [0x20, 0x09, 0x0b, 0x48, 0x7a, 0x22, 0x5c, 0x27, 0x7f, 0x80, 0xff, 0x0a, 0x0c, 0x0d, 0x20];
const asciiCases = [
	...["ascii::bytes", "ascii::slices"].flatMap((name) =>
		Array.from({ length: 4 * (asciiText.length + 1) }, (_, i) => Uint8Array.of(...op(name), i % 4, ...asciiText.slice(0, Math.floor(i / 4)))),
	),
	// Text of every char `ascii.rs` tries, each once.
	...[0, 1, 2, 3].map((kind) => Uint8Array.of(...op("ascii::slices"), kind, ...Array.from({ length: 41 }, (_, i) => i))),
	Uint8Array.of(...op("ascii::chars"), 0, ...[0xd7ff, 0xd800, 0xdfff, 0xe000, 0x1f600, 0x10ffff, 0x110000].flatMap((n) => [n >>> 24, (n >>> 16) & 0xff, (n >>> 8) & 0xff, n & 0xff])),
	// `parts` of each kind at every pair of byte offsets up to two past the end of its
	// 14 bytes of text, and of no text.
	...Array.from({ length: 8 * 17 * 17 }, (_, i) => Uint8Array.of(...op("ascii::parts"), i % 8, Math.floor(i / 8) % 17, Math.floor(i / 8 / 17))),
	...Array.from({ length: 8 * 3 * 3 }, (_, i) => Uint8Array.of(...op("ascii::parts"), 0x10 | (i % 8), Math.floor(i / 8) % 3, Math.floor(i / 24))),
];

export const cases: Uint8Array[] = [...asciiCases];
