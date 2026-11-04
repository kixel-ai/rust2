// `str`'s slices, iterators, and searches, for rust2-ts output. `rt.ts` re-exports all of
// it.
//
// A JS string is UTF-16 where a `str` is UTF-8: these step by code point, and an offset
// they take or return counts UTF-8 bytes, as Rust's does. A string from translated code is
// well formed, so a pattern's text matches where its UTF-8 would, never across a
// surrogate pair.

import {
	ByteSlice,
	byteSliceView,
	Err,
	type FmtResult,
	type Formatter,
	fmtCharDebug,
	fmtChars,
	fmtDebugStructFields,
	fmtDebugTupleFields,
	fmtList,
	fmtPad,
	fmtStr,
	OK,
	Ok,
	type Place,
	panic,
	panicFmt,
	type Result,
	stringLen,
	utf8Check,
} from "../rt.ts";
import { CAPACITY_OVERFLOW, type TryReserveError } from "./alloc.ts";
import { escapeDebug } from "./char.ts";
import { type Bounds, boundsRange, type Part } from "./collections.ts";
import { type char, type u8, type u16, type usize, u8Debug, usizeDebug } from "./int.ts";
import { type Clones, DONE, type Done, Iter, iterFilter, iterForEach, iterMap, iterRev, iterSlice, type RangeInclusive, type SizeHint } from "./iter.ts";
import { drop as dispose } from "./task.ts";

const encoder = /* @__PURE__ */ new TextEncoder();

/** A pattern: a `char` or a `&str`, as its text, or a `FnMut(char) -> bool`. */
export type Pattern = string | ((c: char) => boolean);

/** The UTF-8 length of the code units of `s` from `from` to `to`. */
function utf8Len(s: string, from: number, to: number): number {
	let n = to - from;
	for (let i = from; i < to; i++) {
		const u = s.charCodeAt(i);
		// Each half of a surrogate pair is two of its four bytes.
		if (u >= 0x80) n += u < 0x800 || (u >= 0xd800 && u <= 0xdfff) ? 1 : 2;
	}
	return n;
}

/** Where the char that ends at `end` starts. */
function before(s: string, end: number): number {
	const u = s.charCodeAt(end - 1);
	return u >= 0xdc00 && u <= 0xdfff ? end - 2 : end - 1;
}

/** A char's UTF-8 length. */
function charLen(c: number): number {
	return c < 0x80 ? 1 : c < 0x800 ? 2 : c < 0x10000 ? 3 : 4;
}

/**
 * The code unit of `s` where byte `to` starts a char, reading on from code unit `i`, which
 * is byte `from`; -1 if `to` is inside a char, past the end, or before `from`.
 */
function unitAt(s: string, i: number, from: number, to: number): number {
	while (from < to && i < s.length) {
		const u = s.charCodeAt(i);
		if (u >= 0xd800 && u <= 0xdbff) {
			// A surrogate pair is one char of four bytes.
			from += 4;
			i += 2;
		} else {
			from += u < 0x80 ? 1 : u < 0x800 ? 2 : 3;
			i++;
		}
	}
	return from === to ? i : -1;
}

/** `&s[start..end]` by UTF-8 byte offsets, with `end` defaulting to the length. */
export function strSlice(s: string, start: number, end?: number): string {
	return s.slice(...units(s, start, end));
}

/** `str::get` by a range's UTF-8 byte offsets, with `end` defaulting to the length. */
export function strGet(s: string, start: number, end?: number): string | undefined {
	const at = unitsChecked(s, start, end);
	return at && s.slice(...at);
}

/**
 * The byte offsets `&s[r]` reads for an `a..=b` or a `..=b`, panicking as std's does where
 * `b` is not before the end; the part is checked as `a..b + 1`'s is.
 */
export function strInclusive(s: string, r: RangeInclusive<usize> | { readonly end: usize; readonly inclusive: true }): [number, number] {
	const [lo, hi] = "lo" in r ? [r.lo, r.hi] : [0, r.end];
	if (hi >= stringLen(s)) sliceFailed(s, lo, hi);
	// An `a..=b` iterated to its end starts past it, at `b + 1`, which JS doesn't overflow.
	return ["exhausted" in r && r.exhausted ? hi + 1 : lo, hi + 1];
}

/**
 * The code units of `s` from byte `start` to byte `end`, or to the end, or `undefined` if
 * either is out of bounds or inside a char, or they are backwards.
 */
function unitsChecked(s: string, start: number, end?: number): [number, number] | undefined {
	const from = unitAt(s, 0, 0, start);
	if (from < 0) return undefined;
	const to = end === undefined ? s.length : unitAt(s, from, start, end);
	return to < 0 ? undefined : [from, to];
}

/** `unitsChecked`, panicking as std's indexing does where it gives `undefined`. */
function units(s: string, start: number, end?: number): [number, number] {
	return unitsChecked(s, start, end) ?? sliceFailed(s, start, end ?? stringLen(s));
}

/**
 * `&mut p[from..to]` by code units: a part of the string at `p`, whose writes splice the
 * string anew. A write through a `&mut str` keeps each char's length, so the units hold.
 */
class StrPart implements Place<string> {
	readonly #whole: Place<string>;
	readonly #from: number;
	readonly #to: number;

	constructor(whole: Place<string>, [from, to]: [number, number]) {
		this.#whole = whole;
		this.#from = from;
		this.#to = to;
	}

	get v(): string {
		return this.#whole.v.slice(this.#from, this.#to);
	}

	set v(value: string) {
		const s = this.#whole.v;
		this.#whole.v = s.slice(0, this.#from) + value + s.slice(this.#to);
	}
}

/** `&mut p[start..end]` by UTF-8 byte offsets, with `end` defaulting to the length. */
export function strPart(p: Place<string>, start: number, end?: number): Place<string> {
	return new StrPart(p, units(p.v, start, end));
}

/** `str::get_mut` by a range's UTF-8 byte offsets, with `end` defaulting to the length. */
export function strGetMut(p: Place<string>, start: number, end?: number): Place<string> | undefined {
	const at = unitsChecked(p.v, start, end);
	return at && new StrPart(p, at);
}

/** `str::split_at_mut`. */
export function strSplitAtMut(p: Place<string>, mid: number): [Place<string>, Place<string>] {
	const [, at] = units(p.v, 0, mid);
	return [new StrPart(p, [0, at]), new StrPart(p, [at, p.v.length])];
}

/** `str::split_at_mut_checked`. */
export function strSplitAtMutChecked(p: Place<string>, mid: number): [Place<string>, Place<string>] | undefined {
	const at = unitsChecked(p.v, 0, mid);
	return at && [new StrPart(p, at), new StrPart(p, [at[1], p.v.length])];
}

/**
 * std's panic for a slice of `s` that is out of bounds, backwards, or inside a char, or that
 * ends inclusively at the length.
 */
function sliceFailed(s: string, start: number, end: number): never {
	const len = stringLen(s);
	if (start > len) panic(`start byte index ${start} is out of bounds for string of length ${len}`);
	if (end > len) panic(`end byte index ${end} is out of bounds for string of length ${len}`);
	if (start > end) panic(`byte range starts at ${start} but ends at ${end}`);
	// Neither inside a char: an inclusive end at the length, which is past the last byte.
	const [which, at] =
		unitAt(s, 0, 0, start) < 0
			? ["start", start]
			: unitAt(s, 0, 0, end) < 0
				? ["end", end]
				: panic(`end byte index ${end} is out of bounds for string of length ${len}`);
	// The char `at` is inside, and where its bytes start.
	let floor = 0;
	let c = 0;
	for (const ch of s) {
		c = ch.codePointAt(0) as number;
		if (floor + charLen(c) > at) break;
		floor += charLen(c);
	}
	panicFmt([
		`${which} byte index ${at} is not a char boundary; it is inside `,
		(f) => fmtCharDebug(c as char, f),
		` (bytes ${floor}..${floor + charLen(c)} of string)`,
	]);
}

/** `str::chars`. */
export function strChars(s: string): Iter<char> {
	return new Chars(s);
}

class Chars extends Iter<char> {
	readonly #s: string;
	#front = 0;
	#back: number;

	constructor(s: string) {
		super();
		this.#s = s;
		this.#back = s.length;
	}

	next(): char | Done {
		if (this.#front >= this.#back) return DONE;
		const c = this.#s.codePointAt(this.#front) as number;
		this.#front += c > 0xffff ? 2 : 1;
		return c as char;
	}

	override nextBack(): char | Done {
		if (this.#front >= this.#back) return DONE;
		this.#back = before(this.#s, this.#back);
		return this.#s.codePointAt(this.#back) as char;
	}

	/** std's: a char per four bytes of what is left, and at most one per byte. */
	override sizeHint(): SizeHint {
		const bytes = utf8Len(this.#s, this.#front, this.#back);
		return [Math.ceil(bytes / 4), bytes];
	}

	/** `as_str`: what is left. */
	rest(): string {
		return this.#s.slice(this.#front, this.#back);
	}

	override clone(): Iter<char> {
		const it = new Chars(this.#s);
		it.#front = this.#front;
		it.#back = this.#back;
		return it;
	}

	/** std's `Debug` lists a clone. */
	override remaining(): Iterable<char> {
		return this.clone();
	}
}

/** `str::encode_utf16`: its UTF-16 code units. */
export function strEncodeUtf16(s: string): Iter<u16> {
	return new EncodeUtf16(s);
}

class EncodeUtf16 extends Iter<u16> {
	readonly #s: string;
	#at = 0;

	constructor(s: string) {
		super();
		this.#s = s;
	}

	next(): u16 | Done {
		return this.#at < this.#s.length ? (this.#s.charCodeAt(this.#at++) as u16) : DONE;
	}

	/** std's: a unit per three bytes left at least and one per byte at most, and the half of a pair it holds. */
	override sizeHint(): SizeHint {
		const u = this.#s.charCodeAt(this.#at);
		const held = u >= 0xdc00 && u <= 0xdfff ? 1 : 0;
		const bytes = utf8Len(this.#s, this.#at + held, this.#s.length);
		return [Math.ceil(bytes / 3) + held, bytes + held];
	}

	override clone(): Iter<u16> {
		const it = new EncodeUtf16(this.#s);
		it.#at = this.#at;
		return it;
	}
}

/** `Debug` of a `CharIndices`: the bytes before its front, and its `Chars`. */
export function fmtCharIndices(it: Iter<unknown>, f: Formatter): FmtResult {
	const c = it as CharIndices;
	return fmtDebugStructFields(f, "CharIndices", ["front_offset", "iter"], [(f) => usizeDebug(c.front as usize, f), (f) => fmtChars(c.chars, f)]);
}

/** `str::char_indices`. */
export function strCharIndices(s: string): Iter<[usize, char]> {
	return new CharIndices(new Chars(s));
}

class CharIndices extends Iter<[usize, char]> {
	readonly chars: Chars;
	/** The bytes before the front. */
	front = 0;
	// The bytes before the back, once it first moves.
	#back = -1;

	constructor(chars: Chars) {
		super();
		this.chars = chars;
	}

	next(): [usize, char] | Done {
		const c = this.chars.next();
		if (c === DONE) return DONE;
		const at = this.front;
		this.front += charLen(c);
		return [at as usize, c];
	}

	override nextBack(): [usize, char] | Done {
		if (this.#back < 0) this.#back = this.front + stringLen(this.chars.rest());
		const c = this.chars.nextBack();
		if (c === DONE) return DONE;
		this.#back -= charLen(c);
		return [this.#back as usize, c];
	}

	override sizeHint(): SizeHint {
		return this.chars.sizeHint();
	}

	override clone(): Iter<[usize, char]> {
		const it = new CharIndices(this.chars.clone() as Chars);
		it.front = this.front;
		it.#back = this.#back;
		return it;
	}
}

/**
 * std's searcher of a pattern in `s` between `front` and `back`, in code units. Each step
 * finds the next match from one end, `start` to `end`, and moves that end past it. A
 * `&str` pattern searches from one end only, as std's can. It drops its pattern once, as
 * what holds it may drop it before dropping that.
 */
class Searcher implements Disposable {
	readonly s: string;
	readonly #p: Pattern;
	#dropped = false;
	#front = 0;
	#back: number;
	// An empty pattern matches before each char and after the last, as std's does, from
	// whichever end it searches.
	#matched = false;
	#matchedBack = false;
	start = 0;
	end = 0;

	constructor(s: string, p: Pattern) {
		this.s = s;
		this.#p = p;
		this.#back = s.length;
	}

	/** std's: where it is, with its pattern cloned by `h.f`, a closure's as Rust clones it. */
	clone(h: Clones | undefined): Searcher {
		const m = new Searcher(this.s, h?.f === undefined ? this.#p : (h.f as (p: Pattern) => Pattern)(this.#p));
		m.#front = this.#front;
		m.#back = this.#back;
		m.#matched = this.#matched;
		m.#matchedBack = this.#matchedBack;
		m.start = this.start;
		m.end = this.end;
		return m;
	}

	nextMatch(): boolean {
		const { s } = this;
		const p = this.#p;
		if (p === "") {
			if (this.#matched) {
				if (this.#front >= this.#back) return false;
				this.#front += (s.codePointAt(this.#front) as number) > 0xffff ? 2 : 1;
			}
			this.#matched = true;
			this.start = this.end = this.#front;
			return true;
		}
		if (typeof p === "string") {
			const i = s.indexOf(p, this.#front);
			if (i < 0 || i + p.length > this.#back) {
				this.#front = this.#back;
				return false;
			}
			this.start = i;
			this.end = this.#front = i + p.length;
			return true;
		}
		while (this.#front < this.#back) {
			const i = this.#front;
			const c = s.codePointAt(i) as number;
			this.#front += c > 0xffff ? 2 : 1;
			if (p(c as char)) {
				this.start = i;
				this.end = this.#front;
				return true;
			}
		}
		return false;
	}

	nextMatchBack(): boolean {
		const { s } = this;
		const p = this.#p;
		if (p === "") {
			if (this.#matchedBack) {
				if (this.#back <= this.#front) return false;
				this.#back = before(s, this.#back);
			}
			this.#matchedBack = true;
			this.start = this.end = this.#back;
			return true;
		}
		if (typeof p === "string") {
			const from = this.#back - p.length;
			const i = from < this.#front ? -1 : s.lastIndexOf(p, from);
			if (i < this.#front) {
				this.#back = this.#front;
				return false;
			}
			this.start = this.#back = i;
			this.end = i + p.length;
			return true;
		}
		while (this.#back > this.#front) {
			const end = this.#back;
			this.#back = before(s, end);
			if (p(s.codePointAt(this.#back) as char)) {
				this.start = this.#back;
				this.end = end;
				return true;
			}
		}
		return false;
	}

	[Symbol.dispose](): void {
		if (this.#dropped) return;
		this.#dropped = true;
		dispose(this.#p);
	}
}

/** `str::split`. */
export function strSplit(s: string, p: Pattern): Iter<string> {
	return new Split(new Searcher(s, p), true);
}

/** `str::rsplit`: `split` from the back. */
export function strRsplit(s: string, p: Pattern): Iter<string> {
	return iterRev(strSplit(s, p));
}

/** `str::split_terminator`: `split`, without the empty part after a last match. */
export function strSplitTerminator(s: string, p: Pattern): Iter<string> {
	return new Split(new Searcher(s, p), false);
}

/** `str::rsplit_terminator`: `split_terminator` from the back. */
export function strRsplitTerminator(s: string, p: Pattern): Iter<string> {
	return iterRev(strSplitTerminator(s, p));
}

/** `str::splitn`: `split`, whose `n`th part is the rest. */
export function strSplitn(s: string, n: usize, p: Pattern): Iter<string> {
	return new Split(new Searcher(s, p), true, false, n);
}

/** `str::rsplitn`: `splitn` from the back. */
export function strRsplitn(s: string, n: usize, p: Pattern): Iter<string> {
	return iterRev(strSplitn(s, n, p));
}

/** `str::split_inclusive`: `split_terminator`, each part ending with the match after it. */
export function strSplitInclusive(s: string, p: Pattern): Iter<string> {
	return new Split(new Searcher(s, p), false, true);
}

/** `str::lines`: the parts ending with `\n`, without it or a `\r` before it. */
export function strLines(s: string): Iter<string> {
	return iterMap(strSplitInclusive(s, "\n"), (line) => {
		if (!line.endsWith("\n")) return line;
		return line.slice(0, line.endsWith("\r\n") ? -2 : -1);
	});
}

/** `str::split_whitespace`: the parts between Unicode whitespace, but empty ones. */
export function strSplitWhitespace(s: string): Iter<string> {
	return iterFilter(strSplit(s, charIsWhitespace), (part) => part !== "");
}

/** `char::is_whitespace`: Unicode's `White_Space`. */
export function charIsWhitespace(c: char): boolean {
	if (c <= 0x20) return c === 0x20 || (c >= 0x09 && c <= 0x0d);
	if (c < 0x85) return false;
	return c === 0x85 || c === 0xa0 || c === 0x1680 || (c >= 0x2000 && c <= 0x200a) || c === 0x2028 || c === 0x2029 || c === 0x202f || c === 0x205f || c === 0x3000;
}

/** std's `SplitInternal`, and with a count, `SplitNInternal`. */
class Split extends Iter<string> implements Disposable {
	readonly #m: Searcher;
	#start = 0;
	#end: number;
	// Whether an empty last part is yielded, which `nextBack` sets once it has passed it.
	#trailing: boolean;
	// Whether a part keeps the match that ends it.
	readonly #inclusive: boolean;
	// The parts left before the rest is the last, as `splitn` counts them.
	#count: number;
	#finished = false;

	constructor(m: Searcher, trailing: boolean, inclusive = false, count = Infinity) {
		super();
		this.#m = m;
		this.#end = m.s.length;
		this.#trailing = trailing;
		this.#inclusive = inclusive;
		this.#count = count;
	}

	override clone(h?: Clones): Iter<string> {
		const it = new Split(this.#m.clone(h), this.#trailing, this.#inclusive, this.#count);
		it.#start = this.#start;
		it.#end = this.#end;
		it.#finished = this.#finished;
		return it;
	}

	next(): string | Done {
		if (this.#count === 0) return DONE;
		if (--this.#count === 0) return this.#rest();
		if (this.#finished) return DONE;
		const m = this.#m;
		if (!m.nextMatch()) return this.#rest();
		const part = m.s.slice(this.#start, this.#inclusive ? m.end : m.start);
		this.#start = m.end;
		return part;
	}

	override nextBack(): string | Done {
		if (this.#count === 0) return DONE;
		if (--this.#count === 0) return this.#rest();
		return this.#back();
	}

	#back(): string | Done {
		if (this.#finished) return DONE;
		if (!this.#trailing) {
			// The last part, unless it is empty, as std's does.
			this.#trailing = true;
			const part = this.#back();
			if (part !== DONE && part !== "") return part;
			if (this.#finished) return DONE;
		}
		const m = this.#m;
		if (m.nextMatchBack()) {
			const part = m.s.slice(m.end, this.#end);
			this.#end = this.#inclusive ? m.end : m.start;
			return part;
		}
		this.#finished = true;
		return m.s.slice(this.#start, this.#end);
	}

	/** std's `get_end`: what is left, once, unless it is an empty last part `trailing` drops. */
	#rest(): string | Done {
		if (this.#finished) return DONE;
		this.#finished = true;
		return this.#trailing || this.#start < this.#end ? this.#m.s.slice(this.#start, this.#end) : DONE;
	}

	[Symbol.dispose](): void {
		dispose(this.#m);
	}
}

/** `str::matches`. */
export function strMatches(s: string, p: Pattern): Iter<string> {
	return new Matches(new Searcher(s, p));
}

/** `str::rmatches`: `matches` from the back. */
export function strRmatches(s: string, p: Pattern): Iter<string> {
	return iterRev(strMatches(s, p));
}

class Matches extends Iter<string> implements Disposable {
	readonly #m: Searcher;

	constructor(m: Searcher) {
		super();
		this.#m = m;
	}

	override clone(h?: Clones): Iter<string> {
		return new Matches(this.#m.clone(h));
	}

	next(): string | Done {
		const m = this.#m;
		return m.nextMatch() ? m.s.slice(m.start, m.end) : DONE;
	}

	override nextBack(): string | Done {
		const m = this.#m;
		return m.nextMatchBack() ? m.s.slice(m.start, m.end) : DONE;
	}

	[Symbol.dispose](): void {
		dispose(this.#m);
	}
}

/** `str::match_indices`: each match, after its offset in UTF-8 bytes. */
export function strMatchIndices(s: string, p: Pattern): Iter<[usize, string]> {
	return new MatchIndices(new Searcher(s, p));
}

/** `str::rmatch_indices`: `match_indices` from the back. */
export function strRmatchIndices(s: string, p: Pattern): Iter<[usize, string]> {
	return iterRev(strMatchIndices(s, p));
}

class MatchIndices extends Iter<[usize, string]> implements Disposable {
	readonly #m: Searcher;
	// Where the last match from each end starts, in code units and in bytes. The back's
	// bytes are counted once it first moves.
	#front = 0;
	#frontBytes = 0;
	#back = -1;
	#backBytes = 0;

	constructor(m: Searcher) {
		super();
		this.#m = m;
	}

	override clone(h?: Clones): Iter<[usize, string]> {
		const it = new MatchIndices(this.#m.clone(h));
		it.#front = this.#front;
		it.#frontBytes = this.#frontBytes;
		it.#back = this.#back;
		it.#backBytes = this.#backBytes;
		return it;
	}

	next(): [usize, string] | Done {
		const m = this.#m;
		if (!m.nextMatch()) return DONE;
		this.#frontBytes += utf8Len(m.s, this.#front, m.start);
		this.#front = m.start;
		return [this.#frontBytes as usize, m.s.slice(m.start, m.end)];
	}

	override nextBack(): [usize, string] | Done {
		const m = this.#m;
		if (!m.nextMatchBack()) return DONE;
		if (this.#back < 0) {
			this.#back = m.s.length;
			this.#backBytes = this.#frontBytes + utf8Len(m.s, this.#front, this.#back);
		}
		this.#backBytes -= utf8Len(m.s, m.start, this.#back);
		this.#back = m.start;
		return [this.#backBytes as usize, m.s.slice(m.start, m.end)];
	}

	[Symbol.dispose](): void {
		dispose(this.#m);
	}
}

/** `str::contains`. */
export function strContains(s: string, p: Pattern): boolean {
	return typeof p === "string" ? s.includes(p) : new Searcher(s, p).nextMatch();
}

/** `str::starts_with`. */
export function strStartsWith(s: string, p: Pattern): boolean {
	if (typeof p === "string") return s.startsWith(p);
	return s.length > 0 && p(s.codePointAt(0) as char);
}

/** `str::trim_start_matches`, which an empty pattern leaves as it is. */
export function strTrimStartMatches(s: string, p: Pattern): string {
	let i = 0;
	if (typeof p === "string") {
		if (p === "") return s;
		while (s.startsWith(p, i)) i += p.length;
	} else {
		while (i < s.length) {
			const c = s.codePointAt(i) as number;
			if (!p(c as char)) break;
			i += c > 0xffff ? 2 : 1;
		}
	}
	return s.slice(i);
}

/** `str::trim_end_matches`, which an empty pattern leaves as it is. */
export function strTrimEndMatches(s: string, p: Pattern): string {
	let end = s.length;
	if (typeof p === "string") {
		if (p === "") return s;
		while (end >= p.length && s.startsWith(p, end - p.length)) end -= p.length;
	} else {
		while (end > 0) {
			const i = before(s, end);
			if (!p(s.codePointAt(i) as char)) break;
			end = i;
		}
	}
	return s.slice(0, end);
}

/**
 * `str::trim_matches`: without the matches at either end. A predicate is called from the
 * front up to the first char it rejects, then from the back up to that char, as std's is.
 */
export function strTrimMatches(s: string, p: Pattern): string {
	if (typeof p === "string") return strTrimEndMatches(strTrimStartMatches(s, p), p);
	let start = 0;
	while (start < s.length && p(s.codePointAt(start) as char)) start += (s.codePointAt(start) as number) > 0xffff ? 2 : 1;
	// The char it rejected, which the back stops after.
	const floor = start < s.length ? start + ((s.codePointAt(start) as number) > 0xffff ? 2 : 1) : start;
	let end = s.length;
	while (end > floor) {
		const i = before(s, end);
		if (!p(s.codePointAt(i) as char)) break;
		end = i;
	}
	return s.slice(start, end);
}

/** `str::strip_prefix`: what follows a match at the start, which a predicate tests the first char for. */
export function strStripPrefix(s: string, p: Pattern): string | undefined {
	if (typeof p === "string") return s.startsWith(p) ? s.slice(p.length) : undefined;
	if (s.length === 0) return undefined;
	const c = s.codePointAt(0) as number;
	return p(c as char) ? s.slice(c > 0xffff ? 2 : 1) : undefined;
}

/** `str::strip_suffix`: what precedes a match at the end, which a predicate tests the last char for. */
export function strStripSuffix(s: string, p: Pattern): string | undefined {
	if (typeof p === "string") return s.endsWith(p) ? s.slice(0, s.length - p.length) : undefined;
	if (s.length === 0) return undefined;
	const i = before(s, s.length);
	return p(s.codePointAt(i) as char) ? s.slice(0, i) : undefined;
}

/** `str::find`: the first match's offset in UTF-8 bytes. */
export function strFind(s: string, p: Pattern): usize | undefined {
	const m = new Searcher(s, p);
	return m.nextMatch() ? (utf8Len(s, 0, m.start) as usize) : undefined;
}

/** `str::replace`, and `replacen` of the first `n` matches. */
export function strReplace(s: string, p: Pattern, to: string, n = Infinity): string {
	const m = new Searcher(s, p);
	let out = "";
	let last = 0;
	for (let i = 0; i < n && m.nextMatch(); i++) {
		out += s.slice(last, m.start) + to;
		last = m.end;
	}
	return out + s.slice(last);
}

/** `str::rfind`: the last match's offset in UTF-8 bytes. */
export function strRfind(s: string, p: Pattern): usize | undefined {
	const m = new Searcher(s, p);
	return m.nextMatchBack() ? (utf8Len(s, 0, m.start) as usize) : undefined;
}

/** A pattern of `[char; N]` or `&[char]`: whether a char is any of `cs`. */
export function patternChars(cs: ArrayLike<char>): (c: char) => boolean {
	return (c) => {
		for (let i = 0; i < cs.length; i++) if (cs[i] === c) return true;
		return false;
	};
}

/** `str::ends_with`. */
export function strEndsWith(s: string, p: Pattern): boolean {
	if (typeof p === "string") return s.endsWith(p);
	return s.length > 0 && p(s.codePointAt(before(s, s.length)) as char);
}

/** `str::split_once`: the text before the first match and after it. */
export function strSplitOnce(s: string, p: Pattern): [string, string] | undefined {
	const m = new Searcher(s, p);
	return m.nextMatch() ? [s.slice(0, m.start), s.slice(m.end)] : undefined;
}

/** `str::rsplit_once`: the text before the last match and after it. */
export function strRsplitOnce(s: string, p: Pattern): [string, string] | undefined {
	const m = new Searcher(s, p);
	return m.nextMatchBack() ? [s.slice(0, m.start), s.slice(m.end)] : undefined;
}

/** `str::is_char_boundary`: whether byte `i` starts a char, or is the end. */
export function strIsCharBoundary(s: string, i: number): boolean {
	return unitAt(s, 0, 0, i) >= 0;
}

/** `str::bytes`: its UTF-8 bytes. */
export function strBytes(s: string): Iter<u8> {
	return iterSlice(encoder.encode(s));
}

// Lossy and chunked UTF-8. A non-fatal `TextDecoder` writes one U+FFFD for each maximal
// invalid sequence, as WHATWG's decoder and std's both do.

// `ignoreBOM` keeps a leading U+FEFF, as Rust does.
const lossy = /* @__PURE__ */ new TextDecoder("utf-8", { ignoreBOM: true });

/** `String::from_utf8_lossy`: the text, each invalid sequence replaced by U+FFFD. */
export function stringFromUtf8Lossy(bytes: ByteSlice): string {
	return lossy.decode(byteSliceView(bytes));
}

/** `core::str::Utf8Chunk`: valid text, and the invalid bytes after it. */
export type Utf8Chunk = {
	/** `Utf8Chunk::valid`. */
	readonly valid: string;
	/** `Utf8Chunk::invalid`: empty only in the last chunk. */
	readonly invalid: ByteSlice;
};

/** `<[u8]>::utf8_chunks`. */
export function byteSliceUtf8Chunks(bytes: ByteSlice): Iter<Utf8Chunk> {
	return new Utf8Chunks(bytes);
}

class Utf8Chunks extends Iter<Utf8Chunk> {
	#source: ByteSlice;

	constructor(source: ByteSlice) {
		super();
		this.#source = source;
	}

	next(): Utf8Chunk | Done {
		const s = this.#source;
		if (s.length === 0) return DONE;
		const error = utf8Check(byteSliceView(s));
		const valid = error?.validUpTo ?? s.length;
		// A sequence the input ends in the middle of is invalid up to the end.
		const end = error === undefined ? s.length : valid + (error.errorLen ?? s.length - valid);
		this.#source = new ByteSlice(s.data, s.start + end, s.length - end);
		const text = lossy.decode(s.data.subarray(s.start, s.start + valid));
		return { valid: text, invalid: new ByteSlice(s.data, s.start + valid, end - valid) };
	}

	/** What it has left, which std's `Debug` writes. */
	source(): ByteSlice {
		return this.#source;
	}

	override clone(): Iter<Utf8Chunk> {
		return new Utf8Chunks(this.#source);
	}
}

/** `Debug` of a `Utf8Chunks`: what it has left as a string, each invalid byte as `\xNN`. */
export function fmtUtf8Chunks(it: Iter<Utf8Chunk>, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "Utf8Chunks", ["source"], [
		(f) => {
			let out = '"';
			for (const chunk of new Utf8Chunks((it as Utf8Chunks).source())) {
				for (const c of chunk.valid) out += escapeDebug(c, '"');
				for (const b of byteSliceView(chunk.invalid)) out += `\\x${b.toString(16).toUpperCase().padStart(2, "0")}`;
			}
			return f.out.writeStr(`${out}"`);
		},
	]);
}

/** `Debug` of a `Utf8Chunk`, as std derives it. */
export function fmtUtf8ChunkDebug(chunk: Utf8Chunk, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "Utf8Chunk", ["valid", "invalid"], [
		(f) => fmtStr(chunk.valid, f),
		(f) => fmtList(byteSliceView(chunk.invalid), f, u8Debug),
	]);
}

// `String`'s edits, each giving the edited string, which the caller writes where the
// `&mut String` points, and what the edit returns beside it.

/** `isize::MAX` on wasm32, past which std refuses a capacity. */
const ISIZE_MAX = 0x7fffffff;

/**
 * `concat` and `join` of strings by `sep`, which panic as std's do where the bytes they
 * reserve pass `usize::MAX` or what a `String` may hold.
 */
export function strsJoin(parts: readonly string[], sep: string): string {
	const seps = Math.max(parts.length - 1, 0);
	let units = sep.length * seps;
	for (const part of parts) units += part.length;
	// A code unit is one to three bytes, so only a long result needs its bytes counted, each
	// distinct part once, as such a result mostly repeats a few.
	if (units * 3 > ISIZE_MAX) {
		const counted = new Map<string, number>();
		const count = (s: string): number => {
			let n = counted.get(s);
			if (n === undefined) counted.set(s, (n = stringLen(s)));
			return n;
		};
		let len = count(sep) * seps;
		for (const part of parts) len += count(part);
		if (len > 0xffffffff) panic("attempt to join into collection with len > usize::MAX");
		if (len > ISIZE_MAX) panic("capacity overflow");
	}
	return parts.join(sep);
}

/** `String::FromUtf16Error`: `{ kind }`, its private `FromUtf16ErrorKind`'s name. */
export class FromUtf16Error {
	readonly kind: "LoneSurrogate" | "OddBytes";

	constructor(kind: "LoneSurrogate" | "OddBytes") {
		this.kind = kind;
	}
}

/**
 * The text of `len` UTF-16 units, `unit(i)` the `i`th, as `char::decode_utf16` reads them:
 * a surrogate without its pair is U+FFFD if `lossy`, and else leaves no text.
 */
function fromUtf16Units(len: number, unit: (i: number) => number, lossy: boolean): string | undefined {
	let s = "";
	for (let i = 0; i < len; i++) {
		const u = unit(i);
		if (u < 0xd800 || u > 0xdfff) {
			s += String.fromCharCode(u);
			continue;
		}
		const next = i + 1 < len ? unit(i + 1) : 0;
		if (u <= 0xdbff && next >= 0xdc00 && next <= 0xdfff) {
			s += String.fromCharCode(u, next);
			i++;
		} else if (lossy) s += "\ufffd";
		else return undefined;
	}
	return s;
}

/** The text of `len` units, `unit(i)` the `i`th, or a `LoneSurrogate` error, as `String::from_utf16` reads them. */
function fromUtf16(len: number, unit: (i: number) => number): Result<string, FromUtf16Error> {
	const s = fromUtf16Units(len, unit, false);
	return s === undefined ? Err(new FromUtf16Error("LoneSurrogate")) : Ok(s);
}

/** `String::from_utf16`. */
export function stringFromUtf16(v: ArrayLike<u16>): Result<string, FromUtf16Error> {
	return fromUtf16(v.length, (i) => v[i]);
}

/** `String::from_utf16_lossy`: U+FFFD for each surrogate without its pair. */
export function stringFromUtf16Lossy(v: ArrayLike<u16>): string {
	return fromUtf16Units(v.length, (i) => v[i], true) as string;
}

/** The `i`th unit of `v`'s bytes, little-endian unless `big`. */
function byteUnit(v: ByteSlice, big: boolean): (i: number) => number {
	const [hi, lo] = big ? [0, 1] : [1, 0];
	return (i) => (v.data[v.start + 2 * i + hi] << 8) | v.data[v.start + 2 * i + lo];
}

/** `String::from_utf16le`, an `OddBytes` error of an odd number of bytes. */
export function stringFromUtf16le(v: ByteSlice): Result<string, FromUtf16Error> {
	return v.length % 2 === 1 ? Err(new FromUtf16Error("OddBytes")) : fromUtf16(v.length / 2, byteUnit(v, false));
}

/** `String::from_utf16be`, an `OddBytes` error of an odd number of bytes. */
export function stringFromUtf16be(v: ByteSlice): Result<string, FromUtf16Error> {
	return v.length % 2 === 1 ? Err(new FromUtf16Error("OddBytes")) : fromUtf16(v.length / 2, byteUnit(v, true));
}

/** `String::from_utf16le_lossy`, and U+FFFD for an odd byte at the end. */
export function stringFromUtf16leLossy(v: ByteSlice): string {
	return (fromUtf16Units(v.length >>> 1, byteUnit(v, false), true) as string) + (v.length % 2 === 1 ? "\ufffd" : "");
}

/** `String::from_utf16be_lossy`, and U+FFFD for an odd byte at the end. */
export function stringFromUtf16beLossy(v: ByteSlice): string {
	return (fromUtf16Units(v.length >>> 1, byteUnit(v, true), true) as string) + (v.length % 2 === 1 ? "\ufffd" : "");
}

/** `Debug` of `FromUtf16Error`. */
export function fmtFromUtf16ErrorDebug(e: FromUtf16Error, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "FromUtf16Error", ["kind"], [(f) => f.out.writeStr(e.kind)]);
}

/** `Display` of `FromUtf16Error`, padded as a `str`. */
export function fmtFromUtf16ErrorDisplay(e: FromUtf16Error, f: Formatter): FmtResult {
	return fmtPad(f, e.kind === "LoneSurrogate" ? "invalid utf-16: lone surrogate found" : "invalid utf-16: odd number of bytes");
}

/** `String::with_capacity`: the empty string, which keeps no capacity. */
export function stringWithCapacity(capacity: usize): string {
	if (capacity > ISIZE_MAX) panic("capacity overflow");
	return "";
}

/** `String::reserve` and `reserve_exact`, which panic as std's do past `isize::MAX`. */
export function stringReserve(s: string, additional: usize): void {
	if (!stringTryReserve(s, additional).ok) panic("capacity overflow");
}

/** `String::try_reserve` and `try_reserve_exact`: the error where `reserve` panics. */
export function stringTryReserve(s: string, additional: usize): Result<undefined, TryReserveError> {
	// A code unit is at most three bytes, so only a large `additional` needs the length.
	const over = additional > ISIZE_MAX - 3 * s.length && additional > ISIZE_MAX - utf8Len(s, 0, s.length);
	return over ? CAPACITY_OVERFLOW : OK;
}

/** The code unit where byte `i` of `s` starts a char, or panic with `message`. */
function boundary(s: string, i: number, message: string): number {
	const at = unitAt(s, 0, 0, i);
	if (at < 0) panic(message);
	return at;
}

/** `String::truncate`: `s` to `len` bytes, or as it is if no longer. */
export function stringTruncate(s: string, len: usize): string {
	const at = unitAt(s, 0, 0, len);
	if (at >= 0) return s.slice(0, at);
	if (len > utf8Len(s, 0, s.length)) return s;
	panic("assertion failed: self.is_char_boundary(new_len)");
}

/** `String::insert_str`, and `insert` of a char's text. */
export function stringInsertStr(s: string, i: usize, text: string): string {
	const at = boundary(s, i, "assertion failed: self.is_char_boundary(idx)");
	return s.slice(0, at) + text + s.slice(at);
}

/** `String::retain`: the chars `f` keeps, calling it on each in order. */
export function stringRetain(s: string, f: (c: char) => boolean): string {
	let kept = "";
	for (const ch of s) if (f(ch.codePointAt(0) as char)) kept += ch;
	return kept;
}

/** `String::replace_range`. */
export function stringReplaceRange(s: string, r: Bounds<number>, text: string): string {
	const [start, end] = boundsRange(r, utf8Len(s, 0, s.length));
	const from = boundary(s, start, "start of range should be a character boundary");
	const to = boundary(s, end, "end of range should be a character boundary");
	return s.slice(0, from) + text + s.slice(to);
}

/** `String::extend_from_within`. */
export function stringExtendFromWithin(s: string, r: Bounds<number>): string {
	const [start, end] = boundsRange(r, utf8Len(s, 0, s.length));
	const from = boundary(s, start, "assertion failed: self.is_char_boundary(start)");
	const to = boundary(s, end, "assertion failed: self.is_char_boundary(end)");
	return s + s.slice(from, to);
}

/** `String::pop`: the rest, and the last char if any. */
export function stringPop(s: string): [string, char | undefined] {
	if (s.length === 0) return [s, undefined];
	const at = before(s, s.length);
	return [s.slice(0, at), s.codePointAt(at) as char];
}

/** `String::remove`: the rest, and the char at byte `i`. */
export function stringRemove(s: string, i: usize): [string, char] {
	const at = unitAt(s, 0, 0, i);
	if (at < 0) sliceFailed(s, i, utf8Len(s, 0, s.length));
	if (at === s.length) panic("cannot remove a char from the end of a string");
	const c = s.codePointAt(at) as number;
	return [s.slice(0, at) + s.slice(at + (c > 0xffff ? 2 : 1)), c as char];
}

/** `String::split_off`: what comes before byte `i`, and after it. */
export function stringSplitOff(s: string, i: usize): [string, string] {
	const at = boundary(s, i, "assertion failed: self.is_char_boundary(at)");
	return [s.slice(0, at), s.slice(at)];
}

// `String` from and into iterators. Each reads the iterator by `for_each`, as std's does,
// which `drops` makes its own `fold`.

/**
 * `Extend<char>` and `Extend<&char>` of `s`, and `collect` of chars from `""`: `s` and each
 * char, once it reserved the iterator's lower bound, which panics past `isize::MAX`.
 */
export function stringExtendChars(s: string, it: Iter<char>, drops = false): string {
	stringReserve(s, it.sizeHint()[0]);
	iterForEach(
		it,
		(c) => {
			s += String.fromCodePoint(c);
		},
		drops,
	);
	return s;
}

/** `String` as a tuple's part, of chars, which reserves as `stringReserve`. */
export const STRING_CHARS_PART: Part<string, char> = {
	reserve: stringReserve,
	one: (s, c) => s + String.fromCodePoint(c),
};

/** `String` as a tuple's part, of strings, which never reserves. */
export const STRING_PART: Part<string, string> = { one: (s, text) => s + text };

/** `Extend` of `s` by strings, `&str`, `String`, or `Cow<str>`, and `collect` of `&str`s from `""`. */
export function stringExtend(s: string, it: Iter<string>, drops = false): string {
	iterForEach(
		it,
		(text) => {
			s += text;
		},
		drops,
	);
	return s;
}

/** `collect` of owned strings, a `String` or `Cow<str>`: the first by `next`, then the rest extend it. */
export function stringFromStrings(it: Iter<string>, drops = false): string {
	const first = it.next();
	if (first !== DONE) return stringExtend(first, it, drops);
	if (drops) dispose(it);
	return "";
}

/**
 * `String::drain`: the chars within the byte range `r` of the string at `s`, which leaves
 * the string without them as it drops, and as it was if it never does, as `mem::forget`
 * leaves it.
 */
export function stringDrain(s: Place<string>, r: Bounds<number>): Iter<char> {
	const text = s.v;
	const [start, end] = boundsRange(r, utf8Len(text, 0, text.length));
	const from = boundary(text, start, "assertion failed: self.is_char_boundary(start)");
	const to = boundary(text, end, "assertion failed: self.is_char_boundary(end)");
	return new StringDrain(s, text, from, to);
}

/** `Drain::as_str`: the chars `drain` has yet to yield. */
export function stringDrainAsStr(drain: Iter<char>): string {
	return (drain as Chars).rest();
}

/** `Debug` of a `String`'s `Drain`: the chars it has yet to yield. */
export function fmtStringDrain(drain: Iter<unknown>, f: Formatter): FmtResult {
	return fmtDebugTupleFields(f, "Drain", [(f) => fmtStr((drain as Chars).rest(), f)]);
}

class StringDrain extends Chars implements Disposable {
	readonly #s: Place<string>;
	readonly #text: string;
	readonly #from: number;
	readonly #to: number;

	constructor(s: Place<string>, text: string, from: number, to: number) {
		super(text.slice(from, to));
		this.#s = s;
		this.#text = text;
		this.#from = from;
		this.#to = to;
	}

	[Symbol.dispose](): void {
		this.#s.v = this.#text.slice(0, this.#from) + this.#text.slice(this.#to);
	}
}
