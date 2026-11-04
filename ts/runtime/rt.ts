// The runtime of rust2-ts output: what Rust's std does that plain JS does not.
//
// Every operation is its own exported function, named by what it acts on, `sliceRange`
// or `fmtPad`, so a bundler drops every one translated code never calls. Methods are
// left only where a call dispatches on its receiver: an iterator's `next`, a writer's
// `writeStr`, a future's `poll`, a waker's, and `[Symbol.dispose]`.

import type { Shared } from "./rt/bytes.ts";
import type { Ordering } from "./rt/collections.ts";
import type { ParseFloatError } from "./rt/float.ts";
import { type char, type ParseCharError, type ParseIntError, type TryFromIntError, type u32, type usize, u8Debug, usizeDebug, usizeDisplay } from "./rt/int.ts";
import type { Iter, Range, RangeInclusive } from "./rt/iter.ts";
import { drop as dispose, dropAll, partPlace, type Pending } from "./rt/task.ts";

import { allocError, CAPACITY_OVERFLOW, type TryReserveError } from "./rt/alloc.ts";
import { AtIndex } from "./rt/collections.ts";
import { escapeDebug } from "./rt/char.ts";
export * from "./rt/alloc.ts";
export * from "./rt/ascii.ts";
export * from "./rt/bytes.ts";
export * from "./rt/char.ts";
export * from "./rt/collections.ts";
export * from "./rt/float.ts";
export * from "./rt/hash.ts";
export * from "./rt/int.ts";
export * from "./rt/iter.ts";
export * from "./rt/location.ts";
export * from "./rt/rand.ts";
export * from "./rt/sort.ts";
export * from "./rt/str.ts";
export * from "./rt/task.ts";
export * from "./rt/time.ts";

/** A Rust panic. */
export class Panic extends Error {
	override name = "Panic";
}

/** An abort that runs no panic hook, as std's `rtabort!`: a `Panic`, so catching one catches both. */
export class Abort extends Panic {
	override name = "Abort";
}

/** Panic with `message`. */
export function panic(message: string): never {
	throw new Panic(message);
}

/** `resume_unwind`, which aborts as `panic=abort`'s runtime does, running no panic hook. */
export function resumeUnwind(): never {
	throw new Abort("resume_unwind");
}

/**
 * Panic with a formatted message, which keeps what was written before a value failed to
 * format, as std's does.
 */
export function panicFmt(message: Arguments): never {
	const out = new StringWriter();
	write(out, message);
	throw new Panic(out.s);
}

/** A `match` without arms, on a value Rust proved can't exist, such as a `(Void, u8)`. */
export function unreachable(_value: unknown): never {
	throw new Panic("internal error: entered unreachable code");
}

/** `assert_eq!` or `assert_ne!` failing: `op` is `==` or `!=`. */
export function assertFailed(op: string, left: Fmt, right: Fmt, args?: Arguments): never {
	const failed: Arguments = args === undefined ? [" failed"] : [" failed: ", (f) => fmtWriteFmt(f, args)];
	panicFmt(["assertion `left ", op, " right`", ...failed, "\n  left: ", left, "\n right: ", right]);
}

// Places

/**
 * A `&mut` to a value JS copies, such as a number, which no JS value can point at: where
 * the value is, read and written through `v`.
 */
export interface Place<T> {
	v: T;
}

/** A place read and written by `get` and `set`: a local's, a field's, or an element's. */
class Accessed<T> implements Place<T> {
	private readonly read: () => T;
	private readonly write: (value: T) => void;

	constructor(read: () => T, write: (value: T) => void) {
		this.read = read;
		this.write = write;
	}

	get v(): T {
		return this.read();
	}

	set v(value: T) {
		this.write(value);
	}
}

/**
 * The payload's place of the `Option` at `p`, whose `None` is `none`: `p` itself, as
 * `Some(x)` is `x`, or `undefined` while it is `None`.
 */
export function optionAsMut<T>(p: Place<T | undefined>): Place<T> | undefined;
export function optionAsMut<T>(p: Place<T | None>, none: None): Place<T> | undefined;
export function optionAsMut<T>(p: Place<T | undefined | None>, none?: None): Place<T> | undefined {
	return p.v === none ? undefined : (p as Place<T>);
}

/**
 * The payload's place of the `Result` at `p`, while it is `Ok`: writing it makes `p` a
 * new `Ok`, as a `Result` is never changed in place.
 */
export function resultAsMut<T, E>(p: Place<Result<T, E>>): Place<T> | undefined {
	if (!p.v.ok) return undefined;
	return place(
		() => (p.v as { readonly value: T }).value,
		(value) => {
			p.v = Ok(value);
		},
	);
}

/** The place `get` reads and `set` writes. */
export function place<T>(get: () => T, set: (value: T) => void): Place<T> {
	return new Accessed(get, set);
}

// Raw pointers

/** A null raw pointer, of any type: nothing reads through it. */
export const NULL: never = null as never;

/** A dangling raw pointer, `NonNull::dangling`, of any type: nothing reads through it. */
export const DANGLING: never = /* @__PURE__ */ Object.freeze({}) as never;

/** The raw pointer `p` unsized by `make`, or `p` itself if it is null or dangling, which point at nothing. */
export function rawUnsize<P, Q>(p: P, make: (p: P) => Q): Q {
	return p === NULL || p === DANGLING ? (p as never) : make(p);
}

/**
 * `rawUnsize` of `p`, the place of a value, to the place of what `make` makes of whatever
 * value is there as it is read, so it may be uninitialized or replaced until then. Nothing
 * writes an unsized value whole.
 */
export function rawUnsizePlace<T, U>(p: Place<T>, make: (value: T) => U): Place<U> {
	return rawUnsize(p, (p) => place(() => make(p.v), unreachable));
}

/** A dangling pointer to `()`, std's every one: a place that holds only `()`. */
export const DANGLING_UNIT: Place<undefined> = { v: undefined };

/** The types of the pointers cast to `*const ()`, by their JS value, which is its identity. */
const ERASED = /* @__PURE__ */ new WeakMap<object, string[]>();

/**
 * A `*const ()` at an address no memory is at, as a dangling pointer cast to one is, by
 * that address: `DANGLING_UNIT` at 1, as `NonNull::<()>::dangling()`'s is.
 */
const ERASED_AT = /* @__PURE__ */ new Map<number, Place<undefined>>([[1, DANGLING_UNIT]]);

/** The address of `p`, a `*const ()` that is one of `ERASED_AT`'s, if it is. */
function erasedAddress(p: Place<undefined>): number | undefined {
	for (const [addr, at] of ERASED_AT) if (at === p) return addr;
	return undefined;
}

/**
 * `p as *const ()`, of a pointer to a value of Rust type `type`, aligned to `align` bytes:
 * the same identity, which remembers `type` for a cast back. A dangling pointer is at
 * `align`, which a cast back keeps.
 */
export function ptrErase(p: object, type: string, align: number): Place<undefined> {
	if (p === NULL) return NULL;
	if (p === DANGLING || p instanceof Address) {
		const addr = p === DANGLING ? align : (p as Address).addr;
		let at = ERASED_AT.get(addr);
		if (at === undefined) ERASED_AT.set(addr, (at = { v: undefined }));
		return at;
	}
	const types = ERASED.get(p);
	if (types === undefined) ERASED.set(p, [type]);
	else if (!types.includes(type)) types.push(type);
	return p as Place<undefined>;
}

/**
 * `p as *const T`, of a `*const ()`, where `T` is Rust type `type` aligned to `align`
 * bytes: the pointer `ptrErase` erased, which must point at a `type`, since JS keeps no
 * bytes to read as another, or one at the address it was.
 */
export function ptrUnerase(p: Place<undefined>, type: string, align: number): unknown {
	if (p === NULL) return NULL;
	const addr = erasedAddress(p);
	if (addr !== undefined) return addr === align ? DANGLING : addressAt(addr);
	const types = ERASED.get(p);
	if (types?.includes(type)) return p;
	const was = types === undefined ? "a value no cast to `*const ()` erased" : types.map((t) => `\`${t}\``).join(" or ");
	throw new Error(`rust2-ts: a \`*const ()\` cast back to a pointer to \`${type}\`, which points at ${was}`);
}

/**
 * A generic type's pointer or reference to a `T` that only an instance decides: the object,
 * the `Place` of a value JS copies, or the `ByteSlice` of a `[u8; N]`'s bytes.
 */
export type Ptr<T> = T | Place<T> | ByteSlice;

/** A generic type's `[T; N]`: the array, or the `Uint8Array` of a `[u8; N]`. */
export type ArrayOf<T> = T[] | Uint8Array;

/** A generic type's `[T]`, as behind a reference: the array, or the `ByteSlice` of a `[u8]`. */
export type SliceOf<T> = T[] | ByteSlice;

/** A generic type's `Vec<T>`: the array, or the `ByteVec` of a `Vec<u8>`. */
export type VecOf<T> = T[] | ByteVec;

/** A generic type's `Cow<[T]>`: the `Cow`, or the `ByteCow` of a `Cow<[u8]>`. */
export type CowOf<T> = Cow<T> | ByteCow;

/**
 * The pointer to element `index` of `s`, a slice, a `[T; N]`, or a `Vec`'s storage. A null
 * or dangling pointer to an array is one to its first element, at the same address.
 */
export function sliceAsPtr<T>(s: { [i: number]: T }, index = 0): Place<T> {
	if (s === NULL) return NULL;
	if (s === DANGLING) return DANGLING;
	const view = (s as { [VIEW]?: View<T> })[VIEW];
	const [array, at] = view ? [view.s, view.start + index] : [s, index];
	// Null's is `null`, which `??` would pass over.
	const from = EMPTY.get(array) as Place<T> | undefined;
	return from !== undefined ? from : new AtIndex(array, at);
}

/** The pointer to byte `index` of `s`, or `s` if it is null or dangling, as `sliceAsPtr`'s. */
export function byteSliceAsPtr(s: ByteSlice, index = 0): Place<number> {
	if (s === NULL) return NULL;
	if (s === DANGLING) return DANGLING;
	const from = EMPTY.get(s.data) as Place<number> | undefined;
	if (from !== undefined) return from;
	return new AtIndex(s.data, s.start + index);
}

/** The pointer to the first byte of `v`'s storage. */
export function byteVecAsPtr(v: ByteVec): Place<number> {
	return new AtIndex(v.data, 0);
}

/** A pointer wrapping arithmetic moved off null or a dangling one: an address no memory is at. */
class Address implements Place<never> {
	readonly addr: number;

	constructor(addr: number) {
		this.addr = addr;
	}

	get v(): never {
		return nowhere();
	}

	set v(_: never) {
		nowhere();
	}
}

/** Each `Address` by its address, while any pointer holds it, so pointers at one compare as one. */
const ADDRESSES = /* @__PURE__ */ new Map<number, WeakRef<Address>>();

/** Forget an address once no pointer holds its `Address`. */
const UNHELD = /* @__PURE__ */ new FinalizationRegistry<number>((addr) => {
	if (ADDRESSES.get(addr)?.deref() === undefined) ADDRESSES.delete(addr);
});

/** The pointer at `addr`, which no memory is at: the same `Address` as every other one there. */
function addressAt(addr: number): Address {
	let at = ADDRESSES.get(addr)?.deref();
	if (at === undefined) {
		at = new Address(addr);
		ADDRESSES.set(addr, new WeakRef(at));
		UNHELD.register(at, addr);
	}
	return at;
}

function nowhere(): never {
	throw new Error("rust2-ts: a read or write through a pointer to no memory");
}

/** The array of one element that a pointer to a value in no array points into, by its place. */
const LONE = /* @__PURE__ */ new WeakMap<Place<unknown>, unknown[]>();

/** What an array of one element `elemOf` made reads as the pointer it was made of. */
const LONE_OF: unique symbol = /* @__PURE__ */ Symbol("loneOf");

/**
 * The pointer `p` was moved from, if `p` is element 0 of the array of one that a pointer
 * to a value in no array points into, as arithmetic moving back to it gives: Rust's same
 * address, which a table keyed by the pointer finds. Otherwise `p`.
 */
export function ptrHome<P extends object>(p: P): P {
	if (!(p instanceof AtIndex) || p.index !== 0) return p;
	return ((p.array as { [LONE_OF]?: P })[LONE_OF] ?? p) as P;
}

/** The traps of an array of one element, the value at `p`. */
class Lone<T> implements ProxyHandler<T[]> {
	readonly p: Place<T>;

	constructor(p: Place<T>) {
		this.p = p;
	}

	get(target: T[], key: string | symbol): unknown {
		if (key === LONE_OF) return this.p;
		return key === "0" ? this.p.v : Reflect.get(target, key);
	}

	set(target: T[], key: string | symbol, value: T): boolean {
		if (key !== "0") return Reflect.set(target, key, value);
		this.p.v = value;
		return true;
	}

	has(target: T[], key: string | symbol): boolean {
		return key === "0" || Reflect.has(target, key);
	}

	getOwnPropertyDescriptor(target: T[], key: string | symbol): PropertyDescriptor | undefined {
		if (key !== "0") return Reflect.getOwnPropertyDescriptor(target, key);
		return { value: this.p.v, writable: true, enumerable: true, configurable: true };
	}
}

/**
 * Where `p` points, as an element of an array: its own, or for a pointer to a value in no
 * array, element 0 of an array of one that reads and writes it, as Rust treats one. Null
 * and dangling pointers point into an array of none, so any access through them fails.
 */
function elemOf<T>(p: Place<T>): AtIndex<T> {
	if (p instanceof AtIndex) {
		// A pointer into a mutable part of an array points into the array.
		const view = (p.array as { [VIEW]?: View<T> })[VIEW];
		return view ? new AtIndex(view.s, view.start + p.index) : p;
	}
	if (p === NULL || p === DANGLING) return new AtIndex([] as T[], 0);
	let lone = LONE.get(p) as T[] | undefined;
	if (!lone) {
		lone = new Proxy(new Array<T>(1), new Lone(p));
		LONE.set(p, lone);
	}
	return new AtIndex(lone, 0);
}

/**
 * The index `n` elements of `size` bytes past `index`, where wasm32's 32-bit addresses
 * wrap: two indices whose addresses differ by a multiple of 2^32 are the same.
 */
function wrapIndex(index: number, n: number, size: number): number {
	// The addresses of indices a period apart are the same.
	const low = size & -size;
	const period = low ? 2 ** 32 / low : 2 ** 32;
	const at = (index + n) % period;
	return at >= period / 2 ? at - period : at < -period / 2 ? at + period : at;
}

/** `p.add(n)` and `p.offset(n)`: `n` elements past where `p` points. */
export function ptrAdd<T>(p: Place<T>, n: number): Place<T> {
	if (n === 0) return p;
	const e = elemOf(p);
	return new AtIndex(e.array, e.index + n);
}

/**
 * `p.wrapping_add(n)` and `wrapping_offset`, `n` an offset of `size`-byte elements read
 * as wasm32's `isize`, wrapping as a 32-bit address does. Null is at address 0, and a
 * dangling pointer at `align`, which the arithmetic moves by address.
 */
export function ptrWrappingAdd<T>(p: Place<T>, n: number, size: number, align: number): Place<T> {
	if (n === 0) return p;
	if (p === NULL || p === DANGLING || p instanceof Address) {
		const base = p === NULL ? 0 : p === DANGLING ? align : (p as Address).addr;
		const addr = Number((BigInt(base) + BigInt(n | 0) * BigInt(size)) & 0xffffffffn);
		return addr === 0 ? NULL : addr === align ? DANGLING : (addressAt(addr) as Place<T>);
	}
	const e = elemOf(p);
	return new AtIndex(e.array, wrapIndex(e.index, n | 0, size));
}

/** `p.offset_from(origin)`: how many elements `p` is past `origin`, in the same array or byte buffer. */
export function ptrOffsetFrom<T>(p: Place<T>, origin: Place<T>): number {
	// Equal addresses are none apart, in no allocation as in one.
	if (p === origin || (p instanceof Address && origin instanceof Address && p.addr === origin.addr)) return 0;
	const [a, b] = [elemOf(p), elemOf(origin)];
	if (a.array === b.array) return a.index - b.index;
	// Views of one buffer's bytes, such as a slice's and its iterator's, are one array.
	if (a.array instanceof Uint8Array && b.array instanceof Uint8Array && a.array.buffer === b.array.buffer) {
		return a.array.byteOffset + a.index - (b.array.byteOffset + b.index);
	}
	throw new Error("rust2-ts: `offset_from` of pointers JS keeps in different arrays");
}

/**
 * `ptr::copy` and `copy_nonoverlapping` of `n` elements from `src` to `dst`, as if through
 * a buffer between, each made a value of its own by `copy` where the crate writes into its parts.
 */
export function ptrCopy<T>(src: Place<T>, dst: Place<T>, n: number, copy?: (value: T) => T): void {
	if (n === 0) return;
	const [from, to] = [elemOf(src), elemOf(dst)];
	from.reach(n);
	to.reach(n);
	if (ArrayBuffer.isView(from.array) && ArrayBuffer.isView(to.array)) {
		const bytes = from.array as unknown as Uint8Array;
		(to.array as unknown as Uint8Array).set(bytes.subarray(from.index, from.index + n), to.index);
		return;
	}
	const values = Array.from({ length: n }, (_, i) => from.array[from.index + i]);
	// Memory nothing wrote yet is copied as it is, which no value's copy can read.
	for (let i = 0; i < n; i++) to.array[to.index + i] = copy && values[i] !== undefined ? copy(values[i]) : values[i];
}

/**
 * The pointer each empty slice made from a pointer into no array was made from, by the
 * storage every part of it shares, which `sliceAsPtr` and `byteSliceAsPtr` give back.
 */
const EMPTY = /* @__PURE__ */ new WeakMap<object, Place<unknown>>();

/** `slice::from_raw_parts` of `len` elements from `p`. */
export function ptrSlice<T>(p: Place<T>, len: number): T[] {
	// None from a pointer into no array, such as null's or a dangling one's, keeps it.
	if (len === 0 && !(p instanceof AtIndex)) {
		const none = Object.freeze([]) as unknown as T[];
		EMPTY.set(none, p);
		return none;
	}
	const at = elemOf(p);
	at.reach(len);
	const array = at.array as T[];
	return at.index === 0 && len === array.length ? array : sliceView(array, at.index, at.index + len);
}

/**
 * A raw pointer to a slice, `*const [T]`, `*mut [T]`, or `NonNull<[T]>`: where its first
 * element is and how many it spans, which nothing checks until a read through it, as in Rust.
 */
export class RawSlice<T> {
	readonly ptr: Place<T>;
	readonly length: number;
	/** What the first read through it made, which later reads share while it spans `length`. */
	#made: T[] | ByteSlice | undefined;
	/** Where `#made` starts in the array beneath it, if anywhere. */
	#at: AtIndex<T> | undefined;
	/** Whether `#made` is the array itself, which spans more once it grows. */
	#whole = false;

	constructor(ptr: Place<T>, length: number) {
		this.ptr = ptr;
		this.length = length;
	}

	/** The slice it points at, read through it, which must be there. */
	deref(): T[] {
		const made = this.#made as T[] | undefined;
		if (made !== undefined) {
			// Fails as a new read would once the array has shrunk past it.
			this.#at?.reach(this.length);
			if (!this.#whole || made.length === this.length) return made;
		}
		const slice = ptrSlice(this.ptr, this.length);
		this.#at = this.ptr instanceof AtIndex ? elemOf(this.ptr) : undefined;
		this.#whole = slice === this.#at?.array;
		return (this.#made = slice);
	}

	/** The bytes it points at, read through it, `shared` as `ptrByteSlice` takes it. */
	derefBytes(this: RawSlice<number>, shared: boolean): ByteSlice {
		if (this.#made !== undefined) return this.#made as ByteSlice;
		const made = ptrByteSlice(this.ptr, this.length, shared);
		// A copy of a byte in no array would miss a later write to the byte.
		if (this.length === 0 || (this.ptr instanceof AtIndex && this.ptr.array instanceof Uint8Array)) this.#made = made;
		return made;
	}
}

/** The raw pointer to `s`, a slice, a `[T; N]`, or a `Vec`'s storage. */
export function rawSliceOf<T>(s: { readonly length: number; [i: number]: T }): RawSlice<T> {
	return new RawSlice(sliceAsPtr(s), s.length);
}

/** The raw pointer to the bytes of `s`. */
export function rawByteSliceOf(s: ByteSlice): RawSlice<number> {
	return new RawSlice(byteSliceAsPtr(s), s.length);
}

/**
 * `slice::from_raw_parts` of `len` bytes from `p`. Only a `shared` reference to bytes in
 * no array of bytes may be a copy of them, since nothing writes them while it lives.
 */
export function ptrByteSlice(p: Place<number>, len: number, shared: boolean): ByteSlice {
	// None from a pointer into no array of bytes, such as a lone byte's, keeps it.
	if (len === 0 && !(p instanceof AtIndex && p.array instanceof Uint8Array)) {
		const none = new Uint8Array(0);
		EMPTY.set(none, p);
		return new ByteSlice(none, 0, 0);
	}
	const at = elemOf(p);
	at.reach(len);
	if (at.array instanceof Uint8Array) return new ByteSlice(at.array, at.index, len);
	// A byte in no array, which no `Uint8Array` can share.
	if (!shared) throw new Error("rust2-ts: a raw or mutable slice of a byte in no array of bytes");
	return new ByteSlice(Uint8Array.from({ length: len }, (_, i) => at.array[at.index + i]), 0, len);
}

/**
 * `ptr::write` of `value` through a pointer to an object: `value` copied into it, one
 * object deep, or moved by `$move`, so every pointer to it sees the write.
 */
export function ptrWrite<T extends object>(p: T, value: T): void {
	if (ArrayBuffer.isView(p)) (p as unknown as Uint8Array).set(value as unknown as Uint8Array);
	else if (moved(value)) value.$move(p as T & Moved);
	else Object.assign(p, value);
}

/**
 * `mem::replace` of what a `&mut` to an object points at, in place: `value`'s fields, or
 * its items and capacity for a JS array, moved into `p`, and what `p` held returned as an
 * object of its own, copied as `objectCopy` copies to `depth`.
 */
export function replaceIn<T extends object>(p: T, value: T, depth = 0): T {
	const old = objectCopy(p, depth);
	if (Array.isArray(p)) vecAssign(p, value as unknown[]);
	else ptrWrite(p, value);
	return old;
}

/**
 * A copy of the object `p`, one object deep: its fields, its bytes, or its items and
 * capacity for a JS array. Its items, and theirs to `depth` levels of arrays, are objects
 * a write may land in, as a `sliceIn`'s, which a view of one copies too.
 */
export function objectCopy<T extends object>(p: T, depth = 0): T {
	if (ArrayBuffer.isView(p)) return (p as unknown as Uint8Array).slice() as T;
	if (Array.isArray(p)) {
		const old: unknown[] & Capped = p.slice();
		if ((p as Capped).cap !== undefined) old.cap = (p as Capped).cap;
		if (depth > 0 && writesIn(p)) for (let i = 0; i < old.length; i++) old[i] = objectCopy(old[i] as object, depth - 1);
		return old as T;
	}
	if (moved(p)) return p.$move();
	return Object.assign(Object.create(Object.getPrototypeOf(p)) as T, p);
}

/**
 * An object whose class keeps its state private, where copying its fields can't reach,
 * so `$move`, which no Rust field can be named, moves it: into `to`, or else into a new
 * object of its class, which it returns.
 */
interface Moved {
	$move(to?: this): this;
}

function moved(p: object): p is Moved {
	return "$move" in p;
}

// Bindings

/**
 * `T` if it can be a bound Rust type's value, or else why not: never `undefined` or
 * `Pending`, which `None` and `Poll::Pending` are, an object unless Rust copies it, and
 * disposable exactly when it drops.
 */
export type Bound<T, Copy extends boolean, Drop extends boolean> = [undefined] extends [T]
	? "a bound value can't be undefined, which None is"
	: [typeof Pending] extends [T]
		? "a bound value can't be Pending, which Poll::Pending is"
		: [Copy, T] extends [false, object]
			? Disposes<T, Drop>
			: Copy extends true
				? Disposes<T, Drop>
				: "a bound value Rust doesn't copy must be an object, which a reference changes in place";

/** `T` if it has a `[Symbol.dispose]` exactly when `Drop`, or else why not. */
type Disposes<T, Drop extends boolean> = Drop extends true
	? [T] extends [Disposable]
		? T
		: "a bound value that drops must have a [Symbol.dispose]"
	: [T] extends [{ [Symbol.dispose]: unknown }]
		? "a bound value with a [Symbol.dispose] must bind with drop = true"
		: T;

// Results

/** `Result<T, E>`. */
export type Result<T, E> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly error: E };

/** `Ok(value)`. */
export function Ok<T, E = never>(value: T): Result<T, E> {
	return { ok: true, value };
}

/** `Err(error)`. */
export function Err<E, T = never>(error: E): Result<T, E> {
	return { ok: false, error };
}

/** `Ok(())`, one frozen object every evaluation shares, as nothing can change it. */
export const OK: Result<undefined, never> = /* @__PURE__ */ Object.freeze({ ok: true, value: undefined });

/** `Err(())`, shared as `OK` is. */
export const ERR: Result<never, undefined> = /* @__PURE__ */ Object.freeze({ ok: false, error: undefined });

/** Panic with `message` and `value`'s `Debug`. */
function unwrapFailed<T>(message: string, value: T, debug: Debug<T>): never {
	panicFmt([message, ": ", (f) => debug(value, f)]);
}

// A function that may drop an operand std's would, rather than return it, takes what
// drops it, and drops it where std's does: as it returns, or after any closure it calls.

/** `unwrap`, which shows the error with `debug`. */
export function resultUnwrap<T, E>(r: Result<T, E>, debug: Debug<E>): T {
	if (!r.ok) unwrapFailed("called `Result::unwrap()` on an `Err` value", r.error, debug);
	return r.value;
}

/** `Result::expect`: the value, or a panic with `message` and the error's `Debug`. */
export function resultExpect<T, E>(r: Result<T, E>, message: string, debug: Debug<E>): T {
	if (!r.ok) unwrapFailed(message, r.error, debug);
	return r.value;
}

/** `Result::unwrap_or_else`: the value, or what `f` makes of the error. */
export function resultUnwrapOrElse<T, E>(r: Result<T, E>, f: (error: E) => T): T {
	return r.ok ? r.value : f(r.error);
}

/** `Result::unwrap_err`: the error, or a panic with the value's `Debug`. */
export function resultUnwrapErr<T, E>(r: Result<T, E>, debug: Debug<T>): E {
	if (r.ok) unwrapFailed("called `Result::unwrap_err()` on an `Ok` value", r.value, debug);
	return r.error;
}

// Options, which are `T | undefined`, or `T | None` where `T` can already be `undefined`.
// A function taking one takes that `None` after its own arguments.

/** `None` of an `Option` whose `Some` can already be `undefined`, as `Option<()>`'s is. */
export const None: unique symbol = /* @__PURE__ */ Symbol("None");

/** The type of `None`. */
export type None = typeof None;

/** `f` of what `o` holds, or `None`: `Option::map` of a payload that is never `undefined`. */
export function optionMap<T, U>(o: T | undefined, f: (value: T) => U): U | undefined {
	return o === undefined ? undefined : f(o);
}

/** `Option::unwrap`: the value, or a panic. */
export function optionUnwrap<T>(o: T | undefined): T;
export function optionUnwrap<T>(o: T | None, none: None): T;
export function optionUnwrap<T>(o: T | undefined | None, none?: None): T {
	if (o === none) panic("called `Option::unwrap()` on a `None` value");
	return o as T;
}

/** `Option::expect`: the value, or a panic with `message`. */
export function optionExpect<T>(o: T | undefined, message: string): T;
export function optionExpect<T>(o: T | None, message: string, none: None): T;
export function optionExpect<T>(o: T | undefined | None, message: string, none?: None): T {
	if (o === none) panic(message);
	return o as T;
}

// Slices: arrays, but for bytes, which are `ByteSlice`s.

/** A bounds-checked index into a slice. */
export function idx(slice: { readonly length: number }, index: number): number {
	if (index >= slice.length) panic(`index out of bounds: the len is ${slice.length} but the index is ${index}`);
	return index;
}

/** A bounds-checked read of a slice. */
export function at<T>(slice: ArrayLike<T>, index: number): T {
	return slice[idx(slice, index)];
}

/** Panic unless `start..end` is in a slice of `length`, as std's range checks do. */
export function rangeCheck(length: number, start: number, end: number): void {
	if (start > length) panic(`range start index ${start} out of range for slice of length ${length}`);
	if (end > length) panic(`range end index ${end} out of range for slice of length ${length}`);
	if (start > end) panic(`slice index starts at ${start} but ends at ${end}`);
}

/** `&s[start..end]`, with `end` defaulting to the length. */
export function sliceRange<T>(s: T[], start: number, end: number = s.length): T[] {
	rangeCheck(s.length, start, end);
	return slicePart(s, start, end);
}

/**
 * Elements `start` to `end` of `s`, a copy as a shared part is, or of an empty slice the
 * slice itself, which keeps the pointer it was made from as Rust's part does.
 */
export function slicePart<T>(s: T[], start: number, end: number = s.length): T[] {
	return s.length === 0 ? s : s.slice(start, end);
}

/**
 * `s[start..end]` lent mutably: an array whose every element is `s`'s, read and written
 * where it is, as a JS array has no views of its own. Its own array, holes of the same
 * length, answers for the rest, such as `length` and the methods. A view of a view reads
 * the array beneath both.
 */
export function sliceView<T>(s: T[], start: number, end: number): T[] {
	const outer = (s as { [VIEW]?: View<T> })[VIEW];
	const view = outer ? outer.part(start, end - start) : new View(s, start, end - start);
	return new Proxy(new Array<T>(end - start), view);
}

/** `Option::as_slice` of `o`, whose `None` is `none`: its payload, if any, as `slice::from_ref` makes a slice of one. */
export function optionAsSlice<T>(o: T | undefined): T[];
export function optionAsSlice<T>(o: T | None, none: None): T[];
export function optionAsSlice<T>(o: T, none?: None): T[] {
	return o === none ? [] : [o];
}

/** `optionAsSlice` of a byte. */
export function byteOptionAsSlice(o: number | undefined): ByteSlice {
	return byteSliceFrom(o === undefined ? new Uint8Array(0) : Uint8Array.of(o));
}

/**
 * `slice::from_mut` or `array::from_mut`: a slice of the one value at `p`, read and
 * written there, as a `sliceView` is of its array's.
 */
export function sliceOfPlace<T>(p: Place<T>): T[] {
	const one = {
		get 0() {
			return p.v;
		},
		set 0(value: T) {
			p.v = value;
		},
	};
	ONE.add(one);
	return new Proxy(new Array<T>(1), new View(one, 0, 1));
}

/**
 * `slice::from_mut` or `array::from_mut` of the object a `&mut` is, which no place is
 * known to hold: a slice of it whose whole write lands in it, as what holds it holds the
 * object itself.
 */
export function sliceIn<T extends object>(p: T): T[] {
	return sliceOfPlace(partPlace(p));
}

/** What `sliceOfPlace`'s views read. */
const ONE = /* @__PURE__ */ new WeakSet<object>();

/**
 * Whether a whole write of an element of `s` may land in the object there, as one of
 * `sliceIn`'s does, or of a place in one, which only `sliceOfPlace`'s views can be.
 */
function writesIn(s: object): boolean {
	const view = (s as { [VIEW]?: View<unknown> })[VIEW];
	return view !== undefined && ONE.has(view.s);
}

/**
 * `mem::replace` of element `i` of `s`, a slice of objects: what it held, as an object of
 * its own, copied to `depth` as `objectCopy` copies, where the write lands in it.
 */
export function sliceReplace<T extends object>(s: T[], i: number, value: T, depth = 0): T {
	if (writesIn(s)) return replaceIn(s[i], value, depth);
	const old = s[i];
	s[i] = value;
	return old;
}

/**
 * `mem::swap` of element `i` of `a` and element `j` of `b`, slices of objects: the one
 * whose write lands in its object is copied out first, to `depth` as `sliceReplace`
 * copies, as the other's slot would otherwise take that object itself.
 */
export function sliceSwapAcross<T extends object>(a: T[], i: number, b: T[], j: number, depth = 0): void {
	if (writesIn(b)) [a, i, b, j] = [b, j, a, i];
	b[j] = sliceReplace(a, i, b[j], depth);
}

/** What a `sliceView` answers to its own `View` by. */
const VIEW = /* @__PURE__ */ Symbol("view");

/** A `sliceView`'s traps: of `length` elements of `s` from `start`. */
class View<T> implements ProxyHandler<T[]> {
	s: { [index: number]: T };
	start: number;
	readonly length: number;

	constructor(s: { [index: number]: T }, start: number, length: number) {
		this.s = s;
		this.start = start;
		this.length = length;
	}

	/** The traps of a view of `length` of these elements from `start`. */
	part(start: number, length: number): View<T> {
		return new View(this.s, this.start + start, length);
	}

	/** Element `i` of `s`. */
	item(i: number): T {
		return this.s[i];
	}

	/** A write of element `i` of `s`. */
	put(i: number, value: T): void {
		this.s[i] = value;
	}

	/** Holds a copy of what it views from now on, as a read of the array it views copied it. */
	detach(): void {
		this.s = Array.from({ length: this.length }, (_, i) => this.item(this.start + i));
		this.start = 0;
	}

	/** Where in `s` the element `key` names is, or -1 if it names none. */
	at(key: string | symbol): number {
		if (typeof key !== "string") return -1;
		const i = Number(key);
		return Number.isInteger(i) && i >= 0 && i < this.length && String(i) === key ? this.start + i : -1;
	}

	get(target: T[], key: string | symbol): unknown {
		if (key === VIEW) return this;
		const i = this.at(key);
		return i < 0 ? Reflect.get(target, key) : this.item(i);
	}

	set(target: T[], key: string | symbol, value: T): boolean {
		const i = this.at(key);
		if (i < 0) return Reflect.set(target, key, value);
		this.put(i, value);
		return true;
	}

	has(target: T[], key: string | symbol): boolean {
		return this.at(key) >= 0 || Reflect.has(target, key);
	}

	ownKeys(target: T[]): (string | symbol)[] {
		return [...Array.from({ length: this.length }, (_, i) => String(i)), ...Reflect.ownKeys(target)];
	}

	getOwnPropertyDescriptor(target: T[], key: string | symbol): PropertyDescriptor | undefined {
		const i = this.at(key);
		if (i < 0) return Reflect.getOwnPropertyDescriptor(target, key);
		return { value: this.item(i), writable: true, enumerable: true, configurable: true };
	}
}

/**
 * `as_chunks_mut`'s slice of arrays: element `i` the view of run `i` of `n` elements of `flat`
 * from `offset`, made the first time it is read and kept in `views`. A whole write of a run
 * copies into it, and the view a read took of it first detaches to a copy of what it held,
 * as Rust's read copied the array.
 */
class Runs<T> extends View<T[]> {
	flat: T[];
	offset: number;
	readonly n: number;

	constructor(flat: T[], at: number, n: number, views: T[][], start: number, length: number) {
		super(views, start, length);
		this.flat = flat;
		this.offset = at;
		this.n = n;
	}

	override part(start: number, length: number): Runs<T> {
		return new Runs(this.flat, this.offset, this.n, this.s as T[][], this.start + start, length);
	}

	override item(i: number): T[] {
		const from = this.offset + i * this.n;
		return ((this.s as (T[] | undefined)[])[i] ??= sliceView(this.flat, from, from + this.n));
	}

	override put(i: number, value: T[]): void {
		const views = this.s as (T[] | undefined)[];
		const held = views[i];
		if (held !== undefined) {
			(held as { [VIEW]?: View<T> })[VIEW]!.detach();
			views[i] = undefined;
		}
		const from = this.offset + i * this.n;
		for (let k = 0; k < this.n; k++) this.flat[from + k] = value[k];
	}

	override detach(): void {
		const from = this.offset + this.start * this.n;
		const runs = isRuns(this.flat);
		this.flat = Array.from({ length: this.length * this.n }, (_, k) => (runs ? copyRun(this.flat[from + k]) : this.flat[from + k]));
		this.offset = -this.start * this.n;
		this.s = [];
	}
}

/** Whether `s` is runs, or a view of them, whose elements are views of their runs. */
function isRuns(s: object): boolean {
	const view = (s as { [VIEW]?: unknown })[VIEW];
	return view instanceof Runs || view instanceof ByteRuns;
}

/**
 * `x`, a run read out of runs, as a value of its own, since a write through the runs
 * would change it: a copy of its elements, and of each run it holds as runs at any depth,
 * but not of what its elements point at.
 */
function copyRun<T>(x: T): T {
	if (x instanceof Uint8Array) return x.slice() as T;
	const run = x as unknown as unknown[];
	return (isRuns(run) ? Array.from(run, copyRun) : Array.from(run)) as T;
}

/**
 * `as_chunks_mut` of bytes' slice of arrays: element `i` a `Uint8Array` view of run `i` of
 * `n` bytes of `data` from `offset`, made the first time it is read and kept in `views`, and a
 * whole write of a run copied into it. A `Uint8Array` can't detach as `Runs`' views do, so
 * a read of one by value copies it, as `byValue` has the runtime's own reads do.
 */
class ByteRuns extends View<Uint8Array> {
	data: Uint8Array;
	offset: number;
	readonly n: number;

	constructor(data: Uint8Array, at: number, n: number, views: Uint8Array[], start: number, length: number) {
		super(views, start, length);
		this.data = data;
		this.offset = at;
		this.n = n;
	}

	override part(start: number, length: number): ByteRuns {
		return new ByteRuns(this.data, this.offset, this.n, this.s as Uint8Array[], this.start + start, length);
	}

	override item(i: number): Uint8Array {
		const from = this.offset + i * this.n;
		return ((this.s as (Uint8Array | undefined)[])[i] ??= this.data.subarray(from, from + this.n));
	}

	override put(i: number, value: Uint8Array): void {
		this.data.set(value, this.offset + i * this.n);
	}

	override detach(): void {
		const from = this.offset + this.start * this.n;
		this.data = this.data.slice(from, from + this.length * this.n);
		this.offset = -this.start * this.n;
		this.s = [];
	}

	override get(target: Uint8Array[], key: string | symbol): unknown {
		if (key !== "reverse") return super.get(target, key);
		const s = new Proxy(target, this);
		return () => byValue(s, (values) => values.reverse());
	}
}

/**
 * `op` of `s`, a slice it reads elements of before writes that may overwrite them: of
 * copies of each, then each written back, if `s` holds `ByteRuns`' views, which a write
 * through `s` would change under `op`.
 */
export function byValue<T>(s: T[], op: (s: T[]) => void): void {
	if (!byteRuns(s)) return op(s);
	const values = Array.from(s as Uint8Array[], (run) => run.slice()) as T[];
	op(values);
	for (let i = 0; i < values.length; i++) s[i] = values[i];
}

/** `x`, an element of `s` read before a write that may overwrite it: a copy if `s` holds `ByteRuns`' views. */
function held<T>(s: T[], x: T): T {
	return byteRuns(s) ? ((x as Uint8Array).slice() as T) : x;
}

/** Whether `s` holds `ByteRuns`' views. */
function byteRuns(s: object): boolean {
	return (s as { [VIEW]?: unknown })[VIEW] instanceof ByteRuns;
}

/** `&mut s[start..end]`: the runtime's view of it, checked as `sliceRange` checks it. */
export function sliceRangeMut<T>(s: T[], start: number, end: number = s.length): T[] {
	rangeCheck(s.length, start, end);
	return sliceView(s, start, end);
}

/** `get` of `start..end`: the part, or `undefined` unless it is within the slice. */
export function sliceGetRange<T>(s: T[], start: number, end: number = s.length): T[] | undefined {
	return start <= end && end <= s.length ? slicePart(s, start, end) : undefined;
}

/** `get_mut` of `start..end`: the runtime's view of it, or `undefined` as `sliceGetRange`. */
export function sliceGetRangeMut<T>(s: T[], start: number, end: number = s.length): T[] | undefined {
	return start <= end && end <= s.length ? sliceView(s, start, end) : undefined;
}

/** `last_chunk`: the last `n` elements, or `undefined` if fewer. */
export function sliceLastChunk<T>(s: T[], n: number): T[] | undefined {
	return n > s.length ? undefined : slicePart(s, s.length - n);
}

/** `last_chunk_mut`: a view of the last `n` elements, or `undefined` if fewer. */
export function sliceLastChunkMut<T>(s: T[], n: number): T[] | undefined {
	return n > s.length ? undefined : sliceView(s, s.length - n, s.length);
}

/** `split_last_chunk`: the rest and the last `n` elements, or `undefined` if fewer. */
export function sliceSplitLastChunk<T>(s: T[], n: number): [T[], T[]] | undefined {
	return n > s.length ? undefined : sliceSplitAt(s, s.length - n);
}

/** `split_last_chunk_mut`: views of the rest and the last `n` elements, or `undefined` if fewer. */
export function sliceSplitLastChunkMut<T>(s: T[], n: number): [T[], T[]] | undefined {
	return n > s.length ? undefined : sliceSplitAtMut(s, s.length - n);
}

/** `as_chunks`: copies of each run of `n` elements from the front, and the rest, panicking at a size of 0. */
export function sliceAsChunks<T>(s: T[], n: number): [T[][], T[]] {
	const count = chunkCount(s.length, n);
	return [runs(count, n, (start, end) => s.slice(start, end)), slicePart(s, count * n)];
}

/** `as_rchunks`: the rest, and copies of each run of `n` elements up to the back, panicking at a size of 0. */
export function sliceAsRchunks<T>(s: T[], n: number): [T[], T[][]] {
	const count = chunkCount(s.length, n);
	const rest = s.length - count * n;
	return [slicePart(s, 0, rest), runs(count, n, (start, end) => s.slice(rest + start, rest + end))];
}

/** `as_chunks_mut`: each run of `n` elements from the front as an array, written where the slice holds it, and the rest, panicking at a size of 0. */
export function sliceAsChunksMut<T>(s: T[], n: number): [T[][], T[]] {
	const count = chunkCount(s.length, n);
	return [runsOf(s, 0, n, count), sliceView(s, count * n, s.length)];
}

/** `as_rchunks_mut`: the rest, and each run of `n` elements up to the back as an array, written where the slice holds it, panicking at a size of 0. */
export function sliceAsRchunksMut<T>(s: T[], n: number): [T[], T[][]] {
	const count = chunkCount(s.length, n);
	const rest = s.length - count * n;
	return [sliceView(s, 0, rest), runsOf(s, rest, n, count)];
}

/** `count` runs of `n` elements of `s` from `at`, as `Runs` lends them. */
function runsOf<T>(s: T[], at: number, n: number, count: number): T[][] {
	return new Proxy(new Array<T[]>(count), new Runs(s, at, n, [], 0, count));
}

/** How many runs of `n` elements fit in `length`, panicking at a size of 0, as `as_chunks` does. */
function chunkCount(length: number, n: number): number {
	if (n === 0) panic("chunk size must be non-zero");
	return Math.floor(length / n);
}

/** `count` runs of `n` elements, each made from its bounds. */
function runs<R>(count: number, n: number, run: (start: number, end: number) => R): R[] {
	const out = new Array<R>(count);
	for (let i = 0; i < count; i++) out[i] = run(i * n, i * n + n);
	return out;
}

/** `split_first`: the first element and the rest, or `undefined` if empty. */
export function sliceSplitFirst<T>(s: T[]): [T, T[]] | undefined {
	return s.length === 0 ? undefined : [s[0], s.slice(1)];
}

/** `split_at`, panicking past the length. */
export function sliceSplitAt<T>(s: T[], n: number): [T[], T[]] {
	if (n > s.length) panic("mid > len");
	return [slicePart(s, 0, n), slicePart(s, n)];
}

/** `split_at_checked`: the halves, or `undefined` past the length. */
export function sliceSplitAtChecked<T>(s: T[], n: number): [T[], T[]] | undefined {
	return n > s.length ? undefined : [slicePart(s, 0, n), slicePart(s, n)];
}

/** `split_at_mut`: views of either half, panicking past the length. */
export function sliceSplitAtMut<T>(s: T[], n: number): [T[], T[]] {
	if (n > s.length) panic("mid > len");
	return [sliceView(s, 0, n), sliceView(s, n, s.length)];
}

/** `split_at_mut_checked`: views of either half, or `undefined` past the length. */
export function sliceSplitAtMutChecked<T>(s: T[], n: number): [T[], T[]] | undefined {
	return n > s.length ? undefined : [sliceView(s, 0, n), sliceView(s, n, s.length)];
}

/**
 * `split_first_mut`: the first element, as its place if it is one JS copies, and a view
 * of the rest, or `undefined` if empty.
 */
export function sliceSplitFirstMut<T>(s: T[], place: true): [Place<T>, T[]] | undefined;
export function sliceSplitFirstMut<T>(s: T[], place: false): [T, T[]] | undefined;
export function sliceSplitFirstMut<T>(s: T[], place: boolean): [Place<T> | T, T[]] | undefined {
	if (s.length === 0) return undefined;
	return [place ? new AtIndex(s, 0) : s[0], sliceView(s, 1, s.length)];
}

/** `split_last_mut`: as `sliceSplitFirstMut`, the last element and a view of the rest. */
export function sliceSplitLastMut<T>(s: T[], place: true): [Place<T>, T[]] | undefined;
export function sliceSplitLastMut<T>(s: T[], place: false): [T, T[]] | undefined;
export function sliceSplitLastMut<T>(s: T[], place: boolean): [Place<T> | T, T[]] | undefined {
	const n = s.length - 1;
	if (n < 0) return undefined;
	return [place ? new AtIndex(s, n) : s[n], sliceView(s, 0, n)];
}

/** `first`, and `first_mut` of an element that is an object: the first element, or `none` if empty. */
export function sliceFirst<T>(s: ArrayLike<T>): T | undefined;
export function sliceFirst<T>(s: ArrayLike<T>, none: None): T | None;
export function sliceFirst<T>(s: ArrayLike<T>, none?: None): T | undefined | None {
	return s.length === 0 ? none : s[0];
}

/** `get`, and `get_mut` of an element that is an object: the element at `index`, or `none` past the end. */
export function sliceGet<T>(s: ArrayLike<T>, index: number): T | undefined;
export function sliceGet<T>(s: ArrayLike<T>, index: number, none: None): T | None;
export function sliceGet<T>(s: ArrayLike<T>, index: number, none?: None): T | undefined | None {
	return index < s.length ? s[index] : none;
}

/** `core::ops::Bound`, tagged as the crate's own enums are. */
export type RangeBound<T> = { readonly tag: "Included"; readonly _0: T } | { readonly tag: "Excluded"; readonly _0: T } | { readonly tag: "Unbounded" };

/** `Bound::cloned` and `copied`: `b` with `clone` of what it holds. */
export function boundMap<T>(b: RangeBound<T>, clone: (value: T) => T): RangeBound<T> {
	return b.tag === "Unbounded" ? b : { tag: b.tag, _0: clone(b._0) };
}

/** `NonZero::new`: `n`, unless it is 0. */
export function nonZero<T>(n: T): T | undefined {
	return n === 0 ? undefined : n;
}

/** `last`, and `last_mut` of an element that is an object: the last element, or `none` if empty. */
export function sliceLast<T>(s: ArrayLike<T>): T | undefined;
export function sliceLast<T>(s: ArrayLike<T>, none: None): T | None;
export function sliceLast<T>(s: ArrayLike<T>, none?: None): T | undefined | None {
	return s.length === 0 ? none : s[s.length - 1];
}

/** `get_mut` of an element JS copies: its place, or `undefined` past the end. */
export function sliceGetMut<T>(s: { [i: number]: T; length: number }, index: number): Place<T> | undefined {
	return index < s.length ? new AtIndex(s, index) : undefined;
}

/** `first_mut` of an element JS copies. */
export function sliceFirstMut<T>(s: { [i: number]: T; length: number }): Place<T> | undefined {
	return sliceGetMut(s, 0);
}

/** `last_mut` of an element JS copies. */
export function sliceLastMut<T>(s: { [i: number]: T; length: number }): Place<T> | undefined {
	return s.length === 0 ? undefined : new AtIndex(s, s.length - 1);
}

/** `contains`, by `eq` of each element and then `x`, as std's `==` reads them. */
export function sliceContains<T>(s: ArrayLike<T>, x: T, eq: (a: T, b: T) => boolean): boolean {
	for (let i = 0; i < s.length; i++) if (eq(s[i], x)) return true;
	return false;
}

/**
 * `binary_search_by`: `Ok` of where an element `f` finds equal is, or else `Err` of where
 * one would go, calling `f` on the elements std's search probes, in its order, so a run of
 * equal elements gives std's index.
 */
export function sliceBinarySearchBy<T>(s: ArrayLike<T>, f: (item: T) => Ordering): Result<usize, usize> {
	return searchBy(s, 0, s.length, f);
}

/** `sliceBinarySearchBy` of the `size` elements of `s` from `start`, whose indices are from it. */
export function searchBy<T>(s: ArrayLike<T>, start: number, size: number, f: (item: T) => Ordering): Result<usize, usize> {
	if (size === 0) return Err(0);
	let base = 0;
	while (size > 1) {
		const half = Math.floor(size / 2);
		const mid = base + half;
		if (f(s[start + mid]!) !== 1) base = mid;
		size -= half;
	}
	const order = f(s[start + base]!);
	return order === 0 ? Ok(base) : Err(order === -1 ? base + 1 : base);
}

/** `binary_search`: by `cmp` of each element it probes with `x`. */
export function sliceBinarySearch<T>(s: ArrayLike<T>, x: T, cmp: (a: T, b: T) => Ordering): Result<usize, usize> {
	return sliceBinarySearchBy(s, (item) => cmp(item, x));
}

/** `binary_search_by_key`: by `cmp` of the key `f` makes of each element it probes with `b`, dropping the key after. */
export function sliceBinarySearchByKey<T, K>(
	s: ArrayLike<T>,
	b: K,
	f: (item: T) => K,
	cmp: (a: K, b: K) => Ordering,
	drop?: (key: K) => void,
): Result<usize, usize> {
	return sliceBinarySearchBy(s, (item) => {
		const key = f(item);
		const order = cmp(key, b);
		drop?.(key);
		return order;
	});
}

/** `partition_point`: where the elements `pred` holds for end, by std's search. */
export function slicePartitionPoint<T>(s: ArrayLike<T>, pred: (item: T) => boolean): usize {
	const found = sliceBinarySearchBy(s, (item) => (pred(item) ? -1 : 1));
	return found.ok ? found.value : found.error;
}

/** `swap`, panicking on either index past the end, `a` first, as std's indexing does. */
export function sliceSwap<T>(s: T[], a: number, b: number): void {
	const x = held(s, s[idx(s, a)]);
	s[a] = s[idx(s, b)];
	s[b] = x;
}

/**
 * `fill`: `value` cloned over each slot but the last by `cloneOver`, then `value` itself
 * into the last, or `value` dropped if there is none.
 */
export function sliceFill<T>(s: T[], value: T, clone: ((value: T) => T) | undefined, drop?: (item: T) => void, over?: CloneFrom<T>): void {
	const last = s.length - 1;
	if (last < 0) return drop?.(value);
	for (let i = 0; i < last; i++) s[i] = cloneOver(s[i], value, clone, drop, over);
	drop?.(s[last]);
	s[last] = value;
}

/** A `clone_from` as a function of the value it writes over and its source, returning what takes the old value's place. */
export type CloneFrom<T> = (old: T, src: T) => T;

/**
 * What `clone_from` of `src` leaves over `old`: `over`'s, where it is its own, or else
 * `src`'s clone, made before `old` drops. `clone` is `undefined` where a clone is the value.
 */
export function cloneOver<T>(old: T, src: T, clone: ((value: T) => T) | undefined, drop?: (item: T) => void, over?: CloneFrom<T>): T {
	if (over !== undefined) return over(old, src);
	const item = clone === undefined ? src : clone(src);
	drop?.(old);
	return item;
}

/** A crate's own `clone_from` of an object, `f`, which writes over it in place. */
export function cloneFromFn<T>(f: (old: T, src: T) => void): CloneFrom<T> {
	return (old, src) => {
		f(old, src);
		return old;
	};
}

/** A crate's own `clone_from` of a value a `&mut` lends as an `rt.Place`, `f`. */
export function cloneFromPlaceFn<T>(f: (old: Place<T>, src: T) => void): CloneFrom<T> {
	return (old, src) => {
		f(place(() => old, (value) => (old = value)), src);
		return old;
	};
}

/** `fill_with`: `f()` into each slot in order, each old element dropped once its slot's value is made. */
export function sliceFillWith<T>(s: T[], f: () => T, drop?: (item: T) => void): void {
	for (let i = 0; i < s.length; i++) {
		const item = f();
		drop?.(s[i]);
		s[i] = item;
	}
}

/** `rotate_left`, panicking past the length. */
export function sliceRotateLeft<T>(s: T[], mid: number): void {
	if (mid > s.length) panic("assertion failed: mid <= self.len()");
	rotate(s, mid);
}

/** `rotate_right`, panicking past the length. */
export function sliceRotateRight<T>(s: T[], k: number): void {
	if (k > s.length) panic("assertion failed: k <= self.len()");
	rotate(s, s.length - k);
}

/** The elements from `mid` on, then those before it, in place. */
function rotate<T>(s: T[], mid: number): void {
	if (mid === 0 || mid === s.length) return;
	byValue(s, (s) => {
		const head = s.slice(0, mid);
		s.copyWithin(0, mid);
		for (let i = 0; i < mid; i++) s[s.length - mid + i] = head[i];
	});
}

/** `copy_within`: `s[start..end]` copied to index `dest`, panicking out of bounds; `copy` is `undefined` where a copy is the value. */
export function sliceCopyWithin<T>(s: T[], start: number, end: number, dest: number, copy?: (value: T) => T): void {
	copyWithinCheck(s.length, start, end, dest);
	if (copy === undefined) {
		s.copyWithin(dest, start, end);
		return;
	}
	// Each copied before any write, which may land in what one views.
	const items = s.slice(start, end).map(copy);
	for (let i = 0; i < items.length; i++) s[dest + i] = items[i];
}

/** `copy_within`'s checks of `start..end` and `dest` in `length` elements. */
function copyWithinCheck(length: number, start: number, end: number, dest: number): void {
	// `slice::range` checks the end first, then `range` reports the rest.
	if (end > length) panic(`range end index ${end} out of range for slice of length ${length}`);
	rangeCheck(length, start, end);
	if (dest > length - (end - start)) panic("dest is out of bounds");
}

/** `copy_from_slice`, and `clone_from_slice` of a `TrivialClone`, panicking if the lengths differ; `copy` as `sliceCopyWithin`'s. */
export function sliceCopyFromSlice<T>(dst: T[], src: ArrayLike<T>, copy?: (value: T) => T): void {
	if (dst.length !== src.length) copyLengthFail(dst.length, src.length);
	for (let i = 0; i < src.length; i++) dst[i] = copy === undefined ? src[i] : copy(src[i]);
}

/** `clone_from_slice`: each element cloned over its slot's in order by `cloneOver`, panicking first if the lengths differ. */
export function sliceCloneFromSlice<T>(dst: T[], src: ArrayLike<T>, clone: ((value: T) => T) | undefined, drop?: (item: T) => void, over?: CloneFrom<T>): T[] {
	if (dst.length !== src.length) panic("destination and source slices have different lengths");
	for (let i = 0; i < src.length; i++) dst[i] = cloneOver(dst[i], src[i], clone, drop, over);
	return dst;
}

/** `sliceCloneFromSlice` as a `CloneFrom` of an array. */
export function sliceCloneFromSliceFn<T>(clone: ((value: T) => T) | undefined, drop?: (item: T) => void, over?: CloneFrom<T>): CloneFrom<T[]> {
	return (old, src) => sliceCloneFromSlice(old, src, clone, drop, over);
}

/** `Clone::clone_from` of a `Box<[T]>`, `dst`: itself, by `clone_from_slice`, where the lengths match, or else a clone of `src`, made before `dst`'s elements drop in order. */
export function boxSliceCloneFrom<T>(dst: T[], src: T[], clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): T[] {
	if (dst.length === src.length) return sliceCloneFromSlice(dst, src, clone, drop, over);
	const v = clone === undefined ? src.slice() : src.map(clone);
	if (drop !== undefined) for (const item of dst) drop(item);
	return v;
}

/** `boxSliceCloneFrom` as a `CloneFrom`. */
export function boxSliceCloneFromFn<T>(clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): CloneFrom<T[]> {
	return (old, src) => boxSliceCloneFrom(old, src, clone, drop, over);
}

/**
 * `Option::clone_from` of `src` over `old`, whose `None` is `none`: of two `Some`s, `over`
 * of what they hold, and else `src`'s clone, made before `old` drops.
 */
export function optionCloneFrom<T>(old: T | undefined, src: T | undefined, clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>): T | undefined;
export function optionCloneFrom<T>(old: T | None, src: T | None, clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>, none: None): T | None;
export function optionCloneFrom<T>(old: T, src: T, clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>, none?: None): T {
	return optionOver(old, src, clone, drop, over, none);
}

/** `optionCloneFrom` as a `CloneFrom`. */
export function optionCloneFromFn<T>(clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>): CloneFrom<T | undefined>;
export function optionCloneFromFn<T>(clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>, none: None): CloneFrom<T | None>;
export function optionCloneFromFn<T>(clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>, none?: None): CloneFrom<T> {
	return (old, src) => optionOver(old, src, clone, drop, over, none);
}

/** `optionCloneFrom`, of any `none`. */
function optionOver<T>(old: T, src: T, clone: ((value: T) => T) | undefined, drop: ((item: T) => void) | undefined, over: CloneFrom<T>, none: unknown): T {
	if (old !== none && src !== none) return over(old, src);
	const item = src === none || clone === undefined ? src : clone(src);
	if (old !== none) drop?.(old);
	return item;
}

/**
 * `Result::clone_from` of `src` over `old`: of two `Ok`s or two `Err`s, what `src` holds
 * cloned over what `old` does, as `cloneOver` clones it by each side's own functions, and
 * else `src`'s clone, made before `old` drops.
 */
export function resultCloneFrom<T, E>(
	old: Result<T, E>,
	src: Result<T, E>,
	cloneOk?: (value: T) => T,
	dropOk?: (item: T) => void,
	overOk?: CloneFrom<T>,
	cloneErr?: (value: E) => E,
	dropErr?: (item: E) => void,
	overErr?: CloneFrom<E>,
): Result<T, E> {
	if (old.ok && src.ok) return Ok(cloneOver(old.value, src.value, cloneOk, dropOk, overOk));
	if (!old.ok && !src.ok) return Err(cloneOver(old.error, src.error, cloneErr, dropErr, overErr));
	const item: Result<T, E> = src.ok ? Ok(cloneOk === undefined ? src.value : cloneOk(src.value)) : Err(cloneErr === undefined ? src.error : cloneErr(src.error));
	if (old.ok) dropOk?.(old.value);
	else dropErr?.(old.error);
	return item;
}

/** `resultCloneFrom` as a `CloneFrom`. */
export function resultCloneFromFn<T, E>(
	cloneOk?: (value: T) => T,
	dropOk?: (item: T) => void,
	overOk?: CloneFrom<T>,
	cloneErr?: (value: E) => E,
	dropErr?: (item: E) => void,
	overErr?: CloneFrom<E>,
): CloneFrom<Result<T, E>> {
	return (old, src) => resultCloneFrom(old, src, cloneOk, dropOk, overOk, cloneErr, dropErr, overErr);
}

/** `Clone::clone_from` of a `Box<[u8]>`, `dst`: itself, `src` copied in, where the lengths match, or else a copy of `src`. */
export function byteBoxSliceCloneFrom(dst: ByteSlice, src: ByteSlice): ByteSlice {
	if (dst.length !== src.length) return byteSliceFrom(byteSliceToArray(src));
	byteSliceCopyFromSlice(dst, src);
	return dst;
}

function copyLengthFail(dst: number, src: number): never {
	panic(`copy_from_slice: source slice length (${src}) does not match destination slice length (${dst})`);
}

/**
 * `is_sorted`, by `le`, the elements' `PartialOrd::le`: as std's, which below 33 elements
 * stops at the first pair out of order, and past that tests every pair of each chunk of 33
 * before it stops, the last element of one the first of the next.
 */
export function sliceIsSorted<T>(s: { [i: number]: T; length: number }, le: (a: T, b: T) => boolean): boolean {
	const CHUNK = 33;
	const n = s.length;
	let i = 0;
	if (n >= CHUNK) {
		for (; i < n - CHUNK; i += CHUNK - 1) {
			let sorted = true;
			for (let j = i + 1; j < i + CHUNK; j++) sorted = le(s[j - 1], s[j]) && sorted;
			if (!sorted) return false;
		}
	}
	for (i++; i < n; i++) if (!le(s[i - 1], s[i])) return false;
	return true;
}

/**
 * Keeps each of the first `n` elements of `a` that `keep` of its index returns true for,
 * in order, and drops each other one once `keep` rejects it: how many it kept.
 */
function retainAt<T>(a: { [i: number]: T }, n: number, keep: (i: number) => boolean, drop?: (item: T) => void): number {
	let write = 0;
	for (let read = 0; read < n; read++) {
		if (keep(read)) a[write++] = a[read];
		else drop?.(a[read]);
	}
	return write;
}

/**
 * Drops each of the first `n` elements of `a` that `same` of its index and the previous
 * kept one's is true for, in order: how many it kept.
 */
function dedupAt<T>(a: { [i: number]: T }, n: number, same: (i: number, kept: number) => boolean, drop?: (item: T) => void): number {
	if (n === 0) return 0;
	let write = 1;
	for (let read = 1; read < n; read++) {
		if (same(read, write - 1)) drop?.(a[read]);
		else a[write++] = a[read];
	}
	return write;
}

/** `Vec::retain`, and `retain_mut` of an element that is an object. */
export function vecRetain<T>(v: T[], f: (item: T) => boolean, drop?: (item: T) => void): void {
	v.length = retainAt(v, v.length, (i) => f(v[i]), drop);
}

/** `Vec::retain_mut` of an element JS copies, which `f` takes the place of. */
export function vecRetainMut<T>(v: T[], f: (item: Place<T>) => boolean, drop?: (item: T) => void): void {
	v.length = retainAt(v, v.length, (i) => f(new AtIndex(v, i)), drop);
}

/** `Vec::dedup_by`: drops each element `same(element, previous kept)` is true for, in order. */
export function vecDedupBy<T>(v: T[], same: (a: T, b: T) => boolean, drop?: (item: T) => void): void {
	v.length = dedupAt(v, v.length, (i, j) => same(v[i], v[j]), drop);
}

/** `Vec::dedup_by` of an element JS copies, whose places `same` takes. */
export function vecDedupByMut<T>(v: T[], same: (a: Place<T>, b: Place<T>) => boolean, drop?: (item: T) => void): void {
	v.length = dedupAt(v, v.length, (i, j) => same(new AtIndex(v, i), new AtIndex(v, j)), drop);
}

/** `Vec::dedup_by_key`: as `vecDedupBy`, by `eq` of `key` of each element, then of the previous kept. */
export function vecDedupByKey<T, K>(v: T[], key: (item: T) => K, eq: (a: K, b: K) => boolean, drop?: (item: T) => void): void {
	vecDedupBy(v, (a, b) => eq(key(a), key(b)), drop);
}

/** `Vec::dedup_by_key` of an element JS copies, whose places `key` takes. */
export function vecDedupByKeyMut<T, K>(v: T[], key: (item: Place<T>) => K, eq: (a: K, b: K) => boolean, drop?: (item: T) => void): void {
	vecDedupByMut(v, (a, b) => eq(key(a), key(b)), drop);
}

/** `*v = value` through a `&mut` to a `Vec<T>` other than of bytes, or to a boxed slice: `value`'s items and capacity, in place. */
export function vecAssign<T>(v: T[] & Capped, value: T[] & Capped): void {
	v.length = value.length;
	for (let i = 0; i < value.length; i++) v[i] = value[i]!;
	if (v.cap !== undefined || value.cap !== undefined) v.cap = value.cap;
}

/** `Vec::insert`: `item` at `i`, with those from `i` on moved up. */
export function vecInsert<T>(v: T[], i: number, item: T): void {
	if (i > v.length) panic(`insertion index (is ${i}) should be <= len (is ${v.length})`);
	v.splice(i, 0, item);
}

/** `Vec::pop` of an element that can be `undefined`: the last element, or `none` if empty. */
export function vecPop<T>(v: T[], none: None): T | None {
	return v.length === 0 ? none : (v.pop() as T);
}

/**
 * `Vec::pop_if`: the last element if `f` takes it, given the element, or else `none`. An
 * empty `v` drops `f` by `dropF` uncalled.
 */
export function vecPopIf<T>(v: T[], f: (item: T) => boolean, dropF?: (f: (item: T) => boolean) => void): T | undefined;
export function vecPopIf<T>(v: T[], f: (item: T) => boolean, dropF: ((f: (item: T) => boolean) => void) | undefined, none: None): T | None;
export function vecPopIf<T>(v: T[], f: (item: T) => boolean, dropF?: (f: (item: T) => boolean) => void, none?: None): T | None | undefined {
	if (v.length === 0) {
		dropF?.(f);
		return none;
	}
	return f(v[v.length - 1]) ? (v.pop() as T) : none;
}

/** `Vec::pop_if` of an element JS copies, which `f` takes the place of. */
export function vecPopIfMut<T>(v: T[], f: (item: Place<T>) => boolean, dropF?: (f: (item: Place<T>) => boolean) => void): T | undefined;
export function vecPopIfMut<T>(
	v: T[],
	f: (item: Place<T>) => boolean,
	dropF: ((f: (item: Place<T>) => boolean) => void) | undefined,
	none: None,
): T | None;
export function vecPopIfMut<T>(
	v: T[],
	f: (item: Place<T>) => boolean,
	dropF?: (f: (item: Place<T>) => boolean) => void,
	none?: None,
): T | None | undefined {
	if (v.length === 0) {
		dropF?.(f);
		return none;
	}
	return f(new AtIndex(v, v.length - 1)) ? (v.pop() as T) : none;
}

/** `Vec::remove`: the element at `i`, with those after it moved down. */
export function vecRemove<T>(v: T[], i: number): T {
	if (i >= v.length) panic(`removal index (is ${i}) should be < len (is ${v.length})`);
	return v.splice(i, 1)[0];
}

/** `Vec::truncate`, and `clear` to 0: the first `len` elements, if there are more, and then each other drops in order. */
export function vecTruncate<T>(v: T[], len: number, drop?: (item: T) => void): void {
	if (len >= v.length) return;
	const cut = v.splice(len);
	if (drop !== undefined) dropAll(cut, drop);
}

/** `Vec::swap_remove`: the element at `i`, with the last moved into its place. */
export function vecSwapRemove<T>(v: T[], i: number): T {
	if (i >= v.length) panic(`swap_remove index (is ${i}) should be < len (is ${v.length})`);
	const item = v[i];
	v[i] = v[v.length - 1];
	v.pop();
	return item;
}

/** `Vec::extend_from_slice` of `size`-byte elements: room reserved for `s`, then a clone of each of its elements pushed in order, as far as they clone. */
export function vecExtendFromSlice<T>(v: T[], s: ArrayLike<T>, size: number, clone?: (item: T) => T): void {
	vecReserve(v, s.length, size);
	for (let i = 0; i < s.length; i++) v.push(clone === undefined ? s[i] : clone(s[i]));
}

/**
 * `Vec::clone_from` of `src`, and `clone_into` of a slice, over `v` of `size`-byte
 * elements, as std's: truncated to `src`'s length, each element it keeps cloned over by
 * `cloneOver`, and then the rest of `src` cloned onto its end.
 */
export function vecCloneFrom<T>(v: T[] & Capped, src: ArrayLike<T>, size: number, clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): T[] {
	// Its buffer, kept as it is cut, where its length was all that said how big it is.
	const cap = vecCapacity(v, size);
	vecTruncate(v, src.length, drop);
	capSet(v, v.length, cap, size);
	const kept = v.length;
	for (let i = 0; i < kept; i++) v[i] = cloneOver(v[i], src[i], clone, drop, over);
	vecReserve(v, src.length - kept, size);
	for (let i = kept; i < src.length; i++) v.push(clone === undefined ? src[i] : clone(src[i]));
	return v;
}

/** `vecCloneFrom` as a `CloneFrom`. */
export function vecCloneFromFn<T>(size: number, clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): CloneFrom<T[]> {
	return (old, src) => vecCloneFrom(old, src, size, clone, drop, over);
}

/** `Vec::append` of `size`-byte elements: room reserved for `other`'s, which then move over in order, leaving it empty. */
export function vecAppend<T>(v: T[] & Capped, other: T[], size: number): void {
	vecReserve(v, other.length, size);
	for (let i = 0; i < other.length; i++) v.push(other[i]);
	other.length = 0;
}

/**
 * `Vec::resize` of `size`-byte elements to `len`: past its length, room reserved for the
 * rest, then `len - 1` clones of `value` by `clone` and `value` itself; else truncated,
 * each element cut dropping in order, and then `value`.
 */
export function vecResize<T>(v: T[] & Capped, len: number, value: T, size: number, clone: ((value: T) => T) | undefined, drop?: (item: T) => void): void {
	if (len <= v.length) {
		vecTruncate(v, len, drop);
		drop?.(value);
		return;
	}
	vecReserve(v, len - v.length, size);
	while (v.length < len - 1) v.push(clone === undefined ? value : clone(value));
	v.push(value);
}

/** `Vec::resize_with` of `size`-byte elements to `len`: past its length, room reserved for the rest, then each `f` makes in order; else truncated. */
export function vecResizeWith<T>(v: T[] & Capped, len: number, f: () => T, size: number, drop?: (item: T) => void): void {
	if (len <= v.length) return vecTruncate(v, len, drop);
	vecReserve(v, len - v.length, size);
	while (v.length < len) v.push(f());
}

/** `Vec::split_off` of `size`-byte elements: those from `at` on, moved to a new vector with room for exactly them. */
export function vecSplitOff<T>(v: T[], at: number, size: number): T[] {
	if (at > v.length) panic(`\`at\` split index (is ${at}) should be <= len (is ${v.length})`);
	return vecExact(v.splice(at), size);
}

/**
 * `Vec::into_flattened` of arrays of `n` `size`-byte elements: their elements in order,
 * with room for `n` times as many as it had room for arrays, as std's keeps its buffer.
 */
export function vecIntoFlattened<T>(v: (T[] & Capped)[] & Capped, n: number, size: number): T[] {
	if (size === 0 && v.length * n > USIZE_MAX) panic("the product of vec len and N shouldn't overflow");
	const out: T[] & Capped = v.flat() as T[];
	capSet(out, out.length, vecCapacity(v, size * n) * n, size);
	return out;
}

/** `Vec::into_flattened` of byte arrays of `n`. */
export function byteVecIntoFlattened(v: Uint8Array[] & Capped, n: number): ByteVec {
	const out = new ByteVec(vecCapacity(v, n) * n);
	for (const a of v) {
		out.data.set(a, out.length);
		out.length += n;
	}
	return out;
}

/** `Clone` of a tuple, by `clone` of each field in order, `undefined` for one whose value is itself. */
export function tupleClone<T extends readonly unknown[]>(t: T, clones: readonly (((value: never) => unknown) | undefined)[]): T {
	const out: unknown[] = [];
	for (let i = 0; i < clones.length; i++) {
		const clone = clones[i] as ((value: unknown) => unknown) | undefined;
		out.push(clone === undefined ? t[i] : clone(t[i]));
	}
	return out as unknown as T;
}

/**
 * `&[u8]` and `&mut [u8]`, and `&[u8; N]`: `length` bytes of `data` from `start`. It never changes, so a
 * split makes another, which costs an object where a `Uint8Array` view costs several
 * times as much.
 */
export class ByteSlice {
	readonly data: Uint8Array;
	readonly start: number;
	readonly length: number;

	constructor(data: Uint8Array, start: number, length: number) {
		this.data = data;
		this.start = start;
		this.length = length;
	}
}

/**
 * A `bytes::Bytes` that is one of the handles counted of `shared`, its storage. A `Bytes`
 * that is a plain `ByteSlice` is static: it holds no storage, as `Bytes::new` makes. It is
 * here, not in `rt/bytes.ts`, as that module runs before this one defines `ByteSlice`.
 */
export class CountedBytes extends ByteSlice {
	readonly shared: Shared;
	/**
	 * Whether it is the handle `bytes` made of a vector as long as its capacity, which a
	 * `truncate` to nothing gives the storage up from. None of its clones is.
	 */
	readonly promotable: boolean;

	constructor(data: Uint8Array, start: number, length: number, shared: Shared, promotable: boolean) {
		super(data, start, length);
		this.shared = shared;
		this.promotable = promotable;
	}
}

/** All of `bytes` as a slice, as `&[u8; N]` coerces to `&[u8]`. */
export function byteSliceFrom(bytes: Uint8Array): ByteSlice {
	return new ByteSlice(bytes, 0, bytes.length);
}

/** A view of the bytes, for what reads a `Uint8Array`. */
export function byteSliceView(s: ByteSlice): Uint8Array {
	return s.data.subarray(s.start, s.start + s.length);
}

/** `&s[start..end]`, with `end` defaulting to the length. */
export function byteSliceRange(s: ByteSlice, start: number, end: number = s.length): ByteSlice {
	rangeCheck(s.length, start, end);
	return new ByteSlice(s.data, s.start + start, end - start);
}

/** `get` of `start..end`: the part, or `undefined` unless it is within the slice. */
export function byteSliceGetRange(s: ByteSlice, start: number, end: number = s.length): ByteSlice | undefined {
	return start <= end && end <= s.length ? new ByteSlice(s.data, s.start + start, end - start) : undefined;
}

/** `last_chunk`: the last `n` bytes, or `undefined` if fewer. */
export function byteSliceLastChunk(s: ByteSlice, n: number): ByteSlice | undefined {
	return n > s.length ? undefined : new ByteSlice(s.data, s.start + s.length - n, n);
}

/** `split_last_chunk`: the rest and the last `n` bytes, or `undefined` if fewer. */
export function byteSliceSplitLastChunk(s: ByteSlice, n: number): [ByteSlice, ByteSlice] | undefined {
	return n > s.length ? undefined : byteSliceSplitAt(s, s.length - n);
}

/** `split_first`: the first byte and the rest, or `undefined` if empty. */
export function byteSliceSplitFirst(s: ByteSlice): [number, ByteSlice] | undefined {
	return s.length === 0 ? undefined : [s.data[s.start], new ByteSlice(s.data, s.start + 1, s.length - 1)];
}

/** `split_first_mut`: the first byte's place and the rest, or `undefined` if empty. */
export function byteSliceSplitFirstMut(s: ByteSlice): [Place<number>, ByteSlice] | undefined {
	return s.length === 0 ? undefined : [new AtIndex(s.data, s.start), new ByteSlice(s.data, s.start + 1, s.length - 1)];
}

/** `split_last_mut`: the last byte's place and the rest, or `undefined` if empty. */
export function byteSliceSplitLastMut(s: ByteSlice): [Place<number>, ByteSlice] | undefined {
	const n = s.length - 1;
	return n < 0 ? undefined : [new AtIndex(s.data, s.start + n), new ByteSlice(s.data, s.start, n)];
}

/**
 * `ptr::eq` of pointers to `[u8; N]`s: the same null or dangling pointer, or slices of the
 * same bytes, which separate views of one buffer may hold.
 */
export function byteSliceSame(a: ByteSlice, b: ByteSlice): boolean {
	if (!(a instanceof ByteSlice && b instanceof ByteSlice)) return a === b;
	return a.data.buffer === b.data.buffer && a.data.byteOffset + a.start === b.data.byteOffset + b.start;
}

/** `ptr::write` of `bytes` through a pointer to a `[u8; N]`: copied into its slice. */
export function byteSliceSet(s: ByteSlice, bytes: Uint8Array): void {
	s.data.set(bytes, s.start);
}

/** `*r` of a `&[u8; N]`: a copy of its bytes. */
export function byteSliceToArray(s: ByteSlice): Uint8Array {
	return s.data.slice(s.start, s.start + s.length);
}

/** `split_at`, panicking past the length. */
export function byteSliceSplitAt(s: ByteSlice, n: number): [ByteSlice, ByteSlice] {
	if (n > s.length) panic("mid > len");
	return [new ByteSlice(s.data, s.start, n), new ByteSlice(s.data, s.start + n, s.length - n)];
}

/** `split_at_checked`: the halves, or `undefined` past the length. */
export function byteSliceSplitAtChecked(s: ByteSlice, n: number): [ByteSlice, ByteSlice] | undefined {
	if (n > s.length) return undefined;
	return [new ByteSlice(s.data, s.start, n), new ByteSlice(s.data, s.start + n, s.length - n)];
}

/** `as_chunks` of bytes: views of each run of `n` from the front, and the rest, panicking at a size of 0. */
export function byteSliceAsChunks(s: ByteSlice, n: number): [Uint8Array[], ByteSlice] {
	const count = chunkCount(s.length, n);
	const at = s.start;
	return [runs(count, n, (start, end) => s.data.subarray(at + start, at + end)), new ByteSlice(s.data, at + count * n, s.length - count * n)];
}

/** `as_rchunks` of bytes: the rest, and views of each run of `n` up to the back, panicking at a size of 0. */
export function byteSliceAsRchunks(s: ByteSlice, n: number): [ByteSlice, Uint8Array[]] {
	const count = chunkCount(s.length, n);
	const at = s.start + s.length - count * n;
	return [new ByteSlice(s.data, s.start, s.length - count * n), runs(count, n, (start, end) => s.data.subarray(at + start, at + end))];
}

/** `as_chunks_mut` of bytes: each run of `n` from the front as an array, written where the slice holds it, and the rest, panicking at a size of 0. */
export function byteSliceAsChunksMut(s: ByteSlice, n: number): [Uint8Array[], ByteSlice] {
	const count = chunkCount(s.length, n);
	return [byteRunsOf(s.data, s.start, n, count), new ByteSlice(s.data, s.start + count * n, s.length - count * n)];
}

/** `as_rchunks_mut` of bytes: the rest, and each run of `n` up to the back as an array, written where the slice holds it, panicking at a size of 0. */
export function byteSliceAsRchunksMut(s: ByteSlice, n: number): [ByteSlice, Uint8Array[]] {
	const count = chunkCount(s.length, n);
	const rest = s.length - count * n;
	return [new ByteSlice(s.data, s.start, rest), byteRunsOf(s.data, s.start + rest, n, count)];
}

/** `count` runs of `n` bytes of `data` from `at`, as `ByteRuns` lends them. */
function byteRunsOf(data: Uint8Array, at: number, n: number, count: number): Uint8Array[] {
	return new Proxy(new Array<Uint8Array>(count), new ByteRuns(data, at, n, [], 0, count));
}

/** `first`: the first byte, or `undefined` if empty. */
export function byteSliceFirst(s: ByteSlice): number | undefined {
	return s.length === 0 ? undefined : s.data[s.start];
}

/** `last`: the last byte, or `undefined` if empty. */
export function byteSliceLast(s: ByteSlice): number | undefined {
	return s.length === 0 ? undefined : s.data[s.start + s.length - 1];
}

/** `swap`, panicking on either index past the end, `a` first, as std's indexing does. */
export function byteSliceSwap(s: ByteSlice, a: number, b: number): void {
	const x = s.data[s.start + idx(s, a)];
	s.data[s.start + a] = s.data[s.start + idx(s, b)];
	s.data[s.start + b] = x;
}

/** `get`: the byte at `index`, or `undefined` past the end. */
export function byteSliceGet(s: ByteSlice, index: number): number | undefined {
	return index < s.length ? s.data[s.start + index] : undefined;
}

/** `==` of byte slices: the same length and bytes. */
export function byteSliceEq(a: ByteSlice, b: ByteSlice): boolean {
	if (a.length !== b.length) return false;
	for (let i = 0; i < a.length; i++) if (a.data[a.start + i] !== b.data[b.start + i]) return false;
	return true;
}

/** Copy `src` into `dst` at `at`; a few bytes copy faster than a view of them is made. */
export function copyBytes(dst: Uint8Array, at: number, src: Uint8Array, start: number, length: number): void {
	if (length <= 16) {
		for (let i = 0; i < length; i++) dst[at + i] = src[start + i];
	} else {
		dst.set(src.subarray(start, start + length), at);
	}
}

/** `copy_from_slice`, panicking if the lengths differ. */
export function byteSliceCopyFromSlice(dst: ByteSlice, src: ByteSlice): void {
	if (dst.length !== src.length) copyLengthFail(dst.length, src.length);
	copyBytes(dst.data, dst.start, src.data, src.start, src.length);
}

/** `copy_within`: `s[start..end]` copied to index `dest`, panicking out of bounds. */
export function byteSliceCopyWithin(s: ByteSlice, start: number, end: number, dest: number): void {
	copyWithinCheck(s.length, start, end, dest);
	s.data.copyWithin(s.start + dest, s.start + start, s.start + end);
}

/** `fill` of bytes. */
export function byteSliceFill(s: ByteSlice, value: number): void {
	s.data.fill(value, s.start, s.start + s.length);
}

/** `fill_with` of bytes: `f()` into each in order. */
export function byteSliceFillWith(s: ByteSlice, f: () => number): void {
	for (let i = 0; i < s.length; i++) s.data[s.start + i] = f();
}

/** `rotate_left` of bytes, panicking past the length. */
export function byteSliceRotateLeft(s: ByteSlice, mid: number): void {
	if (mid > s.length) panic("assertion failed: mid <= self.len()");
	byteRotate(s, mid);
}

/** `rotate_right` of bytes, panicking past the length. */
export function byteSliceRotateRight(s: ByteSlice, k: number): void {
	if (k > s.length) panic("assertion failed: k <= self.len()");
	byteRotate(s, s.length - k);
}

/** The bytes from `mid` on, then those before it, in place. */
function byteRotate(s: ByteSlice, mid: number): void {
	const head = s.data.slice(s.start, s.start + mid);
	s.data.copyWithin(s.start, s.start + mid, s.start + s.length);
	s.data.set(head, s.start + s.length - mid);
}

/** `to_vec` of bytes. */
export function byteSliceToByteVec(s: ByteSlice): ByteVec {
	const v = new ByteVec(s.length);
	copyBytes(v.data, 0, s.data, s.start, s.length);
	v.length = s.length;
	return v;
}

/** `vec![value; n]`: `n - 1` clones of `value`, in order, then `value` itself, with room for exactly them, or `value` dropped where `n` is 0; `clone` is `undefined` where a clone is the value. */
export function vecFromElem<T>(value: T, n: number, clone: ((value: T) => T) | undefined, drop?: (item: T) => void): T[] {
	const v: T[] & Capped = [];
	if (n === 0) {
		drop?.(value);
		return v;
	}
	for (let i = 1; i < n; i++) v.push(clone === undefined ? value : clone(value));
	v.push(value);
	v.cap = n;
	return v;
}

const SLAB = 8192;
let slab = /* @__PURE__ */ new ArrayBuffer(SLAB);
let slabUsed = 0;

/**
 * `n` zeroed bytes. Up to 1 KiB, they share an `ArrayBuffer`, as Node's `Buffer` pool
 * does: a lone array's own buffer costs V8 about a microsecond, made when the array is
 * past 64 bytes or else the first time something views it. No byte is handed out twice.
 */
export function bytes(n: number): Uint8Array {
	if (n > SLAB / 8) return new Uint8Array(n);
	if (slabUsed + n > SLAB) {
		slab = new ArrayBuffer(SLAB);
		slabUsed = 0;
	}
	const out = new Uint8Array(slab, slabUsed, n);
	// Aligned, as Node aligns its pool.
	slabUsed += (n + 7) & ~7;
	return out;
}

/** `array::from_fn`: `f` of each index, called in order. */
export function arrayFromFn<T>(n: number, f: (i: number) => T): T[] {
	const out: T[] = [];
	for (let i = 0; i < n; i++) out.push(f(i));
	return out;
}

/** `array::from_fn` of bytes. */
export function bytesFromFn(n: number, f: (i: number) => number): Uint8Array {
	const out = bytes(n);
	for (let i = 0; i < n; i++) out[i] = f(i);
	return out;
}

/** `<[T; N]>::map`: `f` of each element, called in order. */
export function arrayMap<T, U>(a: ArrayLike<T>, f: (x: T) => U): U[] {
	const out: U[] = [];
	for (let i = 0; i < a.length; i++) out.push(f(a[i]));
	return out;
}

/** `<[T; N]>::map` to bytes. */
export function bytesMap<T>(a: ArrayLike<T>, f: (x: T) => number): Uint8Array {
	const out = bytes(a.length);
	for (let i = 0; i < a.length; i++) out[i] = f(a[i]);
	return out;
}

/** `<[T; N]>::each_ref` and `each_mut` of elements lent as themselves. */
export function arrayFrom<T>(a: ArrayLike<T>): T[] {
	return Array.from(a);
}

/** `<[T; N]>::each_mut` of elements JS copies: the place of each. */
export function arrayPlaces<T>(a: { [i: number]: T; length: number }): Place<T>[] {
	const out: Place<T>[] = [];
	for (let i = 0; i < a.length; i++) out.push(new AtIndex(a, i));
	return out;
}

/** `Vec<u8>`: a growable byte buffer. */
export class ByteVec {
	/** The storage; only the first `length` bytes are the vector. */
	data: Uint8Array;
	length = 0;

	constructor(capacity = 0) {
		this.data = bytes(capacity);
	}
}

/** A `Vec<u8>` of a copy of `bytes`. */
export function byteVecFrom(bytes: Uint8Array): ByteVec {
	const v = new ByteVec(bytes.length);
	v.data.set(bytes);
	v.length = bytes.length;
	return v;
}

/** `vec![value; n]` of bytes. */
export function byteVecOf(value: number, n: number): ByteVec {
	const v = new ByteVec(n);
	v.data.fill(value);
	v.length = n;
	return v;
}

/** Make room for `additional` more bytes. */
function reserve(v: ByteVec, additional: number): void {
	const needed = v.length + additional;
	if (needed <= v.data.length) return;
	const grown = bytes(Math.max(needed, v.data.length * 2, 8));
	// All of `data`, which costs less than making a view of its first `length` bytes.
	grown.set(v.data);
	v.data = grown;
}

/** The bytes, as a view that a later push may detach. */
export function byteVecView(v: ByteVec): Uint8Array {
	return v.data.subarray(0, v.length);
}

/** The bytes, as `Vec`'s `Deref` borrows them, which a later push may detach. */
export function byteVecSlice(v: ByteVec): ByteSlice {
	return new ByteSlice(v.data, 0, v.length);
}

/** `Vec::push`. */
export function byteVecPush(v: ByteVec, byte: number): void {
	reserve(v, 1);
	v.data[v.length++] = byte;
}

/** `Vec::pop`: the last byte, or `undefined` if empty. */
export function byteVecPop(v: ByteVec): number | undefined {
	return v.length === 0 ? undefined : v.data[--v.length];
}

/** `Vec::pop_if`, which `f` takes the last byte's place in. An empty `v` drops `f` by `dropF` uncalled. */
export function byteVecPopIf(
	v: ByteVec,
	f: (byte: Place<number>) => boolean,
	dropF?: (f: (byte: Place<number>) => boolean) => void,
): number | undefined {
	if (v.length === 0) {
		dropF?.(f);
		return undefined;
	}
	return f(new AtIndex(v.data, v.length - 1)) ? v.data[--v.length] : undefined;
}

/** `Vec::extend_from_slice`. */
export function byteVecExtendFromSlice(v: ByteVec, s: ByteSlice): void {
	reserve(v, s.length);
	copyBytes(v.data, v.length, s.data, s.start, s.length);
	v.length += s.length;
}

/** `Vec::truncate`. */
export function byteVecTruncate(v: ByteVec, length: number): void {
	if (length < v.length) v.length = length;
}

/** `Vec::spare_capacity_mut`: the room past the bytes, a slice of the storage that a write through reaches. */
export function byteVecSpare(v: ByteVec): ByteSlice {
	return new ByteSlice(v.data, v.length, v.data.length - v.length);
}

/** `Vec::set_len`, which aborts past the capacity, as std's check of its precondition does. */
export function byteVecSetLen(v: ByteVec, length: number): void {
	if (length > v.data.length) {
		throw new Abort(
			"unsafe precondition(s) violated: Vec::set_len requires that new_len <= capacity()\n\nThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.",
		);
	}
	v.length = length;
}

/** `Vec::clear`. */
export function byteVecClear(v: ByteVec): void {
	v.length = 0;
}

/** `Vec::insert`: `byte` at `i`, with those from `i` on moved up. */
export function byteVecInsert(v: ByteVec, i: number, byte: number): void {
	if (i > v.length) panic(`insertion index (is ${i}) should be <= len (is ${v.length})`);
	reserve(v, 1);
	v.data.copyWithin(i + 1, i, v.length);
	v.data[i] = byte;
	v.length++;
}

/** `Vec::remove`: the byte at `i`, with those after it moved down. */
export function byteVecRemove(v: ByteVec, i: number): number {
	if (i >= v.length) panic(`removal index (is ${i}) should be < len (is ${v.length})`);
	const byte = v.data[i];
	v.data.copyWithin(i, i + 1, v.length);
	v.length--;
	return byte;
}

/** `Vec::swap_remove`: the byte at `i`, with the last moved into its place. */
export function byteVecSwapRemove(v: ByteVec, i: number): number {
	if (i >= v.length) panic(`swap_remove index (is ${i}) should be < len (is ${v.length})`);
	const byte = v.data[i];
	v.data[i] = v.data[--v.length];
	return byte;
}

/** `Vec::retain`. */
export function byteVecRetain(v: ByteVec, f: (byte: number) => boolean): void {
	v.length = retainAt(v.data, v.length, (i) => f(v.data[i]));
}

/** `Vec::retain_mut`, which `f` takes each byte's place in. */
export function byteVecRetainMut(v: ByteVec, f: (byte: Place<number>) => boolean): void {
	v.length = retainAt(v.data, v.length, (i) => f(new AtIndex(v.data, i)));
}

/** `Vec::dedup`. */
export function byteVecDedup(v: ByteVec): void {
	v.length = dedupAt(v.data, v.length, (i, j) => v.data[i] === v.data[j]);
}

/** `Vec::dedup_by`, whose places `same` takes. */
export function byteVecDedupByMut(v: ByteVec, same: (a: Place<number>, b: Place<number>) => boolean): void {
	v.length = dedupAt(v.data, v.length, (i, j) => same(new AtIndex(v.data, i), new AtIndex(v.data, j)));
}

/** `Vec::dedup_by_key`, whose places `key` takes. */
export function byteVecDedupByKeyMut<K>(v: ByteVec, key: (byte: Place<number>) => K, eq: (a: K, b: K) => boolean): void {
	byteVecDedupByMut(v, (a, b) => eq(key(a), key(b)));
}

// Capacity. std leaves a vector's growth unspecified past `capacity() >= len` and what
// `with_capacity` and `reserve` ask for, so a `Vec` or `VecDeque` holds one, as `cap`,
// only where an operation sets it, and a push never reads or writes it. Reading one
// doubles it while short of the length, from the least `RawVec` allocates, as std's
// pushes grow it. An `rt.ByteVec`'s is its buffer's length.

/** wasm32's `isize::MAX`, the most bytes std's `Layout` describes. */
const ISIZE_MAX = 0x7fffffff;

/** wasm32's `usize::MAX`. */
const USIZE_MAX = 0xffffffff;

/** A vector's storage, and `cap`, the capacity an operation last set it to, if any. */
export interface Capped {
	cap?: number;
}

/** How many `size`-byte elements a vector holds at most: `usize::MAX` of zero-sized ones. */
export function vecMax(size: number): number {
	return size === 0 ? USIZE_MAX : Math.floor(ISIZE_MAX / size);
}

/** The capacity of a vector holding `len` of `size`-byte elements, from `cap`, the one last set, as pushes grow std's. */
export function capOf(len: number, cap: number | undefined, size: number): number {
	if (size === 0) return USIZE_MAX;
	let c = cap ?? 0;
	while (c < len) c = Math.max(c * 2, size === 1 ? 8 : size <= 1024 ? 4 : 1);
	return c;
}

/** `c`'s capacity set to `cap`, holding `len`, where reading it would not give that. */
export function capSet(c: object & Capped, len: number, cap: number, size: number): void {
	if (size !== 0 && (c.cap !== undefined || cap !== capOf(len, undefined, size))) c.cap = cap;
}

/** `c`'s capacity set to `cap`, which `reserve` asked for, so it holds as the length drops. */
export function capHold(c: object & Capped, cap: number, size: number): void {
	if (size !== 0) c.cap = cap;
}

/**
 * The capacity `cap`, holding `len`, once `try_reserve` made room for `additional` more, as
 * std's `RawVec::grow_amortized`: at least double, or `undefined` past what it may hold.
 */
export function capTryReserve(len: number, cap: number, additional: number, size: number): number | undefined {
	if (additional <= cap - len) return cap;
	const grown = Math.max(cap * 2, len + additional, size === 1 ? 8 : size <= 1024 ? 4 : 1);
	return grown > vecMax(size) ? undefined : grown;
}

/** The capacity `cap`, holding `len`, once `try_reserve_exact` made room for `additional` more, or `undefined` past what it may hold. */
export function capTryReserveExact(len: number, cap: number, additional: number, size: number): number | undefined {
	if (additional <= cap - len) return cap;
	return len + additional > vecMax(size) ? undefined : len + additional;
}

/** `capTryReserve`'s capacity, or the panic of `reserve` past what it may hold. */
export function capReserve(len: number, cap: number, additional: number, size: number): number {
	return capTryReserve(len, cap, additional, size) ?? panic("capacity overflow");
}

/** `capTryReserveExact`'s capacity, or the panic of `reserve_exact` past what it may hold. */
export function capReserveExact(len: number, cap: number, additional: number, size: number): number {
	return capTryReserveExact(len, cap, additional, size) ?? panic("capacity overflow");
}

/** The capacity `cap`, holding `len`, once `shrink_to` of `min` shrank it, as `shrink_to_fit` of 0 does. */
export function capShrink(len: number, cap: number, min: number): number {
	return cap > min ? Math.max(len, min) : cap;
}

/** `Vec::with_capacity` of `size`-byte elements, which panics past what it may hold. */
export function vecWithCapacity<T>(n: number, size: number): T[] {
	if (n > vecMax(size)) panic("capacity overflow");
	const v: T[] & Capped = [];
	capSet(v, 0, n, size);
	return v;
}

/** `Vec::capacity` of `size`-byte elements. */
export function vecCapacity<T>(v: T[] & Capped, size: number): number {
	return capOf(v.length, v.cap, size);
}

/** `Vec::reserve` of `size`-byte elements. */
export function vecReserve<T>(v: T[] & Capped, additional: number, size: number): void {
	capHold(v, capReserve(v.length, vecCapacity(v, size), additional, size), size);
}

/** `Vec::reserve_exact` of `size`-byte elements. */
export function vecReserveExact<T>(v: T[] & Capped, additional: number, size: number): void {
	capHold(v, capReserveExact(v.length, vecCapacity(v, size), additional, size), size);
}

/** `Vec::try_reserve` of `size`-byte elements: `reserve`'s capacity, or the error where it panics. */
export function vecTryReserve<T>(v: T[] & Capped, additional: number, size: number): Result<undefined, TryReserveError> {
	return capTried(v, capTryReserve(v.length, vecCapacity(v, size), additional, size), size);
}

/** `Vec::try_reserve_exact` of `size`-byte elements. */
export function vecTryReserveExact<T>(v: T[] & Capped, additional: number, size: number): Result<undefined, TryReserveError> {
	return capTried(v, capTryReserveExact(v.length, vecCapacity(v, size), additional, size), size);
}

/** `v`'s capacity held at `cap`, or the overflow `undefined` is. */
function capTried(v: object & Capped, cap: number | undefined, size: number): Result<undefined, TryReserveError> {
	if (cap === undefined) return CAPACITY_OVERFLOW;
	capHold(v, cap, size);
	return OK;
}

/** `Vec::shrink_to` of `size`-byte elements, and `shrink_to_fit` of 0. */
export function vecShrinkTo<T>(v: T[] & Capped, min: number, size: number): void {
	capSet(v, v.length, capShrink(v.length, vecCapacity(v, size), min), size);
}

/**
 * `Vec::clone_from` of `src`, and `clone_into` of a byte slice, over `v`: its buffer, grown
 * as `reserve` grows it if it holds fewer, holding a copy of `src`'s bytes.
 */
export function byteVecCloneFrom(v: ByteVec, src: ByteVec | ByteSlice): ByteVec {
	v.length = 0;
	reserve(v, src.length);
	copyBytes(v.data, 0, src.data, src instanceof ByteSlice ? src.start : 0, src.length);
	v.length = src.length;
	return v;
}

/** `byteVecCloneFrom` as a `CloneFrom`. */
export function byteVecCloneFromFn(): CloneFrom<ByteVec> {
	return byteVecCloneFrom;
}

/** `v`, a new `Vec` of `size`-byte elements, with room for exactly what it holds, as std's `clone` makes one. */
export function vecExact<T>(v: T[], size: number): T[] {
	capSet(v, v.length, v.length, size);
	return v;
}

/** `v`'s storage moved to `cap` bytes. */
function realloc(v: ByteVec, cap: number): void {
	const data = bytes(cap);
	data.set(v.data.subarray(0, v.length));
	v.data = data;
}

/**
 * The bytes of `v` as a `Vec<number>`, with its capacity, as a deque or heap takes it:
 * held, as `reserve` may have asked for it, though its length gives it now.
 */
export function byteVecToVec(v: ByteVec): number[] {
	const out = Array.from(byteVecView(v));
	capHold(out, v.data.length, 1);
	return out;
}

/** A `Vec<u8>` of the bytes in `v`, with its capacity, as a heap gives them up. */
export function byteVecFromVec(v: number[] & Capped): ByteVec {
	const out = new ByteVec(capOf(v.length, v.cap, 1));
	out.data.set(v);
	out.length = v.length;
	return out;
}

/** `Vec::with_capacity` of bytes, which panics past what it may hold. */
export function byteVecWithCapacity(n: number): ByteVec {
	if (n > ISIZE_MAX) panic("capacity overflow");
	return new ByteVec(n);
}

/** `Vec::reserve` of bytes. */
export function byteVecReserve(v: ByteVec, additional: number): void {
	const cap = capReserve(v.length, v.data.length, additional, 1);
	if (cap !== v.data.length) realloc(v, cap);
}

/** `Vec::reserve_exact` of bytes. */
export function byteVecReserveExact(v: ByteVec, additional: number): void {
	const cap = capReserveExact(v.length, v.data.length, additional, 1);
	if (cap !== v.data.length) realloc(v, cap);
}

/** `Vec::try_reserve` of bytes: `reserve`'s storage, or the error where it panics or JS can't make it. */
export function byteVecTryReserve(v: ByteVec, additional: number): Result<undefined, TryReserveError> {
	return byteVecTried(v, capTryReserve(v.length, v.data.length, additional, 1));
}

/** `Vec::try_reserve_exact` of bytes. */
export function byteVecTryReserveExact(v: ByteVec, additional: number): Result<undefined, TryReserveError> {
	return byteVecTried(v, capTryReserveExact(v.length, v.data.length, additional, 1));
}

/** `v`'s storage moved to `cap` bytes, or the overflow `undefined` is, or the failure to allocate them. */
function byteVecTried(v: ByteVec, cap: number | undefined): Result<undefined, TryReserveError> {
	if (cap === undefined) return CAPACITY_OVERFLOW;
	if (cap === v.data.length) return OK;
	try {
		realloc(v, cap);
	} catch (e) {
		return allocError(e, cap);
	}
	return OK;
}

/** `Vec::append` of bytes. */
export function byteVecAppend(v: ByteVec, other: ByteVec): void {
	byteVecReserve(v, other.length);
	copyBytes(v.data, v.length, other.data, 0, other.length);
	v.length += other.length;
	other.length = 0;
}

/** `Vec::resize` of bytes. */
export function byteVecResize(v: ByteVec, len: number, byte: number): void {
	if (len <= v.length) {
		v.length = len;
		return;
	}
	byteVecReserve(v, len - v.length);
	v.data.fill(byte, v.length, len);
	v.length = len;
}

/** `Vec::resize_with` of bytes. */
export function byteVecResizeWith(v: ByteVec, len: number, f: () => number): void {
	if (len <= v.length) {
		v.length = len;
		return;
	}
	byteVecReserve(v, len - v.length);
	while (v.length < len) v.data[v.length++] = f();
}

/** `Vec::split_off` of bytes. */
export function byteVecSplitOff(v: ByteVec, at: number): ByteVec {
	if (at > v.length) panic(`\`at\` split index (is ${at}) should be <= len (is ${v.length})`);
	const other = new ByteVec(v.length - at);
	copyBytes(other.data, 0, v.data, at, v.length - at);
	other.length = v.length - at;
	v.length = at;
	return other;
}

/** `Vec::shrink_to` of bytes, and `shrink_to_fit` of 0. */
export function byteVecShrinkTo(v: ByteVec, min: number): void {
	const cap = capShrink(v.length, v.data.length, min);
	if (cap !== v.data.length) realloc(v, cap);
}

// Cows of slices. An owned one is its `Vec` and a borrowed one its slice, told apart by
// class: a `Cow<[u8]>` is an `rt.ByteVec` or an `rt.ByteSlice`, and any other is an array
// or the `rt.Borrowed` wrapping the array it borrows.

/**
 * The slice a `Cow<[T]>` borrows, which it clones before it changes. A `&mut` to the
 * borrow writes `slice`, so no two `Cow`s share one.
 */
export class Borrowed<T> {
	slice: T[];

	constructor(slice: T[]) {
		this.slice = slice;
	}
}

/** `Cow<[T]>`: the `Vec` it owns, or the slice it borrows. */
export type Cow<T> = T[] | Borrowed<T>;
/** `Cow<[u8]>`: the `Vec<u8>` it owns, or the bytes it borrows. */
export type ByteCow = ByteVec | ByteSlice;

/** `Deref` of a `Cow<[T]>`: its slice. */
export function cowSlice<T>(c: Cow<T>): T[] {
	return c instanceof Borrowed ? c.slice : c;
}

/** `Deref` of a `Cow<[u8]>`: its bytes. */
export function byteCowSlice(c: ByteCow): ByteSlice {
	return c instanceof ByteVec ? byteVecSlice(c) : c;
}

/**
 * `Cow::into_owned` of a `Cow<[T]>`, of `size`-byte items: its `Vec`, or a clone of what it
 * borrows, each item by `clone` if given, with room for exactly them, as `to_vec` makes it.
 */
export function cowIntoOwned<T>(c: Cow<T>, size: number, clone?: (item: T) => T): T[] {
	if (!(c instanceof Borrowed)) return c;
	return vecExact(clone === undefined ? c.slice.slice() : c.slice.map(clone), size);
}

/** `Cow::into_owned` of a `Cow<[u8]>`. */
export function byteCowIntoOwned(c: ByteCow): ByteVec {
	return c instanceof ByteVec ? c : byteSliceToByteVec(c);
}

/**
 * `Clone` of a `Cow<[T]>`, of `size`-byte items: a borrow of the same slice, or a clone of
 * its `Vec` with room for exactly its items, as `to_vec` makes it.
 */
export function cowClone<T>(c: Cow<T>, size: number, clone?: (item: T) => T): Cow<T> {
	if (c instanceof Borrowed) return new Borrowed(c.slice);
	return vecExact(clone === undefined ? c.slice() : c.map(clone), size);
}

/** `Clone` of a `Cow<[u8]>`. */
export function byteCowClone(c: ByteCow): ByteCow {
	return c instanceof ByteVec ? byteSliceToByteVec(byteVecSlice(c)) : c;
}

/**
 * `clone_from` of a `Cow<[T]>`, of `size`-byte items: an owned one's `Vec`, of an owned
 * one's, as `vecCloneFrom` clones into it, and else `src`'s clone, made before `c` drops.
 */
export function cowCloneFrom<T>(c: Cow<T>, src: Cow<T>, size: number, clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): Cow<T> {
	if (!(c instanceof Borrowed || src instanceof Borrowed)) return vecCloneFrom(c, src, size, clone, drop, over);
	const item = cowClone(src, size, clone);
	cowDrop(c, drop);
	return item;
}

/** `cowCloneFrom` as a `CloneFrom`. */
export function cowCloneFromFn<T>(size: number, clone?: (value: T) => T, drop?: (item: T) => void, over?: CloneFrom<T>): CloneFrom<Cow<T>> {
	return (old, src) => cowCloneFrom(old, src, size, clone, drop, over);
}

/** `cowCloneFrom` of a `Cow<[u8]>`. */
export function byteCowCloneFrom(c: ByteCow, src: ByteCow): ByteCow {
	return c instanceof ByteVec && src instanceof ByteVec ? byteVecCloneFrom(c, src) : byteCowClone(src);
}

/** `byteCowCloneFrom` as a `CloneFrom`. */
export function byteCowCloneFromFn(): CloneFrom<ByteCow> {
	return byteCowCloneFrom;
}

/** Drop a `Cow<[T]>`: the items it owns, by `f` if given. */
export function cowDrop<T>(c: Cow<T>, f?: (item: T) => void): void {
	if (!(c instanceof Borrowed)) dropAll(c, f);
}

/** `f` of the slices two `Cow<[T]>`s deref to, as their `PartialEq` and `Ord` compare them. */
export function cowBy<T, R>(f: (a: T[], b: T[]) => R): (a: Cow<T>, b: Cow<T>) => R {
	return (a, b) => f(cowSlice(a), cowSlice(b));
}

/** `Hash::hash` of a `Cow<[T]>`: its slice's. */
export function hashCow<T, S>(hash: (value: T[], s: S) => void): (value: Cow<T>, s: S) => void {
	return (value, s) => hash(cowSlice(value), s);
}

// Strings

const encoder = /* @__PURE__ */ new TextEncoder();
// `ignoreBOM` keeps a leading U+FEFF, as Rust does.
const decoder = /* @__PURE__ */ new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });

/** `core::str::Utf8Error`: where bytes stop being UTF-8. */
export type Utf8Error = {
	/** `Utf8Error::valid_up_to`. */
	readonly validUpTo: usize;
	/** `Utf8Error::error_len`: `undefined` if the input ended mid-character. */
	readonly errorLen: usize | undefined;
};

/** `alloc::string::FromUtf8Error`: the bytes that were not UTF-8, and where they stop being. */
export class FromUtf8Error {
	readonly bytes: ByteVec;
	readonly error: Utf8Error;

	constructor(bytes: ByteVec) {
		this.bytes = bytes;
		// Made only of bytes that are not UTF-8.
		this.error = utf8Check(byteVecView(bytes)) as Utf8Error;
	}
}

/** `Clone` of a `FromUtf8Error`: its bytes, copied. */
export function fromUtf8ErrorClone(e: FromUtf8Error): FromUtf8Error {
	return new FromUtf8Error(byteVecFrom(byteVecView(e.bytes)));
}

/** Where `v` stops being UTF-8, and how many bytes are invalid there, as std reports it: `undefined` if it never does. */
export function utf8Check(v: Uint8Array): Utf8Error | undefined {
	const tail = (b: number | undefined) => b !== undefined && b >= 0x80 && b <= 0xbf;
	for (let i = 0; i < v.length; ) {
		const first = v[i];
		const width = first < 0x80 ? 1 : first >= 0xc2 && first <= 0xdf ? 2 : first >= 0xe0 && first <= 0xef ? 3 : first >= 0xf0 && first <= 0xf4 ? 4 : 0;
		if (width === 1) {
			i++;
			continue;
		}
		if (width === 0) return { validUpTo: i, errorLen: 1 };
		const second = v[i + 1];
		if (second === undefined) return { validUpTo: i, errorLen: undefined };
		const [lo, hi] =
			first === 0xe0 ? [0xa0, 0xbf] : first === 0xed ? [0x80, 0x9f] : first === 0xf0 ? [0x90, 0xbf] : first === 0xf4 ? [0x80, 0x8f] : [0x80, 0xbf];
		if (second < lo || second > hi) return { validUpTo: i, errorLen: 1 };
		for (let k = 2; k < width; k++) {
			if (i + k >= v.length) return { validUpTo: i, errorLen: undefined };
			if (!tail(v[i + k])) return { validUpTo: i, errorLen: k };
		}
		i += width;
	}
	return undefined;
}

/** `str::from_utf8`. */
export function strFromUtf8(bytes: ByteSlice): Result<string, Utf8Error> {
	const v = byteSliceView(bytes);
	try {
		return Ok(decoder.decode(v));
	} catch {
		return Err(utf8Check(v) as Utf8Error);
	}
}

/** `String::from_utf8`. */
export function stringFromUtf8(bytes: ByteVec): Result<string, FromUtf8Error> {
	try {
		return Ok(decoder.decode(byteVecView(bytes)));
	} catch {
		return Err(new FromUtf8Error(bytes));
	}
}

/** `str::as_bytes`: its UTF-8 bytes. */
export function stringAsBytes(s: string): ByteSlice {
	return byteSliceFrom(encoder.encode(s));
}

/** `String::into_bytes`: its UTF-8 bytes. */
export function stringIntoBytes(s: string): ByteVec {
	return byteVecFrom(encoder.encode(s));
}

/** `str::len`: its length in UTF-8 bytes. */
export function stringLen(s: string): number {
	return encoder.encode(s).length;
}

/** `char::from_u32`: `n`, unless it is a surrogate or past the last char. */
export function charFromU32(n: u32): char | undefined {
	return n > 0x10ffff || (n >= 0xd800 && n <= 0xdfff) ? undefined : (n as char);
}

// Formatting, as `core::fmt` does it: the same output byte for byte.

/** `core::fmt::Error`. */
export class FmtError {}

/** `core::str::ParseBoolError`. */
export class ParseBoolError {}

/** `fmt::Result`. */
export type FmtResult = Result<undefined, FmtError>;

/** A value ready to write: `&dyn Display`, `&dyn Debug`, or one placeholder's argument. */
export type Fmt = (f: Formatter) => FmtResult;

/** A type's `Debug` or `Display`, as a function of its values. */
export type Debug<T> = (value: T, f: Formatter) => FmtResult;

/** `Box<dyn Debug>` or the like of `value`, written by `write`, which drops it by `drop` as it drops. */
export function fmtOwned<T>(value: T, write: Debug<T>, drop?: (value: T) => void): Fmt {
	const fmt = (f: Formatter) => write(value, f);
	return drop === undefined ? fmt : Object.assign(fmt, { [Symbol.dispose]: () => drop(value) });
}

/** `fmt::Arguments`: literal pieces, and the values written between them. */
export type Arguments = readonly (string | Fmt)[];

/** One placeholder's options: `{:*^9}` is `{ fill: "*", align: "^", width: 9 }`. */
export interface Spec {
	readonly fill?: string;
	readonly align?: "<" | "^" | ">";
	/** The `+` flag: a sign on a nonnegative number too. */
	readonly plus?: boolean;
	/** The `#` flag: `{:#?}` and `{:#x}`. */
	readonly alternate?: boolean;
	/** The `0` flag: zeros between the sign and the digits. */
	readonly zero?: boolean;
	/** `{:x?}` or `{:X?}`: an integer's `Debug` in hex. */
	readonly debugHex?: "x" | "X";
	readonly width?: number;
	/** The `.N` of `{:.N}`: the most chars `pad` writes, or digits past a number's point. */
	readonly precision?: number;
}

/** A width or precision from an argument, as in `{:1$}`: a `fmt::rt::Argument` only a spec reads. */
export interface Count extends Fmt {
	readonly count: number;
}

/** `Argument::from_usize`: `n` as a width or precision, panicking past what one holds. */
export function fmtCount(n: number): Count {
	if (n > 0xffff) panic("Formatting argument out of range");
	const unreachable = (): never => {
		throw new Error("rust2-ts: a width or precision written as a placeholder");
	};
	return Object.assign(unreachable, { count: n });
}

/** The width or precision `argument`, a `Count`, holds. */
export function fmtCountOf(argument: Fmt): number {
	return (argument as Count).count;
}

/** `dyn fmt::Write`: where a `Formatter` writes. */
export interface Write {
	writeStr(s: string): FmtResult;
}

class StringWriter implements Write {
	s = "";

	writeStr(s: string): FmtResult {
		this.s += s;
		return OK;
	}
}

/** Indents each line it writes, for `{:#?}`'s nesting. */
class PadAdapter implements Write {
	private onNewline = true;
	private readonly out: Write;

	constructor(out: Write) {
		this.out = out;
	}

	writeStr(s: string): FmtResult {
		for (const line of s.split(/(?<=\n)/)) {
			if (line === "") continue;
			if (this.onNewline) {
				const r = this.out.writeStr("    ");
				if (!r.ok) return r;
			}
			this.onNewline = line.endsWith("\n");
			const r = this.out.writeStr(line);
			if (!r.ok) return r;
		}
		return OK;
	}
}

/** `core::fmt::Formatter`: where it writes, and its placeholder's options. */
export interface Formatter {
	readonly out: Write;
	readonly spec: Spec;
}

/** `fmt::write`: each placeholder gets a fresh `Formatter` with only its own options. */
function write(out: Write, args: Arguments): FmtResult {
	for (const part of args) {
		const r = typeof part === "string" ? out.writeStr(part) : part({ out, spec: {} });
		if (!r.ok) return r;
	}
	return OK;
}

/** The number of chars in `s`, as Rust counts them for padding. */
function chars(s: string): number {
	let n = 0;
	for (const _ of s) n++;
	return n;
}

/** `Formatter::write_str`. */
export function fmtWriteStr(f: Formatter, s: string): FmtResult {
	return f.out.writeStr(s);
}

/** `Formatter::write_fmt`. */
export function fmtWriteFmt(f: Formatter, args: Arguments): FmtResult {
	return write(f.out, args);
}

/** `write!` into a `String`, `s`: `s` with what was written, even before an error, and the result. */
export function fmtWriteString(s: string, args: Arguments): { s: string; result: FmtResult } {
	const out = new StringWriter();
	out.s = s;
	const result = write(out, args);
	return { s: out.s, result };
}

/** `Formatter::alternate`: whether the spec has `#`. */
export function fmtAlternate(f: Formatter): boolean {
	return f.spec.alternate === true;
}

/** `Formatter::width`: the spec's width, or `undefined`. */
export function fmtWidth(f: Formatter): number | undefined {
	return f.spec.width;
}

/** Write `s`, cut to the precision in chars, padded to the width. */
export function fmtPad(f: Formatter, s: string): FmtResult {
	const { width, precision } = f.spec;
	if (width === undefined && precision === undefined) return f.out.writeStr(s);
	let count = 0;
	if (precision === undefined) {
		count = chars(s);
	} else {
		let end = 0;
		for (const c of s) {
			if (count === precision) break;
			end += c.length;
			count++;
		}
		s = s.slice(0, end);
	}
	if (width === undefined || count >= width) return f.out.writeStr(s);
	return padded(f, width - count, f.spec.align ?? "<", f.spec.fill ?? " ", s);
}

/** Write an integer's sign, `prefix` if alternate, and `digits`, padded to the width. */
export function fmtPadIntegral(f: Formatter, nonnegative: boolean, prefix: string, digits: string): FmtResult {
	return fmtPadFormatted(f, (nonnegative ? (f.spec.plus ? "+" : "") : "-") + (fmtAlternate(f) ? prefix : ""), digits);
}

/** `Formatter::pad_formatted_parts`: `head` and `digits`, ASCII, padded to the width, the `0` flag's zeros between them. */
export function fmtPadFormatted(f: Formatter, head: string, digits: string): FmtResult {
	const length = head.length + digits.length;
	const width = f.spec.width ?? 0;
	if (length >= width) return f.out.writeStr(head + digits);
	if (f.spec.zero) {
		const r = f.out.writeStr(head);
		return r.ok ? padded(f, width - length, ">", "0", digits) : r;
	}
	return padded(f, width - length, f.spec.align ?? ">", f.spec.fill ?? " ", head + digits);
}

/** Write `s` with `padding` fills around it. */
function padded(f: Formatter, padding: number, align: "<" | "^" | ">", fill: string, s: string): FmtResult {
	const left = align === "<" ? 0 : align === ">" ? padding : Math.floor(padding / 2);
	const r = f.out.writeStr(fill.repeat(left) + s);
	return r.ok ? f.out.writeStr(fill.repeat(padding - left)) : r;
}

/** `Formatter::debug_struct`. */
export function fmtDebugStruct(f: Formatter, name: string): DebugStruct {
	return { fmt: f, result: f.out.writeStr(name), hasFields: false };
}

/** `Formatter::debug_tuple`. */
export function fmtDebugTuple(f: Formatter, name: string): DebugTuple {
	return { fmt: f, result: f.out.writeStr(name), fields: 0, emptyName: name === "" };
}

/** `Formatter::debug_list`. */
export function fmtDebugList(f: Formatter): DebugList {
	return { fmt: f, result: f.out.writeStr("["), hasFields: false };
}

/** `debug_struct_fields_finish`, which derived `Debug` calls past five fields. */
export function fmtDebugStructFields(f: Formatter, name: string, names: readonly string[], values: readonly Fmt[]): FmtResult {
	if (names.length !== values.length) {
		assertFailed(
			"==",
			(f) => usizeDisplay(names.length, f),
			(f) => usizeDisplay(values.length, f),
		);
	}
	const builder = fmtDebugStruct(f, name);
	names.forEach((field, i) => debugStructField(builder, field, values[i]));
	return debugStructFinish(builder);
}

/** `debug_tuple_fields_finish`, which derived `Debug` calls past five fields. */
export function fmtDebugTupleFields(f: Formatter, name: string, values: readonly Fmt[]): FmtResult {
	const builder = fmtDebugTuple(f, name);
	for (const value of values) debugTupleField(builder, value);
	return debugTupleFinish(builder);
}

/** `fmt::DebugStruct`. */
export interface DebugStruct {
	readonly fmt: Formatter;
	result: FmtResult;
	hasFields: boolean;
}

/** `DebugStruct::field`. */
export function debugStructField(b: DebugStruct, name: string, value: Fmt): DebugStruct {
	if (b.result.ok) {
		if (fmtAlternate(b.fmt)) {
			const head = b.hasFields ? OK : b.fmt.out.writeStr(" {\n");
			b.result = head.ok ? pretty(b.fmt, [name, ": ", value, ",\n"]) : head;
		} else {
			b.result = b.fmt.out.writeStr((b.hasFields ? ", " : " { ") + name + ": ");
			if (b.result.ok) b.result = value(b.fmt);
		}
	}
	b.hasFields = true;
	return b;
}

/** `DebugStruct::finish_non_exhaustive`. */
export function debugStructFinishNonExhaustive(b: DebugStruct): FmtResult {
	if (b.result.ok) {
		if (!b.hasFields) b.result = b.fmt.out.writeStr(" { .. }");
		else if (!fmtAlternate(b.fmt)) b.result = b.fmt.out.writeStr(", .. }");
		else b.result = closePretty(b.fmt, "}");
	}
	return b.result;
}

/** `DebugStruct::finish`. */
export function debugStructFinish(b: DebugStruct): FmtResult {
	if (b.hasFields && b.result.ok) b.result = b.fmt.out.writeStr(fmtAlternate(b.fmt) ? "}" : " }");
	return b.result;
}

/** `fmt::DebugTuple`. */
export interface DebugTuple {
	readonly fmt: Formatter;
	result: FmtResult;
	fields: number;
	readonly emptyName: boolean;
}

/** `DebugTuple::field`. */
export function debugTupleField(b: DebugTuple, value: Fmt): DebugTuple {
	if (b.result.ok) {
		if (fmtAlternate(b.fmt)) {
			const head = b.fields > 0 ? OK : b.fmt.out.writeStr("(\n");
			b.result = head.ok ? pretty(b.fmt, [value, ",\n"]) : head;
		} else {
			b.result = b.fmt.out.writeStr(b.fields === 0 ? "(" : ", ");
			if (b.result.ok) b.result = value(b.fmt);
		}
	}
	b.fields++;
	return b;
}

/** `DebugTuple::finish_non_exhaustive`. */
export function debugTupleFinishNonExhaustive(b: DebugTuple): FmtResult {
	if (b.result.ok) {
		if (b.fields === 0) b.result = b.fmt.out.writeStr("(..)");
		else if (!fmtAlternate(b.fmt)) b.result = b.fmt.out.writeStr(", ..)");
		else b.result = closePretty(b.fmt, ")");
	}
	return b.result;
}

/** `DebugTuple::finish`. */
export function debugTupleFinish(b: DebugTuple): FmtResult {
	if (b.fields > 0 && b.result.ok) {
		const comma = b.fields === 1 && b.emptyName && !fmtAlternate(b.fmt);
		b.result = b.fmt.out.writeStr(comma ? ",)" : ")");
	}
	return b.result;
}

/** `fmt::DebugList`. */
export interface DebugList {
	readonly fmt: Formatter;
	result: FmtResult;
	hasFields: boolean;
}

/** `DebugList::entry`. */
export function debugListEntry(b: DebugList, value: Fmt): DebugList {
	if (b.result.ok) {
		if (fmtAlternate(b.fmt)) {
			const head = b.hasFields ? OK : b.fmt.out.writeStr("\n");
			b.result = head.ok ? pretty(b.fmt, [value, ",\n"]) : head;
		} else {
			b.result = b.hasFields ? b.fmt.out.writeStr(", ") : OK;
			if (b.result.ok) b.result = value(b.fmt);
		}
	}
	b.hasFields = true;
	return b;
}

/** `DebugList::entries`: each item written and then dropped by `drop`, and then `items` dropped, as std's `for` does. */
export function debugListEntries<T>(b: DebugList, items: Iterable<T>, debug: Debug<T>, drop?: (value: T) => void): DebugList {
	for (const item of items) {
		debugListEntry(b, (f) => debug(item, f));
		drop?.(item);
	}
	dispose(items);
	return b;
}

/** `DebugList::finish_non_exhaustive`. */
export function debugListFinishNonExhaustive(b: DebugList): FmtResult {
	if (!b.result.ok) return b.result;
	if (!b.hasFields) return b.fmt.out.writeStr("..]");
	if (!fmtAlternate(b.fmt)) return b.fmt.out.writeStr(", ..]");
	return closePretty(b.fmt, "]");
}

/** `DebugList::finish`. */
export function debugListFinish(b: DebugList): FmtResult {
	if (b.result.ok) b.result = b.fmt.out.writeStr("]");
	return b.result;
}

/** Write `parts` indented, as a pretty builder writes one field: values keep `f`'s options. */
function pretty(f: Formatter, parts: Arguments): FmtResult {
	const writer: Formatter = { out: new PadAdapter(f.out), spec: f.spec };
	for (const part of parts) {
		const r = typeof part === "string" ? writer.out.writeStr(part) : part(writer);
		if (!r.ok) return r;
	}
	return OK;
}

/** A pretty builder's `finish_non_exhaustive`: an indented `..`, then `close`. */
function closePretty(f: Formatter, close: string): FmtResult {
	const r = pretty(f, ["..\n"]);
	return r.ok ? f.out.writeStr(close) : r;
}

// `Debug` and `Display` of what std defines them for. A string's `Display` is `fmtPad`.

/** `format!`, from `alloc::fmt::format`. */
export function fmtFormat(args: Arguments): string {
	const out = new StringWriter();
	if (!write(out, args).ok) {
		panic("a formatting trait implementation returned an error when the underlying stream did not: Error");
	}
	return out.s;
}

/** `ToString::to_string` of a `Display` value. */
export function fmtToString(value: Fmt): string {
	const out = new StringWriter();
	if (!value({ out, spec: {} }).ok) panic("a Display implementation returned an error unexpectedly: Error");
	return out.s;
}

/** A placeholder with options, which replace the defaults its value is written with. */
export function fmtSpec(spec: Spec, value: Fmt): Fmt {
	return (f) => value({ out: f.out, spec });
}

/** `Display` and `Debug` of a `bool`. */
export function fmtBool(value: boolean, f: Formatter): FmtResult {
	return fmtPad(f, value ? "true" : "false");
}

/** `Debug` of a string. */
export function fmtStr(s: string, f: Formatter): FmtResult {
	let out = '"';
	for (const c of s) out += escapeDebug(c, '"');
	return f.out.writeStr(out + '"');
}

/** `Display` of a `char`. */
export function fmtCharDisplay(c: char, f: Formatter): FmtResult {
	return fmtPad(f, String.fromCodePoint(c));
}

/** `Debug` of a `char`, which no width pads. */
export function fmtCharDebug(c: char, f: Formatter): FmtResult {
	return f.out.writeStr(`'${escapeDebug(String.fromCodePoint(c), "'")}'`);
}

/** `Debug` of `()`. */
export function fmtUnit(_: undefined, f: Formatter): FmtResult {
	return fmtPad(f, "()");
}

/** `Debug` of an `Option`, through its value's. */
export function fmtOption<T>(o: T | undefined, f: Formatter, debug: Debug<T>): FmtResult;
export function fmtOption<T>(o: T | None, f: Formatter, debug: Debug<T>, none: None): FmtResult;
export function fmtOption<T>(o: T | undefined | None, f: Formatter, debug: Debug<T>, none?: None): FmtResult {
	if (o === none) return f.out.writeStr("None");
	return debugTupleFinish(debugTupleField(fmtDebugTuple(f, "Some"), (f) => debug(o as T, f)));
}

/** `Debug` of a `Result`, through its value's and error's. */
export function fmtResult<T, E>(r: Result<T, E>, f: Formatter, ok: Debug<T>, err: Debug<E>): FmtResult {
	const field: Fmt = r.ok ? (f) => ok(r.value, f) : (f) => err(r.error, f);
	return debugTupleFinish(debugTupleField(fmtDebugTuple(f, r.ok ? "Ok" : "Err"), field));
}

/** `Debug` of a slice, array, `Vec`, `VecDeque`, or `BinaryHeap`. */
export function fmtList<T>(items: Iterable<T>, f: Formatter, debug: Debug<T>): FmtResult {
	return debugListFinish(debugListEntries(fmtDebugList(f), items, debug));
}

/** `Debug` of a set: std's `DebugSet` is its `DebugList` in braces. */
export function fmtSet<T>(items: Iterable<T>, f: Formatter, debug: Debug<T>): FmtResult {
	const b = debugListEntries({ fmt: f, result: f.out.writeStr("{"), hasFields: false }, items, debug);
	if (b.result.ok) b.result = f.out.writeStr("}");
	return b.result;
}

/** `Debug` of a map: a set of entries, each its key and value apart by `": "` in one indented field. */
export function fmtMap<K, V>(entries: Iterable<[K, V]>, f: Formatter, key: Debug<K>, value: Debug<V>): FmtResult {
	return fmtSet(entries, f, ([k, v], f) => {
		let r = key(k, f);
		if (r.ok) r = f.out.writeStr(": ");
		return r.ok ? value(v, f) : r;
	});
}

/** What one of std's iterators writes around its list: a tuple struct's name, or a struct's name and its one field's. */
export type IterWrap = string | readonly [name: string, field: string];

/** `Debug` of one of std's iterators: the items `it` has left, as a list inside `wraps`, outermost first. */
export function fmtIter<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>, wraps: readonly IterWrap[] = []): FmtResult {
	return wrapIn(f, wraps, (f) => fmtList(it.remaining() as Iterable<T>, f, debug));
}

/** `Debug` of an `Option`'s, a `Result`'s, or a `Once`'s iterator: the item `it` has left, if any, as an `Option` inside `wraps`, outermost first. */
export function fmtIterOption<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>, wraps: readonly IterWrap[]): FmtResult {
	const items = [...it.remaining()] as T[];
	return wrapIn(f, wraps, (f) => (items.length === 0 ? f.out.writeStr("None") : fmtDebugTupleFields(f, "Some", [(f) => debug(items[0], f)])));
}

/** `Debug` of what std writes as the list of what a clone of it yields: a `HashSet`'s set operations. */
export function fmtIterClone<T>(it: Iter<T>, f: Formatter, debug: Debug<T>): FmtResult {
	return fmtList(it.clone(), f, debug);
}

/** `Debug` of a `LinkedList`'s `Iter` or `IterMut`, `name`: the list it has left, then its length by `len`. */
export function fmtListIter<T>(it: Iter<unknown>, f: Formatter, debug: Debug<T>, len: Debug<usize>, name: string): FmtResult {
	const list: Fmt = (f) => fmtList(it.remaining() as Iterable<T>, f, debug);
	return fmtDebugTupleFields(f, name, [list, (f) => len(it.len(), f)]);
}

function wrapIn(f: Formatter, wraps: readonly IterWrap[], inner: Fmt): FmtResult {
	if (wraps.length === 0) return inner(f);
	const [wrap, ...rest] = wraps;
	const field: Fmt = (f) => wrapIn(f, rest, inner);
	if (typeof wrap === "string") return fmtDebugTupleFields(f, wrap, [field]);
	return fmtDebugStructFields(f, wrap[0], [wrap[1]], [field]);
}

/** `Debug` of `str::Chars`: its name around the list of chars it has left, which a field's indent would not be. */
export function fmtChars(it: Iter<char>, f: Formatter): FmtResult {
	let r = f.out.writeStr("Chars(");
	if (r.ok) r = fmtList(it.remaining() as Iterable<char>, f, fmtCharDebug);
	return r.ok ? f.out.writeStr(")") : r;
}

/** `Debug` of a key as a `BTreeSet`'s `range` and `into_iter` list it: its map's entry, the key and a `SetValZST`. */
export function fmtSetVal<K>(debug: Debug<K>): Debug<K> {
	return (key, f) => fmtTuple([(f) => debug(key, f), (f) => f.out.writeStr("SetValZST")], f);
}

/** `Debug` of a range: each bound present, written with the formatter's options, around `dots`, which none pads. */
function fmtBounds(f: Formatter, start: Fmt | undefined, dots: string, end: Fmt | undefined): FmtResult {
	if (start !== undefined) {
		const r = start(f);
		if (!r.ok) return r;
	}
	const r = f.out.writeStr(dots);
	return r.ok && end !== undefined ? end(f) : r;
}

/** `Debug` of `start..end`. */
export function fmtRange<T>(r: Range<T>, f: Formatter, debug: Debug<T>): FmtResult {
	return fmtBounds(f, (f) => debug(r.start, f), "..", (f) => debug(r.end, f));
}

/** `Debug` of `start..=end`, which notes once iteration has exhausted it. */
export function fmtRangeInclusive<T>(r: RangeInclusive<T>, f: Formatter, debug: Debug<T>): FmtResult {
	const result = fmtBounds(f, (f) => debug(r.lo, f), "..=", (f) => debug(r.hi, f));
	return result.ok && r.exhausted ? f.out.writeStr(" (exhausted)") : result;
}

/** `Debug` of `start..`. */
export function fmtRangeFrom<T>(r: { start: T }, f: Formatter, debug: Debug<T>): FmtResult {
	return fmtBounds(f, (f) => debug(r.start, f), "..", undefined);
}

/** `Debug` of `..end`. */
export function fmtRangeTo<T>(r: { end: T }, f: Formatter, debug: Debug<T>): FmtResult {
	return fmtBounds(f, undefined, "..", (f) => debug(r.end, f));
}

/** `Debug` of `..=end`. */
export function fmtRangeToInclusive<T>(r: { end: T }, f: Formatter, debug: Debug<T>): FmtResult {
	return fmtBounds(f, undefined, "..=", (f) => debug(r.end, f));
}

/** `Debug` of `..`. */
export function fmtRangeFull(_: undefined, f: Formatter): FmtResult {
	return f.out.writeStr("..");
}

/** `Debug` of a `Bound`: its variant, around what it holds. */
export function fmtBound<T>(b: RangeBound<T>, f: Formatter, debug: Debug<T>): FmtResult {
	if (b.tag === "Unbounded") return f.out.writeStr("Unbounded");
	return fmtDebugTupleFields(f, b.tag, [(f) => debug(b._0, f)]);
}

/** `Debug` of an `Ordering`, unpadded. */
export function fmtOrdering(o: Ordering, f: Formatter): FmtResult {
	return f.out.writeStr(o < 0 ? "Less" : o > 0 ? "Greater" : "Equal");
}

/** `Debug` of a `ControlFlow` held as a `Result`, `Ok` to continue: `stop` writes what it breaks with, the `Err` itself with `residual`, as `?` breaks with it. */
export function fmtControlFlow<C, B>(r: Result<C, B>, f: Formatter, go: Debug<C>, stop: Debug<B>): FmtResult;
export function fmtControlFlow<C, E>(r: Result<C, E>, f: Formatter, go: Debug<C>, stop: Debug<Result<never, E>>, residual: true): FmtResult;
export function fmtControlFlow<C, B>(r: Result<C, B>, f: Formatter, go: Debug<C>, stop: Debug<B> | Debug<Result<never, B>>, residual = false): FmtResult {
	if (r.ok) return fmtDebugTupleFields(f, "Continue", [(f) => go(r.value, f)]);
	const held: Fmt = residual ? (f) => (stop as Debug<Result<never, B>>)(r, f) : (f) => (stop as Debug<B>)(r.error, f);
	return fmtDebugTupleFields(f, "Break", [held]);
}

/** `Debug` of a `ControlFlow` held as the `Option` `?` was applied to, which breaks with `None`. */
export function fmtControlFlowOption<C>(o: C | undefined, f: Formatter, go: Debug<C>, stop: Debug<undefined>): FmtResult;
export function fmtControlFlowOption<C>(o: C | None, f: Formatter, go: Debug<C>, stop: Debug<undefined>, none: None): FmtResult;
export function fmtControlFlowOption<C>(o: C | undefined | None, f: Formatter, go: Debug<C>, stop: Debug<undefined>, none?: None): FmtResult {
	if (o === none) return fmtDebugTupleFields(f, "Break", [(f) => stop(undefined, f)]);
	return fmtDebugTupleFields(f, "Continue", [(f) => go(o as C, f)]);
}

/** `Debug` of a tuple, given each element's. */
export function fmtTuple(fields: readonly Fmt[], f: Formatter): FmtResult {
	const builder = fmtDebugTuple(f, "");
	for (const field of fields) debugTupleField(builder, field);
	return debugTupleFinish(builder);
}

/** `Display` of `fmt::Arguments`, which ignores the formatter's options. */
export function fmtArguments(args: Arguments, f: Formatter): FmtResult {
	return fmtWriteFmt(f, args);
}

/** `Debug` of `fmt::Error`. */
export function fmtErrorDebug(_: FmtError, f: Formatter): FmtResult {
	return f.out.writeStr("Error");
}

/** `Display` of `fmt::Error`. */
export function fmtErrorDisplay(_: FmtError, f: Formatter): FmtResult {
	return fmtPad(f, "an error occurred when formatting an argument");
}

/** `Debug` of `TryFromIntError`. */
export function fmtTryFromIntErrorDebug(e: TryFromIntError, f: Formatter): FmtResult {
	return debugTupleFinish(debugTupleField(fmtDebugTuple(f, "TryFromIntError"), (f) => f.out.writeStr(e.kind)));
}

/** `Debug` of `ParseIntError`. */
export function fmtParseIntErrorDebug(e: ParseIntError, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "ParseIntError", ["kind"], [(f) => f.out.writeStr(e.kind)]);
}

/** `Display` of `ParseIntError`. */
export function fmtParseIntErrorDisplay(e: ParseIntError, f: Formatter): FmtResult {
	const messages = {
		Empty: "cannot parse integer from empty string",
		InvalidDigit: "invalid digit found in string",
		PosOverflow: "number too large to fit in target type",
		NegOverflow: "number too small to fit in target type",
		Zero: "number would be zero for non-zero type",
		NotAPowerOfTwo: "number is not a power of two",
	};
	return fmtPad(f, messages[e.kind]);
}

/** `Debug` of `ParseBoolError`. */
export function fmtParseBoolErrorDebug(_: ParseBoolError, f: Formatter): FmtResult {
	return f.out.writeStr("ParseBoolError");
}

/** `Display` of `ParseBoolError`. */
export function fmtParseBoolErrorDisplay(_: ParseBoolError, f: Formatter): FmtResult {
	return fmtPad(f, "provided string was not `true` or `false`");
}

/** `Debug` of `ParseCharError`. */
export function fmtParseCharErrorDebug(e: ParseCharError, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "ParseCharError", ["kind"], [(f) => f.out.writeStr(e.kind)]);
}

/** `Display` of `ParseCharError`. */
export function fmtParseCharErrorDisplay(e: ParseCharError, f: Formatter): FmtResult {
	return fmtPad(f, e.kind === "EmptyString" ? "cannot parse char from empty string" : "too many characters in string");
}

/** `Debug` of `ParseFloatError`. */
export function fmtParseFloatErrorDebug(e: ParseFloatError, f: Formatter): FmtResult {
	return fmtDebugStructFields(f, "ParseFloatError", ["kind"], [(f) => f.out.writeStr(e.kind)]);
}

/** `Display` of `ParseFloatError`. */
export function fmtParseFloatErrorDisplay(e: ParseFloatError, f: Formatter): FmtResult {
	return fmtPad(f, e.kind === "Empty" ? "cannot parse float from empty string" : "invalid float literal");
}

/** `Debug` of `PoisonError`. */
export function fmtPoisonErrorDebug(_: unknown, f: Formatter): FmtResult {
	return f.out.writeStr("PoisonError { .. }");
}

/** `Display` of `PoisonError`. */
export function fmtPoisonErrorDisplay(_: unknown, f: Formatter): FmtResult {
	return fmtPad(f, "poisoned lock: another task failed inside");
}

/** `Display` of `TryFromIntError`. */
export function fmtTryFromIntErrorDisplay(e: TryFromIntError, f: Formatter): FmtResult {
	const positive = e.kind === "PosOverflow";
	return fmtPad(f, `number too ${positive ? "large" : "small"} to fit in target type`);
}

/** `Debug` of `Utf8Error`. */
export function fmtUtf8ErrorDebug(e: Utf8Error, f: Formatter): FmtResult {
	return debugStructFinish(
		debugStructField(
			debugStructField(fmtDebugStruct(f, "Utf8Error"), "valid_up_to", (f) => usizeDebug(e.validUpTo, f)),
			"error_len",
			(f) => fmtOption(e.errorLen, f, usizeDebug),
		),
	);
}

/** `Display` of `Utf8Error`, which `write!`s and so ignores the formatter's options. */
export function fmtUtf8ErrorDisplay(e: Utf8Error, f: Formatter): FmtResult {
	if (e.errorLen === undefined) return f.out.writeStr(`incomplete utf-8 byte sequence from index ${e.validUpTo}`);
	return f.out.writeStr(`invalid utf-8 sequence of ${e.errorLen} bytes from index ${e.validUpTo}`);
}

/** `Debug` of `FromUtf8Error`. */
export function fmtFromUtf8ErrorDebug(e: FromUtf8Error, f: Formatter): FmtResult {
	return debugStructFinish(
		debugStructField(
			debugStructField(fmtDebugStruct(f, "FromUtf8Error"), "bytes", (f) => fmtList(byteVecView(e.bytes), f, u8Debug)),
			"error",
			(f) => fmtUtf8ErrorDebug(e.error, f),
		),
	);
}

/** `Display` of `FromUtf8Error`: its `Utf8Error`'s. */
export function fmtFromUtf8ErrorDisplay(e: FromUtf8Error, f: Formatter): FmtResult {
	return fmtUtf8ErrorDisplay(e.error, f);
}

