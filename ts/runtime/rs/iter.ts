// Iterators, for rust2-ts output: std's `Iterator`, its adaptors, and `Range`, lazy as
// Rust's are. `rs.ts` re-exports all of it.
//
// `next` returns an item or `DONE`, never `undefined`, so an iterator of `Option<T>` keeps
// its `None` items apart from its end. Each adaptor calls what it wraps, and the closures
// it was given, in the order and as many times as std's does, and stops where std's
// stops: a `find` or `any` reads no further than its match. Adaptors and consumers are
// functions, `iterMap(it, f)`, so a bundler drops the ones nothing calls; an iterator's
// methods are only what each kind does its own way.
//
// An iterator that owns its items, as a `Vec`'s `into_iter` does, is given what drops
// each, and drops those it still holds as it drops, by `[Symbol.dispose]`; an adaptor
// drops what it wraps. Where std drops an item it skips, rejects, or replaces, the runtime
// function is given what drops it, after its own arguments, and so is a consumer that
// takes the iterator by value, which drops it as std's does. Each kind's `nth`,
// `nth_back`, and `advance_by` are its own, as std's are, reading and dropping what they
// skip in its order. A consumer std folds reads what `folded` returns, which steps
// an inclusive range as its `try_fold` does, and by `nextBack` as its `try_rfold` does,
// as does each adaptor whose std's folds its iterator; `folded(true)` reads as `fold`
// does, which differs only for a zip that reads no more pairs than its size hint. Where
// std's `count` and `last` drop items in an order each kind of iterator has its own, the
// `owned` functions follow it, dispatching on the kind, so a bundle that never drops an
// item carries none of it.

import { ByteSlice, ByteVec, violated, type ArrayRef, type ArraySlice, arraySliceItems, arraySliceAt, arraySlicePart, byteVecExtendFromSlice, byteVecPush, byteVecReserve, byteVecReserveExact, byteVecShrinkTo, byteVecWithCapacity, capReserve, type Debug, Err, type Fmt, type FmtResult, type Formatter, fmtBool, fmtDebugStructFields, fmtDebugTupleFields, fmtOption, type None, Ok, type Place, panic, place, type RangeBound, type Result, sliceItems, slicePart, type Slice, type SliceLike, Vec, vecCapacity, vecExtendFromSlice, vecFrom, vecHold, vecMax, vecWithCapacity, assertFailed, fmtOwned } from "../rs.ts";
import { AtIndex, BTreeIter, HashIter, type Ordering } from "./collections.ts";
import { type Hasher, hash8 } from "./hash.ts";
import { charStep, intToBig, type Limbs, type Step, usizeDebug, lt as wideLt } from "./int.ts";
import { closure, drop as dispose, dropRange } from "./task.ts";
import type { Location } from "./location.ts";

/** What `next` returns once an iterator is exhausted, which no item is. */
export const DONE: unique symbol = /* @__PURE__ */ Symbol("DONE");

/** The type of `DONE`. */
export type Done = typeof DONE;

/** wasm32's `usize::MAX`, where std's adaptors saturate or split a count. */
const USIZE_MAX = 0xffffffff;

/** What drops a value. */
type Drop<T> = (value: T) => void;

/** `a + b`, saturating at `usize::MAX` as std's size hints do. */
export function satAdd(a: number, b: number): number {
	return Math.min(a + b, USIZE_MAX);
}

/** `a + b` of two upper bounds: `None` if either is, or if it passes `usize::MAX`, as std's `checked_add`. */
function addUpper(a: number | undefined, b: number | undefined): number | undefined {
	if (a === undefined || b === undefined) return undefined;
	const sum = a + b;
	return sum > USIZE_MAX ? undefined : sum;
}

/** `Iterator::size_hint`: at least `lower` items, and at most `upper`, if it is known. */
export type SizeHint = [lower: number, upper: number | undefined];

/** The size hint of an iterator that knows it has exactly `n` items. */
export function exactHint(n: number): SizeHint {
	return [n, n];
}

/**
 * std's default `ExactSizeIterator::len` of `it`: its size hint's lower bound, which
 * panics unless the upper bound is the same.
 */
function hintLen(it: Iter<unknown>): number {
	const [lower, upper] = it.sizeHint();
	if (upper !== lower) {
		const hint = (n: number | undefined, f: Formatter) => fmtOption(n, f, usizeDebug);
		assertFailed("==", fmtOwned(upper, hint), fmtOwned(lower, hint), undefined, "exact_len");
	}
	return lower;
}

/** std's hint of an adaptor that may yield fewer than `it`, and no more: none at least, and `it`'s upper bound. */
function fewer(it: Iter<unknown>): SizeHint {
	return [0, it.sizeHint()[1]];
}

/**
 * What only the iterators std implements these traits for have, which Rust calls only
 * there: declared, not stubbed, so no bundle carries a stub.
 */
export interface Iter<T> {
	/** `DoubleEndedIterator::next_back`. */
	nextBack(): T | Done;
	/** `ExactSizeIterator::len`. */
	len(): number;
	/** The items it has left, by value, without taking them: what std's `Debug` of it lists. */
	remaining(): Iterable<unknown>;
	/** `Clone::clone`, reading on from where it is, independently of it, by `h`'s clones. */
	clone(h?: Clones): Iter<T>;
}

/**
 * What clones the parts of one of std's iterators that it can't clone itself, as std's
 * `Clone` of each does: `i` and `o` the iterators it wraps, `f` its closure, and `v` an item
 * or another value it holds. A part without one clones as itself.
 */
export interface Clones {
	readonly i?: Clones;
	readonly o?: Clones;
	readonly f?: (f: never) => unknown;
	readonly v?: (value: never) => unknown;
}

/** `value` cloned by `clone`, or itself. */
function cloned<T>(value: T, clone: ((value: never) => unknown) | undefined): T {
	return clone === undefined ? value : (clone as (value: T) => T)(value);
}

/** `Iterator`: `next`, and the methods std's iterators each do their own way. */
export abstract class Iter<T> implements Iterable<T> {
	/** The next item, or `DONE`. */
	abstract next(): T | Done;

	/**
	 * `advance_by`: skip up to `n` items, each dropped by `drop`, and return how many it fell
	 * short. std's folds it, as `folded` steps it.
	 */
	advanceBy(n: number, drop?: Drop<T>): number {
		const folded = this.folded();
		for (let i = 0; i < n; i++) {
			const item = folded.next();
			if (item === DONE) return n - i;
			drop?.(item);
		}
		return 0;
	}

	/** `advance_back_by`, from the back. */
	advanceBackBy(n: number, drop?: Drop<T>): number {
		const folded = this.folded();
		for (let i = 0; i < n; i++) {
			const item = folded.nextBack();
			if (item === DONE) return n - i;
			drop?.(item);
		}
		return 0;
	}

	/** `nth`, as `next` returns it, dropping what it skips by `drop`. */
	nth(n: number, drop?: Drop<T>): T | Done {
		return this.advanceBy(n, drop) === 0 ? this.next() : DONE;
	}

	/** `nth_back`, as `next_back` returns it. */
	nthBack(n: number, drop?: Drop<T>): T | Done {
		return this.advanceBackBy(n, drop) === 0 ? this.nextBack() : DONE;
	}

	/**
	 * `find`, as `next` returns it, dropping what `f` rejects by `drop`: as `try_fold` reads
	 * it, but where its kind forwards std's `find` to an iterator it holds.
	 */
	find(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		return findItem(this, f, drop);
	}

	/** `rfind`, as `next_back` returns it, as `find`. */
	rfind(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		return rfindItem(this, f, drop);
	}

	/**
	 * Whether its `find`, `rfind`, or `find_map` may be the translated crate's own, which reads
	 * it other than as `try_fold` does.
	 */
	finds(): boolean {
		return false;
	}

	/** `find_map` of `f`, whose `None` is `none`, as `next` returns it: as `try_fold` reads it. */
	findMap<U>(f: (item: T) => U | undefined | None, none?: None): U | Done {
		const folded = this.folded();
		for (let item = folded.next(); item !== DONE; item = folded.next()) {
			const found = f(item);
			if (found !== none) return found as U;
		}
		return DONE;
	}

	/**
	 * What std's `try_fold` reads it by, and `try_rfold` by `nextBack`, or its `fold` and
	 * `rfold` if `fold`: itself, but for an inclusive range, which steps its own way, a zip
	 * whose `fold` stops at its size hint, and an adaptor that folds either.
	 */
	folded(_fold = false): Iter<T> {
		return this;
	}

	/**
	 * What std reads it by where it reads it by index, by `__iterator_get_unchecked`, from
	 * the front and the back, and whose `advanceBackBy` passes items it never reads: itself,
	 * but for an adaptor that maps, skips, or zips what may have side effects. Only an
	 * iterator std reads by index is read so.
	 */
	indexed(): Iter<T> {
		return this;
	}

	/** `size_hint`, as std's of its kind gives it: `(0, None)` unless it knows better. */
	sizeHint(): SizeHint {
		return [0, undefined];
	}

	/**
	 * `fold`, as std's of its kind, which takes the iterator by value and drops what it
	 * holds as std's does. `release` drops what `f` owns, as std drops the closure it was
	 * given: as its fold ends, before the iterator.
	 */
	fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		return foldBy(this, false, acc, f, release);
	}

	/** `rfold`, from the back, as `fold`. */
	rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		return foldBy(this, true, acc, f, release);
	}

	/** Its items, for a JS `for ... of`. */
	*[Symbol.iterator](): Iterator<T> {
		for (let item = this.next(); item !== DONE; item = this.next()) yield item;
	}
}

/**
 * std's default `fold`, or `rfold` if `back`: each item as `folded` steps it, from the
 * back if `back`, and then `release` and `it`.
 */
function foldBy<T, A>(it: Iter<T>, back: boolean, acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
	const folded = it.folded(true);
	for (;;) {
		const item = back ? folded.nextBack() : folded.next();
		if (item === DONE) break;
		acc = f(acc, item);
	}
	ended(it, release);
	return acc;
}

/**
 * The first step of a fold that reads an item by `read` before it folds the rest: whether
 * `it` ended, which drops `release` and then `it` as the fold returns, and the accumulator,
 * then `f` of the item.
 */
function foldFirst<T, A>(it: unknown, read: () => T | Done, acc: A, f: (acc: A, item: T) => A, release?: () => void): [boolean, A] {
	const item = read();
	if (item !== DONE) return [false, f(acc, item)];
	ended(it, release);
	return [true, acc];
}

/** `release`, then `it` dropped, as a fold's closure drops and then its iterator. */
function ended(it: unknown, release?: () => void): void {
	release?.();
	dispose(it);
}

/** What drops `a`'s and then `b`'s, as a closure that captured them in that order drops. */
function then(a: (() => void) | undefined, b: (() => void) | undefined): (() => void) | undefined {
	if (a === undefined) return b;
	if (b === undefined) return a;
	return () => {
		a();
		b();
	};
}

/** An adaptor of `inner`, which drops with it. */
abstract class Adaptor<S, T> extends Iter<T> implements Disposable {
	readonly inner: Iter<S>;

	constructor(inner: Iter<S>) {
		super();
		this.inner = inner;
	}

	[Symbol.dispose](): void {
		dispose(this.inner);
	}
}

/**
 * An adaptor of `inner` that calls `f`, which drops after it, as std's fields do, once:
 * a consumer may drop `inner` before dropping the adaptor that holds it.
 */
abstract class Calling<S, T, F> extends Adaptor<S, T> {
	protected readonly f: F;
	private dropped = false;

	constructor(inner: Iter<S>, f: F) {
		super(inner);
		this.f = f;
	}

	/** What drops `f`, which a fold of `inner` takes from it: dropping it then drops nothing. */
	protected take(): () => void {
		this.dropped = true;
		return () => dispose(this.f);
	}

	override [Symbol.dispose](): void {
		if (this.dropped) return;
		this.dropped = true;
		dispose(this.inner);
		dispose(this.f);
	}
}

/** An adaptor folded, whose `next` and `nextBack` are `front` and `back`, which share its state. */
export class Folded<T> extends Iter<T> {
	readonly front: () => T | Done;
	readonly back: () => T | Done;

	constructor(front: () => T | Done, back: () => T | Done) {
		super();
		this.front = front;
		this.back = back;
	}

	next(): T | Done {
		return this.front();
	}

	override nextBack(): T | Done {
		return this.back();
	}
}

/** An iterator as std reads it by index, whose `skip` passes `n` items from the back unread. */
class Indexed<T> extends Folded<T> {
	private readonly skip: (n: number) => void;

	constructor(front: () => T | Done, back: () => T | Done, skip: (n: number) => void) {
		super(front, back);
		this.skip = skip;
	}

	override advanceBackBy(n: number): number {
		this.skip(n);
		return 0;
	}
}

/** The item `next` returned, as an `Option`. */
export function iterToOption<T>(item: T | Done): T | undefined;
export function iterToOption<T>(item: T | Done, none: None): T | None;
export function iterToOption<T>(item: T | Done, none?: None): T | undefined | None;
export function iterToOption<T>(item: T | Done, none?: None): T | undefined | None {
	return item === DONE ? none : item;
}

/** An `Option`, as `next` returns it. */
export function iterFromOption<T>(option: T | undefined): T | Done;
export function iterFromOption<T>(option: T | None, none: None): T | Done;
export function iterFromOption<T>(option: T | undefined | None, none?: None): T | Done;
export function iterFromOption<T>(option: T | undefined | None, none?: None): T | Done {
	return option === none ? DONE : (option as T);
}

// Where std's iterators come from.

/** `slice::Iter`, and with what drops each item, what iterating an array by value makes. */
export function iterSlice<T>(s: ArrayLike<T>, drop?: Drop<T>): Iter<T> {
	return new SliceIter(s, drop);
}

/** `Vec::into_iter`, with what drops each item: over its items, keeping its buffer, which a `collect` in place or into a deque takes. */
export function vecIntoIter<T>(v: Vec<T>, drop?: Drop<T>): Iter<T> {
	const it = new SliceIter(v.items, drop);
	it.v = v;
	return it;
}

/** `Vec<u8>::into_iter`: over the vector's whole buffer, whose length is its capacity, which a deque collected from it keeps. */
export function byteVecIntoIter(v: ByteVec): Iter<number> {
	const it = new SliceIter(v.data);
	it.back = v.length;
	return it;
}

/** `slice::IterMut` of elements JS copies: the place of each. */
export function iterSliceMut<T>(s: SliceLike<T>): Iter<Place<T>> {
	const it = new SliceMutIter(s.data);
	it.front = s.start;
	it.back = s.start + s.length;
	return it;
}

/** `slice::windows`: each run of `size` elements, panicking on a size of 0 as std's does. */
export function sliceWindows<T>(s: Slice<T>, size: number, at?: Location): Iter<Slice<T>> {
	if (size === 0) panic("window size must be non-zero", at, true);
	return new PartsIter(s, s.length, size, 1, Math.max(s.length - size + 1, 0), false, slicePart);
}

/** `slice::chunks` and `chunks_mut`: each `size` elements, the last fewer, panicking on a size of 0 as std's does. */
export function sliceChunks<T>(s: Slice<T>, size: number, at?: Location): Iter<Slice<T>> {
	return chunks(s, s.length, size, false, false, slicePart, at);
}

/** `slice::chunks_exact` and `chunks_exact_mut`: each `size` elements, leaving the rest as its remainder. */
export function sliceChunksExact<T>(s: Slice<T>, size: number, at?: Location): Iter<Slice<T>> {
	return chunks(s, s.length, size, true, false, slicePart, at);
}

/** `slice::rchunks` and `rchunks_mut`: `chunks` from the back, the last part the front of the slice. */
export function sliceRchunks<T>(s: Slice<T>, size: number, at?: Location): Iter<Slice<T>> {
	return chunks(s, s.length, size, false, true, slicePart, at);
}

/** `slice::rchunks_exact` and `rchunks_exact_mut`: `chunks_exact` from the back, the remainder the front. */
export function sliceRchunksExact<T>(s: Slice<T>, size: number, at?: Location): Iter<Slice<T>> {
	return chunks(s, s.length, size, true, true, slicePart, at);
}

/** `slice::windows` of bytes. */
export function byteSliceWindows(s: ByteSlice, size: number, at?: Location): Iter<ByteSlice> {
	if (size === 0) panic("window size must be non-zero", at, true);
	return new PartsIter(s, s.length, size, 1, Math.max(s.length - size + 1, 0), false, byteSlicePart);
}

/** `slice::chunks` and `chunks_mut` of bytes, whose parts are views already. */
export function byteSliceChunks(s: ByteSlice, size: number, at?: Location): Iter<ByteSlice> {
	return chunks(s, s.length, size, false, false, byteSlicePart, at);
}

/** `slice::chunks_exact` and `chunks_exact_mut` of bytes. */
export function byteSliceChunksExact(s: ByteSlice, size: number, at?: Location): Iter<ByteSlice> {
	return chunks(s, s.length, size, true, false, byteSlicePart, at);
}

/** `slice::rchunks` and `rchunks_mut` of bytes. */
export function byteSliceRchunks(s: ByteSlice, size: number, at?: Location): Iter<ByteSlice> {
	return chunks(s, s.length, size, false, true, byteSlicePart, at);
}

/** `slice::rchunks_exact` and `rchunks_exact_mut` of bytes. */
export function byteSliceRchunksExact(s: ByteSlice, size: number, at?: Location): Iter<ByteSlice> {
	return chunks(s, s.length, size, true, true, byteSlicePart, at);
}

/**
 * `remainder` of a `ChunksExact` or an `RChunksExact`, and `into_remainder` of their
 * `_mut` forms: what no part holds, however many parts were taken.
 */
export function partsRemainder<S>(parts: Iter<S>): S {
	return (parts as PartsIter<S>).remainder();
}

/** `slice::split` and `split_mut`: the parts between the elements `f` is true of, the empty ones too. */
export function sliceSplit<T>(s: Slice<T>, f: (item: T) => boolean): Iter<Slice<T>> {
	return new SplitIter(s, s.data, s.start, slicePart, f, false, false);
}

/** `slice::rsplit` and `rsplit_mut`: `split` from the back. */
export function sliceRsplit<T>(s: Slice<T>, f: (item: T) => boolean): Iter<Slice<T>> {
	return new SplitIter(s, s.data, s.start, slicePart, f, true, false);
}

/** `slice::split_inclusive` and `split_inclusive_mut`: as `split`, but each part ends with its separator. */
export function sliceSplitInclusive<T>(s: Slice<T>, f: (item: T) => boolean): Iter<Slice<T>> {
	return new SplitIter(s, s.data, s.start, slicePart, f, false, true);
}

/** `slice::splitn` and `splitn_mut`: `split`, but the `n`th part all that is left. */
export function sliceSplitn<T>(s: Slice<T>, n: number, f: (item: T) => boolean): Iter<Slice<T>> {
	return new SplitIter(s, s.data, s.start, slicePart, f, false, false, n);
}

/** `slice::rsplitn` and `rsplitn_mut`: `rsplit`, but the `n`th part all that is left. */
export function sliceRsplitn<T>(s: Slice<T>, n: number, f: (item: T) => boolean): Iter<Slice<T>> {
	return new SplitIter(s, s.data, s.start, slicePart, f, true, false, n);
}

/** `slice::split` and `split_mut` of an array slice: the parts between the arrays `f` is true of, each lent where the slice holds it. */
export function arraySliceSplit<S extends ArrayRef>(r: ArraySlice<S>, f: (run: S) => boolean): Iter<ArraySlice<S>> {
	return new SplitIter(r, (i: number) => arraySliceAt(r, i), 0, arraySlicePart, f, false, false);
}

/** `slice::rsplit` and `rsplit_mut` of an array slice. */
export function arraySliceRsplit<S extends ArrayRef>(r: ArraySlice<S>, f: (run: S) => boolean): Iter<ArraySlice<S>> {
	return new SplitIter(r, (i: number) => arraySliceAt(r, i), 0, arraySlicePart, f, true, false);
}

/** `slice::split_inclusive` and `split_inclusive_mut` of an array slice. */
export function arraySliceSplitInclusive<S extends ArrayRef>(r: ArraySlice<S>, f: (run: S) => boolean): Iter<ArraySlice<S>> {
	return new SplitIter(r, (i: number) => arraySliceAt(r, i), 0, arraySlicePart, f, false, true);
}

/** `slice::splitn` and `splitn_mut` of an array slice. */
export function arraySliceSplitn<S extends ArrayRef>(r: ArraySlice<S>, n: number, f: (run: S) => boolean): Iter<ArraySlice<S>> {
	return new SplitIter(r, (i: number) => arraySliceAt(r, i), 0, arraySlicePart, f, false, false, n);
}

/** `slice::rsplitn` and `rsplitn_mut` of an array slice. */
export function arraySliceRsplitn<S extends ArrayRef>(r: ArraySlice<S>, n: number, f: (run: S) => boolean): Iter<ArraySlice<S>> {
	return new SplitIter(r, (i: number) => arraySliceAt(r, i), 0, arraySlicePart, f, true, false, n);
}

/** `slice::split` and `split_mut` of bytes, whose parts are views already. */
export function byteSliceSplit(s: ByteSlice, f: (byte: number) => boolean): Iter<ByteSlice> {
	return new SplitIter(s, s.data, s.start, byteSlicePart, f, false, false);
}

/** `slice::rsplit` and `rsplit_mut` of bytes. */
export function byteSliceRsplit(s: ByteSlice, f: (byte: number) => boolean): Iter<ByteSlice> {
	return new SplitIter(s, s.data, s.start, byteSlicePart, f, true, false);
}

/** `slice::split_inclusive` and `split_inclusive_mut` of bytes. */
export function byteSliceSplitInclusive(s: ByteSlice, f: (byte: number) => boolean): Iter<ByteSlice> {
	return new SplitIter(s, s.data, s.start, byteSlicePart, f, false, true);
}

/** `slice::splitn` and `splitn_mut` of bytes. */
export function byteSliceSplitn(s: ByteSlice, n: number, f: (byte: number) => boolean): Iter<ByteSlice> {
	return new SplitIter(s, s.data, s.start, byteSlicePart, f, false, false, n);
}

/** `slice::rsplitn` and `rsplitn_mut` of bytes. */
export function byteSliceRsplitn(s: ByteSlice, n: number, f: (byte: number) => boolean): Iter<ByteSlice> {
	return new SplitIter(s, s.data, s.start, byteSlicePart, f, true, false, n);
}

/** `slice::Split::as_slice`: what is left to split, or nothing once it is done. */
export function splitAsSlice<S>(parts: Iter<S>): S {
	return (parts as SplitIter<S, unknown>).asSlice();
}

/** `size` elements at a time, all of them or `exact`ly so, from the back if `rev`. */
function chunks<S>(s: S, length: number, size: number, exact: boolean, rev: boolean, part: (s: S, start: number, end: number) => S, at?: Location): Iter<S> {
	if (size === 0) panic("chunk size must be non-zero", at);
	const count = exact ? Math.floor(length / size) : Math.ceil(length / size);
	return new PartsIter(s, length, size, size, count, rev, part);
}

function byteSlicePart(s: ByteSlice, start: number, end: number): ByteSlice {
	return new ByteSlice(s.data, s.start + start, end - start);
}

/** `slice::iter` and `iter_mut` of an array slice: each array, lent where the slice holds it. */
export function arraySliceIter<S extends ArrayRef>(r: ArraySlice<S>): Iter<S> {
	return new ArraySliceIter(r);
}

/** `slice::windows` of an array slice. */
export function arraySliceWindows<S extends ArrayRef>(r: ArraySlice<S>, size: number, at?: Location): Iter<ArraySlice<S>> {
	if (size === 0) panic("window size must be non-zero", at, true);
	return new PartsIter(r, r.length, size, 1, Math.max(r.length - size + 1, 0), false, arraySlicePart);
}

/** `slice::chunks` and `chunks_mut` of an array slice. */
export function arraySliceChunks<S extends ArrayRef>(r: ArraySlice<S>, size: number, at?: Location): Iter<ArraySlice<S>> {
	return chunks(r, r.length, size, false, false, arraySlicePart, at);
}

/** `slice::chunks_exact` and `chunks_exact_mut` of an array slice. */
export function arraySliceChunksExact<S extends ArrayRef>(r: ArraySlice<S>, size: number, at?: Location): Iter<ArraySlice<S>> {
	return chunks(r, r.length, size, true, false, arraySlicePart, at);
}

/** `slice::rchunks` and `rchunks_mut` of an array slice. */
export function arraySliceRchunks<S extends ArrayRef>(r: ArraySlice<S>, size: number, at?: Location): Iter<ArraySlice<S>> {
	return chunks(r, r.length, size, false, true, arraySlicePart, at);
}

/** `slice::rchunks_exact` and `rchunks_exact_mut` of an array slice. */
export function arraySliceRchunksExact<S extends ArrayRef>(r: ArraySlice<S>, size: number, at?: Location): Iter<ArraySlice<S>> {
	return chunks(r, r.length, size, true, true, arraySlicePart, at);
}

/** `Option::iter`, and with what drops its item, `option::IntoIter`. */
export function iterOption<T>(o: T | undefined, drop?: Drop<T>): Iter<T>;
export function iterOption<T>(o: T | None, drop: Drop<T> | undefined, none: None): Iter<T>;
export function iterOption<T>(o: T | undefined | None, drop?: Drop<T>, none?: None): Iter<T> {
	return new OptionIter(o as T, o !== none, drop);
}

/**
 * `Result::iter`, and with what drops its item, `result::IntoIter`, which drops an `Err`
 * by `dropError` as it is made.
 */
export function iterResult<T, E>(r: Result<T, E>, drop?: Drop<T>, dropError?: Drop<E>): Iter<T> {
	if (r.ok) return new OptionIter(r.value, true, drop);
	dropError?.(r.error);
	return new OptionIter<T>(undefined, false);
}

/** `iter::once`. */
export function iterOnce<T>(value: T, drop?: Drop<T>): Iter<T> {
	return new OptionIter<T>(value, true, drop);
}

/** `iter::empty`. */
export function iterEmpty<T>(): Iter<T> {
	return new OptionIter<T>(undefined, false);
}

/** `iter::from_fn`, whose closure returns an `Option`, and drops with it. */
export function iterFromFn<T>(f: () => T | undefined): Iter<T>;
export function iterFromFn<T>(f: () => T | None, none: None): Iter<T>;
export function iterFromFn<T>(f: () => T | undefined | None, none?: None): Iter<T> {
	return new FromNext(f, (f) => iterFromOption(f(), none), dispose);
}

/** `iter::repeat_with`: `f` called for each item, endlessly, which drops with it. */
export function iterRepeatWith<T>(f: () => T): Iter<T> {
	return new FromNext(f, (f) => f(), dispose, { hint: () => [USIZE_MAX, undefined] });
}

/** `iter::repeat`: a clone of `element` by `clone` for each item, endlessly; `element` drops by `drop` with it. */
export function iterRepeat<T>(element: T, clone?: (element: T) => T, drop?: Drop<T>): Iter<T> {
	return new RepeatIter(element, clone, drop);
}

/** `iter::repeat_n`: `n` items of `element`, each a clone by `clone` but the last; `element` drops by `drop` with it, or at once if `n` is 0. */
export function iterRepeatN<T>(element: T, n: number, clone?: (element: T) => T, drop?: Drop<T>): Iter<T> {
	if (n === 0) drop?.(element);
	return new RepeatNIter(element, n, clone, drop);
}

/** `iter::once_with`: what `make` makes, called as it is first read, from either end; it drops uncalled with it. */
export function iterOnceWith<T>(make: () => T): Iter<T> {
	return new OnceWithIter(make);
}

/**
 * `iter::successors`: `first`, then `succ` of each item, called as that item is yielded.
 * The item it holds next drops by `drop`, and then `succ`, as std's fields drop.
 */
export function iterSuccessors<T>(first: T | undefined, succ: (item: T) => T | undefined, drop?: Drop<T>): Iter<T>;
export function iterSuccessors<T>(first: T | None, succ: (item: T) => T | None, drop: Drop<T> | undefined, none: None): Iter<T>;
export function iterSuccessors<T>(first: T | undefined | None, succ: (item: T) => T | undefined | None, drop?: Drop<T>, none?: None): Iter<T> {
	return new SuccessorsIter(first, succ, drop, none);
}

/**
 * The methods of std's iterator traits an iterator of the translated crate has of its own,
 * beside `next`, each a function of its source: `iterFrom` calls each where std's
 * adaptors and consumers do. `fold`, `rfold`, `count`, `last`, `forEach`, `reduce`, and
 * `isSortedBy` take it by value. Each that takes a closure is given a function of what
 * std's is given.
 */
export interface Own<S, T> {
	readonly hint?: (source: S) => SizeHint;
	readonly back?: (source: S) => T | Done;
	readonly len?: (source: S) => number;
	readonly nth?: (source: S, n: number) => T | Done;
	readonly nthBack?: (source: S, n: number) => T | Done;
	/** Of one accumulator type in Rust, which only moves the one it is given, as any it is called with. */
	readonly fold?: (source: S, acc: never, f: never) => unknown;
	readonly rfold?: (source: S, acc: never, f: never) => unknown;
	readonly count?: (source: S) => number;
	readonly last?: (source: S) => T | Done;
	readonly find?: (source: S, f: never) => T | Done;
	readonly rfind?: (source: S, f: never) => T | Done;
	/** Of one found type in Rust, which only moves what it finds, given each boxed in an array. */
	readonly findMap?: (source: S, f: never) => unknown;
	readonly forEach?: (source: S, f: never) => void;
	readonly reduce?: (source: S, f: never) => T | Done;
	readonly all?: (source: S, f: never) => boolean;
	readonly isSortedBy?: (source: S, f: never) => boolean;
}

/**
 * An `impl Iterator` of the translated crate, which `next` calls the `next` of, `drop`
 * drops, and `own` reads by the methods it has of its own.
 */
export function iterFrom<S, T>(source: S, next: (source: S) => T | Done, drop?: Drop<S>, own?: Own<S, T>): Iter<T> {
	return new FromNext(source, next, drop, own);
}

/** An iterator borrowed by `&mut`, which std's adaptors take as one: its items, but never it to drop. */
export function iterBorrow<T>(it: Iter<T>): Iter<T> {
	return new Borrowed(it);
}

/**
 * A `dyn Iterator` of `it`: what std's vtable has, as `it` does it, and the rest by `next`, as
 * std's default methods read a trait object. It drops `it` as it drops.
 */
export function iterDyn<T>(it: Iter<T>): Iter<T> {
	return new Dyn(it);
}

/**
 * A `Box` of `it`: what std's `Iterator` for `Box<I>` forwards, `last` among them, as `it`
 * does it, and the rest by `next`, `advance_by` and the folds included. It drops `it` as it
 * drops.
 */
export function iterBox<T>(it: Iter<T>): Iter<T> {
	return new Boxed(it);
}

// How std's `Vec` and `VecDeque` reserve for an iterator they collect or extend by, of
// elements `size` bytes each, which panics past what they may hold: a `TrustedLen` one's
// upper bound first, and else, after each item that finds them full, the lower bound and
// one more; as a tuple's part, the lower bound first.

/** The upper bound of a `TrustedLen` iterator `it`, which std's reserves, panicking without one. */
export function trustedUpper(it: Iter<unknown>): number {
	const upper = it.sizeHint()[1];
	if (upper === undefined) panic("capacity overflow", "trusted_capacity");
	return upper;
}

/** The capacity `cap`, holding `len`, once it reserved for the next item of `it` and those it holds past it. */
export function capNext(it: Iter<unknown>, len: number, cap: number, size: number): number {
	return capReserve(len, cap, satAdd(it.sizeHint()[0], 1), size);
}

/**
 * `join` of slices of `size`-byte elements by the slice `sep`, or `concat` without one:
 * each item's elements, read from it by `slice` unless it is the slice, with `sep`'s
 * between them, each cloned in order by `clone` if given.
 */
export function slicesJoin<V, T>(items: Slice<V>, size: number, sep?: Slice<T>, slice?: (item: V) => Slice<T>, clone?: (item: T) => T): Vec<T> {
	const all = sliceItems(items);
	const parts = slice === undefined ? (all as unknown as readonly Slice<T>[]) : all.map(slice);
	// std's makes one with room for exactly them.
	const v = vecWithCapacity<T>(joinedLength(parts, sep, size), size);
	for (let i = 0; i < parts.length; i++) {
		if (i > 0 && sep !== undefined) vecExtendFromSlice(v, sep, size, clone);
		vecExtendFromSlice(v, parts[i], size, clone);
	}
	return v;
}

/**
 * `join` of byte slices by the bytes `sep`, or `concat` without them: each item's bytes,
 * read from it by `slice` unless it is the slice, with `sep` between them, into a
 * `Vec<u8>` of the capacity std's has, their exact length.
 */
export function byteSlicesJoin<V>(items: Slice<V>, sep?: ByteSlice, slice?: (item: V) => ByteSlice): ByteVec {
	const all = sliceItems(items);
	const parts = slice === undefined ? (all as unknown as readonly ByteSlice[]) : all.map(slice);
	const v = new ByteVec(joinedLength(parts, sep, 1));
	for (let i = 0; i < parts.length; i++) {
		if (i > 0 && sep !== undefined) byteVecExtendFromSlice(v, sep);
		byteVecExtendFromSlice(v, parts[i]);
	}
	return v;
}

/**
 * What `concat` or `join` by `sep` of `parts`, of `size`-byte elements, reserves before it
 * clones any, as std's does: its sum of the parts' lengths panics past `usize::MAX`, the
 * separators' count wraps, and `Vec::with_capacity` of that panics past what the `Vec` may
 * hold. Where the count wraps, the `Vec` grows as it fills, which memory bounds first.
 */
function joinedLength(parts: readonly { length: number }[], sep: { length: number } | undefined, size: number): number {
	let sum = 0;
	for (const part of parts) sum += part.length;
	if (sum > USIZE_MAX) panic("attempt to add with overflow", "int_sum");
	const seps = sep === undefined || parts.length === 0 ? 0 : Math.imul(sep.length, parts.length - 1) >>> 0;
	const length = (sum + seps) % (USIZE_MAX + 1);
	if (length > vecMax(size)) panic("capacity overflow", "capacity");
	return length;
}

/**
 * `Extend::extend` of a `Vec`, which takes `items` as `take` says, and with elements
 * `size` bytes each, reserves as std's does for a `trusted`, `TrustedLen`, iterator or any
 * other.
 */
export function iterExtend<T>(v: Vec<T>, items: Iter<T>, take?: Take, size?: number, trusted = false): Vec<T> {
	const d = v.items;
	if (size !== undefined && !trusted) {
		let cap = vecCapacity(v, size);
		for (let item = items.next(); item !== DONE; item = items.next()) {
			if (d.length === cap) cap = capNext(items, d.length, cap, size);
			d.push(item);
		}
		vecHold(v, cap);
		if (take !== undefined) dispose(items);
		return v;
	}
	if (size !== undefined) vecHold(v, capReserve(d.length, vecCapacity(v, size), trustedUpper(items), size));
	if (take === undefined) {
		const folded = items.folded(trusted);
		for (let item = folded.next(); item !== DONE; item = folded.next()) d.push(item);
		return v;
	}
	each(items, (item) => d.push(item), take);
	return v;
}

/** `Extend::extend` of a `Vec<u8>`, reserving as `iterExtend` does. */
export function iterExtendBytes(v: ByteVec, items: Iter<number>, take?: Take, trusted = false): ByteVec {
	if (!trusted) {
		for (let item = items.next(); item !== DONE; item = items.next()) {
			if (v.length === v.data.length) byteVecReserve(v, satAdd(items.sizeHint()[0], 1));
			byteVecPush(v, item);
		}
		if (take !== undefined) dispose(items);
		return v;
	}
	byteVecReserve(v, trustedUpper(items));
	if (take === undefined) {
		const folded = items.folded(true);
		for (let item = folded.next(); item !== DONE; item = folded.next()) byteVecPush(v, item);
		return v;
	}
	each(items, (item) => byteVecPush(v, item), take);
	return v;
}

// Adaptors

/** `Iterator::map`. */
export function iterMap<T, U>(it: Iter<T>, f: (item: T) => U): Iter<U> {
	return new MapIter(it, f);
}

/**
 * `it` with each item lent by `f` as translated code reads it, as a `&[u8; N]` of a
 * `Uint8Array` is lent as an `rs.ByteSlice`.
 */
export function iterLend<T, U>(it: Iter<T>, f: (item: T) => U): Iter<U> {
	return new LendIter(it, f);
}

/** `Iterator::filter`, dropping what `f` rejects by `drop`. */
export function iterFilter<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): Iter<T> {
	return new FilterIter(it, f, drop);
}

/** `Iterator::filter_map`. */
export function iterFilterMap<T, U>(it: Iter<T>, f: (item: T) => U | undefined): Iter<U>;
export function iterFilterMap<T, U>(it: Iter<T>, f: (item: T) => U | None, none: None): Iter<U>;
export function iterFilterMap<T, U>(it: Iter<T>, f: (item: T) => U | undefined | None, none?: None): Iter<U> {
	return new FilterMapIter(it, f, none);
}

/** `Iterator::enumerate`. */
export function iterEnumerate<T>(it: Iter<T>): Iter<[number, T]> {
	return new EnumerateIter(it);
}

/** `chain`, with `other` already made an iterator, as std does when it is called. */
export function iterChain<T>(it: Iter<T>, other: Iter<T>): Iter<T> {
	return new ChainIter(it, other);
}

/**
 * How std's `Zip` reads its iterators, by the traits both implement: two
 * `TrustedRandomAccess` ones, as whether each may have side effects (`MAY_HAVE_SIDE_EFFECT`),
 * which it reads by index up to the shorter length and past it only one that may; `len`
 * of two `TrustedLen` ones, whose `fold` reads as many pairs as its size hint's upper
 * bound, where `next` reads the first once past a shorter second; and `index` of two
 * `TrustedRandomAccessNoCoerce` ones, whose `fold` reads as many by index.
 */
export type Trusted = readonly [boolean, boolean] | "len" | "index";

/**
 * `zip`, with `other` already made an iterator, as std does when it is called, read as
 * `trusted` says, dropping an item of either it reads without one of the other by `dropA`
 * or `dropB`.
 */
export function iterZip<T, U>(it: Iter<T>, other: Iter<U>, trusted?: Trusted, dropA?: Drop<T>, dropB?: Drop<U>): Iter<[T, U]> {
	if (trusted === undefined || typeof trusted === "string") return new ZipIter(it, other, trusted, dropA, dropB);
	return new TrustedZipIter(it, other, trusted, dropA, dropB);
}

/** `Iterator::rev`. */
export function iterRev<T>(it: Iter<T>): Iter<T> {
	return new RevIter(it);
}

/** `Iterator::skip`, dropping what it skips by `drop`, through its iterator's own `nth`. */
export function iterSkip<T>(it: Iter<T>, n: number, drop?: Drop<T>): Iter<T> {
	return new SkipIter(it, n, drop);
}

/**
 * `Iterator::take`, dropping what `next_back` skips past its limit by `drop`, whose `fold`
 * reads its iterator by index if it `indexes`, a `TrustedRandomAccess` one.
 */
export function iterTake<T>(it: Iter<T>, n: number, drop?: Drop<T>, indexes = false): Iter<T> {
	return it instanceof RepeatIter ? new TakeRepeat(it, n, drop, indexes) : new TakeIter(it, n, drop, indexes);
}

/** `Iterator::skip_while`, dropping what it skips by `drop`. */
export function iterSkipWhile<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): Iter<T> {
	return new SkipWhileIter(it, f, drop);
}

/** `Iterator::take_while`, dropping the item that ends it by `drop`. */
export function iterTakeWhile<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): Iter<T> {
	return new TakeWhileIter(it, f, drop);
}

/** `Iterator::map_while`. */
export function iterMapWhile<T, U>(it: Iter<T>, f: (item: T) => U | undefined): Iter<U>;
export function iterMapWhile<T, U>(it: Iter<T>, f: (item: T) => U | None, none: None): Iter<U>;
export function iterMapWhile<T, U>(it: Iter<T>, f: (item: T) => U | undefined | None, none?: None): Iter<U> {
	return new MapWhileIter(it, f, none);
}

/**
 * `Iterator::scan`, which hands `f` its state, as its place if `copied`, where JS copies it,
 * and drops it by `drop` after `f`, as std's fields drop.
 */
export function iterScan<T, S, U>(it: Iter<T>, state: S, f: (state: never, item: T) => U | undefined, copied?: boolean, drop?: Drop<S>): Iter<U>;
export function iterScan<T, S, U>(it: Iter<T>, state: S, f: (state: never, item: T) => U | None, copied: boolean | undefined, drop: Drop<S> | undefined, none: None): Iter<U>;
export function iterScan<T, S, U>(it: Iter<T>, state: S, f: (state: never, item: T) => U | undefined | None, copied = false, drop?: Drop<S>, none?: None): Iter<U> {
	return new ScanIter(it, state, f, copied, drop, none);
}

/**
 * `Iterator::step_by`, panicking on a step of 0, and dropping what it steps over by `drop`.
 * Of a `Range` of an unsigned integer no wider than a `usize`, whose greatest is `max`, it
 * counts down the range's end, as std's specializes it to.
 */
export function iterStepBy<T>(it: Iter<T>, step: number, drop?: Drop<T>, max?: number, at?: Location): Iter<T> {
	if (step === 0) panic("assertion failed: step != 0", at);
	if (max === undefined) return new StepByIter(it, step, drop);
	const range = it as never as Range<number>;
	range.end = Math.ceil(range.sizeHint()[0] / step);
	return new RangeStepBy(range, step - 1, max) as never as Iter<T>;
}

/**
 * `Iterator::cycle`: `it`, and then clones of it as it was, by `h`, as std's `Clone` of
 * it clones each part. It holds the first clone as it is made.
 */
export function iterCycle<T>(it: Iter<T>, h?: Clones): Iter<T> {
	return new CycleIter(it.clone(h), it, h);
}

/**
 * `Iterator::copied` of items JS copies by `copy`, which std's skips through: its `nth`
 * and `advance_by` are its iterator's.
 */
export function iterCopied<R, T>(it: Iter<R>, copy: (item: R) => T): Iter<T> {
	return new CopiedIter(it, copy);
}

/** `Iterator::cloned` of items that are JS values, whose `nth` reads by `next`, as std's does. */
export function iterCloned<T>(it: Iter<T>): Iter<T> {
	return new MapIter(it, same);
}

/** `Iterator::inspect`. */
export function iterInspect<T>(it: Iter<T>, f: (item: T) => void): Iter<T> {
	return new InspectIter(it, f);
}

/**
 * `Iterator::fuse`, which drops `it` once it ends, unless it is `fused`, a
 * `FusedIterator`, which std's keeps.
 */
export function iterFuse<T>(it: Iter<T>, fused = false): Iter<T> {
	return new FuseIter(it, fused);
}

/** `Iterator::peekable`, which drops an item it peeked by `drop`. */
export function iterPeekable<T>(it: Iter<T>, drop?: Drop<T>): Peekable<T> {
	return new Peekable(it, drop);
}

/**
 * `flat_map`: `into` makes each of `f`'s results an iterator, as `IntoIterator` does. Its
 * source drops once it ends unless it is `fused`, as `fuse` does. `size` and `oneShot`
 * are as `flatten`'s.
 */
export function iterFlatMap<T, V>(it: Iter<T>, f: (item: T) => Iter<V>, into?: undefined, fused?: boolean, size?: number, oneShot?: boolean): Iter<V>;
export function iterFlatMap<T, U, V>(it: Iter<T>, f: (item: T) => U, into?: (value: U) => Iter<V>, fused?: boolean, size?: number, oneShot?: boolean): Iter<V>;
export function iterFlatMap<T, U, V>(it: Iter<T>, f: (item: T) => U, into?: (value: U) => Iter<V>, fused = false, size?: number, oneShot = false): Iter<V> {
	return new FlattenIter(new MapIter(it, f), into ?? (itself as (value: U) => Iter<V>), fused, size, oneShot);
}

/** A `flatten`'s or `flat_map`'s front iterator, source, and back iterator, as std's `into_parts` gives them. */
export function iterFlattenParts<V>(it: Iter<V>): [Iter<V> | undefined, Iter<unknown> | undefined, Iter<V> | undefined] {
	if (!(it instanceof FlattenIter)) throw new TypeError("iterFlattenParts of an iterator that is not a flatten");
	return [it.front, it.outer, it.back];
}

/**
 * `flatten`: `into` makes each item an iterator, as `IntoIterator` does, `fused` is as
 * `flat_map`'s, `size` is each item's length if it is an array, and `oneShot` says each
 * item's iterator has at most one item, as std's size hint specializes on.
 */
export function iterFlatten<V>(it: Iter<Iter<V>>, into?: undefined, fused?: boolean, size?: number, oneShot?: boolean): Iter<V>;
export function iterFlatten<T, V>(it: Iter<T>, into?: (value: T) => Iter<V>, fused?: boolean, size?: number, oneShot?: boolean): Iter<V>;
export function iterFlatten<T, V>(it: Iter<T>, into?: (value: T) => Iter<V>, fused = false, size?: number, oneShot = false): Iter<V> {
	return new FlattenIter(it, into ?? (itself as (value: T) => Iter<V>), fused, size, oneShot);
}

// Debug. Each writes what std's `Debug` of an adaptor writes: its fields in std's order,
// each iterator it wraps, an `I`, by the function given for its type, and no closure.

/**
 * What an adaptor's field holds as Rust sees it, from the iterator the adaptor reads: what
 * a box or a `&mut` borrow forwards to, through any number of them, and the crate's own
 * iterator, which `iterFrom` reads.
 */
export function iterHeld<I>(it: Iter<unknown>): I {
	while (it instanceof Forward) it = it.inner;
	return (it instanceof FromNext ? it.source : it) as I;
}

/** `Debug` of an adaptor std writes as the iterator it wraps alone, under `field`. */
export function fmtAdaptor<I>(it: Iter<unknown>, f: Formatter, name: string, inner: Debug<I>, field = "iter"): FmtResult {
	return fmtDebugStructFields(f, name, [field], [(f) => inner(iterHeld((it as Adaptor<unknown, unknown>).inner), f)]);
}

/**
 * `Debug` of a struct std writes around one field, `field`, that is the iterator it is
 * itself: a `Copied` of items JS reads as is, and a `str`'s escape, its `flat_map`.
 */
export function fmtAround<I>(it: Iter<unknown>, f: Formatter, name: string, field: string, inner: Debug<I>): FmtResult {
	return fmtDebugStructFields(f, name, [field], [(f) => inner(iterHeld(it), f)]);
}

/** `Debug` of an `Enumerate`: its iterator and the index it yields next. */
export function fmtEnumerate<I>(it: Iter<unknown>, f: Formatter, inner: Debug<I>): FmtResult {
	const e = it as EnumerateIter<unknown>;
	return fmtDebugStructFields(f, "Enumerate", ["iter", "count"], [(f) => inner(iterHeld(e.inner), f), (f) => usizeDebug(e.index, f)]);
}

/** `Debug` of a `Skip` or a `Take`, `name`: its iterator and how many it has left to skip or take. */
export function fmtCounted<I>(it: Iter<unknown>, f: Formatter, name: string, inner: Debug<I>): FmtResult {
	const c = it as SkipIter<unknown> | TakeIter<unknown>;
	return fmtDebugStructFields(f, name, ["iter", "n"], [(f) => inner(iterHeld(c.inner), f), (f) => usizeDebug(c.n, f)]);
}

/** `Debug` of a `SkipWhile` or a `TakeWhile`, `name`: its iterator and whether its closure has failed. */
export function fmtFlagged<I>(it: Iter<unknown>, f: Formatter, name: string, inner: Debug<I>): FmtResult {
	const w = it as SkipWhileIter<unknown> | TakeWhileIter<unknown>;
	return fmtDebugStructFields(f, name, ["iter", "flag"], [(f) => inner(iterHeld(w.inner), f), (f) => fmtBool(w.flag, f)]);
}

/** `Debug` of a `Scan`: its iterator and its state, by `state`. */
export function fmtScan<I, S>(it: Iter<unknown>, f: Formatter, inner: Debug<I>, state: Debug<S>): FmtResult {
	const s = it as ScanIter<unknown, S, unknown>;
	return fmtDebugStructFields(f, "Scan", ["iter", "state"], [(f) => inner(iterHeld(s.inner), f), (f) => state(s.state, f)]);
}

/** `Debug` of a `Peekable`: its iterator and what it peeked, `Some` of the item by `item` or of `None`. */
export function fmtPeekable<I, T>(it: Peekable<T>, f: Formatter, inner: Debug<I>, item: Debug<T>): FmtResult {
	const some = (f: Formatter, value: Fmt): FmtResult => fmtDebugTupleFields(f, "Some", [value]);
	const peeked = (f: Formatter): FmtResult => {
		if (!it.peeked) return f.out.writeStr("None");
		const peek = it.item;
		return some(f, (f) => (peek === DONE ? f.out.writeStr("None") : some(f, (f) => item(peek, f))));
	};
	return fmtDebugStructFields(f, "Peekable", ["iter", "peeked"], [(f) => inner(iterHeld(it.inner), f), peeked]);
}

/** `Debug` of a `StepBy`: its iterator, its step less one, and whether it has yet to take its first item. */
export function fmtStepBy<I>(it: Iter<unknown>, f: Formatter, inner: Debug<I>): FmtResult {
	const s = it as StepByIter<unknown> | RangeStepBy;
	const fields = [(f: Formatter) => inner(iterHeld(s.inner), f), (f: Formatter) => usizeDebug(s.step, f), (f: Formatter) => fmtBool(s.firstTake, f)];
	return fmtDebugStructFields(f, "StepBy", ["iter", "step_minus_one", "first_take"], fields);
}

/** `Debug` of a `Chain`: each of its iterators, by `a` and `b`, until it has dropped it. */
export function fmtChain<A, B>(it: Iter<unknown>, f: Formatter, a: Debug<A>, b: Debug<B>): FmtResult {
	const c = it as ChainIter<unknown>;
	return fmtDebugStructFields(f, "Chain", ["a", "b"], [(f) => fmtOption(c.a && iterHeld<A>(c.a), f, a), (f) => fmtOption(c.b && iterHeld<B>(c.b), f, b)]);
}

/**
 * `Debug` of a `Zip`: each of its iterators, by `a` and `b`, or neither, as std's writes a
 * zip of two `TrustedRandomAccessNoCoerce` ones, without them.
 */
export function fmtZip<A, B>(it: Iter<unknown>, f: Formatter, a?: Debug<A>, b?: Debug<B>): FmtResult {
	const z = it as ZipIter<unknown, unknown>;
	if (a === undefined || b === undefined) return fmtDebugStructFields(f, "Zip", [], []);
	return fmtDebugStructFields(f, "Zip", ["a", "b"], [(f) => a(iterHeld(z.a), f), (f) => b(iterHeld(z.b), f)]);
}

/** `Debug` of a `Cycle`: the iterator as it was, and the one it reads. */
export function fmtCycle<I>(it: Iter<unknown>, f: Formatter, inner: Debug<I>): FmtResult {
	const c = it as CycleIter<unknown>;
	return fmtDebugStructFields(f, "Cycle", ["orig", "iter"], [(f) => inner(iterHeld(c.orig), f), (f) => inner(iterHeld(c.iter), f)]);
}

/** `Debug` of a `Fuse`: its iterator, until it has dropped it. */
export function fmtFuse<I>(it: Iter<unknown>, f: Formatter, inner: Debug<I>): FmtResult {
	const fused = (it as FuseIter<unknown>).inner;
	return fmtDebugStructFields(f, "Fuse", ["iter"], [(f) => fmtOption(fused && iterHeld<I>(fused), f, inner)]);
}

/**
 * `Debug` of a `Flatten` or a `FlatMap`, `name`: its `FlattenCompat`, of its source fused,
 * by `source`, and the iterators it reads from the front and the back, by `inner`.
 */
export function fmtFlatten<S, I>(it: Iter<unknown>, f: Formatter, name: string, source: Debug<S>, inner: Debug<I>): FmtResult {
	const flat = it as FlattenIter<unknown, unknown>;
	const fused = (f: Formatter): FmtResult => fmtDebugStructFields(f, "Fuse", ["iter"], [(f) => fmtOption(flat.outer && iterHeld<S>(flat.outer), f, source)]);
	const fields = [fused, (f: Formatter) => fmtOption(flat.front && iterHeld<I>(flat.front), f, inner), (f: Formatter) => fmtOption(flat.back && iterHeld<I>(flat.back), f, inner)];
	return fmtDebugStructFields(f, name, ["inner"], [(f) => fmtDebugStructFields(f, "FlattenCompat", ["iter", "frontiter", "backiter"], fields)]);
}

/** `Debug` of a `Successors`: the item it yields next, if any. */
export function fmtSuccessors<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>): FmtResult {
	const s = it as SuccessorsIter<T>;
	const next = (f: Formatter): FmtResult => (s.item === s.none ? f.out.writeStr("None") : fmtDebugTupleFields(f, "Some", [(f) => debug(s.item as T, f)]));
	return fmtDebugStructFields(f, "Successors", ["next"], [next]);
}

/**
 * `Debug` of a slice's `Windows`, `Chunks`, `ChunksExact`, `RChunks`, or `RChunksExact`,
 * `name`: what its parts left cover by `slice`, an exact one's remainder, and its size.
 */
export function fmtParts<S>(it: Iter<unknown>, f: Formatter, slice: Debug<S>, name: string): FmtResult {
	const p = it as PartsIter<S>;
	const [v, size]: Fmt[] = [(f) => slice(p.rest(), f), (f) => usizeDebug(p.size, f)];
	if (name === "Windows") return fmtDebugStructFields(f, name, ["v", "size"], [v, size]);
	if (!name.endsWith("Exact")) return fmtDebugStructFields(f, name, ["v", "chunk_size"], [v, size]);
	return fmtDebugStructFields(f, name, ["v", "rem", "chunk_size"], [v, (f) => slice(p.remainder(), f), size]);
}

/**
 * `Debug` of a slice's `Split` or one of its forms, `name`: what is left to split by
 * `slice`, which a `&mut` one gives up as it finishes if `taken`, and whether it has
 * finished; inside its `GenericSplitN` beside its count, under `n`, for a `splitn`.
 */
export function fmtSplit<S>(it: Iter<unknown>, f: Formatter, slice: Debug<S>, name: string, taken: boolean, n?: string): FmtResult {
	const s = it as SplitIter<S, unknown>;
	const split = (f: Formatter): FmtResult => fmtDebugStructFields(f, name, ["v", "finished"], [(f) => slice(s.rest(taken), f), (f) => fmtBool(s.finished, f)]);
	if (n === undefined) return split(f);
	const generic = (f: Formatter): FmtResult => fmtDebugStructFields(f, "GenericSplitN", ["iter", "count"], [split, (f) => usizeDebug(s.count as number, f)]);
	return fmtDebugStructFields(f, n, ["inner"], [generic]);
}

// Consumers. Each that takes an iterator by value that `drops` takes it as std's does: by
// its own `fold` where std's folds, which drops what it holds as it goes, or as `take`
// says, and else by `next`, dropping it once done.

/**
 * How a consumer takes an iterator by value, where std's may either way: by `next`, as
 * std's does one it can't trust the length of, by its own `fold`, which drops what it
 * holds, or by index, as std's `collect` in place reads a `TrustedRandomAccessNoCoerce`
 * one, before dropping it whole. Without any, it drops nothing, and std's folds it.
 */
export type Take = "next" | "fold" | "index";

/**
 * `it`, taken as `take` says: `f` of each item, and then it dropped. Taken by `fold`, it is
 * read as std's `for_each` reads it.
 */
function each<T>(it: Iter<T>, f: (item: T) => void, take: Take | undefined): void {
	if (take === "fold") {
		forEach(it, f);
		return;
	}
	if (take === "index") {
		const indexed = it.indexed();
		consumed(it, () => {
			for (let n = it.len(); n > 0; n--) f(indexed.next() as T);
		});
		return;
	}
	consumed(it, () => {
		for (let item = it.next(); item !== DONE; item = it.next()) f(item);
	});
}

/** `Iterator::count` of an iterator that drops nothing. */
export function iterCount<T>(it: Iter<T>): number {
	const folded = it.folded(true);
	let n = 0;
	while (folded.next() !== DONE) n++;
	return n;
}

/** `Iterator::last` of an iterator that drops nothing. */
export function iterLast<T>(it: Iter<T>): T | undefined;
export function iterLast<T>(it: Iter<T>, none: None): T | None;
export function iterLast<T>(it: Iter<T>, none?: None): T | undefined | None {
	// std's of a box: what it holds, by its own.
	while (it instanceof Boxed) it = it.inner;
	const folded = it.folded(true);
	let final: T | Done = DONE;
	for (let item = folded.next(); item !== DONE; item = folded.next()) final = item;
	return iterToOption(final, none);
}

/** `Iterator::nth`, dropping what it skips by `drop`, as its own `nth` does. */
export function iterNth<T>(it: Iter<T>, n: number, drop?: Drop<T>): T | undefined;
export function iterNth<T>(it: Iter<T>, n: number, drop: Drop<T> | undefined, none: None): T | None;
export function iterNth<T>(it: Iter<T>, n: number, drop?: Drop<T>, none?: None): T | undefined | None {
	return iterToOption(it.nth(n, drop), none);
}

/** `DoubleEndedIterator::nth_back`, dropping what it skips by `drop`, as its own `nth_back` does. */
export function iterNthBack<T>(it: Iter<T>, n: number, drop?: Drop<T>): T | undefined;
export function iterNthBack<T>(it: Iter<T>, n: number, drop: Drop<T> | undefined, none: None): T | None;
export function iterNthBack<T>(it: Iter<T>, n: number, drop?: Drop<T>, none?: None): T | undefined | None {
	return iterToOption(it.nthBack(n, drop), none);
}

/** `Iterator::any`. */
export function iterAny<T>(it: Iter<T>, f: (item: T) => boolean): boolean {
	const folded = it.folded();
	for (let item = folded.next(); item !== DONE; item = folded.next()) if (f(item)) return true;
	return false;
}

/** `Iterator::all`. */
export function iterAll<T>(it: Iter<T>, f: (item: T) => boolean): boolean {
	const folded = it.folded();
	for (let item = folded.next(); item !== DONE; item = folded.next()) if (!f(item)) return false;
	return true;
}

/** `find`, as `next` returns it: what `f` rejects drops by `drop`. */
function findItem<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): T | Done {
	const folded = it.folded();
	for (let item = folded.next(); item !== DONE; item = folded.next()) {
		if (f(item)) return item;
		drop?.(item);
	}
	return DONE;
}

/** `rfind`, as `next_back` returns it, as `try_rfold` steps it. */
function rfindItem<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): T | Done {
	const folded = it.folded();
	for (let item = folded.nextBack(); item !== DONE; item = folded.nextBack()) {
		if (f(item)) return item;
		drop?.(item);
	}
	return DONE;
}

/** `Iterator::find`, dropping what `f` rejects by `drop`. */
export function iterFind<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): T | undefined;
export function iterFind<T>(it: Iter<T>, f: (item: T) => boolean, drop: Drop<T> | undefined, none: None): T | None;
export function iterFind<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>, none?: None): T | undefined | None {
	return iterToOption(it.find(f, drop), none);
}

/** `Iterator::find_map`. */
export function iterFindMap<T, U>(it: Iter<T>, f: (item: T) => U | undefined): U | undefined;
export function iterFindMap<T, U>(it: Iter<T>, f: (item: T) => U | None, none: None): U | None;
export function iterFindMap<T, U>(it: Iter<T>, f: (item: T) => U | undefined | None, none?: None): U | undefined | None {
	const found = it.findMap(f, none);
	return found === DONE ? none : found;
}

/** `DoubleEndedIterator::rfind`, dropping what `f` rejects by `drop`. */
export function iterRfind<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>): T | undefined;
export function iterRfind<T>(it: Iter<T>, f: (item: T) => boolean, drop: Drop<T> | undefined, none: None): T | None;
export function iterRfind<T>(it: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>, none?: None): T | undefined | None {
	return iterToOption(it.rfind(f, drop), none);
}

/** `Iterator::rposition`: from the back, where its item is counting from the front. */
export function iterRposition<T>(it: Iter<T>, f: (item: T) => boolean): number | undefined {
	let i = it.len();
	const folded = it.folded();
	for (let item = folded.nextBack(); item !== DONE; item = folded.nextBack()) {
		i--;
		if (f(item)) return i;
	}
	return undefined;
}

/** `Iterator::position`. */
export function iterPosition<T>(it: Iter<T>, f: (item: T) => boolean): number | undefined {
	const folded = it.folded();
	let i = 0;
	for (let item = folded.next(); item !== DONE; item = folded.next()) {
		if (f(item)) return i;
		i++;
	}
	return undefined;
}

/**
 * `Iterator::reduce`: the first item by `next`, folded with the rest by `f`. If it `owns`
 * `f`, `f` drops as `iterFold`'s does, or before `it` if `it` is empty.
 */
export function iterReduce<T>(it: Iter<T>, f: (acc: T, item: T) => T, drops?: boolean, owns?: boolean): T | undefined;
export function iterReduce<T>(it: Iter<T>, f: (acc: T, item: T) => T, drops: boolean | undefined, owns: boolean | undefined, none: None): T | None;
export function iterReduce<T>(it: Iter<T>, f: (acc: T, item: T) => T, drops = false, owns = false, none?: None): T | undefined | None {
	const first = it.next();
	if (first !== DONE) return iterFold(it, first, f, drops, owns);
	if (owns) dispose(f);
	if (drops) dispose(it);
	return none;
}

/**
 * `Iterator::fold`. If it `owns` `f`, a closure that drops, `f` drops as the fold ends,
 * before `it`, as std's drops the closure it was given.
 */
export function iterFold<T, A>(it: Iter<T>, init: A, f: (acc: A, item: T) => A, drops = false, owns = false): A {
	if (!drops) {
		const folded = it.folded(true);
		let acc = init;
		for (let item = folded.next(); item !== DONE; item = folded.next()) acc = f(acc, item);
		if (owns) dispose(f);
		return acc;
	}
	return it.fold(init, f, owns ? () => dispose(f) : undefined);
}

/** `DoubleEndedIterator::rfold`, from the back, as `iterFold`. */
export function iterRfold<T, A>(it: Iter<T>, init: A, f: (acc: A, item: T) => A, drops = false, owns = false): A {
	if (!drops) {
		const folded = it.folded(true);
		let acc = init;
		for (let item = folded.nextBack(); item !== DONE; item = folded.nextBack()) acc = f(acc, item);
		if (owns) dispose(f);
		return acc;
	}
	return it.rfold(init, f, owns ? () => dispose(f) : undefined);
}

/**
 * `Iterator::try_fold`, as `try_fold` steps it, until `f` breaks: by an `Option`'s `None`,
 * `none` where that is `rs.None`, if `option`, and else by an `rs.Result`'s `Err`, as a
 * `Result` or a `ControlFlow` breaks. It returns what broke, or the accumulator as the type
 * continues with it. If it `owns` `f`, `f` drops as it returns.
 */
export function iterTryFold<T, A, R>(it: Iter<T>, init: A, f: (acc: A, item: T) => R, option = false, owns = false, none?: None): R {
	return tryFold(it, false, init, f, option, none, owns ? () => dispose(f) : undefined);
}

/** `DoubleEndedIterator::try_rfold`, from the back, as `iterTryFold`. */
export function iterTryRfold<T, A, R>(it: Iter<T>, init: A, f: (acc: A, item: T) => R, option = false, owns = false, none?: None): R {
	return tryFold(it, true, init, f, option, none, owns ? () => dispose(f) : undefined);
}

/** `Iterator::try_for_each`, as `iterTryFold` of `()`. */
export function iterTryForEach<T, R>(it: Iter<T>, f: (item: T) => R, option = false, owns = false, none?: None): R {
	return tryFold(it, false, undefined, (_, item: T) => f(item), option, none, owns ? () => dispose(f) : undefined);
}

/** `iterTryFold`, or from the back if `back`, which ends by `release`. */
function tryFold<T, A, R>(it: Iter<T>, back: boolean, init: A, f: (acc: A, item: T) => R, option: boolean, none: None | undefined, release: (() => void) | undefined): R {
	const folded = it.folded();
	const read = back ? () => folded.nextBack() : () => folded.next();
	let acc = init;
	for (let item = read(); item !== DONE; item = read()) {
		const step = f(acc, item);
		if (option ? step === none : !(step as Result<A, unknown>).ok) {
			release?.();
			return step;
		}
		acc = option ? (step as unknown as A) : (step as { value: A }).value;
	}
	release?.();
	return (option ? acc : Ok(acc)) as R;
}

/** `Iterator::for_each`, which drops `f` if it `owns` it, as `iterFold` does. */
export function iterForEach<T>(it: Iter<T>, f: (item: T) => void, drops = false, owns = false): undefined {
	if (!drops) {
		const folded = it.folded(true);
		for (let item = folded.next(); item !== DONE; item = folded.next()) f(item);
		if (owns) dispose(f);
		return undefined;
	}
	forEach(it, f, owns ? () => dispose(f) : undefined);
	return undefined;
}

/**
 * std's `for_each` of `it`, which takes it by value: the translated crate's own, given `f`
 * as `FromNext`'s folds give theirs, or else by `fold`, which drops `release` as it ends.
 */
function forEach<T>(it: Iter<T>, f: (item: T) => void, release?: () => void): void {
	if (it instanceof FromNext && it.own?.forEach !== undefined) {
		it.taken((source) => (it.own!.forEach as (source: unknown, f: (item: T) => void) => void)(source, lend(f, release)));
		return;
	}
	it.fold<void>(undefined, (_, item) => f(item), release);
}

/**
 * `f`, given to the crate's own method as a closure std owns, which drops `release`'s as the
 * crate's drops it, or without `release` as a loan of `f`, which drops nothing.
 */
function lend<F extends (...args: never[]) => unknown>(f: F, release: (() => void) | undefined): F {
	const g = ((...args: never[]) => f(...args)) as F;
	return release === undefined ? g : closure(g, release);
}

/** `sum`, adding with an integer type's checked `add`, so overflow panics as Rust's does. */
export function iterSum<T>(it: Iter<T>, add: (a: T, b: T) => T, zero: T, drops = false): T {
	return iterFold(it, zero, add, drops);
}

/** `product`, multiplying with an integer type's checked `mul`. */
export function iterProduct<T>(it: Iter<T>, mul: (a: T, b: T) => T, one: T, drops = false): T {
	return iterFold(it, one, mul, drops);
}

/** `max`, by `lt`, its items' `PartialOrd::lt`, which `Ord::max` tests: the last of the greatest. */
export function iterMax<T>(it: Iter<T>, lt: Lt<T>, drops?: boolean, drop?: Drop<T>): T | undefined;
export function iterMax<T>(it: Iter<T>, lt: Lt<T>, drops: boolean | undefined, drop: Drop<T> | undefined, none: None): T | None;
export function iterMax<T>(it: Iter<T>, lt: Lt<T>, drops = false, drop?: Drop<T>, none?: None): T | undefined | None {
	return iterToOption(best(it, (item: T) => item, lt, false, drops, drop), none);
}

/** `min`, by `lt`, as `iterMax`: the first of the least. */
export function iterMin<T>(it: Iter<T>, lt: Lt<T>, drops?: boolean, drop?: Drop<T>): T | undefined;
export function iterMin<T>(it: Iter<T>, lt: Lt<T>, drops: boolean | undefined, drop: Drop<T> | undefined, none: None): T | None;
export function iterMin<T>(it: Iter<T>, lt: Lt<T>, drops = false, drop?: Drop<T>, none?: None): T | undefined | None {
	return iterToOption(best(it, (item: T) => item, lt, true, drops, drop), none);
}

/**
 * `max_by_key`, by `lt` of the keys, as `iterMax`, each item's key taken once. If it
 * `owns` `f`, `f` drops as std's `map` of the iterator by it does: as the fold ends, or
 * after `it` if `it` is empty. Each key drops by `dropKey`, as std pairs it with its
 * item: before the item as that loses, and the best's once the fold ends.
 */
export function iterMaxByKey<T, K>(it: Iter<T>, f: (item: T) => K, lt: Lt<K>, drops?: boolean, drop?: Drop<T>, owns?: boolean, dropKey?: Drop<K>): T | undefined;
export function iterMaxByKey<T, K>(it: Iter<T>, f: (item: T) => K, lt: Lt<K>, drops: boolean | undefined, drop: Drop<T> | undefined, owns: boolean | undefined, dropKey: Drop<K> | undefined, none: None): T | None;
export function iterMaxByKey<T, K>(it: Iter<T>, f: (item: T) => K, lt: Lt<K>, drops = false, drop?: Drop<T>, owns = false, dropKey?: Drop<K>, none?: None): T | undefined | None {
	return iterToOption(best(it, f, lt, false, drops, drop, owns ? () => dispose(f) : undefined, true, dropKey), none);
}

/** `min_by_key`, by `lt` of the keys, as `iterMaxByKey`. */
export function iterMinByKey<T, K>(it: Iter<T>, f: (item: T) => K, lt: Lt<K>, drops?: boolean, drop?: Drop<T>, owns?: boolean, dropKey?: Drop<K>): T | undefined;
export function iterMinByKey<T, K>(it: Iter<T>, f: (item: T) => K, lt: Lt<K>, drops: boolean | undefined, drop: Drop<T> | undefined, owns: boolean | undefined, dropKey: Drop<K> | undefined, none: None): T | None;
export function iterMinByKey<T, K>(it: Iter<T>, f: (item: T) => K, lt: Lt<K>, drops = false, drop?: Drop<T>, owns = false, dropKey?: Drop<K>, none?: None): T | undefined | None {
	return iterToOption(best(it, f, lt, true, drops, drop, owns ? () => dispose(f) : undefined, true, dropKey), none);
}

/**
 * `max_by`, by `compare`, as `iterMax`: the last of the greatest. If it `owns` `compare`, `compare`
 * drops as `iterReduce`'s `f` does.
 */
export function iterMaxBy<T>(it: Iter<T>, compare: Compare<T>, drops?: boolean, drop?: Drop<T>, owns?: boolean): T | undefined;
export function iterMaxBy<T>(it: Iter<T>, compare: Compare<T>, drops: boolean | undefined, drop: Drop<T> | undefined, owns: boolean | undefined, none: None): T | None;
export function iterMaxBy<T>(it: Iter<T>, compare: Compare<T>, drops = false, drop?: Drop<T>, owns = false, none?: None): T | undefined | None {
	const release = owns ? () => dispose(compare) : undefined;
	return iterToOption(best(it, (item: T) => item, byCompare(compare), false, drops, drop, release), none);
}

/** `min_by`, by `compare`, as `iterMin`: the first of the least, and as `iterMaxBy`. */
export function iterMinBy<T>(it: Iter<T>, compare: Compare<T>, drops?: boolean, drop?: Drop<T>, owns?: boolean): T | undefined;
export function iterMinBy<T>(it: Iter<T>, compare: Compare<T>, drops: boolean | undefined, drop: Drop<T> | undefined, owns: boolean | undefined, none: None): T | None;
export function iterMinBy<T>(it: Iter<T>, compare: Compare<T>, drops = false, drop?: Drop<T>, owns = false, none?: None): T | undefined | None {
	const release = owns ? () => dispose(compare) : undefined;
	return iterToOption(best(it, (item: T) => item, byCompare(compare), true, drops, drop, release), none);
}

/** A `PartialOrd::lt`. */
type Lt<T> = (a: T, b: T) => boolean;

/** A comparator, as `max_by` and `min_by` call it: of the best so far, and then the next. */
type Compare<T> = (best: T, next: T) => Ordering;

/**
 * `compare` as the `lt` that `best` tests the next and the best by: whether the best is
 * `Greater`, which std's `cmp::max_by` keeps and `min_by` replaces.
 */
function byCompare<T>(compare: Compare<T>): Lt<T> {
	return (next, best) => compare(best, next) > 0;
}

/**
 * The item of the greatest key, the last of equal ones, or of the least if `least`, the
 * first, as std's `reduce` by `Ord::max` or `min` keeps it: the first by `next`, and the
 * rest by `fold`, each tested for `lt` the best's and dropped as it loses. `release` drops
 * the closure std's reduces by as the fold ends, or if `it` is empty, before `it`, or
 * after it if `keyed`, as the `map` std's `_by_key` forms reduce drops its function. Each
 * key drops by `dropKey` just before its item does, and the best's after the fold. Unless
 * `keyed`, std reduces `it` itself, by the translated crate's own `reduce` if it has one.
 */
function best<T, K>(
	it: Iter<T>,
	f: (item: T) => K,
	lt: Lt<K>,
	least: boolean,
	drops: boolean,
	drop?: Drop<T>,
	release?: () => void,
	keyed = false,
	dropKey?: Drop<K>,
): T | Done {
	if (!keyed && it instanceof FromNext && it.own?.reduce !== undefined) {
		const pick = (best: T, item: T): T => {
			const wins = least ? lt(f(item), f(best)) : !lt(f(item), f(best));
			drop?.(wins ? best : item);
			return wins ? item : best;
		};
		return it.reduced(pick, release);
	}
	if (!drops && drop === undefined && dropKey === undefined) {
		let best = it.next();
		if (best !== DONE) {
			let key = f(best);
			const folded = it.folded(true);
			for (let item = folded.next(); item !== DONE; item = folded.next()) {
				const k = f(item);
				if (least ? lt(k, key) : !lt(k, key)) [best, key] = [item, k];
			}
		}
		release?.();
		return best;
	}
	const first = it.next();
	if (first === DONE) {
		if (!keyed) release?.();
		dispose(it);
		if (keyed) release?.();
		return DONE;
	}
	const [key, item] = it.fold<[K, T]>(
		[f(first), first],
		(best, item) => {
			const k = f(item);
			const wins = least ? lt(k, best[0]) : !lt(k, best[0]);
			dropKey?.(wins ? best[0] : k);
			drop?.(wins ? best[1] : item);
			return wins ? [k, item] : best;
		},
		release,
	);
	dropKey?.(key);
	return item;
}

/**
 * `is_sorted_by`: whether `compare` holds of each item and the next, read by `next` and
 * then as `all` folds, or by the translated crate's own `all`, stopping at the first it
 * fails. Each item drops by `drop` as the next replaces it, or after the one it failed
 * against; then `compare` by `dropCompare`, as `all` drops it, the last item, and `it`, if
 * `drops`. The translated crate's own `is_sorted_by`, which `is_sorted` calls, takes `it`
 * by value in its place.
 */
export function iterIsSortedBy<T>(
	it: Iter<T>,
	compare: (a: T, b: T) => boolean,
	drops = false,
	drop?: Drop<T>,
	dropCompare?: Drop<(a: T, b: T) => boolean>,
): boolean {
	if (it instanceof FromNext && it.own?.isSortedBy !== undefined) {
		const sorted = it.own.isSortedBy as (source: unknown, f: (a: T, b: T) => boolean) => boolean;
		const dropped = dropCompare;
		const release = dropped === undefined ? undefined : () => dropped(compare);
		return it.taken((source) => sorted(source, lend(compare, release)));
	}
	let last = it.next();
	let sorted = true;
	if (last !== DONE && it instanceof FromNext && it.own?.all !== undefined) {
		const check = (item: T): boolean => {
			if (!compare(last as T, item)) {
				drop?.(item);
				return false;
			}
			drop?.(last as T);
			last = item;
			return true;
		};
		const dropped = dropCompare;
		const release = dropped === undefined ? undefined : () => dropped(compare);
		sorted = it.allOwn(check, release);
		// The crate's `all` dropped it.
		dropCompare = undefined;
	} else if (last !== DONE) {
		const folded = it.folded();
		for (let item = folded.next(); item !== DONE; item = folded.next()) {
			if (!compare(last, item)) {
				drop?.(item);
				sorted = false;
				break;
			}
			drop?.(last);
			last = item;
		}
	}
	dropCompare?.(compare);
	if (last !== DONE) drop?.(last);
	if (drops) dispose(it);
	return sorted;
}

/**
 * What std's comparisons of iterators drop, after the iterators and the comparator: whether
 * the first iterator drops, whether the second does, which drops before it, and whether
 * the comparator owns values that drop, as a `_by` form's may. Then what drops an item of
 * each that std drops itself: one of the first read past the second's end, and one of the
 * second read past the first's.
 */
type Compared<T, U> = [drops?: boolean, dropsOther?: boolean, owns?: boolean, dropX?: Drop<T>, dropY?: Drop<U>];

/**
 * std's `iter_compare`: each item of `a` as `try_fold` reads it, against the next of `b`,
 * by `f`, which takes both, until it finds them other than `Equal`. If `b` ends first, the
 * item of `a` read past it drops by `dropX`, and `a` is `Greater`. `release` drops what
 * `f` owns once `a`'s fold ends, as std drops the closure there; then, if `a` ended, `b`
 * reads once more: `Less` if it has another, which drops by `dropY`, and else `Equal`.
 */
function iterCompare<T, U, O extends Ordering | undefined>(
	a: Iter<T>,
	b: Iter<U>,
	f: (x: T, y: U) => O,
	release?: () => void,
	dropX?: Drop<T>,
	dropY?: Drop<U>,
): O {
	const folded = a.folded();
	for (let x = folded.next(); x !== DONE; x = folded.next()) {
		const y = b.next();
		if (y === DONE) {
			dropX?.(x);
			release?.();
			return 1 as O;
		}
		const order = f(x, y);
		if (order !== 0) {
			release?.();
			return order;
		}
	}
	release?.();
	const y = b.next();
	if (y === DONE) return 0 as O;
	dropY?.(y);
	return -1 as O;
}

/** `f` as std's own closure of a comparison, which drops what it is given, the second first, once it compares them. */
export function consuming<T, U, R>(f: (x: T, y: U) => R, dropX?: Drop<T>, dropY?: Drop<U>): (x: T, y: U) => R {
	return (x, y) => {
		const result = f(x, y);
		dropY?.(y);
		dropX?.(x);
		return result;
	};
}

/**
 * `Iterator::cmp`, `partial_cmp`, and their `_by` forms: `iter_compare` of `a` and `b`
 * by `cmp`, and then `b` and `a` dropped as `Compared` says.
 */
export function iterCmp<T, U, O extends Ordering | undefined>(
	a: Iter<T>,
	b: Iter<U>,
	cmp: (x: T, y: U) => O,
	...[drops = false, dropsOther = false, owns = false, dropX, dropY]: Compared<T, U>
): O {
	const order = iterCompare(a, b, cmp, owns ? () => dispose(cmp) : undefined, dropX, dropY);
	if (dropsOther) dispose(b);
	if (drops) dispose(a);
	return order;
}

/**
 * `Iterator::eq` and `eq_by`: `iter_compare` of `a` and `b` until `eq` fails. If `trusted`,
 * as std's of two `TrustedLen` iterators, unequal at once, reading neither, unless their
 * size hints' upper bounds are the same or both unknown.
 */
export function iterEq<T, U>(a: Iter<T>, b: Iter<U>, eq: (x: T, y: U) => boolean, trusted = false, ...[drops = false, dropsOther = false, owns = false, dropX, dropY]: Compared<T, U>): boolean {
	const release = owns ? () => dispose(eq) : undefined;
	let equal = false;
	if (trusted && a.sizeHint()[1] !== b.sizeHint()[1]) release?.();
	else equal = iterCompare(a, b, (x: T, y: U) => (eq(x, y) ? 0 : 1), release, dropX, dropY) === 0;
	if (dropsOther) dispose(b);
	if (drops) dispose(a);
	return equal;
}

/**
 * The capacity std's `collect` of `it` into a `Vec` of `size`-byte elements keeps of the
 * `Vec` of `src`-byte ones it writes in place of, read before it reads `it`: through
 * adaptors, all of its bytes, and of that `Vec`'s own `into_iter`, its buffer while unread
 * at the front or at least half full, and else room reserved for what is left.
 */
function keptCapacity(it: Iter<unknown>, src: number, size: number): number | undefined {
	let from: Iter<unknown> | undefined = it;
	while (from !== undefined && !(from instanceof SliceIter)) {
		// std's `SourceIter` reads through each adaptor to the iterator it wraps, a zip's first.
		const wraps = from as unknown as { inner?: Iter<unknown>; a?: Iter<unknown>; outer?: Iter<unknown> };
		from = wraps.inner ?? wraps.a ?? wraps.outer;
	}
	if (from === undefined) return undefined;
	const cap = from.v === undefined ? from.s.length : vecCapacity(from.v, src);
	if (from !== it) return Math.floor((cap * src) / size);
	const left = from.back - from.front;
	return from.front === 0 || left >= Math.floor(cap / 2) ? cap : capReserve(0, 0, left, size);
}

/** `collect` into a `Vec`, as `iterExtend` of an empty one, in place of a `Vec` of `src`-byte elements if given. */
export function iterToVec<T>(it: Iter<T>, take?: Take, size?: number, trusted = false, src?: number): Vec<T> {
	const kept = src === undefined || size === undefined ? undefined : keptCapacity(it, src, size);
	const out = toArray(it, take, size, trusted);
	if (kept !== undefined) vecHold(out, kept);
	return out;
}

function toArray<T>(it: Iter<T>, take?: Take, size?: number, trusted = false): Vec<T> {
	if (size !== undefined && !trusted) {
		const d: T[] = [];
		// std's reads the size hint once it has the first item, zero-sized ones' included,
		// whose capacity is then `usize::MAX`.
		let cap = 0;
		for (let item = it.next(); item !== DONE; item = it.next()) {
			if (d.length === cap && size === 0) {
				it.sizeHint();
				cap = USIZE_MAX;
			} else if (d.length === cap) cap = capNext(it, d.length, cap, size);
			d.push(item);
		}
		if (take !== undefined) dispose(it);
		return new Vec(d, cap);
	}
	// std's makes one with room for exactly the upper bound.
	const out = size === undefined ? new Vec<T>() : vecWithCapacity<T>(trustedUpper(it), size);
	const d = out.items;
	if (take === undefined) {
		const folded = it.folded(trusted);
		for (let item = folded.next(); item !== DONE; item = folded.next()) d.push(item);
		return out;
	}
	each(it, (item) => d.push(item), take);
	return out;
}

/** `collect` into a `Vec<u8>`, as `iterExtendBytes` of an empty one, in place of a `Vec` of `src`-byte elements if given. */
export function iterToByteVec(it: Iter<number>, take?: Take, trusted = false, src?: number): ByteVec {
	const kept = src === undefined ? undefined : keptCapacity(it, src, 1);
	const out = toByteVec(it, take, trusted);
	if (kept !== undefined && kept > out.data.length) byteVecReserveExact(out, kept - out.length);
	else if (kept !== undefined && kept < out.data.length) byteVecShrinkTo(out, kept);
	return out;
}

function toByteVec(it: Iter<number>, take?: Take, trusted = false): ByteVec {
	// Its own loops: through `iterExtendBytes`, JavaScriptCore took 1.9x as long.
	if (!trusted) {
		const out = new ByteVec();
		for (let item = it.next(); item !== DONE; item = it.next()) {
			if (out.length === out.data.length) byteVecReserve(out, satAdd(it.sizeHint()[0], 1));
			byteVecPush(out, item);
		}
		if (take !== undefined) dispose(it);
		return out;
	}
	const out = byteVecWithCapacity(trustedUpper(it));
	if (take === undefined) {
		const folded = it.folded(true);
		for (let item = folded.next(); item !== DONE; item = folded.next()) byteVecPush(out, item);
		return out;
	}
	each(it, (item) => byteVecPush(out, item), take);
	return out;
}

/**
 * `collect` of `Result`s: the last error `into` read, or every value collected by `into`,
 * which drops the iterator. On an error, what it collected drops by `drop`, and an error a
 * later one replaced by `dropError`.
 */
export function iterCollectResult<U, E, C>(it: Iter<Result<U, E>>, into: (values: Iter<U>) => C, drop?: Drop<C>, dropError?: Drop<E>): Result<C, E> {
	const shunt = new ResultShunt(it, dropError);
	const values = into(shunt);
	if (shunt.error === undefined) return Ok(values);
	const error = Err(shunt.error.value);
	drop?.(values);
	return error;
}

/**
 * `collect` of `Option`s: `None` at the first `None`, or every value collected by `into`,
 * as `iterCollectResult`. An item's `None` is `itemNone`, and the result's `none`, each
 * `rs.None` where `undefined` is a value.
 */
export function iterCollectOption<U, C>(it: Iter<U | undefined>, into: (values: Iter<U>) => C, drop?: Drop<C>): C | undefined;
export function iterCollectOption<U, C>(it: Iter<U | None>, into: (values: Iter<U>) => C, drop: Drop<C> | undefined, itemNone: None): C | undefined;
export function iterCollectOption<U, C>(it: Iter<U | undefined | None>, into: (values: Iter<U>) => C, drop: Drop<C> | undefined, itemNone: None | undefined, none: None): C | None;
export function iterCollectOption<U, C>(it: Iter<U | undefined | None>, into: (values: Iter<U>) => C, drop?: Drop<C>, itemNone?: None, none?: None): C | undefined | None {
	const shunt = new OptionShunt(it, itemNone);
	const values = into(shunt);
	if (!shunt.none) return values;
	drop?.(values);
	return none;
}

// What std's iterators override to count or take the last: `count` and `last`, each of
// which drops items in an order of its own kind's. These follow it, dispatching on the
// kind, rather than as methods every iterator would carry, and the translation calls them
// only where an item or the iterator drops.

/** `Iterator::count` of an iterator that drops what it holds: each item by `drop`, as its own `count` does, and then it. */
export function ownedCount<T>(it: Iter<T>, drop?: Drop<T>, at?: Location): number {
	return count(it, drop, at);
}

/** `Iterator::last` of an iterator that drops what it holds: each item but the last by `drop`, as its own `last` does, and then it. */
export function ownedLast<T>(it: Iter<T>, drop?: Drop<T>, none?: undefined, at?: Location): T | undefined;
export function ownedLast<T>(it: Iter<T>, drop: Drop<T> | undefined, none: None, at?: Location): T | None;
export function ownedLast<T>(it: Iter<T>, drop?: Drop<T>, none?: None, at?: Location): T | undefined | None {
	return iterToOption(last(it, drop, at), none);
}

/** `min`, or with `back`, `max`, of a B-tree's `into_keys` that drops what it holds: an end, as std's reads it, and then the rest drop with it. */
export function ownedEnd<T>(it: Iter<T>, back: boolean): T | undefined;
export function ownedEnd<T>(it: Iter<T>, back: boolean, none: None): T | None;
export function ownedEnd<T>(it: Iter<T>, back: boolean, none?: None): T | undefined | None {
	return iterToOption(consumed(it, () => (back ? it.nextBack() : it.next())), none);
}

/** `count`, dropping each item by `drop`, and then `it`, as std's of its kind does. */
function count<T>(it: Iter<T>, drop: Drop<T> | undefined, at?: Location): number {
	if (it instanceof RepeatIter) panic("iterator is infinite", at);
	// std's: its length, and then its element drops with it.
	if (it instanceof RepeatNIter) return consumed(it, () => it.left);
	if (it instanceof FromNext && it.own?.count !== undefined) return it.taken(it.own.count);
	if (it instanceof EnumerateIter) return count(it.inner, items(drop), at);
	if (it instanceof ChainIter) {
		// std's: each iterator's own `count`, the first's first.
		const { a, b } = it;
		it.a = it.b = undefined;
		const n = a === undefined ? 0 : count(a, drop, at);
		return n + (b === undefined ? 0 : count(b, drop, at));
	}
	if (it instanceof SkipIter) {
		// std's: past what it skips by `nth`, and then its iterator's own `count`.
		return skipped(it, drop) ? 0 : count(it.inner, drop, at);
	}
	if (it instanceof TakeIter) {
		// std's: its iterator advanced through the limit, dropping what it passes.
		const n = it.n;
		it.n = 0;
		return consumed(it.inner, () => (n === 0 ? 0 : n - it.inner.advanceBy(n, drop)));
	}
	if (it instanceof FuseIter) {
		const inner = it.inner;
		it.inner = undefined;
		return inner === undefined ? 0 : count(inner, drop, at);
	}
	if (it instanceof FlattenIter) {
		// std's: each iterator's own `count` in turn.
		return flattenFold(it as FlattenIter<unknown, T>, false, 0, (n, inner) => n + count(inner, drop, at));
	}
	if (it instanceof Peekable) {
		// std's: the item it peeked drops after its iterator is counted.
		if (!it.peeked) return count(it.inner, drop, at);
		const peeked = it.take();
		if (peeked === DONE) {
			dispose(it.inner);
			return 0;
		}
		const n = 1 + count(it.inner, drop, at);
		drop?.(peeked);
		return n;
	}
	if (it instanceof HashIter && !it.drain) {
		// std's `into_iter`'s: its length, and then each entry drops with it.
		const n = it.left;
		dispose(it);
		return n;
	}
	return it.fold(0, (n, item) => {
		drop?.(item);
		return n + 1;
	});
}

/**
 * `last`, dropping each item but the last by `drop`, and then `it`, as std's of its kind
 * does. By `fold`, each item drops as the next replaces it.
 */
function last<T>(it: Iter<T>, drop: Drop<T> | undefined, at?: Location): T | Done {
	if (it instanceof FromNext && it.own?.last !== undefined) return it.taken(it.own.last);
	// std's of a box: what it holds, by its own.
	if (it instanceof Boxed) return last(it.inner, drop, at);
	if (it instanceof RepeatIter) panic("iterator is infinite", at);
	// std's: its element, uncloned.
	if (it instanceof RepeatNIter) return it.left === 0 ? DONE : it.take();
	if ((it instanceof SliceIter && drop !== undefined) || (it instanceof BTreeIter && it.back)) {
		// std's, of a `Vec`'s or an array's, and of a B-tree's `into_keys` and
		// `into_values`: the last item, and then the rest drop with it.
		const item = it.nextBack();
		dispose(it);
		return item;
	}
	if (it instanceof ChainIter) {
		// std's: each iterator's last, the first's held while the second is read, and
		// dropped once the second has one.
		const { a, b } = it;
		it.a = it.b = undefined;
		const aLast = a === undefined ? DONE : last(a, drop, at);
		if (b === undefined) return aLast;
		return or(last(b, drop, at), aLast, drop);
	}
	if (it instanceof SkipIter) {
		// std's: past what it skips by `nth`, and then its iterator's own `last`.
		return skipped(it, drop) ? DONE : last(it.inner, drop, at);
	}
	if (it instanceof FuseIter) {
		const inner = it.inner;
		it.inner = undefined;
		return inner === undefined ? DONE : last(inner, drop, at);
	}
	if (it instanceof FlattenIter) {
		// std's: the last of each iterator in turn, the last so far dropped once a later
		// one has one.
		return flattenFold<unknown, T, T | Done>(it as FlattenIter<unknown, T>, false, DONE, (prev, inner) => or(last(inner, drop, at), prev, drop));
	}
	if (it instanceof Peekable) {
		// std's: the item it peeked, unless its iterator has a last, which drops it after.
		let peeked: T | Done = DONE;
		if (it.peeked) {
			peeked = it.take();
			if (peeked === DONE) {
				dispose(it.inner);
				return DONE;
			}
		}
		return or(last(it.inner, drop, at), peeked, drop);
	}
	return it.fold<T | Done>(DONE, (prev, item) => {
		if (prev !== DONE) drop?.(prev);
		return item;
	});
}

/** `item.or(prev)`: `item`, unless that is `DONE`, and then `prev`, which drops by `drop` if not. */
function or<T>(item: T | Done, prev: T | Done, drop: Drop<T> | undefined): T | Done {
	if (item === DONE) return prev;
	if (prev !== DONE) drop?.(prev);
	return item;
}

/** `f()`, a consumer of `it` by value, and then `it` dropped. */
function consumed<T, R>(it: Iter<T>, f: () => R): R {
	const r = f();
	dispose(it);
	return r;
}

/** What drops an `enumerate`'s item, of one its iterator yields: `drop` is of `[number, T]`. */
function items(drop: Drop<never> | undefined): Drop<unknown> | undefined {
	return drop === undefined ? undefined : (item) => drop([0, item] as never);
}

/**
 * Read past what `it` still skips, as `nth` reads, and whether its iterator ended first,
 * which then drops.
 */
function skipped<T>(it: SkipIter<T>, drop: Drop<T> | undefined): boolean {
	if (it.n === 0) return false;
	const n = it.n;
	it.n = 0;
	const item = it.inner.nth(n - 1, drop);
	if (item !== DONE) {
		drop?.(item);
		return false;
	}
	dispose(it.inner);
	return true;
}

/**
 * std's `iter_fold` of a flatten, or `iter_rfold` if `back`: `g` of each iterator it holds
 * in turn, the front, then each of its source's by the source's own fold, and the back.
 * `release` drops what `g` owns as it ends, before what it still holds.
 */
function flattenFold<T, V, A>(it: FlattenIter<T, V>, back: boolean, acc: A, g: (acc: A, inner: Iter<V>) => A, release?: () => void): A {
	const { outer } = it;
	const [first, rest] = back ? [it.back, it.front] : [it.front, it.back];
	it.outer = it.front = it.back = undefined;
	if (first !== undefined) acc = g(acc, first);
	if (outer !== undefined) {
		const each = (acc: A, item: T) => g(acc, it.into(item));
		acc = back ? outer.rfold(acc, each) : outer.fold(acc, each);
	}
	if (rest !== undefined) acc = g(acc, rest);
	release?.();
	return acc;
}

// Iterators. One has `[Symbol.dispose]` if it owns items or wraps an iterator: what holds
// it drops it by `rs.drop`, not knowing its kind.

class SliceIter<T> extends Iter<T> implements Disposable {
	readonly s: ArrayLike<T>;
	/** The `Vec` whose items `s` is, as its `into_iter`, whose capacity a `collect` in place keeps. */
	v: Vec<T> | undefined;
	front = 0;
	back: number;
	/** What drops each item it still holds, if it owns them. */
	private readonly drop: Drop<T> | undefined;

	constructor(s: ArrayLike<T>, drop?: Drop<T>) {
		super();
		this.s = s;
		this.back = s.length;
		this.drop = drop;
	}

	next(): T | Done {
		return this.front < this.back ? this.s[this.front++] : DONE;
	}

	override nextBack(): T | Done {
		return this.front < this.back ? this.s[--this.back] : DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	override remaining(): Iterable<T> {
		return Array.prototype.slice.call(this.s, this.front, this.back);
	}

	/** std's: what it has left, each cloned in order by `h.v` into storage of its own, of exactly their capacity, or else shared. */
	override clone(h?: Clones): Iter<T> {
		if (h?.v !== undefined) {
			const items = Array.prototype.slice.call(this.s, this.front, this.back) as T[];
			for (let i = 0; i < items.length; i++) items[i] = cloned(items[i], h.v);
			return vecIntoIter(vecFrom(items), this.drop);
		}
		const it = new SliceIter(this.s, this.drop);
		it.front = this.front;
		it.back = this.back;
		return it;
	}

	[Symbol.dispose](): void {
		if (this.drop === undefined) return;
		const from = this.front;
		this.front = this.back;
		dropRange(this.s, this.drop, from, this.back);
	}

	/** std's, of a `Vec`'s: past them all first, then each dropped, as a slice drops. */
	override advanceBy(n: number, drop?: Drop<T>): number {
		const step = Math.min(n, this.back - this.front);
		const from = this.front;
		this.front += step;
		if (drop !== undefined) dropRange(this.s, drop, from, this.front);
		return n - step;
	}

	/** std's: those it skips drop front to back, as a slice drops. */
	override advanceBackBy(n: number, drop?: Drop<T>): number {
		const step = Math.min(n, this.back - this.front);
		this.back -= step;
		if (drop !== undefined) dropRange(this.s, drop, this.back, this.back + step);
		return n - step;
	}
}

/** The arrays of a slice of arrays, from both ends, each lent where the slice holds it. */
class ArraySliceIter<S extends ArrayRef> extends Iter<S> {
	private readonly r: ArraySlice<S>;
	front = 0;
	back: number;

	constructor(r: ArraySlice<S>) {
		super();
		this.r = r;
		this.back = r.length;
	}

	next(): S | Done {
		return this.front < this.back ? arraySliceAt(this.r, this.front++) : DONE;
	}

	override nextBack(): S | Done {
		return this.front < this.back ? arraySliceAt(this.r, --this.back) : DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	override remaining(): Iterable<unknown> {
		return arraySliceItems(arraySlicePart(this.r, this.front, this.back));
	}

	override clone(): Iter<S> {
		const it = new ArraySliceIter(this.r);
		it.front = this.front;
		it.back = this.back;
		return it;
	}

	override advanceBy(n: number): number {
		const step = Math.min(n, this.back - this.front);
		this.front += step;
		return n - step;
	}

	override advanceBackBy(n: number): number {
		const step = Math.min(n, this.back - this.front);
		this.back -= step;
		return n - step;
	}
}

/** The places of a slice's elements, from both ends. */
class SliceMutIter<T> extends Iter<Place<T>> {
	private readonly s: { [i: number]: T; length: number };
	front = 0;
	back: number;

	constructor(s: { [i: number]: T; length: number }) {
		super();
		this.s = s;
		this.back = s.length;
	}

	next(): Place<T> | Done {
		return this.front < this.back ? new AtIndex(this.s, this.front++) : DONE;
	}

	override nextBack(): Place<T> | Done {
		return this.front < this.back ? new AtIndex(this.s, --this.back) : DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	override remaining(): Iterable<T> {
		return Array.prototype.slice.call(this.s, this.front, this.back);
	}

	override advanceBy(n: number): number {
		const step = Math.min(n, this.back - this.front);
		this.front += step;
		return n - step;
	}

	override advanceBackBy(n: number): number {
		const step = Math.min(n, this.back - this.front);
		this.back -= step;
		return n - step;
	}
}

/**
 * `slice::Windows` and `Chunks` of `s`, of `length` elements: the part from each `step`th
 * element on, `size` long or up to the end, as `part` slices it, while one starts before
 * the end and, for windows, which step by 1, is `size` long.
 */
class PartsIter<S> extends Iter<S> {
	private readonly s: S;
	private readonly length: number;
	readonly size: number;
	private readonly step: number;
	private readonly count: number;
	/** Whether the first part is at the back of the slice, as `rchunks`' is. */
	private readonly rev: boolean;
	private readonly part: (s: S, start: number, end: number) => S;
	private front = 0;
	/** How many parts there are, less those taken from the back. */
	private back: number;
	/** Whether a read past the end emptied what it has left, as std's `nth` and `next_back` of a `Windows` do. */
	private cleared = false;

	constructor(s: S, length: number, size: number, step: number, count: number, rev: boolean, part: (s: S, start: number, end: number) => S) {
		super();
		this.s = s;
		this.length = length;
		this.size = size;
		this.step = step;
		this.count = count;
		this.rev = rev;
		this.part = part;
		this.back = count;
	}

	private at(i: number): S {
		if (this.rev) {
			const end = this.length - i * this.step;
			return this.part(this.s, Math.max(end - this.size, 0), end);
		}
		const start = i * this.step;
		return this.part(this.s, start, Math.min(start + this.size, this.length));
	}

	override clone(): Iter<S> {
		const it = new PartsIter(this.s, this.length, this.size, this.step, this.count, this.rev, this.part);
		it.front = this.front;
		it.back = this.back;
		it.cleared = this.cleared;
		return it;
	}

	/** What std's holds of the slice: what the parts it has left cover, or nothing once a read past the end emptied it. */
	rest(): S {
		if (this.cleared) return this.part(this.s, 0, 0);
		if (this.rev) {
			const start = Math.max(this.length - this.back * this.step, 0);
			return this.part(this.s, start, Math.max(this.length - this.front * this.step, start));
		}
		const end = Math.min(this.back * this.step + this.size - this.step, this.length);
		return this.part(this.s, Math.min(this.front * this.step, end), end);
	}

	/** What the parts leave over, at the end the parts don't start from. */
	remainder(): S {
		const held = this.count * this.size;
		return this.rev ? this.part(this.s, 0, this.length - held) : this.part(this.s, held, this.length);
	}

	next(): S | Done {
		return this.front < this.back ? this.at(this.front++) : DONE;
	}

	override nextBack(): S | Done {
		if (this.front < this.back) return this.at(--this.back);
		this.cleared = true;
		return DONE;
	}

	override nth(n: number): S | Done {
		if (n < this.len()) {
			this.front += n;
			return this.next();
		}
		this.front = this.back;
		this.cleared = true;
		return DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	override advanceBy(n: number): number {
		const step = Math.min(n, this.back - this.front);
		this.front += step;
		return n - step;
	}

	/** std's reads by `next_back`, so one past the end empties it. */
	override advanceBackBy(n: number): number {
		const step = Math.min(n, this.back - this.front);
		this.back -= step;
		if (step < n) this.cleared = true;
		return n - step;
	}
}

/**
 * `slice::Split` and its forms of `s`, whose elements `items` holds from `offset`: the
 * parts between the elements `f` is true of, as `part` slices them, read as std's are.
 * Each read searches what is left from its end, calling `f` on each element in turn up to
 * the first it is true of, and keeps what it passed by in the part. A part found from the
 * back of a `split_inclusive` ends with an element the search leaves out, so a read from
 * the front may call `f` on it again, as std's does.
 */
class SplitIter<S, T> extends Iter<S> implements Disposable {
	readonly #s: S;
	/** The items, from `#offset`, or what reads item `i`, as an array slice lends each array only once read. */
	readonly #items: ArrayLike<T> | ((i: number) => T);
	readonly #offset: number;
	readonly #part: (s: S, start: number, end: number) => S;
	readonly #f: (item: T) => boolean;
	/** Whether `next` reads from the back, as `rsplit`'s does. */
	readonly #rev: boolean;
	/** Whether each part ends with its separator, as `split_inclusive`'s does. */
	readonly #inclusive: boolean;
	/** How many more parts a `splitn` yields, the last of them all that is left. */
	#count: number | undefined;
	/** What is left to split: `s[start..end]`. */
	#start = 0;
	#end: number;
	#finished: boolean;
	#dropped = false;

	constructor(s: S, items: ArrayLike<T> | ((i: number) => T), offset: number, part: (s: S, start: number, end: number) => S, f: (item: T) => boolean, rev: boolean, inclusive: boolean, count?: number) {
		super();
		this.#s = s;
		this.#items = items;
		this.#offset = offset;
		this.#part = part;
		this.#f = f;
		this.#rev = rev;
		this.#inclusive = inclusive;
		this.#count = count;
		this.#end = (s as { length: number }).length;
		// An empty slice has no part to include a separator in.
		this.#finished = inclusive && this.#end === 0;
	}

	/** Item `i` of what is split. */
	#at(i: number): T {
		const items = this.#items;
		return typeof items === "function" ? items(i) : items[this.#offset + i];
	}

	override clone(h?: Clones): Iter<S> {
		const it = new SplitIter(this.#s, this.#items, this.#offset, this.#part, cloned(this.#f, h?.f), this.#rev, this.#inclusive, this.#count);
		it.#start = this.#start;
		it.#end = this.#end;
		it.#finished = this.#finished;
		return it;
	}

	next(): S | Done {
		if (this.#count !== undefined) {
			if (this.#count === 0) return DONE;
			if (this.#count-- === 1) return this.#finish();
		}
		return this.#rev ? this.#back() : this.#front();
	}

	override nextBack(): S | Done {
		return this.#rev ? this.#front() : this.#back();
	}

	override sizeHint(): SizeHint {
		const left = this.#end - this.#start;
		// No separator yields one part, and all of them one more than there are elements.
		const upper = this.#inclusive ? Math.max(1, left) : left + 1;
		const [lower, most]: SizeHint = this.#finished ? [0, 0] : [1, upper];
		const n = this.#count;
		return n === undefined ? [lower, most] : [Math.min(n, lower), Math.min(n, most as number)];
	}

	/** What is left to split, or nothing once the last part is taken. */
	asSlice(): S {
		return this.#part(this.#s, this.#start, this.#finished ? this.#start : this.#end);
	}

	/** What std's holds of the slice: what is left to split, which only a `&mut` one gives up as it finishes, if `taken`. */
	rest(taken: boolean): S {
		return taken ? this.asSlice() : this.#part(this.#s, this.#start, this.#end);
	}

	/** Whether it has yielded its last part. */
	get finished(): boolean {
		return this.#finished;
	}

	/** How many more parts a `splitn` yields. */
	get count(): number | undefined {
		return this.#count;
	}

	/** The rest, as the last part. */
	#finish(): S | Done {
		if (this.#finished) return DONE;
		this.#finished = true;
		return this.#part(this.#s, this.#start, this.#end);
	}

	#front(): S | Done {
		if (this.#finished) return DONE;
		const start = this.#start;
		let i = start;
		while (i < this.#end && !this.#f(this.#at(i))) i++;
		if (this.#inclusive) {
			const end = Math.min(i + 1, this.#end);
			if (end === this.#end) this.#finished = true;
			this.#start = end;
			return this.#part(this.#s, start, end);
		}
		if (i === this.#end) return this.#finish();
		this.#start = i + 1;
		return this.#part(this.#s, start, i);
	}

	#back(): S | Done {
		if (this.#finished) return DONE;
		const end = this.#end;
		// The last element ends an inclusive part whether it is a separator or not.
		let i = this.#inclusive ? Math.max(end - 1, this.#start) : end;
		while (i > this.#start && !this.#f(this.#at(i - 1))) i--;
		if (this.#inclusive) {
			if (i === this.#start) this.#finished = true;
			this.#end = i;
			return this.#part(this.#s, i, end);
		}
		if (i === this.#start) return this.#finish();
		this.#end = i - 1;
		return this.#part(this.#s, i, end);
	}

	[Symbol.dispose](): void {
		if (this.#dropped) return;
		this.#dropped = true;
		dispose(this.#f);
	}
}

/** `option::IntoIter`, `Once`, and `Empty`: at most one item. */
class OptionIter<T> extends Iter<T> implements Disposable {
	private item: T | undefined;
	private some: boolean;
	private readonly drop: Drop<T> | undefined;

	constructor(item: T | undefined, some: boolean, drop?: Drop<T>) {
		super();
		this.item = item;
		this.some = some;
		this.drop = drop;
	}

	next(): T | Done {
		if (!this.some) return DONE;
		this.some = false;
		const item = this.item as T;
		this.item = undefined;
		return item;
	}

	override nextBack(): T | Done {
		return this.next();
	}

	override len(): number {
		return this.some ? 1 : 0;
	}

	override remaining(): Iterable<T> {
		return this.some ? [this.item as T] : [];
	}

	override clone(h?: Clones): Iter<T> {
		return new OptionIter(this.some ? cloned(this.item as T, h?.v) : undefined, this.some, this.drop);
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	[Symbol.dispose](): void {
		if (this.drop === undefined) return;
		const item = this.next();
		if (item !== DONE) this.drop(item);
	}
}

/** `Repeat`, holding `element`, which `clone` clones for each item, and `drop` drops with it. */
class RepeatIter<T> extends Iter<T> implements Disposable {
	readonly element: T;
	private readonly copy: ((element: T) => T) | undefined;
	private readonly drop: Drop<T> | undefined;

	constructor(element: T, clone?: (element: T) => T, drop?: Drop<T>) {
		super();
		this.element = element;
		this.copy = clone;
		this.drop = drop;
	}

	next(): T | Done {
		return cloned(this.element, this.copy);
	}

	override nextBack(): T | Done {
		return this.next();
	}

	/** std's skips nothing, as each item is the same. */
	override advanceBy(): number {
		return 0;
	}

	override advanceBackBy(): number {
		return 0;
	}

	override sizeHint(): SizeHint {
		return [USIZE_MAX, undefined];
	}

	override clone(): Iter<T> {
		return new RepeatIter(cloned(this.element, this.copy), this.copy, this.drop);
	}

	[Symbol.dispose](): void {
		this.drop?.(this.element);
	}
}

/**
 * `RepeatN`, holding `element` while it has items `left`: each a clone by `clone` but the
 * last, which is `element` itself. `element` drops by `drop` as an advance passes the last,
 * or with it.
 */
class RepeatNIter<T> extends Iter<T> implements Disposable {
	element: T | undefined;
	left: number;
	private readonly copy: ((element: T) => T) | undefined;
	private readonly drop: Drop<T> | undefined;

	constructor(element: T, n: number, clone?: (element: T) => T, drop?: Drop<T>) {
		super();
		this.element = n === 0 ? undefined : element;
		this.left = n;
		this.copy = clone;
		this.drop = drop;
	}

	next(): T | Done {
		if (this.left === 0) return DONE;
		if (--this.left > 0) return cloned(this.element as T, this.copy);
		return this.take();
	}

	override nextBack(): T | Done {
		return this.next();
	}

	/** std's: what it has left less `n`, or else none, `element` dropped. */
	override advanceBy(n: number): number {
		if (this.left > n) {
			this.left -= n;
			return 0;
		}
		const short = n - this.left;
		if (this.left > 0) this.dropElement();
		return short;
	}

	override advanceBackBy(n: number): number {
		return this.advanceBy(n);
	}

	override len(): number {
		return this.left;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.left);
	}

	override clone(): Iter<T> {
		const element = this.left === 0 ? undefined : cloned(this.element as T, this.copy);
		return new RepeatNIter(element as T, this.left, this.copy, this.drop);
	}

	/** `element`, which it no longer holds. */
	take(): T {
		const element = this.element as T;
		this.element = undefined;
		this.left = 0;
		return element;
	}

	/** Drop `element`, which it holds. */
	private dropElement(): void {
		const element = this.take();
		this.drop?.(element);
	}

	[Symbol.dispose](): void {
		if (this.left > 0) this.dropElement();
	}
}

/** `OnceWith`, holding `make` until it is first read, which calls it. */
class OnceWithIter<T> extends Iter<T> implements Disposable {
	make: (() => T) | undefined;

	constructor(make: (() => T) | undefined) {
		super();
		this.make = make;
	}

	next(): T | Done {
		const make = this.make;
		if (make === undefined) return DONE;
		this.make = undefined;
		return make();
	}

	override nextBack(): T | Done {
		return this.next();
	}

	override len(): number {
		return this.make === undefined ? 0 : 1;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	override clone(h?: Clones): Iter<T> {
		return new OnceWithIter(this.make === undefined ? undefined : cloned(this.make, h?.f));
	}

	[Symbol.dispose](): void {
		const make = this.make;
		this.make = undefined;
		dispose(make);
	}
}

/** `Debug` of a `Repeat`: its element. */
export function fmtRepeat<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>): FmtResult {
	return fmtDebugStructFields(f, "Repeat", ["element"], [(f) => debug((it as RepeatIter<T>).element, f)]);
}

/** `Debug` of a `RepeatN`: how many it has left, and its element while it holds it. */
export function fmtRepeatN<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>): FmtResult {
	const r = it as RepeatNIter<T>;
	const element: Fmt = (f) => (r.left === 0 ? f.out.writeStr("None") : fmtDebugTupleFields(f, "Some", [(f) => debug(r.element as T, f)]));
	return fmtDebugStructFields(f, "RepeatN", ["count", "element"], [(f) => usizeDebug(r.left, f), element]);
}

/** `Debug` of a `OnceWith`: whether it holds its closure, which std writes as `_`, unpadded. */
export function fmtOnceWith(it: Iter<unknown>, f: Formatter): FmtResult {
	return f.out.writeStr((it as OnceWithIter<unknown>).make === undefined ? "OnceWith(None)" : "OnceWith(Some(_))");
}

/**
 * An iterator whose `next` is a function of its source, which `drop` drops, and which `own`
 * reads by the methods it has of its own.
 */
class FromNext<S, T> extends Iter<T> implements Disposable {
	readonly source: S;
	private readonly step: (source: S) => T | Done;
	private drop: Drop<S> | undefined;
	readonly own: Own<S, T> | undefined;

	constructor(source: S, step: (source: S) => T | Done, drop?: Drop<S>, own?: Own<S, T>) {
		super();
		this.source = source;
		this.step = step;
		this.drop = drop;
		this.own = own;
	}

	next(): T | Done {
		return this.step(this.source);
	}

	/** Only an iterator with its own `next_back` is read from the back. */
	override nextBack(): T | Done {
		return (this.own?.back as (source: S) => T | Done)(this.source);
	}

	/** Its own, which drops what it skips itself. */
	override nth(n: number, drop?: Drop<T>): T | Done {
		const nth = this.own?.nth;
		return nth === undefined ? super.nth(n, drop) : nth(this.source, n);
	}

	override nthBack(n: number, drop?: Drop<T>): T | Done {
		const nthBack = this.own?.nthBack;
		return nthBack === undefined ? super.nthBack(n, drop) : nthBack(this.source, n);
	}

	override sizeHint(): SizeHint {
		const hint = this.own?.hint;
		return hint === undefined ? super.sizeHint() : hint(this.source);
	}

	/** Its own, or std's, of its size hint. */
	override len(): number {
		const len = this.own?.len;
		return len === undefined ? hintLen(this) : len(this.source);
	}

	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const fold = this.own?.fold;
		return fold === undefined ? super.fold(acc, f, release) : this.folds(fold, acc, f, release);
	}

	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const rfold = this.own?.rfold;
		return rfold === undefined ? super.rfold(acc, f, release) : this.folds(rfold, acc, f, release);
	}

	/**
	 * Its own `fold` or `rfold`, which takes it by value: given `f` as a closure that drops
	 * what it owns by `release`, as the crate's drops the closure std gives it, and nothing
	 * without it, as a loan of `f` does.
	 */
	private folds<A>(fold: (source: S, acc: never, f: never) => unknown, acc: A, f: (acc: A, item: T) => A, release: (() => void) | undefined): A {
		this.drop = undefined;
		return (fold as (source: S, acc: A, f: (acc: A, item: T) => A) => A)(this.source, acc, lend(f, release));
	}

	override finds(): boolean {
		const own = this.own;
		return own?.find !== undefined || own?.rfind !== undefined || own?.findMap !== undefined;
	}

	/** Its own, which drops what `f` rejects itself, or std's. */
	override find(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		const find = this.own?.find as ((source: S, f: (item: T) => boolean) => T | Done) | undefined;
		return find === undefined ? super.find(f, drop) : find(this.source, lend(f, undefined));
	}

	override rfind(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		const rfind = this.own?.rfind as ((source: S, f: (item: T) => boolean) => T | Done) | undefined;
		return rfind === undefined ? super.rfind(f, drop) : rfind(this.source, lend(f, undefined));
	}

	/** Its own, or std's, given what `f` finds boxed, as it only moves it, so no value of it is its `None`. */
	override findMap<U>(f: (item: T) => U | undefined | None, none?: None): U | Done {
		const findMap = this.own?.findMap as ((source: S, f: (item: T) => [U] | undefined) => [U] | undefined) | undefined;
		if (findMap === undefined) return super.findMap(f, none);
		const found = findMap(this.source, (item) => {
			const found = f(item);
			return found === none ? undefined : [found as U];
		});
		return found === undefined ? DONE : found[0];
	}

	/** Its own `reduce` of `f`, which takes it by value, given `f` as `folds` gives it. */
	reduced(f: (a: T, b: T) => T, release: (() => void) | undefined): T | Done {
		this.drop = undefined;
		return (this.own!.reduce as (source: S, f: (a: T, b: T) => T) => T | Done)(this.source, lend(f, release));
	}

	/** Its own `all` of `f`, given `f` as `folds` gives it. */
	allOwn(f: (item: T) => boolean, release: (() => void) | undefined): boolean {
		return (this.own!.all as (source: S, f: (item: T) => boolean) => boolean)(this.source, lend(f, release));
	}

	/** Its own `count` or `last`, which takes it by value. */
	taken<R>(f: (source: S) => R): R {
		this.drop = undefined;
		return f(this.source);
	}

	/** Its source cloned by `h.v`, as the crate's own `Clone` of it. */
	override clone(h?: Clones): Iter<T> {
		return new FromNext(cloned(this.source, h?.v), this.step, this.drop, this.own);
	}

	[Symbol.dispose](): void {
		const drop = this.drop;
		this.drop = undefined;
		drop?.(this.source);
	}
}

/** `Successors`, holding the item it yields next, which drops once, as `Calling` does. */
class SuccessorsIter<T> extends Iter<T> implements Disposable {
	item: T | undefined | None;
	private readonly succ: (item: T) => T | undefined | None;
	private readonly drop: Drop<T> | undefined;
	/** The `Option`'s `None`. */
	readonly none: None | undefined;
	private dropped = false;

	constructor(first: T | undefined | None, succ: (item: T) => T | undefined | None, drop?: Drop<T>, none?: None) {
		super();
		this.item = first;
		this.succ = succ;
		this.drop = drop;
		this.none = none;
	}

	next(): T | Done {
		const item = this.item;
		if (item === this.none) return DONE;
		this.item = this.succ(item as T);
		return item as T;
	}

	override sizeHint(): SizeHint {
		return this.item === this.none ? [0, 0] : [1, undefined];
	}

	/** std's: the item it holds next by `h.v`, then its closure by `h.f`. */
	override clone(h?: Clones): Iter<T> {
		const item = this.item === this.none ? this.item : cloned(this.item as T, h?.v);
		return new SuccessorsIter(item, cloned(this.succ, h?.f), this.drop, this.none);
	}

	[Symbol.dispose](): void {
		if (this.dropped) return;
		this.dropped = true;
		const item = this.item;
		this.item = this.none;
		if (item !== this.none) this.drop?.(item as T);
		dispose(this.succ);
	}
}

/**
 * An iterator of `inner` that forwards what std's `Iterator` for `Box<I>` and a trait
 * object's vtable both have, and drops it as it drops.
 */
abstract class Forward<T> extends Iter<T> implements Disposable {
	readonly inner: Iter<T>;

	constructor(inner: Iter<T>) {
		super();
		this.inner = inner;
	}

	override sizeHint(): SizeHint {
		return this.inner.sizeHint();
	}

	next(): T | Done {
		return this.inner.next();
	}

	override nextBack(): T | Done {
		return this.inner.nextBack();
	}

	override len(): number {
		return this.inner.len();
	}

	override nth(n: number, drop?: Drop<T>): T | Done {
		return this.inner.nth(n, drop);
	}

	override nthBack(n: number, drop?: Drop<T>): T | Done {
		return this.inner.nthBack(n, drop);
	}

	[Symbol.dispose](): void {
		dispose(this.inner);
	}
}

/** A box, whose `last` is `inner`'s too, which `last` dispatches on. */
class Boxed<T> extends Forward<T> {}

/** What std's vtable of an iterator holds, as `inner` does it: a box's, and `advance_by` too. */
class Dyn<T> extends Forward<T> {
	override advanceBy(n: number, drop?: Drop<T>): number {
		return this.inner.advanceBy(n, drop);
	}

	override advanceBackBy(n: number, drop?: Drop<T>): number {
		return this.inner.advanceBackBy(n, drop);
	}
}


/**
 * An iterator borrowed by `&mut`, which forwards only what std's `impl Iterator for &mut
 * I` does, its folds included, as std's specializes them for a sized `I`, and which
 * dropping drops nothing.
 */
class Borrowed<T> extends Dyn<T> {
	override folded(): Iter<T> {
		return this.inner.folded();
	}

	override [Symbol.dispose](): void {}
}

class MapIter<T, U> extends Calling<T, U, (item: T) => U> {

	constructor(inner: Iter<T>, f: (item: T) => U) {
		super(inner, f);
	}

	override sizeHint(): SizeHint {
		return this.inner.sizeHint();
	}

	next(): U | Done {
		const item = this.inner.next();
		return item === DONE ? DONE : this.f(item);
	}

	override nextBack(): U | Done {
		const item = this.inner.nextBack();
		return item === DONE ? DONE : this.f(item);
	}

	override len(): number {
		return this.inner.len();
	}

	override folded(fold = false): Iter<U> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner) return this;
		return new MapIter(inner, this.f);
	}

	/** std's: `f` of its iterator's item, which may have side effects, so never of one it passes. */
	override indexed(): Iter<U> {
		const inner = this.inner.indexed();
		const f = this.f;
		return new Indexed(
			() => {
				const item = inner.next();
				return item === DONE ? DONE : f(item);
			},
			() => {
				const item = inner.nextBack();
				return item === DONE ? DONE : f(item);
			},
			(n) => inner.advanceBackBy(n),
		);
	}

	/** Its iterator by `h.i`, then `f` by `h.f`, as each kind that maps does. */
	override clone(h?: Clones): Iter<U> {
		const kind = this.constructor as new (inner: Iter<T>, f: (item: T) => U) => MapIter<T, U>;
		return new kind(this.inner.clone(h?.i), cloned(this.f, h?.f));
	}

	/** std's: `inner`'s, by a closure that drops `release`'s and then `f`. */
	override fold<A>(acc: A, g: (acc: A, item: U) => A, release?: () => void): A {
		return this.inner.fold(acc, mapFold(this.f, g), then(release, this.take()));
	}

	override rfold<A>(acc: A, g: (acc: A, item: U) => A, release?: () => void): A {
		return this.inner.rfold(acc, mapFold(this.f, g), then(release, this.take()));
	}
}

/** What `iterLend` makes, which has left the items `inner` has, as it holds them. */
class LendIter<T, U> extends MapIter<T, U> {
	override remaining(): Iterable<unknown> {
		return this.inner.remaining();
	}
}

/** std's `map_fold`: `g` of what `f` makes of each item. */
function mapFold<T, U, A>(f: (item: T) => U, g: (acc: A, item: U) => A): (acc: A, item: T) => A {
	return (acc, item) => g(acc, f(item));
}

/** `Copied` of items JS copies by a function: a `map` whose `nth` and `advance_by` are its iterator's. */
class CopiedIter<R, T> extends MapIter<R, T> {
	override advanceBy(n: number): number {
		return this.inner.advanceBy(n);
	}

	override advanceBackBy(n: number): number {
		return this.inner.advanceBackBy(n);
	}

	override nth(n: number): T | Done {
		const item = this.inner.nth(n);
		return item === DONE ? DONE : this.f(item);
	}
}

class FilterIter<T> extends Calling<T, T, (item: T) => boolean> {
	private readonly drop: Drop<T> | undefined;

	constructor(inner: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>) {
		super(inner, f);
		this.drop = drop;
	}

	/** std's: its iterator's `find`, and `rfind` from the back. */
	next(): T | Done {
		return this.inner.find(this.f, this.drop);
	}

	override sizeHint(): SizeHint {
		return fewer(this.inner);
	}

	override nextBack(): T | Done {
		return this.inner.rfind(this.f, this.drop);
	}

	/**
	 * std's `try_fold` and `fold` read its iterator by its own, as `folded` steps it, where
	 * `next` reads it by `find`, which the crate's own iterator may have of its own.
	 */
	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner && !inner.finds()) return this;
		const { f, drop } = this;
		return new Folded(
			() => findItem(inner, f, drop),
			() => rfindItem(inner, f, drop),
		);
	}

	override clone(h?: Clones): Iter<T> {
		return new FilterIter(this.inner.clone(h?.i), cloned(this.f, h?.f), this.drop);
	}

	/** std's: `inner`'s, by a closure that drops `f` and then `release`'s. */
	override fold<A>(acc: A, g: (acc: A, item: T) => A, release?: () => void): A {
		return this.inner.fold(acc, this.filterFold(g), then(this.take(), release));
	}

	override rfold<A>(acc: A, g: (acc: A, item: T) => A, release?: () => void): A {
		return this.inner.rfold(acc, this.filterFold(g), then(this.take(), release));
	}

	/** std's `filter_fold`: `g` of each item `f` keeps. */
	private filterFold<A>(g: (acc: A, item: T) => A): (acc: A, item: T) => A {
		const { f, drop } = this;
		return (acc, item) => {
			if (f(item)) return g(acc, item);
			drop?.(item);
			return acc;
		};
	}
}

/** The next value `f`, whose `None` is `none`, makes of an item of `it` by `next`, or `nextBack` if `back`. */
function filterMapped<T, U>(it: Iter<T>, back: boolean, f: (item: T) => U | undefined | None, none: None | undefined): U | Done {
	for (let item = back ? it.nextBack() : it.next(); item !== DONE; item = back ? it.nextBack() : it.next()) {
		const mapped = f(item);
		if (mapped !== none) return mapped as U;
	}
	return DONE;
}

class FilterMapIter<T, U> extends Calling<T, U, (item: T) => U | undefined | None> {
	/** The `Option`'s `None` that `f` returns. */
	private readonly none: None | undefined;

	constructor(inner: Iter<T>, f: (item: T) => U | undefined | None, none?: None) {
		super(inner, f);
		this.none = none;
	}

	/** std's: its iterator's `find_map`. */
	next(): U | Done {
		return this.inner.findMap(this.f, this.none);
	}

	override sizeHint(): SizeHint {
		return fewer(this.inner);
	}

	/** std's, by its iterator's `try_rfold`. */
	override nextBack(): U | Done {
		return filterMapped(this.inner.folded(), true, this.f, this.none);
	}

	/**
	 * std's `try_fold` and `fold` read its iterator by its own, as `folded` steps it, where
	 * `next` reads it by `find_map`, which the crate's own iterator may have of its own.
	 */
	override folded(fold = false): Iter<U> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner && !inner.finds()) return this;
		const { f, none } = this;
		return new Folded(
			() => filterMapped(inner, false, f, none),
			() => filterMapped(inner, true, f, none),
		);
	}

	override clone(h?: Clones): Iter<U> {
		return new FilterMapIter(this.inner.clone(h?.i), cloned(this.f, h?.f), this.none);
	}

	/** std's: `inner`'s, by a closure that drops `f` and then `release`'s. */
	override fold<A>(acc: A, g: (acc: A, item: U) => A, release?: () => void): A {
		return this.inner.fold(acc, this.filterMapFold(g), then(this.take(), release));
	}

	override rfold<A>(acc: A, g: (acc: A, item: U) => A, release?: () => void): A {
		return this.inner.rfold(acc, this.filterMapFold(g), then(this.take(), release));
	}

	/** std's `filter_map_fold`: `g` of each value `f` makes. */
	private filterMapFold<A>(g: (acc: A, item: U) => A): (acc: A, item: T) => A {
		const [f, none] = [this.f, this.none];
		return mapFold(f, (acc, mapped: U | undefined | None) => (mapped === none ? acc : g(acc, mapped as U)));
	}
}

class EnumerateIter<T> extends Adaptor<T, [number, T]> {
	index = 0;

	override clone(h?: Clones): Iter<[number, T]> {
		const it = new EnumerateIter(this.inner.clone(h?.i));
		it.index = this.index;
		return it;
	}

	override sizeHint(): SizeHint {
		return this.inner.sizeHint();
	}

	next(): [number, T] | Done {
		const item = this.inner.next();
		return item === DONE ? DONE : [this.index++, item];
	}

	override nextBack(): [number, T] | Done {
		const item = this.inner.nextBack();
		return item === DONE ? DONE : [this.index + this.inner.len(), item];
	}

	override len(): number {
		return this.inner.len();
	}

	/** std's, which counts what its iterator skipped. */
	override advanceBy(n: number, drop?: Drop<[number, T]>): number {
		const left = this.inner.advanceBy(n, items(drop));
		this.index += n - left;
		return left;
	}

	override advanceBackBy(n: number, drop?: Drop<[number, T]>): number {
		return this.inner.advanceBackBy(n, items(drop));
	}

	/** std's, which counts only when it finds an item. */
	override nth(n: number, drop?: Drop<[number, T]>): [number, T] | Done {
		const item = this.inner.nth(n, items(drop));
		if (item === DONE) return DONE;
		const i = this.index + n;
		this.index = i + 1;
		return [i, item];
	}

	override nthBack(n: number, drop?: Drop<[number, T]>): [number, T] | Done {
		const item = this.inner.nthBack(n, items(drop));
		return item === DONE ? DONE : [this.index + this.inner.len(), item];
	}

	/**
	 * std's: its iterator folded, counting each item as it goes, and from the back down from
	 * its length as the fold starts.
	 */
	override folded(fold = false): Iter<[number, T]> {
		const inner = this.inner.folded(fold);
		const front = (): [number, T] | Done => {
			const item = inner.next();
			return item === DONE ? DONE : [this.index++, item];
		};
		let i: number | undefined;
		const back = (): [number, T] | Done => {
			i ??= this.index + this.inner.len();
			const item = inner.nextBack();
			return item === DONE ? DONE : [--i, item];
		};
		return new Folded(front, back);
	}

	/** std's: its iterator's, counted from where it is. */
	override indexed(): Iter<[number, T]> {
		const inner = this.inner.indexed();
		if (inner === this.inner) return this;
		return new Indexed(
			() => {
				const item = inner.next();
				return item === DONE ? DONE : [this.index++, item];
			},
			() => {
				const item = inner.nextBack();
				return item === DONE ? DONE : [this.index + this.inner.len(), item];
			},
			(n) => inner.advanceBackBy(n),
		);
	}

	override fold<A>(acc: A, g: (acc: A, item: [number, T]) => A, release?: () => void): A {
		let i = this.index;
		return this.inner.fold(acc, (acc, item) => g(acc, [i++, item]), release);
	}

	override rfold<A>(acc: A, g: (acc: A, item: [number, T]) => A, release?: () => void): A {
		let i = this.index + this.inner.len();
		return this.inner.rfold(acc, (acc, item) => g(acc, [--i, item]), release);
	}
}

/**
 * `Chain`, which drops its first iterator once it ends, but not its second, as std's
 * stops reading it.
 */
class ChainIter<T> extends Iter<T> implements Disposable {
	a: Iter<T> | undefined;
	b: Iter<T> | undefined;

	constructor(a: Iter<T> | undefined, b: Iter<T> | undefined) {
		super();
		this.a = a;
		this.b = b;
	}

	/** std's: each half it still holds, the first by `h.i` and the second by `h.o`. */
	override clone(h?: Clones): Iter<T> {
		return new ChainIter(this.a?.clone(h?.i), this.b?.clone(h?.o));
	}

	/** Drop `a`, which ended. */
	endA(): void {
		const a = this.a;
		this.a = undefined;
		dispose(a);
	}

	/** Drop `b`, which ended from the back. */
	endB(): void {
		const b = this.b;
		this.b = undefined;
		dispose(b);
	}

	override sizeHint(): SizeHint {
		const { a, b } = this;
		if (a === undefined || b === undefined) return a?.sizeHint() ?? b?.sizeHint() ?? [0, 0];
		const [aLower, aUpper] = a.sizeHint();
		const [bLower, bUpper] = b.sizeHint();
		return [satAdd(aLower, bLower), addUpper(aUpper, bUpper)];
	}

	next(): T | Done {
		if (this.a !== undefined) {
			const item = this.a.next();
			if (item !== DONE) return item;
			this.endA();
		}
		return this.b === undefined ? DONE : this.b.next();
	}

	override nextBack(): T | Done {
		if (this.b !== undefined) {
			const item = this.b.nextBack();
			if (item !== DONE) return item;
			this.endB();
		}
		return this.a === undefined ? DONE : this.a.nextBack();
	}

	[Symbol.dispose](): void {
		this.endA();
		this.endB();
	}

	/** std's: each folded in turn, the first dropped once it ends. */
	override folded(fold = false): Iter<T> {
		const a = this.a?.folded(fold);
		const b = this.b?.folded(fold);
		if (a === this.a && b === this.b) return this;
		const front = (): T | Done => {
			if (this.a !== undefined) {
				const item = a!.next();
				if (item !== DONE) return item;
				this.endA();
			}
			return this.b === undefined ? DONE : b!.next();
		};
		const back = (): T | Done => {
			if (this.b !== undefined) {
				const item = b!.nextBack();
				if (item !== DONE) return item;
				this.endB();
			}
			return this.a === undefined ? DONE : a!.nextBack();
		};
		return new Folded(front, back);
	}

	override advanceBy(n: number, drop?: Drop<T>): number {
		if (this.a !== undefined) {
			n = this.a.advanceBy(n, drop);
			if (n === 0) return 0;
			this.endA();
		}
		return this.b === undefined ? n : this.b.advanceBy(n, drop);
	}

	override advanceBackBy(n: number, drop?: Drop<T>): number {
		if (this.b !== undefined) {
			n = this.b.advanceBackBy(n, drop);
			if (n === 0) return 0;
			this.endB();
		}
		return this.a === undefined ? n : this.a.advanceBackBy(n, drop);
	}

	override nth(n: number, drop?: Drop<T>): T | Done {
		if (this.a !== undefined) {
			n = this.a.advanceBy(n, drop);
			if (n === 0) {
				const item = this.a.next();
				if (item !== DONE) return item;
			}
			this.endA();
		}
		return this.b === undefined ? DONE : this.b.nth(n, drop);
	}

	override nthBack(n: number, drop?: Drop<T>): T | Done {
		if (this.b !== undefined) {
			n = this.b.advanceBackBy(n, drop);
			if (n === 0) {
				const item = this.b.nextBack();
				if (item !== DONE) return item;
			}
			this.endB();
		}
		return this.a === undefined ? DONE : this.a.nthBack(n, drop);
	}

	override finds(): boolean {
		return this.a?.finds() === true || this.b?.finds() === true;
	}

	/** std's: the first's `find`, which drops it if it finds none, and then the second's. */
	override find(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		if (this.a !== undefined) {
			const item = this.a.find(f, drop);
			if (item !== DONE) return item;
			this.endA();
		}
		return this.b === undefined ? DONE : this.b.find(f, drop);
	}

	/** std's: the second's `rfind`, which drops it if it finds none, and then the first's. */
	override rfind(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		if (this.b !== undefined) {
			const item = this.b.rfind(f, drop);
			if (item !== DONE) return item;
			this.endB();
		}
		return this.a === undefined ? DONE : this.a.rfind(f, drop);
	}

	/** std's: the first's fold and then the second's, which drops `release`'s as it ends. */
	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const { a, b } = this;
		this.a = this.b = undefined;
		if (a !== undefined) acc = a.fold(acc, f);
		if (b !== undefined) return b.fold(acc, f, release);
		release?.();
		return acc;
	}

	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const { a, b } = this;
		this.a = this.b = undefined;
		if (b !== undefined) acc = b.rfold(acc, f);
		if (a !== undefined) return a.rfold(acc, f, release);
		release?.();
		return acc;
	}
}

class ZipIter<A, B> extends Iter<[A, B]> implements Disposable {
	readonly a: Iter<A>;
	readonly b: Iter<B>;
	/** How `fold` reads it where it stops at its size hint: by `next` or by index. */
	private readonly bounded: "len" | "index" | undefined;
	protected readonly dropA: Drop<A> | undefined;
	protected readonly dropB: Drop<B> | undefined;

	constructor(a: Iter<A>, b: Iter<B>, bounded?: "len" | "index", dropA?: Drop<A>, dropB?: Drop<B>) {
		super();
		this.a = a;
		this.b = b;
		this.bounded = bounded;
		this.dropA = dropA;
		this.dropB = dropB;
	}

	override clone(h?: Clones): Iter<[A, B]> {
		return new ZipIter(this.a.clone(h?.i), this.b.clone(h?.o), this.bounded, this.dropA, this.dropB);
	}

	/**
	 * std's `SpecFold` of two `TrustedLen` iterators, or the `fold` of two
	 * `TrustedRandomAccessNoCoerce` ones, which `try_fold` doesn't share.
	 */
	override folded(fold = false): Iter<[A, B]> {
		if (!fold || this.bounded === undefined) return this;
		return this.bounded === "len" ? new ZipFold(this, this.a, this.b) : new ZipFold(this, this.a.indexed(), this.b.indexed());
	}

	/**
	 * std's of two `TrustedRandomAccessNoCoerce` iterators: each side's item at its index.
	 * Only a zip of two `TrustedRandomAccess` ones reads by index from the back, of which
	 * neither side is this.
	 */
	override indexed(): Iter<[A, B]> {
		if (this.bounded !== "index") return this;
		const [a, b] = [this.a.indexed(), this.b.indexed()];
		if (a === this.a && b === this.b) return this;
		return new Folded(
			() => [a.next() as A, b.next() as B],
			() => this.nextBack(),
		);
	}

	/** An item of `a` it read, dropped if `b` ends. */
	next(): [A, B] | Done {
		const a = this.a.next();
		if (a === DONE) return DONE;
		const b = this.b.next();
		if (b !== DONE) return [a, b];
		this.dropA?.(a);
		return DONE;
	}

	/** std's: the longer side drops its extra items from the back first. */
	override nextBack(): [A, B] | Done {
		const aLen = this.a.len();
		const bLen = this.b.len();
		for (let i = bLen; i < aLen; i++) {
			const item = this.a.nextBack();
			if (item !== DONE) this.dropA?.(item);
		}
		for (let i = aLen; i < bLen; i++) {
			const item = this.b.nextBack();
			if (item !== DONE) this.dropB?.(item);
		}
		const a = this.a.nextBack();
		const b = this.b.nextBack();
		if (a === DONE && b === DONE) return DONE;
		if (a === DONE || b === DONE) panic("internal error: entered unreachable code", "zip_next_back");
		return [a, b];
	}

	/** std's, of its size hint. */
	override len(): number {
		return hintLen(this);
	}

	/** std's: the shorter side's, where either knows its upper bound. */
	override sizeHint(): SizeHint {
		const [aLower, aUpper] = this.a.sizeHint();
		const [bLower, bUpper] = this.b.sizeHint();
		const upper = aUpper === undefined || bUpper === undefined ? (aUpper ?? bUpper) : Math.min(aUpper, bUpper);
		return [Math.min(aLower, bLower), upper];
	}

	[Symbol.dispose](): void {
		dispose(this.a);
		dispose(this.b);
	}
}

/**
 * A zip as std's `fold` reads it where it stops at its size hint: as many pairs as its
 * upper bound when it starts, which `TrustedLen` promises each holds, read from `a` and
 * `b`, its sides or their views by index, and from the back by `next_back`, as `rfold`
 * reads it.
 */
class ZipFold<A, B> extends Iter<[A, B]> {
	private readonly zip: ZipIter<A, B>;
	private readonly a: Iter<A>;
	private readonly b: Iter<B>;
	/** The pairs left, once it starts, or -1. */
	private left = -1;

	constructor(zip: ZipIter<A, B>, a: Iter<A>, b: Iter<B>) {
		super();
		this.zip = zip;
		this.a = a;
		this.b = b;
	}

	next(): [A, B] | Done {
		if (this.left < 0) this.left = this.zip.sizeHint()[1] ?? Infinity;
		if (this.left === 0) return DONE;
		this.left--;
		const a = this.a.next();
		const b = this.b.next();
		// Only std's iterators are `TrustedLen`, which never run out early.
		if (a === DONE || b === DONE) violated("hint::unreachable_unchecked must never be reached");
		return [a, b];
	}

	override nextBack(): [A, B] | Done {
		return this.zip.nextBack();
	}
}

/**
 * std's `Zip` of two `TrustedRandomAccess` iterators, which counts the pairs it reads, and
 * reads them by index, as `fold` does. It reads an item it never yields only from a side
 * that may have side effects, and skips the rest of a side that has none in one step.
 */
class TrustedZipIter<A, B> extends ZipIter<A, B> {
	private index = 0;
	private end: number;
	/** Whether both sides hold just the pairs left, as a read from the back trims them to and reading pairs keeps. */
	private even = false;
	/** Whether each side may have side effects. */
	private readonly effects: readonly [boolean, boolean];
	/** Each side as it reads it by index. */
	private readonly ia: Iter<A>;
	private readonly ib: Iter<B>;

	constructor(a: Iter<A>, b: Iter<B>, effects: readonly [boolean, boolean], dropA?: Drop<A>, dropB?: Drop<B>) {
		super(a, b, undefined, dropA, dropB);
		this.end = Math.min(a.len(), b.len());
		this.effects = effects;
		this.ia = a.indexed();
		this.ib = b.indexed();
	}

	/** std's: its count of pairs read, with its iterators, which it reads by index as they are. */
	override clone(h?: Clones): Iter<[A, B]> {
		const it = new TrustedZipIter(this.a.clone(h?.i), this.b.clone(h?.o), this.effects, this.dropA, this.dropB);
		it.index = this.index;
		it.end = this.end;
		it.even = this.even;
		return it;
	}

	override next(): [A, B] | Done {
		if (this.index >= this.end) return DONE;
		this.index++;
		return [this.ia.next() as A, this.ib.next() as B];
	}

	/**
	 * std's: each pair it skips made and dropped one side at a time, of only the sides that
	 * may have side effects, then `next`.
	 */
	override nth(n: number): [A, B] | Done {
		const skip = Math.min(n, this.end - this.index);
		const [effectA, effectB] = this.effects;
		if (!effectA) this.a.advanceBy(skip, this.dropA);
		if (!effectB) this.b.advanceBy(skip, this.dropB);
		if (!effectA && !effectB) this.index += skip;
		else {
			const { ia: a, ib: b } = this;
			for (let i = 0; i < skip; i++) {
				this.index++;
				if (effectA) {
					const item = a.next() as A;
					this.dropA?.(item);
				}
				if (effectB) {
					const item = b.next() as B;
					this.dropB?.(item);
				}
			}
		}
		return this.next();
	}

	/**
	 * std's: the longer side trimmed to the pairs left, as only the first pair it reads
	 * from the back needs, reading each item past them back to front from a side that may
	 * have side effects, and none from one that has none.
	 */
	override nextBack(): [A, B] | Done {
		if (this.index >= this.end) return DONE;
		if (!this.even) {
			const left = this.end - this.index;
			trim(this.a, this.a.len() - left, this.effects[0], this.dropA);
			trim(this.b, this.b.len() - left, this.effects[1], this.dropB);
			this.even = true;
		}
		this.end--;
		return [this.ia.nextBack() as A, this.ib.nextBack() as B];
	}

	/** std's: each side's item at its index, and none past the shorter side, from either end. */
	override indexed(): Iter<[A, B]> {
		const { ia: a, ib: b } = this;
		if (a === this.a && b === this.b) return this;
		const back = (): [A, B] | Done => {
			if (this.index >= this.end) return DONE;
			if (!this.even) {
				const left = this.end - this.index;
				a.advanceBackBy(this.a.len() - left);
				b.advanceBackBy(this.b.len() - left);
				this.even = true;
			}
			this.end--;
			return [a.nextBack() as A, b.nextBack() as B];
		};
		const skip = (n: number) => {
			this.end -= n;
			this.even = false;
		};
		return new Indexed(() => this.next(), back, skip);
	}

	/** `advance_by`, by `next`, in one step where neither side has side effects. */
	override advanceBy(n: number, drop?: Drop<[A, B]>): number {
		if (this.effects[0] || this.effects[1]) return super.advanceBy(n, drop);
		const skip = Math.min(n, this.len());
		this.index += skip;
		this.a.advanceBy(skip, this.dropA);
		this.b.advanceBy(skip, this.dropB);
		return n - skip;
	}

	/** `advance_back_by`, by `nextBack`, in one step where neither side has side effects. */
	override advanceBackBy(n: number, drop?: Drop<[A, B]>): number {
		if (this.effects[0] || this.effects[1]) return super.advanceBackBy(n, drop);
		const skip = Math.min(n, this.len());
		const left = this.len() - skip;
		trim(this.a, this.a.len() - left, false, this.dropA);
		trim(this.b, this.b.len() - left, false, this.dropB);
		this.end -= skip;
		this.even = true;
		return n - skip;
	}

	override len(): number {
		return this.end - this.index;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}
}

/** Drop `n` items from the back of `it`, read one at a time if it may have side effects. */
function trim<T>(it: Iter<T>, n: number, effect: boolean, drop?: Drop<T>): void {
	if (!effect) {
		it.advanceBackBy(n, drop);
		return;
	}
	for (let i = 0; i < n; i++) {
		const item = it.nextBack();
		if (item !== DONE) drop?.(item);
	}
}

class RevIter<T> extends Adaptor<T, T> {
	override sizeHint(): SizeHint {
		return this.inner.sizeHint();
	}

	next(): T | Done {
		return this.inner.nextBack();
	}

	override nextBack(): T | Done {
		return this.inner.next();
	}

	override len(): number {
		return this.inner.len();
	}

	override advanceBy(n: number, drop?: Drop<T>): number {
		return this.inner.advanceBackBy(n, drop);
	}

	override advanceBackBy(n: number, drop?: Drop<T>): number {
		return this.inner.advanceBy(n, drop);
	}

	override nth(n: number, drop?: Drop<T>): T | Done {
		return this.inner.nthBack(n, drop);
	}

	override nthBack(n: number, drop?: Drop<T>): T | Done {
		return this.inner.nth(n, drop);
	}

	override finds(): boolean {
		return this.inner.finds();
	}

	override find(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		return this.inner.rfind(f, drop);
	}

	override rfind(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		return this.inner.find(f, drop);
	}

	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner) return this;
		return new RevIter(inner);
	}

	override clone(h?: Clones): Iter<T> {
		return new RevIter(this.inner.clone(h?.i));
	}

	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		return this.inner.rfold(acc, f, release);
	}

	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		return this.inner.fold(acc, f, release);
	}
}

/** `Skip`, whose items drop by `drop` as its iterator's own `nth` skips them. */
class SkipIter<T> extends Adaptor<T, T> {
	n: number;
	private readonly drop: Drop<T> | undefined;

	constructor(inner: Iter<T>, n: number, drop?: Drop<T>) {
		super(inner);
		this.n = n;
		this.drop = drop;
	}

	override clone(h?: Clones): Iter<T> {
		return new SkipIter(this.inner.clone(h?.i), this.n, this.drop);
	}

	override sizeHint(): SizeHint {
		const [lower, upper] = this.inner.sizeHint();
		return [Math.max(lower - this.n, 0), upper === undefined ? undefined : Math.max(upper - this.n, 0)];
	}

	next(): T | Done {
		if (this.n === 0) return this.inner.next();
		const n = this.n;
		this.n = 0;
		return this.inner.nth(n, this.drop);
	}

	override nextBack(): T | Done {
		return this.len() > 0 ? this.inner.nextBack() : DONE;
	}

	/** std's, of its size hint. */
	override len(): number {
		return hintLen(this);
	}

	/** std's: what it skips and `n` more in one step, and the rest of `n` if that saturated a `usize`. */
	override advanceBy(n: number, drop?: Drop<T>): number {
		const skip = this.n;
		const both = Math.min(skip + n, USIZE_MAX);
		const left = this.inner.advanceBy(both, drop);
		const advanced = both - left;
		n -= Math.max(advanced - skip, 0);
		this.n = Math.max(skip - advanced, 0);
		if (left === 0 && n > 0) n = this.inner.advanceBy(n, drop);
		return n;
	}

	/** std's: its iterator advanced from the back in one step, short of what it skips. */
	override advanceBackBy(n: number, drop?: Drop<T>): number {
		const min = Math.min(this.len(), n);
		this.inner.advanceBackBy(min, drop);
		return n - min;
	}

	/**
	 * std's: what it still skips and then `n` more, in one step, or past a `usize`'s
	 * greatest, what it skips in one and `n` in another.
	 */
	override nth(n: number, drop?: Drop<T>): T | Done {
		const skip = this.n;
		this.n = 0;
		if (skip + n <= USIZE_MAX) return this.inner.nth(skip + n, drop);
		const item = this.inner.nth(skip - 1, drop);
		if (item === DONE) return DONE;
		drop?.(item);
		return this.inner.nth(n, drop);
	}

	/**
	 * std's: past what it skips by `nth`, one short of where `next` reads, as a zip's `nth`
	 * reads its last pair whole, then its iterator folded, and from the back, its iterator
	 * folded down to what it skips, by its length as the fold starts.
	 */
	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded(fold);
		const front = (): T | Done => {
			if (this.n > 0) {
				const n = this.n;
				this.n = 0;
				const item = this.inner.nth(n - 1, this.drop);
				if (item === DONE) return DONE;
				this.drop?.(item);
			}
			return inner.next();
		};
		let left: number | undefined;
		const back = (): T | Done => {
			left ??= this.len();
			if (left === 0) return DONE;
			left--;
			return inner.nextBack();
		};
		return new Folded(front, back);
	}

	/**
	 * std's: as it reads index 0, from either end, each item it skips made by index and
	 * dropped whole, where its iterator may have side effects.
	 */
	override indexed(): Iter<T> {
		const inner = this.inner.indexed();
		if (inner === this.inner) return this;
		const first = (): T | Done => {
			for (; this.n > 0; this.n--) {
				const item = inner.next();
				if (item !== DONE) this.drop?.(item);
			}
			return inner.next();
		};
		const back = (): T | Done => {
			const len = this.len();
			if (len === 0) return DONE;
			return len === 1 ? first() : inner.nextBack();
		};
		return new Indexed(first, back, (n) => inner.advanceBackBy(Math.min(n, this.len())));
	}

	/** std's: past what it skips by `nth`, and then its iterator's own. */
	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		if (this.n > 0) {
			const item = this.inner.nth(this.n - 1, this.drop);
			if (item === DONE) {
				ended(this.inner, release);
				return acc;
			}
			this.drop?.(item);
		}
		return this.inner.fold(acc, f, release);
	}

	/** std's: past what it doesn't skip, its iterator read from the back through what it does. */
	override nthBack(n: number, drop?: Drop<T>): T | Done {
		const len = this.len();
		if (n < len) return this.inner.nthBack(n, drop);
		if (len > 0) {
			const item = this.inner.nthBack(len - 1, drop);
			if (item !== DONE) drop?.(item);
		}
		return DONE;
	}
}

/** `Take`, of which what its iterator holds past the limit drops by `drop` as `next_back` reads through it. */
class TakeIter<T> extends Adaptor<T, T> {
	n: number;
	private readonly drop: Drop<T> | undefined;
	/** Whether its iterator is `TrustedRandomAccess`, which its `fold` reads by index. */
	private readonly indexes: boolean;

	constructor(inner: Iter<T>, n: number, drop?: Drop<T>, indexes = false) {
		super(inner);
		this.n = n;
		this.drop = drop;
		this.indexes = indexes;
	}

	override clone(h?: Clones): Iter<T> {
		const kind = this.constructor as new (inner: Iter<T>, n: number, drop?: Drop<T>, indexes?: boolean) => TakeIter<T>;
		return new kind(this.inner.clone(h?.i), this.n, this.drop, this.indexes);
	}

	override sizeHint(): SizeHint {
		if (this.n === 0) return [0, 0];
		const [lower, upper] = this.inner.sizeHint();
		return [Math.min(lower, this.n), Math.min(upper ?? this.n, this.n)];
	}

	next(): T | Done {
		if (this.n === 0) return DONE;
		this.n--;
		return this.inner.next();
	}

	/** std's: what its iterator holds past the limit is read through from the back. */
	override nextBack(): T | Done {
		if (this.n === 0) return DONE;
		const n = this.n--;
		return this.inner.nthBack(Math.max(this.inner.len() - n, 0), this.drop);
	}

	/** std's, of its size hint. */
	override len(): number {
		return hintLen(this);
	}

	override advanceBy(n: number, drop?: Drop<T>): number {
		const min = Math.min(this.n, n);
		const advanced = min - this.inner.advanceBy(min, drop);
		this.n -= advanced;
		return n - advanced;
	}

	/**
	 * std's: its iterator folded, counting each item it reads against the limit, and from
	 * the back, past what it holds beyond the limit by `nth_back` as the fold starts,
	 * folded without counting, where `next_back` reads past it by one `nth_back` of its own.
	 * Its `fold` reads a `TrustedRandomAccess` one by index from the front, no further than
	 * its size, which a `skip` past its end makes 0.
	 */
	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded();
		if (!(fold && this.indexes)) return this.foldedBy(inner, inner);
		this.n = Math.min(this.n, this.inner.len());
		return this.foldedBy(inner, this.inner.indexed());
	}

	/** Read from the front by `front`, and from the back by `inner`, its iterator folded. */
	private foldedBy(inner: Iter<T>, front: Iter<T>): Folded<T> {
		const next = (): T | Done => {
			if (this.n === 0) return DONE;
			const item = front.next();
			if (item !== DONE) this.n--;
			return item;
		};
		let ended: boolean | undefined;
		const back = (): T | Done => {
			if (ended === undefined) {
				ended = this.n === 0;
				const len = ended ? 0 : this.inner.len();
				if (len > this.n) {
					const item = this.inner.nthBack(len - this.n - 1, this.drop);
					if (item === DONE) ended = true;
					else this.drop?.(item);
				}
			}
			return ended ? DONE : inner.nextBack();
		};
		return new Folded(next, back);
	}

	/** std's: its iterator advanced from the back past the limit and `n` more, in one step. */
	override advanceBackBy(n: number, drop?: Drop<T>): number {
		const trim = Math.max(this.inner.len() - this.n, 0);
		const advanced = trim + n - this.inner.advanceBackBy(trim + n, drop) - trim;
		this.n -= advanced;
		return n - advanced;
	}

	/** std's: past the limit, it reads the rest of the limit through, drops it, and ends. */
	override nth(n: number, drop?: Drop<T>): T | Done {
		if (this.n > n) {
			this.n -= n + 1;
			return this.inner.nth(n, drop);
		}
		if (this.n > 0) {
			const item = this.inner.nth(this.n - 1, drop);
			if (item !== DONE) drop?.(item);
			this.n = 0;
		}
		return DONE;
	}

	/** std's: past what its iterator holds beyond the limit by `nth_back`, and then its iterator's own. */
	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const len = this.n === 0 ? 0 : this.inner.len();
		if (this.n === 0 || len > this.n) {
			const item = this.n > 0 ? this.inner.nthBack(len - this.n - 1, this.drop) : DONE;
			if (item === DONE) {
				ended(this.inner, release);
				return acc;
			}
			this.drop?.(item);
		}
		return this.inner.rfold(acc, f, release);
	}

	/** std's: past the limit, it reads the whole rest from the back, even at a limit of 0. */
	override nthBack(n: number, drop?: Drop<T>): T | Done {
		const len = this.inner.len();
		if (this.n > n) {
			const m = Math.max(len - this.n, 0) + n;
			this.n -= n + 1;
			return this.inner.nthBack(m, drop);
		}
		if (len > 0) {
			const item = this.inner.nthBack(len - 1, drop);
			if (item !== DONE) drop?.(item);
		}
		return DONE;
	}
}

/** std's `Take` of a `Repeat`, which reads from the back as it does from the front. */
class TakeRepeat<T> extends TakeIter<T> {
	override nextBack(): T | Done {
		return this.next();
	}

	override nthBack(n: number, drop?: Drop<T>): T | Done {
		return this.nth(n, drop);
	}

	override advanceBackBy(n: number, drop?: Drop<T>): number {
		return this.advanceBy(n, drop);
	}

	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		return this.fold(acc, f, release);
	}

	override folded(fold = false): Iter<T> {
		const front = super.folded(fold);
		return new Folded(
			() => front.next(),
			() => front.next(),
		);
	}
}

class SkipWhileIter<T> extends Calling<T, T, (item: T) => boolean> {
	private readonly drop: Drop<T> | undefined;
	flag = false;

	constructor(inner: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>) {
		super(inner, f);
		this.drop = drop;
	}

	override clone(h?: Clones): Iter<T> {
		const it = new SkipWhileIter(this.inner.clone(h?.i), cloned(this.f, h?.f), this.drop);
		it.flag = this.flag;
		return it;
	}

	/** std's: its iterator's `find`. */
	next(): T | Done {
		return this.inner.find(
			(item) => {
				if (this.flag || !this.f(item)) {
					this.flag = true;
					return true;
				}
				return false;
			},
			this.drop,
		);
	}

	override sizeHint(): SizeHint {
		return fewer(this.inner);
	}

	/**
	 * std's `try_fold` and `fold`: its first item by `next`, and then its iterator folded,
	 * where `next` reads it by `find`, which the crate's own iterator may have of its own.
	 */
	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner && !inner.finds()) return this;
		return new FromNext(inner, (inner) => (this.flag ? inner.next() : this.next()));
	}

	/** std's: its first item by `next`, and then its iterator's fold, after which `f` drops. */
	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		if (!this.flag) {
			let done: boolean;
			[done, acc] = foldFirst(this, () => this.next(), acc, f, release);
			if (done) return acc;
		}
		const pred = this.take();
		acc = this.inner.fold(acc, f, release);
		pred();
		return acc;
	}
}

class TakeWhileIter<T> extends Calling<T, T, (item: T) => boolean> {
	private readonly drop: Drop<T> | undefined;
	flag = false;

	constructor(inner: Iter<T>, f: (item: T) => boolean, drop?: Drop<T>) {
		super(inner, f);
		this.drop = drop;
	}

	override clone(h?: Clones): Iter<T> {
		const it = new TakeWhileIter(this.inner.clone(h?.i), cloned(this.f, h?.f), this.drop);
		it.flag = this.flag;
		return it;
	}

	next(): T | Done {
		return this.from(this.inner);
	}

	override folded(): Iter<T> {
		const inner = this.inner.folded();
		if (inner === this.inner) return this;
		return new FromNext(inner, (inner) => this.from(inner));
	}

	/** The next item of `inner`, its iterator or that folded, while `f` holds of each. */
	private from(inner: Iter<T>): T | Done {
		if (this.flag) return DONE;
		const item = inner.next();
		if (item === DONE) return DONE;
		if (this.f(item)) return item;
		this.flag = true;
		this.drop?.(item);
		return DONE;
	}

	override sizeHint(): SizeHint {
		return this.flag ? [0, 0] : fewer(this.inner);
	}
}

/** `MapWhile`, which reads on after `f` returns `None` if called again, as std's does. */
class MapWhileIter<T, U> extends Calling<T, U, (item: T) => U | undefined | None> {
	/** The `Option`'s `None` that `f` returns. */
	private readonly none: None | undefined;

	constructor(inner: Iter<T>, f: (item: T) => U | undefined | None, none?: None) {
		super(inner, f);
		this.none = none;
	}

	next(): U | Done {
		const item = this.inner.next();
		return item === DONE ? DONE : iterFromOption(this.f(item), this.none);
	}

	override folded(): Iter<U> {
		const inner = this.inner.folded();
		if (inner === this.inner) return this;
		return new MapWhileIter(inner, this.f, this.none);
	}

	override clone(h?: Clones): Iter<U> {
		return new MapWhileIter(this.inner.clone(h?.i), cloned(this.f, h?.f), this.none);
	}

	override sizeHint(): SizeHint {
		return fewer(this.inner);
	}
}

/**
 * `Scan`, which hands `f` its state, or its place, and yields what `f` returns until it
 * returns `None`: its `next` reads on after, but a fold, by `try_fold`, stops there.
 */
class ScanIter<T, S, U> extends Calling<T, U, (state: never, item: T) => U | undefined | None> {
	state: S;
	private readonly at: S | Place<S>;
	private readonly drop: Drop<S> | undefined;
	/** The `Option`'s `None` that `f` returns. */
	private readonly none: None | undefined;
	private stateDropped = false;

	constructor(inner: Iter<T>, state: S, f: (state: never, item: T) => U | undefined | None, copied: boolean, drop?: Drop<S>, none?: None) {
		super(inner, f);
		this.state = state;
		this.at = copied
			? place(
					() => this.state,
					(v) => {
						this.state = v;
					},
				)
			: state;
		this.drop = drop;
		this.none = none;
	}

	next(): U | Done {
		return this.scan(this.inner.next());
	}

	/** std's folds read its iterator as its `try_fold` does. */
	override folded(): Iter<U> {
		const inner = this.inner.folded();
		if (inner === this.inner) return this;
		return new Folded(
			() => this.scan(inner.next()),
			() => this.nextBack(),
		);
	}

	/** What `f` makes of `item`, which ends it at `None`. */
	private scan(item: T | Done): U | Done {
		return item === DONE ? DONE : iterFromOption(this.f(this.at as never, item), this.none);
	}

	override sizeHint(): SizeHint {
		return fewer(this.inner);
	}

	override [Symbol.dispose](): void {
		super[Symbol.dispose]();
		if (this.stateDropped) return;
		this.stateDropped = true;
		this.drop?.(this.state);
	}
}

/** `StepBy`, whose items drop by `drop` as its iterator's own `nth` and `nth_back` step over them. */
class StepByIter<T> extends Adaptor<T, T> {
	readonly step: number;
	firstTake = true;
	private readonly drop: Drop<T> | undefined;

	constructor(inner: Iter<T>, step: number, drop?: Drop<T>) {
		super(inner);
		this.step = step - 1;
		this.drop = drop;
	}

	override clone(h?: Clones): Iter<T> {
		const it = new StepByIter(this.inner.clone(h?.i), this.step + 1, this.drop);
		it.firstTake = this.firstTake;
		return it;
	}

	next(): T | Done {
		const n = this.firstTake ? 0 : this.step;
		this.firstTake = false;
		return this.inner.nth(n, this.drop);
	}

	override nextBack(): T | Done {
		return this.inner.nthBack(this.backIndex(), this.drop);
	}

	/** How far from the back its last item is. */
	backIndex(): number {
		const rem = this.inner.len() % (this.step + 1);
		return this.firstTake ? (rem === 0 ? this.step : rem - 1) : rem;
	}

	/** std's, of its size hint. */
	override len(): number {
		return hintLen(this);
	}

	override sizeHint(): SizeHint {
		const [lower, upper] = this.inner.sizeHint();
		return [this.size(lower), upper === undefined ? undefined : this.size(upper)];
	}

	/** How many of `n` items it yields. */
	private size(n: number): number {
		if (this.firstTake) return n === 0 ? 0 : 1 + Math.floor((n - 1) / (this.step + 1));
		return Math.floor(n / (this.step + 1));
	}

	/**
	 * std's: the first item, dropped unless it is the one, then every step in one `nth`,
	 * as many as a `usize` counts at a time.
	 */
	override nth(n: number, drop?: Drop<T>): T | Done {
		if (this.firstTake) {
			this.firstTake = false;
			const first = this.inner.next();
			if (first === DONE || n === 0) return first;
			drop?.(first);
			n -= 1;
		}
		let step = this.step + 1;
		if (n === USIZE_MAX) {
			const item = this.inner.nth(step - 1, drop);
			if (item === DONE) return DONE;
			drop?.(item);
		} else {
			n += 1;
		}
		for (;;) {
			if (n * step <= USIZE_MAX) return this.inner.nth(n * step - 1, drop);
			const byN = Math.floor(USIZE_MAX / n) * n;
			const byStep = Math.floor(USIZE_MAX / step) * step;
			if (byN > byStep) step -= Math.floor(USIZE_MAX / n);
			else n -= Math.floor(USIZE_MAX / step);
			const item = this.inner.nth(Math.max(byN, byStep) - 1, drop);
			if (item === DONE) return DONE;
			drop?.(item);
		}
	}

	/** std's: every step from the back in one, saturating at a `usize`'s greatest. */
	override nthBack(n: number, drop?: Drop<T>): T | Done {
		const steps = Math.min(n * (this.step + 1), USIZE_MAX);
		return this.inner.nthBack(Math.min(steps + this.backIndex(), USIZE_MAX), drop);
	}

	/** std's: the first item by `next`, then each step by `nth`, as `next` reads past it. */
	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		if (this.firstTake) {
			this.firstTake = false;
			let done: boolean;
			[done, acc] = foldFirst(this.inner, () => this.inner.next(), acc, f, release);
			if (done) return acc;
		}
		return foldBy(this, false, acc, f, release);
	}

	/** std's `try_rfold`: the last item by `next_back`, then each step back by `nth_back`. */
	override folded(): Iter<T> {
		let first = true;
		const back = (): T | Done => {
			if (!first) return this.inner.nthBack(this.step, this.drop);
			first = false;
			return this.nextBack();
		};
		return new Folded(() => this.next(), back);
	}

	/** std's: the last item by `next_back`, then each step back by `nth_back`. */
	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		let done: boolean;
		[done, acc] = foldFirst(this.inner, () => this.nextBack(), acc, f, release);
		if (done) return acc;
		const steps = new FromNext(this.inner, (inner) => inner.nthBack(this.step, this.drop));
		acc = foldBy(steps, false, acc, f, release);
		dispose(this.inner);
		return acc;
	}
}

/**
 * std's `StepBy` of a `Range` of an unsigned integer no wider than a `usize`, whose
 * greatest is `max`: its range's end counts the items it has left, and its start is the
 * next from the front, which steps past `max` back to 0 only once it has none left.
 */
class RangeStepBy extends Adaptor<number, number> {
	declare readonly inner: Range<number>;
	readonly step: number;
	readonly firstTake = true;
	private readonly max: number;

	/** Of `inner`, whose end already counts its items, by a step of `step` and one. */
	constructor(inner: Range<number>, step: number, max: number) {
		super(inner);
		this.step = step;
		this.max = max;
	}

	override clone(): Iter<number> {
		return new RangeStepBy(this.inner.clone(), this.step, this.max);
	}

	next(): number | Done {
		const r = this.inner;
		if (r.end === 0) return DONE;
		const item = r.start;
		r.start = (item + Math.min(this.step + 1, this.max)) % (this.max + 1);
		r.end--;
		return item;
	}

	override nextBack(): number | Done {
		const r = this.inner;
		if (r.end === 0) return DONE;
		r.end--;
		return r.start + (this.step + 1) * r.end;
	}

	override len(): number {
		return this.inner.end;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.inner.end);
	}
}

/**
 * std's `Cycle`: `iter`, read, and `orig`, the iterator as it was, which it clones by `h`
 * in place of `iter` each time that ends. It drops `orig` and then `iter`, as std's fields
 * order them.
 */
class CycleIter<T> extends Iter<T> implements Disposable {
	readonly orig: Iter<T>;
	iter: Iter<T>;
	private readonly h: Clones | undefined;

	constructor(orig: Iter<T>, iter: Iter<T>, h: Clones | undefined) {
		super();
		this.orig = orig;
		this.iter = iter;
		this.h = h;
	}

	/** A clone of `orig` in place of `iter`, which drops once the clone is made. */
	private restart(): void {
		const ended = this.iter;
		this.iter = this.orig.clone(this.h);
		dispose(ended);
	}

	next(): T | Done {
		const item = this.iter.next();
		if (item !== DONE) return item;
		this.restart();
		return this.iter.next();
	}

	/** std's: by `orig`'s, either empty or endless. */
	override sizeHint(): SizeHint {
		const [lower, upper] = this.orig.sizeHint();
		if (lower > 0) return [USIZE_MAX, undefined];
		return upper === 0 ? [0, 0] : [0, undefined];
	}

	/** std's: `orig` by `h.i`, then `iter`. */
	override clone(h?: Clones): Iter<T> {
		return new CycleIter(this.orig.clone(h?.i), this.iter.clone(h?.i), this.h);
	}

	/** std's: `iter` advanced, then clones, until one advances no further than it had left. */
	override advanceBy(n: number, drop?: Drop<T>): number {
		n = this.iter.advanceBy(n, drop);
		while (n > 0) {
			this.restart();
			const left = this.iter.advanceBy(n, drop);
			if (left === n) return n;
			n = left;
		}
		return 0;
	}

	/**
	 * std's `try_fold`: the rest of `iter` folded, then a clone, which ends it if that was
	 * empty, and then clones without end. `fold` is std's default, by `next`.
	 */
	override folded(fold = false): Iter<T> {
		if (fold) return this;
		let reading = this.iter.folded();
		// Which clone it reads: 0 for none yet, 1 for the first, and 2 for one after.
		let clones = 0;
		let empty = true;
		return new FromNext(this, (cycle) => {
			for (;;) {
				const item = reading.next();
				if (item !== DONE) {
					if (clones === 1) empty = false;
					return item;
				}
				if (clones === 1 && empty) return DONE;
				clones = Math.min(clones + 1, 2);
				cycle.restart();
				reading = cycle.iter.folded();
			}
		});
	}

	[Symbol.dispose](): void {
		dispose(this.orig);
		dispose(this.iter);
	}
}

class InspectIter<T> extends Calling<T, T, (item: T) => void> {
	override sizeHint(): SizeHint {
		return this.inner.sizeHint();
	}

	next(): T | Done {
		return this.inspect(this.inner.next());
	}

	override nextBack(): T | Done {
		return this.inspect(this.inner.nextBack());
	}

	/** `f` of `item`. */
	private inspect(item: T | Done): T | Done {
		if (item !== DONE) this.f(item);
		return item;
	}

	override len(): number {
		return this.inner.len();
	}

	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner) return this;
		return new InspectIter(inner, this.f);
	}

	override clone(h?: Clones): Iter<T> {
		return new InspectIter(this.inner.clone(h?.i), cloned(this.f, h?.f));
	}

	/** std's: `inner`'s, by a closure that drops `f` and then `release`'s. */
	override fold<A>(acc: A, g: (acc: A, item: T) => A, release?: () => void): A {
		return this.inner.fold(acc, this.inspectFold(g), then(this.take(), release));
	}

	override rfold<A>(acc: A, g: (acc: A, item: T) => A, release?: () => void): A {
		return this.inner.rfold(acc, this.inspectFold(g), then(this.take(), release));
	}

	/** std's `inspect_fold`: `f` of each item, then `g`. */
	private inspectFold<A>(g: (acc: A, item: T) => A): (acc: A, item: T) => A {
		const f = this.f;
		return (acc, item) => {
			f(item);
			return g(acc, item);
		};
	}
}

class FuseIter<T> extends Iter<T> implements Disposable {
	inner: Iter<T> | undefined;
	/** Whether its iterator is a `FusedIterator`, which std's keeps once it ends. */
	private readonly fused: boolean;

	constructor(inner: Iter<T> | undefined, fused: boolean) {
		super();
		this.inner = inner;
		this.fused = fused;
	}

	/** std's: its iterator, while it holds it. */
	override clone(h?: Clones): Iter<T> {
		return new FuseIter(this.inner?.clone(h?.i), this.fused);
	}

	override sizeHint(): SizeHint {
		return this.inner?.sizeHint() ?? [0, 0];
	}

	/** `item`, dropping its iterator first if it ended there and is not fused. */
	clear(item: T | Done): T | Done {
		if (item === DONE && !this.fused) {
			const inner = this.inner;
			this.inner = undefined;
			dispose(inner);
		}
		return item;
	}

	next(): T | Done {
		return this.inner === undefined ? DONE : this.clear(this.inner.next());
	}

	override nextBack(): T | Done {
		return this.inner === undefined ? DONE : this.clear(this.inner.nextBack());
	}

	/** std's: its iterator's, dropped if it falls short unless it is fused. */
	override advanceBackBy(n: number, drop?: Drop<T>): number {
		if (this.inner === undefined) return n;
		const left = this.inner.advanceBackBy(n, drop);
		if (left !== 0) this.clear(DONE);
		return left;
	}

	override len(): number {
		return this.inner === undefined ? 0 : this.inner.len();
	}

	[Symbol.dispose](): void {
		const inner = this.inner;
		this.inner = undefined;
		dispose(inner);
	}

	override nth(n: number, drop?: Drop<T>): T | Done {
		return this.inner === undefined ? DONE : this.clear(this.inner.nth(n, drop));
	}

	override finds(): boolean {
		return this.inner?.finds() === true;
	}

	/** std's: its iterator's, dropped if it finds none unless it is fused. */
	override find(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		return this.inner === undefined ? DONE : this.clear(this.inner.find(f, drop));
	}

	override rfind(f: (item: T) => boolean, drop?: Drop<T>): T | Done {
		return this.inner === undefined ? DONE : this.clear(this.inner.rfind(f, drop));
	}

	/** std's: its iterator folded, dropped once it ends unless it is fused. */
	override folded(fold = false): Iter<T> {
		const inner = this.inner?.folded(fold);
		if (inner === this.inner) return this;
		const front = () => (this.inner === undefined ? DONE : this.clear(inner!.next()));
		const back = () => (this.inner === undefined ? DONE : this.clear(inner!.nextBack()));
		return new Folded(front, back);
	}

	/** std's: its iterator's, which it never reads past its end. */
	override indexed(): Iter<T> {
		const inner = this.inner?.indexed();
		if (inner === this.inner) return this;
		return new Indexed(
			() => inner!.next(),
			() => inner!.nextBack(),
			(n) => inner!.advanceBackBy(n),
		);
	}

	override nthBack(n: number, drop?: Drop<T>): T | Done {
		return this.inner === undefined ? DONE : this.clear(this.inner.nthBack(n, drop));
	}

	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const inner = this.inner;
		this.inner = undefined;
		if (inner !== undefined) return inner.fold(acc, f, release);
		release?.();
		return acc;
	}

	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		const inner = this.inner;
		this.inner = undefined;
		if (inner !== undefined) return inner.rfold(acc, f, release);
		release?.();
		return acc;
	}
}

/**
 * `Flatten` and `FlatMap`: the iterator of each item it reads, which drops once it ends,
 * as does its source unless it is `fused`.
 */
class FlattenIter<T, V> extends Iter<V> implements Disposable {
	outer: Iter<T> | undefined;
	readonly into: (value: T) => Iter<V>;
	readonly fused: boolean;
	/** How many items each of its source's has, if each is an array. */
	readonly size: number | undefined;
	/** Whether each of its source's items makes an iterator of at most one item. */
	readonly oneShot: boolean;
	front: Iter<V> | undefined;
	back: Iter<V> | undefined;

	constructor(outer: Iter<T> | undefined, into: (value: T) => Iter<V>, fused: boolean, size?: number, oneShot = false) {
		super();
		this.outer = outer;
		this.into = into;
		this.fused = fused;
		this.size = size;
		this.oneShot = oneShot;
	}

	/** std's: its source by `h.i`, while it holds it, then its front and back by `h.o`. */
	override clone(h?: Clones): Iter<V> {
		const it = new FlattenIter(this.outer?.clone(h?.i), this.into, this.fused, this.size, this.oneShot);
		it.front = this.front?.clone(h?.o);
		it.back = this.back?.clone(h?.o);
		return it;
	}

	/**
	 * std's: what its front and back hold, and its source's items, if it knows how many
	 * each has. Of items that hold at most one, only its source's, which std never holds
	 * the iterator of.
	 */
	override sizeHint(): SizeHint {
		const [lower, upper] = this.outer?.sizeHint() ?? [0, 0];
		if (this.oneShot) {
			if (this.size === 0) return [0, 0];
			return this.size === 1 ? [lower, upper] : [0, upper];
		}
		const [frontLower, frontUpper] = this.front?.sizeHint() ?? [0, 0];
		const [backLower, backUpper] = this.back?.sizeHint() ?? [0, 0];
		const held = satAdd(frontLower, backLower);
		const heldUpper = addUpper(frontUpper, backUpper);
		if (this.size !== undefined) {
			const all = upper === undefined || upper * this.size > USIZE_MAX ? undefined : upper * this.size;
			return [satAdd(lower * this.size, held), addUpper(heldUpper, all)];
		}
		return [held, lower === 0 && upper === 0 ? heldUpper : undefined];
	}

	/** The source's next item, or `DONE`, which drops it unless it is fused. */
	private nextOuter(back: boolean): T | Done {
		const outer = this.outer;
		if (outer === undefined) return DONE;
		const item = back ? outer.nextBack() : outer.next();
		if (item === DONE && !this.fused) {
			this.outer = undefined;
			dispose(outer);
		}
		return item;
	}

	/** `it`'s next item, from the back if `back`, dropping it if it ended. */
	private static step<V>(it: Iter<V>, back: boolean): V | Done {
		const item = back ? it.nextBack() : it.next();
		if (item === DONE) dispose(it);
		return item;
	}

	next(): V | Done {
		for (;;) {
			if (this.front !== undefined) {
				const item = FlattenIter.step(this.front, false);
				if (item !== DONE) return item;
				this.front = undefined;
			}
			const inner = this.nextOuter(false);
			if (inner === DONE) {
				if (this.back === undefined) return DONE;
				const item = FlattenIter.step(this.back, false);
				if (item === DONE) this.back = undefined;
				return item;
			}
			this.front = this.into(inner);
		}
	}

	override nextBack(): V | Done {
		for (;;) {
			if (this.back !== undefined) {
				const item = FlattenIter.step(this.back, true);
				if (item !== DONE) return item;
				this.back = undefined;
			}
			const inner = this.nextOuter(true);
			if (inner === DONE) {
				if (this.front === undefined) return DONE;
				const item = FlattenIter.step(this.front, true);
				if (item === DONE) this.front = undefined;
				return item;
			}
			this.back = this.into(inner);
		}
	}

	/** Hold `it` as the front iterator, dropping the one it held, as std's `insert` does. */
	setFront(it: Iter<V> | undefined): void {
		const old = this.front;
		this.front = it;
		dispose(old);
	}

	setBack(it: Iter<V> | undefined): void {
		const old = this.back;
		this.back = it;
		dispose(old);
	}

	[Symbol.dispose](): void {
		const outer = this.outer;
		const front = this.front;
		const back = this.back;
		this.outer = undefined;
		this.front = undefined;
		this.back = undefined;
		dispose(outer);
		dispose(front);
		dispose(back);
	}

	/** Its fold's view, which each fold starts anew. */
	private view: FlattenFold<T, V> | undefined;

	override folded(fold = false): Iter<V> {
		this.view ??= new FlattenFold(this);
		return this.view.reset(fold);
	}

	override fold<A>(acc: A, f: (acc: A, item: V) => A, release?: () => void): A {
		return flattenFold(this, false, acc, (acc, inner) => inner.fold(acc, f), release);
	}

	override rfold<A>(acc: A, f: (acc: A, item: V) => A, release?: () => void): A {
		return flattenFold(this, true, acc, (acc, inner) => inner.rfold(acc, f), release);
	}

	/**
	 * std's, by `try_fold`: each iterator in turn advanced by its own `advance_by`, of its
	 * source folded, one that falls short dropped only as the next replaces it, and the
	 * source dropped once it ends unless it is fused.
	 */
	override advanceBy(n: number, drop?: Drop<V>): number {
		if (this.front !== undefined) {
			n = this.front.advanceBy(n, drop);
			if (n === 0) return 0;
		}
		this.setFront(undefined);
		const outer = this.outer;
		if (outer !== undefined) {
			const source = outer.folded();
			for (let inner = source.next(); inner !== DONE; inner = source.next()) {
				const front = this.into(inner);
				this.setFront(front);
				n = front.advanceBy(n, drop);
				if (n === 0) return 0;
			}
			if (!this.fused) {
				this.outer = undefined;
				dispose(outer);
			}
		}
		this.setFront(undefined);
		if (this.back !== undefined) {
			n = this.back.advanceBy(n, drop);
			if (n === 0) return 0;
		}
		this.setBack(undefined);
		return n;
	}

	override advanceBackBy(n: number, drop?: Drop<V>): number {
		if (this.back !== undefined) {
			n = this.back.advanceBackBy(n, drop);
			if (n === 0) return 0;
		}
		this.setBack(undefined);
		const outer = this.outer;
		if (outer !== undefined) {
			const source = outer.folded();
			for (let inner = source.nextBack(); inner !== DONE; inner = source.nextBack()) {
				const back = this.into(inner);
				this.setBack(back);
				n = back.advanceBackBy(n, drop);
				if (n === 0) return 0;
			}
			if (!this.fused) {
				this.outer = undefined;
				dispose(outer);
			}
		}
		this.setBack(undefined);
		if (this.front !== undefined) {
			n = this.front.advanceBackBy(n, drop);
			if (n === 0) return 0;
		}
		this.setFront(undefined);
		return n;
	}
}

/**
 * A flatten as std's `try_fold` reads it, or `try_rfold` from the back: the iterator it
 * holds at that end folded, which then drops, then its source folded, each iterator it
 * makes held and folded in turn, dropping the one before, and then the other end's, which
 * drops once it ends. The source drops once it ends unless it is fused.
 */
class FlattenFold<T, V> extends Iter<V> {
	readonly of: FlattenIter<T, V>;
	/** Whether it reads as `fold` does, which folds its source and each iterator. */
	private whole = false;
	/** At its own end, then its source's, then the other end. */
	private phase = 0;
	private source: Iter<T> | undefined;
	/** The iterator it reads, and that folded. */
	private held: Iter<V> | undefined;
	private reading: Iter<V> | undefined;

	constructor(of: FlattenIter<T, V>) {
		super();
		this.of = of;
	}

	/** Itself, at the start of a fold, or a `try_fold` unless `fold`. */
	reset(fold: boolean): this {
		this.whole = fold;
		this.phase = 0;
		this.source = this.of.outer?.folded(fold);
		this.held = this.reading = undefined;
		return this;
	}

	next(): V | Done {
		return this.step(false);
	}

	override nextBack(): V | Done {
		return this.step(true);
	}

	private step(back: boolean): V | Done {
		const of = this.of;
		for (;;) {
			const it = back !== (this.phase === 2) ? of.back : of.front;
			if (it !== undefined) {
				if (it !== this.held) [this.held, this.reading] = [it, it.folded(this.whole)];
				const item = back ? this.reading!.nextBack() : this.reading!.next();
				if (item !== DONE) return item;
			}
			if (this.phase === 2) {
				this.hold(back, undefined, true);
				return DONE;
			}
			if (this.phase === 1) {
				const outer = of.outer;
				const value = outer === undefined ? DONE : back ? this.source!.nextBack() : this.source!.next();
				if (value !== DONE) {
					this.hold(back, of.into(value), false);
					continue;
				}
				if (outer !== undefined && !of.fused) {
					of.outer = undefined;
					dispose(outer);
				}
			}
			this.hold(back, undefined, false);
			this.phase++;
		}
	}

	/** Hold `it` at the end it folds from, from the back if `back`, or at the other. */
	private hold(back: boolean, it: Iter<V> | undefined, other: boolean): void {
		if (back !== other) this.of.setBack(it);
		else this.of.setFront(it);
	}
}

/** `Peekable`, which drops the item it peeked by `drop`. */
export class Peekable<T> extends Adaptor<T, T> {
	peeked = false;
	item: T | Done = DONE;
	readonly drop: Drop<T> | undefined;

	constructor(inner: Iter<T>, drop?: Drop<T>) {
		super(inner);
		this.drop = drop;
	}

	/** std's: its iterator by `h.i`, then the item it peeked by `h.v`. */
	override clone(h?: Clones): Peekable<T> {
		const it = new Peekable(this.inner.clone(h?.i), this.drop);
		it.peeked = this.peeked;
		it.item = this.item === DONE ? DONE : cloned(this.item, h?.v);
		return it;
	}

	/** The item it peeked, if any, which it no longer holds. */
	take(): T | Done {
		this.peeked = false;
		const item = this.item;
		this.item = DONE;
		return item;
	}

	/** std's: none past a peeked end, and one more than its iterator's for a peeked item. */
	override sizeHint(): SizeHint {
		if (!this.peeked) return this.inner.sizeHint();
		if (this.item === DONE) return [0, 0];
		const [lower, upper] = this.inner.sizeHint();
		return [satAdd(lower, 1), addUpper(upper, 1)];
	}

	next(): T | Done {
		if (!this.peeked) return this.inner.next();
		return this.take();
	}

	override nextBack(): T | Done {
		if (!this.peeked) return this.inner.nextBack();
		if (this.item === DONE) return DONE;
		const item = this.inner.nextBack();
		if (item !== DONE) return item;
		// The peeked item is the last: taking it leaves the end peeked.
		const peeked = this.item;
		this.item = DONE;
		return peeked;
	}

	/** std's, of its size hint. */
	override len(): number {
		return hintLen(this);
	}

	override [Symbol.dispose](): void {
		dispose(this.inner);
		const item = this.take();
		if (item !== DONE) this.drop?.(item);
	}

	/**
	 * std's: the item it peeked, or none past a peeked end, and then its iterator folded; from
	 * the back, its iterator folded and then the item it peeked, which it holds until then.
	 */
	override folded(fold = false): Iter<T> {
		const inner = this.inner.folded(fold);
		if (inner === this.inner) return this;
		const front = () => (this.peeked ? this.take() : inner.next());
		const back = (): T | Done => {
			if (!this.peeked) return inner.nextBack();
			if (this.item === DONE) return this.take();
			const item = inner.nextBack();
			return item !== DONE ? item : this.take();
		};
		return new Folded(front, back);
	}

	/** std's: the item it peeked, and then its iterator's fold. */
	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		if (this.peeked) {
			let done: boolean;
			[done, acc] = foldFirst(this.inner, () => this.take(), acc, f, release);
			if (done) return acc;
		}
		return this.inner.fold(acc, f, release);
	}

	/** std's: its iterator's from the back, and then the item it peeked. */
	override rfold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		if (!this.peeked) return this.inner.rfold(acc, f, release);
		const peeked = this.take();
		if (peeked === DONE) {
			ended(this.inner, release);
			return acc;
		}
		acc = f(this.inner.rfold(acc, f), peeked);
		release?.();
		return acc;
	}

	/** std's: the item it peeked drops after what its iterator skips. */
	override nth(n: number, drop?: Drop<T>): T | Done {
		if (!this.peeked) return this.inner.nth(n, drop);
		const peeked = this.take();
		if (peeked === DONE) return DONE;
		if (n === 0) return peeked;
		const item = this.inner.nth(n - 1, drop);
		drop?.(peeked);
		return item;
	}
}

/** The item `p` peeked, peeking it first if it has not. */
function peeked<T>(p: Peekable<T>): T | Done {
	if (!p.peeked) {
		p.item = p.inner.next();
		p.peeked = true;
	}
	return p.item;
}

/** `Peekable::peek`. */
export function peekablePeek<T>(p: Peekable<T>): T | undefined;
export function peekablePeek<T>(p: Peekable<T>, none: None): T | None;
export function peekablePeek<T>(p: Peekable<T>, none?: None): T | undefined | None {
	return iterToOption(peeked(p), none);
}

/** `Peekable::peek_mut`: the item it peeked, as its place if it is one JS copies. */
export function peekablePeekMut<T>(p: Peekable<T>, lend: true): Place<T> | undefined;
export function peekablePeekMut<T>(p: Peekable<T>, lend: false): T | undefined;
export function peekablePeekMut<T>(p: Peekable<T>, lend: boolean): Place<T> | T | undefined {
	const item = peeked(p);
	if (item === DONE) return undefined;
	return lend ? place(() => p.item as T, (value: T) => (p.item = value)) : item;
}

/** `Peekable::next_if`: an item `f` rejects is peeked again. */
export function peekableNextIf<T>(p: Peekable<T>, f: (item: T) => boolean): T | undefined;
export function peekableNextIf<T>(p: Peekable<T>, f: (item: T) => boolean, none: None): T | None;
export function peekableNextIf<T>(p: Peekable<T>, f: (item: T) => boolean, none?: None): T | undefined | None;
export function peekableNextIf<T>(p: Peekable<T>, f: (item: T) => boolean, none?: None): T | undefined | None {
	const item = p.next();
	if (item !== DONE && f(item)) return item;
	p.peeked = true;
	p.item = item;
	return none;
}

/** `Peekable::next_if_eq`: `next_if` by `eq` of the next item and `expected`. */
export function peekableNextIfEq<T, U>(p: Peekable<T>, expected: U, eq: (a: T, b: U) => boolean): T | undefined;
export function peekableNextIfEq<T, U>(p: Peekable<T>, expected: U, eq: (a: T, b: U) => boolean, none: None): T | None;
export function peekableNextIfEq<T, U>(p: Peekable<T>, expected: U, eq: (a: T, b: U) => boolean, none?: None): T | undefined | None {
	return peekableNextIf(p, (item: T) => eq(item, expected), none);
}

/**
 * What `iterCollectResult` collects from: values until an error, which it keeps. Read
 * again, it reads on, as std's does, and a later error replaces it, which drops by `dropError`.
 */
class ResultShunt<U, E> extends Adaptor<Result<U, E>, U> {
	error: { value: E } | undefined;
	private readonly dropError: Drop<E> | undefined;

	constructor(inner: Iter<Result<U, E>>, dropError: Drop<E> | undefined) {
		super(inner);
		this.dropError = dropError;
	}

	/** std's, by `try_for_each`, which folds its iterator. */
	next(): U | Done {
		const item = this.inner.folded().next();
		if (item === DONE) return DONE;
		if (item.ok) return item.value;
		if (this.error !== undefined) this.dropError?.(this.error.value);
		this.error = { value: item.error };
		return DONE;
	}

	override sizeHint(): SizeHint {
		return this.error === undefined ? fewer(this.inner) : [0, 0];
	}
}

/**
 * What `iterCollectOption` collects from: values until a `None`, which is `itemNone`. Read
 * again, it reads on, as std's does.
 */
class OptionShunt<U> extends Adaptor<U | undefined | None, U> {
	none = false;
	private readonly itemNone: None | undefined;

	constructor(inner: Iter<U | undefined | None>, itemNone: None | undefined) {
		super(inner);
		this.itemNone = itemNone;
	}

	next(): U | Done {
		const item = this.inner.folded().next();
		if (item === DONE) return DONE;
		if (item !== this.itemNone) return item as U;
		this.none = true;
		return DONE;
	}

	override sizeHint(): SizeHint {
		return this.none ? [0, 0] : fewer(this.inner);
	}
}

/** `a < b` of two integers, `char`s, or floats of one type, or two booleans. */
function lt<T>(a: T, b: T): boolean {
	if (typeof a === "object" || typeof b === "object") return wideLt(a as number | Limbs, b as number | Limbs);
	return (a as number) < (b as number);
}

/** `a <= b` of two of what `lt` compares: never of a NaN. */
function le<T>(a: T, b: T): boolean {
	if (typeof a === "object" || typeof b === "object") return !wideLt(b as number | Limbs, a as number | Limbs);
	return (a as number) <= (b as number);
}

/**
 * `n + by` in the integer type `int` steps, or `undefined` past its range, which a range
 * stepping past its end only needs to stay past. A narrow integer's sum is an exact
 * number whatever its type; a `char`'s skips the surrogates.
 */
function step<T>(n: T, by: number, int: Step<T> | undefined): T | undefined {
	if (typeof n === "number" && int !== (charStep as unknown) && Number.isSafeInteger(n + by)) return (n + by) as T;
	if (int === undefined) throw new Error("rust2-ts: stepping a wide range without its type");
	const [sum, over] = int(n, by);
	return over ? undefined : sum;
}

/**
 * std's `Step::steps_between` of `a` and `b`, at most `b`, in the type `int` steps:
 * `(usize::MAX, None)` past `usize::MAX`, and less the surrogates `char`s skip.
 */
function stepsBetween<T>(a: T, b: T, int: Step<T> | undefined): SizeHint {
	if (int === (charStep as unknown)) {
		const n = (b as number) - (a as number);
		return exactHint((a as number) < 0xd800 && (b as number) >= 0xe000 ? n - 0x800 : n);
	}
	const n = typeof a === "number" && typeof b === "number" ? b - a : intToBig(b as number | Limbs) - intToBig(a as number | Limbs);
	return n <= USIZE_MAX ? exactHint(Number(n)) : [USIZE_MAX, undefined];
}

/**
 * `ops::Range`: `start..end`, which iterates its integers by moving `start`. A 64- or
 * 128-bit range takes its type's `Step`, `rs.u64Step`, to step past 2^53, and a range
 * of `char`s `rs.charStep`, to step past the surrogates.
 */
export class Range<T> extends Iter<T> {
	start: T;
	end: T;
	readonly int: Step<T> | undefined;

	constructor(start: T, end: T, int?: Step<T>) {
		super();
		this.start = start;
		this.end = end;
		this.int = int;
	}

	next(): T | Done {
		const start = this.start;
		if (!lt(start, this.end)) return DONE;
		this.start = step(start, 1, this.int)!;
		return start;
	}

	override nextBack(): T | Done {
		if (!lt(this.start, this.end)) return DONE;
		this.end = step(this.end, -1, this.int)!;
		return this.end;
	}

	/** std's `nth`, in one step: past the end, the range ends. */
	override nth(n: number): T | Done {
		const item = step(this.start, n, this.int);
		if (item !== undefined && lt(item, this.end)) {
			this.start = step(item, 1, this.int)!;
			return item;
		}
		this.start = this.end;
		return DONE;
	}

	/** std's `advance_by`, in one step. */
	override advanceBy(n: number): number {
		const [len] = this.sizeHint();
		const skip = Math.min(n, len);
		this.start = step(this.start, skip, this.int)!;
		return n - skip;
	}

	/** std's `advance_back_by`, in one step. */
	override advanceBackBy(n: number): number {
		const [len] = this.sizeHint();
		const skip = Math.min(n, len);
		this.end = step(this.end, -skip, this.int)!;
		return n - skip;
	}

	/** std's `nth_back`, in one step. */
	override nthBack(n: number): T | Done {
		const item = step(this.end, -n, this.int);
		if (item !== undefined && lt(this.start, item)) {
			this.end = step(item, -1, this.int)!;
			return this.end;
		}
		this.end = this.start;
		return DONE;
	}

	override len(): number {
		return lt(this.start, this.end) ? Number(this.end) - Number(this.start) : 0;
	}

	override sizeHint(): SizeHint {
		return lt(this.start, this.end) ? stepsBetween(this.start, this.end, this.int) : [0, 0];
	}

	override clone(): Range<T> {
		return new Range(this.start, this.end, this.int);
	}
}

/** `Range::is_empty`. */
export function rangeIsEmpty<T>(r: Range<T>): boolean {
	return !lt(r.start, r.end);
}

/** `Range::contains`: `start <= item < end`, so nothing past a NaN. */
export function rangeContains<T>(r: Range<T>, item: T): boolean {
	return le(r.start, item) && lt(item, r.end);
}

/**
 * `ops::RangeFrom`: `start..`, which iterates by moving `start` and never ends. It takes
 * its type's `Step`, `rs.u8Step` or `rs.charStep`, past whose last value it panics, as
 * std's does in a debug build.
 */
export class RangeFrom<T> extends Iter<T> {
	start: T;
	readonly int: Step<T> | undefined;

	constructor(start: T, int?: Step<T>) {
		super();
		this.start = start;
		this.int = int;
	}

	/** std's: the next `start` is found before the item is given, so the last value panics. */
	next(): T | Done {
		const start = this.start;
		this.start = forward(start, 1, this.int);
		return start;
	}

	override nth(n: number): T | Done {
		const item = forward(this.start, n, this.int);
		this.start = forward(item, 1, this.int);
		return item;
	}

	override sizeHint(): SizeHint {
		return [USIZE_MAX, undefined];
	}

	override clone(): RangeFrom<T> {
		return new RangeFrom(this.start, this.int);
	}
}

/** `RangeFrom::contains`. */
export function rangeFromContains<T>(r: RangeFrom<T>, item: T): boolean {
	return le(r.start, item);
}

/** `RangeTo::contains` and `RangeToInclusive::contains`. */
export function rangeToContains<T>(r: { readonly end: T; readonly inclusive?: true }, item: T): boolean {
	return r.inclusive ? le(item, r.end) : lt(item, r.end);
}

/** `RangeBounds::contains` of a pair of `Bound`s. */
export function boundsContains<T>([start, end]: readonly [RangeBound<T>, RangeBound<T>], item: T): boolean {
	const above = start.tag === "Unbounded" || (start.tag === "Included" ? le(start._0, item) : lt(start._0, item));
	return above && (end.tag === "Unbounded" || (end.tag === "Included" ? le(item, end._0) : lt(item, end._0)));
}

/** std's `Step::forward` in a debug build: `n + by`, panicking past `int`'s last value. */
function forward<T>(n: T, by: number, int: Step<T> | undefined): T {
	if (int === undefined) throw new Error("rust2-ts: stepping a range of no integer type");
	const [sum, over] = int(n, by);
	if (!over) return sum;
	if (int === (charStep as unknown)) panic("overflow in `Step::forward`", "char_forward", true);
	return panic("attempt to add with overflow", "int_forward");
}

/** `ops::RangeInclusive`: `start..=end`, which remembers once it has yielded `end`. */
export class RangeInclusive<T> extends Iter<T> {
	lo: T;
	hi: T;
	exhausted = false;
	/** The integer type, which `nth` wraps within as std's does. */
	readonly int: Step<T> | undefined;

	constructor(start: T, end: T, int?: Step<T>) {
		super();
		this.lo = start;
		this.hi = end;
		this.int = int;
	}

	/** std's: `start` steps past the item, so it passes `end`, and only wrapping exhausts it. */
	next(): T | Done {
		if (rangeInclusiveIsEmpty(this)) return DONE;
		const lo = this.lo;
		[this.lo, this.exhausted] = stepOverflowing(lo, 1, this.int);
		return lo;
	}

	override folded(): Iter<T> {
		return new RangeInclusiveFold(this);
	}

	override clone(): RangeInclusive<T> {
		const range = new RangeInclusive(this.lo, this.hi, this.int);
		range.exhausted = this.exhausted;
		return range;
	}

	override nextBack(): T | Done {
		if (rangeInclusiveIsEmpty(this)) return DONE;
		const hi = this.hi;
		[this.hi, this.exhausted] = stepOverflowing(hi, -1, this.int);
		return hi;
	}

	override len(): number {
		return rangeInclusiveIsEmpty(this) ? 0 : Number(this.hi) - Number(this.lo) + 1;
	}

	override sizeHint(): SizeHint {
		if (rangeInclusiveIsEmpty(this)) return [0, 0];
		const [lower, upper] = stepsBetween(this.lo, this.hi, this.int);
		return [satAdd(lower, 1), addUpper(upper, 1)];
	}

	/** std's `nth`: `start` moves past the item, wrapping if it overflows, which ends the range. */
	override nth(n: number): T | Done {
		if (rangeInclusiveIsEmpty(this)) return DONE;
		const [item, over] = overflowing(this.lo, n, this.int);
		const [after, overAfter] = overflowing(item, 1, this.int);
		this.lo = after;
		this.exhausted = over || overAfter;
		return !over && !lt(this.hi, item) ? item : DONE;
	}

	/** std's `nth_back`, from the end. */
	override nthBack(n: number): T | Done {
		if (rangeInclusiveIsEmpty(this)) return DONE;
		const [item, over] = overflowing(this.hi, -n, this.int);
		const [before, overBefore] = overflowing(item, -1, this.int);
		this.hi = before;
		this.exhausted = over || overBefore;
		return !over && !lt(item, this.lo) ? item : DONE;
	}
}

/**
 * An inclusive range as std's `try_fold` steps it, and `try_rfold` from the back: `start`
 * or `end` stops at the other, which exhausts it, so it never wraps.
 */
class RangeInclusiveFold<T> extends Iter<T> {
	readonly range: RangeInclusive<T>;

	constructor(range: RangeInclusive<T>) {
		super();
		this.range = range;
	}

	next(): T | Done {
		const r = this.range;
		if (rangeInclusiveIsEmpty(r)) return DONE;
		const lo = r.lo;
		if (lt(lo, r.hi)) r.lo = step(lo, 1, r.int)!;
		else r.exhausted = true;
		return lo;
	}

	override nextBack(): T | Done {
		const r = this.range;
		if (rangeInclusiveIsEmpty(r)) return DONE;
		const hi = r.hi;
		if (lt(r.lo, hi)) r.hi = step(hi, -1, r.int)!;
		else r.exhausted = true;
		return hi;
	}

	override sizeHint(): SizeHint {
		return this.range.sizeHint();
	}
}

/** `RangeInclusive::start`. */
export function rangeInclusiveStart<T>(r: RangeInclusive<T>): T {
	return r.lo;
}

/** `RangeInclusive::end`. */
export function rangeInclusiveEnd<T>(r: RangeInclusive<T>): T {
	return r.hi;
}

/** `RangeInclusive::into_inner`: its bounds as they are, even exhausted. */
export function rangeInclusiveIntoInner<T>(r: RangeInclusive<T>): [T, T] {
	return [r.lo, r.hi];
}

/** `RangeInclusive::is_empty`: exhausted, or not `start <= end`, as of a NaN. */
export function rangeInclusiveIsEmpty<T>(r: RangeInclusive<T>): boolean {
	return r.exhausted || !le(r.lo, r.hi);
}

/** `RangeInclusive::contains`. */
export function rangeInclusiveContains<T>(r: RangeInclusive<T>, item: T): boolean {
	// Exhausted, it may have wrapped past its end, and contains nothing, as std's bounds say.
	return !r.exhausted && le(r.lo, item) && le(item, r.hi);
}

/** What `==` and `Hash` read of any of std's ranges that holds a bound, whatever steps it. */
type Bounded<T> =
	| { readonly lo: T; readonly hi: T; readonly exhausted: boolean }
	| { readonly start: T; readonly end: T }
	| { readonly start: T }
	| { readonly end: T };

/**
 * `PartialEq::eq` of std's ranges of one kind, from their bounds' `eq`: each bound's, then
 * whether an inclusive one is exhausted, as std's derived one compares its fields.
 */
export function rangeEq<T>(eq: (a: T, b: T) => boolean): (a: Bounded<T>, b: Bounded<T>) => boolean {
	return (a, b) => {
		if ("exhausted" in a) {
			const c = b as typeof a;
			return eq(a.lo, c.lo) && eq(a.hi, c.hi) && a.exhausted === c.exhausted;
		}
		const c = b as { readonly start: T; readonly end: T };
		return (!("start" in a) || eq(a.start, c.start)) && (!("end" in a) || eq(a.end, c.end));
	};
}

/** `Hash::hash` of std's ranges, from their bounds': each bound's, then whether an inclusive one is exhausted. */
export function rangeHash<T>(hash: (value: T, s: Hasher) => void): (value: Bounded<T>, s: Hasher) => void {
	return (r, s) => {
		if ("exhausted" in r) {
			hash(r.lo, s);
			hash(r.hi, s);
			hash8(r.exhausted, s);
			return;
		}
		if ("start" in r) hash(r.start, s);
		if ("end" in r) hash(r.end, s);
	};
}

/** `RangeInclusive::start_bound`: its start, included. */
export function rangeInclusiveStartBound<T>(r: RangeInclusive<T>): RangeBound<T> {
	return { tag: "Included", _0: r.lo };
}

/** `RangeInclusive::end_bound`: its end, included, until wrapping exhausts it and it excludes its start, as std's. */
export function rangeInclusiveEndBound<T>(r: RangeInclusive<T>): RangeBound<T> {
	return r.exhausted ? { tag: "Excluded", _0: r.lo } : { tag: "Included", _0: r.hi };
}

/** `Step::forward_overflowing` and `backward_overflowing`: `n + by`, wrapped within `int`, and whether it wrapped. */
/** `Step::forward_overflowing` by `by`, of an integer type's step or a safe number's. */
function stepOverflowing<T>(n: T, by: number, int: Step<T> | undefined): [T, boolean] {
	if (int !== undefined) return int(n, by);
	return [step(n, by, int)!, false];
}

function overflowing<T>(n: T, by: number, int: Step<T> | undefined): [T, boolean] {
	if (int === undefined) throw new Error("rust2-ts: stepping a range of no integer type");
	return int(n, by);
}

/** An iterator as itself: what `into_iter` makes of one. */
function itself<T>(iter: Iter<T>): Iter<T> {
	return iter;
}

/** A value as itself: what `cloned` makes of a JS value. */
function same<T>(value: T): T {
	return value;
}
