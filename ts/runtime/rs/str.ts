// `str`'s slices, iterators, and searches, for rust2-ts output. `rs.ts` re-exports all of
// it.
//
// A JS string is UTF-16 where a `str` is UTF-8: these step by code point, and an offset
// they take or return counts UTF-8 bytes, as Rust's does. A string from translated code is
// well formed, so a pattern's text matches where its UTF-8 would, never across a
// surrogate pair.

import {
	Borrowed,
	ByteSlice,
	capReserve,
	capReserveExact,
	capShrink,
	capTryReserve,
	capTryReserveExact,
	type Slice,
	byteSliceView,
	Err,
	type Fmt,
	type FmtResult,
	type Formatter,
	fmtBool,
	fmtCharDebug,
	fmtChars,
	fmtDebugStructFields,
	fmtDebugTupleFields,
	fmtList,
	fmtOwned,
	fmtPad,
	fmtStr,
	OK,
	Ok,
	type Place,
	panic,
	panicFmt,
	panicSizes,
	type Result,
	type StrCow,
	utf8Check,
} from "../rs.ts";
import { CAPACITY_OVERFLOW, type TryReserveError } from "./alloc.ts";
import { isAsciiWhitespace } from "./ascii.ts";
import { escapeDebug } from "./char.ts";
import { type Bounds, boundsRange, type Part } from "./collections.ts";
import { type char, type u8, type u16, type usize, u8Debug, u8UpperHex, u64Debug, u64Of, usizeDebug } from "./int.ts";
import { type Clones, DONE, type Done, Iter, iterFilter, iterForEach, iterHeld, iterMap, iterRev, iterSlice, type RangeInclusive, type SizeHint } from "./iter.ts";
import { drop as dispose } from "./task.ts";
import type { Location } from "./location.ts";

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
export function strSlice(s: string, start: number, end?: number, at?: Location): string {
	return s.slice(...units(s, start, end, at));
}

/** `str::get` by a range's UTF-8 byte offsets, with `end` defaulting to the length. */
export function strGet(s: string, start: number, end?: number): string | undefined {
	const pos = unitsChecked(s, start, end);
	return pos && s.slice(...pos);
}

/**
 * The byte offsets `&s[r]` reads for an `a..=b` or a `..=b`, panicking as std's does where
 * `b` is not before the end; the part is checked as `a..b + 1`'s is.
 */
export function strInclusive(s: string, r: RangeInclusive<usize> | { readonly end: usize; readonly inclusive: true }, at?: Location): [number, number] {
	const [lo, hi] = "lo" in r ? [r.lo, r.hi] : [0, r.end];
	if (hi >= stringLen(s)) sliceFailed(s, lo, hi, at);
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
function units(s: string, start: number, end?: number, at?: Location): [number, number] {
	return unitsChecked(s, start, end) ?? sliceFailed(s, start, end ?? stringLen(s), at);
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
export function strPart(p: Place<string>, start: number, end?: number, at?: Location): Place<string> {
	return new StrPart(p, units(p.v, start, end, at));
}

/** `str::get_mut` by a range's UTF-8 byte offsets, with `end` defaulting to the length. */
export function strGetMut(p: Place<string>, start: number, end?: number): Place<string> | undefined {
	const pos = unitsChecked(p.v, start, end);
	return pos && new StrPart(p, pos);
}

/** `str::split_at_mut`. */
export function strSplitAtMut(p: Place<string>, mid: number, at?: Location): [Place<string>, Place<string>] {
	const [, pos] = units(p.v, 0, mid, at);
	return [new StrPart(p, [0, pos]), new StrPart(p, [pos, p.v.length])];
}

/** `str::split_at_mut_checked`. */
export function strSplitAtMutChecked(p: Place<string>, mid: number): [Place<string>, Place<string>] | undefined {
	const pos = unitsChecked(p.v, 0, mid);
	return pos && [new StrPart(p, pos), new StrPart(p, [pos[1], p.v.length])];
}

/**
 * std's panic for a slice of `s` that is out of bounds, backwards, or inside a char, or that
 * ends inclusively at the length.
 */
function sliceFailed(s: string, start: number, end: number, at?: Location): never {
	const len = stringLen(s);
	const outOfBounds = (): never => panicSizes(["end byte index ", end, " is out of bounds for string of length ", len], at);
	if (start > len) panicSizes(["start byte index ", start, " is out of bounds for string of length ", len], at);
	if (end > len) outOfBounds();
	if (start > end) panicSizes(["byte range starts at ", start, " but ends at ", end], at);
	// Neither inside a char: an inclusive end at the length, which is past the last byte.
	const [which, pos] =
		unitAt(s, 0, 0, start) < 0
			? ["start", start]
			: unitAt(s, 0, 0, end) < 0
				? ["end", end]
				: outOfBounds();
	// The char `at` is inside, and where its bytes start.
	let floor = 0;
	let c = 0;
	for (const ch of s) {
		c = ch.codePointAt(0) as number;
		if (floor + charLen(c) > pos) break;
		floor += charLen(c);
	}
	const size = (n: number): Fmt => fmtOwned(n as usize, usizeDebug);
	panicFmt([`${which} byte index `, size(pos), " is not a char boundary; it is inside ", fmtOwned(c as char, fmtCharDebug), " (bytes ", size(floor), "..", size(floor + charLen(c)), " of string)"], at);
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
		const pos = this.front;
		this.front += charLen(c);
		return [pos as usize, c];
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
	// whichever end it searches: 0 before its first match there, 1 after it, and 2 once it
	// has found no more.
	#matched = 0;
	#matchedBack = 0;
	start = 0;
	end = 0;

	/** For `Debug`: its pattern, where it is from each end, and an empty pattern's state from each. */
	static state(m: Searcher): [p: Pattern, front: number, back: number, matched: number, matchedBack: number] {
		return [m.#p, m.#front, m.#back, m.#matched, m.#matchedBack];
	}

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
				if (this.#front >= this.#back) {
					this.#matched = 2;
					return false;
				}
				this.#front += (s.codePointAt(this.#front) as number) > 0xffff ? 2 : 1;
			}
			this.#matched = 1;
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
				if (this.#back <= this.#front) {
					this.#matchedBack = 2;
					return false;
				}
				this.#back = before(s, this.#back);
			}
			this.#matchedBack = 1;
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

/** `str::split_ascii_whitespace`: the parts between ASCII whitespace, but empty ones. */
export function strSplitAsciiWhitespace(s: string): Iter<string> {
	return new SplitAsciiWhitespace(new Split(new Searcher(s, isAsciiWhitespace), true));
}

/** std's `SplitAsciiWhitespace`: a slice's `Split` of the bytes, without its empty parts. */
class SplitAsciiWhitespace extends Iter<string> {
	readonly split: Split;

	constructor(split: Split) {
		super();
		this.split = split;
	}

	next(): string | Done {
		for (;;) {
			const part = this.split.next();
			if (part !== "") return part;
		}
	}

	override nextBack(): string | Done {
		for (;;) {
			const part = this.split.nextBack();
			if (part !== "") return part;
		}
	}

	/** std's slice `Split`'s: up to a part per byte left and one more, which a filter keeps as its upper bound. */
	override sizeHint(): SizeHint {
		const [start, end, , finished] = Split.state(this.split);
		return [0, finished ? 0 : utf8Len(this.split.m.s, start, end) + 1];
	}

	override clone(): Iter<string> {
		return new SplitAsciiWhitespace(this.split.clone() as Split);
	}
}

/** `char::is_whitespace`: Unicode's `White_Space`. */
export function charIsWhitespace(c: char): boolean {
	if (c <= 0x20) return c === 0x20 || (c >= 0x09 && c <= 0x0d);
	if (c < 0x85) return false;
	return c === 0x85 || c === 0xa0 || c === 0x1680 || (c >= 0x2000 && c <= 0x200a) || c === 0x2028 || c === 0x2029 || c === 0x202f || c === 0x205f || c === 0x3000;
}

/** std's `SplitInternal`, and with a count, `SplitNInternal`. */
class Split extends Iter<string> implements Disposable {
	readonly m: Searcher;
	#start = 0;
	#end: number;
	// Whether an empty last part is yielded, which `nextBack` sets once it has passed it.
	#trailing: boolean;
	// Whether a part keeps the match that ends it.
	readonly #inclusive: boolean;
	// The parts left before the rest is the last, as `splitn` counts them.
	#count: number;
	#finished = false;

	/** For `Debug`: where the parts left start and end, its flags, and its count. */
	static state(it: Split): [start: number, end: number, trailing: boolean, finished: boolean, count: number] {
		return [it.#start, it.#end, it.#trailing, it.#finished, it.#count];
	}

	constructor(m: Searcher, trailing: boolean, inclusive = false, count = Infinity) {
		super();
		this.m = m;
		this.#end = m.s.length;
		this.#trailing = trailing;
		this.#inclusive = inclusive;
		this.#count = count;
	}

	override clone(h?: Clones): Iter<string> {
		const it = new Split(this.m.clone(h), this.#trailing, this.#inclusive, this.#count);
		it.#start = this.#start;
		it.#end = this.#end;
		it.#finished = this.#finished;
		return it;
	}

	next(): string | Done {
		if (this.#count === 0) return DONE;
		if (--this.#count === 0) return this.#rest();
		if (this.#finished) return DONE;
		const m = this.m;
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
		const m = this.m;
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
		return this.#trailing || this.#start < this.#end ? this.m.s.slice(this.#start, this.#end) : DONE;
	}

	[Symbol.dispose](): void {
		dispose(this.m);
	}
}

/**
 * `Debug` of a `str` search, `name`, std's struct of it: a `Split`, one of its forms, a
 * `Matches`, a `MatchIndices`, or one of those from the back, by `searcher`, the name of
 * std's searcher of its pattern; or a `Lines`, a `SplitWhitespace`, or a
 * `SplitAsciiWhitespace`. Each writes std's state in UTF-8 bytes, from where it is.
 */
export function fmtStrSearch(it: Iter<unknown>, f: Formatter, name: string, searcher = "CharSearcher"): FmtResult {
	let held = iterHeld<Iter<unknown>>(it);
	// One from the back is `iterRev` of one from the front, `lines` a map of a split and
	// `split_whitespace` a filter of one.
	if (name.startsWith("R") || name === "Lines" || name === "SplitWhitespace") held = iterHeld((held as unknown as { inner: Iter<unknown> }).inner);
	if (name === "SplitAsciiWhitespace") {
		const { split } = held as SplitAsciiWhitespace;
		const [start, end, , finished] = Split.state(split);
		const bytes = encoder.encode(split.m.s.slice(start, end));
		const slice: Fmt = (f) => fmtDebugStructFields(f, "Split", ["v", "finished"], [(f) => fmtList(bytes as Iterable<u8>, f, u8Debug), (f) => fmtBool(finished, f)]);
		const filter: Fmt = (f) => fmtDebugStructFields(f, "Filter", ["iter"], [slice]);
		return fmtDebugStructFields(f, name, ["inner"], [(f) => fmtDebugStructFields(f, "Map", ["iter"], [filter])]);
	}
	if (name === "Lines") {
		const split: Fmt = (f) => fmtDebugStructFields(f, "SplitInclusive", ["0"], [(f) => fmtSplitInternal(held as Split, f, searcher)]);
		return fmtDebugTupleFields(f, name, [(f) => fmtDebugStructFields(f, "Map", ["iter"], [split])]);
	}
	if (name === "SplitWhitespace") {
		const split: Fmt = (f) => fmtDebugTupleFields(f, "Split", [(f) => fmtSplitInternal(held as Split, f, "CharPredicateSearcher")]);
		return fmtDebugStructFields(f, name, ["inner"], [(f) => fmtDebugStructFields(f, "Filter", ["iter"], [split])]);
	}
	const inner: Fmt = (f) => {
		switch (name.replace(/^R/, "")) {
			case "SplitN": {
				const split = held as Split;
				return fmtDebugStructFields(f, "SplitNInternal", ["iter", "count"], [(f) => fmtSplitInternal(split, f, searcher), (f) => usizeDebug(Split.state(split)[4] as usize, f)]);
			}
			case "Matches":
				return fmtDebugTupleFields(f, "MatchesInternal", [(f) => fmtSearcher((held as Matches).m, f, searcher)]);
			case "MatchIndices":
				return fmtDebugTupleFields(f, "MatchIndicesInternal", [(f) => fmtSearcher((held as MatchIndices).m, f, searcher)]);
			default:
				return fmtSplitInternal(held as Split, f, searcher);
		}
	};
	return name === "SplitInclusive" ? fmtDebugStructFields(f, name, ["0"], [inner]) : fmtDebugTupleFields(f, name, [inner]);
}

/** `Debug` of std's `SplitInternal`: where the parts left start and end, its searcher, and its flags. */
function fmtSplitInternal(split: Split, f: Formatter, searcher: string): FmtResult {
	const { s } = split.m;
	const [start, end, trailing, finished] = Split.state(split);
	return fmtDebugStructFields(
		f,
		"SplitInternal",
		["start", "end", "matcher", "allow_trailing_empty", "finished"],
		[
			(f) => usizeDebug(utf8Len(s, 0, start) as usize, f),
			(f) => usizeDebug(utf8Len(s, 0, end) as usize, f),
			(f) => fmtSearcher(split.m, f, searcher),
			(f) => fmtBool(trailing, f),
			(f) => fmtBool(finished, f),
		],
	);
}

/**
 * `Debug` of std's searcher, `name`, of `m`'s pattern: each keeps where it is from each end
 * in UTF-8 bytes, a `char`'s its UTF-8, a `&str`'s its byte or its two-way search, and the
 * others the chars left between the ends.
 */
function fmtSearcher(m: Searcher, f: Formatter, name: string): FmtResult {
	const { s } = m;
	const [p, front, back, matched, matchedBack] = Searcher.state(m);
	const haystack: Fmt = (f) => fmtStr(s, f);
	const at = (i: number): Fmt => (f) => usizeDebug(utf8Len(s, 0, i) as usize, f);
	if (name === "CharSearcher") {
		const c = (p as string).codePointAt(0) as char;
		const utf8 = new Uint8Array(4);
		encoder.encodeInto(p as string, utf8);
		const names = ["haystack", "finger", "finger_back", "needle", "utf8_size", "utf8_encoded"];
		const fields: Fmt[] = [haystack, at(front), at(back), (f) => fmtCharDebug(c, f), (f) => u8Debug(charLen(c) as u8, f), (f) => fmtList(utf8 as Iterable<u8>, f, u8Debug)];
		return fmtDebugStructFields(f, name, names, fields);
	}
	if (name === "StrSearcher") {
		const needle = p as string;
		let searcher: Fmt;
		if (needle === "") {
			const names = ["position", "end", "is_match_fw", "is_match_bw", "is_finished"];
			const flags = [matched !== 1, matchedBack !== 1, matched === 2 || matchedBack === 2];
			const empty: Fmt = (f) => fmtDebugStructFields(f, "EmptyNeedle", names, [at(front), at(back), ...flags.map((b): Fmt => (f) => fmtBool(b, f))]);
			searcher = (f) => fmtDebugTupleFields(f, "Empty", [empty]);
		} else if (needle.length === 1 && needle.charCodeAt(0) < 0x80) {
			const byte: Fmt = (f) => fmtDebugStructFields(f, "ByteNeedle", ["b", "position", "end"], [(f) => u8Debug(needle.charCodeAt(0) as u8, f), at(front), at(back)]);
			searcher = (f) => fmtDebugTupleFields(f, "Byte", [byte]);
		} else {
			const w = twoWay(m);
			const names = ["crit_pos", "crit_pos_back", "period", "byteset", "position", "end", "memory", "memory_back"];
			const sizes = [w.critPos, w.critPosBack, w.period].map((n): Fmt => (f) => usizeDebug(n as usize, f));
			const byteset: Fmt = (f) => u64Debug(u64Of((BigInt(w.byteset[1] >>> 0) << 32n) | BigInt(w.byteset[0] >>> 0)), f);
			const state = [w.position, w.end, w.memory, w.memoryBack].map((n): Fmt => (f) => usizeDebug(n as usize, f));
			const two: Fmt = (f) => fmtDebugStructFields(f, "TwoWaySearcher", names, [...sizes, byteset, ...state]);
			searcher = (f) => fmtDebugTupleFields(f, "TwoWay", [two]);
		}
		return fmtDebugStructFields(f, name, ["haystack", "needle", "searcher"], [haystack, (f) => fmtStr(needle, f), searcher]);
	}
	// A `MultiCharEqSearcher`'s chars left, which std's `CharIndices` keeps from the front.
	const indices: Fmt = (f) => fmtDebugStructFields(f, "CharIndices", ["front_offset", "iter"], [at(front), (f) => fmtChars(new Chars(s.slice(front, back)), f)]);
	if (name === "CharPredicateSearcher") return fmtDebugStructFields(f, name, ["haystack", "char_indices"], [haystack, indices]);
	const chars = Array.from((p as CharSet).chars);
	const multi: Fmt = (f) => fmtDebugStructFields(f, "MultiCharEqSearcher", ["char_eq", "haystack", "char_indices"], [(f) => fmtList(chars, f, fmtCharDebug), haystack, indices]);
	return fmtDebugTupleFields(f, name, [multi]);
}

/** usize::MAX on wasm32, std's `memory` of a needle whose period is long. */
const LONG = 0xffffffff;

/**
 * std's `TwoWaySearcher` of `m`'s `&str` pattern, where it is: from an end it has searched
 * to its limit, where a search that found nothing leaves it, its memory of that search,
 * which a replay of each from the start finds; otherwise just past the last match.
 */
function twoWay(m: Searcher): TwoWay {
	const [p, front, back] = Searcher.state(m);
	const h = encoder.encode(m.s);
	const n = encoder.encode(p as string);
	const w = new TwoWay(n, h.length);
	if (front === m.s.length) while (w.next(h, n));
	else w.position = utf8Len(m.s, 0, front);
	if (back === 0) while (w.nextBack(h, n));
	else w.end = utf8Len(m.s, 0, back);
	return w;
}

/** std's two-way search of a needle's UTF-8, with its fields, for `Debug` to replay. */
class TwoWay {
	readonly critPos: number;
	readonly critPosBack: number;
	readonly period: number;
	/** Each bit of `byte & 63` the needle holds, in two halves of 32 bits. */
	readonly byteset: [number, number] = [0, 0];
	position = 0;
	end: number;
	memory: number;
	memoryBack: number;

	constructor(n: Uint8Array, end: number) {
		const [critFalse, periodFalse] = maximalSuffix(n, false);
		const [critTrue, periodTrue] = maximalSuffix(n, true);
		const [critPos, period] = critFalse > critTrue ? [critFalse, periodFalse] : [critTrue, periodTrue];
		this.critPos = critPos;
		this.end = end;
		let short = period + critPos <= n.length;
		for (let i = 0; short && i < critPos; i++) short = n[i] === n[period + i];
		if (short) {
			this.critPosBack = n.length - Math.max(reverseMaximalSuffix(n, period, false), reverseMaximalSuffix(n, period, true));
			this.period = period;
			this.#bytes(n.subarray(0, period));
			this.memory = 0;
			this.memoryBack = n.length;
		} else {
			this.critPosBack = critPos;
			this.period = Math.max(critPos, n.length - critPos) + 1;
			this.#bytes(n);
			this.memory = this.memoryBack = LONG;
		}
	}

	#bytes(bytes: Uint8Array): void {
		for (const b of bytes) this.byteset[(b & 0x3f) >> 5] |= 1 << (b & 0x1f);
	}

	#contains(b: number): boolean {
		return ((this.byteset[(b & 0x3f) >> 5] >>> (b & 0x1f)) & 1) !== 0;
	}

	/** std's `next`, whether it matched. */
	next(h: Uint8Array, n: Uint8Array): boolean {
		const long = this.memory === LONG;
		search: for (;;) {
			if (this.position + n.length - 1 >= h.length) {
				this.position = h.length;
				return false;
			}
			const at = this.position;
			if (!this.#contains(h[at + n.length - 1])) {
				this.position += n.length;
				if (!long) this.memory = 0;
				continue;
			}
			for (let i = long ? this.critPos : Math.max(this.critPos, this.memory); i < n.length; i++) {
				if (n[i] !== h[at + i]) {
					this.position += i - this.critPos + 1;
					if (!long) this.memory = 0;
					continue search;
				}
			}
			for (let i = this.critPos - 1; i >= (long ? 0 : this.memory); i--) {
				if (n[i] !== h[at + i]) {
					this.position += this.period;
					if (!long) this.memory = n.length - this.period;
					continue search;
				}
			}
			this.position += n.length;
			if (!long) this.memory = 0;
			return true;
		}
	}

	/** std's `next_back`, whether it matched. */
	nextBack(h: Uint8Array, n: Uint8Array): boolean {
		const long = this.memory === LONG;
		search: for (;;) {
			if (this.end < n.length) {
				this.end = 0;
				return false;
			}
			const at = this.end - n.length;
			if (!this.#contains(h[at])) {
				this.end -= n.length;
				if (!long) this.memoryBack = n.length;
				continue;
			}
			for (let i = (long ? this.critPosBack : Math.min(this.critPosBack, this.memoryBack)) - 1; i >= 0; i--) {
				if (n[i] !== h[at + i]) {
					this.end -= this.critPosBack - i;
					if (!long) this.memoryBack = n.length;
					continue search;
				}
			}
			for (let i = this.critPosBack; i < (long ? n.length : this.memoryBack); i++) {
				if (n[i] !== h[at + i]) {
					this.end -= this.period;
					if (!long) this.memoryBack = this.period;
					continue search;
				}
			}
			this.end -= n.length;
			if (!long) this.memoryBack = n.length;
			return true;
		}
	}
}

/** std's `maximal_suffix`: where the needle's maximal suffix by one order starts, and its period. */
function maximalSuffix(n: Uint8Array, greater: boolean): [number, number] {
	let [left, right, offset, period] = [0, 1, 0, 1];
	while (right + offset < n.length) {
		const [a, b] = [n[right + offset], n[left + offset]];
		if (greater ? a > b : a < b) {
			right += offset + 1;
			offset = 0;
			period = right - left;
		} else if (a === b) {
			if (offset + 1 === period) {
				right += offset + 1;
				offset = 0;
			} else offset++;
		} else {
			left = right;
			right++;
			offset = 0;
			period = 1;
		}
	}
	return [left, period];
}

/** std's `reverse_maximal_suffix`: the reversed needle's, stopping at `known` period. */
function reverseMaximalSuffix(n: Uint8Array, known: number, greater: boolean): number {
	let [left, right, offset, period] = [0, 1, 0, 1];
	while (right + offset < n.length) {
		const [a, b] = [n[n.length - (1 + right + offset)], n[n.length - (1 + left + offset)]];
		if (greater ? a > b : a < b) {
			right += offset + 1;
			offset = 0;
			period = right - left;
		} else if (a === b) {
			if (offset + 1 === period) {
				right += offset + 1;
				offset = 0;
			} else offset++;
		} else {
			left = right;
			right++;
			offset = 0;
			period = 1;
		}
		if (period === known) break;
	}
	return left;
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
	readonly m: Searcher;

	constructor(m: Searcher) {
		super();
		this.m = m;
	}

	override clone(h?: Clones): Iter<string> {
		return new Matches(this.m.clone(h));
	}

	next(): string | Done {
		const m = this.m;
		return m.nextMatch() ? m.s.slice(m.start, m.end) : DONE;
	}

	override nextBack(): string | Done {
		const m = this.m;
		return m.nextMatchBack() ? m.s.slice(m.start, m.end) : DONE;
	}

	[Symbol.dispose](): void {
		dispose(this.m);
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
	readonly m: Searcher;
	// Where the last match from each end starts, in code units and in bytes. The back's
	// bytes are counted once it first moves.
	#front = 0;
	#frontBytes = 0;
	#back = -1;
	#backBytes = 0;

	constructor(m: Searcher) {
		super();
		this.m = m;
	}

	override clone(h?: Clones): Iter<[usize, string]> {
		const it = new MatchIndices(this.m.clone(h));
		it.#front = this.#front;
		it.#frontBytes = this.#frontBytes;
		it.#back = this.#back;
		it.#backBytes = this.#backBytes;
		return it;
	}

	next(): [usize, string] | Done {
		const m = this.m;
		if (!m.nextMatch()) return DONE;
		this.#frontBytes += utf8Len(m.s, this.#front, m.start);
		this.#front = m.start;
		return [this.#frontBytes as usize, m.s.slice(m.start, m.end)];
	}

	override nextBack(): [usize, string] | Done {
		const m = this.m;
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
		dispose(this.m);
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

/** `str::rfind`: the last match's offset in UTF-8 bytes. */
export function strRfind(s: string, p: Pattern): usize | undefined {
	const m = new Searcher(s, p);
	return m.nextMatchBack() ? (utf8Len(s, 0, m.start) as usize) : undefined;
}

/** A pattern of `[char; N]` or `&[char]`: whether a char is any of `cs`, which it keeps for `Debug`. */
export function patternChars(cs: ArrayLike<char>): CharSet {
	const p = ((c: char) => {
		for (let i = 0; i < cs.length; i++) if (cs[i] === c) return true;
		return false;
	}) as CharSet;
	p.chars = cs;
	return p;
}

type CharSet = ((c: char) => boolean) & { chars: ArrayLike<char> };

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

/** `str::from_utf8_unchecked`: the text, each invalid sequence, which its contract rules out, replaced by U+FFFD. */
export function strFromUtf8Unchecked(bytes: ByteSlice): string {
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
	return fmtDebugStructFields(f, "Utf8Chunks", ["source"], [(f) => lossyDebug((it as Utf8Chunks).source(), f)]);
}

/** std's `Debug` of bytes as a string: in quotes, each chunk's valid runs a `write_str` apiece and each escape a `write_char` a char, each invalid byte `\x` and two hex digits. */
function lossyDebug(source: ByteSlice, f: Formatter): FmtResult {
	const out = f.out;
	let r = out.writeChar(QUOTE);
	for (const chunk of new Utf8Chunks(source)) {
		const valid = chunk.valid;
		let from = 0;
		let i = 0;
		for (const c of valid) {
			const escaped = escapeDebug(c, '"');
			if (escaped !== c) {
				if (r.ok) r = out.writeStr(valid.slice(from, i));
				for (let j = 0; r.ok && j < escaped.length; j++) r = out.writeChar(escaped.charCodeAt(j) as char);
				from = i + c.length;
			}
			i += c.length;
		}
		if (r.ok) r = out.writeStr(valid.slice(from));
		for (const b of byteSliceView(chunk.invalid)) {
			if (r.ok) r = out.writeStr("\\x");
			if (r.ok) r = u8UpperHex(b, { out, spec: { zero: true, width: 2 } });
		}
		if (!r.ok) return r;
	}
	return r.ok ? out.writeChar(QUOTE) : r;
}

const QUOTE = 0x22 as char;

/** `Debug` of a `Utf8Chunk`, as std derives it. */
export function fmtUtf8ChunkDebug(chunk: Utf8Chunk, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "Utf8Chunk", ["valid", "invalid"], [
		(f) => fmtStr(chunk.valid, f),
		(f) => fmtList(byteSliceView(chunk.invalid), f, u8Debug),
	]);
}

// `String`: a JS string, as a `str` is, and the length in UTF-8 bytes and the capacity std's
// would hold. Each step grows it as `RawVec` grows a `Vec<u8>`'s, so a `capacity` read gives
// std's. A `&mut String` is the object, edited in place, and so is a `&mut str` of it, as
// no write through one changes its length.

/** `alloc::string::String`: its text, `v`, its length in UTF-8 bytes, and its capacity. */
class RsString implements Place<string> {
	v: string;
	len: usize;
	cap: usize;

	constructor(v: string, len: usize, cap: usize) {
		this.v = v;
		this.len = len;
		this.cap = cap;
	}
}

export { RsString as String };

/** `isize::MAX` on wasm32, past which std refuses a capacity. */
const ISIZE_MAX = 0x7fffffff;

/** `str::len`: its length in UTF-8 bytes. */
export function stringLen(s: string): usize {
	return utf8Len(s, 0, s.length) as usize;
}

/** `String::new`, which holds nothing. */
export function stringNew(): RsString {
	return new RsString("", 0 as usize, 0 as usize);
}

/** A `String` of `text` with room for exactly its bytes, as `str::to_owned` makes one. */
export function stringFrom(text: string): RsString {
	const len = stringLen(text);
	return new RsString(text, len, len);
}

/** A `String` of `text`, `len` bytes long, with room for `cap`. */
export function stringOf(text: string, len: usize, cap: usize): RsString {
	return new RsString(text, len, cap);
}

/** `String::with_capacity`, which panics past what it may hold. */
export function stringWithCapacity(capacity: usize): RsString {
	return new RsString("", 0 as usize, capReserveExact(0, 0, capacity, 1) as usize);
}

/** `String::reserve`: room for `additional` more bytes, at least doubled if it grows. */
export function stringReserve(s: RsString, additional: usize): void {
	s.cap = capReserve(s.len, s.cap, additional, 1) as usize;
}

/** `String::reserve_exact`: room for exactly `additional` more bytes if it grows. */
export function stringReserveExact(s: RsString, additional: usize): void {
	s.cap = capReserveExact(s.len, s.cap, additional, 1) as usize;
}

/** `String::try_reserve`: `reserve`'s room, or the error where it panics. */
export function stringTryReserve(s: RsString, additional: usize): Result<undefined, TryReserveError> {
	return reserved(s, capTryReserve(s.len, s.cap, additional, 1));
}

/** `String::try_reserve_exact`. */
export function stringTryReserveExact(s: RsString, additional: usize): Result<undefined, TryReserveError> {
	return reserved(s, capTryReserveExact(s.len, s.cap, additional, 1));
}

/** `s` with room for `cap`, or the error of an overflow, `undefined`. */
function reserved(s: RsString, cap: number | undefined): Result<undefined, TryReserveError> {
	if (cap === undefined) return CAPACITY_OVERFLOW;
	s.cap = cap as usize;
	return OK;
}

/** `String::shrink_to`, and `shrink_to_fit`, of 0. */
export function stringShrinkTo(s: RsString, min: usize): void {
	s.cap = capShrink(s.len, s.cap, min) as usize;
}

/** `String::push_str` of `text`, `len` bytes long. */
export function stringPushStr(s: RsString, text: string, len: usize = stringLen(text)): void {
	stringReserve(s, len);
	s.v += text;
	s.len = (s.len + len) as usize;
}

/** `String::push`. */
export function stringPush(s: RsString, c: char): void {
	stringPushStr(s, globalThis.String.fromCodePoint(c), charLen(c) as usize);
}

/** `String + &str`, and `+=`: `s`, once it pushed `text`. */
export function stringAdd(s: RsString, text: string): RsString {
	stringPushStr(s, text);
	return s;
}

/** Push each char of `text` in turn, as std's `push` of each grows `s`. */
export function stringPushChars(s: RsString, text: string): void {
	let { len, cap } = s;
	for (let i = 0; i < text.length; i++) {
		const u = text.charCodeAt(i);
		// A pair's high half is two bytes, and its low half the other two.
		const n = u < 0x80 ? 1 : u < 0x800 ? 2 : u >= 0xd800 && u <= 0xdbff ? 4 : u >= 0xdc00 && u <= 0xdfff ? 0 : 3;
		if (n > cap - len) cap = capReserve(len, cap, n, 1);
		len += n;
	}
	s.v += text;
	s.len = len as usize;
	s.cap = cap as usize;
}

/** `Clone` of a `String`: its text, with room for exactly its bytes. */
export function stringClone(s: RsString): RsString {
	return new RsString(s.v, s.len, s.len);
}

/**
 * `clone_from` of `src` into `s`, as `Vec<u8>`'s clears it and extends it by `src`'s bytes,
 * and `str::clone_into`, which clears it and pushes the text.
 */
export function stringCloneFrom(s: RsString, src: RsString | string): RsString {
	s.v = "";
	s.len = 0 as usize;
	if (typeof src === "string") stringPushStr(s, src);
	else stringPushStr(s, src.v, src.len);
	return s;
}

/** `String::clear`, which keeps its room. */
export function stringClear(s: RsString): void {
	s.v = "";
	s.len = 0 as usize;
}

/**
 * `concat` and `join` of strings or `String`s by `sep`: a `String` with room for exactly
 * their bytes, as std's reserves them, which panics as std's does where they pass
 * `usize::MAX` or what a `String` may hold.
 */
export function strsJoin(parts: readonly (string | RsString)[], sep: string): RsString {
	const seps = Math.max(parts.length - 1, 0);
	// Each distinct text's bytes counted once, as a long result mostly repeats a few.
	const counted = new Map<string, number>();
	const count = (s: string): number => {
		let n = counted.get(s);
		if (n === undefined) counted.set(s, (n = stringLen(s)));
		return n;
	};
	let len = count(sep) * seps;
	const texts: string[] = [];
	for (const part of parts) {
		if (typeof part === "string") {
			len += count(part);
			texts.push(part);
		} else {
			len += part.len;
			texts.push(part.v);
		}
	}
	if (len > 0xffffffff) panic("attempt to join into collection with len > usize::MAX", "join_len", true);
	if (len > ISIZE_MAX) panic("capacity overflow", "capacity");
	return new RsString(texts.join(sep), len as usize, len as usize);
}

/**
 * `str::replace` of a pattern, `utf8` if a `char`, `&str`, or `&String`: room for `s`'s
 * bytes if what replaces each match is no shorter, and else none, which each push grows.
 * One ASCII byte for another is a copy, with room for exactly its bytes.
 */
export function strReplace(s: string, p: Pattern, to: string, utf8: boolean): RsString {
	const len = stringLen(s);
	if (utf8 && typeof p === "string" && p.length === 1 && p.charCodeAt(0) < 0x80 && to.length === 1 && to.charCodeAt(0) < 0x80) {
		return new RsString(s.replaceAll(p, to), len, len);
	}
	const toLen = stringLen(to);
	const fits = utf8 && typeof p === "string" && stringLen(p) <= toLen;
	return replaced(new RsString("", 0 as usize, fits ? len : (0 as usize)), s, p, to, toLen, Infinity);
}

/** `str::replacen`: room for 32 bytes, which each push of the first `n` matches grows. */
export function strReplacen(s: string, p: Pattern, to: string, n: usize): RsString {
	return replaced(new RsString("", 0 as usize, 32 as usize), s, p, to, stringLen(to), n);
}

/** `out`, once it pushed the text of `s` between its first `n` matches of `p`, and `to` for each. */
function replaced(out: RsString, s: string, p: Pattern, to: string, toLen: usize, n: number): RsString {
	const m = new Searcher(s, p);
	let last = 0;
	for (let i = 0; i < n && m.nextMatch(); i++) {
		stringPushStr(out, s.slice(last, m.start));
		stringPushStr(out, to, toLen);
		last = m.end;
	}
	stringPushStr(out, s.slice(last));
	return out;
}

/** `str::to_lowercase` and `to_uppercase` of `s` to `text`: room for `s`'s bytes, which each char's push grows. */
export function stringCased(s: string, text: string): RsString {
	const out = new RsString("", 0 as usize, stringLen(s));
	stringPushChars(out, text);
	return out;
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
			s += globalThis.String.fromCharCode(u);
			continue;
		}
		const next = i + 1 < len ? unit(i + 1) : 0;
		if (u <= 0xdbff && next >= 0xdc00 && next <= 0xdfff) {
			s += globalThis.String.fromCharCode(u, next);
			i++;
		} else if (lossy) s += "�";
		else return undefined;
	}
	return s;
}

/**
 * The `String` of `len` units, `unit(i)` the `i`th, or a `LoneSurrogate` error, as
 * `String::from_utf16` reads them: room for a byte a unit, which each char's push grows.
 */
function fromUtf16(len: number, unit: (i: number) => number): Result<RsString, FromUtf16Error> {
	const text = fromUtf16Units(len, unit, false);
	if (text === undefined) return Err(new FromUtf16Error("LoneSurrogate"));
	const s = stringWithCapacity(len as usize);
	stringPushChars(s, text);
	return Ok(s);
}

/**
 * The `String` of `len` units, `unit(i)` the `i`th, U+FFFD for each surrogate without its
 * pair, as `String::from_utf16_lossy` collects them: room for half a char a unit, which
 * each char's push grows.
 */
function fromUtf16Lossy(len: number, unit: (i: number) => number): RsString {
	const s = stringNew();
	stringReserve(s, Math.ceil(len / 2) as usize);
	stringPushChars(s, fromUtf16Units(len, unit, true) as string);
	return s;
}

/** `String::from_utf16`. */
export function stringFromUtf16(v: Slice<u16>): Result<RsString, FromUtf16Error> {
	return fromUtf16(v.length, (i) => v.data[v.start + i]);
}

/** `String::from_utf16_lossy`: U+FFFD for each surrogate without its pair. */
export function stringFromUtf16Lossy(v: Slice<u16>): RsString {
	return fromUtf16Lossy(v.length, (i) => v.data[v.start + i]);
}

/** The `i`th unit of `v`'s bytes, little-endian unless `big`. */
function byteUnit(v: ByteSlice, big: boolean): (i: number) => number {
	const [hi, lo] = big ? [0, 1] : [1, 0];
	return (i) => (v.data[v.start + 2 * i + hi] << 8) | v.data[v.start + 2 * i + lo];
}

/** `String::from_utf16le`, an `OddBytes` error of an odd number of bytes. */
export function stringFromUtf16le(v: ByteSlice): Result<RsString, FromUtf16Error> {
	return v.length % 2 === 1 ? Err(new FromUtf16Error("OddBytes")) : fromUtf16(v.length / 2, byteUnit(v, false));
}

/** `String::from_utf16be`, an `OddBytes` error of an odd number of bytes. */
export function stringFromUtf16be(v: ByteSlice): Result<RsString, FromUtf16Error> {
	return v.length % 2 === 1 ? Err(new FromUtf16Error("OddBytes")) : fromUtf16(v.length / 2, byteUnit(v, true));
}

/** `String::from_utf16le_lossy`, and U+FFFD pushed for an odd byte at the end. */
export function stringFromUtf16leLossy(v: ByteSlice): RsString {
	return oddByte(fromUtf16Lossy(v.length >>> 1, byteUnit(v, false)), v);
}

/** `String::from_utf16be_lossy`, and U+FFFD pushed for an odd byte at the end. */
export function stringFromUtf16beLossy(v: ByteSlice): RsString {
	return oddByte(fromUtf16Lossy(v.length >>> 1, byteUnit(v, true)), v);
}

/** `s`, and U+FFFD pushed if `v` has an odd byte at the end. */
function oddByte(s: RsString, v: ByteSlice): RsString {
	if (v.length % 2 === 1) stringPushStr(s, "�", 3 as usize);
	return s;
}

/** `Debug` of `FromUtf16Error`. */
export function fmtFromUtf16ErrorDebug(e: FromUtf16Error, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "FromUtf16Error", ["kind"], [(f) => f.out.writeStr(e.kind)]);
}

/** `Display` of `FromUtf16Error`, padded as a `str`. */
export function fmtFromUtf16ErrorDisplay(e: FromUtf16Error, f: Formatter): FmtResult {
	return fmtPad(f, e.kind === "LoneSurrogate" ? "invalid utf-16: lone surrogate found" : "invalid utf-16: odd number of bytes");
}

// `String`'s edits, in place.

/** The code unit where byte `i` of `s` starts a char, or panic with `message`. */
function boundary(s: string, i: number, message: string, at?: Location): number {
	const pos = unitAt(s, 0, 0, i);
	if (pos < 0) panic(message, at);
	return pos;
}

/** `String::truncate`: to `len` bytes, unless already no longer. */
export function stringTruncate(s: RsString, len: usize, at?: Location): void {
	if (len > s.len) return;
	s.v = s.v.slice(0, boundary(s.v, len, "assertion failed: self.is_char_boundary(new_len)", at));
	s.len = len;
}

/** `String::insert_str`, and `insert` of a char's text: room for its bytes reserved. */
export function stringInsertStr(s: RsString, i: usize, text: string, at?: Location): void {
	const pos = boundary(s.v, i, "assertion failed: self.is_char_boundary(idx)", at);
	const len = stringLen(text);
	stringReserve(s, len);
	s.v = s.v.slice(0, pos) + text + s.v.slice(pos);
	s.len = (s.len + len) as usize;
}

/** `String::retain`: the chars `f` keeps, calling it on each in order. */
export function stringRetain(s: RsString, f: (c: char) => boolean): void {
	let kept = "";
	for (const ch of s.v) if (f(ch.codePointAt(0) as char)) kept += ch;
	s.v = kept;
	s.len = stringLen(kept);
}

/** `String::replace_range`: room reserved for what `text` adds. */
export function stringReplaceRange(s: RsString, r: Bounds<number>, text: string, at?: Location): void {
	const [start, end] = boundsRange(r, s.len, at);
	const from = boundary(s.v, start, "start of range should be a character boundary", at);
	const to = boundary(s.v, end, "end of range should be a character boundary", at);
	const len = stringLen(text);
	if (len > end - start) stringReserve(s, (len - (end - start)) as usize);
	s.v = s.v.slice(0, from) + text + s.v.slice(to);
	s.len = (s.len - (end - start) + len) as usize;
}

/** `String::extend_from_within`: room reserved for the bytes it copies. */
export function stringExtendFromWithin(s: RsString, r: Bounds<number>, at?: Location): void {
	const [start, end] = boundsRange(r, s.len, at);
	const from = boundary(s.v, start, "assertion failed: self.is_char_boundary(start)", at);
	const to = boundary(s.v, end, "assertion failed: self.is_char_boundary(end)", at);
	stringPushStr(s, s.v.slice(from, to), (end - start) as usize);
}

/** `String::pop`: the last char, if any. */
export function stringPop(s: RsString): char | undefined {
	if (s.len === 0) return undefined;
	const pos = before(s.v, s.v.length);
	const c = s.v.codePointAt(pos) as char;
	s.v = s.v.slice(0, pos);
	s.len = (s.len - charLen(c)) as usize;
	return c;
}

/** `String::remove`: the char at byte `i`. */
export function stringRemove(s: RsString, i: usize, at?: Location): char {
	const pos = unitAt(s.v, 0, 0, i);
	if (pos < 0) sliceFailed(s.v, i, s.len, at);
	if (pos === s.v.length) panic("cannot remove a char from the end of a string", at);
	const c = s.v.codePointAt(pos) as char;
	s.v = s.v.slice(0, pos) + s.v.slice(pos + (c > 0xffff ? 2 : 1));
	s.len = (s.len - charLen(c)) as usize;
	return c;
}

/** `String::split_off`: what comes after byte `i`, with room for exactly its bytes. */
export function stringSplitOff(s: RsString, i: usize, at?: Location): RsString {
	const pos = boundary(s.v, i, "assertion failed: self.is_char_boundary(at)", at);
	const len = (s.len - i) as usize;
	const rest = new RsString(s.v.slice(pos), len, len);
	s.v = s.v.slice(0, pos);
	s.len = i;
	return rest;
}

// `String` from and into iterators. Each reads the iterator by `for_each`, as std's does,
// which `drops` makes its own `fold`.

/**
 * `Extend<char>` and `Extend<&char>` of `s`, and `collect` of chars from `String::new`: each
 * char pushed, once it reserved the iterator's lower bound.
 */
export function stringExtendChars(s: RsString, it: Iter<char>, drops = false): RsString {
	stringReserve(s, it.sizeHint()[0] as usize);
	iterForEach(it, (c) => stringPush(s, c), drops);
	return s;
}

/** `String` as a tuple's part, of chars, which reserves as `stringReserve`. */
export const STRING_CHARS_PART: Part<RsString, char> = {
	reserve: stringReserve,
	one: (s, c) => (stringPush(s, c), s),
};

/** `String` as a tuple's part, of strings, which never reserves. */
export const STRING_PART: Part<RsString, string | RsString> = { one: (s, text) => (pushText(s, text), s) };

/** `String` as a tuple's part, of `Cow<str>`s, which never reserves. */
export const STRING_COW_PART: Part<RsString, StrCow> = { one: (s, c) => (pushText(s, strCowText(c)), s) };

/** `push_str` of a `str` or a `String`'s text. */
function pushText(s: RsString, text: string | RsString): void {
	if (typeof text === "string") stringPushStr(s, text);
	else stringPushStr(s, text.v, text.len);
}

/** A `Cow<str>`'s text, or the `String` it owns. */
function strCowText(c: StrCow): string | RsString {
	return c instanceof Borrowed ? c.slice : c;
}

/**
 * `Extend` of `s` by `&str`, `Box<str>`, `String`, or `Cow<str>`, each pushed, and `collect`
 * of `&str`s and `Box<str>`s from `String::new`.
 */
export function stringExtend(s: RsString, it: Iter<string | RsString | StrCow>, drops = false): RsString {
	iterForEach(it, (text) => pushText(s, text instanceof Borrowed ? text.slice : text), drops);
	return s;
}

/** `Extend<&str>` of an array's strings, and `from_iter` of one: room reserved for all of them, then each pushed. */
export function stringExtendArray(s: RsString, it: Iter<string>): RsString {
	const texts: string[] = [];
	const lens: usize[] = [];
	let len = 0;
	iterForEach(it, (text) => {
		const n = stringLen(text);
		texts.push(text);
		lens.push(n);
		len += n;
	});
	// std's sum saturates, so a reserve past `usize::MAX` panics as one of it does.
	stringReserve(s, Math.min(len, 0xffffffff) as usize);
	texts.forEach((text, i) => stringPushStr(s, text, lens[i]));
	return s;
}

/**
 * `collect` of `String`s or `Cow<str>`s: the first, owned, by `next`, which the rest extend,
 * as std's keeps the first's room.
 */
export function stringFromStrings(it: Iter<RsString | StrCow>, drops = false): RsString {
	const first = it.next();
	if (first !== DONE) return stringExtend(first instanceof Borrowed ? stringFrom(first.slice) : first, it, drops);
	if (drops) dispose(it);
	return stringNew();
}

/**
 * `String::drain`: the chars within the byte range `r` of `s`, which leaves `s` without them
 * as it drops, and as it was if it never does, as `mem::forget` leaves it.
 */
export function stringDrain(s: RsString, r: Bounds<number>, at?: Location): Iter<char> {
	const text = s.v;
	const [start, end] = boundsRange(r, s.len, at);
	const from = boundary(text, start, "assertion failed: self.is_char_boundary(start)", at);
	const to = boundary(text, end, "assertion failed: self.is_char_boundary(end)", at);
	return new StringDrain(s, text, from, to, end - start);
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
	readonly #s: RsString;
	readonly #text: string;
	readonly #from: number;
	readonly #to: number;
	readonly #bytes: number;

	constructor(s: RsString, text: string, from: number, to: number, bytes: number) {
		super(text.slice(from, to));
		this.#s = s;
		this.#text = text;
		this.#from = from;
		this.#to = to;
		this.#bytes = bytes;
	}

	[Symbol.dispose](): void {
		this.#s.v = this.#text.slice(0, this.#from) + this.#text.slice(this.#to);
		this.#s.len = (this.#s.len - this.#bytes) as usize;
	}
}
