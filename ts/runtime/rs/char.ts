// `char`'s Unicode classes, case maps, escapes, and encodings, for rust2-ts output, and the
// iterators they make, and `str`'s case maps and escapes. `rs.ts` re-exports all of it.
//
// Each reads std's own Unicode tables, which `rs/unicode.ts` holds, rather than the
// engine's, whose Unicode version may differ from std's.

import { type ByteSlice, type Debug, debugListEntry, debugListFinish, debugStructField, debugStructFinish, debugTupleField, debugTupleFinish, Err, type FmtResult, type Formatter, fmtCharDebug, fmtDebugList, fmtDebugStruct, fmtDebugStructFields, fmtDebugTuple, fmtOption, fmtOwned, fmtPad, fmtSpec, fmtUnit, fmtWriteFmt, OK, Ok, type Place, panic, panicFmt, type Result, Slice } from "../rs.ts";
import { type char, type u8, type u16, u16Debug, u16LowerHex, type u32, u32UpperHex, type usize, usizeDisplay } from "./int.ts";
import { type Clones, DONE, type Done, Iter, iterFlatMap, iterHeld, type SizeHint } from "./iter.ts";
import { type String as RsString, strChars, stringCased } from "./str.ts";
import { drop as dispose } from "./task.ts";
import { ALPHABETIC, CASE_IGNORABLE, LOWERCASE, NUMERIC, TITLECASE, TO_LOWERCASE, TO_LOWERCASE_MULTI, TO_UPPERCASE, TO_UPPERCASE_MULTI, UNPRINTABLE, UPPERCASE } from "./unicode.ts";
import type { Location } from "./location.ts";

/** A class of chars, as runs from one of `rs/unicode.ts`'s tables, decoded once one is looked up. */
class Runs {
	readonly #table: readonly number[];
	// Where each run starts and ends, in order.
	#bounds: Uint32Array | undefined;

	constructor(table: readonly number[]) {
		this.#table = table;
	}

	/** Whether `point` is in one of the runs. */
	has(point: number): boolean {
		if (this.#bounds === undefined) {
			this.#bounds = new Uint32Array(this.#table.length);
			let pos = 0;
			for (let i = 0; i < this.#table.length; i++) this.#bounds[i] = pos += this.#table[i];
		}
		// How many bounds are at most `point`: odd inside a run.
		const bounds = this.#bounds;
		let lo = 0;
		let hi = bounds.length;
		while (lo < hi) {
			const mid = (lo + hi) >>> 1;
			if (bounds[mid] <= point) lo = mid + 1;
			else hi = mid;
		}
		return (lo & 1) === 1;
	}
}

const encoder = /* @__PURE__ */ new TextEncoder();
// A byte order mark is a char of the string, as std keeps it.
const decoder = /* @__PURE__ */ new TextDecoder("utf-8", { ignoreBOM: true });

const unprintable = /* @__PURE__ */ new Runs(UNPRINTABLE);
const alphabetic = /* @__PURE__ */ new Runs(ALPHABETIC);
const lowercase = /* @__PURE__ */ new Runs(LOWERCASE);
const uppercase = /* @__PURE__ */ new Runs(UPPERCASE);
const numeric = /* @__PURE__ */ new Runs(NUMERIC);
const titlecase = /* @__PURE__ */ new Runs(TITLECASE);
const caseIgnorable = /* @__PURE__ */ new Runs(CASE_IGNORABLE);

/** `char::is_alphabetic`: Unicode's `Alphabetic`. */
export function charIsAlphabetic(c: char): boolean {
	return alphabetic.has(c);
}

/** `char::is_lowercase`: Unicode's `Lowercase`. */
export function charIsLowercase(c: char): boolean {
	return lowercase.has(c);
}

/** `char::is_uppercase`: Unicode's `Uppercase`. */
export function charIsUppercase(c: char): boolean {
	return uppercase.has(c);
}

/** `char::is_numeric`: Unicode's general categories for numbers, `Nd`, `Nl`, and `No`. */
export function charIsNumeric(c: char): boolean {
	return numeric.has(c);
}

/** `char::is_alphanumeric`: either `is_alphabetic` or `is_numeric`. */
export function charIsAlphanumeric(c: char): boolean {
	return alphabetic.has(c) || numeric.has(c);
}

/** `char::is_control`: Unicode's `Cc`, which never changes. */
export function charIsControl(c: char): boolean {
	return c <= 0x1f || (c >= 0x7f && c <= 0x9f);
}

/** A case map, from one of `rs/unicode.ts`'s tables of runs and of chars that map to more than one, decoded once one is looked up. */
class CaseMap {
	readonly #table: readonly number[];
	readonly #multi: readonly number[];
	// Each run's first char, its last, its step, and how far past each its char is.
	#starts: Uint32Array | undefined;
	#lasts = new Uint32Array(0);
	#steps = new Uint8Array(0);
	#deltas = new Int32Array(0);
	#many = new Map<number, char[]>();

	constructor(table: readonly number[], multi: readonly number[]) {
		this.#table = table;
		this.#multi = multi;
	}

	/** What `c` maps to: one char or more. */
	map(c: char): char[] {
		if (this.#starts === undefined) this.#decode();
		const many = this.#many.get(c);
		if (many !== undefined) return many;
		const starts = this.#starts as Uint32Array;
		// The last run that starts at or before `c`.
		let lo = 0;
		let hi = starts.length;
		while (lo < hi) {
			const mid = (lo + hi) >>> 1;
			if (starts[mid] <= c) lo = mid + 1;
			else hi = mid;
		}
		const i = lo - 1;
		if (i >= 0 && c <= this.#lasts[i] && (c - starts[i]) % this.#steps[i] === 0) return [(c + this.#deltas[i]) as char];
		return [c];
	}

	#decode(): void {
		const table = this.#table;
		const runs = table.length / 4;
		const starts = new Uint32Array(runs);
		this.#lasts = new Uint32Array(runs);
		this.#steps = new Uint8Array(runs);
		this.#deltas = new Int32Array(runs);
		let end = 0;
		for (let i = 0; i < runs; i++) {
			const [gap, len, step, delta] = table.slice(4 * i, 4 * i + 4);
			starts[i] = end + gap;
			this.#lasts[i] = starts[i] + (len - 1) * step;
			this.#steps[i] = step;
			this.#deltas[i] = delta;
			end = this.#lasts[i] + 1;
		}
		const multi = this.#multi;
		let point = 0;
		for (let i = 0; i < multi.length; ) {
			point += multi[i];
			const len = multi[i + 1];
			this.#many.set(point, multi.slice(i + 2, i + 2 + len) as char[]);
			i += 2 + len;
		}
		this.#starts = starts;
	}
}

const toLowercase = /* @__PURE__ */ new CaseMap(TO_LOWERCASE, TO_LOWERCASE_MULTI);
const toUppercase = /* @__PURE__ */ new CaseMap(TO_UPPERCASE, TO_UPPERCASE_MULTI);

/** `str::to_lowercase`: each char's, but a capital sigma's at the end of a word, which is final. */
export function strToLowercase(s: string): RsString {
	let out = "";
	for (let i = 0; i < s.length; ) {
		const c = s.codePointAt(i) as char;
		const end = i + (c > 0xffff ? 2 : 1);
		if (c === 0x3a3) out += finalSigma(s, i, end) ? "ς" : "σ";
		else out += String.fromCodePoint(...toLowercase.map(c));
		i = end;
	}
	return stringCased(s, out);
}

/**
 * Whether the sigma from `start` to `end` of `s` ends a word, as Unicode's `Final_Sigma`
 * says: a cased char before it and none after, past any case-ignorable ones.
 */
function finalSigma(s: string, start: number, end: number): boolean {
	return casedNext(s, start, true) && !casedNext(s, end, false);
}

/** Whether the first char of `s` from `i` on, or before it if `back`, that is not case-ignorable is cased. */
function casedNext(s: string, i: number, back: boolean): boolean {
	while (back ? i > 0 : i < s.length) {
		if (back) i -= isLowSurrogate(s.charCodeAt(i - 1)) ? 2 : 1;
		const c = s.codePointAt(i) as number;
		if (!back) i += c > 0xffff ? 2 : 1;
		if (!caseIgnorable.has(c)) return lowercase.has(c) || uppercase.has(c) || titlecase.has(c);
	}
	return false;
}

/** Whether `u` is the second half of a surrogate pair. */
function isLowSurrogate(u: number): boolean {
	return u >= 0xdc00 && u <= 0xdfff;
}

/** `str::to_uppercase`: each char's. */
export function strToUppercase(s: string): RsString {
	let out = "";
	for (let i = 0; i < s.length; ) {
		const c = s.codePointAt(i) as char;
		out += String.fromCodePoint(...toUppercase.map(c));
		i += c > 0xffff ? 2 : 1;
	}
	return stringCased(s, out);
}

/** The chars a case map or an escape still has, from either end, exactly sized. */
abstract class Chars extends Iter<char> {
	protected readonly chars: readonly char[];
	protected front = 0;
	protected back: number;

	constructor(chars: readonly char[]) {
		super();
		this.chars = chars;
		this.back = chars.length;
	}

	next(): char | Done {
		return this.front < this.back ? this.chars[this.front++] : DONE;
	}

	override nextBack(): char | Done {
		return this.front < this.back ? this.chars[--this.back] : DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return [this.len(), this.len()];
	}

	/** The chars still alive. */
	rest(): string {
		return String.fromCodePoint(...this.chars.slice(this.front, this.back));
	}

	/** `Display`, unpadded. */
	abstract display(f: Formatter): FmtResult;

	/** `Debug`, of the iterator named `name`. */
	abstract debug(f: Formatter, name: string): FmtResult;
}

/** `ToLowercase` and `ToUppercase`: std's `CaseMappingIter` of an `array::IntoIter`. */
class CaseMapping extends Chars {
	override clone(): CaseMapping {
		const it = new CaseMapping(this.chars);
		it.front = this.front;
		it.back = this.back;
		return it;
	}

	/** Each char still alive, a `write_char` apiece. */
	display(f: Formatter): FmtResult {
		for (let i = this.front; i < this.back; i++) {
			const r = f.out.writeChar(this.chars[i]);
			if (!r.ok) return r;
		}
		return OK;
	}

	debug(f: Formatter, name: string): FmtResult {
		const list = (f: Formatter) => {
			let b = fmtDebugList(f);
			for (const c of this.chars.slice(this.front, this.back)) b = debugListEntry(b, (f) => fmtCharDebug(c, f));
			return debugListFinish(b);
		};
		const into = (f: Formatter) => debugTupleFinish(debugTupleField(fmtDebugTuple(f, "IntoIter"), list));
		const inner = (f: Formatter) => debugTupleFinish(debugTupleField(fmtDebugTuple(f, "CaseMappingIter"), into));
		return debugTupleFinish(debugTupleField(fmtDebugTuple(f, name), inner));
	}
}

/** `EscapeUnicode`, `EscapeDefault`, and `EscapeDebug`: std's `EscapeIterInner` of what it writes. */
class Escape extends Chars {
	// Whether it is `escape_debug`'s char as it is, which std writes even once it has yielded it.
	readonly #literal: boolean;

	constructor(chars: readonly char[], literal: boolean) {
		super(chars);
		this.#literal = literal;
	}

	override clone(): Escape {
		const it = new Escape(this.chars, this.#literal);
		it.front = this.front;
		it.back = this.back;
		return it;
	}

	/** The literal char by `write_char`, else what is still alive by one `write_str`. */
	display(f: Formatter): FmtResult {
		return this.#literal ? f.out.writeChar(this.chars[0]) : f.out.writeStr(this.rest());
	}

	debug(f: Formatter, name: string): FmtResult {
		const quoted = (f: Formatter) => fmtWriteFmt(f, ["'", (f) => this.display(f), "'"]);
		const inner = (f: Formatter) => debugTupleFinish(debugTupleField(fmtDebugTuple(f, "EscapeIterInner"), quoted));
		return debugTupleFinish(debugTupleField(fmtDebugTuple(f, name), inner));
	}
}

/** `char::to_lowercase`. */
export function charToLowercase(c: char): Iter<char> {
	return new CaseMapping(toLowercase.map(c));
}

/** `char::to_uppercase`. */
export function charToUppercase(c: char): Iter<char> {
	return new CaseMapping(toUppercase.map(c));
}

/** The chars of `s`, an escape, or a char as it is if `literal`. */
function escape(s: string, literal = false): Escape {
	return new Escape(Array.from(s, (c) => c.codePointAt(0) as char), literal);
}

/** `char::escape_unicode`: `\u{..}` of its scalar in lowercase hex. */
export function charEscapeUnicode(c: char): Iter<char> {
	return escape(`\\u{${c.toString(16)}}`);
}

/** `char::escape_default`: a backslash before a tab, carriage return, line feed, quote, or backslash, printable ASCII as it is, and else `escape_unicode`. */
export function charEscapeDefault(c: char): Iter<char> {
	switch (c) {
		case 0x09:
			return escape("\\t");
		case 0x0d:
			return escape("\\r");
		case 0x0a:
			return escape("\\n");
		case 0x22:
		case 0x27:
		case 0x5c:
			return escape(`\\${String.fromCharCode(c)}`);
	}
	return c >= 0x20 && c <= 0x7e ? escape(String.fromCharCode(c)) : charEscapeUnicode(c);
}

/** `char::escape_debug`: as `Debug` of a char escapes it, and `"` too. */
export function charEscapeDebug(c: char): Iter<char> {
	const s = String.fromCodePoint(c);
	const escaped = c === 0x22 ? '\\"' : escapeDebug(s, "'");
	return escape(escaped, escaped === s);
}

/** `str::escape_unicode`: each char's `escape_unicode`, as std's `flat_map` of them. */
export function strEscapeUnicode(s: string): Iter<char> {
	return iterFlatMap(strChars(s), charEscapeUnicode, undefined, true);
}

/** `str::escape_default`: each char's `escape_default`, as std's `flat_map` of them. */
export function strEscapeDefault(s: string): Iter<char> {
	return iterFlatMap(strChars(s), charEscapeDefault, undefined, true);
}

/** `str::escape_debug`: each char's `escape_debug`, as std's `flat_map` of them. */
export function strEscapeDebug(s: string): Iter<char> {
	return iterFlatMap(strChars(s), charEscapeDebug, undefined, true);
}

/** `Display` of a `str`'s escape: what it has left, a char at a time, as std's `write_char` of a clone's. */
export function fmtStrEscapeDisplay(it: Iter<char>, f: Formatter): FmtResult {
	const rest = it.clone();
	for (let c = rest.next(); c !== DONE; c = rest.next()) {
		const r = f.out.writeChar(c);
		if (!r.ok) return r;
	}
	return OK;
}

/** One char of `Debug` of a string, which escapes `"`, or of a char, which escapes `'`. */
export function escapeDebug(c: string, quote: '"' | "'"): string {
	switch (c) {
		case quote:
			return `\\${quote}`;
		case "\\":
			return "\\\\";
		case "\n":
			return "\\n";
		case "\t":
			return "\\t";
		case "\r":
			return "\\r";
		case "\0":
			return "\\0";
	}
	const point = c.codePointAt(0) as number;
	return unprintable.has(point) ? `\\u{${point.toString(16)}}` : c;
}

/** `Display` of a `ToLowercase`, a `ToUppercase`, or a char's escape: what it has left, unpadded, as std writes it by `write_char` or `write_str`. */
export function fmtCharsDisplay(it: Iter<char>, f: Formatter): FmtResult {
	return (it as Chars).display(f);
}

/** `Debug` of a `ToLowercase`, a `ToUppercase`, or a char's escape, named `name`: what it has left, in std's derived fields. */
export function fmtCharsDebug(it: Iter<char>, f: Formatter, name: string): FmtResult {
	return (it as Chars).debug(f, name);
}

/** `char::len_utf8`. */
export function charLenUtf8(c: char): usize {
	return (c < 0x80 ? 1 : c < 0x800 ? 2 : c < 0x10000 ? 3 : 4) as usize;
}

/** `char::len_utf16`. */
export function charLenUtf16(c: char): usize {
	return (c < 0x10000 ? 1 : 2) as usize;
}

/** std's panic for a buffer too short to encode `c` into, by `encode_utf8` or `encode_utf16`. */
function tooShort(encoding: string, c: char, len: number, has: number): never {
	const code = fmtSpec({ zero: true, width: 4 }, fmtOwned(c as u32, u32UpperHex));
	panicFmt([`${encoding}: need `, fmtOwned(len as usize, usizeDisplay), " bytes to encode U+", code, " but buffer has just ", fmtOwned(has as usize, usizeDisplay)], encoding);
}

/** The `&mut str` `encode_utf8` returns: the bytes it wrote, which a write changes in place. */
class EncodedUtf8 implements Place<string> {
	readonly #bytes: Uint8Array;

	constructor(bytes: Uint8Array) {
		this.#bytes = bytes;
	}

	get v(): string {
		return decoder.decode(this.#bytes);
	}

	// A write through a `&mut str` keeps its length, so it fills the same bytes.
	set v(s: string) {
		encoder.encodeInto(s, this.#bytes);
	}
}

/** `char::encode_utf8`: its UTF-8 written to the front of `dst`, and that part of it as a `&mut str`. */
export function charEncodeUtf8(c: char, dst: ByteSlice): Place<string> {
	const len = charLenUtf8(c);
	if (dst.length < len) tooShort("encode_utf8", c, len, dst.length);
	const bytes = dst.data.subarray(dst.start, dst.start + len);
	encoder.encodeInto(String.fromCodePoint(c), bytes);
	return new EncodedUtf8(bytes);
}

/** `char::encode_utf16`: its UTF-16 written to the front of `dst`, and a view of that part of it. */
export function charEncodeUtf16(c: char, dst: Slice<u16>): Slice<u16> {
	const len = charLenUtf16(c);
	if (dst.length < len) tooShort("encode_utf16", c, len, dst.length);
	const s = String.fromCodePoint(c);
	for (let i = 0; i < len; i++) dst.data[dst.start + i] = s.charCodeAt(i) as u16;
	return new Slice(dst.data, dst.start, len);
}

/** `char::DecodeUtf16Error`: the surrogate that had no pair. */
export class DecodeUtf16Error {
	readonly code: u16;

	constructor(code: u16) {
		this.code = code;
	}
}

/** `char::decode_utf16`: the chars of `it`'s UTF-16, or an error for each unpaired surrogate. */
export function charDecodeUtf16(it: Iter<u16>): Iter<Result<char, DecodeUtf16Error>> {
	return new DecodeUtf16(it);
}

/** `Debug` of a `DecodeUtf16`: its iterator, by `inner`, and the unit it holds to read next. */
export function fmtDecodeUtf16<I>(it: Iter<unknown>, f: Formatter, inner: Debug<I>): FmtResult {
	const d = it as DecodeUtf16;
	return fmtDebugStructFields(f, "DecodeUtf16", ["iter", "buf"], [(f) => inner(iterHeld(d.it), f), (f) => fmtOption(d.buf, f, u16Debug)]);
}

/** `char::DecodeUtf16`: `it`, and the unit after a leading surrogate that was no trailing one, which it reads next. */
class DecodeUtf16 extends Iter<Result<char, DecodeUtf16Error>> implements Disposable {
	readonly it: Iter<u16>;
	buf: u16 | undefined;

	constructor(it: Iter<u16>, buf?: u16) {
		super();
		this.it = it;
		this.buf = buf;
	}

	next(): Result<char, DecodeUtf16Error> | Done {
		let u = this.buf;
		this.buf = undefined;
		if (u === undefined) {
			const next = this.it.next();
			if (next === DONE) return DONE;
			u = next;
		}
		if (u < 0xd800 || u > 0xdfff) return Ok(u as number as char);
		if (u >= 0xdc00) return Err(new DecodeUtf16Error(u));
		const u2 = this.it.next();
		if (u2 === DONE) return Err(new DecodeUtf16Error(u));
		if (u2 < 0xdc00 || u2 > 0xdfff) {
			this.buf = u2;
			return Err(new DecodeUtf16Error(u));
		}
		return Ok((((u & 0x3ff) << 10) + (u2 & 0x3ff) + 0x10000) as char);
	}

	/** std's: half `it`'s lower bound, rounded up, and its upper bound, each with the unit it holds. */
	override sizeHint(): SizeHint {
		const [low, high] = this.it.sizeHint();
		const u = this.buf;
		const [lowBuf, highBuf] = u === undefined ? [0, 0] : u < 0xd800 || u > 0xdfff || high === 0 ? [1, 1] : [0, 1];
		const upper = high === undefined || high + highBuf > 0xffffffff ? undefined : high + highBuf;
		return [Math.ceil(low / 2) + lowBuf, upper];
	}

	override clone(h?: Clones): DecodeUtf16 {
		return new DecodeUtf16(this.it.clone(h?.i), this.buf);
	}

	[Symbol.dispose](): void {
		dispose(this.it);
	}
}

/** `Display` of a `DecodeUtf16Error`. */
export function fmtDecodeUtf16ErrorDisplay(e: DecodeUtf16Error, f: Formatter): FmtResult {
	return fmtWriteFmt(f, ["unpaired surrogate found: ", (f) => u16LowerHex(e.code, f)]);
}

/** `Debug` of a `DecodeUtf16Error`. */
export function fmtDecodeUtf16ErrorDebug(e: DecodeUtf16Error, f: Formatter): FmtResult {
	return debugStructFinish(debugStructField(fmtDebugStruct(f, "DecodeUtf16Error"), "code", (f) => u16Debug(e.code, f)));
}

/** `char::from_digit`: the char of `num` in `radix`, lowercase past 9, if it is a digit of it. */
export function charFromDigit(num: u32, radix: u32, at?: Location): char | undefined {
	if (radix > 36) panic("from_digit: radix is too high (maximum 36)", at);
	if (num >= radix) return undefined;
	return (num < 10 ? 0x30 + num : 0x61 + num - 10) as char;
}

/** `char::CharTryFromError`. */
export class CharTryFromError {}

/** `char::TryFromCharError`. */
export class TryFromCharError {}

/** `char`'s `try_from` a `u32`: it, unless it is a surrogate or past the last char. */
export function charTryFrom(n: u32): Result<char, CharTryFromError> {
	return n > 0x10ffff || (n >= 0xd800 && n <= 0xdfff) ? Err(new CharTryFromError()) : Ok(n as number as char);
}

/** `u8`'s `try_from` a `char`: its scalar, if it is at most `u8::MAX`. */
export function u8TryFromChar(c: char): Result<u8, TryFromCharError> {
	return c > 0xff ? Err(new TryFromCharError()) : Ok(c as number as u8);
}

/** `u16`'s `try_from` a `char`: its scalar, if it is at most `u16::MAX`. */
export function u16TryFromChar(c: char): Result<u16, TryFromCharError> {
	return c > 0xffff ? Err(new TryFromCharError()) : Ok(c as number as u16);
}

/** `Display` of a `CharTryFromError`, padded as a `str`. */
export function fmtCharTryFromErrorDisplay(_: CharTryFromError, f: Formatter): FmtResult {
	return fmtPad(f, "converted integer out of range for `char`");
}

/** `Debug` of a `CharTryFromError`. */
export function fmtCharTryFromErrorDebug(_: CharTryFromError, f: Formatter): FmtResult {
	return debugTupleFinish(debugTupleField(fmtDebugTuple(f, "CharTryFromError"), (f) => fmtUnit(undefined, f)));
}

/** `Display` of a `TryFromCharError`, padded as a `str`. */
export function fmtTryFromCharErrorDisplay(_: TryFromCharError, f: Formatter): FmtResult {
	return fmtPad(f, "unicode code point out of range");
}

/** `Debug` of a `TryFromCharError`. */
export function fmtTryFromCharErrorDebug(_: TryFromCharError, f: Formatter): FmtResult {
	return debugTupleFinish(debugTupleField(fmtDebugTuple(f, "TryFromCharError"), (f) => fmtUnit(undefined, f)));
}
