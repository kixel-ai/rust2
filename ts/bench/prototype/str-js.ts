// Prototype only, for moq-dev/rust2#574's decision: a `String` as a JS string with its
// UTF-8 length and std's exact capacity. `&str` and `Box<str>` stay JS strings. Copied to
// a translation's `rt/proto.ts` by str-routes.py.

import { type Arguments, Err, type FmtResult, FromUtf8Error, type Formatter, type HashMap, OK, Ok, type Result, type Write, byteVecView, type ByteVec, type char } from "../rt.ts";

const decoder = /* @__PURE__ */ new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });


/** A `String`: its text, its length in UTF-8 bytes, and its capacity. */
export class PString {
	s: string;
	len: number;
	cap: number;

	constructor(s: string, len: number, cap: number) {
		this.s = s;
		this.len = len;
		this.cap = cap;
	}
}

/** The UTF-8 length of `s`. */
export function pStrLen(s: string): number {
	let n = s.length;
	for (let i = 0; i < s.length; i++) {
		const u = s.charCodeAt(i);
		if (u >= 0x80) n += u < 0x800 || (u >= 0xd800 && u <= 0xdfff) ? 1 : 2;
	}
	return n;
}

/** Room for `add` more bytes, as `RawVec::grow_amortized` makes it. */
function grow(v: PString, add: number): void {
	const needed = v.len + add;
	if (needed > v.cap) v.cap = Math.max(v.cap * 2, needed, 8);
}

/** `String::with_capacity`. */
export function pStringNew(cap: number): PString {
	return new PString("", 0, cap);
}

/** `String::push_str`. */
export function pStringPushStr(v: PString, t: string): void {
	const n = pStrLen(t);
	grow(v, n);
	v.len += n;
	v.s += t;
}

/** `String::push_str` of a literal, whose length the translator knows. */
export function pStringPushLit(v: PString, t: string, n: number): void {
	grow(v, n);
	v.len += n;
	v.s += t;
}

/** `String::clone`: room for exactly its bytes. */
export function pStringClone(v: PString): PString {
	return new PString(v.s, v.len, v.len);
}

/** `String::from_utf8`, which keeps the vector's capacity. */
export function pStringFromUtf8(bytes: ByteVec): Result<PString, FromUtf8Error> {
	try {
		return Ok(new PString(decoder.decode(byteVecView(bytes)), bytes.length, bytes.data.length));
	} catch {
		return Err(new FromUtf8Error(bytes));
	}
}


class Writer implements Write {
	readonly v: PString;

	constructor(v: PString) {
		this.v = v;
	}

	writeStr(s: string): FmtResult {
		pStringPushStr(this.v, s);
		return OK;
	}

	writeChar(c: char): FmtResult {
		const n = c < 0x80 ? 1 : c < 0x800 ? 2 : c < 0x10000 ? 3 : 4;
		grow(this.v, n);
		this.v.len += n;
		this.v.s += String.fromCodePoint(c);
		return OK;
	}
}

function write(out: Write, args: Arguments): FmtResult {
	for (const part of args) {
		const r = typeof part === "string" ? out.writeStr(part) : part({ out, spec: {} } as Formatter);
		if (!r.ok) return r;
	}
	return OK;
}

/** `write!` into a `String`. */
export function pFmtWriteString(v: PString, args: Arguments): FmtResult {
	return write(new Writer(v), args);
}

/** `Arguments::estimated_capacity`, of the literal pieces' lengths, which the translator knows. */
function estimate(args: Arguments, pieces: number): number {
	if (args.every((a) => typeof a === "string")) return pieces;
	return typeof args[0] !== "string" && pieces < 16 ? 0 : pieces * 2;
}

/** `format!`, its pieces' length given. */
export function pFmtFormat(args: Arguments, pieces: number): PString {
	const v = new PString("", 0, estimate(args, pieces));
	write(new Writer(v), args);
	return v;
}

/** `HashMap::insert` of a `String` key: the map goes by its text, and keeps the key with its value. */
export function pHashInsertString<V>(m: HashMap<string, { k: PString; v: V }>, key: PString, value: V): V | undefined {
	const old = m.p.get(key.s);
	m.p.set(key.s, { k: key, v: value });
	return old?.v;
}

/** `HashMap::get` by a `&str`. */
export function pHashGetString<V>(m: HashMap<string, { k: PString; v: V }>, key: string): V | undefined {
	return m.p.get(key)?.v;
}
