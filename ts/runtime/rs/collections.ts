// Collections, for rust2-ts output: std's `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`,
// `VecDeque`, `LinkedList`, and `BinaryHeap`, hashbrown's `HashTable`, which std's maps are
// built on, and `Ordering`, which their keys need. `rs.ts`
// re-exports all of it. Each collection is one type whose functions are free functions
// here, so a bundler keeps only those a caller reaches.
//
// A key compares as Rust's `Eq` or `Ord` says, never by JS identity: a hash collection
// finds a key that is a JS object through its translated `Hash` and `Eq`, and a B-tree
// orders every key by its translated `Ord`. Which `Hash` and `Eq` calls the runtime makes,
// and how often, are its own, not std's; a B-tree makes std's `Ord` calls.
//
// No key, value, or item is ever `undefined`, which the translator refuses, so
// `undefined` is a missing one.
//
// A function that may drop a key, value, or item std's would, rather than return or keep
// it, takes what drops each that does, as `k`, `v`, or `f`, and drops it where std's does.

import { type ArrayRef, ArraySlice, NO_MEMORY, nowhere, arraySliceOfSlice, Borrowed, ByteSlice, ByteVec, byteVecPush, byteVecReserve, capReserve, capReserveExact, capTryReserve, capTryReserveExact, type CloneFrom, type Debug, debugStructField, debugStructFinishNonExhaustive, Err, type Fmt, fmtDebugStruct, fmtDebugStructFields, fmtDebugTupleFields, fmtIter, fmtList, type FmtResult, fmtSet, fmtTuple, type Formatter, type None, OK, Ok, panic, type Place, type RangeBound, rangeCheck, type Result, type Room, roomPast, searchBy, type Elements, Slice, sliceData, sliceFrom, sliceLength, sliceSort, sliceStart, Vec, vecAppend, vecCapacity, vecCloneFrom, vecFrom, vecHold, vecKeep, vecMax, vecReserve, vecWithCapacity, panicSizes } from "../rs.ts";
import { allocError, CAPACITY_OVERFLOW, type TryReserveError } from "./alloc.ts";
import type { HashKey, Hashing } from "./hash.ts";
import { type Limbs, lt, type u64, type usize } from "./int.ts";
import type { String as RsString } from "./str.ts";
import {
	type Clones,
	DONE,
	type Done,
	exactHint,
	Folded,
	Iter,
	iterChain,
	iterEmpty,
	iterFilter,
	iterFold,
	iterExtend,
	iterExtendBytes,
	iterForEach,
	iterHeld,
	iterMap,
	iterToVec,
	iterToByteVec,
	iterOnce,
	iterSlice,
	type RangeInclusive,
	rangeInclusiveEndBound,
	rangeInclusiveStartBound,
	type SizeHint,
	satAdd,
	type Take,
	capNext,
	trustedUpper,
	vecIntoIter,
} from "./iter.ts";
import { type Arc, closure, drop as dispose, dropAll, dropRange } from "./task.ts";
import type { Location } from "./location.ts";

// Ordering

/** `core::cmp::Ordering`: its discriminant, as `as i8` reads it. */
export type Ordering = -1 | 0 | 1;

/** `Ord::cmp` of two integers of one type, or of two `bool`s. */
export function cmp<T extends number | boolean | Limbs>(a: T, b: T): Ordering {
	if (typeof a !== "object" && typeof b !== "object") return a < b ? -1 : a > b ? 1 : 0;
	const [x, y] = [a as number | Limbs, b as number | Limbs];
	return lt(x, y) ? -1 : lt(y, x) ? 1 : 0;
}

/**
 * `Ord::cmp` of two strings, by code point as Rust's UTF-8 bytes sort. JS's `<` compares
 * UTF-16 units, which puts a character past U+FFFF, a surrogate pair, below U+E000.
 */
export function cmpStr(x: string | RsString, y: string | RsString): Ordering {
	const a = typeof x === "string" ? x : x.v;
	const b = typeof y === "string" ? y : y.v;
	if (a === b) return 0;
	const n = Math.min(a.length, b.length);
	for (let i = 0; i < n; i++) {
		let x = a.charCodeAt(i);
		let y = b.charCodeAt(i);
		if (x === y) continue;
		if (x >= 0xd800 && y >= 0xd800) {
			x = x >= 0xe000 ? x - 0x800 : x + 0x2000;
			y = y >= 0xe000 ? y - 0x800 : y + 0x2000;
		}
		return x < y ? -1 : 1;
	}
	return a.length < b.length ? -1 : 1;
}

/** `Ord::cmp` of what holds nothing, as `()` and `PhantomData`: always `Equal`. */
export function cmpNothing(_a: undefined, _b: undefined): Ordering {
	return 0;
}

/**
 * `Ord::cmp` of tuples of type `T`, or `PartialOrd::partial_cmp`, from each field's: the
 * first that is not `Equal`, as `None` is not. A caller passes `T` and `O`, which `tsc`
 * infers from neither a generic field, such as a slice's of `rs.cmp`, nor only total ones.
 */
export function cmpTuple<T extends readonly unknown[], O extends Ordering | undefined>(fields: { readonly [I in keyof T]: (a: T[I], b: T[I]) => O }): (a: T, b: T) => O {
	return (a, b) => {
		for (let i = 0; i < fields.length; i++) {
			const c = (fields[i] as (a: unknown, b: unknown) => O)(a[i], b[i]);
			if (c !== 0) return c;
		}
		return 0 as O;
	};
}

/**
 * `PartialOrd::lt` of tuples, or `le`, `gt`, or `ge`: `test` of their last fields, unless
 * `cmps`, the `partial_cmp`s of the others, find one not `Equal` first, which decides
 * whether it is `order`, as `None` is not.
 */
export function testTuple<T extends readonly unknown[]>(
	cmps: readonly ((a: never, b: never) => Ordering | undefined)[],
	test: (a: never, b: never) => boolean,
	order: Ordering,
): (a: T, b: T) => boolean {
	return (a, b) => {
		for (let i = 0; i < cmps.length; i++) {
			const c = (cmps[i] as (a: unknown, b: unknown) => Ordering | undefined)(a[i], b[i]);
			if (c !== 0) return c === order;
		}
		return (test as (a: unknown, b: unknown) => boolean)(a[cmps.length], b[cmps.length]);
	};
}

/** `PartialOrd::lt` by `cmp`, a `partial_cmp`, as the trait's default tests it: whether it is `Less`. */
export function cmpLt<T>(cmp: (a: T, b: T) => Ordering | undefined): (a: T, b: T) => boolean {
	return (a, b) => cmp(a, b) === -1;
}

/** `PartialOrd::le` by `cmp`, a `partial_cmp`: whether it is `Less` or `Equal`. */
export function cmpLe<T>(cmp: (a: T, b: T) => Ordering | undefined): (a: T, b: T) => boolean {
	return (a, b) => {
		const c = cmp(a, b);
		return c !== undefined && c <= 0;
	};
}

/** `PartialOrd::gt` by `cmp`, a `partial_cmp`: whether it is `Greater`. */
export function cmpGt<T>(cmp: (a: T, b: T) => Ordering | undefined): (a: T, b: T) => boolean {
	return (a, b) => cmp(a, b) === 1;
}

/** `PartialOrd::ge` by `cmp`, a `partial_cmp`: whether it is `Greater` or `Equal`. */
export function cmpGe<T>(cmp: (a: T, b: T) => Ordering | undefined): (a: T, b: T) => boolean {
	return (a, b) => {
		const c = cmp(a, b);
		return c !== undefined && c >= 0;
	};
}

/** `Ord::cmp` of `Option`s, or `PartialOrd::partial_cmp`, from their values': `None` first. */
export function cmpOption<T, O extends Ordering | undefined>(cmp: (a: T, b: T) => O): (a: T | undefined, b: T | undefined) => O;
export function cmpOption<T, O extends Ordering | undefined>(cmp: (a: T, b: T) => O, none: None): (a: T | None, b: T | None) => O;
export function cmpOption<T, O extends Ordering | undefined>(
	cmp: (a: T, b: T) => O,
	none?: None,
): (a: T | undefined | None, b: T | undefined | None) => O {
	return (a, b) => (a === none ? (b === none ? 0 : -1) : b === none ? 1 : cmp(a as T, b as T)) as O;
}

/**
 * `Ord::cmp` of `Reverse`s, or `PartialOrd::partial_cmp` or one of its tests: their
 * values', the other way.
 */
export function cmpReverse<T, O>(cmp: (a: T, b: T) => O): (a: T, b: T) => O {
	return (a, b) => cmp(b, a);
}

/**
 * `Ord::cmp` of slices, or `PartialOrd::partial_cmp`, by their elements' in order, then
 * by length.
 */
export function cmpSlice<T, O extends Ordering | undefined>(cmp: (a: T, b: T) => O): (x: Elements<T>, y: Elements<T>) => O {
	return (x, y) => {
		const [a, i, m, b, j, n] = [sliceData(x), sliceStart(x), sliceLength(x), sliceData(y), sliceStart(y), sliceLength(y)];
		for (let k = 0; k < Math.min(m, n); k++) {
			const c = cmp(a[i + k], b[j + k]);
			if (c !== 0) return c;
		}
		return (m < n ? -1 : m > n ? 1 : 0) as O;
	};
}

/** `cmpSlice`'s of `VecDeque`s, by their items in order however each one's buffer wraps. */
export function cmpDeque<T, O extends Ordering | undefined>(cmp: (a: T, b: T) => O): (a: VecDeque<T>, b: VecDeque<T>) => O {
	return (a, b) => {
		const n = Math.min(a.n, b.n);
		for (let i = 0; i < n; i++) {
			const c = cmp(a.b[phys(a, i)] as T, b.b[phys(b, i)] as T);
			if (c !== 0) return c;
		}
		return (a.n < b.n ? -1 : a.n > b.n ? 1 : 0) as O;
	};
}

/** `cmpSlice`'s of `LinkedList`s, as their `Iterator::cmp` and `partial_cmp` read them. */
export function cmpList<T, O extends Ordering | undefined>(cmp: (a: T, b: T) => O): (a: LinkedList<T>, b: LinkedList<T>) => O {
	return (a, b) => {
		let [x, y] = [a.h, b.h];
		for (; x !== undefined && y !== undefined; x = x.x, y = y.x) {
			const c = cmp(x.v, y.v);
			if (c !== 0) return c;
		}
		return (a.n < b.n ? -1 : a.n > b.n ? 1 : 0) as O;
	};
}

/** `PartialEq::eq` of `LinkedList`s: their lengths, then each pair in order by `eq`, which std's `Iterator::eq` calls. */
export function eqList<T>(eq: (a: T, b: T) => boolean): (a: LinkedList<T>, b: LinkedList<T>) => boolean {
	return (a, b) => {
		if (a.n !== b.n) return false;
		for (let [x, y] = [a.h, b.h]; x !== undefined && y !== undefined; x = x.x, y = y.x) if (!eq(x.v, y.v)) return false;
		return true;
	};
}

/**
 * `Ord::cmp` of byte arrays, slices, or vectors, or `PartialOrd::partial_cmp`: by their
 * bytes in order, then by length. Each is a `[u8; N]`, a `&[u8]` or `&[u8; N]`, or a
 * `Vec<u8>`, whichever a key or an operand holds.
 */
export function cmpBytes(a: Uint8Array | ByteSlice | ByteVec, b: Uint8Array | ByteSlice | ByteVec): Ordering {
	const x = a instanceof Uint8Array ? a : a.data;
	const y = b instanceof Uint8Array ? b : b.data;
	const i = a instanceof ByteSlice ? a.start : 0;
	const j = b instanceof ByteSlice ? b.start : 0;
	const n = Math.min(a.length, b.length);
	for (let k = 0; k < n; k++) {
		if (x[i + k] !== y[j + k]) return x[i + k] < y[j + k] ? -1 : 1;
	}
	return a.length < b.length ? -1 : a.length > b.length ? 1 : 0;
}

/**
 * `PartialEq::eq` of byte arrays, slices, or vectors: their lengths', then their bytes in
 * order, of whichever form each is, as `cmpBytes` reads them.
 */
export function eqBytes(a: Uint8Array | ByteSlice | ByteVec, b: Uint8Array | ByteSlice | ByteVec): boolean {
	if (a.length !== b.length) return false;
	const x = a instanceof Uint8Array ? a : a.data;
	const y = b instanceof Uint8Array ? b : b.data;
	const i = a instanceof ByteSlice ? a.start : 0;
	const j = b instanceof ByteSlice ? b.start : 0;
	for (let k = 0; k < a.length; k++) if (x[i + k] !== y[j + k]) return false;
	return true;
}

/** `Ord::cmp` of `Arc`s, or `PartialOrd::partial_cmp` or one of its tests: their values'. */
export function cmpArc<T, O>(cmp: (a: T, b: T) => O): (a: Arc<T>, b: Arc<T>) => O {
	return (a, b) => cmp(a.value, b.value);
}

/**
 * `Ord::clamp`, by `PartialOrd`'s `le`, `lt`, and `gt`: `min` if `a` is less, `max` if
 * greater, or else `a`, once `min <= max` is asserted. The operands it does not return
 * drop, the last first, as std's do.
 */
export function ordClamp<T>(
	a: T,
	min: T,
	max: T,
	le: (a: T, b: T) => boolean,
	lt: (a: T, b: T) => boolean,
	gt: (a: T, b: T) => boolean,
	drop?: (value: T) => void): T {
	if (!le(min, max)) panic("assertion failed: min <= max", "ord_clamp");
	const out = lt(a, min) ? 1 : gt(a, max) ? 2 : 0;
	if (drop !== undefined) dropAll([max, min, a].filter((_, i) => 2 - i !== out), drop);
	return out === 1 ? min : out === 2 ? max : a;
}

/** `Ord::max`, by `lt`: `a` if `b < a`, or else `b`, and the other drops. */
export function ordMax<T>(a: T, b: T, lt: (a: T, b: T) => boolean, drop?: (value: T) => void): T {
	if (lt(b, a)) {
		drop?.(b);
		return a;
	}
	drop?.(a);
	return b;
}

/** `Ord::min`, by `lt`: `b` if `b < a`, or else `a`, and the other drops. */
export function ordMin<T>(a: T, b: T, lt: (a: T, b: T) => boolean, drop?: (value: T) => void): T {
	if (lt(b, a)) {
		drop?.(a);
		return b;
	}
	drop?.(b);
	return a;
}

/** `Ordering::reverse`. */
export function orderingReverse(o: Ordering): Ordering {
	return o === 0 ? 0 : o === 1 ? -1 : 1;
}

/** `Ordering::then`, and with `lazy`, `then_with`, whose `other` is a function. */
export function orderingThen(o: Ordering, other: Ordering | (() => Ordering), lazy = false): Ordering {
	if (o !== 0) return o;
	return lazy ? (other as () => Ordering)() : (other as Ordering);
}

// Equality

/** `PartialEq::eq` of two values JS compares as Rust does: numbers, strings, booleans. */
export function same<T>(a: T, b: T): boolean {
	return a === b;
}

/** `PartialEq::eq` of text of any form, a `str`, a `String`, or a `Cow<str>`: their text. */
export function eqStr(a: string | RsString | Borrowed<string>, b: string | RsString | Borrowed<string>): boolean {
	return textOf(a) === textOf(b);
}

/** The text of a `str`, a `String`, or a `Cow<str>`. */
function textOf(s: string | RsString | Borrowed<string>): string {
	return typeof s === "string" ? s : s instanceof Borrowed ? s.slice : s.v;
}

/**
 * How a hash collection finds a key that is a `String` or a `Cow<str>`, or a `str` looking
 * one up: by its text, which the engine hashes as a JS map key, so the keys filed under
 * one are equal.
 */
export const HASH_TEXT: Hashing<string | RsString | Borrowed<string>> = { hash: textOf, eq: () => true, all: true };

/** `PartialEq::eq` of tuples, from each field's. */
export function eqTuple<T extends readonly unknown[]>(fields: readonly ((a: never, b: never) => boolean)[]): (a: T, b: T) => boolean {
	return (a, b) => {
		for (let i = 0; i < fields.length; i++) if (!(fields[i] as (a: unknown, b: unknown) => boolean)(a[i], b[i])) return false;
		return true;
	};
}

/** `PartialEq::eq` of slices: their lengths', then their elements' in order. */
export function eqSlice<T>(eq: (a: T, b: T) => boolean): (x: Elements<T>, y: Elements<T>) => boolean {
	return (x, y) => {
		const [a, i, m, b, j, n] = [sliceData(x), sliceStart(x), sliceLength(x), sliceData(y), sliceStart(y), sliceLength(y)];
		if (m !== n) return false;
		for (let k = 0; k < m; k++) if (!eq(a[i + k], b[j + k])) return false;
		return true;
	};
}

/** A slice, an array, or a `Vec`, of bytes in any form, or a `VecDeque`. */
type Seq<T> = Elements<T> | ByteSlice | ByteVec | VecDeque<T>;

/** The length of `s` and what reads its element at an index. */
function seqItems<T>(s: Seq<T>): [number, (i: number) => T] {
	if (s instanceof ByteSlice) return [s.length, (i) => s.data[s.start + i] as T];
	if (s instanceof Slice) return [s.length, (i) => s.data[s.start + i]];
	if (s instanceof ByteVec) return [s.length, (i) => s.data[i] as T];
	if (s instanceof Vec) return [s.items.length, (i) => s.items[i] as T];
	if (s instanceof ArraySlice) {
		const items = sliceData(s) as ArrayLike<T>;
		return [items.length, (i) => items[i]];
	}
	if (!("length" in s)) return [s.n, (i) => s.b[phys(s, i)] as T];
	return [s.length, (i) => s[i] as T];
}

/**
 * `PartialEq::eq` of sequences of any form against each other, as std's of slices, arrays,
 * `Vec`s, and `VecDeque`s are: their lengths, then each pair in order by `eq`.
 */
export function eqSeq<T, U>(eq: (a: T, b: U) => boolean): (a: Seq<T>, b: Seq<U>) => boolean {
	return (a, b) => {
		const [n, x] = seqItems(a);
		const [m, y] = seqItems(b);
		if (n !== m) return false;
		for (let i = 0; i < n; i++) if (!eq(x(i), y(i))) return false;
		return true;
	};
}

/** `PartialEq::eq` of `Option`s, from their values'. */
export function eqOption<T>(eq: (a: T, b: T) => boolean): (a: T | undefined, b: T | undefined) => boolean;
export function eqOption<T>(eq: (a: T, b: T) => boolean, none: None): (a: T | None, b: T | None) => boolean;
export function eqOption<T>(
	eq: (a: T, b: T) => boolean,
	none?: None,
): (a: T | undefined | None, b: T | undefined | None) => boolean {
	return (a, b) => (a === none || b === none ? a === b : eq(a as T, b as T));
}

/**
 * `PartialEq::eq` of `Arc`s: their values', after, with `ident`, whether they are one
 * `Arc`, which std checks first of a value that is `Eq`.
 */
export function eqArc<T>(eq: (a: T, b: T) => boolean, ident = false): (a: Arc<T>, b: Arc<T>) => boolean {
	return (a, b) => (ident && a === b) || eq(a.value, b.value);
}

/** A value as its own clone. */
export function itself<T>(value: T): T {
	return value;
}

/** The place of element `index` of `array`, which a raw pointer into it is too. */
export class AtIndex<T> implements Place<T> {
	readonly array: { [i: number]: T };
	readonly index: number;

	constructor(array: { [i: number]: T }, index: number) {
		this.array = array;
		this.index = index;
	}

	get v(): T {
		if (this.index >= 0 && this.index < (this.array as ArrayLike<T>).length) return this.array[this.index];
		return this.room(1).get(this.index);
	}

	set v(value: T) {
		if (this.index >= 0 && this.index < (this.array as ArrayLike<T>).length) this.array[this.index] = value;
		else this.room(1).set(this.index, value);
	}

	/** The place of element `index` of the same array, as pointer arithmetic moves to it. */
	at(index: number): AtIndex<T> {
		return new AtIndex(this.array, index);
	}

	/**
	 * Fails where `n` elements from here run past either end of the array, which JS
	 * doesn't keep Rust's memory around, but into the room of the `Vec` it is the items
	 * of: a shared subslice is a copy, which a pointer moved past it would read nothing of.
	 */
	reach(n: number): void {
		if (this.index >= 0 && this.index + n <= (this.array as ArrayLike<T>).length) return;
		this.room(n);
	}

	/** The room past the array that holds `n` elements from here, which `reach` requires. */
	private room(n: number): Room<T> {
		const room = this.index >= 0 ? roomPast<T>(this.array, this.index + n) : undefined;
		if (room === undefined && this.array === NO_MEMORY) nowhere();
		if (room === undefined) throw new Error("rust2-ts: a read or write through a pointer past the array JS keeps it in");
		return room;
	}
}

/** The place of `k`'s value in `m`, which holds it. */
class AtKey<K, V> implements Place<V> {
	private readonly m: Map<K, V>;
	private readonly k: K;

	constructor(m: Map<K, V>, k: K) {
		this.m = m;
		this.k = k;
	}

	get v(): V {
		return this.m.get(this.k) as V;
	}

	set v(value: V) {
		this.m.set(this.k, value);
	}
}

// HashMap and HashSet

/** An entry whose key is a JS object, in the chain of its hash. */
interface Slot<K, V> {
	k: K;
	v: V;
	next: Slot<K, V> | undefined;
}

/**
 * `HashMap<K, V>`, and `HashSet<K>` as a map to `undefined`, std's or hashbrown's. A key
 * that is a JS primitive, a number, string, or boolean, is its own JS map key, as Rust
 * compares it; a key that is an object goes by its hash, to a chain of the keys `Eq`
 * tells apart, or by its text where it is a `String` or a `Cow<str>`, as `HASH_TEXT` finds one. Iteration visits the first kind in insertion order and then the second by
 * hash, an order Rust leaves unspecified.
 */
export interface HashMap<K, V> {
	p: Map<K, V>;
	o: Map<HashKey, Slot<K, V>> | undefined;
	/** How many keys `o` holds. */
	n: number;
	/** How to find a key that is an object, or `undefined` if no key is one. */
	readonly h: Hashing<K> | undefined;
	/**
	 * Its own `BuildHasher`, which `hasher` lends, or `undefined` for a `RandomState`. No
	 * key goes through it: `h` finds them by the runtime's own hasher.
	 */
	readonly s: unknown;
	/** The capacity an operation last set it to, if any, which `hashCapacity` reads. */
	cap?: number;
}

/** `HashSet<K>`. */
export type HashSet<K> = HashMap<K, undefined>;

/**
 * `HashMap::new` and `HashSet::new`, with how to find a key that is an object, and
 * `with_hasher` of its own `BuildHasher` `s`. The type `K` is asserted, not inferred from
 * `h`: a slice's hashing takes any `ArrayLike`.
 */
export function hashNew<K, V>(h?: Hashing<NoInfer<K>>, s?: unknown): HashMap<K, V> {
	return { p: new Map(), o: undefined, n: 0, h, s };
}

/** Whether `key` goes by its hash rather than as its own JS map key. */
function hashed<K, V>(m: HashMap<K, V>, key: K): boolean {
	return m.h !== undefined && (m.h.all || typeof key === "object");
}

/**
 * The slot holding a key equal to `key`, a hashed key whose hash is `hash` if given, which
 * compares by `key`'s `eq`, as std's lookups do.
 */
function slot<K, V>(m: HashMap<K, V>, key: K, hash?: HashKey): Slot<K, V> | undefined {
	const h = m.h as Hashing<K>;
	for (let s = m.o?.get(hash ?? h.hash(key)); s !== undefined; s = s.next) if (h.eq(key, s.k)) return s;
	return undefined;
}

export function hashLen<K, V>(m: HashMap<K, V>): number {
	return m.p.size + m.n;
}

// Capacity. std guarantees a hash collection's only past `capacity() >= len` and what
// `with_capacity`, `reserve`, and `shrink_to` ask for, so a map holds one, as `cap`, only
// where an operation sets it, and an insert never reads or writes it. Reading one doubles
// it while short of the length, as inserts grow std's, and a removal keeps what it read,
// as a table keeps its allocation.

/** The least capacity a map holds once it holds anything. */
const HASH_MIN = 3;

/** `capacity` of a hash collection. */
export function hashCapacity<K, V>(m: HashMap<K, V>): number {
	const n = hashLen(m);
	let c = m.cap ?? 0;
	while (c < n) c = Math.max(c * 2, HASH_MIN);
	return c;
}

/** Keep the capacity `m` reads before it holds fewer. */
function hold<K, V>(m: HashMap<K, V>): void {
	if ((m.cap ?? 0) < hashLen(m)) m.cap = hashCapacity(m);
}

/** `with_capacity`, of at most `max` entries, past which it panics, as hashbrown's does. */
export function hashWithCapacity<K, V>(n: number, max: number, h?: Hashing<NoInfer<K>>, s?: unknown): HashMap<K, V> {
	if (n > max) panic("Hash table capacity overflow", "hash_capacity");
	const m = hashNew<K, V>(h, s);
	if (n > 0) m.cap = n;
	return m;
}

/**
 * `reserve`, of room for `additional` more, at least doubling, which panics past `max`
 * entries. A `HashTable`'s takes its hasher, which it never calls, as it moves no item.
 */
export function hashReserve<K, V>(m: HashMap<K, V>, additional: number, max: number, _hasher?: (item: V) => u64): void {
	if (!hashTryReserve(m, additional, max).ok) panic("Hash table capacity overflow", "hash_capacity");
}

/** std's `try_reserve`: `reserve`'s room, or the error where it panics. */
export function hashTryReserve<K, V>(m: HashMap<K, V>, additional: number, max: number): Result<undefined, TryReserveError> {
	const n = hashLen(m);
	const cap = hashCapacity(m);
	if (additional <= cap - n) return OK;
	if (n + additional > max) return CAPACITY_OVERFLOW;
	m.cap = Math.min(Math.max(cap * 2, n + additional), max);
	return OK;
}

/** `shrink_to`, and `shrink_to_fit` of 0: room for no fewer than `min` and what it holds. A table's hasher goes uncalled. */
export function hashShrinkTo<K, V>(m: HashMap<K, V>, min: number, _hasher?: (item: V) => u64): void {
	const target = Math.max(hashLen(m), min);
	if (target < hashCapacity(m)) m.cap = target;
}

/** hashbrown's `allocation_size` of entries `size` bytes each: a slot and a control byte for each it has room for. */
export function hashAllocationSize<K, V>(m: HashMap<K, V>, size: number): number {
	return hashCapacity(m) * (size + 1);
}

/** `HashMap::get`: the value there, or else `none`, the `None` of an `Option` of a value that can be `undefined`. */
export function hashGet<K, V>(m: HashMap<K, V>, key: K): V | undefined;
export function hashGet<K, V>(m: HashMap<K, V>, key: K, none: None): V | None;
export function hashGet<K, V>(m: HashMap<K, V>, key: K, none?: None): V | None | undefined {
	if (hashed(m, key)) {
		const s = slot(m, key);
		return s === undefined ? none : s.v;
	}
	const v = m.p.get(key);
	return v !== undefined || none === undefined || m.p.has(key) ? v : none;
}

/** `HashMap::get_key_value`: the equal key the map holds, and its value. */
export function hashGetKeyValue<K, V>(m: HashMap<K, V>, key: K): [K, V] | undefined {
	if (hashed(m, key)) {
		const s = slot(m, key);
		return s === undefined ? undefined : [s.k, s.v];
	}
	return m.p.has(key) ? [key, m.p.get(key) as V] : undefined;
}

/** `HashMap::get_mut` of a value JS copies: its place, if the map holds `key`. */
export function hashGetMut<K, V>(m: HashMap<K, V>, key: K): Place<V> | undefined {
	if (hashed(m, key)) return slot(m, key);
	return m.p.has(key) ? new AtKey(m.p, key) : undefined;
}

export function hashContains<K, V>(m: HashMap<K, V>, key: K): boolean {
	return hashed(m, key) ? slot(m, key) !== undefined : m.p.has(key);
}

/** `HashMap::index`: `map[&key]`. */
export function hashIndex<K, V>(m: HashMap<K, V>, key: K, at?: Location): V {
	if (!hashContains(m, key)) panic("no entry found for key", at, true);
	return hashGet(m, key) as V;
}

/** Drop `key` by `k` and then `value` by `v`. */
function dropPair<K, V>(key: K, value: V, k?: (key: K) => void, v?: (value: V) => void): void {
	k?.(key);
	v?.(value);
}

/** Drop each key by `k` and its value by `v`, in order. */
function dropPairs<K, V>(keys: readonly K[], values: readonly V[], k?: (key: K) => void, v?: (value: V) => void): void {
	for (let i = 0; i < keys.length; i++) dropPair(keys[i], values[i], k, v);
}

/** `HashMap::insert`: the old value, or else `none`. An equal key already there stays, and `key` drops. */
export function hashInsert<K, V>(m: HashMap<K, V>, key: K, value: V, k?: (key: K) => void): V | undefined;
export function hashInsert<K, V>(m: HashMap<K, V>, key: K, value: V, k: ((key: K) => void) | undefined, none: None): V | None;
export function hashInsert<K, V>(m: HashMap<K, V>, key: K, value: V, k?: (key: K) => void, none?: None): V | None | undefined {
	if (!hashed(m, key)) {
		const had = m.p.has(key);
		const old = m.p.get(key);
		m.p.set(key, value);
		if (!had) return none;
		k?.(key);
		return old;
	}
	// Hashed once, as std's is, into an empty map too.
	const hash = (m.h as Hashing<K>).hash(key);
	const found = slot(m, key, hash);
	if (found !== undefined) {
		const old = found.v;
		found.v = value;
		k?.(key);
		return old;
	}
	add(m, key, value, hash);
	return none;
}

/** Add a hashed key that is not there yet, whose hash is `hash`: its new slot. */
function add<K, V>(m: HashMap<K, V>, key: K, value: V, hash = (m.h as Hashing<K>).hash(key)): Slot<K, V> {
	m.o ??= new Map();
	const s = { k: key, v: value, next: m.o.get(hash) };
	m.o.set(hash, s);
	m.n++;
	return s;
}

/** `HashSet::replace`: the equal key the set held, which `key` takes the place of, or else `none`, with `key` added. */
export function hashReplace<K>(m: HashSet<K>, key: K): K | undefined;
export function hashReplace<K>(m: HashSet<K>, key: K, none: None): K | None;
export function hashReplace<K>(m: HashSet<K>, key: K, none?: None): K | None | undefined {
	if (!hashed(m, key)) {
		if (m.p.has(key)) return key;
		m.p.set(key, undefined);
		return none;
	}
	const hash = (m.h as Hashing<K>).hash(key);
	const s = slot(m, key, hash);
	if (s === undefined) {
		add(m, key, undefined, hash);
		return none;
	}
	const old = s.k;
	s.k = key;
	return old;
}

/** `HashSet::insert`: whether `key` was not there yet. An equal key already there stays, and `key` drops. */
export function hashAdd<K>(m: HashSet<K>, key: K, k?: (key: K) => void): boolean {
	if (!hashed(m, key)) {
		if (m.p.has(key)) {
			k?.(key);
			return false;
		}
		m.p.set(key, undefined);
		return true;
	}
	const hash = (m.h as Hashing<K>).hash(key);
	if (slot(m, key, hash) !== undefined) {
		k?.(key);
		return false;
	}
	add(m, key, undefined, hash);
	return true;
}

/** `HashMap::remove_entry`: the key and value there, if any. */
export function hashRemoveEntry<K, V>(m: HashMap<K, V>, key: K): [K, V] | undefined {
	hold(m);
	if (!hashed(m, key)) {
		if (!m.p.has(key)) return undefined;
		const value = m.p.get(key) as V;
		m.p.delete(key);
		return [key, value];
	}
	const hash = (m.h as Hashing<K>).hash(key);
	const s = slot(m, key, hash);
	if (s === undefined) return undefined;
	unlink(m, hash, s);
	return [s.k, s.v];
}

/** Take `s`, a slot whose key's hash is `hash`, out of `m`. */
function unlink<K, V>(m: HashMap<K, V>, hash: HashKey, s: Slot<K, V>): void {
	const o = m.o as Map<HashKey, Slot<K, V>>;
	let prev = o.get(hash) as Slot<K, V>;
	if (prev === s) {
		if (s.next !== undefined) o.set(hash, s.next);
		else o.delete(hash);
	} else {
		while (prev.next !== s) prev = prev.next as Slot<K, V>;
		prev.next = s.next;
	}
	m.n--;
}

/** `HashMap::remove`: the value there, whose key drops, or else `none`. */
export function hashRemove<K, V>(m: HashMap<K, V>, key: K, k?: (key: K) => void): V | undefined;
export function hashRemove<K, V>(m: HashMap<K, V>, key: K, k: ((key: K) => void) | undefined, none: None): V | None;
export function hashRemove<K, V>(m: HashMap<K, V>, key: K, k?: (key: K) => void, none?: None): V | None | undefined {
	if (!hashed(m, key)) {
		hold(m);
		if (!m.p.has(key)) return none;
		const old = m.p.get(key);
		m.p.delete(key);
		k?.(key);
		return old;
	}
	const entry = hashRemoveEntry(m, key);
	if (entry === undefined) return none;
	k?.(entry[0]);
	return entry[1];
}

/** `HashSet::remove`: whether `key` was there. The key it held drops. */
export function hashDelete<K>(m: HashSet<K>, key: K, k?: (key: K) => void): boolean {
	if (!hashed(m, key)) {
		hold(m);
		if (!m.p.delete(key)) return false;
		k?.(key);
		return true;
	}
	const entry = hashRemoveEntry(m, key);
	if (entry !== undefined) k?.(entry[0]);
	return entry !== undefined;
}

/** `HashSet::get`, and `HashSet::take` if `take`: the equal key the set holds, or else `none`. */
export function hashKey<K>(m: HashSet<K>, key: K, take?: boolean): K | undefined;
export function hashKey<K>(m: HashSet<K>, key: K, take: boolean, none: None): K | None;
export function hashKey<K>(m: HashSet<K>, key: K, take = false, none?: None): K | None | undefined {
	if (take) hold(m);
	if (hashed(m, key) && take) {
		const entry = hashRemoveEntry(m, key);
		return entry === undefined ? none : entry[0];
	}
	if (hashed(m, key)) {
		const s = slot(m, key);
		return s === undefined ? none : s.k;
	}
	if (!m.p.has(key)) return none;
	if (take) m.p.delete(key);
	return key;
}

/**
 * Drop `m`: its own `BuildHasher` by `s`, as hashbrown's field before its table, then each
 * key by `k` and each value by `v`, of those that drop, entry by entry.
 */
export function hashDrop<K, V>(m: HashMap<K, V>, k: ((key: K) => void) | undefined, v?: (value: V) => void, s?: (state: never) => void): void {
	s?.(m.s as never);
	for (const [key, value] of m.p) dropPair(key, value, k, v);
	for (const first of m.o?.values() ?? []) {
		for (let s: Slot<K, V> | undefined = first; s !== undefined; s = s.next) dropPair(s.k, s.v, k, v);
	}
}

/** `clear`, which drops each entry and leaves the map empty. */
export function hashClear<K, V>(m: HashMap<K, V>, k?: (key: K) => void, v?: (value: V) => void): void {
	hold(m);
	if (k !== undefined || v !== undefined) hashDrop(m, k, v);
	m.p.clear();
	m.o = undefined;
	m.n = 0;
}

/** `retain`, which keeps each entry `f` returns true for. Each entry goes, and drops, as `f` rejects it. */
export function hashRetain<K, V>(m: HashMap<K, V>, f: (key: K, value: V) => boolean, k?: (key: K) => void, v?: (value: V) => void): void {
	retainHash(m, f as (key: K, value: V | Place<V>) => boolean, false, k, v);
}

/** `retain` of values JS copies: `f` takes the place of each. */
export function hashRetainMut<K, V>(m: HashMap<K, V>, f: (key: K, value: Place<V>) => boolean, k?: (key: K) => void, v?: (value: V) => void): void {
	retainHash(m, f as (key: K, value: V | Place<V>) => boolean, true, k, v);
}

/** `retain`, handing `f` each value or, if `lend`, its place. */
function retainHash<K, V>(m: HashMap<K, V>, f: (key: K, value: V | Place<V>) => boolean, lend: boolean, k?: (key: K) => void, v?: (value: V) => void): void {
	hold(m);
	for (const [key, value] of m.p) {
		if (f(key, lend ? new AtKey(m.p, key) : value)) continue;
		const now = lend ? (m.p.get(key) as V) : value;
		m.p.delete(key);
		dropPair(key, now, k, v);
	}
	if (m.o === undefined) return;
	for (const [hash, first] of m.o) {
		let prev: Slot<K, V> | undefined;
		for (let s: Slot<K, V> | undefined = first; s !== undefined; s = s.next) {
			if (f(s.k, lend ? s : s.v)) {
				prev = s;
				continue;
			}
			m.n--;
			if (prev !== undefined) prev.next = s.next;
			else if (s.next !== undefined) m.o.set(hash, s.next);
			else m.o.delete(hash);
			dropPair(s.k, s.v, k, v);
		}
	}
}

/**
 * `extract_if`: each entry `f` returns true for, taken out as it is read, in `hashIter`'s
 * order, as `[key, value]`, or of a set its key. `f` is given each key and value or, with
 * `places`, the value's place, and drops with the iterator; what it has not read stays.
 */
class HashExtractIf<K, V, T> extends Iter<T> implements Disposable {
	private readonly m: HashMap<K, V>;
	private readonly p: Iterator<[K, V]>;
	private readonly o: Iterator<[HashKey, Slot<K, V>]> | undefined;
	/** The chain it reads, by its hash, the slot it reads next, and the last it kept there. */
	private hash: HashKey = 0;
	private at: Slot<K, V> | undefined;
	private kept: Slot<K, V> | undefined;
	private readonly f: (key: K, value: V | Place<V>) => boolean;
	private readonly places: boolean;
	private readonly keys: boolean;
	/** How many it has left to read. */
	private left: number;

	constructor(m: HashMap<K, V>, f: (key: K, value: V | Place<V>) => boolean, places: boolean, keys: boolean) {
		super();
		hold(m);
		this.m = m;
		this.p = m.p.entries();
		this.o = m.o?.entries();
		this.f = f;
		this.places = places;
		this.keys = keys;
		this.left = hashLen(m);
	}

	next(): T | Done {
		const m = this.m;
		for (; this.left > 0; this.left--) {
			const entry = this.p.next();
			if (!entry.done) {
				const [key, value] = entry.value;
				if (!this.f(key, this.places ? new AtKey(m.p, key) : value)) continue;
				const now = this.places ? (m.p.get(key) as V) : value;
				m.p.delete(key);
				this.left--;
				return (this.keys ? key : [key, now]) as T;
			}
			if (this.at === undefined) {
				[this.hash, this.at] = (this.o as Iterator<[HashKey, Slot<K, V>]>).next().value as [HashKey, Slot<K, V>];
				this.kept = undefined;
			}
			const s = this.at;
			this.at = s.next;
			if (!this.f(s.k, this.places ? s : s.v)) {
				this.kept = s;
				continue;
			}
			const o = m.o as Map<HashKey, Slot<K, V>>;
			if (this.kept !== undefined) this.kept.next = s.next;
			else if (s.next !== undefined) o.set(this.hash, s.next);
			else o.delete(this.hash);
			m.n--;
			this.left--;
			return (this.keys ? s.k : [s.k, s.v]) as T;
		}
		return DONE;
	}

	override sizeHint(): SizeHint {
		return [0, this.left];
	}

	[Symbol.dispose](): void {
		dispose(this.f);
	}
}

/** `extract_if` of a map, handing `f` each key and value. */
export function hashExtractIf<K, V>(m: HashMap<K, V>, f: (key: K, value: V) => boolean): Iter<[K, V]> {
	return new HashExtractIf(m, f as (key: K, value: V | Place<V>) => boolean, false, false);
}

/** `extract_if` of a map of values JS copies: `f` takes the place of each. */
export function hashExtractIfMut<K, V>(m: HashMap<K, V>, f: (key: K, value: Place<V>) => boolean): Iter<[K, V]> {
	return new HashExtractIf(m, f as (key: K, value: V | Place<V>) => boolean, true, false);
}

/** `extract_if` of a set, handing `f` each key. */
export function hashExtractKeysIf<K>(m: HashSet<K>, f: (key: K) => boolean): Iter<K> {
	return new HashExtractIf(m, f as (key: K) => boolean, false, true);
}

/**
 * What a collection's iterator yields: its entries as `[key, value]`, keys, values,
 * `[key, place]`, the place of a value JS copies, or that place alone.
 */
type Pick = 0 | 1 | 2 | 3 | 4;
const ENTRIES = 0;
const KEYS = 1;
const VALUES = 2;
const PLACES = 3;
const VALUE_PLACES = 4;

/** What `pick` yields by value: each place its value. */
function byValue(pick: Pick): Pick {
	return pick === PLACES ? ENTRIES : pick === VALUE_PLACES ? VALUES : pick;
}

/**
 * An iterator of a hash collection: `iter`, `keys`, `values`, `drain`, and the rest.
 * One that owns its entries is given what drops each key and value of those that drop,
 * and drops each entry it still holds as it drops.
 */
export class HashIter<K, V, T> extends Iter<T> implements Disposable {
	private readonly h: HashMap<K, V>;
	private readonly m: Map<K, V>;
	private readonly p: Iterator<[K, V]>;
	private readonly o: Iterator<Slot<K, V>> | undefined;
	private at: Slot<K, V> | undefined;
	private readonly pick: Pick;
	left: number;
	private readonly k: ((key: K) => void) | undefined;
	private readonly v: ((value: V) => void) | undefined;
	/** Whether it is a `drain`, whose `count` reads each entry, as an `into_iter`'s does not. */
	readonly drain: boolean;

	constructor(m: HashMap<K, V>, pick: Pick, k?: (key: K) => void, v?: (value: V) => void, drain = false) {
		super();
		this.h = m;
		this.m = m.p;
		this.p = m.p.entries();
		this.o = m.o?.values();
		this.pick = pick;
		this.left = hashLen(m);
		this.k = k;
		this.v = v;
		this.drain = drain;
	}

	/** The next entry, of the `left` it has. */
	private entry(): [K, V] {
		const entry = this.p.next();
		if (!entry.done) return entry.value;
		const s = this.slot();
		return [s.k, s.v];
	}

	/** The next entry of the `left` it has, as its key and the place of its value. */
	private place(): [K, Place<V>] {
		const entry = this.p.next();
		if (!entry.done) return [entry.value[0], new AtKey(this.m, entry.value[0])];
		const s = this.slot();
		return [s.k, s];
	}

	/** The next chained entry, once the primitive keys are done. */
	private slot(): Slot<K, V> {
		if (this.at === undefined) this.at = (this.o as Iterator<Slot<K, V>>).next().value;
		const s = this.at as Slot<K, V>;
		this.at = s.next;
		return s;
	}

	/**
	 * The next item, `into_keys`' and `into_values`' with the other half dropped before it
	 * is returned, as std's `next` drops it.
	 */
	next(): T | Done {
		if (this.left === 0) return DONE;
		this.left--;
		if (this.pick === PLACES) return this.place() as T;
		if (this.pick === VALUE_PLACES) return this.place()[1] as T;
		const entry = this.entry();
		if (this.pick === ENTRIES) return entry as T;
		if (this.pick === KEYS) {
			this.v?.(entry[1]);
			return entry[0] as unknown as T;
		}
		this.k?.(entry[0]);
		return entry[1] as unknown as T;
	}

	/** std's `fold` of `into_keys` and `into_values`: the other half drops after `f` has its item. */
	override fold<A>(acc: A, f: (acc: A, item: T) => A, release?: () => void): A {
		if (this.pick !== KEYS && this.pick !== VALUES) return super.fold(acc, f, release);
		const keys = this.pick === KEYS;
		while (this.left > 0) {
			this.left--;
			const [key, value] = this.entry();
			if (keys) {
				acc = f(acc, key as unknown as T);
				this.v?.(value);
			} else {
				acc = f(acc, value as unknown as T);
				this.k?.(key);
			}
		}
		release?.();
		return acc;
	}

	override remaining(): Iterable<unknown> {
		return this.from(byValue(this.pick));
	}

	/** std's, of an iterator that borrows the map: where it is, independently of it. */
	override clone(): Iter<T> {
		return this.from(this.pick);
	}

	/**
	 * A fresh iterator of the same map yielding `pick`, past what this one took: the map is
	 * unchanged while it is held, and JS can't copy where its own iterator is.
	 */
	private from<U>(pick: Pick): HashIter<K, V, U> {
		const it = new HashIter<K, V, U>(this.h, pick);
		while (it.left > this.left) it.next();
		return it;
	}

	override len(): number {
		return this.left;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	[Symbol.dispose](): void {
		if (this.k === undefined && this.v === undefined) return;
		const n = this.left;
		this.left = 0;
		for (let i = 0; i < n; i++) {
			const [key, value] = this.entry();
			dropPair(key, value, this.k, this.v);
		}
	}
}

/**
 * `iter` and `iter_mut` of a map, and with what drops each key and value of those that
 * drop, `into_iter`: its entries, as `[key, value]`. Each consuming one drops the map's own
 * `BuildHasher` by `s` at once, as hashbrown's does.
 */
export function hashIter<K, V>(m: HashMap<K, V>, k?: (key: K) => void, v?: (value: V) => void, s?: (state: never) => void): Iter<[K, V]> {
	s?.(m.s as never);
	return new HashIter<K, V, [K, V]>(m, ENTRIES, k, v);
}

/** `keys` of a map and `iter` of a set, and with what drops each key and value, `into_keys` and `into_iter`. */
export function hashKeys<K, V>(m: HashMap<K, V>, k?: (key: K) => void, v?: (value: V) => void, s?: (state: never) => void): Iter<K> {
	s?.(m.s as never);
	return new HashIter<K, V, K>(m, KEYS, k, v);
}

/** `values` and `values_mut`, and with what drops each value and key, `into_values`. */
export function hashValues<K, V>(m: HashMap<K, V>, v?: (value: V) => void, k?: (key: K) => void, s?: (state: never) => void): Iter<V> {
	s?.(m.s as never);
	return new HashIter<K, V, V>(m, VALUES, k, v);
}

/** `iter_mut` of values JS copies: each key and the place of its value, in `hashIter`'s order. */
export function hashIterMut<K, V>(m: HashMap<K, V>): Iter<[K, Place<V>]> {
	return new HashIter<K, V, [K, Place<V>]>(m, PLACES);
}

/** `values_mut` of values JS copies: the place of each. */
export function hashValuesMut<K, V>(m: HashMap<K, V>): Iter<Place<V>> {
	return new HashIter<K, V, Place<V>>(m, VALUE_PLACES);
}

/** `drain` of a map: every entry, taken out at once, which drops as `hashIter`'s. */
export function hashDrain<K, V>(m: HashMap<K, V>, k?: (key: K) => void, v?: (value: V) => void): Iter<[K, V]> {
	return new HashIter<K, V, [K, V]>(taken(m), ENTRIES, k, v, true);
}

/** `drain` of a set: every key, taken out at once. */
export function hashDrainKeys<K, V>(m: HashMap<K, V>, k?: (key: K) => void): Iter<K> {
	return new HashIter<K, V, K>(taken(m), KEYS, k, undefined, true);
}

/** What `m` held, which it no longer does, with the room it had. */
function taken<K, V>(m: HashMap<K, V>): HashMap<K, V> {
	hold(m);
	const out: HashMap<K, V> = { p: m.p, o: m.o, n: m.n, h: m.h, s: undefined };
	m.p = new Map();
	hashClear(m);
	return out;
}

/** Extend `c` by `add` of each of `items`, taken as `take` says, as std's `extend` takes the iterator. */
function extending<C, T>(c: C, items: Iter<T>, add: (item: T) => void, take?: Take): C {
	if (take === "fold") iterForEach(items, add, true);
	else {
		// By `next` where std's reads by it, and else as its `for_each` folds.
		const read = take === "next" ? items : items.folded(true);
		for (let item = read.next(); item !== DONE; item = read.next()) add(item);
	}
	if (take === "next") dispose(items);
	return c;
}

/**
 * `extending` a `Vec` holding `len`, which with elements `size` bytes each reserves as
 * `iterExtend` does, into the capacity `held` holds: a `trusted` iterator up front, and any
 * other by `next`.
 */
function vecExtending<C, T>(c: C, len: number, items: Iter<T>, add: (item: T) => void, take?: Take, size?: number, trusted = false, held: Vec<unknown> = new Vec()): C {
	if (size === undefined) return extending(c, items, add, take);
	let cap = vecCapacity(held, size);
	if (trusted) {
		vecHold(held, capReserve(len, cap, trustedUpper(items), size));
		return extending(c, items, add, take);
	}
	for (let item = items.next(); item !== DONE; item = items.next()) {
		if (len === cap) cap = capNext(items, len, cap, size);
		add(item);
		len++;
	}
	vecHold(held, cap);
	if (take !== undefined) dispose(items);
	return c;
}

/**
 * hashbrown's `extend_reserve` of `m` for `additional` more, as its `extend` makes for an
 * iterator's lower bound: all of it, or half rounded up if `m` holds any, which panics past
 * `max` entries, the most its table holds.
 */
function extendReserve<K, V>(m: HashMap<K, V>, additional: number, max: number | undefined): void {
	if (max === undefined) return;
	const n = hashLen(m);
	if (n + (n === 0 ? additional : Math.ceil(additional / 2)) > max) panic("Hash table capacity overflow", "hash_capacity");
}

/**
 * `extend` of a map with `[key, value]`s, and `collect` into one through `hashNew`, which
 * reserves first as `extendReserve` does. Of equal keys, the first stays and each later one
 * drops, then the value it replaced.
 */
export function hashExtend<K, V>(m: HashMap<K, V>, items: Iter<[K, V]>, k?: (key: K) => void, v?: (value: V) => void, take?: Take, max?: number): HashMap<K, V> {
	extendReserve(m, items.sizeHint()[0], max);
	return hashFill(m, items, k, v, take);
}

/**
 * hashbrown's `from_iter` of a map, as `hashExtend` into what `make` makes, but which reads
 * `items`' size hint first, as hashbrown's does before it makes its `BuildHasher`.
 */
export function hashFromIter<K, V>(
	items: Iter<[K, V]>,
	make: () => HashMap<K, V>,
	k?: (key: K) => void,
	v?: (value: V) => void,
	take?: Take,
	max?: number): HashMap<K, V> {
	const n = items.sizeHint()[0];
	const m = make();
	extendReserve(m, n, max);
	return hashFill(m, items, k, v, take);
}

/** Insert each of `items` into `m`, as `hashExtend` does once it has reserved. */
function hashFill<K, V>(m: HashMap<K, V>, items: Iter<[K, V]>, k?: (key: K) => void, v?: (value: V) => void, take?: Take): HashMap<K, V> {
	const add = (item: [K, V]) => {
		const old = hashInsert(m, item[0], item[1], k);
		if (old !== undefined) v?.(old);
	};
	return extending(m, items, add, take);
}

/** `extend` of a set with keys, and `collect` into one, as `hashExtend`: a key already there drops. */
export function hashExtendKeys<K>(m: HashSet<K>, keys: Iter<K>, k?: (key: K) => void, take?: Take, max?: number): HashSet<K> {
	extendReserve(m, keys.sizeHint()[0], max);
	return extending(m, keys, (key) => hashAdd(m, key, k), take);
}

/**
 * `clone`, with its own `BuildHasher` through `state`, first, as hashbrown's, then each key
 * and value through its `Clone`, or each as itself without one.
 */
export function hashClone<K, V>(
	m: HashMap<K, V>,
	key: (k: K) => K = itself,
	value: (v: V) => V = itself,
	state?: (s: never) => unknown,
): HashMap<K, V> {
	const out = hashNew<K, V>(m.h, state === undefined ? m.s : state(m.s as never));
	for (const [k, v] of m.p) out.p.set(key(k), value(v));
	// Each clone keeps its original's hash, as hashbrown's copies its table without
	// rehashing, and a table has no hasher to rehash by.
	for (const [hash, first] of m.o ?? []) {
		for (let s: Slot<K, V> | undefined = first; s !== undefined; s = s.next) add(out, key(s.k), value(s.v), hash);
	}
	return out;
}

/**
 * `PartialEq::eq` of hash maps, by their values' `eq`, or of sets, without one: as std's,
 * equal lengths, then each entry of `a` found in `b`, by the key and then the value of
 * `a`'s `eq`. The runtime's order decides which entries it compares before one differs.
 */
export function hashEq<V>(eq?: (a: V, b: V) => boolean): <K>(a: HashMap<K, V>, b: HashMap<K, V>) => boolean {
	return <K>(a: HashMap<K, V>, b: HashMap<K, V>) => {
		if (hashLen(a) !== hashLen(b)) return false;
		for (const [key, value] of hashIter(a)) {
			const found = hashGetKeyValue(b, key);
			if (found === undefined || (eq !== undefined && !eq(value, found[1]))) return false;
		}
		return true;
	};
}

// Set operations of hash sets, as std's: each walks one set's keys in the runtime's
// order, which picks which set's key of two equal ones it yields.

/** `HashSet::union`: each key of the larger, `a` of two as large, then each of the other's that it lacks. */
export function hashUnion<K>(a: HashSet<K>, b: HashSet<K>): Iter<K> {
	const [large, small] = hashLen(a) >= hashLen(b) ? [a, b] : [b, a];
	return iterChain(hashKeys(large), hashDifference(small, large));
}

/** `HashSet::intersection`: each key of the smaller, `a` of two as large, that the other holds. */
export function hashIntersection<K>(a: HashSet<K>, b: HashSet<K>): Iter<K> {
	const [small, large] = hashLen(a) <= hashLen(b) ? [a, b] : [b, a];
	return iterFilter(hashKeys(small), (key) => hashContains(large, key));
}

/** `HashSet::difference`: each key of `a` that `b` lacks. */
export function hashDifference<K>(a: HashSet<K>, b: HashSet<K>): Iter<K> {
	return iterFilter(hashKeys(a), (key) => !hashContains(b, key));
}

/** `HashSet::symmetric_difference`: `a`'s difference from `b`, then `b`'s from `a`. */
export function hashSymmetricDifference<K>(a: HashSet<K>, b: HashSet<K>): Iter<K> {
	return iterChain(hashDifference(a, b), hashDifference(b, a));
}

/** `HashSet::is_subset`: whether `b` holds each key of `a`. */
export function hashIsSubset<K>(a: HashSet<K>, b: HashSet<K>): boolean {
	if (hashLen(a) > hashLen(b)) return false;
	for (const key of hashKeys(a)) if (!hashContains(b, key)) return false;
	return true;
}

/** `HashSet::is_superset`: whether `a` holds each key of `b`. */
export function hashIsSuperset<K>(a: HashSet<K>, b: HashSet<K>): boolean {
	return hashIsSubset(b, a);
}

/** `HashSet::is_disjoint`: whether the larger lacks each key of the smaller, `a` of two as large. */
export function hashIsDisjoint<K>(a: HashSet<K>, b: HashSet<K>): boolean {
	const [small, large] = hashLen(a) <= hashLen(b) ? [a, b] : [b, a];
	for (const key of hashKeys(small)) if (hashContains(large, key)) return false;
	return true;
}

// HashTable

/**
 * hashbrown's `HashTable<T>`: a map of nothing to each item, under the hash its caller
 * gives rather than one of its own, which the caller's equality tells apart. So the map's
 * functions read, iterate, drain, drop, and size it, of its values.
 */
export type HashTable<T> = HashMap<undefined, T>;

/** The JS map key of a hash: the number, or one past 2^53 its limbs make. */
function tableKey(hash: u64): number {
	return typeof hash === "number" ? hash : hash.hi * 0x1_0000_0000 + hash.lo;
}

/** The slot of `t` under `hash` whose item `eq` accepts. */
function tableSlot<T>(t: HashTable<T>, hash: number, eq: (item: T) => boolean): Slot<undefined, T> | undefined {
	for (let s = t.o?.get(hash); s !== undefined; s = s.next) if (eq(s.v)) return s;
	return undefined;
}

/** `HashTable::find`: the item under `hash` that `eq` accepts, or else `none`. */
export function tableFind<T>(t: HashTable<T>, hash: u64, eq: (item: T) => boolean): T | undefined;
export function tableFind<T>(t: HashTable<T>, hash: u64, eq: (item: T) => boolean, none: None): T | None;
export function tableFind<T>(t: HashTable<T>, hash: u64, eq: (item: T) => boolean, none?: None): T | None | undefined {
	const s = tableSlot(t, tableKey(hash), eq);
	return s === undefined ? none : s.v;
}

/** `HashTable::find_mut` of items JS copies: the place of the item `eq` accepts. */
export function tableFindMut<T>(t: HashTable<T>, hash: u64, eq: (item: T) => boolean): Place<T> | undefined {
	return tableSlot(t, tableKey(hash), eq);
}

/**
 * A `HashTable`'s `Entry`, `OccupiedEntry`, `VacantEntry`, or `AbsentEntry`: the table, the
 * hash, and the slot it found, which is the place of its item while it is occupied.
 */
export class TableEntry<T> implements Place<T> {
	readonly t: HashTable<T>;
	private readonly h: number;
	private s: Slot<undefined, T> | undefined;

	constructor(t: HashTable<T>, h: number, s: Slot<undefined, T> | undefined) {
		this.t = t;
		this.h = h;
		this.s = s;
	}

	get v(): T {
		return (this.s as Slot<undefined, T>).v;
	}

	set v(item: T) {
		(this.s as Slot<undefined, T>).v = item;
	}

	/** Whether it is occupied. */
	get o(): boolean {
		return this.s !== undefined;
	}

	/** Insert `item`, which it holds then. */
	fill(item: T): this {
		this.s = add(this.t, undefined, item, this.h);
		return this;
	}

	/** Take its item out of the table, which leaves it vacant. */
	take(): T {
		const s = this.s as Slot<undefined, T>;
		hold(this.t);
		unlink(this.t, this.h, s);
		this.s = undefined;
		return s.v;
	}
}

/** `HashTable::entry`: occupied by the item under `hash` that `eq` accepts, or else vacant. Its hasher goes uncalled. */
export function tableEntry<T>(t: HashTable<T>, hash: u64, eq: (item: T) => boolean, _hasher?: (item: T) => u64): TableEntry<T> {
	const h = tableKey(hash);
	return new TableEntry(t, h, tableSlot(t, h, eq));
}

/** `HashTable::find_entry`: `tableEntry`'s, as `Ok` if occupied and else as an `AbsentEntry` in `Err`. */
export function tableFindEntry<T>(t: HashTable<T>, hash: u64, eq: (item: T) => boolean): Result<TableEntry<T>, TableEntry<T>> {
	const e = tableEntry(t, hash, eq);
	return e.o ? Ok(e) : Err(e);
}

/** `HashTable::insert_unique`: the entry of `item`, inserted under `hash`. Its hasher goes uncalled. */
export function tableInsertUnique<T>(t: HashTable<T>, hash: u64, item: T, _hasher?: (item: T) => u64): TableEntry<T> {
	return new TableEntry(t, tableKey(hash), undefined).fill(item);
}

/** Whether `e` is `Entry::Occupied`. */
export function tableOccupied<T>(e: TableEntry<T>): boolean {
	return e.o;
}

/** `Entry::insert` and `VacantEntry::insert`: `e`, holding `item` in place of what it held, which drops. */
export function tableEntryInsert<T>(e: TableEntry<T>, item: T, f?: (item: T) => void): TableEntry<T> {
	if (!e.o) return e.fill(item);
	const old = e.v;
	e.v = item;
	f?.(old);
	return e;
}

/** `Entry::or_insert`: `e`, holding `item` if it was vacant, or else what it held, and `item` drops. */
export function tableOrInsert<T>(e: TableEntry<T>, item: T, f?: (item: T) => void): TableEntry<T> {
	if (!e.o) return e.fill(item);
	f?.(item);
	return e;
}

/** `Entry::or_insert_with`: `e`, holding `make()`'s item if it was vacant, or else `make` drops by `dropF` uncalled. */
export function tableOrInsertWith<T>(e: TableEntry<T>, make: () => T, dropF?: (f: () => T) => void): TableEntry<T> {
	if (!e.o) return e.fill(make());
	dropF?.(make);
	return e;
}

/** `Entry::and_modify`: `f` of its item, if occupied, or else `f` drops by `dropF` uncalled. */
export function tableAndModify<T>(e: TableEntry<T>, f: (item: T) => void, dropF?: (f: (item: T) => void) => void): TableEntry<T> {
	if (e.o) f(e.v);
	else dropF?.(f);
	return e;
}

/** `Entry::and_modify` of items JS copies: `f` of its item's place, if occupied, as `tableAndModify`. */
export function tableAndModifyMut<T>(e: TableEntry<T>, f: (item: Place<T>) => void, dropF?: (f: (item: Place<T>) => void) => void): TableEntry<T> {
	if (e.o) f(e);
	else dropF?.(f);
	return e;
}

/** `OccupiedEntry::remove`: its item, and itself, vacant. */
export function tableRemove<T>(e: TableEntry<T>): [T, TableEntry<T>] {
	return [e.take(), e];
}

/** `HashTable::retain`, which keeps each item `f` returns true for. Each goes, and drops by `d`, as `f` rejects it. */
export function tableRetain<T>(t: HashTable<T>, f: (item: T) => boolean, d?: (item: T) => void): void {
	retainHash(t, (_, item) => f(item as T), false, undefined, d);
}

/** `HashTable::retain` of items JS copies: `f` takes the place of each. */
export function tableRetainMut<T>(t: HashTable<T>, f: (item: Place<T>) => boolean, d?: (item: T) => void): void {
	retainHash(t, (_, item) => f(item as Place<T>), true, undefined, d);
}

/** `HashTable::drain`: every item, taken out at once, each it still holds dropping by `d` as it drops. */
export function tableDrain<T>(t: HashTable<T>, d?: (item: T) => void): Iter<T> {
	return new HashIter<undefined, T, T>(taken(t), VALUES, undefined, d, true);
}

// BTreeMap and BTreeSet

/**
 * The most keys a node holds, std's: `2 * B - 1` of its `B` of 6. Each node is std's,
 * split, merged, and evened out where std's is, so a search reads the same keys in the same
 * order.
 */
const CAPACITY = 11;
/** The fewest keys a node other than the root holds after std's removals. */
const MIN_LEN = 5;

/**
 * A node, as std's: its keys in order, each key's value at the same index, and of an
 * internal node, its children, one more than its keys, each holding the keys between the
 * two around it. Each knows its parent and its index among the parent's children.
 */
interface Node<K, V> {
	k: K[];
	v: V[];
	e: Node<K, V>[] | undefined;
	p: Node<K, V> | undefined;
	i: number;
}

/** `BTreeMap<K, V>`, and `BTreeSet<K>` as a map to `undefined`: std's B-tree, ordered by `Ord`. */
export interface BTreeMap<K, V> {
	/** The root, which is a leaf where `h` is 0. */
	t: Node<K, V>;
	/** The height: how many internal nodes are above each leaf. */
	h: number;
	/** How many keys it holds. */
	n: number;
	/** `Ord::cmp` of the keys. */
	readonly c: (a: K, b: K) => Ordering;
	/**
	 * Whether std's map would have its root node: from its first insert until `clear`.
	 * Only a map with one checks a range's bounds.
	 */
	r: boolean;
}

/** `BTreeSet<K>`. */
export type BTreeSet<K> = BTreeMap<K, undefined>;

/** An empty node, internal if it has children `e`, which it becomes the parent of. */
function node<K, V>(e?: Node<K, V>[]): Node<K, V> {
	const n: Node<K, V> = { k: [], v: [], e, p: undefined, i: 0 };
	if (e !== undefined) adopt(n);
	return n;
}

/** Makes `n` the parent of each of its children from index `from` on, at its index. */
function adopt<K, V>(n: Node<K, V>, from = 0): void {
	const e = n.e as Node<K, V>[];
	for (let i = from; i < e.length; i++) {
		e[i].p = n;
		e[i].i = i;
	}
}

/**
 * `BTreeMap::new` and `BTreeSet::new`, ordered by `cmp`. The type `K` is asserted, not
 * inferred from `cmp`: a slice's order takes any `ArrayLike`.
 */
export function btreeNew<K, V>(cmp: (a: NoInfer<K>, b: NoInfer<K>) => Ordering): BTreeMap<K, V> {
	return { t: node(), h: 0, n: 0, c: cmp, r: false };
}

/**
 * The index a function that finds a key or edge leaves here beside the node it returns,
 * which its caller reads before anything else runs.
 */
let idx = 0;

/**
 * A key of another type `Q`, which a B-tree's keys borrow as, that it is searched by as
 * std's search by a `Borrow` form is: by `Q`'s `Ord::cmp` of it against each key's `borrow`.
 */
export class By<K> {
	readonly q: unknown;
	/** `Q`'s `Ord::cmp`. */
	readonly cmp: (a: unknown, b: unknown) => Ordering;
	/** `Q`'s `Ord::cmp` of a `Q` against a key's `borrow`. */
	readonly c: (q: unknown, k: K) => Ordering;

	constructor(q: unknown, cmp: (a: unknown, b: unknown) => Ordering, c: (q: unknown, k: K) => Ordering) {
		this.q = q;
		this.cmp = cmp;
		this.c = c;
	}
}

/** `q`, a key or a range of keys of type `Q`, by `Q`'s `cmp` against each key `K`'s `borrow`. */
export function by<Q, K>(q: NoInfer<Q | Bounds<Q>>, cmp: (a: Q, b: Q) => Ordering, borrow: (k: K) => Q): By<K> {
	return new By(q, cmp as (a: unknown, b: unknown) => Ordering, (q, k) => cmp(q as Q, borrow(k)));
}

/**
 * std's `find_key_index`: the index of the first of `n`'s keys from `from` on that `key` is
 * not greater than, by `key.cmp(k)`, if equal, or else the bitwise not of the edge before it.
 */
function find<Q, K, V>(cmp: (a: Q, b: K) => Ordering, n: Node<K, V>, key: Q, from = 0): number {
	const keys = n.k;
	const len = keys.length;
	for (let i = from; i < len; i++) {
		const order = cmp(key, keys[i]);
		if (order === 0) return i;
		if (order < 0) return ~i;
	}
	return ~len;
}

/**
 * std's `search_tree`: the node holding a key equal to `key`, with `idx` its index, or else
 * the leaf it would go in, with `idx` the bitwise not of the edge it goes at.
 */
function seek<K, V>(m: BTreeMap<K, V>, key: K): Node<K, V> {
	return seekBy(m, m.c, key);
}

/** `seek` by `key`, or by what a `By` holds, as std's lookups search by a `Borrow` form. */
function lookup<K, V>(m: BTreeMap<K, V>, key: K | By<K>): Node<K, V> {
	return key instanceof By ? seekBy(m, key.c, key.q) : seekBy(m, m.c, key);
}

/** `seek` by `cmp`, of `key` against each key. */
function seekBy<Q, K, V>(m: BTreeMap<K, V>, cmp: (a: Q, b: K) => Ordering, key: Q): Node<K, V> {
	let n = m.t;
	for (;;) {
		const i = find(cmp, n, key);
		if (i >= 0 || n.e === undefined) {
			idx = i;
			return n;
		}
		n = n.e[~i];
	}
}

/** The first leaf, or with `last`, the last. */
function end<K, V>(m: BTreeMap<K, V>, last: boolean): Node<K, V> {
	let n = m.t;
	while (n.e !== undefined) n = n.e[last ? n.e.length - 1 : 0];
	return n;
}

/** std's `next_leaf_edge` of the key at index `i` of `n`: the leaf, with `idx` the edge's index. */
function nextEdge<K, V>(n: Node<K, V>, i: number): Node<K, V> {
	if (n.e === undefined) {
		idx = i + 1;
		return n;
	}
	n = n.e[i + 1];
	while (n.e !== undefined) n = n.e[0];
	idx = 0;
	return n;
}

/** std's `next_back_leaf_edge` of the key at index `i` of `n`: the leaf, with `idx` the edge's index. */
function backEdge<K, V>(n: Node<K, V>, i: number): Node<K, V> {
	if (n.e === undefined) {
		idx = i;
		return n;
	}
	n = n.e[i];
	while (n.e !== undefined) n = n.e[n.e.length - 1];
	idx = n.k.length;
	return n;
}

export function btreeLen<K, V>(m: BTreeMap<K, V>): number {
	return m.n;
}

/** `BTreeMap::get`: the value there, or else `none`, the `None` of an `Option` of a value that can be `undefined`. */
export function btreeGet<K, V>(m: BTreeMap<K, V>, key: K | By<K>): V | undefined;
export function btreeGet<K, V>(m: BTreeMap<K, V>, key: K | By<K>, none: None): V | None;
export function btreeGet<K, V>(m: BTreeMap<K, V>, key: K | By<K>, none?: None): V | None | undefined {
	const n = lookup(m, key);
	return idx < 0 ? none : n.v[idx];
}

/** `BTreeMap::get_mut` of a value JS copies: its place, if the map holds `key`. */
export function btreeGetMut<K, V>(m: BTreeMap<K, V>, key: K | By<K>): Place<V> | undefined {
	const n = lookup(m, key);
	return idx < 0 ? undefined : new AtIndex(n.v, idx);
}

/** `BTreeMap::get_key_value`: the equal key the map holds, and its value. */
export function btreeGetKeyValue<K, V>(m: BTreeMap<K, V>, key: K | By<K>): [K, V] | undefined {
	const n = lookup(m, key);
	return idx < 0 ? undefined : [n.k[idx], n.v[idx]];
}

/** `BTreeSet::get`, and `BTreeSet::take` if `take`: the equal key the set holds, or else `none`. */
export function btreeKey<K, V>(m: BTreeMap<K, V>, key: K | By<K>, take?: boolean): K | undefined;
export function btreeKey<K, V>(m: BTreeMap<K, V>, key: K | By<K>, take: boolean, none: None): K | None;
export function btreeKey<K, V>(m: BTreeMap<K, V>, key: K | By<K>, take = false, none?: None): K | None | undefined {
	const n = lookup(m, key);
	const i = idx;
	if (i < 0) return none;
	const held = n.k[i];
	if (take) removeAt(m, n, i);
	return held;
}

/** `BTreeSet::replace`: the equal key the set held, which `key` takes the place of, or else `none`, with `key` added. */
export function btreeReplace<K>(m: BTreeSet<K>, key: K): K | undefined;
export function btreeReplace<K>(m: BTreeSet<K>, key: K, none: None): K | None;
export function btreeReplace<K>(m: BTreeSet<K>, key: K, none?: None): K | None | undefined {
	const n = seek(m, key);
	const i = idx;
	if (i < 0) {
		insertAt(m, n, ~i, key, undefined);
		return none;
	}
	const old = n.k[i];
	n.k[i] = key;
	return old;
}

export function btreeContains<K, V>(m: BTreeMap<K, V>, key: K | By<K>): boolean {
	lookup(m, key);
	return idx >= 0;
}

/** `BTreeMap::index`: `map[&key]`. */
export function btreeIndex<K, V>(m: BTreeMap<K, V>, key: K | By<K>, at?: Location): V {
	const n = lookup(m, key);
	if (idx < 0) panic("no entry found for key", at, true);
	return n.v[idx];
}

/** `BTreeMap::insert`: the old value, or else `none`. An equal key already there stays, and `key` drops. */
export function btreeInsert<K, V>(m: BTreeMap<K, V>, key: K, value: V, k?: (key: K) => void): V | undefined;
export function btreeInsert<K, V>(m: BTreeMap<K, V>, key: K, value: V, k: ((key: K) => void) | undefined, none: None): V | None;
export function btreeInsert<K, V>(m: BTreeMap<K, V>, key: K, value: V, k?: (key: K) => void, none?: None): V | None | undefined {
	const n = seek(m, key);
	const i = idx;
	if (i < 0) {
		insertAt(m, n, ~i, key, value);
		return none;
	}
	k?.(key);
	const old = n.v[i];
	n.v[i] = value;
	return old;
}

/**
 * std's `insert_recursing`: puts `key` and `value` at edge `i` of leaf `l`, splitting a full
 * node about std's `splitpoint` and putting its middle key into its parent in turn, or a
 * new root. Returns the leaf the key went in, with `idx` its index.
 */
function insertAt<K, V>(m: BTreeMap<K, V>, l: Node<K, V>, i: number, key: K, value: V): Node<K, V> {
	m.r = true;
	m.n++;
	let n = l;
	let edge: Node<K, V> | undefined;
	let leaf: Node<K, V> | undefined;
	let index = 0;
	for (;;) {
		let into = n;
		let j = i;
		let right: Node<K, V> | undefined;
		if (n.k.length === CAPACITY) {
			const mid = i < 5 ? 4 : i <= 6 ? 5 : 6;
			right = split(n, mid);
			if (i > mid) {
				into = right;
				j = i - mid - 1;
			}
		}
		put(into.k, j, key);
		put(into.v, j, value);
		if (edge !== undefined) {
			put(into.e as Node<K, V>[], j + 1, edge);
			adopt(into, j + 1);
		}
		if (leaf === undefined) {
			leaf = into;
			index = j;
		}
		if (right === undefined) break;
		// The middle key, left last in `n`, goes up.
		key = n.k.pop() as K;
		value = n.v.pop() as V;
		edge = right;
		if (n.p === undefined) {
			const root = node([n, right]);
			root.k.push(key);
			root.v.push(value);
			m.t = root;
			m.h++;
			break;
		}
		i = n.i;
		n = n.p;
	}
	idx = index;
	return leaf;
}

/** Moves the keys of `n` after index `mid`, and the children after them, into a new node, which it returns. */
function split<K, V>(n: Node<K, V>, mid: number): Node<K, V> {
	const right = node(n.e?.splice(mid + 1));
	right.k = n.k.splice(mid + 1);
	right.v = n.v.splice(mid + 1);
	return right;
}

/** Puts `item` at index `i` of `a`, after moving the rest up: in a node, faster than `splice`. */
function put<T>(a: T[], i: number, item: T): void {
	let j = a.length;
	a.push(item);
	for (; j > i; j--) a[j] = a[j - 1];
	a[i] = item;
}

/** Takes index `i` out of `a`, moving the rest down. */
function take<T>(a: T[], i: number): void {
	const n = a.length - 1;
	for (let j = i; j < n; j++) a[j] = a[j + 1];
	a.pop();
}

/** `BTreeSet::insert`: whether `key` was not there yet. An equal key already there stays, and `key` drops. */
export function btreeAdd<K>(m: BTreeSet<K>, key: K, k?: (key: K) => void): boolean {
	const n = seek(m, key);
	const i = idx;
	if (i >= 0) {
		k?.(key);
		return false;
	}
	insertAt(m, n, ~i, key, undefined);
	return true;
}

/**
 * std's `remove_kv_tracking`: takes the key and value at index `i` of `n` out, an internal
 * node's by moving the last key before it in its place, and rebalances as std's does.
 * Returns the leaf edge where it was, the leaf, with `idx` the edge's index.
 */
function removeAt<K, V>(m: BTreeMap<K, V>, n: Node<K, V>, i: number): Node<K, V> {
	m.n--;
	if (n.e === undefined) return removeLeaf(m, n, i);
	let l = n.e[i];
	while (l.e !== undefined) l = l.e[l.e.length - 1];
	const last = l.k.length - 1;
	const key = l.k[last];
	const value = l.v[last];
	// Rebalancing may move the key being replaced, which is the next one after the hole.
	let hole = removeLeaf(m, l, last);
	let j = idx;
	while (j === hole.k.length) {
		j = hole.i;
		hole = hole.p as Node<K, V>;
	}
	hole.k[j] = key;
	hole.v[j] = value;
	return nextEdge(hole, j);
}

/**
 * std's `remove_leaf_kv`: takes index `i` out of leaf `n`, and if that leaves it too small,
 * merges it with a sibling or takes one key from it, the left one if it has one, then fixes
 * each ancestor that leaves too small. Returns where the key was, as `removeAt`.
 */
function removeLeaf<K, V>(m: BTreeMap<K, V>, n: Node<K, V>, i: number): Node<K, V> {
	take(n.k, i);
	take(n.v, i);
	const p = n.p;
	if (n.k.length >= MIN_LEN || p === undefined) {
		idx = i;
		return n;
	}
	let pos = n;
	if (n.i > 0) {
		const left = (p.e as Node<K, V>[])[n.i - 1];
		if (left.k.length + 1 + n.k.length <= CAPACITY) {
			i += left.k.length + 1;
			merge(p, n.i - 1);
			pos = left;
		} else {
			stealLeft(p, n.i - 1, 1);
			i++;
		}
	} else if (n.k.length + 1 + (p.e as Node<K, V>[])[1].k.length <= CAPACITY) {
		merge(p, 0);
	} else {
		stealRight(p, 0, 1);
	}
	if (!fixUp(pos.p as Node<K, V>)) popRoot(m);
	idx = i;
	return pos;
}

/**
 * std's `fix_node_and_affected_ancestors`: merges `n` and each ancestor that left too
 * small with a sibling, or evens it out with one. False if it emptied the internal root.
 */
function fixUp<K, V>(n: Node<K, V>): boolean {
	for (;;) {
		const len = n.k.length;
		if (len >= MIN_LEN) return true;
		const p = n.p;
		if (p === undefined) return len > 0;
		const e = p.e as Node<K, V>[];
		const j = n.i > 0 ? n.i - 1 : 0;
		if (e[j].k.length + 1 + e[j + 1].k.length <= CAPACITY) {
			merge(p, j);
			n = p;
		} else {
			if (n.i > 0) stealLeft(p, j, MIN_LEN - len);
			else stealRight(p, j, MIN_LEN - len);
			return true;
		}
	}
}

/** std's `pop_internal_level`: the root's first child becomes the root. */
function popRoot<K, V>(m: BTreeMap<K, V>): void {
	m.t = (m.t.e as Node<K, V>[])[0];
	m.t.p = undefined;
	m.h--;
}

/** std's `do_merge`: moves key `j` of `p`, then its right child's keys and children, into its left child. */
function merge<K, V>(p: Node<K, V>, j: number): void {
	const e = p.e as Node<K, V>[];
	const left = e[j];
	const right = e[j + 1];
	const old = left.k.length;
	left.k.push(p.k[j]);
	left.v.push(p.v[j]);
	append(left.k, right.k, 0);
	append(left.v, right.v, 0);
	take(p.k, j);
	take(p.v, j);
	take(e, j + 1);
	adopt(p, j + 1);
	if (left.e !== undefined) {
		append(left.e, right.e as Node<K, V>[], 0);
		adopt(left, old + 1);
	}
}

/**
 * std's `bulk_steal_left`: moves the last `count` keys of the left child of key `j` of `p`
 * through it into its right child, the first of them into `p`, with their children.
 */
function stealLeft<K, V>(p: Node<K, V>, j: number, count: number): void {
	const e = p.e as Node<K, V>[];
	const left = e[j];
	const right = e[j + 1];
	const keep = left.k.length - count;
	passRight(left.k, right.k, p.k, j, keep, count);
	passRight(left.v, right.v, p.v, j, keep, count);
	if (right.e !== undefined) {
		const from = left.e as Node<K, V>[];
		shiftUp(right.e, count);
		for (let t = 0; t < count; t++) right.e[t] = from[keep + 1 + t];
		from.length = keep + 1;
		adopt(right);
	}
}

/** Moves `from`'s items from `keep` on through index `j` of `parent` to the front of `to`, as `stealLeft` moves keys. */
function passRight<T>(from: T[], to: T[], parent: T[], j: number, keep: number, count: number): void {
	shiftUp(to, count);
	for (let t = 0; t < count - 1; t++) to[t] = from[keep + 1 + t];
	to[count - 1] = parent[j];
	parent[j] = from[keep];
	from.length = keep;
}

/**
 * std's `bulk_steal_right`: moves the first `count` keys of the right child of key `j` of
 * `p` through it into its left child, the last of them into `p`, with their children.
 */
function stealRight<K, V>(p: Node<K, V>, j: number, count: number): void {
	const e = p.e as Node<K, V>[];
	const left = e[j];
	const right = e[j + 1];
	const old = left.k.length;
	passLeft(right.k, left.k, p.k, j, count);
	passLeft(right.v, left.v, p.v, j, count);
	if (left.e !== undefined) {
		const from = right.e as Node<K, V>[];
		for (let t = 0; t < count; t++) left.e.push(from[t]);
		shiftDown(from, count);
		adopt(left, old + 1);
		adopt(right);
	}
}

/** Moves `from`'s first `count` items through index `j` of `parent` to the end of `to`, as `stealRight` moves keys. */
function passLeft<T>(from: T[], to: T[], parent: T[], j: number, count: number): void {
	to.push(parent[j]);
	for (let t = 0; t < count - 1; t++) to.push(from[t]);
	parent[j] = from[count - 1];
	shiftDown(from, count);
}

/** Moves each item of `a` up `count` slots, leaving the first `count` to be written. */
function shiftUp<T>(a: T[], count: number): void {
	const n = a.length;
	for (let t = 0; t < count; t++) a.push(a[n - 1]);
	for (let i = n - 1; i >= 0; i--) a[i + count] = a[i];
}

/** Takes the first `count` items out of `a`, moving the rest down. */
function shiftDown<T>(a: T[], count: number): void {
	const n = a.length - count;
	for (let i = 0; i < n; i++) a[i] = a[i + count];
	a.length = n;
}

/** Pushes `b`'s items from `from` on onto `a`. */
function append<T>(a: T[], b: T[], from: number): void {
	for (let i = from; i < b.length; i++) a.push(b[i]);
}

/** `BTreeMap::remove_entry`. */
export function btreeRemoveEntry<K, V>(m: BTreeMap<K, V>, key: K | By<K>): [K, V] | undefined {
	const n = lookup(m, key);
	const i = idx;
	if (i < 0) return undefined;
	const entry: [K, V] = [n.k[i], n.v[i]];
	removeAt(m, n, i);
	return entry;
}

/** `BTreeMap::remove`: the value there, whose key drops, or else `none`. */
export function btreeRemove<K, V>(m: BTreeMap<K, V>, key: K | By<K>, k?: (key: K) => void): V | undefined;
export function btreeRemove<K, V>(m: BTreeMap<K, V>, key: K | By<K>, k: ((key: K) => void) | undefined, none: None): V | None;
export function btreeRemove<K, V>(m: BTreeMap<K, V>, key: K | By<K>, k?: (key: K) => void, none?: None): V | None | undefined {
	const n = lookup(m, key);
	const i = idx;
	if (i < 0) return none;
	const held = n.k[i];
	const value = n.v[i];
	removeAt(m, n, i);
	k?.(held);
	return value;
}

/** `BTreeSet::remove`: whether `key` was there. The key it held drops. */
export function btreeDelete<K>(m: BTreeSet<K>, key: K | By<K>, k?: (key: K) => void): boolean {
	const n = lookup(m, key);
	const i = idx;
	if (i < 0) return false;
	const held = n.k[i];
	removeAt(m, n, i);
	k?.(held);
	return true;
}

/** An internal node with no keys and one child, and below it another, down to an empty leaf: `height` of them above it. */
function pillar<K, V>(height: number): Node<K, V> {
	let n = node<K, V>();
	for (let h = 0; h < height; h++) n = node([n]);
	return n;
}

/**
 * std's `bulk_push` into `m`, an empty tree: `keys` and their `values`, in order and
 * without equal keys, each pushed onto the last leaf, or where that is full, onto the
 * lowest node above it with room, with a new right child of empty nodes, before the right
 * border takes enough from its left siblings.
 */
function bulk<K, V>(m: BTreeMap<K, V>, keys: K[], values: V[]): void {
	m.t = node();
	m.h = 0;
	m.n = keys.length;
	let leaf = m.t;
	for (let j = 0; j < keys.length; j++) {
		if (leaf.k.length < CAPACITY) {
			leaf.k.push(keys[j]);
			leaf.v.push(values[j]);
			continue;
		}
		let open = leaf.p;
		let h = 1;
		for (; open !== undefined && open.k.length === CAPACITY; h++) open = open.p;
		if (open === undefined) {
			open = node([m.t]);
			m.t = open;
			m.h++;
		}
		const e = open.e as Node<K, V>[];
		open.k.push(keys[j]);
		open.v.push(values[j]);
		e.push(pillar(h - 1));
		adopt(open, e.length - 1);
		leaf = open;
		while (leaf.e !== undefined) leaf = leaf.e[leaf.e.length - 1];
	}
	// std's `fix_right_border_of_plentiful`.
	for (let n = m.t; n.e !== undefined; n = n.e[n.e.length - 1]) {
		const short = MIN_LEN - n.e[n.e.length - 1].k.length;
		if (short > 0) stealLeft(n, n.k.length - 1, short);
	}
}

/** `first_key_value`, and with `last`, `last_key_value`. */
export function btreeFirst<K, V>(m: BTreeMap<K, V>, last = false): [K, V] | undefined {
	if (m.n === 0) return undefined;
	const l = end(m, last);
	const i = last ? l.k.length - 1 : 0;
	return [l.k[i], l.v[i]];
}

/** `BTreeSet::first`, and with `last`, `BTreeSet::last`, or else `none`. */
export function btreeFirstKey<K, V>(m: BTreeMap<K, V>, last?: boolean): K | undefined;
export function btreeFirstKey<K, V>(m: BTreeMap<K, V>, last: boolean, none: None): K | None;
export function btreeFirstKey<K, V>(m: BTreeMap<K, V>, last = false, none?: None): K | None | undefined {
	if (m.n === 0) return none;
	const l = end(m, last);
	return l.k[last ? l.k.length - 1 : 0];
}

/** `pop_first`, and with `last`, `pop_last`. */
export function btreePop<K, V>(m: BTreeMap<K, V>, last = false): [K, V] | undefined {
	if (m.n === 0) return undefined;
	const l = end(m, last);
	const i = last ? l.k.length - 1 : 0;
	const entry: [K, V] = [l.k[i], l.v[i]];
	removeAt(m, l, i);
	return entry;
}

/** `BTreeSet::pop_first`, and with `last`, `pop_last`, or else `none`. */
export function btreePopKey<K, V>(m: BTreeMap<K, V>, last?: boolean): K | undefined;
export function btreePopKey<K, V>(m: BTreeMap<K, V>, last: boolean, none: None): K | None;
export function btreePopKey<K, V>(m: BTreeMap<K, V>, last = false, none?: None): K | None | undefined {
	const entry = btreePop(m, last);
	return entry === undefined ? none : entry[0];
}

/**
 * Drop `m`: each key by `k` and each value by `v`, of those that drop, as std's `drop_tree`
 * does, node by node in postorder, so each node's entries in order after its children's.
 */
export function btreeDrop<K, V>(m: BTreeMap<K, V>, k: ((key: K) => void) | undefined, v?: (value: V) => void): void {
	dropTree(m.t, k, v);
}

/** Drops the entries of `n` and the nodes below it, as `btreeDrop` does. */
function dropTree<K, V>(n: Node<K, V>, k: ((key: K) => void) | undefined, v?: (value: V) => void): void {
	if (n.e !== undefined) for (const child of n.e) dropTree(child, k, v);
	dropPairs(n.k, n.v, k, v);
}

/** `clear`, which leaves the map empty and then drops it, as `btreeDrop` does. */
export function btreeClear<K, V>(m: BTreeMap<K, V>, k?: (key: K) => void, v?: (value: V) => void): void {
	if (k !== undefined || v !== undefined) {
		const old: BTreeMap<K, V> = { t: m.t, h: m.h, n: m.n, c: m.c, r: m.r };
		btreeClear(m);
		btreeDrop(old, k, v);
		return;
	}
	m.t = node();
	m.h = 0;
	m.n = 0;
	m.r = false;
}

/**
 * `split_off` of a map or set, as std's: the entries from `key` on, cut from each node on
 * the path to it into a column of empty nodes, whose borders then even out. Like std's,
 * the new tree has a root unless `m` was empty.
 */
export function btreeSplitOff<K, V>(m: BTreeMap<K, V>, key: K | By<K>): BTreeMap<K, V> {
	const out = btreeNew<K, V>(m.c);
	if (m.n === 0) return out;
	const [cmp, q] = key instanceof By ? [key.c, key.q] : [m.c as (a: unknown, b: K) => Ordering, key];
	out.t = pillar(m.h);
	out.h = m.h;
	out.r = true;
	let left = m.t;
	let right = out.t;
	for (;;) {
		const i = find(cmp, left, q);
		const cut = i < 0 ? ~i : i;
		right.k = left.k.splice(cut);
		right.v = left.v.splice(cut);
		if (left.e === undefined) break;
		const e = right.e as Node<K, V>[];
		e.push(...left.e.splice(cut + 1));
		adopt(right, 1);
		left = left.e[cut];
		right = e[0];
	}
	fixBorder(m, true);
	fixBorder(out, false);
	out.n = count(out.t);
	m.n -= out.n;
	return out;
}

/** How many keys `n` and the nodes below it hold. */
function count<K, V>(n: Node<K, V>): number {
	let total = n.k.length;
	if (n.e !== undefined) for (const c of n.e) total += count(c);
	return total;
}

/**
 * std's `fix_right_border` of `m`, or else its `fix_left_border`: drops empty roots, and
 * merges each node along the border with its sibling, or has it take enough from it.
 */
function fixBorder<K, V>(m: BTreeMap<K, V>, last: boolean): void {
	while (m.h > 0 && m.t.k.length === 0) popRoot(m);
	if (m.t.k.length === 0) return;
	for (let n = m.t; n.e !== undefined; ) {
		const j = last ? n.k.length - 1 : 0;
		const left = n.e[j];
		const right = n.e[j + 1];
		const short = MIN_LEN + 1 - (last ? right : left).k.length;
		if (left.k.length + 1 + right.k.length <= CAPACITY) {
			merge(n, j);
			n = left;
		} else {
			if (short > 0 && last) stealLeft(n, j, short);
			else if (short > 0) stealRight(n, j, short);
			n = last ? right : left;
		}
	}
	while (m.h > 0 && m.t.k.length === 0) popRoot(m);
}

/**
 * `retain`, which visits the keys in order and keeps each entry `f` returns true for,
 * dropping each other one as `f` rejects it.
 */
export function btreeRetain<K, V>(m: BTreeMap<K, V>, f: (key: K, value: V) => boolean, k?: (key: K) => void, v?: (value: V) => void): void {
	retainBTree(m, f as (key: K, value: V | Place<V>) => boolean, false, k, v);
}

/** `retain` of values JS copies: `f` takes the place of each. */
export function btreeRetainMut<K, V>(m: BTreeMap<K, V>, f: (key: K, value: Place<V>) => boolean, k?: (key: K) => void, v?: (value: V) => void): void {
	retainBTree(m, f as (key: K, value: V | Place<V>) => boolean, true, k, v);
}

/** `retain`, as std's: `extract_if` of the whole map, handing `f` each value or, if `lend`, its place. */
function retainBTree<K, V>(m: BTreeMap<K, V>, f: (key: K, value: V | Place<V>) => boolean, lend: boolean, k?: (key: K) => void, v?: (value: V) => void): void {
	const it = new BTreeExtractIf<K, V, [K, V]>(m, undefined, (key, value) => !f(key, value), lend, false);
	for (let entry = it.next(); entry !== DONE; entry = it.next()) dropPair(entry[0], entry[1], k, v);
}

/**
 * An iterator of a B-tree's entries, keys, or values, from leaf edge `fi` of leaf `fl` up
 * to leaf edge `bi` of leaf `bl`, as std's: the front climbs from a leaf's last edge to the
 * next key, then steps to the first leaf edge after it, and the back likewise. One that
 * owns its entries drops as `HashIter` does.
 */
export class BTreeIter<K, V, T> extends Iter<T> implements Disposable {
	fl: Node<K, V>;
	fi: number;
	bl: Node<K, V>;
	bi: number;
	/** How many items are left, where it is known: an iterator of the whole map. */
	private n: number;
	private readonly pick: Pick;
	private readonly k: ((key: K) => void) | undefined;
	private readonly v: ((value: V) => void) | undefined;
	/** Whether its `last` reads from the back, as a map's `into_keys`' and `into_values`' do. */
	readonly back: boolean;

	constructor(pick: Pick, fl: Node<K, V>, fi: number, bl: Node<K, V>, bi: number, n = -1, k?: (key: K) => void, v?: (value: V) => void, back = false) {
		super();
		this.pick = pick;
		this.fl = fl;
		this.fi = fi;
		this.bl = bl;
		this.bi = bi;
		this.n = n;
		this.k = k;
		this.v = v;
		this.back = back;
	}

	next(): T | Done {
		if (done(this)) return DONE;
		const n = front(this);
		this.n--;
		return this.picked(n, idx);
	}

	override nextBack(): T | Done {
		if (done(this)) return DONE;
		let n = this.bl;
		let i = this.bi;
		while (i === 0) {
			i = n.i;
			n = n.p as Node<K, V>;
		}
		i--;
		this.bl = backEdge(n, i);
		this.bi = idx;
		this.n--;
		return this.picked(n, i);
	}

	/**
	 * What it yields of index `i` of node `n`: of `into_keys` or `into_values`, the other
	 * half drops first.
	 */
	private picked(n: Node<K, V>, i: number): T {
		if (this.pick === ENTRIES) return [n.k[i], n.v[i]] as T;
		if (this.pick === PLACES) return [n.k[i], new AtIndex(n.v, i)] as T;
		if (this.pick === VALUE_PLACES) return new AtIndex(n.v, i) as T;
		if (this.pick === KEYS) {
			this.v?.(n.v[i]);
			return n.k[i] as unknown as T;
		}
		this.k?.(n.k[i]);
		return n.v[i] as unknown as T;
	}

	override remaining(): Iterable<unknown> {
		return new BTreeIter<K, V, unknown>(byValue(this.pick), this.fl, this.fi, this.bl, this.bi);
	}

	/** std's, of an iterator that borrows the map. */
	override clone(): Iter<T> {
		return new BTreeIter<K, V, T>(this.pick, this.fl, this.fi, this.bl, this.bi, this.n);
	}

	override len(): number {
		return this.n;
	}

	override sizeHint(): SizeHint {
		// A range's count is unknown, and std's hint of one is `(0, None)`.
		return this.n < 0 ? [0, undefined] : exactHint(this.n);
	}

	[Symbol.dispose](): void {
		if (this.k === undefined && this.v === undefined) return;
		const keys: K[] = [];
		const values: V[] = [];
		while (!done(this)) {
			const n = front(this);
			keys.push(n.k[idx]);
			values.push(n.v[idx]);
		}
		this.n = 0;
		dropPairs(keys, values, this.k, this.v);
	}
}

/** Whether `it`'s front has met its back: the same leaf edge, as only one lies between two keys. */
function done<K, V, T>(it: BTreeIter<K, V, T>): boolean {
	return it.fi === it.bi && it.fl === it.bl;
}

/** Steps `it`'s front past its next key: the node holding that key, with `idx` its index. */
function front<K, V, T>(it: BTreeIter<K, V, T>): Node<K, V> {
	let n = it.fl;
	let i = it.fi;
	while (i === n.k.length) {
		i = n.i;
		n = n.p as Node<K, V>;
	}
	it.fl = nextEdge(n, i);
	it.fi = idx;
	idx = i;
	return n;
}

function whole<K, V, T>(m: BTreeMap<K, V>, pick: Pick, k?: (key: K) => void, v?: (value: V) => void, back = false): Iter<T> {
	const last = end(m, true);
	return new BTreeIter<K, V, T>(pick, end(m, false), 0, last, last.k.length, m.n, k, v, back);
}

/**
 * `iter` and `iter_mut` of a map, and with what drops each key and value of those that
 * drop, `into_iter`: its entries in key order.
 */
export function btreeIter<K, V>(m: BTreeMap<K, V>, k?: (key: K) => void, v?: (value: V) => void): Iter<[K, V]> {
	return whole(m, ENTRIES, k, v);
}

/**
 * `keys` of a map and `iter` of a set, and with droppers as `btreeIter`'s, `into_iter` of
 * a set, or with `back`, `into_keys`.
 */
export function btreeKeys<K, V>(m: BTreeMap<K, V>, k?: (key: K) => void, v?: (value: V) => void, back = false): Iter<K> {
	return whole(m, KEYS, k, v, back);
}

/** `iter_mut` of values JS copies: each key and the place of its value, in key order. */
export function btreeIterMut<K, V>(m: BTreeMap<K, V>): Iter<[K, Place<V>]> {
	return whole(m, PLACES);
}

/** `values_mut` of values JS copies: the place of each. */
export function btreeValuesMut<K, V>(m: BTreeMap<K, V>): Iter<Place<V>> {
	return whole(m, VALUE_PLACES);
}

/** `values` and `values_mut`, and with droppers as `btreeIter`'s, `into_values`. */
export function btreeValues<K, V>(m: BTreeMap<K, V>, k?: (key: K) => void, v?: (value: V) => void): Iter<V> {
	return whole(m, VALUES, k, v, k !== undefined || v !== undefined);
}

/**
 * A range of keys or indices, as its value is: `a..b`, `a..=b`, `a..` as `{ start }`, `..b`
 * as `{ end }`, `..` as `undefined`, and `(Bound<K>, Bound<K>)` as its pair. Any other
 * `RangeBounds` is a `BoundsOf`, whose `start_bound` and `end_bound` are called where
 * std's calls them.
 */
export type Bounds<K> =
	| { readonly start: K; readonly end: K }
	| RangeInclusive<K>
	| { readonly start: K }
	| { readonly end: K; readonly inclusive?: true }
	| BoundPair<K>
	| BoundsOf<K>
	| undefined;

type BoundPair<K> = readonly [RangeBound<K>, RangeBound<K>];

/** Another `RangeBounds`: its `start_bound` and `end_bound`, each read as it is called. */
interface BoundsOf<K> {
	readonly startBound: () => RangeBound<K>;
	readonly endBound: () => RangeBound<K>;
}

const UNBOUNDED: RangeBound<never> = { tag: "Unbounded" };

/** Another `RangeBounds`, `r`, as `Bounds` reads it, by `start` and `end`. */
export function boundsOf<R, K>(r: R, start: (r: R) => RangeBound<K>, end: (r: R) => RangeBound<K>): BoundsOf<K> {
	return { startBound: () => start(r), endBound: () => end(r) };
}

/** Whether `r` is a `BoundsOf`: a field test, as `inclusive`'s is. */
function other<K>(r: Bounds<K>): r is BoundsOf<K> {
	return typeof r === "object" && "endBound" in r;
}

/** `RangeBounds`: `start_bound`, then `end_bound`. */
function bounds<K>(r: Bounds<K>): BoundPair<K> {
	if (r === undefined) return [UNBOUNDED, UNBOUNDED];
	if (other(r)) return [r.startBound(), r.endBound()];
	if (Array.isArray(r)) return r as BoundPair<K>;
	if (inclusive(r)) return [rangeInclusiveStartBound(r), rangeInclusiveEndBound(r)];
	const start: RangeBound<K> = "start" in r ? { tag: "Included", _0: r.start } : UNBOUNDED;
	if (!("end" in r)) return [start, UNBOUNDED];
	return [start, { tag: "inclusive" in r ? "Included" : "Excluded", _0: r.end }];
}

/**
 * `BTreeMap::range` and `range_mut`: the entries within `r`, or within the range of another
 * type a `By` holds. A key of the crate's own gives its `PartialEq::eq` and
 * `PartialOrd::gt`, which std checks the ends by.
 */
export function btreeRange<K, V>(m: BTreeMap<K, V>, r: NoInfer<Bounds<Key<K>>>, eq?: Test<K>, gt?: Test<K>): Iter<[K, V]>;
export function btreeRange<K, V, Q>(m: BTreeMap<K, V>, r: By<K>, eq?: Test<Q>, gt?: Test<Q>): Iter<[K, V]>;
export function btreeRange<K, V>(m: BTreeMap<K, V>, r: Bounds<K> | By<K>, eq?: Test<unknown>, gt?: Test<unknown>): Iter<[K, V]> {
	return range(m, r, ENTRIES, "BTreeMap", eq, gt);
}

/** `range_mut` of values JS copies: each key within `r` and the place of its value. */
export function btreeRangeMut<K, V>(m: BTreeMap<K, V>, r: NoInfer<Bounds<Key<K>>>, eq?: Test<K>, gt?: Test<K>): Iter<[K, Place<V>]>;
export function btreeRangeMut<K, V, Q>(m: BTreeMap<K, V>, r: By<K>, eq?: Test<Q>, gt?: Test<Q>): Iter<[K, Place<V>]>;
export function btreeRangeMut<K, V>(m: BTreeMap<K, V>, r: Bounds<K> | By<K>, eq?: Test<unknown>, gt?: Test<unknown>): Iter<[K, Place<V>]> {
	return range(m, r, PLACES, "BTreeMap", eq, gt);
}

/** `BTreeSet::range`: the keys within `r`. */
export function btreeRangeKeys<K, V>(m: BTreeMap<K, V>, r: NoInfer<Bounds<Key<K>>>, eq?: Test<K>, gt?: Test<K>): Iter<K>;
export function btreeRangeKeys<K, V, Q>(m: BTreeMap<K, V>, r: By<K>, eq?: Test<Q>, gt?: Test<Q>): Iter<K>;
export function btreeRangeKeys<K, V>(m: BTreeMap<K, V>, r: Bounds<K> | By<K>, eq?: Test<unknown>, gt?: Test<unknown>): Iter<K> {
	return range(m, r, KEYS, "BTreeSet", eq, gt);
}

/**
 * What a range of keys `K` is bounded by: a key, or for an array's or a `Vec`'s, a `Slice`,
 * which `cmpSlice` orders alike, or for a `String`'s or a `Cow<str>`'s, a `str`, which
 * `cmpStr` orders alike.
 */
type Key<K> = K | (K extends (infer E)[] ? Slice<E> : never) | (K extends RsString | Borrowed<string> ? string : never);

/**
 * The items of `m` within `r`, checked as std checks it: by `eq` and `gt`, or by the order
 * of its keys, `m`'s or a `By`'s. Its ends are found as std's
 * `find_leaf_edges_spanning_range` finds them: down one path while both lie below the same
 * edge, then each down its own.
 */
function range<K, V, T>(m: BTreeMap<K, V>, r: Bounds<K> | By<K>, pick: Pick, name: string, eq?: Test<unknown>, gt?: Test<unknown>): Iter<T> {
	if (!m.r) return new BTreeIter<K, V, T>(pick, m.t, 0, m.t, 0);
	const by = r instanceof By ? r : undefined;
	const order = (by?.cmp ?? m.c) as (a: unknown, b: unknown) => Ordering;
	const cmp = (by?.c ?? m.c) as (a: unknown, b: K) => Ordering;
	const [start, stop] = bounds((by === undefined ? r : by.q) as Bounds<unknown>);
	if (start.tag !== "Unbounded" && stop.tag !== "Unbounded") {
		const s = start._0;
		const e = stop._0;
		if (start.tag === "Excluded" && stop.tag === "Excluded" && (eq === undefined ? order(s, e) === 0 : eq(s, e))) {
			panic(`range start and end are equal and excluded in ${name}`, `btree_range_equal_${name}`);
		}
		if (gt === undefined ? order(s, e) > 0 : gt(s, e)) panic(`range start is greater than range end in ${name}`, `btree_range_order_${name}`);
	}
	const lo = side(start);
	const hi = side(stop);
	for (let n = m.t; ; ) {
		const fi = lower(cmp, n, lo);
		const bi = upper(cmp, n, hi, fi);
		if (fi < bi) {
			let fl = n;
			let bl = n;
			let f = fi;
			let b = bi;
			while (fl.e !== undefined) {
				fl = fl.e[f];
				f = lower(cmp, fl, lo);
				bl = (bl.e as Node<K, V>[])[b];
				b = upper(cmp, bl, hi, 0);
			}
			return new BTreeIter<K, V, T>(pick, fl, f, bl, b);
		}
		if (n.e === undefined) return new BTreeIter<K, V, T>(pick, n, fi, n, fi);
		n = n.e[fi];
	}
}

/** std's `SearchBound`s, of a range's end as its search narrows it. */
const INCLUDED = 0;
const EXCLUDED = 1;
const ALL_INCLUDED = 2;
const ALL_EXCLUDED = 3;

/** One end of a range as std's search narrows it: a `SearchBound` and, where it has one, its key. */
interface Side<K> {
	tag: number;
	key: K | undefined;
}

function side<K>(b: RangeBound<K>): Side<K> {
	if (b.tag === "Unbounded") return { tag: ALL_INCLUDED, key: undefined };
	return { tag: b.tag === "Included" ? INCLUDED : EXCLUDED, key: b._0 };
}

/** std's `find_lower_bound_index`: the edge of `n` a range from `s` starts at, narrowing `s` for the child below it. */
function lower<Q, K, V>(cmp: (a: Q, b: K) => Ordering, n: Node<K, V>, s: Side<Q>): number {
	if (s.tag === ALL_INCLUDED) return 0;
	if (s.tag === ALL_EXCLUDED) return n.k.length;
	const i = find(cmp, n, s.key as Q);
	if (i < 0) return ~i;
	if (s.tag === INCLUDED) {
		s.tag = ALL_EXCLUDED;
		return i;
	}
	s.tag = ALL_INCLUDED;
	return i + 1;
}

/** std's `find_upper_bound_index`: the edge of `n` from `from` on that a range to `s` ends at, narrowing `s` likewise. */
function upper<Q, K, V>(cmp: (a: Q, b: K) => Ordering, n: Node<K, V>, s: Side<Q>, from: number): number {
	if (s.tag === ALL_INCLUDED) return n.k.length;
	if (s.tag === ALL_EXCLUDED) return from;
	const i = find(cmp, n, s.key as Q, from);
	if (i < 0) return ~i;
	if (s.tag === INCLUDED) {
		s.tag = ALL_EXCLUDED;
		return i + 1;
	}
	s.tag = ALL_INCLUDED;
	return i;
}

/**
 * `extract_if` of a B-tree: from the first key `r` starts at, each entry `f` returns true
 * for, taken out as it is read, in key order, as `[key, value]`, or of a set its key, up to
 * the first key past `r`'s end, which `r` is read for at each key and found past by `le` and
 * `lt`, a key of the crate's own's `PartialOrd`, or else by the map's order. `f` is given
 * each key and value or, with `places`, the value's place, and drops with the iterator,
 * then `r`, by `drop`; what it has not read stays.
 */
class BTreeExtractIf<K, V, T> extends Iter<T> implements Disposable {
	private readonly m: BTreeMap<K, V>;
	/** std's cursor: the leaf edge before the entry it reads next, until it stops. */
	l: Node<K, V> | undefined;
	i = 0;
	private readonly r: Bounds<K>;
	private readonly f: (key: K, value: V | Place<V>) => boolean;
	private readonly places: boolean;
	private readonly keys: boolean;
	private readonly le: Test<K> | undefined;
	private readonly lt: Test<K> | undefined;
	private readonly drop: (() => void) | undefined;

	constructor(m: BTreeMap<K, V>, r: Bounds<K>, f: (key: K, value: V | Place<V>) => boolean, places: boolean, keys: boolean, le?: Test<K>, lt?: Test<K>, drop?: () => void) {
		super();
		this.m = m;
		this.r = r;
		this.f = f;
		this.places = places;
		this.keys = keys;
		this.le = le;
		this.lt = lt;
		this.drop = drop;
		// std's reads the start only of a map with its root node.
		if (!m.r) return;
		const s = side(other(r) ? r.startBound() : bounds(r)[0]);
		let n = m.t;
		for (let i = lower(m.c, n, s); ; i = lower(m.c, n, s)) {
			if (n.e === undefined) {
				this.l = n;
				this.i = i;
				return;
			}
			n = n.e[i];
		}
	}

	/** Whether `key` is within the end of `r`. */
	private within(key: K): boolean {
		const r = this.r;
		const stop = other(r) ? r.endBound() : bounds(r)[1];
		if (stop.tag === "Unbounded") return true;
		if (stop.tag === "Included") return this.le !== undefined ? this.le(key, stop._0) : this.m.c(key, stop._0) <= 0;
		return this.lt !== undefined ? this.lt(key, stop._0) : this.m.c(key, stop._0) < 0;
	}

	next(): T | Done {
		for (let n = this.l; n !== undefined; n = this.l) {
			let i = this.i;
			// Stopped where it ends, past the end of `r`, or by a panic, as std's is.
			this.l = undefined;
			while (i === n.k.length) {
				if (n.p === undefined) return DONE;
				i = n.i;
				n = n.p;
			}
			const key = n.k[i];
			if (!this.within(key)) return DONE;
			if (!this.f(key, this.places ? new AtIndex(n.v, i) : n.v[i])) {
				this.l = nextEdge(n, i);
				this.i = idx;
				continue;
			}
			const value = n.v[i];
			this.l = removeAt(this.m, n, i);
			this.i = idx;
			return (this.keys ? key : [key, value]) as T;
		}
		return DONE;
	}

	/** std's, which counts what it read and kept, too. */
	override sizeHint(): SizeHint {
		return [0, this.m.n];
	}

	[Symbol.dispose](): void {
		dispose(this.f);
		this.drop?.();
	}
}

/** `Debug` of a B-tree's `ExtractIf`: the entry it reads next, if any, or of a set, with no `value`, its key. */
export function fmtBTreeExtractIf<K, V>(it: Iter<unknown>, f: Formatter, key: Debug<K>, value?: Debug<V>): FmtResult {
	let { l: n, i } = it as BTreeExtractIf<K, V, unknown>;
	while (n !== undefined && i === n.k.length) {
		i = n.i;
		n = n.p;
	}
	let peek: Fmt = (f) => f.out.writeStr("None");
	if (n !== undefined) {
		const kv = n;
		const k: Fmt = (f) => key(kv.k[i], f);
		const both: Fmt = value === undefined ? k : (f) => fmtTuple([k, (f) => value(kv.v[i], f)], f);
		peek = (f) => fmtDebugTupleFields(f, "Some", [both]);
	}
	return debugStructFinishNonExhaustive(debugStructField(fmtDebugStruct(f, "ExtractIf"), "peek", peek));
}

/**
 * `BTreeMap::extract_if`, handing `f` each key and value, with what `BTreeExtractIf` takes:
 * a key of the crate's own's `le` and `lt`, and what drops a range that drops.
 */
export function btreeExtractIf<K, V>(m: BTreeMap<K, V>, r: Bounds<K>, f: (key: K, value: V) => boolean, le?: Test<K>, lt?: Test<K>, drop?: () => void): Iter<[K, V]> {
	return new BTreeExtractIf(m, r, f as (key: K, value: V | Place<V>) => boolean, false, false, le, lt, drop);
}

/** `BTreeMap::extract_if` of values JS copies: `f` takes the place of each. */
export function btreeExtractIfMut<K, V>(m: BTreeMap<K, V>, r: Bounds<K>, f: (key: K, value: Place<V>) => boolean, le?: Test<K>, lt?: Test<K>, drop?: () => void): Iter<[K, V]> {
	return new BTreeExtractIf(m, r, f as (key: K, value: V | Place<V>) => boolean, true, false, le, lt, drop);
}

/** `BTreeSet::extract_if`, handing `f` each key. */
export function btreeExtractKeysIf<K>(m: BTreeSet<K>, r: Bounds<K>, f: (key: K) => boolean, le?: Test<K>, lt?: Test<K>, drop?: () => void): Iter<K> {
	return new BTreeExtractIf(m, r, f as (key: K) => boolean, false, true, le, lt, drop);
}

/**
 * `extend` of a map with `[key, value]`s, which keeps an equal key already there and drops
 * the one given, then the value it replaced, and takes `items` as `take` says. With
 * `last`, `collect` and `from` of entries that drop nothing into an empty one, which keep
 * the last of equal keys, as `btreeCollect` does, and reserve as the `Vec` of `size`-byte
 * entries std's collects into first does, as `iterExtend`.
 */
export function btreeExtend<K, V>(
	m: BTreeMap<K, V>,
	items: Iter<[K, V]>,
	last = false,
	k?: (key: K) => void,
	v?: (value: V) => void,
	take?: Take,
	size?: number,
	trusted = false): BTreeMap<K, V> {
	const add = (item: [K, V]) => {
		if (!last) {
			const old = btreeInsert(m, item[0], item[1], k);
			if (old !== undefined) v?.(old);
			return;
		}
		const n = seek(m, item[0]);
		const i = idx;
		if (i < 0) insertAt(m, n, ~i, item[0], item[1]);
		else {
			n.k[i] = item[0];
			n.v[i] = item[1];
		}
	};
	return vecExtending(m, 0, items, add, take, size, trusted);
}

/** `extend` of a set with keys, and with `last`, `collect` and `from`, as `btreeExtend`. */
export function btreeExtendKeys<K>(m: BTreeSet<K>, keys: Iter<K>, last = false, k?: (key: K) => void, take?: Take, size?: number, trusted = false): BTreeSet<K> {
	const add = (key: K) => {
		if (!last) {
			btreeAdd(m, key, k);
			return;
		}
		const n = seek(m, key);
		const i = idx;
		if (i < 0) insertAt(m, n, ~i, key, undefined);
		else n.k[i] = key;
	};
	return vecExtending(m, 0, keys, add, take, size, trusted);
}

/**
 * `collect` and `from` into `m`, an empty map, of `entries`, which std's collects into a
 * `Vec` first. It sorts them as std's stable sort does by `Ord::cmp` of their keys, of
 * entries `size` bytes long on wasm32 and `Freeze` if `freeze`, and builds the tree of each but those whose key `same`,
 * `!ne`, finds equal to the next one's, which drop in order. Without `same`, the keys'
 * impls agree, so JS's sort and `Ord` alone build the same tree faster.
 */
export function btreeCollect<K, V>(m: BTreeMap<K, V>, entries: [K, V][], size: number, freeze: boolean, same?: Test<K>, k?: (key: K) => void, v?: (value: V) => void): BTreeMap<K, V> {
	if (same === undefined) entries.sort((a, b) => m.c(a[0], b[0]));
	else sliceSort(sliceFrom(entries), (a, b) => m.c(a[0], b[0]) === -1, size, freeze);
	const keys: K[] = [];
	const values: V[] = [];
	const n = entries.length;
	for (let i = 0; i < n; i++) {
		const [key, value] = entries[i];
		if (i + 1 === n || (same === undefined ? m.c(key, entries[i + 1][0]) !== 0 : !same(key, entries[i + 1][0]))) {
			keys.push(key);
			values.push(value);
		} else dropPair(key, value, k, v);
	}
	bulk(m, keys, values);
	// std's makes no root of none.
	m.r = n > 0;
	return m;
}

/**
 * `collect` and `from` into `m`, an empty set, of `keys`, as `btreeCollect`, but sorted by
 * `less`, their `PartialOrd::lt`, or not at all if zero-sized.
 */
export function btreeCollectKeys<K>(m: BTreeSet<K>, keys: K[], size: number, freeze: boolean, less?: Test<K>, same?: Test<K>, k?: (key: K) => void): BTreeSet<K> {
	if (same === undefined) {
		return btreeCollect(
			m,
			keys.map((key) => [key, undefined]),
			size,
			freeze,
			undefined,
			k,
		);
	}
	if (less !== undefined) sliceSort(sliceFrom(keys), less, size, freeze);
	btreeFromSorted(m, iterSlice(keys), same, k);
	m.r = keys.length > 0;
	return m;
}


/**
 * `clone`, of std's shape, node for node, with each key and value through its `Clone`, or as
 * itself without one, in key order, each key before its value, as std's clones them.
 */
export function btreeClone<K, V>(m: BTreeMap<K, V>, key: (k: K) => K = itself, value: (v: V) => V = itself): BTreeMap<K, V> {
	const copy = (n: Node<K, V>): Node<K, V> => {
		const e = n.e === undefined ? undefined : [copy(n.e[0])];
		const out = node(e);
		for (let i = 0; i < n.k.length; i++) {
			out.k.push(key(n.k[i]));
			out.v.push(value(n.v[i]));
			if (e !== undefined) e.push(copy((n.e as Node<K, V>[])[i + 1]));
		}
		if (e !== undefined) adopt(out);
		return out;
	};
	const out = btreeNew<K, V>(m.c);
	// std's clones an empty map as a new one, with no root.
	if (m.n === 0) return out;
	out.t = copy(m.t);
	out.h = m.h;
	out.n = m.n;
	out.r = true;
	return out;
}

// Set operations of B-trees, as std's: each merges both sets' keys in order, or of sets
// whose sizes differ by `TIPPING` times or more, looks up each of the smaller's in the
// larger, which picks which set's key of two equal ones it yields.

/** std's `ITER_PERFORMANCE_TIPPING_SIZE_DIFF`. */
const TIPPING = 16;

/** The first and last keys of `m`, which holds some. */
function ends<K>(m: BTreeSet<K>): [K, K] {
	return [btreeFirstKey(m) as K, btreeFirstKey(m, true) as K];
}

/**
 * std's `MergeIterInner`: the keys of `a` and `b` in order, a step at a time, holding back
 * the greater of two that differ.
 */
abstract class Merge<K> extends Iter<K> {
	readonly a: Iter<K>;
	readonly b: Iter<K>;
	private readonly c: (a: K, b: K) => Ordering;
	/** The key held back, and whether it is `a`'s. */
	held: K | Done = DONE;
	heldA = false;

	constructor(a: Iter<K>, b: Iter<K>, c: (a: K, b: K) => Ordering) {
		super();
		this.a = a;
		this.b = b;
		this.c = c;
	}

	/** std's: each of its iterators, and the key it holds back. */
	override clone(): Iter<K> {
		const kind = this.constructor as new (a: Iter<K>, b: Iter<K>, c: (a: K, b: K) => Ordering) => Merge<K>;
		const it = new kind(this.a.clone(), this.b.clone(), this.c);
		it.held = this.held;
		it.heldA = this.heldA;
		return it;
	}

	/** The next key of each, or `DONE` for one that ended or is held back. */
	protected nexts(): [K | Done, K | Done] {
		const held = this.held;
		this.held = DONE;
		const x = held !== DONE && this.heldA ? held : this.a.next();
		const y = held !== DONE && !this.heldA ? held : this.b.next();
		if (x === DONE || y === DONE) return [x, y];
		const order = this.c(x, y);
		if (order === 0) return [x, y];
		this.held = order < 0 ? y : x;
		this.heldA = order > 0;
		return order < 0 ? [x, DONE] : [DONE, y];
	}

	/** How many keys each has left, the one held back included. */
	protected lens(): [number, number] {
		const held = this.held === DONE ? 0 : 1;
		return [this.a.len() + (this.heldA ? held : 0), this.b.len() + (this.heldA ? 0 : held)];
	}
}

class Union<K> extends Merge<K> {
	next(): K | Done {
		const [x, y] = this.nexts();
		return x !== DONE ? x : y;
	}

	override sizeHint(): SizeHint {
		const [a, b] = this.lens();
		return [Math.max(a, b), a + b];
	}
}

class SymmetricDifference<K> extends Merge<K> {
	next(): K | Done {
		for (;;) {
			const [x, y] = this.nexts();
			if (x === DONE || y === DONE) return x !== DONE ? x : y;
		}
	}

	override sizeHint(): SizeHint {
		const [a, b] = this.lens();
		return [0, a + b];
	}
}

/** `BTreeSet::union`: each key of either in order, `a`'s of two equal ones. */
export function btreeUnion<K>(a: BTreeSet<K>, b: BTreeSet<K>): Iter<K> {
	return new Union(btreeKeys(a), btreeKeys(b), a.c);
}

/** `BTreeSet::symmetric_difference`: each key of one that the other lacks, in order. */
export function btreeSymmetricDifference<K>(a: BTreeSet<K>, b: BTreeSet<K>): Iter<K> {
	return new SymmetricDifference(btreeKeys(a), btreeKeys(b), a.c);
}

/**
 * A set operation that looks each key of `small` up in `large`: an intersection's, yielding
 * those `large` holds if `holds`, or a difference's, those it lacks.
 */
class Search<K> extends Iter<K> {
	readonly small: Iter<K>;
	readonly large: BTreeSet<K>;
	private readonly holds: boolean;

	constructor(small: Iter<K>, large: BTreeSet<K>, holds: boolean) {
		super();
		this.small = small;
		this.large = large;
		this.holds = holds;
	}

	override clone(): Iter<K> {
		return new Search(this.small.clone(), this.large, this.holds);
	}

	next(): K | Done {
		for (let x = this.small.next(); x !== DONE; x = this.small.next()) {
			if (btreeContains(this.large, x) === this.holds) return x;
		}
		return DONE;
	}

	override sizeHint(): SizeHint {
		const n = this.small.len();
		return [this.holds ? 0 : Math.max(n - this.large.n, 0), n];
	}
}

/** Each key of `a` that the other set lacks, found by stepping through its keys by `b`, peeked a key ahead. */
class Difference<K> extends Iter<K> {
	readonly a: Iter<K>;
	readonly b: Iter<K>;
	private readonly c: (a: K, b: K) => Ordering;
	/** `b`'s next key, once `peeked` says it was read. */
	peek: K | Done = DONE;
	peeked = false;

	constructor(a: Iter<K>, b: Iter<K>, c: (a: K, b: K) => Ordering) {
		super();
		this.a = a;
		this.b = b;
		this.c = c;
	}

	/** std's: each of its iterators, and the key it peeked. */
	override clone(): Iter<K> {
		const it = new Difference(this.a.clone(), this.b.clone(), this.c);
		it.peek = this.peek;
		it.peeked = this.peeked;
		return it;
	}

	next(): K | Done {
		let x = this.a.next();
		if (x === DONE) return DONE;
		for (;;) {
			if (!this.peeked) {
				this.peek = this.b.next();
				this.peeked = true;
			}
			const order = this.peek === DONE ? -1 : this.c(x, this.peek);
			if (order < 0) return x;
			if (order === 0) {
				x = this.a.next();
				if (x === DONE) return DONE;
			}
			this.peeked = false;
		}
	}

	override sizeHint(): SizeHint {
		const n = this.a.len();
		const other = this.b.len() + (this.peeked && this.peek !== DONE ? 1 : 0);
		return [Math.max(n - other, 0), n];
	}
}

/** `BTreeSet::difference`: each key of `a` that `b` lacks, in order. */
export function btreeDifference<K>(a: BTreeSet<K>, b: BTreeSet<K>): Iter<K> {
	const keys = btreeKeys(a);
	if (a.n === 0 || b.n === 0) return keys;
	const [aMin, aMax] = ends(a);
	const [bMin, bMax] = ends(b);
	const low = a.c(aMin, bMax);
	const high = a.c(aMax, bMin);
	if (low > 0 || high < 0) return keys;
	if (low === 0) keys.next();
	else if (high === 0) keys.nextBack();
	else if (a.n <= Math.floor(b.n / TIPPING)) return new Search(keys, b, false);
	else return new Difference(keys, btreeKeys(b), b.c);
	return keys;
}

/** An intersection that merges: each key of `a` that `b` holds. */
class Intersection<K> extends Iter<K> {
	readonly a: Iter<K>;
	readonly b: Iter<K>;
	private readonly c: (a: K, b: K) => Ordering;

	constructor(a: Iter<K>, b: Iter<K>, c: (a: K, b: K) => Ordering) {
		super();
		this.a = a;
		this.b = b;
		this.c = c;
	}

	override clone(): Iter<K> {
		return new Intersection(this.a.clone(), this.b.clone(), this.c);
	}

	next(): K | Done {
		let x = this.a.next();
		if (x === DONE) return DONE;
		let y = this.b.next();
		if (y === DONE) return DONE;
		for (;;) {
			const order = this.c(x, y);
			if (order === 0) return x;
			if (order < 0) x = this.a.next();
			else y = this.b.next();
			if (x === DONE || y === DONE) return DONE;
		}
	}

	override sizeHint(): SizeHint {
		return [0, Math.min(this.a.len(), this.b.len())];
	}
}

/**
 * `BTreeSet::intersection`: each key of both in order, `a`'s of two equal ones, unless
 * `b` is the smaller by `TIPPING` times, which has its own looked up in `a`.
 */
export function btreeIntersection<K>(a: BTreeSet<K>, b: BTreeSet<K>): Iter<K> {
	if (a.n === 0 || b.n === 0) return iterEmpty();
	const [aMin, aMax] = ends(a);
	const [bMin, bMax] = ends(b);
	const low = a.c(aMin, bMax);
	const high = a.c(aMax, bMin);
	if (low > 0 || high < 0) return iterEmpty();
	if (low === 0) return iterOnce(aMin);
	if (high === 0) return iterOnce(aMax);
	if (a.n <= Math.floor(b.n / TIPPING)) return new Search(btreeKeys(a), b, true);
	if (b.n <= Math.floor(a.n / TIPPING)) return new Search(btreeKeys(b), a, true);
	return new Intersection(btreeKeys(a), btreeKeys(b), a.c);
}

/**
 * `Debug` of a `BTreeSet`'s `Difference`, `Intersection`, `Union`, or
 * `SymmetricDifference`, `name`, each key by `debug`: the way std picked to make it, by
 * the iterators it reads, a set it looks keys up in, and the key it peeked or holds back.
 */
export function fmtBTreeSetOp<K>(it: Iter<unknown>, f: Formatter, debug: Debug<K>, name: string, at?: Location): FmtResult {
	const keys =
		(it: Iter<unknown>): Fmt =>
		(f) =>
			fmtIter(it, f, debug, ["Iter"], at);
	const none: Fmt = (f) => f.out.writeStr("None");
	const some =
		(value: Fmt): Fmt =>
		(f) =>
			fmtDebugTupleFields(f, "Some", [value]);
	const key =
		(key: K): Fmt =>
		(f) =>
			debug(key, f);
	let inner: Fmt;
	if (it instanceof Merge) {
		const held = it.held as K;
		const peeked = it.held === DONE ? none : some((f) => fmtDebugTupleFields(f, it.heldA ? "A" : "B", [key(held)]));
		inner = (f) => fmtDebugTupleFields(f, "MergeIterInner", [keys(it.a), keys(it.b), peeked]);
	} else if (it instanceof Search) {
		const set: Fmt = (f) => fmtSet(btreeKeys(it.large), f, debug);
		const names = name === "Difference" ? ["self_iter", "other_iter"] : ["small_iter", "large_set"];
		inner = (f) => fmtDebugStructFields(f, "Search", names, [keys(it.small), set]);
	} else if (it instanceof Difference) {
		const peeked = !it.peeked ? none : some(it.peek === DONE ? none : some(key(it.peek as K)));
		const other: Fmt = (f) => fmtDebugStructFields(f, "Peekable", ["iter", "peeked"], [keys(it.b), peeked]);
		inner = (f) => fmtDebugStructFields(f, "Stitch", ["self_iter", "other_iter"], [keys(it.a), other]);
	} else if (it instanceof Intersection) {
		inner = (f) => fmtDebugStructFields(f, "Stitch", ["a", "b"], [keys(it.a), keys(it.b)]);
	} else if (name === "Difference") {
		inner = (f) => fmtDebugTupleFields(f, "Iterate", [keys(it)]);
	} else {
		const left = [...it.remaining()] as K[];
		inner = (f) => fmtDebugTupleFields(f, "Answer", [left.length === 0 ? none : some(key(left[0]))]);
	}
	return fmtDebugTupleFields(f, name, [inner]);
}

/** `BTreeSet::is_subset`: whether `b` holds each key of `a`. */
export function btreeIsSubset<K>(a: BTreeSet<K>, b: BTreeSet<K>): boolean {
	if (a.n > b.n) return false;
	if (a.n === 0) return true;
	if (b.n === 0) return false;
	const [aMin, aMax] = ends(a);
	const [bMin, bMax] = ends(b);
	const keys = btreeKeys(a);
	const low = a.c(aMin, bMin);
	if (low < 0) return false;
	if (low === 0) keys.next();
	const high = a.c(aMax, bMax);
	if (high > 0) return false;
	if (high === 0) keys.nextBack();
	if (keys.len() <= Math.floor(b.n / TIPPING)) {
		for (const key of keys) if (!btreeContains(b, key)) return false;
		return true;
	}
	// Each of `a`'s found by stepping through `b`'s between their ends, which `a`'s left lack.
	const others = btreeKeys(b);
	others.next();
	others.nextBack();
	for (const key of keys) {
		let order: Ordering = 1;
		for (let other = others.next(); other !== DONE; other = others.next()) {
			order = a.c(other, key);
			if (order >= 0) break;
		}
		if (order !== 0) return false;
	}
	return true;
}

/** `BTreeSet::is_superset`: whether `a` holds each key of `b`. */
export function btreeIsSuperset<K>(a: BTreeSet<K>, b: BTreeSet<K>): boolean {
	return btreeIsSubset(b, a);
}

/** `BTreeSet::is_disjoint`: whether their intersection yields nothing. */
export function btreeIsDisjoint<K>(a: BTreeSet<K>, b: BTreeSet<K>): boolean {
	return btreeIntersection(a, b).next() === DONE;
}

/**
 * std's sorted bulk build of `m`, an empty set, which its operators make: of `keys`, in
 * order, but for each that `same`, `!ne`, finds equal to the next, which drops. It reads
 * the next key before it tests one, as std's peeks. Without `same`, the keys are distinct.
 */
export function btreeFromSorted<K>(m: BTreeSet<K>, keys: Iter<K>, same?: Test<K>, k?: (key: K) => void): BTreeSet<K> {
	let held: K[];
	if (same === undefined) held = [...keys];
	else {
		held = [];
		for (let key = keys.next(); key !== DONE; ) {
			const next = keys.next();
			if (next === DONE || !same(key, next)) held.push(key);
			else k?.(key);
			key = next;
		}
	}
	bulk(
		m,
		held,
		held.map(() => undefined),
	);
	// std's bulk build makes the root even of no keys.
	m.r = true;
	return m;
}

// Entries

/**
 * `hash_map::Entry` and `btree_map::Entry`, and their `OccupiedEntry` and `VacantEntry`:
 * the map, the key, and where `entry` found it. The map stays as it was until the entry is
 * consumed, so nothing compares or hashes the key again, as nothing does in Rust.
 */
export interface Entry<K, V> {
	readonly m: HashMap<K, V> | BTreeMap<K, V>;
	/** The key it was given, or the map's own if it is occupied. */
	readonly k: K;
	/** Whether it is occupied: whether the map held the key. */
	readonly o: boolean;
	/** A hashed key's slot if it is occupied, and its hash. */
	readonly s: Slot<K, V> | undefined;
	readonly h: HashKey | undefined;
	/** A B-tree's node holding the key, and its index there, or else the leaf and the complement of where it goes. */
	readonly l: Node<K, V> | undefined;
	readonly i: number;
}

/**
 * `entry`: `key` drops at once if the map holds an equal one. A hashed key is hashed into
 * an empty map too, as std's is.
 */
export function entry<K, V>(m: HashMap<K, V> | BTreeMap<K, V>, key: K, k?: (key: K) => void): Entry<K, V> {
	const e = locate(m, key);
	if (e.o) k?.(key);
	return e;
}

function locate<K, V>(m: HashMap<K, V> | BTreeMap<K, V>, key: K): Entry<K, V> {
	if (!isHash(m)) {
		const l = seek(m, key);
		const i = idx;
		return { m, k: i < 0 ? key : l.k[i], o: i >= 0, s: undefined, h: undefined, l, i };
	}
	if (!hashed(m, key)) return { m, k: key, o: m.p.has(key), s: undefined, h: undefined, l: undefined, i: 0 };
	const h = (m.h as Hashing<K>).hash(key);
	const s = slot(m, key, h);
	return { m, k: s === undefined ? key : s.k, o: s !== undefined, s, h, l: undefined, i: 0 };
}

/** Insert `value` where `e`, a vacant entry, goes: its place. */
function fill<K, V>(e: Entry<K, V>, value: V): Place<V> {
	const m = e.m;
	if (isHash(m)) {
		if (e.h !== undefined) return add(m, e.k, value, e.h);
		m.p.set(e.k, value);
		return new AtKey(m.p, e.k);
	}
	const l = insertAt(m, e.l as Node<K, V>, ~e.i, e.k, value);
	return new AtIndex(l.v, idx);
}

/** Drop `e`, an `Entry`: a vacant one holds the key it was given. */
export function entryDrop<K, V>(e: Entry<K, V>, k: (key: K) => void): void {
	if (!e.o) k(e.k);
}

function isHash<K, V>(m: HashMap<K, V> | BTreeMap<K, V>): m is HashMap<K, V> {
	return "p" in m;
}

/** `Entry::or_insert_with`: the value there, and `f` drops by `dropF` uncalled, or else `f()`'s, inserted. */
export function entryOrInsertWith<K, V>(e: Entry<K, V>, f: (key: K) => V, dropF?: (f: (key: K) => V) => void): V {
	if (e.o) {
		const value = entryGet(e);
		dropF?.(f);
		return value;
	}
	const value = f(e.k);
	fill(e, value);
	return value;
}

/** `Entry::or_insert`: the value there, and `value` drops, or else `value`, inserted. */
export function entryOrInsert<K, V>(e: Entry<K, V>, value: V, v?: (value: V) => void): V {
	if (!e.o) {
		fill(e, value);
		return value;
	}
	v?.(value);
	return entryGet(e);
}

/** Whether `e` is `Entry::Occupied`: whether the map holds its key. */
export function entryOccupied<K, V>(e: Entry<K, V>): boolean {
	return e.o;
}

/** `Entry::and_modify` of a value JS copies: `f` of its place, if any, as `entryAndModify`. */
export function entryAndModifyMut<K, V>(e: Entry<K, V>, f: (value: Place<V>) => void, dropF?: (f: (value: Place<V>) => void) => void): Entry<K, V> {
	if (e.o) f(entryGetMut(e));
	else dropF?.(f);
	return e;
}

/** `Entry::and_modify`: `f` of the value there, if any, or else `f` drops by `dropF` uncalled. */
export function entryAndModify<K, V>(e: Entry<K, V>, f: (value: V) => void, dropF?: (f: (value: V) => void) => void): Entry<K, V> {
	if (e.o) f(entryGet(e));
	else dropF?.(f);
	return e;
}

/** `Entry::key` and `OccupiedEntry::key`: the key the map holds, or else the one looked up. */
export function entryKey<K, V>(e: Entry<K, V>): K {
	return e.k;
}

/** `OccupiedEntry::get` and `get_mut`. */
export function entryGet<K, V>(e: Entry<K, V>): V {
	if (e.s !== undefined) return e.s.v;
	if (e.l !== undefined) return e.l.v[e.i];
	return (e.m as HashMap<K, V>).p.get(e.k) as V;
}

/** `OccupiedEntry::get_mut` and `into_mut` of a value JS copies: its place. */
export function entryGetMut<K, V>(e: Entry<K, V>): Place<V> {
	if (e.s !== undefined) return e.s;
	if (e.l !== undefined) return new AtIndex(e.l.v, e.i);
	return new AtKey((e.m as HashMap<K, V>).p, e.k);
}

/**
 * `Entry::or_insert_with` and `or_insert_with_key` of a value JS copies, and
 * `or_default`: its place, after inserting `f`'s value if the map lacks the key, as
 * `entryOrInsertWith`.
 */
export function entryOrInsertWithMut<K, V>(e: Entry<K, V>, f: (key: K) => V, dropF?: (f: (key: K) => V) => void): Place<V> {
	if (!e.o) return fill(e, f(e.k));
	const place = entryGetMut(e);
	dropF?.(f);
	return place;
}

/** `Entry::or_insert` and `VacantEntry::insert` of a value JS copies: its place, and `value` drops if one was there. */
export function entryOrInsertMut<K, V>(e: Entry<K, V>, value: V, v?: (value: V) => void): Place<V> {
	if (!e.o) return fill(e, value);
	const place = entryGetMut(e);
	v?.(value);
	return place;
}

/** `OccupiedEntry::insert`: the old value. */
export function entryInsert<K, V>(e: Entry<K, V>, value: V): V {
	const place = entryGetMut(e);
	const old = place.v;
	place.v = value;
	return old;
}

/** `OccupiedEntry::remove_entry`. */
export function entryRemoveEntry<K, V>(e: Entry<K, V>): [K, V] {
	const m = e.m;
	if (isHash(m)) hold(m);
	const value = entryGet(e);
	if (!isHash(m)) removeAt(m, e.l as Node<K, V>, e.i);
	else if (e.s !== undefined) unlink(m, e.h as number, e.s);
	else m.p.delete(e.k);
	return [e.k, value];
}

/** `OccupiedEntry::remove`, whose key drops. */
export function entryRemove<K, V>(e: Entry<K, V>, k?: (key: K) => void): V {
	const [key, value] = entryRemoveEntry(e);
	k?.(key);
	return value;
}

/** `first_entry`, and with `last`, `last_entry`: an occupied entry, if the map has one. */
export function btreeFirstEntry<K, V>(m: BTreeMap<K, V>, last = false): Entry<K, V> | undefined {
	if (m.n === 0) return undefined;
	const l = end(m, last);
	const i = last ? l.k.length - 1 : 0;
	return { m, k: l.k[i], o: true, s: undefined, h: undefined, l, i };
}

// VecDeque

/** `usize::MAX` on wasm32, the capacity of a deque of zero-sized items. */
const USIZE_MAX = 0xffffffff;

/**
 * `VecDeque<T>`: a ring buffer of `b.length` slots, std's capacity, holding `n` items of
 * `s` bytes each from `h` on, wrapping past the end. Each step places them where std's
 * does, which `as_slices` shows: a deque of bytes is a `Uint8Array`, which its slices view,
 * and one of zero-sized items has `usize::MAX` slots, as std's.
 */
export interface VecDeque<T> {
	b: (T | undefined)[];
	h: number;
	n: number;
	s: number;
}

/** `cap` empty slots, bytes if `bytes`. */
function slots<T>(cap: number, bytes: boolean): (T | undefined)[] {
	return bytes ? (new Uint8Array(cap) as unknown as T[]) : new Array<T | undefined>(cap);
}

/** Whether `q` holds bytes in a `Uint8Array`. */
function holdsBytes<T>(q: VecDeque<T>): boolean {
	return q.b instanceof Uint8Array;
}

/** `VecDeque::new` of `size`-byte items, held as bytes if `bytes`. */
export function dequeNew<T>(size: number, bytes = false): VecDeque<T> {
	return { b: slots(size === 0 ? USIZE_MAX : 0, bytes), h: 0, n: 0, s: size };
}

/** `VecDeque::with_capacity` of `size`-byte items, which panics past what it may hold. */
export function dequeWithCapacity<T>(n: number, size: number, bytes = false): VecDeque<T> {
	if (n > vecMax(size)) panic("capacity overflow", "capacity");
	const q = dequeNew<T>(size, bytes);
	if (size !== 0) q.b = slots(n, bytes);
	return q;
}

/** A deque of `v`'s items, with its capacity, as std's `From<Vec<T>>` keeps it. */
export function dequeFromVec<T>(v: Vec<T>, size: number, bytes = false): VecDeque<T> {
	const q = dequeWithCapacity<T>(vecCapacity(v, size), size, bytes);
	const d = v.items;
	for (let i = 0; i < d.length; i++) q.b[i] = d[i];
	q.n = d.length;
	return q;
}

/** A deque of `a`'s items, with room for exactly them, as std's `From<[T; N]>` makes it. */
export function dequeFromArray<T>(a: ArrayLike<T>, size: number, bytes = false): VecDeque<T> {
	const q = dequeWithCapacity<T>(a.length, size, bytes);
	for (let i = 0; i < a.length; i++) q.b[i] = a[i];
	q.n = a.length;
	return q;
}

/** `VecDeque::capacity`. */
export function dequeCapacity<T>(q: VecDeque<T>): number {
	return q.b.length;
}

/** `VecDeque::reserve`. */
export function dequeReserve<T>(q: VecDeque<T>, additional: number): void {
	regrow(q, capReserve(q.n, q.b.length, additional, q.s));
}

/** `VecDeque::reserve_exact`. */
export function dequeReserveExact<T>(q: VecDeque<T>, additional: number): void {
	regrow(q, capReserveExact(q.n, q.b.length, additional, q.s));
}

/** `VecDeque::try_reserve`: `reserve`'s capacity, or the error where it panics or JS can't make a byte deque's. */
export function dequeTryReserve<T>(q: VecDeque<T>, additional: number): Result<undefined, TryReserveError> {
	return dequeTried(q, capTryReserve(q.n, q.b.length, additional, q.s));
}

/** `VecDeque::try_reserve_exact`. */
export function dequeTryReserveExact<T>(q: VecDeque<T>, additional: number): Result<undefined, TryReserveError> {
	return dequeTried(q, capTryReserveExact(q.n, q.b.length, additional, q.s));
}

/** `q` regrown to `cap`, or the overflow `undefined` is, or the failure to allocate its bytes. */
function dequeTried<T>(q: VecDeque<T>, cap: number | undefined): Result<undefined, TryReserveError> {
	if (cap === undefined) return CAPACITY_OVERFLOW;
	try {
		regrow(q, cap);
	} catch (e) {
		if (!holdsBytes(q)) throw e;
		return allocError(e, cap);
	}
	return OK;
}

/**
 * `VecDeque::shrink_to`, and `shrink_to_fit` of 0: the items moved within the slots kept
 * as std's moves them, the head to the front if the tail is past them, or else the part
 * past them to the front, or the wrapped head to their end.
 */
export function dequeShrinkTo<T>(q: VecDeque<T>, min: number): void {
	const cap = q.b.length;
	const target = Math.max(min, q.n);
	if (q.s === 0 || cap <= target) return;
	const end = q.h + q.n;
	const tailOutside = end > target && end <= cap;
	let h = q.h;
	if (q.n === 0 || (q.h >= target && tailOutside)) h = 0;
	else if (!tailOutside && q.h > cap - q.n) h = target - (cap - q.h);
	place(q, target, h);
}

/**
 * The slot `n` past slot `j`, as std's `wrap_index` of a `usize` sum: past `usize::MAX`,
 * which only zero-sized items' `usize::MAX` slots reach, it wraps first.
 */
function wrap<T>(q: VecDeque<T>, j: number): number {
	if (j < q.b.length) return j;
	return j > USIZE_MAX ? j - USIZE_MAX - 1 : j - q.b.length;
}

/** The slot of item `i`. */
function phys<T>(q: VecDeque<T>, i: number): number {
	return wrap(q, q.h + i);
}

/** The slot before `q`'s head, as std's `wrap_sub` finds it. */
function before<T>(q: VecDeque<T>): number {
	return wrap(q, q.h - 1 + q.b.length);
}

/**
 * `q` grown to `cap` slots, if more: the items moved as std's `handle_capacity_increase`
 * moves them, the wrapped tail after the old end if it is shorter and fits, or else the
 * head to the new end.
 */
function regrow<T>(q: VecDeque<T>, cap: number): void {
	const old = q.b.length;
	if (cap <= old) return;
	let h = q.h;
	if (h > old - q.n) {
		const head = old - h;
		const tail = q.n - head;
		if (head <= tail || cap - old < tail) h = cap - head;
	}
	place(q, cap, h);
}

/** `q`'s items moved, in order, to `cap` new slots from `h` on. */
function place<T>(q: VecDeque<T>, cap: number, h: number): void {
	const b = slots<T>(cap, holdsBytes(q));
	for (let i = 0, j = h; i < q.n; i++, j = j + 1 === cap ? 0 : j + 1) b[j] = q.b[phys(q, i)];
	q.b = b;
	q.h = h;
}

/** Room for one more item, grown as std's `grow_one`. */
function grow<T>(q: VecDeque<T>): void {
	if (q.n === q.b.length) regrow(q, capReserve(q.n, q.n, 1, q.s));
}

export function dequePushBack<T>(q: VecDeque<T>, item: T): void {
	grow(q);
	q.b[phys(q, q.n++)] = item;
}

export function dequePushFront<T>(q: VecDeque<T>, item: T): void {
	grow(q);
	q.h = before(q);
	q.b[q.h] = item;
	q.n++;
}

/** `pop_front`: the first item, or else `none`. */
export function dequePopFront<T>(q: VecDeque<T>): T | undefined;
export function dequePopFront<T>(q: VecDeque<T>, none: None): T | None;
export function dequePopFront<T>(q: VecDeque<T>, none?: None): T | None | undefined {
	if (q.n === 0) return none;
	const item = q.b[q.h];
	q.b[q.h] = undefined;
	q.h = phys(q, 1);
	q.n--;
	return item;
}

/** `pop_back`: the last item, or else `none`. */
export function dequePopBack<T>(q: VecDeque<T>): T | undefined;
export function dequePopBack<T>(q: VecDeque<T>, none: None): T | None;
export function dequePopBack<T>(q: VecDeque<T>, none?: None): T | None | undefined {
	if (q.n === 0) return none;
	const i = phys(q, --q.n);
	const item = q.b[i];
	q.b[i] = undefined;
	return item;
}

/** `get` and `get_mut`: the item at `i`, or else `none`. */
export function dequeGet<T>(q: VecDeque<T>, i: number): T | undefined;
export function dequeGet<T>(q: VecDeque<T>, i: number, none: None): T | None;
export function dequeGet<T>(q: VecDeque<T>, i: number, none?: None): T | None | undefined {
	return i < q.n ? (q.b[phys(q, i)] as T) : none;
}

/** `front`, and with `back`, `back`, or else `none`. */
export function dequeEnd<T>(q: VecDeque<T>, back?: boolean): T | undefined;
export function dequeEnd<T>(q: VecDeque<T>, back: boolean, none: None): T | None;
export function dequeEnd<T>(q: VecDeque<T>, back = false, none?: None): T | None | undefined {
	return q.n === 0 ? none : (q.b[phys(q, back ? q.n - 1 : 0)] as T);
}

/** `get_mut` of an item JS copies: its place, or `undefined` past the end. */
export function dequeGetMut<T>(q: VecDeque<T>, i: number): Place<T> | undefined {
	return i < q.n ? new AtIndex(q.b as T[], phys(q, i)) : undefined;
}

/** `front_mut` of an item JS copies, and with `back`, `back_mut`. */
export function dequeEndMut<T>(q: VecDeque<T>, back = false): Place<T> | undefined {
	return q.n === 0 ? undefined : new AtIndex(q.b as T[], phys(q, back ? q.n - 1 : 0));
}

/** `index` and `index_mut`: `q[i]`. */
export function dequeIndex<T>(q: VecDeque<T>, i: number, at?: Location): T {
	if (i >= q.n) panic("out of bounds access", at, true);
	return q.b[phys(q, i)] as T;
}

/** `q[i] = item`, through `index_mut`. */
export function dequeSet<T>(q: VecDeque<T>, i: number, item: T, at?: Location): void {
	if (i >= q.n) panic("out of bounds access", at, true);
	q.b[phys(q, i)] = item;
}

/** Drop `q`: each item by `f`, front to back. */
export function dequeDrop<T>(q: VecDeque<T>, f: (item: T) => void): void {
	const items: T[] = [];
	for (let i = 0; i < q.n; i++) items.push(q.b[phys(q, i)] as T);
	dropAll(items, f);
}

/** `clear`, which leaves the deque empty, its head at the front, and then drops each item, front to back. */
export function dequeClear<T>(q: VecDeque<T>, f?: (item: T) => void): void {
	const items: T[] = [];
	for (let i = 0; i < q.n; i++) {
		items.push(q.b[phys(q, i)] as T);
		q.b[phys(q, i)] = undefined;
	}
	q.h = 0;
	q.n = 0;
	if (f !== undefined) dropAll(items, f);
}

/** `truncate`: the first `len` items, if there are more, and then each other drops in order. */
export function dequeTruncate<T>(q: VecDeque<T>, len: number, f?: (item: T) => void): void {
	if (f === undefined || q.n <= len) {
		while (q.n > len) dequePopBack(q);
		return;
	}
	const cut: T[] = [];
	for (let i = len; i < q.n; i++) cut.push(q.b[phys(q, i)] as T);
	while (q.n > len) dequePopBack(q);
	dropAll(cut, f);
}

/**
 * `retain`, which visits the items in order and keeps each `f` returns true for. It swaps
 * each kept item into place, as std's does.
 */
export function dequeRetain<T>(q: VecDeque<T>, f: (item: T) => boolean, drop?: (item: T) => void): void {
	retainDeque(q, f as (item: T | Place<T>) => boolean, false, drop);
}

/** `retain_mut` of items JS copies: `f` takes the place of each. */
export function dequeRetainMut<T>(q: VecDeque<T>, f: (item: Place<T>) => boolean, drop?: (item: T) => void): void {
	retainDeque(q, f as (item: T | Place<T>) => boolean, true, drop);
}

/** `retain`, handing `f` each item or, if `lend`, its place. */
function retainDeque<T>(q: VecDeque<T>, f: (item: T | Place<T>) => boolean, lend: boolean, drop?: (item: T) => void): void {
	let kept = 0;
	for (let i = 0; i < q.n; i++) {
		const keep = f(lend ? new AtIndex(q.b as T[], phys(q, i)) : (q.b[phys(q, i)] as T));
		const item = q.b[phys(q, i)] as T;
		if (!keep) continue;
		q.b[phys(q, i)] = q.b[phys(q, kept)];
		q.b[phys(q, kept++)] = item;
	}
	dequeTruncate(q, kept, drop);
}

/** `contains`, by `eq`. */
export function dequeContains<T>(q: VecDeque<T>, item: T, eq: (a: T, b: T) => boolean): boolean {
	for (let i = 0; i < q.n; i++) if (eq(q.b[phys(q, i)] as T, item)) return true;
	return false;
}

/**
 * `swap_remove_front`, and with `back`, `swap_remove_back`: the item at `i`, with the
 * front or back item moved into its place, or else `none`.
 */
export function dequeSwapRemove<T>(q: VecDeque<T>, i: number, back?: boolean): T | undefined;
export function dequeSwapRemove<T>(q: VecDeque<T>, i: number, back: boolean, none: None): T | None;
export function dequeSwapRemove<T>(q: VecDeque<T>, i: number, back = false, none?: None): T | None | undefined {
	if (i >= q.n) return none;
	const [j, end] = [phys(q, i), phys(q, back ? q.n - 1 : 0)];
	[q.b[j], q.b[end]] = [q.b[end], q.b[j]];
	return (back ? dequePopBack(q) : dequePopFront(q)) as T;
}

/** `remove`: the item at `i`, with the shorter side moved over its slot, as std's, or else `none`. */
export function dequeRemove<T>(q: VecDeque<T>, i: number): T | undefined;
export function dequeRemove<T>(q: VecDeque<T>, i: number, none: None): T | None;
export function dequeRemove<T>(q: VecDeque<T>, i: number, none?: None): T | None | undefined {
	if (i >= q.n) return none;
	const item = q.b[phys(q, i)] as T;
	if (q.n - i - 1 < i) {
		for (let j = i; j + 1 < q.n; j++) q.b[phys(q, j)] = q.b[phys(q, j + 1)];
		q.b[phys(q, q.n - 1)] = undefined;
	} else {
		for (let j = i; j > 0; j--) q.b[phys(q, j)] = q.b[phys(q, j - 1)];
		q.b[q.h] = undefined;
		q.h = phys(q, 1);
	}
	q.n--;
	return item;
}

/** `insert`: `item` at `i`, with the shorter side moved to make room, as std's. */
export function dequeInsert<T>(q: VecDeque<T>, i: number, item: T): void {
	if (i > q.n) panic("index out of bounds", "deque_insert");
	grow(q);
	if (q.n - i < i) {
		for (let j = q.n; j > i; j--) q.b[phys(q, j)] = q.b[phys(q, j - 1)];
	} else {
		q.h = before(q);
		for (let j = 0; j < i; j++) q.b[phys(q, j)] = q.b[phys(q, j + 1)];
	}
	q.b[phys(q, i)] = item;
	q.n++;
}

/** `swap` of items `i` and `j`, each checked as std's asserts. */
export function dequeSwap<T>(q: VecDeque<T>, i: number, j: number): void {
	if (i >= q.n) panic("assertion failed: i < self.len()", "deque_swap_i");
	if (j >= q.n) panic("assertion failed: j < self.len()", "deque_swap_j");
	const a = phys(q, i);
	const b = phys(q, j);
	const item = q.b[a];
	q.b[a] = q.b[b];
	q.b[b] = item;
}

/**
 * `rotate_left` of `n`, or with `right`, `rotate_right`: as std's, the shorter side moved
 * around the end of its slots, the first items past the last or the last before the first.
 */
export function dequeRotate<T>(q: VecDeque<T>, n: number, right = false): void {
	if (n > q.n) panic("assertion failed: n <= self.len()", right ? "deque_rotate_right" : "deque_rotate_left");
	const k = q.n - n;
	if ((n <= k) !== right) rotateLeft(q, right ? k : n);
	else rotateRight(q, right ? n : k);
}

/** The first `mid` items moved past the last, as std's `rotate_left_inner`. */
function rotateLeft<T>(q: VecDeque<T>, mid: number): void {
	// Each slot written is free or one already read.
	const end = phys(q, q.n);
	for (let i = 0; i < mid; i++) {
		const from = phys(q, i);
		const item = q.b[from];
		q.b[from] = undefined;
		q.b[wrap(q, end + i)] = item;
	}
	q.h = phys(q, mid);
}

/**
 * The last `k` items moved before the first, as std's `rotate_right_inner`, whose head
 * steps back by `wrap_sub` even of none, which moves zero-sized items' `usize::MAX` slots'.
 */
function rotateRight<T>(q: VecDeque<T>, k: number): void {
	const h = wrap(q, q.h - k + q.b.length);
	// Back to front, so each slot written is free or one already read.
	for (let i = k - 1; i >= 0; i--) {
		const from = phys(q, q.n - k + i);
		const item = q.b[from];
		q.b[from] = undefined;
		q.b[wrap(q, h + i)] = item;
	}
	q.h = h;
}

/**
 * The head `make_contiguous` leaves: where it was if the items do not wrap, else the
 * front if the free slots hold the head's part, after the tail's part if they hold that,
 * and else after the free slots or at the front, by which part is shorter, as std's.
 * Zero-sized items' is the front.
 */
function contiguous<T>(q: VecDeque<T>): number {
	const cap = q.b.length;
	if (q.s === 0) return 0;
	if (q.h <= cap - q.n) return q.h;
	const free = cap - q.n;
	const head = cap - q.h;
	const tail = q.n - head;
	if (free >= head) return 0;
	if (free >= tail) return tail;
	return head > tail ? free : 0;
}

/** `make_contiguous`: the items moved to one run of slots, as std's, and the slice of them. */
export function dequeMakeContiguous<T>(q: VecDeque<T>): Slice<T> {
	const h = contiguous(q);
	if (h !== q.h) place(q, q.b.length, h);
	return new Slice(q.b as T[], q.h, q.n);
}

/** `make_contiguous` of arrays: the deque's own arrays, each lent by `of`. */
export function dequeArrayMakeContiguous<A, S extends ArrayRef>(q: VecDeque<A>, of: (array: A) => S): ArraySlice<S> {
	return arraySliceOfSlice(dequeMakeContiguous(q), of);
}

/** `make_contiguous` of bytes, viewing the deque's. */
export function dequeByteMakeContiguous(q: VecDeque<number>): ByteSlice {
	const h = contiguous(q);
	if (h !== q.h) place(q, q.b.length, h);
	return new ByteSlice(q.b as unknown as Uint8Array, q.h, q.n);
}

/**
 * `binary_search_by`: by `f` of the second slice's first item, std's search of that slice
 * if it is less, or else of the first slice, as `as_slices` splits them.
 */
export function dequeBinarySearchBy<T>(q: VecDeque<T>, f: (item: T) => Ordering): Result<usize, usize> {
	const front = firstEnd(q, 0, q.n);
	const order = front < q.n ? f(q.b[0] as T) : 1;
	if (order === 0) return Ok(front);
	if (order === 1) return searchBy(q.b as T[], phys(q, 0), front, f);
	const found = searchBy(q.b as T[], 0, q.n - front, f);
	return found.ok ? Ok(found.value + front) : Err(found.error + front);
}

/** `binary_search`: by `cmp` of each item it probes with `x`. */
export function dequeBinarySearch<T>(q: VecDeque<T>, x: T, cmp: (a: T, b: T) => Ordering): Result<usize, usize> {
	return dequeBinarySearchBy(q, (item) => cmp(item, x));
}

/** `binary_search_by_key`: by `cmp` of the key `f` makes of each item it probes with `b`, dropping the key after. */
export function dequeBinarySearchByKey<T, K>(q: VecDeque<T>, b: K, f: (item: T) => K, cmp: (a: K, b: K) => Ordering, drop?: (key: K) => void): Result<usize, usize> {
	return dequeBinarySearchBy(q, (item) => {
		const key = f(item);
		const order = cmp(key, b);
		drop?.(key);
		return order;
	});
}

/** `partition_point`: where the items `pred` holds for end, by std's search. */
export function dequePartitionPoint<T>(q: VecDeque<T>, pred: (item: T) => boolean): usize {
	const found = dequeBinarySearchBy(q, (item) => (pred(item) ? -1 : 1));
	return found.ok ? found.value : found.error;
}

/**
 * Where the first of the two slices of `q`'s items `start..end` ends, as std's
 * `slice_ranges` splits them at the end of its slots.
 */
function firstEnd<T>(q: VecDeque<T>, start: number, end: number): number {
	return start === end ? end : Math.min(end, start + q.b.length - phys(q, start));
}

/**
 * A deque's `Iter`, or `IterMut` lending each item's place if `lend`: the items at `f1..e1`,
 * then `f2..e2`, the two slices std's walks, which it swaps as one ends, as std's does.
 */
class DequeIter<T> extends Iter<T | Place<T>> {
	private readonly q: VecDeque<T>;
	private readonly lend: boolean;
	private f1: number;
	private e1: number;
	private f2: number;
	private e2: number;

	constructor(q: VecDeque<T>, lend: boolean, f1: number, e1: number, f2: number, e2: number) {
		super();
		this.q = q;
		this.lend = lend;
		this.f1 = f1;
		this.e1 = e1;
		this.f2 = f2;
		this.e2 = e2;
	}

	/** Item `i`, or its place. */
	private item(i: number): T | Place<T> {
		return this.lend ? new AtIndex(this.q.b as T[], phys(this.q, i)) : (this.q.b[phys(this.q, i)] as T);
	}

	/** The two slices, swapped, as std's once the one it reads from ends. */
	private swap(): void {
		[this.f1, this.e1, this.f2, this.e2] = [this.f2, this.e2, this.f1, this.e1];
	}

	/** The items each slice has left, as `as_slices` gives them. */
	slices(): [T[], T[]] {
		const items = (f: number, e: number) => {
			const out: T[] = [];
			for (let i = f; i < e; i++) out.push(this.q.b[phys(this.q, i)] as T);
			return out;
		};
		return [items(this.f1, this.e1), items(this.f2, this.e2)];
	}

	next(): T | Place<T> | Done {
		if (this.f1 === this.e1) this.swap();
		return this.f1 < this.e1 ? this.item(this.f1++) : DONE;
	}

	override nextBack(): T | Place<T> | Done {
		if (this.f2 === this.e2) this.swap();
		return this.f2 < this.e2 ? this.item(--this.e2) : DONE;
	}

	override advanceBy(n: number): number {
		const k = Math.min(n, this.e1 - this.f1);
		this.f1 += k;
		if (k === n) return 0;
		this.swap();
		const rest = Math.min(n - k, this.e1 - this.f1);
		this.f1 += rest;
		return n - k - rest;
	}

	override advanceBackBy(n: number): number {
		const k = Math.min(n, this.e2 - this.f2);
		this.e2 -= k;
		if (k === n) return 0;
		this.swap();
		const rest = Math.min(n - k, this.e2 - this.f2);
		this.e2 -= rest;
		return n - k - rest;
	}

	/** std's `try_fold`, which reads the first slice and then the second, swapping neither, and `try_rfold` the second and then the first. */
	override folded(): Iter<T | Place<T>> {
		return new Folded(
			() => (this.f1 < this.e1 ? this.item(this.f1++) : this.f2 < this.e2 ? this.item(this.f2++) : DONE),
			() => (this.f2 < this.e2 ? this.item(--this.e2) : this.f1 < this.e1 ? this.item(--this.e1) : DONE),
		);
	}

	override remaining(): Iterable<T> {
		const [a, b] = this.slices();
		return a.concat(b);
	}

	override clone(): Iter<T | Place<T>> {
		return new DequeIter(this.q, this.lend, this.f1, this.e1, this.f2, this.e2);
	}

	override len(): number {
		return this.e1 - this.f1 + (this.e2 - this.f2);
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}
}

/** A deque's `Iter` over its items `start..end`, split where `as_slices` splits them, or with `lend`, its `IterMut`. */
function dequeSlices<T>(q: VecDeque<T>, lend: boolean, start: number, end: number): DequeIter<T> {
	const mid = firstEnd(q, start, end);
	return new DequeIter(q, lend, start, mid, mid, end);
}

/** `iter` and `iter_mut`. */
export function dequeIter<T>(q: VecDeque<T>): Iter<T> {
	return dequeSlices(q, false, 0, q.n) as Iter<T>;
}

/** `range`: `iter` of the items within `r`, split as std's `slice_ranges` splits them, which panics as std's does past the length. */
export function dequeRange<T>(q: VecDeque<T>, r: Bounds<number>, at?: Location): Iter<T> {
	const [start, end] = boundsRange(r, q.n, at);
	return dequeSlices(q, false, start, end) as Iter<T>;
}

/** `range_mut` of items JS copies: the place of each within `r`, as `range` reads them. */
export function dequeRangeMut<T>(q: VecDeque<T>, r: Bounds<number>, at?: Location): Iter<Place<T>> {
	const [start, end] = boundsRange(r, q.n, at);
	return dequeSlices(q, true, start, end) as Iter<Place<T>>;
}

/** `iter_mut` of items JS copies: the place of each. */
export function dequeIterMut<T>(q: VecDeque<T>): Iter<Place<T>> {
	return dequeSlices(q, true, 0, q.n) as Iter<Place<T>>;
}

/** `Debug` of a deque's `Iter` or `IterMut`, `name`: the two slices it has left. */
export function fmtDequeIter<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>, name: string): FmtResult {
	const [a, b] = (it as DequeIter<T>).slices();
	return fmtDebugTupleFields(f, name, [(f) => fmtList(a, f, debug), (f) => fmtList(b, f, debug)]);
}

/** `as_slices` and `as_mut_slices`: the slices of the deque's slots. */
export function dequeAsSlices<T>(q: VecDeque<T>): [Slice<T>, Slice<T>] {
	const n = firstEnd(q, 0, q.n);
	return [new Slice(q.b as T[], phys(q, 0), n), new Slice(q.b as T[], 0, q.n - n)];
}

/** `as_slices` and `as_mut_slices` of arrays: the deque's own arrays, each lent by `of`. */
export function dequeArraySlices<A, S extends ArrayRef>(q: VecDeque<A>, of: (array: A) => S): [ArraySlice<S>, ArraySlice<S>] {
	const [a, b] = dequeAsSlices(q);
	return [arraySliceOfSlice(a, of), arraySliceOfSlice(b, of)];
}

/** `as_slices` and `as_mut_slices` of bytes, which view the deque's. */
export function dequeByteSlices(q: VecDeque<number>): [ByteSlice, ByteSlice] {
	const n = firstEnd(q, 0, q.n);
	const b = q.b as unknown as Uint8Array;
	return [new ByteSlice(b, phys(q, 0), n), new ByteSlice(b, 0, q.n - n)];
}

/**
 * `into_iter`: the deque itself, popping each item from either end, which drops those it
 * still holds by `drop` as it drops, and which `collect` into a deque gives back as it is.
 */
class DequeIntoIter<T> extends Iter<T> implements Disposable {
	readonly q: VecDeque<T>;
	private readonly drop: ((item: T) => void) | undefined;

	constructor(q: VecDeque<T>, drop?: (item: T) => void) {
		super();
		this.q = q;
		this.drop = drop;
	}

	next(): T | Done {
		return this.q.n === 0 ? DONE : (dequePopFront(this.q) as T);
	}

	override nextBack(): T | Done {
		return this.q.n === 0 ? DONE : (dequePopBack(this.q) as T);
	}

	/** std's: the first `n` drained, or all cleared, either of which drops them in order. */
	override advanceBy(n: number): number {
		const len = this.q.n;
		if (len < n) dequeClear(this.q, this.drop);
		else new DequeDrain(this.q, 0, n, this.drop)[Symbol.dispose]();
		return Math.max(n - len, 0);
	}

	/** std's: the last `n` truncated, or all cleared. */
	override advanceBackBy(n: number): number {
		const len = this.q.n;
		if (len < n) dequeClear(this.q, this.drop);
		else dequeTruncate(this.q, len - n, this.drop);
		return Math.max(n - len, 0);
	}

	override remaining(): Iterable<T> {
		return dequeIter(this.q) as Iterable<T>;
	}

	/** std's: a clone of the deque, each item cloned in order by `h.v`. */
	override clone(h?: Clones): Iter<T> {
		return new DequeIntoIter(dequeClone(this.q, h?.v as ((item: T) => T) | undefined), this.drop);
	}

	override len(): number {
		return this.q.n;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.q.n);
	}

	[Symbol.dispose](): void {
		if (this.drop !== undefined) dequeClear(this.q, this.drop);
	}
}

/** `into_iter`, whose items drop by `drop` with it. */
export function dequeIntoIter<T>(q: VecDeque<T>, drop?: (item: T) => void): Iter<T> {
	return new DequeIntoIter(q, drop);
}

/**
 * `collect` into a deque of `size`-byte items, of bytes if `bytes`, from `it`, a deque's
 * `into_iter` or a `Vec`'s, which std's gives the buffer of as it is: a deque's, or a
 * `Vec`'s with its head where its front was read to.
 */
export function dequeFromIntoIter<T>(it: Iter<T>, size: number, bytes = false): VecDeque<T> {
	if (it instanceof DequeIntoIter) return it.q;
	const v = it as unknown as { s: ArrayLike<T>; v: Vec<T> | undefined; front: number; back: number };
	const q = dequeWithCapacity<T>(v.v === undefined ? v.s.length : vecCapacity(v.v, size), size, bytes);
	q.n = v.back - v.front;
	q.h = size === 0 || q.n === 0 ? 0 : v.front;
	for (let i = 0; i < q.n; i++) q.b[q.h + i] = v.s[v.front + i];
	return q;
}

/**
 * `drain`: the items within `r`, which the deque gives up its length past the start of as
 * std's does, so one never dropped leaks the rest. Those it still holds as it drops each
 * drop by `drop`, front to back, and then the shorter of the items before and after the
 * range moves next to the other.
 */
export function dequeDrain<T>(q: VecDeque<T>, r: Bounds<number>, drop?: (item: T) => void, at?: Location): Iter<T> {
	const [start, end] = boundsRange(r, q.n, at);
	return new DequeDrain(q, start, end - start, drop);
}

/** A deque's drain, with std's counts: `idx` the next item's index, and `rest` of the range left, std's `remaining`. */
class DequeDrain<T> extends Iter<T> implements Disposable {
	private readonly q: VecDeque<T>;
	readonly drainLen: number;
	idx: number;
	readonly tailLen: number;
	rest: number;
	private readonly drop: ((item: T) => void) | undefined;
	private joined = false;

	constructor(q: VecDeque<T>, start: number, drainLen: number, drop?: (item: T) => void) {
		super();
		this.q = q;
		this.drainLen = drainLen;
		this.idx = start;
		this.tailLen = q.n - start - drainLen;
		this.rest = drainLen;
		this.drop = drop;
		q.n = start;
	}

	next(): T | Done {
		if (this.rest === 0) return DONE;
		this.rest--;
		return this.q.b[phys(this.q, this.idx++)] as T;
	}

	override nextBack(): T | Done {
		if (this.rest === 0) return DONE;
		return this.q.b[phys(this.q, this.idx + --this.rest)] as T;
	}

	override len(): number {
		return this.rest;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.rest);
	}

	[Symbol.dispose](): void {
		if (this.joined) return;
		this.joined = true;
		const q = this.q;
		const items: T[] = [];
		for (; this.rest > 0; this.rest--) items.push(q.b[phys(q, this.idx++)] as T);
		if (this.drop !== undefined) dropAll(items, this.drop);
		const head = q.n;
		const tail = this.tailLen;
		const gap = this.drainLen;
		q.n = head + tail;
		if (q.s === 0) return;
		// The shorter side moves next to the other, and the slots it leaves are cleared.
		if (head < tail) {
			for (let i = head - 1; i >= 0; i--) q.b[phys(q, i + gap)] = q.b[phys(q, i)];
			for (let i = 0; i < gap; i++) q.b[phys(q, i)] = undefined;
		} else {
			for (let i = 0; i < tail; i++) q.b[phys(q, head + i)] = q.b[phys(q, head + gap + i)];
			for (let i = 0; i < gap; i++) q.b[phys(q, q.n + i)] = undefined;
		}
		if (q.n === 0) q.h = 0;
		else if (head < tail) q.h = phys(q, gap);
	}
}

/** `Debug` of a deque's `Drain`: its counts, each by `debug`, as std's tuple of them. */
export function fmtDequeDrain(it: Iter<unknown>, f: Formatter, debug: Debug<usize>): FmtResult {
	const d = it as unknown as DequeDrain<unknown>;
	const counts = [d.drainLen, d.idx, d.tailLen, d.rest];
	return fmtDebugTupleFields(f, "Drain", counts.map((n) => (f: Formatter) => debug(n, f)));
}

/** `slice::range`: the start and end of `r` within `n` elements, panicking as std's does. */
export function boundsRange(r: Bounds<number>, n: number, at?: Location): [number, number] {
	const [lo, hi] = bounds(r);
	let end = n;
	if (hi.tag !== "Unbounded") {
		end = hi._0;
		if (hi.tag === "Included" ? end >= n : end > n) sliceIndexFail(0, end, n, at);
		if (hi.tag === "Included") end++;
	}
	let start = 0;
	if (lo.tag !== "Unbounded") {
		start = lo._0;
		if (lo.tag === "Excluded" ? start >= end : start > end) sliceIndexFail(start, end, n, at);
		if (lo.tag === "Excluded") start++;
	}
	return [start, end];
}

/** `slice::range` itself: `r` within `n` elements, as `{ start, end }`, panicking as std's does. */
export function boundsChecked(r: Bounds<number>, n: number, at?: Location): { start: number; end: number } {
	const [start, end] = boundsRange(r, n, at);
	return { start, end };
}

/**
 * A slice's index `r` other than `a..b`, `a..`, `..b`, and `..`, as the start and end of
 * the part it reads within `n` elements, panicking as std's `index` does.
 */
export function sliceIndexBounds(r: Bounds<number>, n: number, at?: Location): [number, number] {
	if (!inclusive(r)) return boundsRange(r, n, at);
	// std checks `a..=b` by its own fields, not by the bounds it gives.
	if (r.hi >= n) sliceIndexFail(r.lo, r.hi, n, at);
	const bounds = sliceGetBounds(r, n);
	if (bounds[0] > bounds[1]) sliceIndexFail(bounds[0], bounds[1], n, at);
	return bounds;
}

/**
 * A slice's index `r`, as `sliceIndexBounds` reads it, unchecked: what `get` reads, which
 * gives `None` unless the start is at most the end and the end at most `n`.
 */
export function sliceGetBounds(r: Bounds<number>, n: number): [number, number] {
	// An `a..=b` iterated to its end starts past it, at `b + 1`, which JS doesn't overflow.
	if (inclusive(r)) return [r.exhausted ? r.hi + 1 : r.lo, r.hi + 1];
	const [lo, hi] = bounds(r);
	const start = lo.tag === "Unbounded" ? 0 : lo.tag === "Included" ? lo._0 : lo._0 + 1;
	const end = hi.tag === "Unbounded" ? n : hi.tag === "Included" ? hi._0 + 1 : hi._0;
	return [start, end];
}

/** Whether `r` is an `a..=b`: a field test, not `instanceof`, so a bundle without one leaves the class out. */
function inclusive<K>(r: Bounds<K>): r is RangeInclusive<K> {
	return typeof r === "object" && "exhausted" in r;
}

/** std's `slice_index_fail`, which names the end where neither bound is out of order. */
function sliceIndexFail(start: number, end: number, n: number, at?: Location): never {
	rangeCheck(n, start, end, at);
	return panicSizes(["range end index ", end, " out of range for slice of length ", n], at);
}

// `Vec::drain`, which leaves the vector only what comes before the range until the drain
// drops and moves the tail back, so a drain that never drops, as `mem::forget` leaves
// it, leaks the rest, as std's does.

/**
 * `Vec::drain`: the elements within `r`. Those it still holds as it drops each drop by
 * `drop`, front to back, before the tail moves back.
 */
export function vecDrain<T>(v: Vec<T>, r: Bounds<number>, drop?: (item: T) => void, at?: Location): Iter<T> {
	const [start, end] = boundsRange(r, v.items.length, at);
	vecKeep(v);
	return new VecDrain(v.items, v.items.splice(start), end - start, drop);
}

/** A `Vec`'s drain, of its items `v`: of `rest`, what followed the range's start, the range, then the tail. */
class VecDrain<T> extends Iter<T> implements Disposable {
	protected readonly v: T[];
	protected readonly rest: T[];
	private front = 0;
	private back: number;
	/** Where the tail starts in `rest`. */
	protected readonly tail: number;
	private readonly drop: ((item: T) => void) | undefined;

	constructor(v: T[], rest: T[], tail: number, drop?: (item: T) => void) {
		super();
		this.v = v;
		this.rest = rest;
		this.back = tail;
		this.tail = tail;
		this.drop = drop;
	}

	next(): T | Done {
		return this.front < this.back ? this.rest[this.front++] : DONE;
	}

	override remaining(): Iterable<T> {
		return this.rest.slice(this.front, this.back);
	}

	override nextBack(): T | Done {
		return this.front < this.back ? this.rest[--this.back] : DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	[Symbol.dispose](): void {
		this.dropRest();
		for (let i = this.tail; i < this.rest.length; i++) this.v.push(this.rest[i]);
		this.rest.length = this.tail;
	}

	/** Drop what it has not yielded, as std's `Drain` drops it before the tail moves back. */
	protected dropRest(): void {
		const from = this.front;
		this.front = this.back;
		if (this.drop !== undefined) dropRange(this.rest, this.drop, from, this.back);
	}
}

/**
 * `Vec::splice` of `size`-byte elements: a drain of `r` that, once dropped, drops what it
 * has not yielded and fills the gap from the `items` `spliceFrom` gave it, as std's: after
 * a drain to the end, by `extend`, which reserves for a `trusted`, `TrustedLen`, iterator
 * as it does; else by each item until the gap is full, then the tail moved past room for
 * `items`' lower bound, reserved for the whole vector, and then past room for the rest,
 * collected as std's collects them. The tail moves back to the last item, and `items`
 * drops last if it `drops`.
 */
export function vecSplice<T>(v: Vec<T>, r: Bounds<number>, size: number, drop?: (item: T) => void, at?: Location): Iter<T> {
	const [start, end] = boundsRange(r, v.items.length, at);
	// Held by the vector, which keeps it if the splice is leaked.
	vecKeep(v);
	return new VecSplice(v, v.items.splice(start), end - start, drop, size);
}

/**
 * A splice, of `vecSplice` or `byteVecSplice`, given the `items` it fills from, which std's
 * makes once it has made the drain, and whether they are `trusted` and `drops`.
 */
export function spliceFrom<T>(s: Iter<T>, items: Iter<T>, trusted = false, drops = false): Iter<T> {
	const splice = s as unknown as { items: Iter<T>; trusted: boolean; drops: boolean };
	splice.items = items;
	splice.trusted = trusted;
	splice.drops = drops;
	return s;
}

class VecSplice<T> extends VecDrain<T> {
	items: Iter<T> = iterEmpty();
	private readonly vec: Vec<T>;
	private readonly size: number;
	/** The vector's capacity, which its length, cut to the drain's start, no longer gives. */
	private cap: number;
	trusted = false;
	drops = false;

	constructor(vec: Vec<T>, rest: T[], tail: number, drop: ((item: T) => void) | undefined, size: number) {
		super(vec.items, rest, tail, drop);
		this.vec = vec;
		this.size = size;
		this.cap = vecCapacity(vec, size);
	}

	override [Symbol.dispose](): void {
		this.dropRest();
		const v = this.v;
		const tailLen = this.rest.length - this.tail;
		if (tailLen === 0) {
			vecHold(this.vec, this.cap);
			iterExtend(this.vec, this.items, undefined, this.size, this.trusted);
		} else {
			this.splice(tailLen);
			for (let i = this.tail; i < this.rest.length; i++) v.push(this.rest[i]);
			vecHold(this.vec, this.cap);
		}
		this.rest.length = this.tail;
		if (this.drops) dispose(this.items);
	}

	/** Each item of `items` pushed into the gap before the tail of `tailLen`, which moves as std's `move_tail` moves it. */
	private splice(tailLen: number): void {
		const v = this.v;
		let tailStart = v.length + this.tail;
		const fill = (from: Iter<T>) => {
			while (v.length < tailStart) {
				const item = from.next();
				if (item === DONE) return false;
				v.push(item);
			}
			return true;
		};
		const moveTail = (additional: number) => {
			this.cap = capReserve(tailStart + tailLen, this.cap, additional, this.size);
			tailStart += additional;
		};
		if (!fill(this.items)) return;
		const lower = this.items.sizeHint()[0];
		if (lower > 0) {
			moveTail(lower);
			if (!fill(this.items)) return;
		}
		const collected = iterToVec(this.items, undefined, this.size, this.trusted).items;
		if (collected.length === 0) return;
		moveTail(collected.length);
		fill(iterSlice(collected));
	}
}

/** `Debug` of a `Vec`'s `Splice`, as std's derives: its drain, of what it has not yielded, each by `debug`, and the iterator it fills from by `items`. */
export function fmtVecSplice<T, I>(it: Iter<T>, f: Formatter, debug: Debug<T>, items: Debug<I>): FmtResult {
	const drain = (f: Formatter) => fmtDebugTupleFields(f, "Drain", [(f) => fmtList(it.remaining() as Iterable<T>, f, debug)]);
	const held = iterHeld<I>((it as unknown as { items: Iter<unknown> }).items);
	return fmtDebugStructFields(f, "Splice", ["drain", "replace_with"], [drain, (f) => items(held, f)]);
}

/**
 * `Vec::extract_if`: each element of `start..end` that `f` returns true for, taken out as
 * it is read, front to back, with each it keeps moved back over the gaps, as std's; once
 * dropped, those after the last it read moved back too, `put` gives the vector back the
 * `len` it keeps of `a`, and `f` drops. `f` is given each element or, with `places`, its
 * place. The vector holds none meanwhile, as std's, so one leaked leaves it empty.
 */
class VecExtractIf<T> extends Iter<T> implements Disposable {
	private readonly put: (len: number) => void;
	private readonly a: T[];
	private idx: number;
	private readonly end: number;
	private del = 0;
	/** The vector's length before, or -1 once given back. */
	private old: number;
	private readonly f: (item: T | Place<T>) => boolean;
	private readonly places: boolean;

	constructor(a: T[], old: number, [idx, end]: [number, number], f: (item: T | Place<T>) => boolean, places: boolean, put: (len: number) => void) {
		super();
		this.put = put;
		this.a = a;
		this.old = old;
		this.idx = idx;
		this.end = end;
		this.f = f;
		this.places = places;
	}

	next(): T | Done {
		while (this.idx < this.end) {
			const i = this.idx;
			const taken = this.f(this.places ? new AtIndex(this.a, i) : this.a[i]);
			this.idx++;
			if (taken) {
				this.del++;
				return this.a[i];
			}
			if (this.del > 0) this.a[i - this.del] = this.a[i];
		}
		return DONE;
	}

	override sizeHint(): SizeHint {
		return [0, this.end - this.idx];
	}

	/** What `Debug` writes of it: those it kept, those it has left to read, and those past its range. */
	parts(): [Iterable<T>, Iterable<T>, Iterable<T>] {
		const part = (start: number, end: number) => (this.a instanceof Uint8Array ? this.a.subarray(start, end) : this.a.slice(start, end)) as Iterable<T>;
		return [part(0, this.idx - this.del), part(this.idx, this.end), part(this.end, this.old)];
	}

	[Symbol.dispose](): void {
		if (this.old < 0) return;
		if (this.del > 0) for (let i = this.idx; i < this.old; i++) this.a[i - this.del] = this.a[i];
		this.put(this.old - this.del);
		this.old = -1;
		dispose(this.f);
	}
}

/**
 * `Vec::extract_if` of the elements within `r`, handing `f` each, or with `places`, its
 * place, panicking at `at` if `r` is out of bounds.
 */
function extractIf<T>(v: Vec<T>, r: Bounds<number>, f: (item: T | Place<T>) => boolean, places: boolean, at?: Location): Iter<T> {
	const d = v.items;
	const old = d.length;
	const range = boundsRange(r, old, at);
	vecKeep(v);
	const a = d.splice(0);
	return new VecExtractIf(a, old, range, f, places, (len) => {
		for (let i = 0; i < len; i++) d.push(a[i]);
	});
}

/** `Vec::extract_if` of the elements within `r`, handing `f` each. */
export function vecExtractIf<T>(v: Vec<T>, r: Bounds<number>, f: (item: T) => boolean, at?: Location): Iter<T> {
	return extractIf(v, r, f as (item: T | Place<T>) => boolean, false, at);
}

/** `vecExtractIf` of elements JS copies: `f` takes the place of each. */
export function vecExtractIfMut<T>(v: Vec<T>, r: Bounds<number>, f: (item: Place<T>) => boolean, at?: Location): Iter<T> {
	return extractIf(v, r, f as (item: T | Place<T>) => boolean, true, at);
}

/** `Vec::extract_if` of bytes, whose places `f` takes in the vector's storage. */
export function byteVecExtractIf(v: ByteVec, r: Bounds<number>, f: (byte: Place<number>) => boolean, at?: Location): Iter<number> {
	const old = v.length;
	const range = boundsRange(r, old, at);
	v.length = 0;
	return new VecExtractIf(v.data as unknown as number[], old, range, f as (item: number | Place<number>) => boolean, true, (len) => {
		v.length = len;
	});
}

/** `Debug` of a `Vec`'s `ExtractIf`, its parts each by `debug`, as std's. */
export function fmtVecExtractIf<T>(it: Iter<T>, f: Formatter, debug: Debug<T>): FmtResult {
	const parts = (it as VecExtractIf<T>).parts();
	let b = fmtDebugStruct(f, "ExtractIf");
	["retained", "remainder", "skipped_tail"].forEach((name, i) => {
		b = debugStructField(b, name, (f) => fmtList(parts[i], f, debug));
	});
	return debugStructFinishNonExhaustive(b);
}

/** `Vec::extend_from_within` of `size`-byte elements: room reserved for those within `r`, then a clone of each by `clone` pushed in order. */
export function vecExtendFromWithin<T>(v: Vec<T>, r: Bounds<number>, size: number, clone?: (item: T) => T, at?: Location): void {
	const d = v.items;
	const [start, end] = boundsRange(r, d.length, at);
	vecReserve(v, end - start, size);
	for (let i = start; i < end; i++) d.push(clone === undefined ? d[i] : clone(d[i]));
}

/** `Vec::extend_from_within` of bytes. */
export function byteVecExtendFromWithin(v: ByteVec, r: Bounds<number>, at?: Location): void {
	const [start, end] = boundsRange(r, v.length, at);
	byteVecReserve(v, end - start);
	v.data.copyWithin(v.length, start, end);
	v.length += end - start;
}

/** `Vec::drain` of bytes. */
export function byteVecDrain(v: ByteVec, r: Bounds<number>, at?: Location): Iter<number> {
	const [start, end] = boundsRange(r, v.length, at);
	const length = v.length;
	v.length = start;
	return new ByteVecDrain(v, start, end, length);
}

/** A `Vec<u8>`'s drain, which reads the range from the vector's storage in place. */
class ByteVecDrain extends Iter<number> implements Disposable {
	protected readonly v: ByteVec;
	private readonly start: number;
	protected front: number;
	protected back: number;
	protected readonly end: number;
	/** The vector's length before the drain, or -1 once the tail moved back. */
	protected length: number;

	constructor(v: ByteVec, start: number, end: number, length: number) {
		super();
		this.v = v;
		this.start = start;
		this.front = start;
		this.back = end;
		this.end = end;
		this.length = length;
	}

	next(): number | Done {
		return this.front < this.back ? this.v.data[this.front++] : DONE;
	}

	override remaining(): Iterable<number> {
		return this.v.data.slice(this.front, this.back);
	}

	override nextBack(): number | Done {
		return this.front < this.back ? this.v.data[--this.back] : DONE;
	}

	override len(): number {
		return this.back - this.front;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.len());
	}

	[Symbol.dispose](): void {
		if (this.length < 0) return;
		this.v.data.copyWithin(this.start, this.end, this.length);
		this.v.length = this.start + this.length - this.end;
		this.length = -1;
	}
}

/** `Vec::splice` of bytes, as `vecSplice`, given its items by `spliceFrom`. */
export function byteVecSplice(v: ByteVec, r: Bounds<number>, at?: Location): Iter<number> {
	const [start, end] = boundsRange(r, v.length, at);
	const length = v.length;
	v.length = start;
	return new ByteVecSplice(v, start, end, length);
}

class ByteVecSplice extends ByteVecDrain {
	items: Iter<number> = iterEmpty();
	trusted = false;
	drops = false;

	override [Symbol.dispose](): void {
		if (this.length < 0) return;
		this.front = this.back;
		const v = this.v;
		const tailLen = this.length - this.end;
		this.length = -1;
		if (tailLen === 0) iterExtendBytes(v, this.items, undefined, this.trusted);
		else {
			let tailStart = this.end;
			const fill = (from: Iter<number>) => {
				while (v.length < tailStart) {
					const byte = from.next();
					if (byte === DONE) return false;
					v.data[v.length++] = byte;
				}
				return true;
			};
			// Room reserved for the whole vector, its tail included, as std's `move_tail`.
			const moveTail = (additional: number) => {
				const filled = v.length;
				v.length = tailStart + tailLen;
				byteVecReserve(v, additional);
				v.length = filled;
				v.data.copyWithin(tailStart + additional, tailStart, tailStart + tailLen);
				tailStart += additional;
			};
			const spliced = () => {
				if (!fill(this.items)) return;
				const lower = this.items.sizeHint()[0];
				if (lower > 0) {
					moveTail(lower);
					if (!fill(this.items)) return;
				}
				const collected = iterToByteVec(this.items, undefined, this.trusted);
				if (collected.length === 0) return;
				moveTail(collected.length);
				fill(iterSlice(collected.data.subarray(0, collected.length)));
			};
			spliced();
			v.data.copyWithin(v.length, tailStart, tailStart + tailLen);
			v.length += tailLen;
		}
		if (this.drops) dispose(this.items);
	}
}

/**
 * `extend`, and `collect` into a deque through `dequeNew`, which takes `items` as `take`
 * says and reserves as `iterExtend` does.
 */
export function dequeExtend<T>(q: VecDeque<T>, items: Iter<T>, take?: Take, trusted = false): VecDeque<T> {
	if (trusted) {
		dequeReserve(q, trustedUpper(items));
		return extending(q, items, (item) => dequePushBack(q, item), take);
	}
	// std's reserves for the lower bound past each item it reads into a full deque, and the first.
	for (let item = items.next(), first = true; item !== DONE; item = items.next(), first = false) {
		if (first || q.n === q.b.length) dequeReserve(q, satAdd(items.sizeHint()[0], 1));
		q.b[phys(q, q.n++)] = item;
	}
	if (take !== undefined) dispose(items);
	return q;
}

/** `clone`, with each item through its `Clone`, or as itself without one, with room for exactly them. */
export function dequeClone<T>(q: VecDeque<T>, item: (value: T) => T = itself): VecDeque<T> {
	const out = dequeWithCapacity<T>(q.n, q.s, holdsBytes(q));
	for (let i = 0; i < q.n; i++) out.b[i] = item(q.b[phys(q, i)] as T);
	out.n = q.n;
	return out;
}

/**
 * `clone_from`: `q` cleared, which drops its items in order, then a clone of each of `src`'s
 * in its room, as std's `extend` reserves for them.
 */
export function dequeCloneFrom<T>(q: VecDeque<T>, src: VecDeque<T>, item: (value: T) => T = itself, drop?: (item: T) => void): VecDeque<T> {
	dequeClear(q, drop);
	dequeReserve(q, src.n);
	for (let i = 0; i < src.n; i++) q.b[phys(q, q.n++)] = item(src.b[phys(src, i)] as T);
	return q;
}

/** `dequeCloneFrom` as a `CloneFrom`. */
export function dequeCloneFromFn<T>(item?: (value: T) => T, drop?: (item: T) => void): CloneFrom<VecDeque<T>> {
	return (old, src) => dequeCloneFrom(old, src, item, drop);
}

/** `append`: room reserved for `other`'s items, which then move over in order, leaving it empty with its head at the front. */
export function dequeAppend<T>(q: VecDeque<T>, other: VecDeque<T>): void {
	dequeReserve(q, other.n);
	for (let i = 0; i < other.n; i++) {
		q.b[phys(q, q.n + i)] = other.b[phys(other, i)];
		other.b[phys(other, i)] = undefined;
	}
	q.n += other.n;
	other.n = 0;
	other.h = 0;
}

/**
 * `resize` to `len`: past its length, room reserved for the rest, then `len - 1` clones of
 * `value` by `clone` and `value` itself; else truncated, each item cut dropping in order,
 * and then `value`.
 */
export function dequeResize<T>(q: VecDeque<T>, len: number, value: T, clone: ((value: T) => T) | undefined, drop?: (item: T) => void): void {
	if (len <= q.n) {
		dequeTruncate(q, len, drop);
		drop?.(value);
		return;
	}
	dequeReserve(q, len - q.n);
	while (q.n < len - 1) q.b[phys(q, q.n++)] = clone === undefined ? value : clone(value);
	q.b[phys(q, q.n++)] = value;
}

/** `resize_with` to `len`: past its length, room reserved for the rest, then each `f` makes in order; else truncated. */
export function dequeResizeWith<T>(q: VecDeque<T>, len: number, f: () => T, drop?: (item: T) => void): void {
	if (len <= q.n) return dequeTruncate(q, len, drop);
	dequeReserve(q, len - q.n);
	while (q.n < len) q.b[phys(q, q.n++)] = f();
}

/** `split_off`: the items from `i` on, moved to a new deque with room for exactly them. */
export function dequeSplitOff<T>(q: VecDeque<T>, i: number): VecDeque<T> {
	if (i > q.n) panic("`at` out of bounds", "deque_split_off");
	const other = dequeWithCapacity<T>(q.n - i, q.s, holdsBytes(q));
	for (let j = i; j < q.n; j++) {
		other.b[j - i] = q.b[phys(q, j)];
		q.b[phys(q, j)] = undefined;
	}
	other.n = q.n - i;
	q.n = i;
	return other;
}

/** `Vec::from` of a deque: its items in order, with its capacity, as std's makes its buffer contiguous and keeps it, however the length then drops. */
export function dequeToVec<T>(q: VecDeque<T>): Vec<T> {
	const d: T[] = [];
	for (let i = 0; i < q.n; i++) d.push(q.b[phys(q, i)] as T);
	return new Vec(d, q.b.length);
}

// LinkedList

/** A `LinkedList` node, linked to the one before, `p`, and after, `x`: the place of its item `v`. */
export interface ListNode<T> {
	v: T;
	p: ListNode<T> | undefined;
	x: ListNode<T> | undefined;
}

/** `LinkedList<T>`: `n` nodes, linked both ways from `h` to `t`, as std's are. */
export interface LinkedList<T> {
	h: ListNode<T> | undefined;
	t: ListNode<T> | undefined;
	n: number;
}

/** `LinkedList::new`. */
export function listNew<T>(): LinkedList<T> {
	return { h: undefined, t: undefined, n: 0 };
}

/** `push_back`, and `push_back_mut`, which lends the node it returns: the item's place. */
export function listPushBack<T>(l: LinkedList<T>, item: T): ListNode<T> {
	const node: ListNode<T> = { v: item, p: l.t, x: undefined };
	if (l.t === undefined) l.h = node;
	else l.t.x = node;
	l.t = node;
	l.n++;
	return node;
}

/** `push_front`, and `push_front_mut`, as `listPushBack`. */
export function listPushFront<T>(l: LinkedList<T>, item: T): ListNode<T> {
	const node: ListNode<T> = { v: item, p: undefined, x: l.h };
	if (l.h === undefined) l.t = node;
	else l.h.p = node;
	l.h = node;
	l.n++;
	return node;
}

/** `pop_front`: the first item, or else `none`. */
export function listPopFront<T>(l: LinkedList<T>): T | undefined;
export function listPopFront<T>(l: LinkedList<T>, none: None): T | None;
export function listPopFront<T>(l: LinkedList<T>, none?: None): T | None | undefined {
	const node = l.h;
	if (node === undefined) return none;
	l.h = node.x;
	if (l.h === undefined) l.t = undefined;
	else l.h.p = undefined;
	l.n--;
	return node.v;
}

/** `pop_back`: the last item, or else `none`. */
export function listPopBack<T>(l: LinkedList<T>): T | undefined;
export function listPopBack<T>(l: LinkedList<T>, none: None): T | None;
export function listPopBack<T>(l: LinkedList<T>, none?: None): T | None | undefined {
	const node = l.t;
	if (node === undefined) return none;
	l.t = node.p;
	if (l.t === undefined) l.h = undefined;
	else l.t.x = undefined;
	l.n--;
	return node.v;
}

/** `front`, and with `back`, `back`, or else `none`. */
export function listEnd<T>(l: LinkedList<T>, back?: boolean): T | undefined;
export function listEnd<T>(l: LinkedList<T>, back: boolean, none: None): T | None;
export function listEnd<T>(l: LinkedList<T>, back = false, none?: None): T | None | undefined {
	const node = back ? l.t : l.h;
	return node === undefined ? none : node.v;
}

/** `front_mut` of an item JS copies, and with `back`, `back_mut`: its node, which is its place. */
export function listEndMut<T>(l: LinkedList<T>, back = false): Place<T> | undefined {
	return back ? l.t : l.h;
}

/** Drop the `n` items from `node` on by `f`, front to back. */
function dropNodes<T>(node: ListNode<T> | undefined, n: number, f: (item: T) => void): void {
	for (; n > 0; n--, node = node?.x) f((node as ListNode<T>).v);
}

/** Drop `l`: each item by `f`, front to back, as std's `pop_front_node` loop does. */
export function listDrop<T>(l: LinkedList<T>, f: (item: T) => void): void {
	dropNodes(l.h, l.n, f);
}

/** `clear`, which leaves the list empty and then drops each item, front to back. */
export function listClear<T>(l: LinkedList<T>, f?: (item: T) => void): void {
	const [h, n] = [l.h, l.n];
	l.h = l.t = undefined;
	l.n = 0;
	if (f !== undefined) dropNodes(h, n, f);
}

/** `contains`, by `eq`. */
export function listContains<T>(l: LinkedList<T>, item: T, eq: (a: T, b: T) => boolean): boolean {
	for (let node = l.h; node !== undefined; node = node.x) if (eq(node.v, item)) return true;
	return false;
}

/** `append`: each of `other`'s nodes linked after `l`'s, leaving `other` empty. */
export function listAppend<T>(l: LinkedList<T>, other: LinkedList<T>): void {
	if (other.h === undefined) return;
	if (l.t === undefined) l.h = other.h;
	else {
		l.t.x = other.h;
		other.h.p = l.t;
	}
	l.t = other.t;
	l.n += other.n;
	other.h = other.t = undefined;
	other.n = 0;
}

/**
 * `split_off`: the nodes from `at` on, unlinked from `l`, found from whichever end is
 * nearer, as std's does. Past the end, it panics as std's does.
 */
export function listSplitOff<T>(l: LinkedList<T>, phys: number): LinkedList<T> {
	if (phys > l.n) panic("Cannot split off at a nonexistent index", "list_split_off");
	const out = listNew<T>();
	if (phys === l.n) return out;
	if (phys === 0) {
		listAppend(out, l);
		return out;
	}
	// The last node `l` keeps.
	let last = l.h as ListNode<T>;
	if (phys - 1 <= l.n - phys) for (let i = 1; i < phys; i++) last = last.x as ListNode<T>;
	else {
		last = l.t as ListNode<T>;
		for (let i = l.n; i > phys; i--) last = last.p as ListNode<T>;
	}
	out.h = last.x as ListNode<T>;
	out.h.p = undefined;
	out.t = l.t;
	out.n = l.n - phys;
	last.x = undefined;
	l.t = last;
	l.n = phys;
	return out;
}

/**
 * An iterator of `n` of a list's nodes, from `h` to `t`, which yields each item, or with
 * `places`, each node, its place. With what drops each item, it owns them, as `into_iter`
 * does, and drops those it still holds, front to back, as it drops.
 */
class ListIter<T> extends Iter<T | Place<T>> implements Disposable {
	private h: ListNode<T> | undefined;
	private t: ListNode<T> | undefined;
	private n: number;
	private readonly places: boolean;
	private readonly drop: ((item: T) => void) | undefined;

	constructor(l: LinkedList<T>, places: boolean, drop?: (item: T) => void) {
		super();
		this.h = l.h;
		this.t = l.t;
		this.n = l.n;
		this.places = places;
		this.drop = drop;
	}

	next(): T | Place<T> | Done {
		const node = this.h;
		if (this.n === 0 || node === undefined) return DONE;
		this.n--;
		this.h = node.x;
		return this.places ? node : node.v;
	}

	override nextBack(): T | Place<T> | Done {
		const node = this.t;
		if (this.n === 0 || node === undefined) return DONE;
		this.n--;
		this.t = node.p;
		return this.places ? node : node.v;
	}

	override len(): number {
		return this.n;
	}

	override sizeHint(): SizeHint {
		return exactHint(this.n);
	}

	override remaining(): Iterable<T> {
		return new ListIter({ h: this.h, t: this.t, n: this.n }, false) as Iter<T>;
	}

	/** std's: where it is, or of `into_iter`, a list of what it has left, each cloned in order by `h.v`. */
	override clone(h?: Clones): Iter<T | Place<T>> {
		const rest = { h: this.h, t: this.t, n: this.n };
		if (h?.v === undefined) return new ListIter(rest, this.places, this.drop);
		return new ListIter(listClone(rest, h.v as (item: T) => T), false, this.drop);
	}

	[Symbol.dispose](): void {
		if (this.drop === undefined) return;
		const [h, n] = [this.h, this.n];
		this.n = 0;
		dropNodes(h, n, this.drop);
	}
}

/** `iter`, and with what drops each item, `into_iter`. */
export function listIter<T>(l: LinkedList<T>, drop?: (item: T) => void): Iter<T> {
	return new ListIter(l, false, drop) as Iter<T>;
}

/** `iter_mut` of items JS copies: the place of each. */
export function listIterMut<T>(l: LinkedList<T>): Iter<Place<T>> {
	return new ListIter(l, true) as Iter<Place<T>>;
}

/**
 * `extract_if`: each item `f` returns true for, unlinked as it is read, front to back. `f`
 * is given each item or, with `places`, its place, and drops with the iterator; what it
 * has not read stays.
 */
class ListExtractIf<T> extends Iter<T> implements Disposable {
	private readonly l: LinkedList<T>;
	node: ListNode<T> | undefined;
	private readonly f: (item: T | Place<T>) => boolean;
	private readonly places: boolean;
	/** How many it has left to read. */
	private left: number;

	constructor(l: LinkedList<T>, f: (item: T | Place<T>) => boolean, places: boolean) {
		super();
		this.l = l;
		this.node = l.h;
		this.f = f;
		this.places = places;
		this.left = l.n;
	}

	next(): T | Done {
		for (let node = this.node; node !== undefined; node = this.node) {
			this.node = node.x;
			this.left--;
			if (!this.f(this.places ? node : node.v)) continue;
			if (node.p === undefined) this.l.h = node.x;
			else node.p.x = node.x;
			if (node.x === undefined) this.l.t = node.p;
			else node.x.p = node.p;
			this.l.n--;
			return node.v;
		}
		return DONE;
	}

	override sizeHint(): SizeHint {
		return [0, this.left];
	}

	[Symbol.dispose](): void {
		dispose(this.f);
	}
}

/** `Debug` of a list's `ExtractIf`: the item it reads next, if any. */
export function fmtListExtractIf<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>): FmtResult {
	const node = (it as ListExtractIf<T>).node;
	const peek: Fmt = (f) => (node === undefined ? f.out.writeStr("None") : fmtDebugTupleFields(f, "Some", [(f) => debug(node.v, f)]));
	return debugStructFinishNonExhaustive(debugStructField(fmtDebugStruct(f, "ExtractIf"), "peek", peek));
}

/** `extract_if`, handing `f` each item. */
export function listExtractIf<T>(l: LinkedList<T>, f: (item: T) => boolean): Iter<T> {
	return new ListExtractIf(l, f as (item: T | Place<T>) => boolean, false);
}

/** `extract_if` of items JS copies: `f` takes the place of each. */
export function listExtractIfMut<T>(l: LinkedList<T>, f: (item: Place<T>) => boolean): Iter<T> {
	return new ListExtractIf(l, f as (item: T | Place<T>) => boolean, true);
}

/** `extend`, and `collect` into a list through `listNew`: each item pushed at the back, read as `take` says. */
export function listExtend<T>(l: LinkedList<T>, items: Iter<T>, take?: Take): LinkedList<T> {
	return extending(l, items, (item) => listPushBack(l, item), take);
}

/** `clone`, with each item through its `Clone`, or as itself without one, front to back. */
export function listClone<T>(l: LinkedList<T>, item: (value: T) => T = itself): LinkedList<T> {
	const out = listNew<T>();
	let node = l.h;
	for (let n = l.n; n > 0; n--, node = node?.x) listPushBack(out, item((node as ListNode<T>).v));
	return out;
}

/**
 * `clone_from`: `l`'s items, front to back, each the clone of `source`'s beside it, which
 * drops by `f` what it replaces, as `*elem = source.clone()` does, or else `source`'s
 * cloned over it by `over`, its own `clone_from`; then those past
 * `source`'s length popped from the back, each dropped as it is; then clones of the rest
 * of `source`'s pushed.
 */
export function listCloneFrom<T>(l: LinkedList<T>, source: LinkedList<T>, item: (value: T) => T = itself, f?: (item: T) => void, over?: CloneFrom<T>): LinkedList<T> {
	let [node, from]: (ListNode<T> | undefined)[] = [l.h, source.h];
	for (; node !== undefined && from !== undefined; node = node.x, from = from.x) {
		if (over !== undefined) {
			node.v = over(node.v, from.v);
			continue;
		}
		const old = node.v;
		node.v = item(from.v);
		f?.(old);
	}
	while (l.n > source.n) {
		const old = listPopBack(l) as T;
		f?.(old);
	}
	for (; from !== undefined; from = from.x) listPushBack(l, item(from.v));
	return l;
}

/** `listCloneFrom` as a `CloneFrom`. */
export function listCloneFromFn<T>(item?: (value: T) => T, f?: (item: T) => void, over?: CloneFrom<T>): CloneFrom<LinkedList<T>> {
	return (old, src) => listCloneFrom(old, src, item, f, over);
}

// `BinaryHeap`: std's max-heap in an array, sifted as std's sifts it, so equal elements
// leave in std's order and each comparison std makes runs. Every function that sifts is
// given the elements' `PartialOrd` tests std's sifts make: `le` up, and `le`, `ge`, and
// `lt` down.

/** A `PartialOrd` test. */
type Test<T> = (a: T, b: T) => boolean;

/** `BinaryHeap<T>`: the `Vec` of its elements, in std's order. */
export type BinaryHeap<T> = Vec<T>;

/** `BinaryHeap::new` and `default`. */
export function heapNew<T>(): BinaryHeap<T> {
	return new Vec();
}

/** `BinaryHeap::with_capacity` of `size`-byte elements: its `Vec`'s. */
export function heapWithCapacity<T>(n: number, size: number): BinaryHeap<T> {
	return vecWithCapacity(n, size);
}

/** `From<Vec<T>>`: the vector, its elements rebuilt into a heap in place, as std's are. */
export function heapFrom<T>(v: Vec<T>, le: Test<T>, ge: Test<T>, lt: Test<T>): BinaryHeap<T> {
	rebuild(v.items, le, ge, lt);
	return v;
}

/** `clone_from`: its vector's, of `size`-byte elements, which keeps its buffer. */
export function heapCloneFrom<T>(h: BinaryHeap<T>, src: BinaryHeap<T>, size: number, clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): BinaryHeap<T> {
	return vecCloneFrom(h, src.items, size, clone, drop, over);
}

/** `heapCloneFrom` as a `CloneFrom`. */
export function heapCloneFromFn<T>(size: number, clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): CloneFrom<BinaryHeap<T>> {
	return (old, src) => heapCloneFrom(old, src, size, clone, drop, over);
}

/** `push`: `item` sifted up from the end. */
export function heapPush<T>(h: BinaryHeap<T>, item: T, le: Test<T>): void {
	h.items.push(item);
	siftUp(h.items, 0, h.items.length - 1, le);
}

/** `peek` of an element that can be `undefined`: the greatest element, or `none` if empty. */
export function heapPeek<T>(h: BinaryHeap<T>, none: None): T | None {
	return h.items.length === 0 ? none : h.items[0];
}

/** `pop`: the greatest element, or `none` if empty, the last element sifted into its place to the bottom and back up. */
export function heapPop<T>(h: BinaryHeap<T>, le: Test<T>): T | undefined;
export function heapPop<T>(h: BinaryHeap<T>, le: Test<T>, none: None): T | None;
export function heapPop<T>(h: BinaryHeap<T>, le: Test<T>, none?: None): T | None | undefined {
	const d = h.items;
	if (d.length === 0) return none;
	vecKeep(h);
	let item = d.pop() as T;
	if (d.length > 0) {
		[item, d[0]] = [d[0], item];
		siftDownToBottom(d, 0, le);
	}
	return item;
}

/** `into_sorted_vec`: the elements in ascending order, each greatest swapped to the end and the rest sifted. */
export function heapIntoSortedVec<T>(h: BinaryHeap<T>, le: Test<T>, ge: Test<T>, lt: Test<T>): Vec<T> {
	const d = h.items;
	for (let end = d.length - 1; end > 0; end--) {
		[d[0], d[end]] = [d[end], d[0]];
		siftDownRange(d, 0, end, le, ge, lt);
	}
	return h;
}

/**
 * `append` of `size`-byte elements: `other`'s elements appended to the larger of the two,
 * as `Vec::append` reserves for them, which `h` then holds, and `other` left empty,
 * keeping its capacity.
 */
export function heapAppend<T>(h: BinaryHeap<T>, other: BinaryHeap<T>, le: Test<T>, ge: Test<T>, lt: Test<T>, size: number): void {
	if (h.items.length < other.items.length) {
		[h.items, other.items] = [other.items, h.items];
		[h.cap, other.cap] = [other.cap, h.cap];
		[h.peak, other.peak] = [other.peak, h.peak];
	}
	const start = h.items.length;
	vecAppend(h, other, size);
	rebuildTail(h.items, start, le, ge, lt);
}

/**
 * `extend`, taking `items` as `take` says and reserving as its `Vec`'s `iterExtend` does,
 * and then the heap rebuilt past what it held.
 */
export function heapExtend<T>(h: BinaryHeap<T>, items: Iter<T>, le: Test<T>, ge: Test<T>, lt: Test<T>, take?: Take, size?: number, trusted = false): BinaryHeap<T> {
	const start = h.items.length;
	vecExtending(h, start, items, (item) => h.items.push(item), take, size, trusted, h);
	rebuildTail(h.items, start, le, ge, lt);
	return h;
}

/**
 * `retain`: the elements `f` keeps, in order, each other dropped by `drop` as `f` rejects
 * it, and then the heap rebuilt from the first it rejected.
 */
export function heapRetain<T>(h: BinaryHeap<T>, f: (item: T) => boolean, le: Test<T>, ge: Test<T>, lt: Test<T>, drop?: (item: T) => void): void {
	const d = h.items;
	const n = d.length;
	let from = n;
	let kept = 0;
	for (let i = 0; i < n; i++) {
		const item = d[i];
		if (f(item)) d[kept++] = item;
		else {
			if (from === n) from = i;
			drop?.(item);
		}
	}
	vecKeep(h);
	d.length = kept;
	rebuildTail(d, from, le, ge, lt);
}

/** `clear`, which leaves the heap empty, keeping its capacity, and then drops each element in order. */
export function heapClear<T>(h: BinaryHeap<T>, drop?: (item: T) => void): void {
	vecKeep(h);
	dropAll(h.items.splice(0), drop);
}

/** `drain`: an iterator of the elements in order, which leaves the heap empty, keeping its capacity, and drops those it still holds by `drop`. */
export function heapDrain<T>(h: BinaryHeap<T>, drop?: (item: T) => void): Iter<T> {
	vecKeep(h);
	return iterSlice(h.items.splice(0), drop);
}

/** `iter` and `into_iter`: the elements in order, any not read dropped by `drop` as the iterator drops, over its `Vec`'s buffer. */
export function heapIter<T>(h: BinaryHeap<T>, drop?: (item: T) => void): Iter<T> {
	return vecIntoIter(h, drop);
}

/** The heap's drop: each element in order. */
export function heapDrop<T>(h: BinaryHeap<T>, drop: (item: T) => void): void {
	dropAll(h.items, drop);
}

/** `clone`, with each element through its `Clone`, or as itself without one, as its `Vec`'s. */
export function heapClone<T>(h: BinaryHeap<T>, item: (value: T) => T = itself): BinaryHeap<T> {
	return vecFrom(h.items.map((value) => item(value)));
}

/**
 * `PeekMut`, the greatest element lent: a place, which writes it, of a heap with at least
 * one. Once `DerefMut` lends it of a heap of more, its drop sifts it down.
 */
export class PeekMut<T> implements Place<T> {
	readonly h: BinaryHeap<T>;
	readonly le: Test<T>;
	readonly ge: Test<T>;
	readonly lt: Test<T>;
	/** The heap's elements `DerefMut` hid behind a heap of the top alone, as std's `set_len(1)` does, so a forgotten guard leaks them as std's does. */
	t: T[] | undefined;

	constructor(h: BinaryHeap<T>, le: Test<T>, ge: Test<T>, lt: Test<T>) {
		this.h = h;
		this.le = le;
		this.ge = ge;
		this.lt = lt;
	}

	get v(): T {
		return this.h.items[0];
	}

	set v(value: T) {
		this.h.items[0] = value;
	}
}

/** `peek_mut`: the greatest element lent, or `undefined` if empty. */
export function heapPeekMut<T>(h: BinaryHeap<T>, le: Test<T>, ge: Test<T>, lt: Test<T>): PeekMut<T> | undefined {
	return h.items.length === 0 ? undefined : new PeekMut(h, le, ge, lt);
}

/** `PeekMut`'s `DerefMut`, which hides the rest of a heap of more than the one element until the guard drops. */
export function peekMutDerefMut<T>(p: PeekMut<T>): void {
	const d = p.h.items;
	if (d.length > 1) {
		vecKeep(p.h);
		p.t = d;
		p.h.items = [d[0]];
	}
}

/** The elements `DerefMut` hid back in the heap, with the top as written; whether there were any. */
function peekMutRestore<T>(p: PeekMut<T>): boolean {
	const t = p.t;
	if (t === undefined) return false;
	t[0] = p.h.items[0];
	p.h.items = t;
	p.t = undefined;
	return true;
}

/** `PeekMut`'s drop: the element sifted down from the top, if `DerefMut` lent it. */
export function peekMutDrop<T>(p: PeekMut<T>): void {
	if (peekMutRestore(p)) siftDownRange(p.h.items, 0, p.h.items.length, p.le, p.ge, p.lt);
}

/** `PeekMut::pop`: the heap's `pop`, which leaves the drop nothing to sift. */
export function peekMutPop<T>(p: PeekMut<T>): T {
	peekMutRestore(p);
	return heapPop(p.h, p.le) as T;
}

/** std's `sift_up`: the element at `pos` moved up past each parent it is not `le`, but not above `start`. */
function siftUp<T>(d: T[], start: number, pos: number, le: Test<T>): number {
	const item = d[pos];
	while (pos > start) {
		const parent = (pos - 1) >>> 1;
		if (le(item, d[parent])) break;
		d[pos] = d[parent];
		pos = parent;
	}
	d[pos] = item;
	return pos;
}

/** std's `sift_down_range`: the element at `pos` moved down past each greater child, within the first `end`. */
function siftDownRange<T>(d: T[], pos: number, end: number, le: Test<T>, ge: Test<T>, lt: Test<T>): number {
	const item = d[pos];
	let child = 2 * pos + 1;
	while (child <= Math.max(end - 2, 0)) {
		if (le(d[child], d[child + 1])) child++;
		if (ge(item, d[child])) {
			d[pos] = item;
			return pos;
		}
		d[pos] = d[child];
		pos = child;
		child = 2 * pos + 1;
	}
	if (child === end - 1 && lt(item, d[child])) {
		d[pos] = d[child];
		pos = child;
	}
	d[pos] = item;
	return pos;
}

/** std's `sift_down_to_bottom`: the element at `pos` moved down to a leaf past the greater child at each level, then sifted up. */
function siftDownToBottom<T>(d: T[], pos: number, le: Test<T>): void {
	const end = d.length;
	const start = pos;
	const item = d[pos];
	let child = 2 * pos + 1;
	while (child <= Math.max(end - 2, 0)) {
		if (le(d[child], d[child + 1])) child++;
		d[pos] = d[child];
		pos = child;
		child = 2 * pos + 1;
	}
	if (child === end - 1) {
		d[pos] = d[child];
		pos = child;
	}
	d[pos] = item;
	siftUp(d, start, pos, le);
}

/** std's `rebuild_tail`: the elements from `start` on added to the heap before them, by a rebuild or by sifting each up, as std's estimate picks. */
function rebuildTail<T>(d: T[], start: number, le: Test<T>, ge: Test<T>, lt: Test<T>): void {
	if (start === d.length) return;
	const tail = d.length - start;
	const better = start < tail || (d.length <= 2048 ? 2 * d.length < tail * (31 - Math.clz32(start)) : 2 * d.length < tail * 11);
	if (better) rebuild(d, le, ge, lt);
	else for (let i = start; i < d.length; i++) siftUp(d, 0, i, le);
}

/** std's `rebuild`: each parent sifted down, the last first. */
function rebuild<T>(d: T[], le: Test<T>, ge: Test<T>, lt: Test<T>): void {
	for (let n = d.length >>> 1; n > 0; ) {
		n--;
		siftDownRange(d, n, d.length, le, ge, lt);
	}
}

// Tuples of collections, which std's `Extend` of 2- through 12-tuples extends by each
// collection's `extend_reserve` and `extend_one` (core/src/iter/traits/collect.rs). A
// `Part` stands in for one collection's.

/**
 * A collection of type `C` as a tuple's `Extend` adds to it: `reserve` is its
 * `extend_reserve`, if it reserves, and `one` its `extend_one` of an item, giving the
 * collection back, which for a `String` is a new one.
 */
export interface Part<C, T> {
	reserve?(c: C, additional: number): void;
	one(c: C, item: T): C;
}

/**
 * `Iterator::partition` into `sides`, the collection's two defaults: each item `f` tests
 * true into the first, and the rest into the second, by `part`'s `extend_one`, through
 * `fold`, which drops `f` as it ends if it `owns` it.
 */
export function iterPartition<T, C>(it: Iter<T>, f: (item: T) => boolean, sides: [C, C], part: Part<C, T>, drops = false, owns = false): [C, C] {
	const g = (_: undefined, item: T): undefined => {
		const i = f(item) ? 0 : 1;
		sides[i] = part.one(sides[i], item);
	};
	iterFold(it, undefined, owns ? closure(g, () => dispose(f)) : g, drops, owns);
	return sides;
}

/**
 * `Extend::extend` of `t`, a tuple of collections, by `items`, and `collect` into the
 * tuple of their defaults: each reserves the lower bound of the size hint, if any, in
 * order, and then each item's fields extend them in order, by `for_each` as `take` says.
 */
export function tupleExtend<T extends unknown[]>(t: T, items: Iter<unknown[]>, parts: Part<unknown, unknown>[], take?: Take): T {
	tupleReserve(t, parts, items.sizeHint()[0]);
	return extending(t, items, (item) => tupleOne(t, parts, item), take);
}

/** A tuple's `extend_reserve` of `additional` more in each of its collections, where the size hint has a lower bound. */
function tupleReserve(t: unknown[], parts: Part<unknown, unknown>[], additional: number): void {
	if (additional === 0) return;
	for (let i = 0; i < parts.length; i++) parts[i].reserve?.(t[i], additional);
}

/** A tuple's `extend_one`: each field of `item` into its collection, in order. */
function tupleOne(t: unknown[], parts: Part<unknown, unknown>[], item: unknown[]): void {
	for (let i = 0; i < parts.length; i++) t[i] = parts[i].one(t[i], item[i]);
}

/** A tuple of collections within a tuple, which reserves and extends each of its own. */
export function tuplePart(parts: Part<unknown, unknown>[]): Part<unknown[], unknown[]> {
	return {
		reserve: (t, additional) => tupleReserve(t, parts, additional),
		one: (t, item) => {
			tupleOne(t, parts, item);
			return t;
		},
	};
}

/** A 1-tuple's `Extend::extend` of `t`, which is its collection's own `extend`, by `extend`, of each item's field. */
export function tupleExtendOne<C, T>(t: [C], items: Iter<[T]>, extend: (c: C, items: Iter<T>) => C): [C] {
	t[0] = extend(t[0], iterMap(items, (item) => item[0]));
	return t;
}

/** `part` of a borrowed `Copy` item, which it adds a `copy` of. */
export function copiedPart<C, T>(part: Part<C, T>, copy: (item: T) => T): Part<C, T> {
	return { reserve: part.reserve, one: (c, item) => part.one(c, copy(item)) };
}

/** A collection kept in a vector, which `push` adds to and `reserve` reserves in, where it knows its elements' size. */
function vectorPart<C, T>(push: (c: C, item: T) => void, reserve?: (c: C, additional: number) => void): Part<C, T> {
	return {
		reserve,
		one: (c, item) => {
			push(c, item);
			return c;
		},
	};
}

/** A `Vec` of `size`-byte elements, which pushes each. */
export function vecPart<T>(size?: number): Part<Vec<T>, T> {
	return vectorPart(
		(v, item) => {
			v.items.push(item);
		},
		size === undefined ? undefined : (v, additional) => vecReserve(v, additional, size),
	);
}

/** A `Vec<u8>`, as `vecPart`. */
export function byteVecPart(): Part<ByteVec, number> {
	return vectorPart(byteVecPush, byteVecReserve);
}

/** A `VecDeque`, which pushes each at the back. */
export function dequePart<T>(): Part<VecDeque<T>, T> {
	return vectorPart(dequePushBack, dequeReserve);
}

/** A `LinkedList`, which pushes each at the back, and never reserves. */
export function listPart<T>(): Part<LinkedList<T>, T> {
	return {
		one: (l, item) => {
			listPushBack(l, item);
			return l;
		},
	};
}

/** A `BinaryHeap` of `size`-byte elements, which pushes each, sifting it up by `le`. */
export function heapPart<T>(le: Test<T>, size?: number): Part<BinaryHeap<T>, T> {
	return vectorPart(
		(h, item) => heapPush(h, item, le),
		size === undefined ? undefined : (h, additional) => vecReserve(h, additional, size),
	);
}

/** A `HashMap`, which inserts each entry as `hashExtend` does, and reserves as `extendReserve`. */
export function hashPart<K, V>(k?: (key: K) => void, v?: (value: V) => void, max?: number): Part<HashMap<K, V>, [K, V]> {
	return {
		reserve: (m, additional) => extendReserve(m, additional, max),
		one: (m, item) => {
			const old = hashInsert(m, item[0], item[1], k);
			if (old !== undefined) v?.(old);
			return m;
		},
	};
}

/** A `HashSet`, which adds each key as `hashExtendKeys` does. */
export function hashKeysPart<K>(k?: (key: K) => void, max?: number): Part<HashSet<K>, K> {
	return {
		reserve: (m, additional) => extendReserve(m, additional, max),
		one: (m, key) => {
			hashAdd(m, key, k);
			return m;
		},
	};
}

/** A `BTreeMap`, which inserts each entry as `btreeExtend` does, and never reserves. */
export function btreePart<K, V>(k?: (key: K) => void, v?: (value: V) => void): Part<BTreeMap<K, V>, [K, V]> {
	return {
		one: (m, item) => {
			const old = btreeInsert(m, item[0], item[1], k);
			if (old !== undefined) v?.(old);
			return m;
		},
	};
}

/** A `BTreeSet`, which adds each key as `btreeExtendKeys` does. */
export function btreeKeysPart<K>(k?: (key: K) => void): Part<BTreeSet<K>, K> {
	return {
		one: (m, key) => {
			btreeAdd(m, key, k);
			return m;
		},
	};
}

/**
 * `Extend::extend` of `c`, the crate's own collection, by `items`, through `extend`, which
 * takes an `rs.Place` of it if `place`, giving it back.
 */
export function ownExtend<C, I>(c: C, items: I, extend: (c: C, items: I) => void): C;
export function ownExtend<C, I>(c: C, items: I, extend: (c: Place<C>, items: I) => void, place: true): C;
export function ownExtend<C, I>(c: C, items: I, extend: (c: never, items: I) => void, place?: boolean): C {
	if (!place) {
		(extend as (c: C, items: I) => void)(c, items);
		return c;
	}
	const p = { v: c };
	(extend as (c: Place<C>, items: I) => void)(p, items);
	return p.v;
}

/**
 * The crate's own collection, which keeps the defaults stable Rust can't override:
 * `extend_reserve` does nothing, and `extend_one` is its `extend`, as `ownExtend`, of
 * `Some(item)`, which is the item.
 */
export function ownPart<C, T>(extend: (c: C, items: T) => void): Part<C, T>;
export function ownPart<C, T>(extend: (c: Place<C>, items: T) => void, place: true): Part<C, T>;
export function ownPart<C, T>(extend: (c: never, items: T) => void, place?: true): Part<C, T> {
	if (place) return { one: (c, item) => ownExtend(c, item, extend as (c: Place<C>, items: T) => void, place) };
	return { one: (c, item) => ownExtend(c, item, extend as (c: C, items: T) => void) };
}

/** `()`, which extends by `()`s. */
export const UNIT_PART: Part<undefined, undefined> = { one: () => undefined };
