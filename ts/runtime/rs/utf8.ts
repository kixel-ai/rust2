// UTF-8 to and from a JS string, for the runtime's own use. A short string or byte run
// takes a loop here, which skips the native call and the buffer of its own that make
// `TextEncoder` cost ten times the work in V8 and `TextDecoder` more than the work; a long
// one takes the native codec. A string from translated code is well formed, so it never
// holds a lone surrogate.

import { bytes } from "../rs.ts";

/**
 * The longest string, in code units, that a loop encodes: past it the loop's lead in
 * Chrome narrows, and Firefox's and JSC's `TextEncoder` win.
 */
const ENCODE_SHORT = 64;

/**
 * The longest byte run a loop decodes, past which `TextDecoder` wins in Chrome. Firefox's
 * wins at any length, but Chrome decides (`ts/README.md`).
 */
const DECODE_SHORT = 32;

const encoder = /* @__PURE__ */ new TextEncoder();
// `ignoreBOM` keeps a leading U+FEFF, as Rust does.
const decoder = /* @__PURE__ */ new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });

/** The code units a short decode builds before it makes them a string: never more than its bytes. */
const units = /* @__PURE__ */ new Uint16Array(DECODE_SHORT);

/** The UTF-8 bytes of `s`, in the runtime's shared buffer where they are short. */
export function utf8Encode(s: string): Uint8Array {
	const n = s.length;
	if (n > ENCODE_SHORT) return encoder.encode(s);
	// Measured first, so a string takes one allocation: a second costs Firefox more than the scan.
	let len = n;
	for (let i = 0; i < n; i++) {
		const u = s.charCodeAt(i);
		// Each half of a surrogate pair is two of its four bytes.
		if (u >= 0x80) len += u < 0x800 || (u >= 0xd800 && u <= 0xdfff) ? 1 : 2;
	}
	const out = bytes(len);
	if (len === n) {
		for (let i = 0; i < n; i++) out[i] = s.charCodeAt(i);
	} else {
		utf8Write(s, 0, out, 0);
	}
	return out;
}

/** Write the UTF-8 of `s` from code unit `from` to `out` at `at`, which has room for it: where it ends. */
function utf8Write(s: string, from: number, out: Uint8Array, at: number): number {
	// Inline, as a call a char costs Firefox twice the loop.
	for (let i = from; i < s.length; i++) {
		let c = s.charCodeAt(i);
		if (c < 0x80) {
			out[at++] = c;
		} else if (c < 0x800) {
			out[at++] = 0xc0 | (c >> 6);
			out[at++] = 0x80 | (c & 0x3f);
		} else if (c < 0xd800 || c > 0xdbff) {
			out[at++] = 0xe0 | (c >> 12);
			out[at++] = 0x80 | ((c >> 6) & 0x3f);
			out[at++] = 0x80 | (c & 0x3f);
		} else {
			c = 0x10000 + ((c - 0xd800) << 10) + (s.charCodeAt(++i) - 0xdc00);
			out[at++] = 0xf0 | (c >> 18);
			out[at++] = 0x80 | ((c >> 12) & 0x3f);
			out[at++] = 0x80 | ((c >> 6) & 0x3f);
			out[at++] = 0x80 | (c & 0x3f);
		}
	}
	return at;
}

/** Write the UTF-8 of code point `c` to `out` at `at`, which has room for it: where it ends. */
export function charWrite(c: number, out: Uint8Array, at: number): number {
	if (c < 0x80) {
		out[at++] = c;
	} else if (c < 0x800) {
		out[at++] = 0xc0 | (c >> 6);
		out[at++] = 0x80 | (c & 0x3f);
	} else if (c < 0x10000) {
		out[at++] = 0xe0 | (c >> 12);
		out[at++] = 0x80 | ((c >> 6) & 0x3f);
		out[at++] = 0x80 | (c & 0x3f);
	} else {
		out[at++] = 0xf0 | (c >> 18);
		out[at++] = 0x80 | ((c >> 12) & 0x3f);
		out[at++] = 0x80 | ((c >> 6) & 0x3f);
		out[at++] = 0x80 | (c & 0x3f);
	}
	return at;
}

/** Write the UTF-8 of `s` to the start of `out`, which has room for it: how many bytes it wrote. */
export function utf8Into(s: string, out: Uint8Array): number {
	if (s.length > ENCODE_SHORT) return encoder.encodeInto(s, out).written;
	let at = 0;
	for (let i = 0; i < s.length; i++) {
		const u = s.charCodeAt(i);
		if (u >= 0x80) return utf8Write(s, i, out, at);
		out[at++] = u;
	}
	return at;
}

/** The text of `v`, or `undefined` if it is not UTF-8 as std's `str::from_utf8` reads it. */
export function utf8Decode(v: Uint8Array): string | undefined {
	const n = v.length;
	if (n > DECODE_SHORT) {
		try {
			return decoder.decode(v);
		} catch {
			return undefined;
		}
	}
	let k = 0;
	for (let i = 0; i < n; ) {
		const b = v[i];
		if (b < 0x80) {
			units[k++] = b;
			i++;
		} else if (b >= 0xc2 && b <= 0xdf) {
			if (!tail(v, i + 1, 0x80, 0xbf)) return undefined;
			units[k++] = ((b & 0x1f) << 6) | (v[i + 1] & 0x3f);
			i += 2;
		} else if (b >= 0xe0 && b <= 0xef) {
			// No overlong form, and no surrogate.
			if (!tail(v, i + 1, b === 0xe0 ? 0xa0 : 0x80, b === 0xed ? 0x9f : 0xbf) || !tail(v, i + 2, 0x80, 0xbf)) return undefined;
			units[k++] = ((b & 0x0f) << 12) | ((v[i + 1] & 0x3f) << 6) | (v[i + 2] & 0x3f);
			i += 3;
		} else if (b >= 0xf0 && b <= 0xf4) {
			// No overlong form, and nothing past U+10FFFF.
			if (!tail(v, i + 1, b === 0xf0 ? 0x90 : 0x80, b === 0xf4 ? 0x8f : 0xbf) || !tail(v, i + 2, 0x80, 0xbf) || !tail(v, i + 3, 0x80, 0xbf)) {
				return undefined;
			}
			const c = (((b & 0x07) << 18) | ((v[i + 1] & 0x3f) << 12) | ((v[i + 2] & 0x3f) << 6) | (v[i + 3] & 0x3f)) - 0x10000;
			units[k++] = 0xd800 | (c >> 10);
			units[k++] = 0xdc00 | (c & 0x3ff);
			i += 4;
		} else {
			return undefined;
		}
	}
	// One call makes a flat string, where `+=` of each would make a rope in V8.
	return String.fromCharCode.apply(null, units.subarray(0, k) as unknown as number[]);
}

/** Whether `v` has a byte at `i`, from `lo` to `hi`. */
function tail(v: Uint8Array, i: number, lo: number, hi: number): boolean {
	const b = v[i];
	return b !== undefined && b >= lo && b <= hi;
}
