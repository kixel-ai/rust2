// Prototype only, for moq-dev/rust2#574's decision: a `String` as its UTF-8 bytes, an
// `rs.ByteVec`, whose storage's length is std's exact capacity, and a `&str` as an
// `rs.ByteSlice`. Copied to a translation's `rs/proto.ts` by str-routes.py.

import {
	type Arguments,
	ByteSlice,
	ByteVec,
	byteVecExtendFromSlice,
	byteVecPush,
	byteVecView,
	Err,
	type FmtResult,
	FromUtf8Error,
	type Formatter,
	fmtPad,
	hashing,
	OK,
	Ok,
	type Result,
	utf8Check,
	type Write,
	type char,
} from "../rs.ts";

const decoder = /* @__PURE__ */ new TextDecoder("utf-8", { ignoreBOM: true });
const encoder = /* @__PURE__ */ new TextEncoder();

/** A literal's bytes, made once where the module loads. */
export function pLit(s: string): ByteSlice {
	const b = encoder.encode(s);
	return new ByteSlice(b, 0, b.length);
}

/** A `String`'s or a `&str`'s bytes. */
function view(s: ByteVec | ByteSlice): Uint8Array {
	return s instanceof ByteVec ? s.data.subarray(0, s.length) : s.data.subarray(s.start, s.start + s.length);
}

/** `String::push_str`. */
export function pStringPushStr(v: ByteVec, t: ByteSlice): void {
	byteVecExtendFromSlice(v, t);
}

/** `String::push` of an ASCII char. */
export function pStringPushAscii(v: ByteVec, c: number): void {
	byteVecPush(v, c);
}

/** `&**s`: a `String`'s text. */
export function pStringAsStr(v: ByteVec): ByteSlice {
	return new ByteSlice(v.data, 0, v.length);
}

/** `String::clone`: room for exactly its bytes. */
export function pStringClone(v: ByteVec): ByteVec {
	const c = new ByteVec(v.length);
	c.data.set(byteVecView(v));
	c.length = v.length;
	return c;
}

/** `String::from_utf8`: the vector itself, checked. */
export function pStringFromUtf8(bytes: ByteVec): Result<ByteVec, FromUtf8Error> {
	return utf8Check(byteVecView(bytes)) === undefined ? Ok(bytes) : Err(new FromUtf8Error(bytes));
}

/** `str::strip_prefix` of a `&str`. */
export function pStrStripPrefix(s: ByteSlice, p: ByteSlice): ByteSlice | undefined {
	if (p.length > s.length) return undefined;
	for (let i = 0; i < p.length; i++) if (s.data[s.start + i] !== p.data[p.start + i]) return undefined;
	return new ByteSlice(s.data, s.start + p.length, s.length - p.length);
}

/** `str::strip_prefix` of an ASCII char. */
export function pStrStripAscii(s: ByteSlice, c: number): ByteSlice | undefined {
	return s.length > 0 && s.data[s.start] === c ? new ByteSlice(s.data, s.start + 1, s.length - 1) : undefined;
}

/** The UTF-8 length of `s`. */
function utf8Len(s: string): number {
	let n = s.length;
	for (let i = 0; i < s.length; i++) {
		const u = s.charCodeAt(i);
		if (u >= 0x80) n += u < 0x800 || (u >= 0xd800 && u <= 0xdfff) ? 1 : 2;
	}
	return n;
}

/** Room for `add` more bytes, as `RawVec::grow_amortized` makes it. */
function reserve(v: ByteVec, add: number): void {
	const needed = v.length + add;
	if (needed <= v.data.length) return;
	const grown = new Uint8Array(Math.max(needed, v.data.length * 2, 8));
	grown.set(v.data);
	v.data = grown;
}

/** Encode a JS string onto `v`, growing it as std's `push_str` of its UTF-8 would. */
function pushJs(v: ByteVec, s: string): void {
	// Room for three bytes a code unit holds it whatever it is, and no growth would happen.
	if (v.data.length - v.length < 3 * s.length) reserve(v, utf8Len(s));
	if (s.length <= 16) {
		const d = v.data;
		let at = v.length;
		for (let i = 0; i < s.length; i++) {
			let c = s.charCodeAt(i);
			if (c < 0x80) {
				d[at++] = c;
				continue;
			}
			if (c >= 0xd800 && c <= 0xdbff) c = s.codePointAt(i++) as number;
			at = putChar(d, at, c);
		}
		v.length = at;
	} else {
		v.length += encoder.encodeInto(s, v.data.subarray(v.length)).written;
	}
}

function putChar(d: Uint8Array, at: number, c: number): number {
	if (c < 0x80) d[at++] = c;
	else if (c < 0x800) {
		d[at++] = 0xc0 | (c >> 6);
		d[at++] = 0x80 | (c & 0x3f);
	} else if (c < 0x10000) {
		d[at++] = 0xe0 | (c >> 12);
		d[at++] = 0x80 | ((c >> 6) & 0x3f);
		d[at++] = 0x80 | (c & 0x3f);
	} else {
		d[at++] = 0xf0 | (c >> 18);
		d[at++] = 0x80 | ((c >> 12) & 0x3f);
		d[at++] = 0x80 | ((c >> 6) & 0x3f);
		d[at++] = 0x80 | (c & 0x3f);
	}
	return at;
}

/** Writes into a `String`'s bytes, and takes a `&str`'s bytes as they are. */
class Writer implements Write {
	readonly v: ByteVec;

	constructor(v: ByteVec) {
		this.v = v;
	}

	writeStr(s: string): FmtResult {
		pushJs(this.v, s);
		return OK;
	}

	writeBytes(s: ByteSlice): FmtResult {
		byteVecExtendFromSlice(this.v, s);
		return OK;
	}

	writeChar(c: char): FmtResult {
		reserve(this.v, c < 0x80 ? 1 : c < 0x800 ? 2 : c < 0x10000 ? 3 : 4);
		this.v.length = putChar(this.v.data, this.v.length, c);
		return OK;
	}
}

/** A `&str`'s `Display`: its bytes as they are into a `String`, else decoded. */
export function pFmtPadStr(f: Formatter, s: ByteSlice): FmtResult {
	if (f.out instanceof Writer && f.spec.width === undefined && f.spec.precision === undefined) return f.out.writeBytes(s);
	return fmtPad(f, decoder.decode(view(s)));
}

function write(out: Write, args: Arguments): FmtResult {
	for (const part of args) {
		const r = typeof part === "string" ? out.writeStr(part) : part({ out, spec: {} } as Formatter);
		if (!r.ok) return r;
	}
	return OK;
}

/** `write!` into a `String`. */
export function pFmtWriteString(v: ByteVec, args: Arguments): FmtResult {
	return write(new Writer(v), args);
}

/** `Arguments::estimated_capacity`, of the literal pieces' lengths, which the translator knows. */
function estimate(args: Arguments, pieces: number): number {
	if (args.every((a) => typeof a === "string")) return pieces;
	return typeof args[0] !== "string" && pieces < 16 ? 0 : pieces * 2;
}

/** `format!`, its pieces' length given. */
export function pFmtFormat(args: Arguments, pieces: number): ByteVec {
	const v = new ByteVec(estimate(args, pieces));
	write(new Writer(v), args);
	return v;
}

const FF = [0xff];

/** A `String` or `&str` key: its bytes, then 0xff, as `Hash for str` writes them. */
export const P_STR_HASHING = /* @__PURE__ */ hashing<ByteVec | ByteSlice>(
	(k, s) => {
		s.ints(view(k), 1);
		s.ints(FF, 1);
	},
	(a, b) => {
		if (a.length !== b.length) return false;
		const x = view(a);
		const y = view(b);
		for (let i = 0; i < x.length; i++) if (x[i] !== y[i]) return false;
		return true;
	},
	true,
);
