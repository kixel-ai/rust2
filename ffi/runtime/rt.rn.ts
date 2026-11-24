// The runtime every generated React Native binding imports, which rust2-ffi copies beside
// it: the web binding's types and integer helpers, then handles, bytes, and futures over
// the JSI module the app's library holds, which converts each value itself.

import { TurboModuleRegistry, type TurboModule } from "react-native";

// #shared

// The JSI module.

/** The JSI module's functions, by each C function's name without the prefix. */
// biome-ignore lint/suspicious/noExplicitAny: each generated module types what it calls.
export let n: Record<string, (...args: any[]) => any> = new Proxy(
	{},
	{
		get(): never {
			throw new Error("the binding is not loaded: await its `load()` first");
		},
	},
);

/** The index of the library's target among those the binding serves, once it is read. */
let target: number | undefined;

/** Throw unless the library's target is one of `targets`, as the item `what` is. */
export function on(targets: readonly number[], what: string): void {
	target ??= n.target() as number;
	if (!targets.includes(target)) {
		throw new Error(`${what}: the library's target lacks it`);
	}
}

/** Text as UTF-8, and back, through the module, as Hermes has no `TextDecoder`. */
const loose = { decode: (bytes: Uint8Array): string => n.$decode(bytes) as string };

/** `text`'s UTF-8, throwing where it holds a lone surrogate, which no Rust string does. */
export function encoded(text: string): Uint8Array {
	return n.$encode(text) as Uint8Array;
}

// Handles.

// Its brand, which a class field keyed by a symbol would not declare without a type checker.
export interface Handle<T extends string = string> {
	readonly [brand]: T;
}

/**
 * A handle to a Rust object, which the JSI module makes: disposing it drops the value, as
 * Rust's own drop does, and the garbage collector frees one never disposed.
 */
export class Handle<T extends string = string> {
	private constructor() {
		throw new TypeError("only Rust makes a handle");
	}

	/** Close it; again, it does nothing. */
	[Symbol.dispose](): void {
		n.$dispose(this);
	}
}

// Before the class, whose static initializer sets them as it is defined.
// Taken at load, before any caller's code can replace it.
const U8 = Uint8Array;
// `set`, which copies a typed array's bytes at once and runs no JavaScript, as `slice`'s
// species lookup would.
const setInto = Function.prototype.call.bind(U8.prototype.set) as (into: Uint8Array, from: Uint8Array) => void;

/** A `slice` bound: `v` as an integer, from the end of `length` where negative, or `missing`. */
function clamp(v: number | undefined, missing: number, length: number): number {
	if (v === undefined) return missing;
	const i = Math.trunc(+v) || 0;
	return i < 0 ? Math.max(length + i, 0) : Math.min(i, length);
}

let bytesMade: (buffer: ArrayBuffer | undefined, lent: boolean, length: number) => Bytes;
let bytesCopy: (b: object) => Uint8Array | undefined;

/**
 * Bytes a call returns. `slice()` copies them into a `Uint8Array`, `at(i)` reads one, and
 * passed to a call that reads them or takes a `Bytes`, they cross with no copy. Past 4 KiB,
 * they are Rust's own buffer, lent in place, which disposing frees, as `using` does, or else
 * the garbage collector; up to 4 KiB, a copy, which reads and disposes the same. A read
 * after disposing throws.
 */
export class Bytes {
	// A copy's view, which the module copies again as a call reads it, so a write to it,
	// which reflection can reach, touches no memory of Rust's. Rust's own buffer has none:
	// React Native's Babel preset makes private fields properties any caller can find, and a
	// view found would outlive the buffer's free, so the module reads it.
	#view: Uint8Array | undefined;
	#buffer: ArrayBuffer | undefined;
	readonly #length: number;
	readonly #lent: boolean;
	#closed = false;

	// Its length is the JSI module's, and its view `U8`'s, so no global or getter a caller
	// replaced runs as it is made.
	private constructor(key: symbol, buffer: ArrayBuffer | undefined, lent: boolean, length: number) {
		if (key !== Bytes.#minting) throw new TypeError("only Rust makes bytes it returned");
		this.#view = buffer === undefined ? undefined : new U8(buffer);
		this.#buffer = buffer;
		this.#length = length;
		this.#lent = lent;
	}

	/** How many bytes it holds. */
	get length(): number {
		return this.#length;
	}

	#open(): void {
		if (this.#closed) throw new Error("Bytes is closed");
	}

	/** The byte at `index`, from the end where it is negative, as `Uint8Array`'s `at`. */
	at(index: number): number | undefined {
		// Converted first, so a `valueOf` that disposes these bytes finds them closed.
		let i = Math.trunc(index) || 0;
		this.#open();
		if (i < 0) i += this.#length;
		if (i < 0 || i >= this.#length) return undefined;
		// Rust's buffer is checked open and bounded by the module itself.
		return this.#lent ? (n.$bytesAt(this, i) as number) : this.#view![i];
	}

	/** A copy of the bytes from `start` up to `end`, as `Uint8Array`'s `slice`. */
	slice(start?: number, end?: number): Uint8Array<ArrayBuffer> {
		// Converted first, as `at`'s index is.
		const from = clamp(start, 0, this.#length);
		let to = clamp(end, this.#length, this.#length);
		if (to < from) to = from;
		// Rust's buffer is checked open and copied in one native call, so no JavaScript a
		// caller replaced, such as a getter or `Math`'s, runs between and frees it.
		if (this.#lent) return n.$bytesSlice(this, from, to) as Uint8Array<ArrayBuffer>;
		// A copy, copied again by `U8` and `set` alone, which run no JavaScript, so no
		// caller's code sees the view.
		this.#open();
		const count = to - from;
		const out = new U8(count);
		setInto(out, new U8(this.#buffer!, from, count));
		return out;
	}

	/** Each byte in turn, each read as `at` reads it. */
	*[Symbol.iterator](): IterableIterator<number> {
		for (let i = 0; i < this.#length; i++) yield this.at(i)!;
	}

	/** Free Rust's buffer, once no call views it; again, it does nothing. */
	[Symbol.dispose](): void {
		if (this.#closed) return;
		this.#closed = true;
		this.#view = undefined;
		this.#buffer = undefined;
		if (this.#lent) n.$bytesFree(this);
	}

	// The key, whose initializer hands out the makers: not a static block, which React Native's
	// Babel preset refuses.
	static #minting = (() => {
		bytesMade = (buffer, lent, length) => new Bytes(minting, buffer, lent, length);
		bytesCopy = (b) => {
			if (!(#view in b) || b.#lent) return undefined;
			b.#open();
			return b.#view;
		};
		return minting;
	})();
}

/** A waker Rust returned, which the JSI module makes. */
class Wakes {
	private constructor() {
		throw new TypeError("only Rust makes a waker");
	}

	wake(): void {
		n.$wake(this);
	}

	[Symbol.dispose](): void {
		n.$wakerDrop(this);
	}
}

// Futures.

/** The calls pending in Rust, whose `cancel` a future given back to Rust calls once dropped. */
const calls = new WeakSet<Promise<unknown>>();

/** The `Call` of a future ready at its start, with `value`. */
function ready<T>(value: T): Call<T> {
	const promise = Promise.resolve(value) as Call<T>;
	promise.cancel = () => {};
	return promise;
}

/** The JSI module's: a `Call` that `cancel` cancels in Rust, and how it settles. */
function pending(cancel: () => void): [Call<unknown>, (v: unknown) => void, (e: unknown) => void] {
	let resolve!: (v: unknown) => void;
	let reject!: (e: unknown) => void;
	const promise = new Promise((res, rej) => {
		resolve = res;
		reject = rej;
	}) as Call<unknown>;
	promise.cancel = cancel;
	// A cancelled call rejects with `Cancelled`, which a caller that stopped waiting ignores.
	promise.catch(() => {});
	calls.add(promise);
	return [promise, resolve, reject];
}

// Loading.

/** What a binding's `load` passes: its global and TurboModule, and what it was built with. */
export type Module = {
	readonly global: string;
	readonly module: string;
	readonly file: string;
	readonly contract: bigint;
	readonly targets: readonly number[];
};

type Installer = { install(kit: object): boolean };

let loaded: Promise<void> | undefined;
/** The natives this runtime's module was installed with, once, which a load tried again reuses. */
let installed: Record<string, (...args: unknown[]) => unknown> | undefined;

/**
 * Install the JSI module into this runtime, from `globalThis[global]` if a host installed
 * it, or else through React Native's TurboModule `module`, and check that the library was
 * built with this binding and for one of its targets.
 */
export function load(m: Module): Promise<void> {
	loaded ??= (async () => {
		let natives = installed;
		if (natives === undefined) {
			const g = globalThis as Record<string, unknown>;
			let installer = g[m.global] as Installer | undefined;
			installer ??= TurboModuleRegistry.getEnforcing<Installer & TurboModule>(m.module);
			natives = {};
			installer.install({ ...kit, natives });
			installed = natives;
		}
		const contract = natives.contract() as bigint;
		if (contract !== m.contract) {
			throw new Error(
				`${m.file}: the library's contract is 0x${contract.toString(16)}, but this binding's is 0x${m.contract.toString(16)}: rebuild one of them`,
			);
		}
		const target = natives.target() as number;
		if (!m.targets.includes(target)) {
			throw new Error(`${m.file}: the library's target, ${target}, is none this binding serves`);
		}
		n = natives;
	})().catch((e: unknown) => {
		// A load that failed leaves the next free to try again.
		loaded = undefined;
		throw e;
	});
	return loaded;
}

/** What the JSI module calls of this runtime. */
const kit = {
	None,
	// A typed array's kind as the engine holds it, or undefined for any other value.
	typedTag: Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), Symbol.toStringTag)!.get!,
	Handle: Handle.prototype,
	Waker: Wakes.prototype,
	Uint8Array,
	// Each typed array a call returns a copy as, taken at load rather than from the global a
	// caller may replace.
	typedArrays: {
		Uint8Array,
		Int8Array,
		Uint16Array,
		Int16Array,
		Uint32Array,
		Int32Array,
		BigUint64Array,
		BigInt64Array,
		Float32Array,
		Float64Array,
	},
	RangeError,
	TypeError,
	Error,
	bytes: (buffer: ArrayBuffer | undefined, lent: boolean, length: number) => bytesMade(buffer, lent, length),
	bytesCopy: (v: unknown) => (typeof v === "object" && v !== null ? bytesCopy(v) : undefined),
	opaqueMade,
	opaqueBytes: (v: unknown) => {
		const bytes = typeof v === "object" && v !== null ? bytesOf(v as Opaque) : undefined;
		if (bytes === undefined) throw new RangeError("a value Rust made is no other's");
		return bytes;
	},
	int,
	char,
	u64Halves,
	i64Halves,
	u128Words,
	f32Bits,
	f64Bits,
	pending,
	ready,
	cancelled: () => new Cancelled(),
	isCancelled: (e: unknown) => e instanceof Cancelled,
	cancelCall: (p: unknown) => {
		if (calls.has(p as Promise<unknown>)) (p as Call<unknown>).cancel();
	},
};

/** A `u128`'s 32-bit words, low first: one out of range throws. */
function u128Words(v: u128): number[] {
	if (typeof v === "number") {
		if (!Number.isSafeInteger(v) || v < 0) throw new RangeError(`${v} is no u128`);
		return [v % TWO32, Math.floor(v / TWO32), 0, 0];
	}
	const words = [v.lo, v.w1, v.w2, v.hi];
	const one = words.every((x) => x >>> 0 === x) && (v.hi !== 0 || v.w2 !== 0 || v.w1 >= 0x200000);
	if (!one) throw new RangeError(`${JSON.stringify(v)} is no u128 in its one form`);
	return words;
}

/** An `f32`'s bits that no number the JSI module took holds: a NaN's, or one that throws. */
function f32Bits(v: f32): number {
	if (typeof v === "number") {
		if (v !== v) return F32_NAN;
		throw new RangeError(`${v} is no f32`);
	}
	const bits = v.bits;
	if (bits >>> 0 !== bits || (bits & 0x7fffffff) <= 0x7f800000 || bits === F32_NAN) {
		throw new RangeError(`${JSON.stringify(v)} is no f32 NaN in its one form`);
	}
	return bits;
}

/** An `f64` NaN's bits, `[hi, lo]`: `NaN` as `f64::NAN`, or its own, or one that throws. */
function f64Bits(v: f64): [number, number] {
	if (typeof v === "number") return [F64_NAN_HI, 0];
	const { hi, lo } = v;
	const high = hi & 0x7fffffff;
	const nan = high > 0x7ff00000 || (high === 0x7ff00000 && lo !== 0);
	if (hi >>> 0 !== hi || lo >>> 0 !== lo || !nan || (hi === F64_NAN_HI && lo === 0)) {
		throw new RangeError(`${JSON.stringify(v)} is no f64 NaN in its one form`);
	}
	return [hi, lo];
}
