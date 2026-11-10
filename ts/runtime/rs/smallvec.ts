// smallvec's `SmallVec`, for rust2-ts output: an `rs.Vec`, whose capacity follows
// smallvec's rather than std's. Its `cap` is smallvec's capacity while it is spilled, past
// `n`, the most it holds inline, and 0 while inline, and its `peak` is the most it held
// since, where a step shortened it. A push past the capacity grows it to the next power of
// two, as smallvec's does, so reading one rounds the most it held up to that, and a push
// reads and writes neither. Each function takes `n`, `usize::MAX` of zero-sized elements,
// which never spill, and those that allocate take `size`, the bytes of an element.
// `rs.ts` re-exports all of it.

import { panic, type Slice, Vec, vecCapacity, vecFrom, vecHold, vecKeep, vecMax } from "../rs.ts";
import { type Bounds, bounds, vecDrain } from "./collections.ts";
import { type Iter, iterExtend, type Take } from "./iter.ts";
import type { Location } from "./location.ts";

/** wasm32's `usize::MAX`. */
const USIZE_MAX = 0xffffffff;

/** The least power of two at least `n`, or `undefined` past `usize::MAX`, as `usize::checked_next_power_of_two`. */
function nextPow2(n: number): number | undefined {
	let p = 1;
	while (p < n) p *= 2;
	return p > USIZE_MAX ? undefined : p;
}

/** `SmallVec::capacity` of one holding `n` inline. */
export function smallVecCapacity<T>(v: Vec<T>, n: number): number {
	const base = v.cap > n ? v.cap : n;
	const most = v.peak > v.items.length ? v.peak : v.items.length;
	return most > base ? (nextPow2(most) as number) : base;
}

/** `SmallVec::spilled`: whether its capacity is past what it holds inline. */
export function smallVecSpilled<T>(v: Vec<T>, n: number): boolean {
	return smallVecCapacity(v, n) > n;
}

/** `v`'s capacity set to `cap`, inline at or under `n`. */
function hold<T>(v: Vec<T>, cap: number, n: number): void {
	vecHold(v, cap > n ? cap : 0);
}

/** `SmallVec::try_grow`: `false` where the layout of `newCap` `size`-byte elements overflows. */
export function smallVecTryGrow<T>(v: Vec<T>, newCap: number, n: number, size: number): boolean {
	const cap = smallVecCapacity(v, n);
	if (newCap < v.items.length) return true;
	if (newCap <= n) {
		// Back inline, if it spilled.
		if (cap > n) hold(v, 0, n);
		return true;
	}
	if (newCap === cap) return true;
	if (newCap > vecMax(size)) return false;
	hold(v, newCap, n);
	return true;
}

/** smallvec's `infallible` of a `try_grow`: its panic of an overflow. */
function grown(ok: boolean, at?: Location): void {
	if (!ok) panic("capacity overflow", at);
}

/** `SmallVec::grow`, to `newCap` `size`-byte elements, which panics at `at` where their layout overflows. */
export function smallVecGrow<T>(v: Vec<T>, newCap: number, n: number, size: number, at?: Location): void {
	grown(smallVecTryGrow(v, newCap, n, size), at);
}

/** `SmallVec::try_reserve`, or `try_reserve_exact` if `exact`: room for `additional` more, or `false` past what it may hold. */
export function smallVecTryReserve<T>(v: Vec<T>, additional: number, n: number, size: number, exact: boolean): boolean {
	const len = v.items.length;
	if (smallVecCapacity(v, n) - len >= additional) return true;
	const want = len + additional > USIZE_MAX ? undefined : len + additional;
	const newCap = want === undefined || exact ? want : nextPow2(want);
	return newCap !== undefined && smallVecTryGrow(v, newCap, n, size);
}

/** `SmallVec::reserve`, or `reserve_exact` if `exact`: `try_reserve`'s room, panicking at `at` where it fails. */
export function smallVecReserve<T>(v: Vec<T>, additional: number, n: number, size: number, exact: boolean, at?: Location): void {
	grown(smallVecTryReserve(v, additional, n, size, exact), at);
}

/** `SmallVec::with_capacity`: a `reserve_exact` of `cap` from empty. */
export function smallVecWithCapacity<T>(cap: number, n: number, size: number, at?: Location): Vec<T> {
	const v = new Vec<T>();
	smallVecReserve(v, cap, n, size, true, at);
	return v;
}

/** `SmallVec::shrink_to_fit`: back inline where its items fit, or else to exactly them. */
export function smallVecShrinkToFit<T>(v: Vec<T>, n: number, size: number): void {
	const cap = smallVecCapacity(v, n);
	if (cap <= n) return;
	const len = v.items.length;
	if (len <= n) hold(v, 0, n);
	else if (cap > len) smallVecTryGrow(v, len, n, size);
}

/** `SmallVec::from_vec`: inline where the `Vec`'s capacity of `size`-byte elements fits, or else with it. */
export function smallVecFromVec<T>(v: Vec<T>, n: number, size: number): Vec<T> {
	hold(v, vecCapacity(v, size), n);
	return v;
}

/** `SmallVec::into_vec`: its storage, where it spilled, or else a `Vec` collected from its items. */
export function smallVecIntoVec<T>(v: Vec<T>, n: number, size: number): Vec<T> {
	const cap = smallVecCapacity(v, n);
	const len = v.items.length;
	// std's `collect` of an iterator it can't trust takes room for the first item and the
	// rest its hint gives, at least its smallest nonzero capacity.
	vecHold(v, cap > n ? cap : len === 0 ? 0 : Math.max(size === 1 ? 8 : size <= 1024 ? 4 : 1, len));
	return v;
}

/** `SmallVec::into_inner`: its items as the array, where it holds `n` of them inline, or else itself. */
export function smallVecIntoInner<T>(v: Vec<T>, n: number, inline: number): T[] | undefined {
	return smallVecCapacity(v, n) <= n && v.items.length === inline ? v.items : undefined;
}

/** `SmallVec::from_buf_and_len`: the first `len` items of `buf`, which leaks the rest, as a `MaybeUninit` does. */
export function smallVecFromBufAndLen<T>(buf: T[], len: number, inline: number, at?: Location): Vec<T> {
	if (len > inline) panic("assertion failed: len <= A::size()", at);
	buf.length = len;
	return new Vec(buf);
}

/** `SmallVec::from_slice` of `Copy` items, each copied by `copy` where JS would share it: inline where they fit, or else in a `Vec` of exactly them. */
export function smallVecFromSlice<T>(s: Slice<T>, n: number, copy?: (value: T) => T): Vec<T> {
	const items = s.data.slice(s.start, s.start + s.length);
	const v = vecFrom(copy === undefined ? items : items.map(copy));
	hold(v, v.items.length, n);
	return v;
}

/** `SmallVec::from_elem`: `count` clones of `elem`, which drops after, or a `Vec`'s, the last of which is `elem`, past `n`. */
export function smallVecFromElem<T>(elem: T, count: number, n: number, clone: ((value: T) => T) | undefined, drop?: (item: T) => void): Vec<T> {
	const items: T[] = [];
	if (count > n) {
		for (let i = 1; i < count; i++) items.push(clone === undefined ? elem : clone(elem));
		items.push(elem);
		return new Vec(items, count);
	}
	for (let i = 0; i < count; i++) items.push(clone === undefined ? elem : clone(elem));
	drop?.(elem);
	return new Vec(items);
}

/** `Clone::clone`, and `From<&[T]>`: each of `items` cloned in order, collected as `FromIterator` does. */
export function smallVecCloned<T>(items: T[], clone?: (value: T) => T): Vec<T> {
	return new Vec(clone === undefined ? items.slice() : items.map(clone));
}

/** `SmallVec::truncate`, and `clear` to 0: each item past `len` dropped from the back. */
export function smallVecTruncate<T>(v: Vec<T>, len: number, drop?: (item: T) => void): void {
	const d = v.items;
	if (len >= d.length) return;
	vecKeep(v);
	while (d.length > len) {
		const item = d.pop() as T;
		drop?.(item);
	}
}

/** `SmallVec::insert`, which grows it first where it is full, then panics at `at` past its length. */
export function smallVecInsert<T>(v: Vec<T>, i: number, item: T, n: number, size: number, at?: Location): void {
	const len = v.items.length;
	if (i > len) {
		if (len === smallVecCapacity(v, n)) grown(smallVecTryGrow(v, nextPow2(len + 1) ?? Infinity, n, size));
		panic("index exceeds length", at);
	}
	v.items.splice(i, 0, item);
}

/** `SmallVec::remove`, which asserts `i` is within its length at `at`. */
export function smallVecRemove<T>(v: Vec<T>, i: number, at?: Location): T {
	if (i >= v.items.length) panic("assertion failed: index < len", at);
	vecKeep(v);
	return v.items.splice(i, 1)[0];
}

/** `SmallVec::swap_remove`: the last item swapped into `i`, then popped, panicking as `len - 1` and the swap do at `at`. */
export function smallVecSwapRemove<T>(v: Vec<T>, i: number, at?: Location): T {
	const d = v.items;
	if (d.length === 0) panic("attempt to subtract with overflow", at);
	if (i >= d.length) panic(`index out of bounds: the len is ${d.length} but the index is ${i}`, at);
	vecKeep(v);
	const last = d.pop() as T;
	if (i === d.length) return last;
	const item = d[i];
	d[i] = last;
	return item;
}

/** `SmallVec::drain`: the items within `r`, whose ends smallvec checks itself, at `at`. */
export function smallVecDrain<T>(v: Vec<T>, r: Bounds<number>, drop?: (item: T) => void, at?: Location): Iter<T> {
	const [lo, hi] = bounds(r);
	const len = v.items.length;
	let start = 0;
	if (lo.tag !== "Unbounded") {
		if (lo.tag === "Excluded" && lo._0 === USIZE_MAX) panic("Range start out of bounds", at);
		start = lo.tag === "Excluded" ? lo._0 + 1 : lo._0;
	}
	let end = len;
	if (hi.tag !== "Unbounded") {
		if (hi.tag === "Included" && hi._0 === USIZE_MAX) panic("Range end out of bounds", at);
		end = hi.tag === "Included" ? hi._0 + 1 : hi._0;
	}
	if (start > end) panic("assertion failed: start <= end", at);
	if (end > len) panic("assertion failed: end <= len", at);
	return vecDrain(v, { start, end }, drop);
}

/** `SmallVec::extend_from_slice` of `Copy` items, and `insert_from_slice` at `i`: room for them, then them. */
export function smallVecInsertFromSlice<T>(v: Vec<T>, i: number, s: Slice<T>, n: number, size: number, copy: ((value: T) => T) | undefined, at?: Location): void {
	smallVecReserve(v, s.length, n, size, false);
	const d = v.items;
	if (i > d.length) panic("assertion failed: index <= len", at);
	// Moved up by a loop rather than a spread, which an engine caps the arguments of.
	const k = s.length;
	const len = d.length;
	d.length = len + k;
	for (let j = len - 1; j >= i; j--) d[j + k] = d[j];
	for (let j = 0; j < k; j++) {
		const x = s.data[s.start + j];
		d[i + j] = copy === undefined ? x : copy(x);
	}
}

/** `Extend::extend`, and `collect` into a new one: room for the lower bound of `items`' hint, as smallvec reserves it, then each item by `next`. */
export function smallVecExtend<T>(v: Vec<T>, items: Iter<T>, n: number, size: number, take: Take): Vec<T> {
	smallVecReserve(v, items.sizeHint()[0], n, size, false);
	return iterExtend(v, items, take);
}

/** `SmallVec::append`: `other`'s items moved onto the end of `v`, as its `extend` of their drain. */
export function smallVecAppend<T>(v: Vec<T>, other: Vec<T>, n: number, size: number): void {
	vecKeep(other);
	const items = other.items.splice(0);
	smallVecReserve(v, items.length, n, size, false);
	for (const item of items) v.items.push(item);
}

/** `SmallVec::resize`: the items past `len` dropped from the back, or else clones of `value` up to it, which then drops. */
export function smallVecResize<T>(v: Vec<T>, len: number, value: T, n: number, size: number, clone: ((value: T) => T) | undefined, drop?: (item: T) => void): void {
	const old = v.items.length;
	if (len <= old) {
		smallVecTruncate(v, len, drop);
		drop?.(value);
		return;
	}
	smallVecReserve(v, len - old, n, size, false);
	for (let i = old; i < len; i++) v.items.push(clone === undefined ? value : clone(value));
	drop?.(value);
}

/** `SmallVec::resize_with`: the items past `len` dropped from the back, or else what `f` makes up to it. */
export function smallVecResizeWith<T>(v: Vec<T>, len: number, f: () => T, n: number, size: number, drop?: (item: T) => void): void {
	const old = v.items.length;
	if (len < old) return smallVecTruncate(v, len, drop);
	if (len === old) return;
	smallVecReserve(v, len - old, n, size, false);
	for (let i = old; i < len; i++) v.items.push(f());
}
