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

/**
 * Bytes a call returns. `slice()` copies them into a `Uint8Array`, `at(i)` reads one, and
 * passed to a call that reads them or takes a `Bytes`, they cross with no copy. Past 64
 * bytes, they are Rust's own buffer, lent in place, which disposing frees, as `using` does,
 * or else the garbage collector; up to 64, a copy, which reads and disposes the same. A
 * read after disposing throws.
 */
export class Bytes {
	// A view of Rust's buffer, or of a copy, private so no caller's write reaches what Rust
	// reads.
	#view: Uint8Array | undefined;
	readonly #length: number;
	readonly #lent: boolean;

	private constructor(key: symbol, buffer: ArrayBuffer, lent: boolean) {
		if (key !== minting) throw new TypeError("only Rust makes bytes it returned");
		this.#view = new Uint8Array(buffer);
		this.#length = buffer.byteLength;
		this.#lent = lent;
	}

	/** How many bytes it holds. */
	get length(): number {
		return this.#length;
	}

	#open(): Uint8Array {
		if (this.#view === undefined) throw new Error("Bytes is closed");
		return this.#view;
	}

	/** The byte at `index`, from the end where it is negative, as `Uint8Array`'s `at`. */
	at(index: number): number | undefined {
		const view = this.#open();
		let i = Math.trunc(index) || 0;
		if (i < 0) i += this.#length;
		return i < 0 || i >= this.#length ? undefined : view[i];
	}

	/** A copy of the bytes from `start` up to `end`, as `Uint8Array`'s `slice`. */
	slice(start?: number, end?: number): Uint8Array<ArrayBuffer> {
		return this.#open().slice(start, end) as Uint8Array<ArrayBuffer>;
	}

	/** Each byte in turn, each read as `at` reads it. */
	*[Symbol.iterator](): IterableIterator<number> {
		for (let i = 0; i < this.#length; i++) yield this.at(i)!;
	}

	/** Free Rust's buffer, once no call views it; again, it does nothing. */
	[Symbol.dispose](): void {
		if (this.#view === undefined) return;
		this.#view = undefined;
		if (this.#lent) n.$bytesFree(this);
	}

	static {
		bytesMade = (buffer, lent) => new Bytes(minting, buffer, lent);
		bytesCopy = (b) => (#view in b && !b.#lent ? b.#open() : undefined);
	}
}

let bytesMade: (buffer: ArrayBuffer, lent: boolean) => Bytes;
let bytesCopy: (b: object) => Uint8Array | undefined;

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

/**
 * Install the JSI module into this runtime, from `globalThis[global]` if a host installed
 * it, or else through React Native's TurboModule `module`, and check that the library was
 * built with this binding and for one of its targets.
 */
export function load(m: Module): Promise<void> {
	loaded ??= (async () => {
		const g = globalThis as Record<string, unknown>;
		let installer = g[m.global] as Installer | undefined;
		installer ??= TurboModuleRegistry.getEnforcing<Installer & TurboModule>(m.module);
		const natives: Record<string, (...args: unknown[]) => unknown> = {};
		installer.install({ ...kit, natives });
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
	Handle: Handle.prototype,
	Waker: Wakes.prototype,
	Uint8Array,
	RangeError,
	TypeError,
	Error,
	bytes: (buffer: ArrayBuffer, lent: boolean) => bytesMade(buffer, lent),
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
