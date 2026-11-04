// `bytes`, for rust2-ts output. A `Bytes` is immutable bytes that its clones and slices
// share, as a `&[u8]` shares them, so it is an `rt.ByteSlice`: an `rt.CountedBytes` where
// it holds storage, which counts its handles as `bytes` does, and a plain one where it is
// static. A `BytesMut` is a window of a vector of its own, or of storage that the handles
// `split_to` makes and the `Bytes` that `freeze` makes share without overlapping. Whether
// a handle is the last of its storage decides `bytes`' capacities, so every handle drops.
// `rt.ts` re-exports all of it.

import {
	ByteSlice,
	ByteVec,
	byteSliceToByteVec,
	bytes,
	byteVecExtendFromSlice,
	byteVecPush,
	byteVecReserve,
	CountedBytes,
	capReserve,
	copyBytes,
	type FmtResult,
	type Formatter,
	fmtWriteStr,
	type Place,
	panic,
	type RangeInclusive,
	type Result,
	Err,
	Ok,
} from "../rt.ts";
import { type char, type u8, u8LowerHex, type usize } from "./int.ts";
import type { Location } from "./location.ts";

/** `isize::MAX` on wasm32, the most bytes an allocation holds. */
const ISIZE_MAX = 0x7fffffff;

/** `usize::MAX` on wasm32, past which `bytes`' own sums overflow. */
const USIZE_MAX = 0xffffffff;

/** The furthest a `bytes` vector reads past its front before it becomes shared storage, on wasm32. */
const MAX_VEC_POS = 0x7ffffff;

/** `bytes::Bytes`. */
export type Bytes = ByteSlice;

/**
 * The storage handles share once one was split or frozen, or a `Bytes` was made of a
 * vector: the vector, and how many live handles hold it.
 */
export class Shared {
	/** The vector's allocation, as long as its capacity. */
	data: Uint8Array;
	refs = 1;
	/**
	 * `bytes`' original capacity of the `BytesMut` it was made of, which `reserve` grows a
	 * handle that leaves it to at least, or `undefined` for a `Bytes`' own storage, which a
	 * `BytesMut` taking it makes a vector of its own again.
	 */
	readonly original: number | undefined;

	constructor(data: Uint8Array, original: number | undefined) {
		this.data = data;
		this.original = original;
	}
}

/** Whether `b` is the one handle of its storage, as `Bytes::is_unique` says. */
export function bytesIsUnique(b: Bytes): boolean {
	return b instanceof CountedBytes && b.shared.refs === 1;
}

/** A `Bytes` of the bytes `s` lends, which counts no storage, as `from_static` and `from_owner` make. */
export function bytesStatic(s: ByteSlice): Bytes {
	return s instanceof CountedBytes ? new ByteSlice(s.data, s.start, s.length) : s;
}

/** Drop a `Bytes`, which counts its storage down. */
export function bytesDrop(b: Bytes): void {
	if (b instanceof CountedBytes) b.shared.refs--;
}

/** `Bytes::clone`: the same bytes, counted once more, which is never promotable. */
export function bytesClone(b: Bytes): Bytes {
	if (!(b instanceof CountedBytes)) return b;
	b.shared.refs++;
	return b.promotable ? new CountedBytes(b.data, b.start, b.length, b.shared, false) : b;
}

/** A clone of `b` that is `length` bytes from `start` of its storage, as `bytes` slices one. */
function part(b: Bytes, start: number, length: number): Bytes {
	if (!(b instanceof CountedBytes)) return new ByteSlice(b.data, start, length);
	b.shared.refs++;
	return new CountedBytes(b.data, start, length, b.shared, false);
}

/** `b` itself, moved to `length` bytes from `start` of its storage. */
function moved(b: Bytes, start: number, length: number): Bytes {
	if (!(b instanceof CountedBytes)) return new ByteSlice(b.data, start, length);
	return new CountedBytes(b.data, start, length, b.shared, b.promotable);
}

/** `Bytes::new`. Its bytes are made each time, as a bundler keeps a constant object. */
export function bytesNew(): Bytes {
	return new ByteSlice(new Uint8Array(0), 0, 0);
}

/** `Bytes::copy_from_slice`: a copy, which later writes to `data` don't change, in a vector as long as it. */
export function bytesCopyFromSlice(data: ByteSlice): Bytes {
	if (data.length === 0) return bytesNew();
	const copy = data.data.slice(data.start, data.start + data.length);
	return new CountedBytes(copy, 0, data.length, new Shared(copy, undefined), true);
}

/** `From<Vec<u8>>`: its storage, promotable where the vector is as long as its capacity, and static where both are 0. */
export function bytesFromVec(v: ByteVec): Bytes {
	const whole = v.length === v.data.length;
	if (whole && v.length === 0) return bytesNew();
	return new CountedBytes(v.data, 0, v.length, new Shared(v.data, undefined), whole);
}

/** `From<Bytes> for Vec<u8>`: its storage's vector, if it is the one handle of it, and else a copy. */
export function bytesToByteVec(b: Bytes): ByteVec {
	if (!bytesIsUnique(b)) {
		bytesDrop(b);
		return byteSliceToByteVec(b);
	}
	const s = (b as CountedBytes).shared;
	s.refs--;
	return vecOf(s.data, b.start, b.length);
}

/** A vector of the allocation `data`, its `length` bytes from `start` moved to its front. */
function vecOf(data: Uint8Array, start: number, length: number): ByteVec {
	data.copyWithin(0, start, start + length);
	const v = new ByteVec();
	v.data = data;
	v.length = length;
	return v;
}

/** `Bytes::try_into_mut`: a `BytesMut` of its storage if it is the one handle of it. */
export function bytesTryIntoMut(b: Bytes, at?: Location): Result<BytesMut, Bytes> {
	return bytesIsUnique(b) ? Ok(bytesMutFromBytes(b, at)) : Err(b);
}

/** `Bytes::slice` of `begin..end`, `end` defaulting to the length, panicking as `bytes` does. */
export function bytesSlice(b: Bytes, begin: usize, end: usize = b.length as usize, at?: Location): Bytes {
	if (begin > end) panic(`range start must not be greater than end: ${begin} <= ${end}`, at);
	if (end > b.length) panic(`range end out of bounds: ${end} <= ${b.length}`, at);
	if (begin === end) return new ByteSlice(b.data, b.start + begin, 0);
	return part(b, b.start + begin, end - begin);
}

/** `Bytes::slice` of `a..=b`; one exhausted by overflowing is empty, ending before its start. */
export function bytesSliceInclusive(b: Bytes, range: RangeInclusive<usize>, at?: Location): Bytes {
	if (range.exhausted) return bytesSlice(b, range.lo, range.lo, at);
	if (range.hi === 0xffffffff) panic("out of range", at);
	return bytesSlice(b, range.lo, (range.hi + 1) as usize, at);
}

/** `Bytes::slice` of `..=end`, which can't end past `usize::MAX`. */
export function bytesSliceToInclusive(b: Bytes, end: usize, at?: Location): Bytes {
	if (end === 0xffffffff) panic("out of range", at);
	return bytesSlice(b, 0 as usize, (end + 1) as usize, at);
}

/** `Debug`: a byte string literal, `b"\x00ab"`, written a piece at a time as `bytes` does. */
export function bytesDebug(b: Bytes, f: Formatter): FmtResult {
	let r = fmtWriteStr(f, 'b"');
	for (let i = 0; i < b.length && r.ok; i++) r = byteDebug(b.data[b.start + i] as u8, f);
	return r.ok ? fmtWriteStr(f, '"') : r;
}

/** One byte of `bytesDebug`, by the `write!` `bytes` makes of it. */
function byteDebug(byte: u8, f: Formatter): FmtResult {
	switch (byte) {
		case 0x0a:
			return f.out.writeStr("\\n");
		case 0x0d:
			return f.out.writeStr("\\r");
		case 0x09:
			return f.out.writeStr("\\t");
		case 0x5c:
		case 0x22: {
			const r = f.out.writeStr("\\");
			return r.ok ? f.out.writeChar(byte as number as char) : r;
		}
		case 0:
			return f.out.writeStr("\\0");
	}
	if (byte >= 0x20 && byte < 0x7f) return f.out.writeChar(byte as number as char);
	const r = f.out.writeStr("\\x");
	return r.ok ? u8LowerHex(byte, { out: f.out, spec: { zero: true, width: 2 } }) : r;
}

/** `Bytes::split_to`: the first `at` bytes, leaving the rest; all of it is the handle itself. */
export function bytesSplitTo(p: Place<Bytes>, pos: usize, at?: Location): Bytes {
	const b = p.v;
	if (pos === b.length) {
		p.v = new ByteSlice(b.data, b.start + pos, 0);
		return b;
	}
	if (pos === 0) return new ByteSlice(b.data, b.start, 0);
	if (pos > b.length) panic(`split_to out of bounds: ${pos} <= ${b.length}`, at);
	const head = part(b, b.start, pos);
	p.v = moved(b, b.start + pos, b.length - pos);
	return head;
}

/** `Bytes::split_off`: the bytes from `at` on, leaving the first `at`; all of it is the handle itself. */
export function bytesSplitOff(p: Place<Bytes>, pos: usize, at?: Location): Bytes {
	const b = p.v;
	if (pos === b.length) return new ByteSlice(b.data, b.start + pos, 0);
	if (pos === 0) {
		p.v = new ByteSlice(b.data, b.start, 0);
		return b;
	}
	if (pos > b.length) panic(`split_off out of bounds: ${pos} <= ${b.length}`, at);
	const tail = part(b, b.start + pos, b.length - pos);
	p.v = moved(b, b.start, pos);
	return tail;
}

/**
 * `Bytes::truncate`, and `clear` of 0. A promotable one splits off the rest and drops it,
 * so truncated to nothing, it is static and gives its storage up.
 */
export function bytesTruncate(p: Place<Bytes>, len: usize): void {
	const b = p.v;
	if (len >= b.length) return;
	if (len === 0 && b instanceof CountedBytes && b.promotable) {
		b.shared.refs--;
		p.v = new ByteSlice(b.data, b.start, 0);
		return;
	}
	p.v = moved(b, b.start, len);
}

/**
 * `Bytes::slice_ref`: the bytes of `b` that `subset` is, by where they are in memory. Its
 * panic names no addresses, which JS has none of. Bytes of other storage throw instead:
 * Rust may share what JS copied, such as a `&'static str`'s bytes, which `as_bytes`
 * encodes anew each time.
 */
export function bytesSliceRef(b: Bytes, subset: ByteSlice, at?: Location): Bytes {
	if (subset.length === 0) return bytesNew();
	if (subset.data.buffer !== b.data.buffer) {
		throw new Error("rust2-ts: `Bytes::slice_ref` of bytes from other storage, which Rust may share where JS copied");
	}
	const pos = subset.data.byteOffset + subset.start;
	const from = b.data.byteOffset + b.start;
	if (pos < from) panic("subset pointer is smaller than self pointer", at);
	if (pos + subset.length > from + b.length) panic("subset is out of bounds", at);
	return part(b, b.start + (pos - from), subset.length);
}

// Buf

/** `bytes`' panic for reading or advancing past what remains. */
function panicAdvance(requested: number, available: number, at?: Location): never {
	panic(`advance out of bounds: the len is ${available} but advancing by ${requested}`, at);
}

/** The `Buf` of `Bytes` or `BytesMut` advancing past what remains, which asserts. */
function panicRemaining(cnt: number, remaining: number, at?: Location): never {
	panic(`cannot advance past \`remaining\`: ${cnt} <= ${remaining}`, at);
}

/** `nbytes` read into an integer of `size` bytes, which holds at most that many. */
function checkFits(size: number, nbytes: number, at?: Location): void {
	if (nbytes > size) panic(`size too large: the integer type can fit ${size} bytes, but nbytes is ${nbytes}`, at);
}

/** `n` bytes from the front of `s`, the other way around if `le`. */
function readOf(s: ByteSlice, n: number, le: boolean): Uint8Array {
	const view = s.data.subarray(s.start, s.start + n);
	return le ? view.slice().reverse() : view;
}

/** The unsigned number of the `n` big-endian bytes of `s` from `i`, `n` at most 4. */
function wordOf(s: ByteSlice, n: number, le: boolean): number {
	let v = 0;
	for (let i = 0; i < n; i++) v = (v << 8) | s.data[s.start + (le ? n - 1 - i : i)];
	return n === 4 ? v >>> 0 : v;
}

/** `<Bytes as Buf>::advance`. */
export function bytesAdvance(p: Place<Bytes>, cnt: usize, at?: Location): void {
	const b = p.v;
	if (cnt > b.length) panicRemaining(cnt, b.length, at);
	p.v = moved(b, b.start + cnt, b.length - cnt);
}

/** `<&[u8] as Buf>::advance`, which panics as reads past the end do. */
export function byteSliceAdvance(p: Place<ByteSlice>, cnt: usize, at?: Location): void {
	const b = p.v;
	if (cnt > b.length) panicAdvance(cnt, b.length, at);
	p.v = new ByteSlice(b.data, b.start + cnt, b.length - cnt);
}

/** `<&[u8] as Buf>::copy_to_bytes`: a copy of its first `len` bytes, read past. */
export function byteSliceCopyToBytes(p: Place<ByteSlice>, len: usize, at?: Location): Bytes {
	const b = p.v;
	if (len > b.length) panicAdvance(len, b.length, at);
	p.v = new ByteSlice(b.data, b.start + len, b.length - len);
	return bytesCopyFromSlice(new ByteSlice(b.data, b.start, len));
}

/**
 * `Buf::copy_to_bytes` as `bytes` provides it, of a buffer read through its own methods:
 * `BytesMut::put` of a `Take` of `len` of it into a new `BytesMut` of capacity `len`, which
 * isn't 0 once anything remains, so it reserves and copies chunk by chunk, then freezes.
 */
export function bufCopyToBytes<B>(
	src: B,
	len: usize,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void,
	at?: Location,
): Bytes {
	if (remaining(src) < len) panicAdvance(len, remaining(src), at);
	const out = bytesMutWithCapacity(len, at);
	let limit: number = len;
	// The `Take`'s `remaining`, which reads the buffer's each time.
	const left = () => Math.min(remaining(src), limit);
	if (left() === 0) return bytesMutFreeze(out);
	bytesMutReserve(out, left() as usize, at);
	while (left() !== 0) {
		const c = chunk(src);
		const s = c.length > limit ? new ByteSlice(c.data, c.start, limit) : c;
		bytesMutExtend(out, s, at);
		advance(src, s.length as usize);
		limit -= s.length;
	}
	return bytesMutFreeze(out);
}

/** `bytes::TryGetError`: what a `try_get_*` asked for, and what remained. */
interface TryGetError {
	requested: usize;
	available: usize;
}

/**
 * `Buf::try_get_*` of an integer of `size` bytes as `bytes` provides it, of a buffer read
 * through its own methods: an error if fewer remain, and else the integer `decode` makes of
 * its bytes, the other way around if `le`, read from its chunk and then advanced past, or
 * copied out by `copyToSlice` where the chunk holds fewer.
 */
export function bufTryGet<B, T>(
	src: B,
	size: usize,
	le: boolean,
	decode: (bytes: Uint8Array) => T,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void,
	copyToSlice: (src: B, dst: ByteSlice) => void,
): Result<T, TryGetError> {
	if (remaining(src) < size) return Err({ requested: size, available: remaining(src) });
	const c = chunk(src);
	let bytes: Uint8Array;
	if (c.length >= size) {
		bytes = c.data.slice(c.start, c.start + size);
		advance(src, size);
	} else {
		bytes = new Uint8Array(size);
		copyToSlice(src, new ByteSlice(bytes, 0, size));
	}
	return Ok(decode(le ? bytes.reverse() : bytes));
}

/** `Buf::get_*` as `bufTryGet` reads it, panicking where it fails. */
export function bufGet<B, T>(
	src: B,
	size: usize,
	le: boolean,
	decode: (bytes: Uint8Array) => T,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void,
	copyToSlice: (src: B, dst: ByteSlice) => void,
	at?: Location,
): T {
	const got = bufTryGet(src, size, le, decode, remaining, chunk, advance, copyToSlice);
	if (!got.ok) panicAdvance(got.error.requested, got.error.available, at);
	return got.value;
}

/** `Buf::get_*` of `Bytes`: its first `n` bytes, the other way around if `le`, read past. */
export function bytesRead(p: Place<Bytes>, n: usize, le: boolean, at?: Location): Uint8Array {
	const b = p.v;
	if (b.length < n) panicAdvance(n, b.length, at);
	p.v = moved(b, b.start + n, b.length - n);
	return readOf(b, n, le);
}

/** `Buf::get_*` of `Bytes` of at most 4 bytes, as an unsigned number. */
export function bytesReadWord(p: Place<Bytes>, n: usize, le: boolean, at?: Location): number {
	const b = p.v;
	if (b.length < n) panicAdvance(n, b.length, at);
	p.v = moved(b, b.start + n, b.length - n);
	return wordOf(b, n, le);
}

/**
 * `Buf::get_uint` or `get_int` of `Bytes`: `nbytes` as the big-endian bytes of an integer of
 * `size` bytes, sign-extended if `signed`.
 */
export function bytesReadInt(p: Place<Bytes>, size: usize, nbytes: usize, signed: boolean, le: boolean, at?: Location): Uint8Array {
	checkFits(size, nbytes, at);
	return widen(bytesRead(p, nbytes, le, at), size, signed);
}

/** `Buf::copy_to_slice` of `Bytes`. */
export function bytesCopyToSlice(p: Place<Bytes>, dst: ByteSlice, at?: Location): void {
	const b = p.v;
	if (b.length < dst.length) panicAdvance(dst.length, b.length, at);
	copyBytes(dst.data, dst.start, b.data, b.start, dst.length);
	p.v = moved(b, b.start + dst.length, b.length - dst.length);
}

/** Big-endian bytes as the low bytes of `size`, the rest zero, or copies of the sign if `signed`. */
function widen(read: Uint8Array, size: number, signed: boolean): Uint8Array {
	const out = new Uint8Array(size);
	if (signed && read.length > 0 && read[0] >= 0x80) out.fill(0xff);
	out.set(read, size - read.length);
	return out;
}

// BytesMut

/**
 * `bytes::BytesMut`: `length` bytes of `data` from `start`, with room for `cap`. While
 * `shared` is `undefined`, `data` is a vector of its own, as long as `start + cap`, which
 * it read `start` bytes past. Once split or frozen from, it is a window of `shared`'s
 * vector, which `start` is how far into.
 */
export class BytesMut implements Disposable {
	data: Uint8Array;
	start: number;
	length: number;
	cap: number;
	shared: Shared | undefined;
	/** `bytes`' original capacity of a vector of its own, which `shared` keeps once it has one. */
	original: number;

	constructor(data: Uint8Array, start: number, length: number, cap: number, shared: Shared | undefined, original: number) {
		this.data = data;
		this.start = start;
		this.length = length;
		this.cap = cap;
		this.shared = shared;
		this.original = original;
	}

	[Symbol.dispose](): void {
		if (this.shared !== undefined) this.shared.refs--;
	}
}

/**
 * `bytes`' original capacity of a vector of capacity `cap`, which it keeps to the power of
 * two at or below it, from 1 KiB to 64 KiB, or 0 below that.
 */
function originalOf(cap: number): number {
	const width = Math.min(32 - Math.clz32(cap >>> 10), 7);
	return width === 0 ? 0 : 2 ** (width + 9);
}

/** A `BytesMut` of the vector `data`, as long as its capacity, of which it holds the first `length` bytes. */
function ofVec(data: Uint8Array, length: number): BytesMut {
	return new BytesMut(data, 0, length, data.length, undefined, originalOf(data.length));
}

/** `BytesMut::with_capacity`, `new` of 0. */
export function bytesMutWithCapacity(capacity: usize, at?: Location): BytesMut {
	if (capacity > ISIZE_MAX) panic("capacity overflow", at);
	return ofVec(bytes(capacity), 0);
}

/** `BytesMut::zeroed`. */
export function bytesMutZeroed(len: usize, at?: Location): BytesMut {
	const b = bytesMutWithCapacity(len, at);
	b.length = len;
	return b;
}

/** A `BytesMut` of a copy of `s`: `From<&[u8]>`, `From<&str>`, and `Clone`. */
export function bytesMutFrom(s: ByteSlice, at?: Location): BytesMut {
	const b = bytesMutWithCapacity(s.length, at);
	copyBytes(b.data, 0, s.data, s.start, s.length);
	b.length = s.length;
	return b;
}

/**
 * `BytesMut::from` a `Bytes`: its storage, if it is the one handle of it, as far into it as
 * the `Bytes` was, and else a copy. A `Bytes`' own storage becomes a vector of its own
 * again, and storage a `BytesMut` shared stays shared.
 */
export function bytesMutFromBytes(s: Bytes, at?: Location): BytesMut {
	if (!bytesIsUnique(s)) {
		bytesDrop(s);
		return bytesMutFrom(s, at);
	}
	const shared = (s as CountedBytes).shared;
	const cap = shared.data.length - s.start;
	if (shared.original !== undefined) return new BytesMut(shared.data, s.start, s.length, cap, shared, 0);
	const b = ofVec(shared.data, s.start + s.length);
	advanceUnchecked(b, s.start);
	return b;
}

/** Its bytes, as `Deref`, `DerefMut`, `AsRef`, `AsMut`, and `Buf::chunk` lend them. */
export function bytesMutView(b: BytesMut): ByteSlice {
	return new ByteSlice(b.data, b.start, b.length);
}

/**
 * `BytesMut::freeze`: a `Bytes` of its bytes, which nothing writes again. A vector of its
 * own becomes a `Bytes`' own storage, promotable if it has no room to spare.
 */
export function bytesMutFreeze(b: BytesMut): Bytes {
	if (b.shared !== undefined) return new CountedBytes(b.data, b.start, b.length, b.shared, false);
	const whole = b.length === b.cap;
	if (whole && b.start + b.length === 0) return bytesNew();
	return new CountedBytes(b.data, b.start, b.length, new Shared(b.data, undefined), whole);
}

/** `From<BytesMut> for Vec<u8>`: its vector, if it is the one handle of it, and else a copy. */
export function bytesMutToByteVec(b: BytesMut): ByteVec {
	const s = b.shared;
	if (s !== undefined && s.refs !== 1) {
		s.refs--;
		return byteSliceToByteVec(bytesMutView(b));
	}
	if (s !== undefined) s.refs--;
	return vecOf(b.data, b.start, b.length);
}

/** `BytesMut::capacity`. */
export function bytesMutCapacity(b: BytesMut): usize {
	return b.cap as usize;
}

/** `b`, which held no other handle's storage, now the first of two handles of it. */
function share(b: BytesMut): Shared {
	if (b.shared !== undefined) {
		b.shared.refs++;
		return b.shared;
	}
	b.shared = new Shared(b.data, b.original);
	b.shared.refs = 2;
	return b.shared;
}

/** `b` read `count` bytes past, which a vector read past `MAX_VEC_POS` does by sharing its storage with no other handle. */
function advanceUnchecked(b: BytesMut, count: number): void {
	if (count === 0) return;
	if (b.shared === undefined && b.start + count > MAX_VEC_POS) b.shared = new Shared(b.data, b.original);
	b.start += count;
	b.length = Math.max(b.length - count, 0);
	b.cap -= count;
}

/**
 * `BytesMut::reserve`: room for `additional` more bytes. A vector grows as `Vec` does,
 * counting the bytes it read past; storage that other handles hold is copied out into a
 * vector of its own; storage no other handle holds any longer is reclaimed.
 */
export function bytesMutReserve(b: BytesMut, additional: usize, at?: Location): void {
	if (additional <= b.cap - b.length) return;
	reserveInner(b, additional, true, at);
}

/** `BytesMut::try_reclaim`: whether it has room for `additional` more bytes, once it made what room it could without allocating. */
export function bytesMutTryReclaim(b: BytesMut, additional: usize, at?: Location): boolean {
	return additional <= b.cap - b.length || reserveInner(b, additional, false, at);
}

/** `bytes`' `reserve_inner`: whether it made room for `additional` more bytes, allocating only if `allocate`. */
function reserveInner(b: BytesMut, additional: number, allocate: boolean, at?: Location): boolean {
	const len = b.length;
	const s = b.shared;
	if (s === undefined) {
		// Its bytes move to the front where that alone makes the room, and leaves it half empty.
		const off = b.start;
		if (b.cap - len + off >= additional && off >= len) {
			b.data.copyWithin(0, off, off + len);
			b.start = 0;
			b.cap += off;
			return true;
		}
		if (!allocate) return false;
		regrow(b, capReserve(off + len, off + b.cap, additional, 1));
		return true;
	}
	let cap = len + additional;
	if (cap > USIZE_MAX || (s.refs === 1 && cap + b.start > USIZE_MAX)) {
		if (!allocate) return false;
		panic("overflow", at);
	}
	if (s.refs === 1) {
		const room = s.data.length;
		const off = b.start;
		if (room >= cap + off) {
			b.cap = cap;
		} else if (room >= cap && off >= len) {
			s.data.copyWithin(0, off, off + len);
			b.start = 0;
			b.cap = room;
		} else {
			if (!allocate) return false;
			const want = Math.max(room * 2, cap + off);
			regrow(b, capReserve(off + len, room, want - off - len, 1));
			s.data = b.data;
		}
		return true;
	}
	if (!allocate) return false;
	cap = Math.max(cap, s.original as number);
	if (cap > ISIZE_MAX) panic("capacity overflow", at);
	const data = bytes(cap);
	copyBytes(data, 0, b.data, b.start, len);
	s.refs--;
	b.data = data;
	b.start = 0;
	b.cap = cap;
	b.shared = undefined;
	b.original = s.original as number;
	return true;
}

/** `b`'s vector moved into an allocation of `capacity` bytes, its bytes where they were. */
function regrow(b: BytesMut, capacity: number): void {
	const data = bytes(capacity);
	copyBytes(data, b.start, b.data, b.start, b.length);
	b.data = data;
	b.cap = capacity - b.start;
}

/** `b` made `other`, as `*self = other` makes it, dropping what it was. */
function replace(b: BytesMut, other: BytesMut): void {
	b[Symbol.dispose]();
	b.data = other.data;
	b.start = other.start;
	b.length = other.length;
	b.cap = other.cap;
	b.shared = other.shared;
	b.original = other.original;
}

/** What `BufMut::put` of a `BytesMut` of no capacity does with the `Bytes` it copied out: takes its storage if no other handle holds it. */
function putTaken(b: BytesMut, s: Bytes, at?: Location): void {
	if (bytesIsUnique(s)) return replace(b, bytesMutFromBytes(s, at));
	bytesMutExtend(b, s, at);
	bytesDrop(s);
}

/**
 * `BufMut::put` of a `Bytes` into a `BytesMut`, which drops it. At zero capacity, `bytes`
 * copies it out whole, which is itself, and takes its storage if it is the one handle of it.
 */
export function bytesMutPutStorage(b: BytesMut, s: Bytes, at?: Location): void {
	if (s.length === 0) return bytesDrop(s);
	if (b.cap === 0) return putTaken(b, s, at);
	bytesMutExtend(b, s, at);
	bytesDrop(s);
}

/**
 * `BufMut::put` of a `BytesMut` into a `BytesMut`, which drops it. At zero capacity, `bytes`
 * splits it all off and freezes that, then drops what is left before taking the storage.
 */
export function bytesMutPutMut(b: BytesMut, src: BytesMut, at?: Location): void {
	if (src.length !== 0 && b.cap === 0) {
		const s = bytesMutFreeze(bytesMutSplitTo(src, src.length as usize, at));
		src[Symbol.dispose]();
		return putTaken(b, s, at);
	}
	if (src.length !== 0) bytesMutExtend(b, bytesMutView(src), at);
	src[Symbol.dispose]();
}

/**
 * `BufMut::put` of a `&[u8]` into a `BytesMut`. At zero capacity, the provided
 * `copy_to_bytes` copies it into a vector as long as it, which it takes.
 */
export function bytesMutPutSlice(b: BytesMut, s: ByteSlice, at?: Location): void {
	if (s.length === 0) return;
	if (b.cap === 0) return replace(b, bytesMutFrom(s, at));
	bytesMutExtend(b, s, at);
}

/**
 * `BufMut::put` of a `Buf` read through its own methods into a `BytesMut`, which `drop`
 * drops: copied out whole by its `copy_to_bytes` at zero capacity, taking the copy's
 * storage if no other handle holds it once the buffer dropped, and else reserved for and
 * copied chunk by chunk.
 */
export function bytesMutPutBuf<B>(
	b: BytesMut,
	src: B,
	hasRemaining: (src: B) => boolean,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void,
	copyToBytes: (src: B, len: usize) => Bytes,
	drop?: (src: B) => void,
	at?: Location,
): void {
	if (!hasRemaining(src)) return drop?.(src);
	if (b.cap === 0) {
		const s = copyToBytes(src, remaining(src));
		drop?.(src);
		return putTaken(b, s, at);
	}
	bytesMutReserve(b, remaining(src), at);
	while (hasRemaining(src)) {
		const s = chunk(src);
		bytesMutExtend(b, s, at);
		advance(src, s.length as usize);
	}
	drop?.(src);
}

/** `BytesMut::extend_from_slice`, and `BufMut::put_slice`. */
export function bytesMutExtend(b: BytesMut, s: ByteSlice, at?: Location): void {
	bytesMutReserve(b, s.length as usize, at);
	copyBytes(b.data, b.start + b.length, s.data, s.start, s.length);
	b.length += s.length;
}

/** `BufMut::put_u8` and the other writes of a narrow integer: its low `n` bytes, big-endian unless `le`. */
export function bytesMutPutWord(b: BytesMut, v: number, n: number, le: boolean, at?: Location): void {
	bytesMutReserve(b, n as usize, at);
	const pos = b.start + b.length;
	for (let i = 0; i < n; i++) b.data[pos + (le ? i : n - 1 - i)] = v >>> (8 * i);
	b.length += n;
}

/** `BufMut::put_*` of the big-endian bytes of a wide integer or float, reversed if `le`. */
export function bytesMutPutBytes(b: BytesMut, be: Uint8Array, le: boolean, at?: Location): void {
	bytesMutReserve(b, be.length as usize, at);
	const pos = b.start + b.length;
	for (let i = 0; i < be.length; i++) b.data[pos + i] = be[le ? be.length - 1 - i : i];
	b.length += be.length;
}

/** `BufMut::put_uint` or `put_int`: the low `nbytes` of the big-endian bytes, which `bytes` checks against `size` the other way around. */
export function bytesMutPutInt(b: BytesMut, be: Uint8Array, nbytes: usize, le: boolean, at?: Location): void {
	if (nbytes > be.length) panic(`size too large: the integer type can fit ${nbytes} bytes, but nbytes is ${be.length}`, at);
	bytesMutPutBytes(b, be.subarray(be.length - nbytes), le, at);
}

/** `BufMut::put_bytes`: `cnt` copies of `val`. */
export function bytesMutPutFill(b: BytesMut, val: number, cnt: usize, at?: Location): void {
	bytesMutReserve(b, cnt, at);
	b.data.fill(val, b.start + b.length, b.start + b.length + cnt);
	b.length += cnt;
}

// The provided `BufMut::put` and `put_bytes` that a `&mut` or a `Box` doesn't forward,
// which write into the room `chunk_mut` makes, 64 bytes more once there is none, rather
// than reserving what they write as the buffers' own do.

/** A provided write of `cnt` bytes, which panics past `remaining_mut`. */
function checkRoom(length: number, cnt: number, at?: Location): void {
	if (cnt > ISIZE_MAX - length) panicAdvance(cnt, ISIZE_MAX - length, at);
}

/** `<BytesMut as BufMut>::chunk_mut`: how many bytes fit past the end, once it reserved 64 if none did. */
function bytesMutRoom(b: BytesMut, at?: Location): number {
	if (b.cap === b.length) bytesMutReserve(b, 64 as usize, at);
	return b.cap - b.length;
}

/** The provided `BufMut::put_bytes` of a `BytesMut`. */
export function bytesMutPutFillChunked(b: BytesMut, val: number, cnt: usize, at?: Location): void {
	checkRoom(b.length, cnt, at);
	for (let left = cnt as number; left > 0; ) {
		const n = Math.min(bytesMutRoom(b, at), left);
		b.data.fill(val, b.start + b.length, b.start + b.length + n);
		b.length += n;
		left -= n;
	}
}

/** The provided `BufMut::put` into a `BytesMut` of a buffer whose bytes are `s`. */
export function bytesMutPutChunked(b: BytesMut, s: ByteSlice, at?: Location): void {
	checkRoom(b.length, s.length, at);
	for (let done = 0; done < s.length; ) {
		const n = Math.min(bytesMutRoom(b, at), s.length - done);
		copyBytes(b.data, b.start + b.length, s.data, s.start + done, n);
		b.length += n;
		done += n;
	}
}

/** The provided `BufMut::put` into a `BytesMut` of a `Buf` read through its own methods, each chunk cut to the room left. */
export function bytesMutPutBufChunked<B>(
	b: BytesMut,
	src: B,
	hasRemaining: (src: B) => boolean,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void,
	at?: Location,
): void {
	checkRoom(b.length, remaining(src), at);
	while (hasRemaining(src)) {
		const s = chunk(src);
		const n = Math.min(bytesMutRoom(b, at), s.length);
		copyBytes(b.data, b.start + b.length, s.data, s.start, n);
		b.length += n;
		advance(src, n as usize);
	}
}

/** `BytesMut::spare_capacity_mut`: the room past its bytes, a slice of its storage that a write through reaches. */
export function bytesMutSpare(b: BytesMut): ByteSlice {
	return new ByteSlice(b.data, b.start + b.length, b.cap - b.length);
}

/** `BytesMut::set_len`, which panics past its capacity as `bytes`' debug assertion does. */
export function bytesMutSetLen(b: BytesMut, len: usize, at?: Location): void {
	if (len > b.cap) panic("set_len out of bounds", at);
	b.length = len;
}

/** `<BytesMut as BufMut>::advance_mut`. */
export function bytesMutAdvanceMut(b: BytesMut, cnt: usize, at?: Location): void {
	const remaining = b.cap - b.length;
	if (cnt > remaining) panicAdvance(cnt, remaining, at);
	b.length += cnt;
}

/** `BytesMut::truncate`, and `clear` of 0. */
export function bytesMutTruncate(b: BytesMut, len: usize): void {
	if (len <= b.length) b.length = len;
}

/** `BytesMut::resize`. */
export function bytesMutResize(b: BytesMut, len: usize, value: number, at?: Location): void {
	if (len <= b.length) {
		b.length = len;
		return;
	}
	bytesMutPutFill(b, value, (len - b.length) as usize, at);
}

/** `BytesMut::split_to`: the first `at` bytes, sharing the storage, leaving the rest. */
export function bytesMutSplitTo(b: BytesMut, pos: usize, at?: Location): BytesMut {
	if (pos > b.length) panic(`split_to out of bounds: ${pos} <= ${b.length}`, at);
	const head = new BytesMut(b.data, b.start, pos, pos, share(b), 0);
	advanceUnchecked(b, pos);
	return head;
}

/** `BytesMut::split_off`: what it holds and has room for from `at` on, sharing the storage, leaving the rest. */
export function bytesMutSplitOff(b: BytesMut, pos: usize, at?: Location): BytesMut {
	if (pos > b.cap) panic(`split_off out of bounds: ${pos} <= ${b.cap}`, at);
	const tail = new BytesMut(b.data, b.start, b.length, b.cap, share(b), 0);
	advanceUnchecked(tail, pos);
	b.cap = pos;
	b.length = Math.min(b.length, pos);
	return tail;
}

/**
 * `BytesMut::try_unsplit`: whether `other` joined these bytes, as one handle of their
 * storage where it follows them in it, or as nothing where it has no room. Joined, it
 * dropped.
 */
function tryUnsplit(b: BytesMut, other: BytesMut): boolean {
	if (other.cap !== 0 && (b.shared === undefined || b.shared !== other.shared || b.start + b.length !== other.start)) {
		return false;
	}
	b.length += other.length;
	b.cap += other.cap;
	other[Symbol.dispose]();
	return true;
}

/** `BytesMut::try_unsplit`, giving `other` back where it doesn't follow these bytes in their storage. */
export function bytesMutTryUnsplit(b: BytesMut, other: BytesMut): Result<undefined, BytesMut> {
	return tryUnsplit(b, other) ? Ok(undefined) : Err(other);
}

/** `BytesMut::unsplit`: `other` itself after nothing, joined where it follows these bytes, and else copied after them. */
export function bytesMutUnsplit(b: BytesMut, other: BytesMut, at?: Location): void {
	if (b.length === 0) return replace(b, other);
	if (tryUnsplit(b, other)) return;
	bytesMutExtend(b, bytesMutView(other), at);
	other[Symbol.dispose]();
}

/** `<BytesMut as Buf>::advance`. */
export function bytesMutAdvance(b: BytesMut, cnt: usize, at?: Location): void {
	if (cnt > b.length) panicRemaining(cnt, b.length, at);
	advanceUnchecked(b, cnt);
}

/** `Buf::get_*` of `BytesMut`: its first `n` bytes, the other way around if `le`, read past. */
export function bytesMutRead(b: BytesMut, n: usize, le: boolean, at?: Location): Uint8Array {
	if (b.length < n) panicAdvance(n, b.length, at);
	const read = readOf(bytesMutView(b), n, le);
	bytesMutAdvance(b, n, at);
	return read;
}

/** `Buf::get_*` of `BytesMut` of at most 4 bytes, as an unsigned number. */
export function bytesMutReadWord(b: BytesMut, n: usize, le: boolean, at?: Location): number {
	if (b.length < n) panicAdvance(n, b.length, at);
	const v = wordOf(bytesMutView(b), n, le);
	bytesMutAdvance(b, n, at);
	return v;
}

/** `Buf::get_uint` or `get_int` of `BytesMut`, as `bytesReadInt`. */
export function bytesMutReadInt(b: BytesMut, size: usize, nbytes: usize, signed: boolean, le: boolean, at?: Location): Uint8Array {
	checkFits(size, nbytes, at);
	return widen(bytesMutRead(b, nbytes, le, at), size, signed);
}

/** `Buf::copy_to_slice` of `BytesMut`. */
export function bytesMutCopyToSlice(b: BytesMut, dst: ByteSlice, at?: Location): void {
	if (b.length < dst.length) panicAdvance(dst.length, b.length, at);
	copyBytes(dst.data, dst.start, b.data, b.start, dst.length);
	bytesMutAdvance(b, dst.length as usize, at);
}

// BufMut of Vec<u8>

// A write reserves what it writes, as `Vec::extend_from_slice` and `resize` do, so the
// capacity grows as `bytes`' does.

/** `BufMut::put` into a `Vec<u8>` of a `Buf` the crate implements, each chunk in turn. */
export function byteVecPutBuf<B>(
	v: ByteVec,
	src: B,
	hasRemaining: (src: B) => boolean,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void): void {
	byteVecReserve(v, remaining(src));
	while (hasRemaining(src)) {
		const s = chunk(src);
		byteVecExtendFromSlice(v, s);
		advance(src, s.length as usize);
	}
}

/** `BufMut::put_u8` and the other writes of a narrow integer to a `Vec<u8>`, as `bytesMutPutWord`. */
export function byteVecPutWord(v: ByteVec, value: number, n: number, le: boolean): void {
	byteVecReserve(v, n);
	for (let i = 0; i < n; i++) byteVecPush(v, value >>> (8 * (le ? i : n - 1 - i)));
}

/** `BufMut::put_*` of big-endian bytes to a `Vec<u8>`, reversed if `le`. */
export function byteVecPutBytes(v: ByteVec, be: Uint8Array, le: boolean): void {
	byteVecReserve(v, be.length);
	for (let i = 0; i < be.length; i++) byteVecPush(v, be[le ? be.length - 1 - i : i]);
}

/** `BufMut::put_uint` or `put_int` to a `Vec<u8>`, as `bytesMutPutInt`. */
export function byteVecPutInt(v: ByteVec, be: Uint8Array, nbytes: usize, le: boolean, at?: Location): void {
	if (nbytes > be.length) panic(`size too large: the integer type can fit ${nbytes} bytes, but nbytes is ${be.length}`, at);
	byteVecPutBytes(v, be.subarray(be.length - nbytes), le);
}

/** `BufMut::put_bytes` to a `Vec<u8>`: its `resize`, which panics past what a vector holds. */
export function byteVecPutFill(v: ByteVec, val: number, cnt: usize): void {
	byteVecReserve(v, cnt);
	v.data.fill(val, v.length, v.length + cnt);
	v.length += cnt;
}

/** `<Vec<u8> as BufMut>::advance_mut`. */
export function byteVecAdvanceMut(v: ByteVec, cnt: usize, at?: Location): void {
	const remaining = v.data.length - v.length;
	if (cnt > remaining) panicAdvance(cnt, remaining, at);
	v.length += cnt;
}

/** `<Vec<u8> as BufMut>::chunk_mut`: how many bytes fit past the end, once it reserved 64 if none did. */
function byteVecRoom(v: ByteVec): number {
	if (v.data.length === v.length) byteVecReserve(v, 64);
	return v.data.length - v.length;
}

/** The provided `BufMut::put_bytes` of a `Vec<u8>`. */
export function byteVecPutFillChunked(v: ByteVec, val: number, cnt: usize, at?: Location): void {
	checkRoom(v.length, cnt, at);
	for (let left = cnt as number; left > 0; ) {
		const n = Math.min(byteVecRoom(v), left);
		v.data.fill(val, v.length, v.length + n);
		v.length += n;
		left -= n;
	}
}

/** The provided `BufMut::put` into a `Vec<u8>` of a buffer whose bytes are `s`. */
export function byteVecPutChunked(v: ByteVec, s: ByteSlice, at?: Location): void {
	checkRoom(v.length, s.length, at);
	for (let done = 0; done < s.length; ) {
		const n = Math.min(byteVecRoom(v), s.length - done);
		copyBytes(v.data, v.length, s.data, s.start + done, n);
		v.length += n;
		done += n;
	}
}

/** The provided `BufMut::put` into a `Vec<u8>` of a `Buf` the crate implements, each chunk cut to the room left. */
export function byteVecPutBufChunked<B>(
	v: ByteVec,
	src: B,
	hasRemaining: (src: B) => boolean,
	remaining: (src: B) => usize,
	chunk: (src: B) => ByteSlice,
	advance: (src: B, cnt: usize) => void,
	at?: Location,
): void {
	if (remaining(src) > ISIZE_MAX - v.length) panicAdvance(remaining(src), ISIZE_MAX - v.length, at);
	while (hasRemaining(src)) {
		const s = chunk(src);
		const n = Math.min(byteVecRoom(v), s.length);
		copyBytes(v.data, v.length, s.data, s.start, n);
		v.length += n;
		advance(src, n as usize);
	}
}
