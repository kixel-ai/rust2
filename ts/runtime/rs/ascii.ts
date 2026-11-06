// The ASCII methods of `u8`, `char`, `[u8]`, and `str`, for rust2-ts output, and the
// iterators `escape_ascii` makes. `rs.ts` re-exports all of it.
//
// A `u8` and a `char` share each function: std's char ones are its byte ones' for an ASCII
// char, and past ASCII no test holds and no case map changes it, as for a byte past 0x7f.

import { ByteSlice, ByteVec, byteSliceView, byteVecFrom, type FmtResult, type Formatter, OK } from "../rs.ts";
import type { char, u8 } from "./int.ts";
import { type Done, DONE, Iter, type SizeHint } from "./iter.ts";

/** A `u8` or a `char`: the value of either, its byte or its scalar. */
type Ascii = u8 | char;

/** `is_ascii`. */
export function isAscii(c: Ascii): boolean {
	return c < 0x80;
}

/** `is_ascii_alphabetic`. */
export function isAsciiAlphabetic(c: Ascii): boolean {
	return isAsciiUppercase(c) || isAsciiLowercase(c);
}

/** `is_ascii_uppercase`. */
export function isAsciiUppercase(c: Ascii): boolean {
	return c >= 0x41 && c <= 0x5a;
}

/** `is_ascii_lowercase`. */
export function isAsciiLowercase(c: Ascii): boolean {
	return c >= 0x61 && c <= 0x7a;
}

/** `is_ascii_alphanumeric`. */
export function isAsciiAlphanumeric(c: Ascii): boolean {
	return isAsciiDigit(c) || isAsciiAlphabetic(c);
}

/** `is_ascii_digit`. */
export function isAsciiDigit(c: Ascii): boolean {
	return c >= 0x30 && c <= 0x39;
}

/** `is_ascii_hexdigit`. */
export function isAsciiHexdigit(c: Ascii): boolean {
	return isAsciiDigit(c) || (c >= 0x41 && c <= 0x46) || (c >= 0x61 && c <= 0x66);
}

/** `is_ascii_punctuation`: `!` to `/`, `:` to `@`, `[` to `` ` ``, and `{` to `~`. */
export function isAsciiPunctuation(c: Ascii): boolean {
	return (c >= 0x21 && c <= 0x2f) || (c >= 0x3a && c <= 0x40) || (c >= 0x5b && c <= 0x60) || (c >= 0x7b && c <= 0x7e);
}

/** `is_ascii_graphic`: `!` to `~`. */
export function isAsciiGraphic(c: Ascii): boolean {
	return c >= 0x21 && c <= 0x7e;
}

/** `is_ascii_whitespace`: tab, line feed, form feed, carriage return, and space, but not vertical tab. */
export function isAsciiWhitespace(c: Ascii): boolean {
	return c === 0x20 || c === 0x09 || c === 0x0a || c === 0x0c || c === 0x0d;
}

/** `is_ascii_control`. */
export function isAsciiControl(c: Ascii): boolean {
	return c < 0x20 || c === 0x7f;
}

/** `to_ascii_uppercase`. */
export function toAsciiUppercase<T extends Ascii>(c: T): T {
	return (isAsciiLowercase(c) ? c - 0x20 : c) as T;
}

/** `to_ascii_lowercase`. */
export function toAsciiLowercase<T extends Ascii>(c: T): T {
	return (isAsciiUppercase(c) ? c + 0x20 : c) as T;
}

/** `eq_ignore_ascii_case`. */
export function eqIgnoreAsciiCase(a: Ascii, b: Ascii): boolean {
	return toAsciiLowercase(a) === toAsciiLowercase(b);
}

/** `[u8]::is_ascii`. */
export function byteSliceIsAscii(s: ByteSlice): boolean {
	return byteSliceView(s).every((b) => b < 0x80);
}

/** `[u8]::eq_ignore_ascii_case`. */
export function byteSliceEqIgnoreAsciiCase(a: ByteSlice, b: ByteSlice): boolean {
	if (a.length !== b.length) return false;
	const x = byteSliceView(a);
	const y = byteSliceView(b);
	return x.every((c, i) => eqIgnoreAsciiCase(c as u8, y[i] as u8));
}

/** `[u8]::make_ascii_uppercase`, in the bytes the slice views. */
export function byteSliceMakeAsciiUppercase(s: ByteSlice): void {
	const v = byteSliceView(s);
	v.forEach((c, i) => (v[i] = toAsciiUppercase(c as u8)));
}

/** `[u8]::make_ascii_lowercase`, in the bytes the slice views. */
export function byteSliceMakeAsciiLowercase(s: ByteSlice): void {
	const v = byteSliceView(s);
	v.forEach((c, i) => (v[i] = toAsciiLowercase(c as u8)));
}

/** `[u8]::to_ascii_uppercase`. */
export function byteSliceToAsciiUppercase(s: ByteSlice): ByteVec {
	return byteVecFrom(byteSliceView(s).map((c) => toAsciiUppercase(c as u8)));
}

/** `[u8]::to_ascii_lowercase`. */
export function byteSliceToAsciiLowercase(s: ByteSlice): ByteVec {
	return byteVecFrom(byteSliceView(s).map((c) => toAsciiLowercase(c as u8)));
}

/** `[u8]::trim_ascii_start`. */
export function byteSliceTrimAsciiStart(s: ByteSlice): ByteSlice {
	let start = s.start;
	const end = s.start + s.length;
	while (start < end && isAsciiWhitespace(s.data[start] as u8)) start++;
	return new ByteSlice(s.data, start, end - start);
}

/** `[u8]::trim_ascii_end`. */
export function byteSliceTrimAsciiEnd(s: ByteSlice): ByteSlice {
	let end = s.start + s.length;
	while (end > s.start && isAsciiWhitespace(s.data[end - 1] as u8)) end--;
	return new ByteSlice(s.data, s.start, end - s.start);
}

/** `[u8]::trim_ascii`. */
export function byteSliceTrimAscii(s: ByteSlice): ByteSlice {
	return byteSliceTrimAsciiEnd(byteSliceTrimAsciiStart(s));
}

/** `str::is_ascii`: each UTF-16 unit is a UTF-8 byte below 0x80 exactly where it is ASCII. */
export function strIsAscii(s: string): boolean {
	return /^[\0-\x7f]*$/.test(s);
}

/**
 * `str::eq_ignore_ascii_case`. std compares UTF-8 bytes, of which only an ASCII char's are
 * below 0x80, so it holds exactly where it does for their UTF-16 units.
 */
export function strEqIgnoreAsciiCase(a: string, b: string): boolean {
	if (a.length !== b.length) return false;
	for (let i = 0; i < a.length; i++) {
		if (!eqIgnoreAsciiCase(a.charCodeAt(i) as char, b.charCodeAt(i) as char)) return false;
	}
	return true;
}

/** `str::to_ascii_uppercase`: only `a` to `z`, which JS's Unicode case map maps as ASCII's does. */
export function strToAsciiUppercase(s: string): string {
	return s.replace(/[a-z]+/g, (m) => m.toUpperCase());
}

/** `str::to_ascii_lowercase`: only `A` to `Z`. */
export function strToAsciiLowercase(s: string): string {
	return s.replace(/[A-Z]+/g, (m) => m.toLowerCase());
}

/** `str::trim_ascii_start`. */
export function strTrimAsciiStart(s: string): string {
	return s.replace(/^[\t\n\f\r ]+/, "");
}

/** `str::trim_ascii_end`. */
export function strTrimAsciiEnd(s: string): string {
	return s.replace(/[\t\n\f\r ]+$/, "");
}

/** `str::trim_ascii`. */
export function strTrimAscii(s: string): string {
	return strTrimAsciiEnd(strTrimAsciiStart(s));
}

/** What `escape_ascii` writes of `b`: a backslash before a tab, line feed, carriage return, quote, or backslash, a hex escape of any other byte outside `0x20..=0x7e`, and the byte itself otherwise. */
function escaped(b: number): string {
	switch (b) {
		case 0x09:
			return "\\t";
		case 0x0a:
			return "\\n";
		case 0x0d:
			return "\\r";
		case 0x22:
		case 0x27:
		case 0x5c:
			return `\\${String.fromCharCode(b)}`;
	}
	return b >= 0x20 && b <= 0x7e ? String.fromCharCode(b) : `\\x${b.toString(16).padStart(2, "0")}`;
}

/** `u8::escape_ascii` and `ascii::escape_default`. */
export function escapeAscii(b: u8): Iter<u8> {
	return new EscapeDefault(escaped(b));
}

/** `ascii::EscapeDefault`: the escape's bytes still alive, from either end, exactly sized. */
class EscapeDefault extends Iter<u8> {
	readonly #text: string;
	#front = 0;
	#back: number;

	constructor(text: string) {
		super();
		this.#text = text;
		this.#back = text.length;
	}

	next(): u8 | Done {
		return this.#front < this.#back ? (this.#text.charCodeAt(this.#front++) as u8) : DONE;
	}

	override nextBack(): u8 | Done {
		return this.#front < this.#back ? (this.#text.charCodeAt(--this.#back) as u8) : DONE;
	}

	override len(): number {
		return this.#back - this.#front;
	}

	override sizeHint(): SizeHint {
		return [this.len(), this.len()];
	}

	override clone(): EscapeDefault {
		const it = new EscapeDefault(this.#text);
		it.#front = this.#front;
		it.#back = this.#back;
		return it;
	}

	/** `Display`: the bytes still alive, by one `write_str`. */
	display(f: Formatter): FmtResult {
		return f.out.writeStr(this.#text.slice(this.#front, this.#back));
	}

	/** Each byte still alive, a `write_char` apiece, as `EscapeAscii` writes its spent escapes. */
	displayChars(f: Formatter): FmtResult {
		for (let i = this.#front; i < this.#back; i++) {
			const r = f.out.writeChar(this.#text.charCodeAt(i) as char);
			if (!r.ok) return r;
		}
		return OK;
	}
}

/** `[u8]::escape_ascii`. */
export function byteSliceEscapeAscii(s: ByteSlice): Iter<u8> {
	return new EscapeAscii(s.data, s.start, s.start + s.length);
}

/**
 * `slice::EscapeAscii`, std's `flat_map` of each byte's escape: the escape it reads from the
 * front, the bytes it has not reached from either end, and the escape it reads from the back.
 */
class EscapeAscii extends Iter<u8> {
	readonly #data: Uint8Array;
	#start: number;
	#end: number;
	#front: EscapeDefault | undefined;
	#back: EscapeDefault | undefined;

	constructor(data: Uint8Array, start: number, end: number) {
		super();
		this.#data = data;
		this.#start = start;
		this.#end = end;
	}

	next(): u8 | Done {
		for (;;) {
			if (this.#front !== undefined) {
				const b = this.#front.next();
				if (b !== DONE) return b;
				this.#front = undefined;
			}
			if (this.#start === this.#end) {
				const b = this.#back?.next() ?? DONE;
				if (b === DONE) this.#back = undefined;
				return b;
			}
			this.#front = escapeAscii(this.#data[this.#start++] as u8) as EscapeDefault;
		}
	}

	override nextBack(): u8 | Done {
		for (;;) {
			if (this.#back !== undefined) {
				const b = this.#back.nextBack();
				if (b !== DONE) return b;
				this.#back = undefined;
			}
			if (this.#start === this.#end) {
				const b = this.#front?.nextBack() ?? DONE;
				if (b === DONE) this.#front = undefined;
				return b;
			}
			this.#back = escapeAscii(this.#data[--this.#end] as u8) as EscapeDefault;
		}
	}

	/** std's `flat_map`'s: what its two escapes hold, and exactly that once it has no bytes left to reach. */
	override sizeHint(): SizeHint {
		const held = (this.#front?.len() ?? 0) + (this.#back?.len() ?? 0);
		return [held, this.#start === this.#end ? held : undefined];
	}

	override clone(): EscapeAscii {
		const it = new EscapeAscii(this.#data, this.#start, this.#end);
		it.#front = this.#front?.clone();
		it.#back = this.#back?.clone();
		return it;
	}

	/** `Display`: what its escapes hold, a `write_char` a byte, around the bytes it has not reached, each run left as is, empty or not, and each escape a `write_str` apiece. */
	display(f: Formatter): FmtResult {
		let r = this.#front?.displayChars(f) ?? OK;
		const data = this.#data;
		for (let i = this.#start; r.ok && i < this.#end; ) {
			let run = "";
			for (; i < this.#end && escaped(data[i] as number).length === 1; i++) run += String.fromCharCode(data[i] as number);
			r = f.out.writeStr(run);
			if (r.ok && i < this.#end) r = f.out.writeStr(escaped(data[i++] as number));
		}
		return r.ok ? (this.#back?.displayChars(f) ?? OK) : r;
	}
}

/** `Display` of an `ascii::EscapeDefault` or a `slice::EscapeAscii`: what it has left, unpadded. */
export function fmtEscapeAscii(it: Iter<u8>, f: Formatter): FmtResult {
	return (it as EscapeDefault | EscapeAscii).display(f);
}
