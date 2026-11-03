// The inputs `strs`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `strs::edits` of each edit on `aé\u{800}😀`, a char of each UTF-8 width, at every byte
 * offset into it and up to two past its end, each made a raw offset by its byte's bits.
 */
const editsOp = op("strs::edits");
const edits = Array.from({ length: 14 * 16 * 16 }, (_, i) => Uint8Array.of(...editsOp, 4, 2, 7, 9, 14, i >> 8, (i >> 4) & 15, i & 15));

/**
 * `strs::drains` of each range and way to end the drain on `aé\u{800}😀`, at every byte
 * offset into it and up to two past its end, reading none, two from either end, or three
 * from the back.
 */
const drainsOp = op("strs::drains");
const drains = Array.from({ length: 32 * 13 * 13 }, (_, i) => {
	const [form, a, b] = [i % 32, Math.floor(i / 32) % 13, Math.floor(i / 416)];
	return Uint8Array.of(...drainsOp, 4, 2, 7, 9, 14, form, a, b, [0, 0x0a, 0x1f][(form + a + b) % 3]);
});

/** `strs::iters` of each kind, on no text, text of each UTF-8 width, and parts between `/`s, empty ones too. */
const iters = Array.from({ length: 12 }, (_, kind) => [[], [2, 7, 9, 14], [0, 2, 0, 0, 7, 9], [14, 0]].map((data) => Uint8Array.of(...op("strs::iters"), kind, ...data))).flat();

/**
 * `strs::strs`' splits and `rsplit_once` by `/`, and `strs::slices`' by an empty and a
 * one-char pattern, on no text, a separator alone, twice, and at either end, read from
 * either end.
 */
const separatorEnds = [[], [0], [0, 0], [0, 2], [2, 0], [0, 2, 0], [0x80, 2, 0], [2, 0x80, 0x80], [0, 0x82, 0x80, 7]];
const splitEnds = separatorEnds.flatMap((data) => [
	...[1, 3].map((kind) => Uint8Array.of(...op("strs::strs"), kind, ...data)),
	...[2, 2 | 8].map((kind) => Uint8Array.of(...op("strs::slices"), kind, ...data)),
]);

/** `strs::hinted` of each adaptor and each pair of range lengths, each way, on `aé` alone. */
const hintedOp = op("strs::hinted");
const hinted = Array.from({ length: 10 * 7 * 21 }, (_, i) => Uint8Array.of(...hintedOp, i % 10, Math.floor(i / 10) % 7, Math.floor(i / 70)));

/**
 * `strs::gets` of each kind at every pair of byte offsets up to two past the end of its 13
 * bytes of text, and of no text, and at `usize::MAX`.
 */
const getsOp = op("strs::gets");
const getsAt = (n: number) => [...Array.from({ length: n + 3 }, (_, i) => i), 0xff];
const gets = [0, 0x10].flatMap((text) => {
	const at = getsAt(text === 0 ? 13 : 0);
	return Array.from({ length: 16 * at.length * at.length }, (_, i) =>
		Uint8Array.of(...getsOp, text | (i % 16), at[Math.floor(i / 16) % at.length], at[Math.floor(i / 16 / at.length)]),
	);
});

export const cases: Uint8Array[] = [...edits, ...drains, ...iters, ...splitEnds, ...hinted, ...gets];
