// Hashing, for rust2-ts output: `DefaultHasher`, which is std's SipHash-1-3 byte for byte,
// `RandomState`, which keys it, the crate's own `Hasher`, and the `Hash` of std's types,
// which call the `Hasher` methods std's call. `rs.ts` re-exports all of it.
//
// A hash collection hashes a key that is a JS object with a faster hasher of its own,
// whose values nothing sees, as std keys a map's hasher at random.

import { ByteSlice, type ByteVec, type Elements, type None, Slice, sliceData, sliceItems, sliceLength, sliceStart } from "../rs.ts";
import {
	type i8,
	type i16,
	type i32,
	type i64,
	type i128,
	type isize,
	type Limbs,
	type U128Limbs,
	type u8,
	type u16,
	type u32,
	type u64,
	type u128,
	type usize,
	u64Cast,
	u64FromHalves,
	u128Cast,
} from "./int.ts";
import type { LinkedList } from "./collections.ts";
import type { String as RsString } from "./str.ts";
import type { Arc } from "./task.ts";

/** What `Hash` writes into, by `Hasher`'s methods: `DefaultHasher`, a hash collection's own, or the crate's. */
export interface Hasher {
	/** `Hasher::write` of `len` bytes of `data` from `start`. */
	write(data: Uint8Array, start: number, len: number): void;
	u8(value: number): void;
	u16(value: number): void;
	u32(value: number): void;
	u64(value: number | Limbs): void;
	u128(value: number | Limbs): void;
	usize(value: number): void;
	i8(value: number): void;
	i16(value: number): void;
	i32(value: number): void;
	i64(value: number | Limbs): void;
	i128(value: number | Limbs): void;
	isize(value: number): void;
	/** `Hasher::write_length_prefix`. */
	prefix(len: number): void;
	/** `Hasher::write_str`. */
	str(str: string): void;
	/** `Hash::hash_slice` of integers `size` bytes wide: their little-endian bytes, in one `write`. */
	ints(values: ArrayLike<number | Limbs>, size: 1 | 2 | 4 | 8 | 16): void;
	/** `Hasher::finish`, which leaves the hasher as it was. */
	finish(): u64;
}

/** What writes 32-bit words: `DefaultHasher` and a hash collection's own. */
interface Words {
	/** Write `word`'s low `n` bytes, little-endian. */
	word(word: number, n: 1 | 2 | 4): void;
}

/** Write `value`, a 64-bit integer, as its low word, then its high one. */
function words64(s: Words, value: number | Limbs): void {
	if (typeof value === "number") {
		s.word(value >>> 0, 4);
		s.word(Math.floor(value / 4294967296) | 0, 4);
	} else {
		s.word(value.lo, 4);
		s.word(value.hi, 4);
	}
}

/** Write `value`, a 128-bit integer, as its four words, low first. */
function words128(s: Words, value: number | Limbs): void {
	if (typeof value === "number") {
		const fill = value < 0 ? -1 : 0;
		words64(s, value);
		s.word(fill, 4);
		s.word(fill, 4);
	} else {
		const wide = value as U128Limbs;
		s.word(wide.lo, 4);
		s.word(wide.w1, 4);
		s.word(wide.w2, 4);
		s.word(wide.hi, 4);
	}
}

/** `Hasher::ints` of what writes words: each integer's, as its bytes are the same. */
function wordInts(s: Words, values: ArrayLike<number | Limbs>, size: 1 | 2 | 4 | 8 | 16): void {
	for (let i = 0; i < values.length; i++) {
		const v = values[i];
		if (size === 8) words64(s, v);
		else if (size === 16) words128(s, v);
		else s.word(v as number, size);
	}
}

/**
 * `DefaultHasher`: std's SipHash-1-3, its 64-bit lanes `v0` to `v3` kept as 32-bit halves,
 * then the tail of bytes not yet compressed, how many it holds, and the low byte of how
 * many were written, which is all `finish` reads of it.
 */
class SipHasher implements Hasher, Words {
	private v0h: number;
	private v0l: number;
	private v1h: number;
	private v1l: number;
	private v2h: number;
	private v2l: number;
	private v3h: number;
	private v3l: number;
	private th = 0;
	private tl = 0;
	private ntail = 0;
	private length = 0;

	/** Keyed by `k0` and `k1`, as 32-bit halves. */
	constructor(k: RandomState) {
		this.v0h = k.k0h ^ 0x736f6d65;
		this.v0l = k.k0l ^ 0x70736575;
		this.v1h = k.k1h ^ 0x646f7261;
		this.v1l = k.k1l ^ 0x6e646f6d;
		this.v2h = k.k0h ^ 0x6c796765;
		this.v2l = k.k0l ^ 0x6e657261;
		this.v3h = k.k1h ^ 0x74656462;
		this.v3l = k.k1l ^ 0x79746573;
	}

	write(data: Uint8Array, start: number, len: number): void {
		for (let i = 0; i < len; i++) this.word(data[start + i], 1);
	}

	u8(value: number): void {
		this.word(value, 1);
	}

	u16(value: number): void {
		this.word(value, 2);
	}

	u32(value: number): void {
		this.word(value, 4);
	}

	u64(value: number | Limbs): void {
		words64(this, value);
	}

	u128(value: number | Limbs): void {
		words128(this, value);
	}

	usize(value: number): void {
		this.word(value, 4);
	}

	i8(value: number): void {
		this.word(value, 1);
	}

	i16(value: number): void {
		this.word(value, 2);
	}

	i32(value: number): void {
		this.word(value, 4);
	}

	i64(value: number | Limbs): void {
		words64(this, value);
	}

	i128(value: number | Limbs): void {
		words128(this, value);
	}

	isize(value: number): void {
		this.word(value, 4);
	}

	prefix(len: number): void {
		this.word(len, 4);
	}

	ints(values: ArrayLike<number | Limbs>, size: 1 | 2 | 4 | 8 | 16): void {
		wordInts(this, values, size);
	}

	word(word: number, n: 1 | 2 | 4): void {
		this.length = (this.length + n) & 0xff;
		const at = this.ntail;
		const free = 8 - at;
		if (n < free) {
			this.put(word, at, n);
			this.ntail = at + n;
			return;
		}
		this.put(word, at, free);
		this.compress(this.th, this.tl);
		this.th = 0;
		this.tl = 0;
		if (n > free) this.put(word >>> (free * 8), 0, n - free);
		this.ntail = n - free;
	}

	str(str: string): void {
		// Its UTF-8 bytes, then 0xff, which no `str` holds, so no write is a prefix of another.
		for (let i = 0; i < str.length; i++) {
			const c = str.codePointAt(i) as number;
			if (c < 0x80) this.word(c, 1);
			else if (c < 0x800) this.word(0x80c0 | (c >> 6) | ((c & 0x3f) << 8), 2);
			else if (c < 0x10000) {
				this.word(0x80e0 | (c >> 12) | (((c >> 6) & 0x3f) << 8), 2);
				this.word(0x80 | (c & 0x3f), 1);
			} else {
				i++;
				this.word(0x808080f0 | (c >> 18) | (((c >> 12) & 0x3f) << 8) | (((c >> 6) & 0x3f) << 16) | ((c & 0x3f) << 24), 4);
			}
		}
		this.word(0xff, 1);
	}

	finish(): u64 {
		const v = LAST;
		v.v0h = this.v0h;
		v.v0l = this.v0l;
		v.v1h = this.v1h;
		v.v1l = this.v1l;
		v.v2h = this.v2h;
		v.v2l = this.v2l;
		v.v3h = this.v3h;
		v.v3l = this.v3l;
		v.compress(this.th | (this.length << 24), this.tl);
		v.v2l ^= 0xff;
		v.rounds(3);
		return u64FromHalves((v.v0h ^ v.v1h ^ v.v2h ^ v.v3h) >>> 0, (v.v0l ^ v.v1l ^ v.v2l ^ v.v3l) >>> 0);
	}

	/** Put `word`'s low `n` bytes into the tail, from its byte `at`. */
	private put(word: number, at: number, n: number): void {
		const bits = n * 8;
		const m = bits === 32 ? word : word & ((1 << bits) - 1);
		const shift = at * 8;
		if (shift >= 32) this.th |= m << (shift - 32);
		else {
			this.tl |= m << shift;
			if (shift + bits > 32) this.th |= m >>> (32 - shift);
		}
	}

	/** Compress the word `hi`:`lo` into the lanes. */
	private compress(hi: number, lo: number): void {
		this.v3h ^= hi;
		this.v3l ^= lo;
		this.rounds(1);
		this.v0h ^= hi;
		this.v0l ^= lo;
	}

	/** `n` of SipHash's rounds over the lanes. */
	private rounds(n: number): void {
		let v0h = this.v0h;
		let v0l = this.v0l;
		let v1h = this.v1h;
		let v1l = this.v1l;
		let v2h = this.v2h;
		let v2l = this.v2l;
		let v3h = this.v3h;
		let v3l = this.v3l;
		let h: number;
		let l: number;
		for (let i = 0; i < n; i++) {
			// v0 += v1; v1 = rotl(v1, 13) ^ v0; v0 = rotl(v0, 32)
			l = (v0l >>> 0) + (v1l >>> 0);
			v0h = (v0h + v1h + (l > 0xffffffff ? 1 : 0)) | 0;
			v0l = l | 0;
			h = v1h;
			v1h = ((h << 13) | (v1l >>> 19)) ^ v0h;
			v1l = ((v1l << 13) | (h >>> 19)) ^ v0l;
			h = v0h;
			v0h = v0l;
			v0l = h;
			// v2 += v3; v3 = rotl(v3, 16) ^ v2
			l = (v2l >>> 0) + (v3l >>> 0);
			v2h = (v2h + v3h + (l > 0xffffffff ? 1 : 0)) | 0;
			v2l = l | 0;
			h = v3h;
			v3h = ((h << 16) | (v3l >>> 16)) ^ v2h;
			v3l = ((v3l << 16) | (h >>> 16)) ^ v2l;
			// v0 += v3; v3 = rotl(v3, 21) ^ v0
			l = (v0l >>> 0) + (v3l >>> 0);
			v0h = (v0h + v3h + (l > 0xffffffff ? 1 : 0)) | 0;
			v0l = l | 0;
			h = v3h;
			v3h = ((h << 21) | (v3l >>> 11)) ^ v0h;
			v3l = ((v3l << 21) | (h >>> 11)) ^ v0l;
			// v2 += v1; v1 = rotl(v1, 17) ^ v2; v2 = rotl(v2, 32)
			l = (v2l >>> 0) + (v1l >>> 0);
			v2h = (v2h + v1h + (l > 0xffffffff ? 1 : 0)) | 0;
			v2l = l | 0;
			h = v1h;
			v1h = ((h << 17) | (v1l >>> 15)) ^ v2h;
			v1l = ((v1l << 17) | (h >>> 15)) ^ v2l;
			h = v2h;
			v2h = v2l;
			v2l = h;
		}
		this.v0h = v0h;
		this.v0l = v0l;
		this.v1h = v1h;
		this.v1l = v1l;
		this.v2h = v2h;
		this.v2l = v2l;
		this.v3h = v3h;
		this.v3l = v3l;
	}
}

/**
 * `RandomState`: the keys `k0` and `k1` of the SipHash-1-3 it builds, as 32-bit halves.
 * Never changed once made, so a clone is itself.
 */
export interface RandomState {
	readonly k0h: number;
	readonly k0l: number;
	readonly k1h: number;
	readonly k1l: number;
}

const UNKEYED: RandomState = { k0h: 0, k0l: 0, k1h: 0, k1l: 0 };

/** Where `finish` runs its last rounds, which leave the hasher as it was. */
const LAST = /* @__PURE__ */ new SipHasher(UNKEYED);

/** `DefaultHasher::new`: SipHash-1-3 keyed by zeros. */
export function hasher(): Hasher {
	return new SipHasher(UNKEYED);
}

/** This thread's keys, `k0` and `k1` as 32-bit halves, high first, once a `RandomState` is made. */
let keys: Int32Array | undefined;

/**
 * `RandomState::new`: this thread's keys, drawn once from `crypto.getRandomValues` where
 * std asks the OS, then `k0` raised by one for each state, as std's is.
 */
export function randomState(): RandomState {
	keys ??= crypto.getRandomValues(new Int32Array(4));
	const state = { k0h: keys[0], k0l: keys[1], k1h: keys[2], k1l: keys[3] };
	keys[1] = (keys[1] + 1) | 0;
	if (keys[1] === 0) keys[0] = (keys[0] + 1) | 0;
	return state;
}

/** `BuildHasher::build_hasher` of a `RandomState`. */
export function buildHasher(state: RandomState): Hasher {
	return new SipHasher(state);
}

/**
 * `BuildHasher::hash_one`: `value`'s hash by its `Hash` into `s`, the hasher built for it,
 * then the crate's own hasher drops, and `value` after it, as std's locals do.
 */
export function hashOne<T, H>(
	s: Hasher,
	value: T,
	hash: (value: T, s: Hasher) => void,
	drop?: (value: T) => void,
	dropHasher?: (h: H) => void,
): u64 {
	hash(value, s);
	const h = s.finish();
	dropHasher?.((s as OwnHasher<H>).h);
	drop?.(value);
	return h;
}

/**
 * The crate's own `Hasher`'s methods, each taking the `&mut` to it first: `write` and
 * `finish`, and each of the others its impl writes itself.
 */
export interface HasherImpl<H> {
	readonly write: (h: H, bytes: ByteSlice) => void;
	readonly finish: (h: H) => u64;
	readonly u8?: (h: H, value: u8) => void;
	readonly u16?: (h: H, value: u16) => void;
	readonly u32?: (h: H, value: u32) => void;
	readonly u64?: (h: H, value: u64) => void;
	readonly u128?: (h: H, value: u128) => void;
	readonly usize?: (h: H, value: usize) => void;
	readonly i8?: (h: H, value: i8) => void;
	readonly i16?: (h: H, value: i16) => void;
	readonly i32?: (h: H, value: i32) => void;
	readonly i64?: (h: H, value: i64) => void;
	readonly i128?: (h: H, value: i128) => void;
	readonly isize?: (h: H, value: isize) => void;
	readonly prefix?: (h: H, len: usize) => void;
	readonly str?: (h: H, str: string) => void;
}

/**
 * The crate's own `Hasher`, `h` its `&mut`: each write calls its impl's method, or std's
 * provided one, which writes through the others as std's does.
 */
export class OwnHasher<H> implements Hasher {
	readonly h: H;
	private readonly m: HasherImpl<H>;

	constructor(h: H, m: HasherImpl<H>) {
		this.h = h;
		this.m = m;
	}

	write(data: Uint8Array, start: number, len: number): void {
		this.m.write(this.h, new ByteSlice(data, start, len));
	}

	u8(value: number): void {
		if (this.m.u8) this.m.u8(this.h, value);
		else this.write(Uint8Array.of(value), 0, 1);
	}

	u16(value: number): void {
		if (this.m.u16) this.m.u16(this.h, value);
		else this.ne(value, 2);
	}

	u32(value: number): void {
		if (this.m.u32) this.m.u32(this.h, value);
		else this.ne(value, 4);
	}

	u64(value: number | Limbs): void {
		if (this.m.u64) this.m.u64(this.h, value as u64);
		else this.ne(value, 8);
	}

	u128(value: number | Limbs): void {
		if (this.m.u128) this.m.u128(this.h, value as u128);
		else this.ne(value, 16);
	}

	usize(value: number): void {
		if (this.m.usize) this.m.usize(this.h, value);
		else this.ne(value, 4);
	}

	i8(value: number): void {
		if (this.m.i8) this.m.i8(this.h, value);
		else this.u8(value & 0xff);
	}

	i16(value: number): void {
		if (this.m.i16) this.m.i16(this.h, value);
		else this.u16(value & 0xffff);
	}

	i32(value: number): void {
		if (this.m.i32) this.m.i32(this.h, value);
		else this.u32(value >>> 0);
	}

	i64(value: number | Limbs): void {
		if (this.m.i64) this.m.i64(this.h, value as i64);
		else this.u64(u64Cast(value));
	}

	i128(value: number | Limbs): void {
		if (this.m.i128) this.m.i128(this.h, value as i128);
		else this.u128(u128Cast(value));
	}

	isize(value: number): void {
		if (this.m.isize) this.m.isize(this.h, value);
		else this.usize(value >>> 0);
	}

	prefix(len: number): void {
		if (this.m.prefix) this.m.prefix(this.h, len);
		else this.usize(len);
	}

	str(str: string): void {
		if (this.m.str) return this.m.str(this.h, str);
		const bytes = encoder.encode(str);
		this.write(bytes, 0, bytes.length);
		this.u8(0xff);
	}

	ints(values: ArrayLike<number | Limbs>, size: 1 | 2 | 4 | 8 | 16): void {
		const bytes = new Uint8Array(values.length * size);
		for (let i = 0; i < values.length; i++) le(bytes, i * size, values[i], size);
		this.write(bytes, 0, bytes.length);
	}

	finish(): u64 {
		return this.m.finish(this.h);
	}

	/** `write` of `value`'s `size` bytes, as `to_ne_bytes` makes them on wasm32. */
	private ne(value: number | Limbs, size: 2 | 4 | 8 | 16): void {
		const bytes = new Uint8Array(size);
		le(bytes, 0, value, size);
		this.write(bytes, 0, size);
	}
}

const encoder = /* @__PURE__ */ new TextEncoder();

/** Put the little-endian bytes of `value`, an integer `size` bytes wide, into `bytes` at `at`. */
function le(bytes: Uint8Array, at: number, value: number | Limbs, size: 1 | 2 | 4 | 8 | 16): void {
	const words = (size + 3) >> 2;
	for (let w = 0; w < words; w++) {
		let word: number;
		if (typeof value !== "number") word = w === 0 ? value.lo : w === words - 1 ? value.hi : w === 1 ? (value as U128Limbs).w1 : (value as U128Limbs).w2;
		else if (w === 0) word = value;
		else if (w === 1) word = Math.floor(value / 4294967296);
		else word = value < 0 ? -1 : 0;
		for (let i = 0; i < 4 && w * 4 + i < size; i++) bytes[at + w * 4 + i] = (word >>> (i * 8)) & 0xff;
	}
}

/** `Hash::hash` into the crate's own `Hasher` of what hashes by the crate's own `Hash`, which takes the `&mut` itself. */
export function hashOwn<T, H>(hash: (value: T, h: H) => void): (value: T, s: Hasher) => void {
	return (value, s) => hash(value, (s as OwnHasher<H>).h);
}

/** A hash collection's own hasher: a running hash of the words written, as a JS map's key. */
class Mixer implements Hasher, Words {
	h = 0;

	word(word: number): void {
		this.h = Math.imul(((this.h << 5) | (this.h >>> 27)) ^ word, 0x9e3779b1);
	}

	write(data: Uint8Array, start: number, len: number): void {
		for (let i = 0; i < len; i++) this.word(data[start + i]);
	}

	u8(value: number): void {
		this.word(value);
	}

	u16(value: number): void {
		this.word(value);
	}

	u32(value: number): void {
		this.word(value);
	}

	u64(value: number | Limbs): void {
		words64(this, value);
	}

	u128(value: number | Limbs): void {
		words128(this, value);
	}

	usize(value: number): void {
		this.word(value);
	}

	i8(value: number): void {
		this.word(value);
	}

	i16(value: number): void {
		this.word(value);
	}

	i32(value: number): void {
		this.word(value);
	}

	i64(value: number | Limbs): void {
		words64(this, value);
	}

	i128(value: number | Limbs): void {
		words128(this, value);
	}

	isize(value: number): void {
		this.word(value);
	}

	prefix(len: number): void {
		this.word(len);
	}

	ints(values: ArrayLike<number | Limbs>, size: 1 | 2 | 4 | 8 | 16): void {
		wordInts(this, values, size);
	}
	str(str: string): void {
		this.word(str.length);
		for (let i = 0; i < str.length; i++) this.word(str.charCodeAt(i));
	}

	finish(): u64 {
		return this.key() as u64;
	}

	/** A 30-bit integer, which V8 keeps unboxed. */
	key(): number {
		return (this.h ^ (this.h >>> 15)) & 0x3fffffff;
	}
}

/** `Hash::hash` of a `u8` or a `bool`. */
export function hash8(value: number | boolean, s: Hasher): void {
	s.u8(+value);
}

/** `Hash::hash` of a `u16`. */
export function hash16(value: number, s: Hasher): void {
	s.u16(value);
}

/** `Hash::hash` of a `u32` or a `char`. */
export function hash32(value: number, s: Hasher): void {
	s.u32(value);
}

/** `Hash::hash` of a `u64`. */
export function hash64(value: number | Limbs, s: Hasher): void {
	s.u64(value);
}

/** `Hash::hash` of a `u128`. */
export function hash128(value: number | Limbs, s: Hasher): void {
	s.u128(value);
}

/** `Hash::hash` of a `usize`. */
export function hashUsize(value: number, s: Hasher): void {
	s.usize(value);
}

/** `Hash::hash` of an `i8`, or of a discriminant of `Ordering`, whose `repr` it is. */
export function hashI8(value: number, s: Hasher): void {
	s.i8(value);
}

/** `Hash::hash` of an `i16`. */
export function hashI16(value: number, s: Hasher): void {
	s.i16(value);
}

/** `Hash::hash` of an `i32`. */
export function hashI32(value: number, s: Hasher): void {
	s.i32(value);
}

/** `Hash::hash` of an `i64`. */
export function hashI64(value: number | Limbs, s: Hasher): void {
	s.i64(value);
}

/** `Hash::hash` of an `i128`. */
export function hashI128(value: number | Limbs, s: Hasher): void {
	s.i128(value);
}

/** `Hash::hash` of an `isize`, or of the discriminant the derive writes of an enum that has no `repr`. */
export function hashIsize(value: number, s: Hasher): void {
	s.isize(value);
}

/** `Hasher::write_length_prefix`. */
export function hashPrefix(len: number, s: Hasher): void {
	s.prefix(len);
}

/** `Hasher::write` of a byte slice. */
export function hashWrite(bytes: ByteSlice, s: Hasher): void {
	s.write(bytes.data, bytes.start, bytes.length);
}

/** `Hash::hash` of a `str` or a `String`, which hash the same. */
export function hashStr(str: string | RsString, s: Hasher): void {
	s.str(typeof str === "string" ? str : str.v);
}

/** `Hash::hash` of what holds nothing, as `()` and `PhantomData`: nothing written. */
export function hashNothing(_value: undefined, _s: Hasher): void {}

/** `Hash::hash` of tuples, from each field's. */
export function hashTuple<T extends readonly unknown[]>(fields: readonly ((value: never, s: Hasher) => void)[]): (value: T, s: Hasher) => void {
	return (value, s) => {
		for (let i = 0; i < fields.length; i++) (fields[i] as (value: unknown, s: Hasher) => void)(value[i], s);
	};
}

/** `Hash::hash` of a slice: its length as a `usize`, then its elements' in order. */
export function hashSlice<T>(hash: (value: T, s: Hasher) => void): (items: Elements<T>, s: Hasher) => void {
	return (items, s) => {
		const [a, i, n] = [sliceData(items), sliceStart(items), sliceLength(items)];
		s.prefix(n);
		for (let k = 0; k < n; k++) hash(a[i + k], s);
	};
}

/**
 * `Hash::hash` of a slice of integers `size` bytes wide: its length as a `usize`, then
 * their bytes in one `write`, as their `hash_slice` writes them.
 */
export function hashInts(size: 1 | 2 | 4 | 8 | 16): (items: Elements<number | Limbs>, s: Hasher) => void {
	return (items, s) => {
		s.prefix(sliceLength(items));
		s.ints(items instanceof Slice ? sliceItems(items) : sliceData(items), size);
	};
}

/** `Hash::hash` of a `LinkedList`: its length as a `usize`, then its items' in order. */
export function hashList<T>(hash: (value: T, s: Hasher) => void): (value: LinkedList<T>, s: Hasher) => void {
	return (value, s) => {
		s.prefix(value.n);
		for (let node = value.h; node !== undefined; node = node.x) hash(node.v, s);
	};
}

/**
 * `Hash::hash` of a byte array, slice, or vector, which is alike in each form: its length
 * as a `usize`, then its bytes.
 */
export function hashBytes(value: Uint8Array | ByteSlice | ByteVec, s: Hasher): void {
	const data = value instanceof Uint8Array ? value : value.data;
	const start = value instanceof ByteSlice ? value.start : 0;
	s.prefix(value.length);
	s.write(data, start, value.length);
}

/** `Hash::hash` of an `Option`: its discriminant as an `isize`, then its value's. */
export function hashOption<T>(hash: (value: T, s: Hasher) => void): (value: T | undefined, s: Hasher) => void;
export function hashOption<T>(hash: (value: T, s: Hasher) => void, none: None): (value: T | None, s: Hasher) => void;
export function hashOption<T>(
	hash: (value: T, s: Hasher) => void,
	none?: None,
): (value: T | undefined | None, s: Hasher) => void {
	return (value, s) => {
		s.isize(value === none ? 0 : 1);
		if (value !== none) hash(value as T, s);
	};
}

/** `Hash::hash` of an `Arc`: its value's. */
export function hashArc<T>(hash: (value: T, s: Hasher) => void): (value: Arc<T>, s: Hasher) => void {
	return (value, s) => hash(value.value, s);
}

/** `Hasher::finish`. */
export function hashFinish(s: Hasher): u64 {
	return s.finish();
}

/** The JS map key a hash collection files a key that is a JS object under: its hash, or its text. */
export type HashKey = number | string;

/** How a hash collection finds a key that is a JS object: its `Hash` and `Eq`. */
export interface Hashing<K> {
	readonly hash: (key: K) => HashKey;
	readonly eq: (a: K, b: K) => boolean;
	/** Whether a key that is a JS primitive goes by them too: an enum with its own `Eq`. */
	readonly all: boolean;
}

/** The `Hashing` of a key type, from its `Hash::hash` and `PartialEq::eq`. */
export function hashing<K>(hash: (key: K, s: Hasher) => void, eq: (a: K, b: K) => boolean, all = false): Hashing<K> {
	return {
		hash: (key) => {
			const s = new Mixer();
			hash(key, s);
			return s.key();
		},
		eq,
		all,
	};
}
