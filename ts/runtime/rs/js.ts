// wasm-bindgen, for rust2-ts output: a `JsValue`, or any type an import declares, is the
// JS value itself, and an import is a direct JS call. These are the conversions at the
// boundary, which take a Rust value to the JS one wasm-bindgen's JS glue passes and back
// as it does, coercing as the WebAssembly ABI coerces, and the JS bodies of wasm-bindgen's
// intrinsics, as its CLI gives them. `rs.ts` re-exports all of it.

import { ByteSlice, ByteVec, byteSliceFrom, Err, Ok, type Result, Slice, Vec } from "../rs.ts";
import { type char, type f32, type i8, type i16, type i32, intFromBig, intToBig, type isize, type Limbs, type u8, type u16, type u32, type usize } from "./int.ts";
import { String as RsString, stringFrom } from "./str.ts";
import { type Future, Pending, type Waker, type WakerVTable, wakerVTable } from "./task.ts";

/** A JS value, as a `JsValue` and every imported type are. */
export type JsValue = unknown;

/** The global object, whose properties an import names. */
// biome-ignore lint/suspicious/noExplicitAny: an import may name any global, with any shape.
export const global: any = globalThis;

/** A JS value as the JS glue reads it, of any shape. */
// biome-ignore lint/suspicious/noExplicitAny: an import's value has whatever shape JS gives it.
export function jsAny(v: unknown): any {
	return v;
}

/** `u64`, `i64`, `u128`, or `i128` to JS: a BigInt of its value. */
export function jsBigInt(v: number | Limbs): bigint {
	return intToBig(v);
}

/** `char` to JS: a string of it. */
export function jsChar(c: char): string {
	return String.fromCodePoint(c);
}

/** `String` to JS. */
export function jsString(s: RsString): string {
	return s.v;
}

/** A `Vec<u8>` or a `Box<[u8]>` to JS: a copy, as the JS glue copies it out of memory. */
export function jsByteVec(v: ByteVec): Uint8Array {
	return v.data.slice(0, v.length);
}

/** A typed array constructor, of a slice's elements. */
type Typed = { new (length: number): ArrayLike<unknown> & { [n: number]: unknown; set(a: ArrayLike<unknown>): void } };

/** A slice of numbers to JS: a typed array of their values, as JS views them in memory. */
export function jsTyped<T>(of: Typed, s: Slice<T>, big: boolean): ArrayLike<unknown> {
	const out = new of(s.length);
	for (let i = 0; i < s.length; i++) out[i] = big ? intToBig(s.data[s.start + i] as number | Limbs) : s.data[s.start + i];
	return out;
}

/** What JS wrote into a typed array `jsTyped` made of `s`, back into `s`, as JS writes a `&mut` slice in memory. */
export function jsTypedBack<T>(s: Slice<T>, from: ArrayLike<unknown>, back: (v: unknown) => T): void {
	for (let i = 0; i < s.length; i++) s.data[s.start + i] = back(from[i]);
}

/** A slice or `Vec` of JS values, strings, or other items to JS: an array of each, converted. */
export function jsArray<T>(items: Slice<T> | Vec<T>, to: (v: T) => unknown): unknown[] {
	const [data, start, length] = items instanceof Slice ? [items.data, items.start, items.length] : [items.items, 0, items.items.length];
	const out = new Array<unknown>(length);
	for (let i = 0; i < length; i++) out[i] = to(data[start + i]);
	return out;
}

/** Whether a JS value is `None` to an `Option`: `undefined` or `null`. */
export function jsIsNone(v: unknown): boolean {
	return v === undefined || v === null;
}

// From JS, as an import's result: each integer through the WebAssembly ABI's `i32`, which
// is `ToInt32`, then read as the Rust type, as wasm-bindgen's `from_abi` reads it. `+` is
// its `ToNumber`, which throws on a BigInt as the ABI does, where `|` would mix them.

export function jsToU8(v: unknown): u8 {
	return +(v as number) & 0xff;
}

export function jsToI8(v: unknown): i8 {
	return (+(v as number) << 24) >> 24;
}

export function jsToU16(v: unknown): u16 {
	return +(v as number) & 0xffff;
}

export function jsToI16(v: unknown): i16 {
	return (+(v as number) << 16) >> 16;
}

export function jsToU32(v: unknown): u32 {
	return +(v as number) >>> 0;
}

export function jsToI32(v: unknown): i32 {
	return +(v as number) | 0;
}

export function jsToUsize(v: unknown): usize {
	return +(v as number) >>> 0;
}

export function jsToIsize(v: unknown): isize {
	return +(v as number) | 0;
}

/** `f64`, through the ABI's `ToNumber`, which throws on a BigInt or a Symbol. */
export function jsToF64(v: unknown): number {
	return +(v as number);
}

export function jsToF32(v: unknown): f32 {
	return Math.fround(+(v as number));
}

export function jsToBool(v: unknown): boolean {
	return (+(v as number) | 0) !== 0;
}

/** A 64- or 128-bit integer: `BigInt.asIntN` and `asUintN` take a BigInt only, as the ABI's `ToBigInt64` does. */
export function jsToWide(v: unknown, bits: number, signed: boolean): number | Limbs {
	// The JS glue splits a 128-bit value by a shift, which throws on anything but a BigInt.
	if (bits === 128) (v as bigint) >> 64n;
	const big = signed ? BigInt.asIntN(bits, v as bigint) : BigInt.asUintN(bits, v as bigint);
	return intFromBig(big, bits, signed);
}

/** `char`: a string's first code point, or 0 for an empty one. No `char` is a surrogate, so a lone one throws, as the JS glue throws for an `Option<char>`. */
export function jsToChar(v: unknown): char {
	const c = ((v as string).codePointAt(0) as number) | 0;
	if (c >= 0x110000 || (c >= 0xd800 && c < 0xe000)) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
	return c as char;
}

/** A string as UTF-8 holds it: each lone surrogate as U+FFFD, as `TextEncoder` writes it. */
function wellFormed(s: string): string {
	return s.replace(/[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff]/g, "�");
}

/** `String`: a copy of a JS string with room for exactly its bytes, as the JS glue allocates it. Anything but a string throws. */
export function jsToString(v: unknown): RsString {
	if (typeof v !== "string") throw new TypeError(`expected a string, found ${typeof v}`);
	return stringFrom(wellFormed(v));
}

/** An `Option` of a `u8`, `i8`, `u16`, or `i16`: its `u32` through the ABI's `i32`, or `undefined` for `None`, which `undefined`, `null`, and `0xFFFFFF` are. */
export function jsToOptionSmall(v: unknown): u32 | undefined {
	if (jsIsNone(v)) return undefined;
	const n = jsToU32(v);
	return n === 0xffffff ? undefined : n;
}

/** A `Vec<u8>` or `Box<[u8]>`: a copy of an array-like's bytes, each wrapped as `Uint8Array.set` wraps it. */
export function jsToByteVec(v: unknown): ByteVec {
	const items = v as ArrayLike<number>;
	const out = new ByteVec(items.length);
	out.data.set(items);
	out.length = items.length;
	return out;
}

/** A `Box<[u8]>`: a copy of an array-like's bytes, as `jsToByteVec` makes. */
export function jsToByteSlice(v: unknown): ByteSlice {
	return byteSliceFrom(Uint8Array.from(jsToByteVec(v).data));
}

/** A `Vec` of numbers, JS values, or strings: each of an array-like's items, converted. */
export function jsToVec<T>(v: unknown, from: (v: unknown) => T): Vec<T> {
	const items = v as ArrayLike<unknown>;
	const out = new Array<T>(items.length);
	for (let i = 0; i < items.length; i++) out[i] = from(items[i]);
	return new Vec(out, out.length);
}

/** A string enum's variant, by the name translation holds it as, to its JS string: `undefined` for the variant standing for any other. */
export function jsEnum(names: readonly string[], values: readonly string[], v: string): string | undefined {
	return values[names.indexOf(v)];
}

/** A JS value to the string enum's variant whose string it is, or else the last, which stands for any other. */
export function jsToEnum<T extends string>(names: readonly T[], values: readonly string[], v: unknown): T {
	const i = values.indexOf(v as string);
	return names[i < 0 ? names.length - 1 : i];
}

/** A `catch` import's call: `Ok` of its result, converted, or `Err` of what it threw. What converting throws is caught where the JS glue converts it (`glue`), and thrown where the WebAssembly ABI coerces it, after the glue returns. */
export function jsCatch<T>(call: () => unknown, from: (v: unknown) => T, glue: boolean): Result<T, unknown> {
	let v: unknown;
	try {
		v = call();
		if (glue) return Ok(from(v));
	} catch (e) {
		return Err(e);
	}
	return Ok(from(v));
}

/** A variadic import's last argument, spread as a spread reads it: by its iterator, throwing where it has none. */
// biome-ignore lint/suspicious/noExplicitAny: arguments of any type, after those before them.
export function jsSpread(v: unknown): any[] {
	return [...(v as Iterable<unknown>)];
}

/** A variadic import's call of `f` on `self`: `rest`, then `last` spread by its iterator. */
// biome-ignore lint/suspicious/noExplicitAny: an import's result has whatever shape JS gives it.
export function jsSpreadCall(f: unknown, self: unknown, rest: unknown[], last: unknown): any {
	return Reflect.apply(f as (...args: unknown[]) => unknown, self, [...rest, ...(last as Iterable<unknown>)]);
}

/** `base[key](...rest, ...last)`, as a spread call reads it: `base` once, its function, then the arguments spread. */
// biome-ignore lint/suspicious/noExplicitAny: an import's result has whatever shape JS gives it.
export function jsSpreadMethod(base: unknown, key: PropertyKey, rest: unknown[], last: unknown): any {
	return jsSpreadCall(jsAny(base)[key], base, rest, last);
}

/** `instanceof` the class `class_` names, false where naming it or the check throws, as where the JS glue finds no such class. */
export function jsInstanceof(v: unknown, class_: () => unknown): boolean {
	try {
		return v instanceof (class_() as new () => unknown);
	} catch (_) {
		return false;
	}
}

/** `__wbindgen_is_object`: an object, not `null`, or a function's not. */
export function jsIsObject(v: unknown): boolean {
	return typeof v === "object" && v !== null;
}

/** An `optional` static's value: `null` where the global it starts from is not defined, as the JS glue's `typeof` check gives, else each property of it in turn. `root` names the global, or is the one a vendor prefix found. */
export function jsOptionalPath(root: unknown, ...props: string[]): unknown {
	let v = typeof root === "string" ? global[root] : root;
	if (v === undefined) return null;
	for (const prop of props) v = jsAny(v)?.[prop];
	return v;
}

/** A global the JS glue names as an identifier: a `ReferenceError` where none is defined, as reading an undeclared identifier throws. */
// biome-ignore lint/suspicious/noExplicitAny: a global, of whatever shape JS gives it.
export function jsGlobal(name: string): any {
	if (!(name in global)) throw new ReferenceError(`${name} is not defined`);
	return global[name];
}

/** A global by name, or the first of its vendor-prefixed names that is defined. */
// biome-ignore lint/suspicious/noExplicitAny: a global, of whatever shape JS gives it.
export function jsVendored(name: string, prefixes: readonly string[]): any {
	for (const prefix of ["", ...prefixes]) {
		if (typeof global[prefix + name] !== "undefined") return global[prefix + name];
	}
	return undefined;
}

/** A property's descriptor on `obj` or the nearest prototype that has one, as the JS glue finds a non-structural getter or setter. */
export function jsDescriptor(obj: unknown, id: string): PropertyDescriptor {
	let o = obj as object | null;
	while (o) {
		const desc = Object.getOwnPropertyDescriptor(o, id);
		if (desc) return desc;
		o = Object.getPrototypeOf(o);
	}
	return {};
}

/** `throw`, as an expression. */
export function jsThrow(v: unknown): never {
	throw v;
}

/** `__wbindgen_debug_string`, the text of `JsValue`'s `Debug`, as the CLI's `debugString` gives it. */
export function jsDebugString(val: unknown): string {
	const type = typeof val;
	if (type === "number" || type === "boolean" || val == null) return `${val}`;
	if (type === "string") return `"${val}"`;
	if (type === "symbol") {
		const description = (val as symbol).description;
		return description == null ? "Symbol" : `Symbol(${description})`;
	}
	if (type === "function") {
		const name = (val as { name: unknown }).name;
		return typeof name === "string" && name.length > 0 ? `Function(${name})` : "Function";
	}
	if (Array.isArray(val)) return `[${val.map(jsDebugString).join(", ")}]`;
	// A BigInt, as any other object, by its class.
	const builtIn = /\[object ([^\]]+)\]/.exec(Object.prototype.toString.call(val));
	if (!builtIn || builtIn.length <= 1) return Object.prototype.toString.call(val);
	const className = builtIn[1];
	if (className === "Object") {
		try {
			return `Object(${JSON.stringify(val)})`;
		} catch (_) {
			return "Object";
		}
	}
	if (val instanceof Error) return `${val.name}: ${val.message}\n${val.stack}`;
	return className;
}

/** `__wbindgen_try_into_number`: the number, or what converting threw. */
export function jsTryIntoNumber(v: unknown): unknown {
	try {
		return +(v as number);
	} catch (e) {
		return e;
	}
}

/** `__wbindgen_checked_div`: the quotient, or the `RangeError` dividing a BigInt by zero threw. */
export function jsCheckedDiv(a: unknown, b: unknown): unknown {
	try {
		return jsAny(a) / jsAny(b);
	} catch (e) {
		if (e instanceof RangeError) return e;
		throw e;
	}
}

/** `__wbindgen_copy_to_typed_array`: `data` into `js`'s bytes. */
export function jsCopyToTypedArray(data: Uint8Array, js: unknown): void {
	const dst = js as ArrayBufferView;
	new Uint8Array(dst.buffer, dst.byteOffset, dst.byteLength).set(data);
}

/** `__wbindgen_string_from_utf8_lossy`: the bytes as UTF-8, each malformed sequence as U+FFFD. */
export function jsUtf8Lossy(data: Uint8Array): string {
	return new TextDecoder("utf-8", { ignoreBOM: true }).decode(data);
}

// Closures

/** What wasm-bindgen's closures throw once ended, or called again while running if `FnMut`. */
const CLOSURE_ENDED = "closure invoked recursively or after being dropped";

/** wasm-bindgen's `ScopedClosure`: the JS function over a closure, which `_wbg_cb_unref` ends, as its drop does. */
// biome-ignore lint/suspicious/noExplicitAny: JS calls it with any arguments.
export type JsClosure = ((...args: any[]) => unknown) & Disposable & { _wbg_cb_unref(): void };

/** How JS calls a closure `f`: its arguments from JS, and its result to JS. */
type Invoke<F> = (f: F, args: unknown[]) => unknown;

/** What an owned closure's JS function keeps: whether it is callable, its count of the `Closure` and the calls running, and its drop. */
interface Owned<F> {
	f: F;
	live: boolean;
	count: number;
	drop: ((f: F) => void) | undefined;
}

/** Drop what `state` keeps, once: as a `Closure` and its last call end, or as JS collects its function. */
function ownedEnd<F>(state: Owned<F>): void {
	if (!state.live) return;
	state.live = false;
	state.drop?.(state.f);
}

/** Each owned closure's state, which drops once JS collects its function, as the CLI's `CLOSURE_DTORS` drops it. */
const CLOSURE_DROPS = /* @__PURE__ */ (() =>
	typeof FinalizationRegistry === "undefined" ? undefined : new FinalizationRegistry<Owned<unknown>>(ownedEnd))();

/**
 * A `Closure`'s JS function over `f`, as the CLI's `makeClosure` and `makeMutClosure` make
 * it: a call throws once it ended, or, if `mut`, while another call runs, and `f` drops by
 * `drop` once the `Closure` ends it and no call of it runs.
 */
export function jsClosure<F>(f: F, mut: boolean, invoke: Invoke<F>, drop?: (f: F) => void): JsClosure {
	const state: Owned<F> = { f, live: true, count: 1, drop };
	const real = ((...args: unknown[]) => {
		state.count++;
		const live = state.live;
		if (mut) state.live = false;
		try {
			if (!live) throw new Error(CLOSURE_ENDED);
			return invoke(state.f, args);
		} finally {
			if (mut) state.live = live;
			real._wbg_cb_unref();
		}
	}) as JsClosure;
	real._wbg_cb_unref = () => {
		if (--state.count !== 0) return;
		CLOSURE_DROPS?.unregister(state);
		ownedEnd(state);
	};
	real[Symbol.dispose] = real._wbg_cb_unref;
	CLOSURE_DROPS?.register(real, state as Owned<unknown>, state);
	return real;
}

/**
 * A borrowed closure's JS function over `f`, as the CLI's glue makes it for a
 * `ScopedClosure::borrow` or a `&dyn FnMut` an import takes: a call throws once it ended,
 * or, if `mut`, while another call runs.
 */
export function jsClosureRef<F>(f: F, mut: boolean, invoke: Invoke<F>): JsClosure {
	const state = { live: true };
	const cb = ((...args: unknown[]) => {
		const live = state.live;
		if (mut) state.live = false;
		try {
			if (!live) throw new Error(CLOSURE_ENDED);
			return invoke(f, args);
		} finally {
			if (mut) state.live = live;
		}
	}) as JsClosure;
	cb._wbg_cb_unref = () => {
		state.live = false;
	};
	cb[Symbol.dispose] = cb._wbg_cb_unref;
	return cb;
}

/** An import's call that borrows `closures`, which end as it returns or throws. */
export function jsLending<T>(closures: JsClosure[], call: () => T): T {
	try {
		return call();
	} finally {
		for (const c of closures) c._wbg_cb_unref();
	}
}

/** What a closure's `Result` returns to JS: its `Ok`, or its `Err`, thrown. */
export function jsReturn<T>(r: Result<T, unknown>): T {
	if (r.ok) return r.value;
	throw r.error;
}

// js-sys's task queue

/** The tasks `spawn_local` queued, in order. */
const localQueue: LocalTask[] = [];
let localScheduled = false;

/** js-sys's `QueueState::run_all`: each task queued before it began, once; a task queued as it runs waits for the next microtask. */
function localRun(): void {
	localScheduled = false;
	for (let left = localQueue.length; left > 0; left--) {
		const task = localQueue.shift();
		if (task === undefined) break;
		task.run();
	}
}

/** The vtable of every local task's waker, whose data is the task. */
const LOCAL_VTABLE: WakerVTable = /* @__PURE__ */ wakerVTable(
	(data) => ({ data, vtable: LOCAL_VTABLE }),
	(data) => (data as unknown as LocalTask).wake(),
	(data) => (data as unknown as LocalTask).wake(),
	() => {},
);

/** js-sys's `Task`: a spawned future, which is also the waker of each poll it gets. */
class LocalTask implements Waker {
	#future: Future<undefined> | undefined;
	#queued = true;

	constructor(future: Future<undefined>) {
		this.#future = future;
	}

	/** Queue it, for the next microtask if none is queued. */
	schedule(): void {
		localQueue.push(this);
		if (localScheduled) return;
		localScheduled = true;
		queueMicrotask(localRun);
	}

	/** Poll it, dropping the future once it is ready, or as what it throws goes on. */
	run(): void {
		const future = this.#future;
		if (future === undefined) return;
		this.#queued = false;
		let ready: boolean;
		try {
			ready = future.poll(this) !== Pending;
		} catch (err) {
			this.#future = undefined;
			future[Symbol.dispose]();
			throw err;
		}
		if (!ready) return;
		this.#future = undefined;
		future[Symbol.dispose]();
	}

	wake(): void {
		if (this.#queued) return;
		this.#queued = true;
		this.schedule();
	}

	wakeByRef(): void {
		this.wake();
	}

	get data(): unknown {
		return this;
	}

	get vtable(): WakerVTable {
		return LOCAL_VTABLE;
	}

	clone(): Waker {
		return this;
	}

	[Symbol.dispose](): void {}
}

/** js-sys's `spawn_local`: run `future` to completion on js-sys's queue, which runs on microtasks. */
export function jsSpawnLocal(future: Future<undefined>): void {
	new LocalTask(future).schedule();
}
