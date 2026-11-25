// The runtime every generated web binding imports, which rust2-ffi copies beside it: the
// module and its memory, the stack a call lowers its values into, handles, Rust's integers
// past 32 bits, futures as promises, what JavaScript implements for Rust, and the web APIs
// that stand in for what the module's std imports.

// #region shared
// `Symbol.dispose` where the engine lacks it, as stable Safari does: the registry's symbol,
// which esbuild's `using` helper falls back to too, though TypeScript's throws without one.
// It runs before any class below keys a method on it.
if (!Symbol.dispose) Object.defineProperty(Symbol, "dispose", { value: Symbol.for("Symbol.dispose") });

declare const brand: unique symbol;

/** A `u64` past 2^53: its high and low 32 bits, each a uint32. */
export type U64Limbs = { readonly hi: number; readonly lo: number; readonly [brand]: "u64" };
/** An `i64` past 2^53 in magnitude: its high 32 bits as an int32, and its low as a uint32. */
export type I64Limbs = { readonly hi: number; readonly lo: number; readonly [brand]: "i64" };
/** A `u128` past 2^53: four 32-bit limbs, `hi`, `w2`, `w1`, and `lo`, each a uint32. */
export type U128Limbs = {
	readonly hi: number;
	readonly w2: number;
	readonly w1: number;
	readonly lo: number;
	readonly [brand]: "u128";
};

/**
 * A number of the Rust integer type `T`: its brand is type-only, so a plain number, such
 * as a literal, passes for any.
 */
type Num<T extends string> = number & { readonly [brand]?: T };

/**
 * Rust's `u64`: a number while it is a safe integer, at most 2^53 - 1, and its limbs past
 * that. Each value has one form, so two compare with `===` only as numbers.
 */
export type u64 = Num<"u64"> | U64Limbs;
/** Rust's `i64`: a number while it is a safe integer, and its limbs past that. */
export type i64 = Num<"i64"> | I64Limbs;
/** Rust's `u128`: a number while it is a safe integer, and its limbs past that. */
export type u128 = Num<"u128"> | U128Limbs;

/** An `f32` NaN other than `NaN`, which stands for Rust's `f32::NAN`: its bits, a uint32. */
export type F32Nan = { readonly bits: number; readonly [brand]: "f32" };
/** An `f64` NaN other than `NaN`, which stands for Rust's `f64::NAN`: its high and low 32
 * bits, each a uint32. */
export type F64Nan = { readonly hi: number; readonly lo: number; readonly [brand]: "f64" };

/**
 * Rust's `f32`: a number, `NaN` among them as Rust's `f32::NAN`, and the bits of any other
 * NaN, which no number keeps: an engine may quiet or canonicalize a NaN it converts. Each
 * value has one form.
 */
export type f32 = Num<"f32"> | F32Nan;
/** Rust's `f64`: a number, `NaN` as `f64::NAN`, and the bits of any other NaN. */
export type f64 = Num<"f64"> | F64Nan;

/** Rust's `Result`: `{ ok: true, value }` or `{ ok: false, error }`. */
export type Result<T, E> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly error: E };

/** An `Ok`. */
export function Ok<T>(value: T): Result<T, never> {
	return { ok: true, value };
}

/** An `Err`. */
export function Err<E>(error: E): Result<never, E> {
	return { ok: false, error };
}

/** The `None` of an `Option` whose `Some` may be `undefined`, as `Option<()>`'s is. */
export const None: unique symbol = Symbol("None");
export type None = typeof None;

/** A place a call writes back what Rust left a `&mut` value as, in `v`. */
export type Place<T> = { v: T };

// The key only `opaqueMade` holds, and the private field's accessors.
const minting: unique symbol = Symbol("minting");
let made: (bytes: Uint8Array) => Opaque;
let bytesOf: (v: Opaque) => Uint8Array | undefined;

// Its brand, which a class field keyed by a symbol would not declare without a type checker.
export interface Opaque<T extends string = string> {
	readonly [brand]: T;
}

/**
 * A `Copy` type whose fields are private: its bytes, which only Rust makes or reads, so
 * JavaScript copies one but never makes one.
 */
export class Opaque<T extends string = string> {
	// Private, so no caller makes or changes one: the bytes may hold what Rust forbids, such
	// as a zero `NonZero`.
	readonly #bytes: Uint8Array;
	private constructor(bytes: Uint8Array, key: symbol) {
		if (key !== Opaque.#minting) throw new TypeError("only Rust makes an opaque value");
		this.#bytes = bytes;
	}
	// The key, whose initializer hands out the makers: not a static block, which React Native's
	// Babel preset refuses.
	static #minting = (() => {
		made = (bytes) => new Opaque(bytes, minting);
		bytesOf = (v) => (#bytes in v ? v.#bytes : undefined);
		return minting;
	})();
}

/** The generated glue's only: the opaque value of bytes Rust wrote. */
export function opaqueMade(bytes: Uint8Array): Opaque {
	return made(bytes);
}

/** Rust's `Duration`: whole seconds, and the nanoseconds below one. */
export type Duration = { readonly secs: u64; readonly nanos: number };

/** Rust's `PathBuf`: the path's bytes as the OS holds them, which may not be UTF-8. */
export type PathBuf = { readonly bytes: Uint8Array };

/** The path of `text`, as UTF-8: a lone surrogate throws, as no Rust string holds one. */
export function pathFrom(text: string): PathBuf {
	return { bytes: encoded(text) };
}

/** A path's text: what is not UTF-8 is replaced, as Rust's `Path::display` shows it. */
export function pathText(path: PathBuf): string {
	return loose.decode(path.bytes);
}

/** Rust's `Ipv4Addr`: its 4 bytes in network order. */
export type Ipv4Addr = { readonly octets: Uint8Array };
/** Rust's `Ipv6Addr`: its 16 bytes in network order. */
export type Ipv6Addr = { readonly octets: Uint8Array };
/** Rust's `IpAddr`. */
export type IpAddr = { readonly tag: "V4"; readonly _0: Ipv4Addr } | { readonly tag: "V6"; readonly _0: Ipv6Addr };
/** Rust's `SocketAddrV4`. */
export type SocketAddrV4 = { readonly ip: Ipv4Addr; readonly port: number };
/** Rust's `SocketAddrV6`: with its flow label and scope. */
export type SocketAddrV6 = {
	readonly ip: Ipv6Addr;
	readonly port: number;
	readonly flowinfo: number;
	readonly scopeId: number;
};
/** Rust's `SocketAddr`. */
export type SocketAddr =
	| { readonly tag: "V4"; readonly _0: SocketAddrV4 }
	| { readonly tag: "V6"; readonly _0: SocketAddrV6 };

/** Where a reader's seek counts from, as Rust's `SeekFrom` does. */
export type SeekFrom = "Start" | "Current" | "End";

/**
 * Bytes Rust reads and seeks, as its `Read + Seek`, lent for the call it is passed to.
 * What either method throws reaches Rust as an `io::Error` of its message.
 */
export interface Reader {
	/** Read up to `buffer.length` bytes into `buffer`, returning how many: 0 only at the end. */
	read(buffer: Uint8Array): number;
	/**
	 * Move to `offset` from `from`, returning the position from the start. From `"Start"`,
	 * `offset` is a `u64`, else an `i64`.
	 */
	seek(offset: u64 | i64, from: SeekFrom): u64;
}

/** The value of a call into Rust's async API: a promise of its result, which `cancel` cancels. */
export type Call<T> = Promise<T> & {
	/** Cancel it in Rust unless it finished, losing nothing: it then rejects with `Cancelled`. */
	cancel(): void;
};

/** What a cancelled `Call` rejects with. */
export class Cancelled extends Error {
	constructor() {
		super("Cancelled");
		this.name = "Cancelled";
	}
}

/** A waker Rust returned: `wake` wakes its task through Rust, and disposing it drops it. */
export type Waker = { wake(): void; [Symbol.dispose](): void };
// #endregion shared

/**
 * Copy an opaque value's bytes into memory at `p`, throwing unless it is one Rust made, of
 * `n` bytes. It returns nothing, so the bytes never reach a caller to change.
 */
export function opaquePut(v: Opaque, n: number, p: number): void {
	const bytes = bytesOf(v);
	if (bytes === undefined || bytes.length !== n) throw new RangeError("a value Rust made is no other's");
	mem().set(bytes, p);
}

/** The module's exports, which generated code calls by their C names. */
type Exports = Record<string, any>;

/** The module's exports, once `load` instantiated it. */
export let w: Exports = new Proxy(
	{},
	{
		get(_, name) {
			throw new Error(`${String(name)}: the module is not loaded: call load first`);
		},
	},
);

// #region web memory
let memory: WebAssembly.Memory;
let view = new DataView(new ArrayBuffer(0));
let bytes: Uint8Array<ArrayBuffer> = new Uint8Array(0);

/** A view of the module's memory, made again once memory grows, which detaches the last. */
export function dv(): DataView {
	// A detached view's length reads 0, a third the cost of reading `memory.buffer`, which
	// every value a call reads or writes through memory pays.
	if (bytes.byteLength === 0) {
		// Never shared: the module is built without threads. Deno's types say it may be.
		const buffer = memory.buffer as ArrayBuffer;
		view = new DataView(buffer);
		bytes = new Uint8Array(buffer);
	}
	return view;
}

/** The module's memory as bytes, as [dv] keeps it. */
export function mem(): Uint8Array<ArrayBuffer> {
	dv();
	return bytes;
}

/** An address in Rust's memory: in the module's, a number. */
export type Addr = number;
/** The null address. */
const NULL: Addr = 0;
/** A pointer's size, and so a word's. */
const WORD = 4;

/** The word at `p`, as a length. */
function lenGet(p: number): number {
	return dv().getUint32(p, true);
}

/** Write the word `n`, a length or a token, at `p`. */
function lenPut(p: number, n: number): void {
	dv().setUint32(p, n, true);
}

/** The address at `p`. */
function addrGet(p: number): Addr {
	return dv().getUint32(p, true);
}

/** Write the address `a` at `p`. */
function addrPut(p: number, a: Addr): void {
	dv().setUint32(p, a, true);
}

/** Write at `p` the address of `at`, a place the stack took. */
function placePut(p: number, at: number): void {
	dv().setUint32(p, at, true);
}

/** A copy of the `len` bytes at `ptr`. */
function lentCopy(ptr: Addr, len: number): Uint8Array<ArrayBuffer> {
	return mem().slice(ptr, ptr + len);
}

/** The text of the `len` bytes of UTF-8 at `ptr`, as `decoder` decodes it. */
function lentText(ptr: Addr, len: number, decoder: TextDecoder): string {
	return decoder.decode(mem().subarray(ptr, ptr + len));
}

/** The byte at `ptr + i`. */
function lentByte(ptr: Addr, i: number): number {
	return mem()[ptr + i]!;
}

/** Write `data` at `ptr`. */
function lentWrite(ptr: Addr, data: Uint8Array): void {
	mem().set(data, ptr);
}
// #endregion web memory

/**
 * The state calls share: how many calls into the module are running less how many calls
 * out of it, which each call into it from JavaScript finds at 0 unless a call trapped,
 * when a call in did not return.
 */
export const s = { d: 0 };

/** Why the module aborted, after which every call throws: nothing of Rust's state is sound. */
let aborted: unknown;

// #region web live
/**
 * Whether Rust may run: no call aborted, nor trapped, which unwinds past the `s.d--` after
 * it and leaves `s.d` raised wherever JavaScript runs next.
 */
function live(): boolean {
	if (s.d !== 0) aborted ??= new Error("a call into the module trapped");
	return aborted === undefined;
}
// #endregion web live

/** The error of a call into a module that aborted. */
function dead(): Error {
	return new Error("the module aborted, so no call into it runs", { cause: aborted });
}

/** Abort the module with `message`: Rust's own abort, which prints it and traps. */
export function abort(message: string): never {
	if (aborted === undefined) {
		const m = top;
		const max = message.length * 3;
		const p = alloc(max, 1);
		const { written } = encoder.encodeInto(message, mem().subarray(p, p + max));
		try {
			w[`${space}_abort`](p, written);
		} catch (e) {
			aborted = e;
		}
		top = m;
	}
	aborted ??= new Error(message);
	throw dead();
}

// The stack: memory each call lowers its values into, taking what it needs from the top and
// giving it back once it returns, so a call Rust makes into JavaScript meanwhile takes and
// gives back above it.
// #region web stack
const STACK = 1 << 16;
let base = 0;
let top = 0;
// #endregion web stack
/** What did not fit, each its pointer, size, and alignment, freed as the stack falls back. */
const big: number[] = [];
/** The handles, and lent bytes, each running call holds, released once it returns. */
const held: (Handle | Bytes)[] = [];
/** The tokens of what each running call lends Rust, forgotten once it returns. */
const lent: number[] = [];
/** Where each running call found the stack: its top, then the length of each list. */
const frames: number[] = [];

/** Start a call: what [leave] takes back to. */
export function enter(): number {
	if (!live()) throw dead();
	frames.push(top, big.length, held.length, lent.length);
	return frames.length;
}

/** End the call [enter] started, giving back what it took. */
export function leave(mark: number): void {
	frames.length = mark;
	const l = frames.pop()!;
	const h = frames.pop()!;
	const b = frames.pop()!;
	top = frames.pop()!;
	if (lent.length > l) {
		while (lent.length > l) tokens.delete(lent.pop()!);
		relayed();
	}
	// After a trap, Rust's state is unsound: forget what the call took, calling none of it.
	if (!live()) {
		held.length = h;
		big.length = b;
		return;
	}
	while (held.length > h) {
		const x = held.pop()!;
		if (x instanceof Handle) x.release();
		else bytesRelease(x);
	}
	while (big.length > b) {
		const align = big.pop()!;
		const size = big.pop()!;
		w[`${space}_free`](big.pop()!, size, align);
	}
}

// #region web alloc
/** `size` bytes aligned to `align`, a power of two, until the running call returns. */
export function alloc(size: number, align: number): number {
	const at = (top + align - 1) & -align;
	if (at + size <= STACK) {
		top = at + size;
		return base + at;
	}
	const p = w[`${space}_alloc`](size, align) as number;
	big.push(p, size, align);
	return p;
}
// #endregion web alloc

/** Throw unless the call `what` wrote no status at `p`: a run-time check failed. */
export function status(p: number, what: string): void {
	const n = dv().getInt32(p, true);
	if (n !== 0) throw new Error(`${what}: ${STATUSES[n - 1] ?? "an unknown status"}`);
}

const STATUSES = [
	"a handle's value was moved out",
	"a handle is borrowed exclusively",
	"a handle is already borrowed",
	"a handle is borrowed on another thread",
	"a handle a call lent is borrowed exclusively or moved out",
	"a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed",
	"a value that is not Send is used on another thread than the one that made it",
];

/**
 * How many of the latest owned handles and lent bytes returned the nursery holds: one
 * disposed while there is freed without the garbage collector's registry, which costs a
 * return more than the rest of it and its disposal together. One that leaves it open,
 * pushed out by later returns or by [flush], registers then, so this bounds what a
 * dropped one keeps of Rust's.
 */
const NURSERY = 64;

/** The nursery, oldest at [next] once full. Each knows its slot, so disposing clears it. */
const nursery: (Handle | Bytes | undefined)[] = new Array(NURSERY).fill(undefined);
let next = 0;

/** A slot of what is in the nursery is its index; of what left it, one of these. */
const REGISTERED = -1;
const UNREGISTERED = -2;

/** Put `x` in the nursery, pushing out the oldest: its slot. */
function nurse(x: Handle | Bytes): number {
	const i = next;
	next = (i + 1) & (NURSERY - 1);
	const old = nursery[i];
	if (old !== undefined) evict(old);
	nursery[i] = x;
	return i;
}

/** Push `x` out of the nursery, registering it unless disposed. */
function evict(x: Handle | Bytes): void {
	if (x instanceof Handle) x.evict();
	else bytesEvict(x);
}

/**
 * Push out all the nursery holds, once a collection ran: so an idle nursery keeps what it
 * holds for two collections, the one that ran this and the one that finds them, not
 * forever. A collected sentinel runs it, as a `WeakRef` to the nursery would only lose it.
 */
function flush(): void {
	for (let i = 0; i < NURSERY; i++) {
		const x = nursery[i];
		if (x !== undefined) evict(x);
		nursery[i] = undefined;
	}
}

const collections = new FinalizationRegistry<undefined>(() => {
	flush();
	watch();
});

/** Register a sentinel, which nothing reaches, so the next collection runs [flush]. */
function watch(): void {
	collections.register({}, undefined);
}

watch();

// Its brand, which a class field keyed by a symbol would not declare without a type checker.
export interface Handle<T extends string = string> {
	readonly [brand]: T;
}

// #region web handles
/** What each object type's handles do: its name, its free, and its drop, if Rust checks it. */
export type Kind = {
	readonly name: string;
	readonly free: string;
	readonly drop?: string;
};

/**
 * Whether a handle Rust lends is freed too, through [FORGET]: not a handle of the module,
 * which is its address.
 */
const FORGETS = false;
/** What frees a lent handle, where [FORGETS]. */
const FORGET = "";

/** The handles' frees the garbage collector makes, of each that left the nursery undisposed. */
const collected = new FinalizationRegistry<[number, string]>(([p, free]) => {
	if (!live()) return;
	s.d++;
	w[free](p);
	s.d--;
});

/**
 * A handle to a Rust object, which counts the calls holding it. An owned one of a value
 * Rust checks is Rust's to account: disposing it drops the value as Rust's own drop does,
 * failing while a call borrows it, so a call after it throws. Any other throws if disposed
 * while a call holds it. Either is freed once disposed and no call holds it, since a call
 * still lowering its arguments has passed its pointer, or once the garbage collector finds
 * it unreachable, if it was never disposed.
 */
export class Handle<T extends string = string> {
	/** The handle's pointer. */
	readonly p: number;
	readonly k: Kind;
	/** The owner's hold until disposed, and one for each call. */
	h = 1;
	closed = false;
	readonly owned: boolean;
	/** Whether disposing it drops its value through Rust's `_drop`. */
	readonly checked: boolean;
	/** An owned one's slot in the nursery, or whether it registered once it left. */
	n: number;

	constructor(p: number, k: Kind, owned: boolean) {
		this.p = p;
		this.k = k;
		this.owned = owned;
		this.checked = owned && k.drop !== undefined;
		this.n = owned || FORGETS ? nurse(this) : UNREGISTERED;
	}

	/** Leave the nursery, registering unless disposed, as its last hold then frees it. */
	evict(): void {
		if (this.closed) {
			this.n = UNREGISTERED;
			return;
		}
		collected.register(this, [this.p, this.owned ? this.k.free : FORGET], this);
		this.n = REGISTERED;
	}

	hold(): number {
		if (this.closed) throw new Error(`${this.k.name} is closed`);
		this.h++;
		return this.p;
	}

	release(): void {
		if (--this.h === 0 && (this.owned || FORGETS)) this.free();
	}

	free(): void {
		if (this.n >= 0) nursery[this.n] = undefined;
		else if (this.n === REGISTERED) collected.unregister(this);
		const m = enter();
		s.d++;
		w[this.owned ? this.k.free : FORGET](this.p);
		s.d--;
		leave(m);
	}

	/** Close it as `Handle` says; again, it does nothing. */
	[Symbol.dispose](): void {
		if (this.closed) return;
		if (this.checked) {
			const m = enter();
			try {
				const st = alloc(4, 4);
				dv().setInt32(st, 0, true);
				s.d++;
				w[this.k.drop!](this.p, st);
				s.d--;
				status(st, this.k.drop!);
			} finally {
				leave(m);
			}
			// Dropped, so a call that still holds it fails Rust's check, and none can hold it
			// again: it is freed with its last hold, rather than by the garbage collector, whose
			// registry costs a returned object ten times its call once it holds many.
		} else if (this.h !== 1) {
			throw new Error(`${this.k.name}: a handle is already borrowed`);
		}
		this.closed = true;
		this.release();
	}
}

/** `handle`'s pointer, held until the running call returns: a closed one throws. */
export function hold(handle: Handle): number {
	const p = handle.hold();
	held.push(handle);
	return p;
}

/** [hold] of a handle, or 0 for none. */
export function holdOr(handle: Handle | undefined): number {
	return handle === undefined ? 0 : hold(handle);
}

// #endregion web handles

/** A handle Rust lends for a call, closed after it. */
export function lentHandle<T extends string>(p: number, k: Kind): Handle<T> {
	return new Handle<T>(p, k, false);
}

// A write the generated code makes of a value it reads: the value is an argument, so a getter
// it runs, which may grow memory, runs before the view it is written through is taken.

/** Write the low byte of `v` at `p`, of either sign. */
export function put8(p: number, v: number): void {
	dv().setUint8(p, v);
}

/** Write the low 16 bits of `v` at `p`, of either sign. */
export function put16(p: number, v: number): void {
	dv().setUint16(p, v, true);
}

/** Write the low 32 bits of `v` at `p`, of either sign. */
export function put32(p: number, v: number): void {
	dv().setUint32(p, v, true);
}

// #region shared
// Integers.

const TWO32 = 4294967296;
/** The bits of Rust's `f32::NAN`, and the high bits of its `f64::NAN`, whose low are 0. */
const F32_NAN = 0x7fc00000;
const F64_NAN_HI = 0x7ff80000;

/** `v`, unless it is no integer from `min` through `max`, which throws naming `what`. */
export function int(v: number, min: number, max: number, what: string): number {
	if (Number.isInteger(v) && v >= min && v <= max) return v;
	throw new RangeError(`${v} is no ${what}`);
}

/** A `NonZero`'s integer, which throws if zero. */
export function nonZero<T extends number | u64 | i64>(v: T): T {
	if (v === 0) throw new RangeError("a NonZero is zero");
	return v;
}

/** An `f32` as a number: a NaN of its own bits as `NaN`, which loses them. */
export function f32Number(v: f32): number {
	return typeof v === "number" ? v : NaN;
}

/** An `f64` as a number: a NaN of its own bits as `NaN`, which loses them. */
export function f64Number(v: f64): number {
	return typeof v === "number" ? v : NaN;
}

/** A `char`: a Unicode scalar value. */
export function char(v: number): number {
	if (Number.isInteger(v) && v >= 0 && v <= 0x10ffff && (v < 0xd800 || v > 0xdfff)) return v;
	throw new RangeError(`${v} is no char`);
}

/** The `u64` of its halves, each a uint32. */
export function u64Of(hi: number, lo: number): u64 {
	return hi < 0x200000 ? hi * TWO32 + lo : ({ hi, lo } as U64Limbs);
}

/** The `i64` of its halves, `hi` an int32 and `lo` a uint32. */
export function i64Of(hi: number, lo: number): i64 {
	if (hi >= -0x200000 && hi < 0x200000) {
		const v = hi * TWO32 + lo;
		if (Number.isSafeInteger(v)) return v;
	}
	return { hi, lo } as I64Limbs;
}

/** A `u64`'s halves, `[hi, lo]`, each a uint32: one out of range throws. */
export function u64Halves(v: u64): [number, number] {
	if (typeof v === "number") {
		if (!Number.isSafeInteger(v) || v < 0) throw new RangeError(`${v} is no u64`);
		return [Math.floor(v / TWO32), v % TWO32];
	}
	// Each read once, so a getter's value is the one checked.
	const { hi, lo } = v;
	if (!(hi >= 0x200000 && hi <= 0xffffffff && Number.isInteger(hi) && lo >>> 0 === lo)) {
		throw new RangeError(`${JSON.stringify(v)} is no u64 in its one form`);
	}
	return [hi, lo];
}

/** An `i64`'s halves, `[hi, lo]`, `hi` an int32 and `lo` a uint32: one out of range throws. */
export function i64Halves(v: i64): [number, number] {
	if (typeof v === "number") {
		if (!Number.isSafeInteger(v)) throw new RangeError(`${v} is no i64`);
		const hi = Math.floor(v / TWO32);
		return [hi | 0, v - hi * TWO32];
	}
	const { hi, lo } = v;
	const safe = hi >= -0x200000 && hi < 0x200000 && Number.isSafeInteger(hi * TWO32 + lo);
	if (safe || hi !== (hi | 0) || lo >>> 0 !== lo) {
		throw new RangeError(`${JSON.stringify(v)} is no i64 in its one form`);
	}
	return [hi, lo];
}

/** A `u64` as the `bigint` a wasm `i64` takes. */
export function u64Big(v: u64): bigint {
	if (typeof v === "number" && Number.isSafeInteger(v) && v >= 0) return BigInt(v);
	const [hi, lo] = u64Halves(v);
	return BigInt.asIntN(64, (BigInt(hi) << 32n) | BigInt(lo));
}

/** An `i64` as the `bigint` a wasm `i64` takes. */
export function i64Big(v: i64): bigint {
	if (typeof v === "number" && Number.isSafeInteger(v)) return BigInt(v);
	const [hi, lo] = i64Halves(v);
	return (BigInt(hi) << 32n) | BigInt(lo);
}

/** The `u64` of what a wasm `i64` returned. */
export function u64OfBig(v: bigint): u64 {
	const u = BigInt.asUintN(64, v);
	return u <= 0x1fffffffffffffn ? Number(u) : u64Of(Number(u >> 32n), Number(u & 0xffffffffn));
}

/** The `i64` of what a wasm `i64` returned. */
export function i64OfBig(v: bigint): i64 {
	return v >= -0x1fffffffffffffn && v <= 0x1fffffffffffffn
		? Number(v)
		: i64Of(Number(v >> 32n), Number(BigInt.asUintN(32, v)));
}

/** A `u64` as a `bigint`, for printing or arithmetic. */
export function u64ToBig(v: u64): bigint {
	return BigInt.asUintN(64, u64Big(v));
}

/** An `i64` as a `bigint`. */
export function i64ToBig(v: i64): bigint {
	return i64Big(v);
}

/** A `u128` as a `bigint`. */
export function u128ToBig(v: u128): bigint {
	if (typeof v === "number") return BigInt(v);
	return (BigInt(v.hi) << 96n) | (BigInt(v.w2) << 64n) | (BigInt(v.w1) << 32n) | BigInt(v.lo);
}

/** The `u64` of a `bigint`, which throws outside its range. */
export function u64FromBig(v: bigint): u64 {
	if (v < 0n || v > 0xffffffffffffffffn) throw new RangeError(`${v} is no u64`);
	return u64OfBig(v);
}

/** The `i64` of a `bigint`, which throws outside its range. */
export function i64FromBig(v: bigint): i64 {
	if (v < -0x8000000000000000n || v > 0x7fffffffffffffffn) throw new RangeError(`${v} is no i64`);
	return i64OfBig(v);
}

/** The `u128` of a `bigint`, which throws outside its range. */
export function u128FromBig(v: bigint): u128 {
	if (v < 0n || v >> 128n !== 0n) throw new RangeError(`${v} is no u128`);
	if (v <= 0x1fffffffffffffn) return Number(v);
	const word = (n: bigint) => Number(BigInt.asUintN(32, v >> n));
	return { hi: word(96n), w2: word(64n), w1: word(32n), lo: word(0n) } as U128Limbs;
}
// #endregion shared

/** The `f32` at `p`: a NaN by its bits, which no number read through a float keeps. */
export function f32Get(p: number): f32 {
	const d = dv();
	const bits = d.getUint32(p, true);
	if ((bits & 0x7fffffff) <= 0x7f800000) return d.getFloat32(p, true);
	return bits === F32_NAN ? NaN : ({ bits } as F32Nan);
}

/** Write the `f32` `v` at `p`: a number a single holds exactly, `NaN` as `f32::NAN`, or a
 * NaN's bits. Any other throws. */
export function f32Put(p: number, v: f32): void {
	if (typeof v === "number") {
		const d = dv();
		if (v !== v) d.setUint32(p, F32_NAN, true);
		else if (Math.fround(v) === v) d.setFloat32(p, v, true);
		else throw new RangeError(`${v} is no f32`);
		return;
	}
	const bits = v.bits;
	if (bits >>> 0 !== bits || (bits & 0x7fffffff) <= 0x7f800000 || bits === F32_NAN) {
		throw new RangeError(`${JSON.stringify(v)} is no f32 NaN in its one form`);
	}
	// Taken after the payload's getters, which may run code that grows memory.
	dv().setUint32(p, bits, true);
}

/** The `f64` at `p`: a NaN by its bits. */
export function f64Get(p: number): f64 {
	const d = dv();
	const hi = d.getUint32(p + 4, true);
	const lo = d.getUint32(p, true);
	const high = hi & 0x7fffffff;
	if (high < 0x7ff00000 || (high === 0x7ff00000 && lo === 0)) return d.getFloat64(p, true);
	return hi === F64_NAN_HI && lo === 0 ? NaN : ({ hi, lo } as F64Nan);
}

/** Write the `f64` `v` at `p`: a number, `NaN` as `f64::NAN`, or a NaN's bits. */
export function f64Put(p: number, v: f64): void {
	if (typeof v === "number") {
		const d = dv();
		if (v === v) d.setFloat64(p, v, true);
		else {
			d.setUint32(p, 0, true);
			d.setUint32(p + 4, F64_NAN_HI, true);
		}
		return;
	}
	const { hi, lo } = v;
	const high = hi & 0x7fffffff;
	const nan = high > 0x7ff00000 || (high === 0x7ff00000 && lo !== 0);
	if (hi >>> 0 !== hi || lo >>> 0 !== lo || !nan || (hi === F64_NAN_HI && lo === 0)) {
		throw new RangeError(`${JSON.stringify(v)} is no f64 NaN in its one form`);
	}
	const d = dv();
	d.setUint32(p, lo, true);
	d.setUint32(p + 4, hi, true);
}

/** The `u64` at `p`. */
export function u64Get(p: number): u64 {
	const d = dv();
	return u64Of(d.getUint32(p + 4, true), d.getUint32(p, true));
}

/** The `i64` at `p`. */
export function i64Get(p: number): i64 {
	const d = dv();
	return i64Of(d.getInt32(p + 4, true), d.getUint32(p, true));
}

/** Write the `u64` `v` at `p`. */
export function u64Put(p: number, v: u64): void {
	const [hi, lo] = u64Halves(v);
	const d = dv();
	d.setUint32(p, lo, true);
	d.setUint32(p + 4, hi, true);
}

/** Write the `i64` `v` at `p`. */
export function i64Put(p: number, v: i64): void {
	const [hi, lo] = i64Halves(v);
	const d = dv();
	d.setUint32(p, lo, true);
	d.setInt32(p + 4, hi, true);
}

/** The `u128` at `p`, as its low then high 64 bits. */
export function u128Get(p: number): u128 {
	const d = dv();
	const lo = d.getUint32(p, true);
	const w1 = d.getUint32(p + 4, true);
	const w2 = d.getUint32(p + 8, true);
	const hi = d.getUint32(p + 12, true);
	if (hi === 0 && w2 === 0 && w1 < 0x200000) return w1 * TWO32 + lo;
	return { hi, w2, w1, lo } as U128Limbs;
}

/** Write the `u128` `v` at `p`. */
export function u128Put(p: number, v: u128): void {
	let words: number[];
	if (typeof v === "number") {
		if (!Number.isSafeInteger(v) || v < 0) throw new RangeError(`${v} is no u128`);
		words = [v % TWO32, Math.floor(v / TWO32), 0, 0];
	} else {
		words = [v.lo, v.w1, v.w2, v.hi];
		const [, w1, w2, hi] = words;
		const one = words.every((x) => x >>> 0 === x) && (hi !== 0 || w2 !== 0 || w1! >= 0x200000);
		if (!one) throw new RangeError(`${JSON.stringify(v)} is no u128 in its one form`);
	}
	const d = dv();
	for (let i = 0; i < 4; i++) d.setUint32(p + 4 * i, words[i], true);
}

// Typed arrays, read as the engine keeps them.

// `%TypedArray%.prototype`'s own getters, taken at load and called through no method a
// caller can replace: no subclass's, caller's, or Proxy's `length`, `byteOffset`, `buffer`,
// or tag reaches them, and each but the tag's throws a TypeError on any other value.
const typedArray = Object.getPrototypeOf(Uint8Array.prototype) as object;
function getter<T>(key: PropertyKey): (v: unknown) => T {
	return Function.prototype.call.bind(Object.getOwnPropertyDescriptor(typedArray, key)!.get!) as (v: unknown) => T;
}
const lengthOf = getter<number>("length");
const offsetOf = getter<number>("byteOffset");
const bufferOf = getter<ArrayBufferLike>("buffer");
const tagOf = getter<string | undefined>(Symbol.toStringTag);

/** How many items `v` holds, throwing a TypeError unless the engine holds it as a `kind`. */
function typedLength(v: unknown, kind: string): number {
	if (tagOf(v) !== kind) {
		const shown = v !== null && (typeof v === "object" || typeof v === "function") ? "an object" : String(v);
		throw new TypeError(`${shown} is no ${kind}`);
	}
	return lengthOf(v);
}

/** A view of the bytes of `v`, a `kind` of `size`-byte items, which throws unless it is one. */
export function typedBytes(v: unknown, kind: string, size: number): Uint8Array {
	const n = typedLength(v, kind);
	return new Uint8Array(bufferOf(v), offsetOf(v), n * size);
}

// Text and bytes.

const encoder = new TextEncoder();
// A leading U+FEFF is text Rust holds, not a mark to drop.
const strict = new TextDecoder("utf-8", { fatal: true, ignoreBOM: true });
const loose = new TextDecoder("utf-8", { ignoreBOM: true });

/** `text`'s UTF-8, throwing where it holds a lone surrogate, which no Rust string does. */
export function encoded(text: string): Uint8Array {
	if (!text.isWellFormed()) throw new RangeError("a string holds a lone surrogate, which no Rust string does");
	return encoder.encode(text);
}

/** Write a view of `text`'s UTF-8 on the stack into the buffer at `dst`. */
export function putStr(dst: number, text: string): void {
	if (!text.isWellFormed()) throw new RangeError("a string holds a lone surrogate, which no Rust string does");
	const max = text.length * 3;
	const p = alloc(max, 1);
	const { written } = encoder.encodeInto(text, mem().subarray(p, p + max));
	placePut(dst, p);
	lenPut(dst + WORD, written);
	lenPut(dst + 2 * WORD, 0);
}

/**
 * Write a view of `data` into the buffer at `dst`: of Rust's buffer it lent, in place,
 * which the call holds until it returns, else of a copy on the stack.
 */
export function putBytes(dst: number, data: Uint8Array | Bytes): void {
	if (data instanceof Bytes) {
		const read = bytesRead(data, true);
		if (!(read instanceof Uint8Array)) {
			addrPut(dst, read[0]);
			lenPut(dst + WORD, read[2]);
			lenPut(dst + 2 * WORD, 0);
			return;
		}
		data = read;
	}
	// `set` copies as many bytes as the engine keeps, which no getter changes.
	const n = typedLength(data, "Uint8Array");
	const p = alloc(n, 1);
	mem().set(data, p);
	placePut(dst, p);
	lenPut(dst + WORD, n);
	lenPut(dst + 2 * WORD, 0);
}

/**
 * Make the view [putBytes] wrote at `dst` of `data`, if it lent Rust's buffer, a second
 * owner of it, which Rust takes with no copy. A call's last step before it reaches Rust,
 * so nothing that throws after leaves the owner given.
 */
export function shared(dst: number, data: Uint8Array | Bytes | undefined): void {
	if (!(data instanceof Bytes)) return;
	const read = bytesRead(data, false);
	if (read instanceof Uint8Array) return;
	const m = top;
	const view = alloc(3 * WORD, WORD);
	addrPut(view, read[0]);
	lenPut(view + WORD, read[2]);
	addrPut(view + 2 * WORD, read[1]);
	s.d++;
	w[`${prefix}_buf_clone`](dst, view);
	s.d--;
	top = m;
}

/** Write the null buffer, `None`, at `dst`. */
export function putNone(dst: number): void {
	lenPut(dst, 0);
	lenPut(dst + WORD, 0);
	lenPut(dst + 2 * WORD, 0);
}

/** Whether the buffer at `p` is `None`. */
export function isNone(p: number): boolean {
	return lenGet(p) === 0;
}

/** A copy of the bytes the buffer at `p` views. */
export function bytesLent(p: number): Uint8Array {
	return lentCopy(addrGet(p), lenGet(p + WORD));
}

/** The text of the UTF-8 the buffer at `p` views. */
export function strLent(p: number): string {
	const len = lenGet(p + WORD);
	return len === 0 ? "" : lentText(addrGet(p), len, strict);
}

/** A copy of the bytes of the buffer Rust returned at `p`, which it frees. */
export function bytes_(p: number): Uint8Array {
	const out = bytesLent(p);
	w[`${prefix}_buf_free`](p);
	return out;
}

/**
 * Bytes up to this many that a call returns are copied out: an engine keeps a typed array
 * this short in its own heap, so the copy costs less than lending Rust's buffer, and past
 * it V8's copy, which needs a buffer of its own, costs more than the lend. SpiderMonkey's
 * lend costs more than its copy up to about 4 KiB, as bench/ffi/README.md measures.
 */
const LEND = 64;

/**
 * How many bytes of Rust's buffers are lent and not yet freed, by their length: a `Vec`
 * crosses with no spare capacity, but a `Bytes` may be a slice of a larger buffer it keeps
 * whole, as Rust's own value does, which its length does not show.
 */
let lentLength = 0;

// #region web lends
/** Memory's size when no bytes were last lent. */
let unlentSize = 0;

/**
 * Whether bytes of `len` a call returned are lent: so that lent bytes a caller forgot to
 * dispose, which the collector frees only once it runs for its own heap, make memory grow
 * by no more than a third, they lend while their length stays under a quarter of memory,
 * and while memory has grown by no more than a third since none were lent, which bounds
 * what slices keep that their length does not.
 */
function lends(len: number): boolean {
	const size = memory.buffer.byteLength;
	if (lentLength === 0) unlentSize = size;
	return len > LEND && lentLength + len <= size / 4 && size - unlentSize <= unlentSize / 3;
}

/** A copy of the bytes from `from` up to `to` of the `len` at `ptr`, as `Uint8Array`'s `slice`. */
function lentSlice(ptr: Addr, len: number, from?: number, to?: number): Uint8Array<ArrayBuffer> {
	return mem().subarray(ptr, ptr + len).slice(from, to);
}

/** Bytes of `len` [lends] lent are freed. */
function unlends(_len: number): void {}
// #endregion web lends

/** The frees the garbage collector makes of lent bytes never disposed: each owner and length. */
const unlent = new FinalizationRegistry<[Addr, number]>(([owner, length]) => {
	if (live()) ownerFree(owner, length);
});

/** Free the owner of `length` lent bytes, which frees Rust's buffer once it is the last. */
function ownerFree(owner: Addr, length: number): void {
	lentLength -= length;
	unlends(length);
	const m = enter();
	const buf = alloc(3 * WORD, WORD);
	lenPut(buf, 0);
	lenPut(buf + WORD, 0);
	addrPut(buf + 2 * WORD, owner);
	s.d++;
	w[`${prefix}_buf_free`](buf);
	s.d--;
	leave(m);
}

// The private fields' accessors, which only the runtime holds.
let bytesMade: (ptr: Addr, length: number, owner: Addr, copy: Uint8Array | undefined) => Bytes;
let bytesRead: (b: Bytes, hold: boolean) => Uint8Array | [ptr: Addr, owner: Addr, length: number];
let bytesRelease: (b: Bytes) => void;
let bytesEvict: (b: Bytes) => void;

/**
 * Bytes a call returns. `slice()` copies them into a `Uint8Array`, `at(i)` reads one, and
 * passed to a call that reads them or takes a `Bytes`, they cross with no copy. Past 64
 * bytes, they are Rust's own buffer,
 * lent in place, which disposing frees, as `using` does, or else the garbage collector;
 * up to 64, and past a quarter of memory lent, a copy, which reads and disposes the same.
 * A read after disposing throws. No view of Rust's buffer reaches a caller: it would detach
 * once memory grows, write what other owners of the bytes read, and outlive the buffer.
 */
export class Bytes {
	// How many bytes it holds, private so that no caller's write reaches what Rust reads.
	readonly #length: number;
	// Rust's buffer and its owner, both null for a copy.
	readonly #ptr: Addr;
	readonly #owner: Addr;
	readonly #copy: Uint8Array | undefined;
	/** The owner's hold until disposed, and one for each call viewing Rust's buffer. */
	#h = 1;
	#closed = false;
	/** A lent one's slot in the nursery, or whether it registered once it left. */
	#n = UNREGISTERED;

	private constructor(key: symbol, ptr: Addr, length: number, owner: Addr, copy: Uint8Array | undefined) {
		if (key !== minting) throw new TypeError("only Rust makes bytes it returned");
		this.#ptr = ptr;
		this.#length = length;
		this.#owner = owner;
		this.#copy = copy;
		if (owner !== NULL) {
			lentLength += length;
			this.#n = nurse(this);
		}
	}

	/** How many bytes it holds. */
	get length(): number {
		return this.#length;
	}

	/** The byte at `index`, from the end where it is negative, as `Uint8Array`'s `at`. */
	at(index: number): number | undefined {
		// Converted first, so a `valueOf` that disposes these bytes finds them closed.
		let i = Math.trunc(index) || 0;
		if (this.#closed) throw new Error("Bytes is closed");
		if (i < 0) i += this.#length;
		if (i < 0 || i >= this.#length) return undefined;
		return this.#copy !== undefined ? this.#copy[i] : lentByte(this.#ptr, i);
	}

	/** A copy of the bytes from `start` up to `end`, as `Uint8Array`'s `slice`. */
	slice(start?: number, end?: number): Uint8Array<ArrayBuffer> {
		// Converted first, as `at`'s index is.
		const from = start === undefined ? undefined : +start;
		const to = end === undefined ? undefined : +end;
		if (this.#closed) throw new Error("Bytes is closed");
		// `subarray` and `slice` make their results through `Symbol.species`, which may run
		// any JavaScript: a hold keeps Rust's buffer until they return.
		this.#h++;
		try {
			if (this.#copy !== undefined) return this.#copy.slice(from, to) as Uint8Array<ArrayBuffer>;
			return lentSlice(this.#ptr, this.#length, from, to);
		} finally {
			this.#release();
		}
	}

	/** Each byte in turn, each read as `at` reads it. */
	*[Symbol.iterator](): IterableIterator<number> {
		for (let i = 0; i < this.#length; i++) yield this.at(i)!;
	}

	/** Free Rust's buffer, once no call views it; again, it does nothing. */
	[Symbol.dispose](): void {
		if (this.#closed) return;
		this.#closed = true;
		this.#release();
	}

	#release(): void {
		if (--this.#h === 0 && this.#owner !== NULL) {
			if (this.#n >= 0) nursery[this.#n] = undefined;
			else if (this.#n === REGISTERED) unlent.unregister(this);
			ownerFree(this.#owner, this.#length);
		}
	}

	static {
		bytesMade = (ptr, length, owner, copy) => new Bytes(minting, ptr, length, owner, copy);
		bytesRead = (b, hold) => {
			// A read without a hold is the call's own, which took its hold first, so disposing
			// since closes nothing it reads.
			if (hold && b.#closed) throw new Error("Bytes is closed");
			if (b.#copy !== undefined) return b.#copy;
			if (hold) {
				b.#h++;
				held.push(b);
			}
			return [b.#ptr, b.#owner, b.#length];
		};
		bytesRelease = (b) => b.#release();
		bytesEvict = (b) => {
			if (b.#closed) {
				b.#n = UNREGISTERED;
				return;
			}
			unlent.register(b, [b.#owner, b.#length], b);
			b.#n = REGISTERED;
		};
	}
}

/** The bytes of the buffer Rust returned at `p`: lent past [LEND] bytes, else copied and freed. */
export function returnedBytes(p: number): Bytes {
	const len = lenGet(p + WORD);
	if (lends(len)) return bytesMade(addrGet(p), len, addrGet(p + 2 * WORD), undefined);
	return bytesMade(NULL, len, NULL, bytes_(p));
}

/** The text of the buffer Rust returned at `p`, which it frees. */
export function str(p: number): string {
	const out = strLent(p);
	w[`${prefix}_buf_free`](p);
	return out;
}

/** Rust's own copy of `text`, as `buf_copy` makes it, written at `dst`: what a closure returns. */
export function copied(dst: number, text: string, message = false): void {
	const m = top;
	const view = alloc(3 * WORD, WORD);
	if (message) putBytes(view, encoder.encode(text));
	else putStr(view, text);
	s.d++;
	w[`${prefix}_buf_copy`](dst, view);
	s.d--;
	top = m;
}

/** Rust's own copy of `data`, as `buf_copy` makes it, written at `dst`. */
export function copiedBytes(dst: number, data: Uint8Array): void {
	const m = top;
	const view = alloc(3 * WORD, WORD);
	putBytes(view, data);
	s.d++;
	w[`${prefix}_buf_copy`](dst, view);
	s.d--;
	top = m;
}

/** Write the `[u8; n]` `data` at `p`, which throws unless it holds exactly `n` bytes. */
export function arrayPut(p: number, data: Uint8Array, n: number): void {
	const length = typedLength(data, "Uint8Array");
	if (length !== n) throw new RangeError(`${length} bytes are no [u8; ${n}]`);
	mem().set(data, p);
}

/** A path's bytes, which may be any on wasm32, as on Unix. */
export function pathBytes(path: PathBuf): Uint8Array {
	return path.bytes;
}

// Values of std's types, as the C ABI lays them out.

/** The `Duration` at `p`. */
export function durationGet(p: number): Duration {
	return { secs: u64Get(p), nanos: dv().getUint32(p + 8, true) };
}

/** Write the `Duration` `v` at `p`: nanoseconds of a second or more throw. */
export function durationPut(p: number, v: Duration): void {
	const nanos = int(v.nanos, 0, 999999999, "Duration's nanoseconds");
	u64Put(p, v.secs);
	put32(p + 8, nanos);
}

function octets(p: number, n: number): Uint8Array {
	return mem().slice(p, p + n);
}

function putOctets(p: number, v: Uint8Array, n: number): void {
	if (typedLength(v, "Uint8Array") !== n) throw new RangeError(`an address is no ${n} bytes`);
	mem().set(v, p);
}

export function ipv4Get(p: number): Ipv4Addr {
	return { octets: octets(p, 4) };
}

export function ipv4Put(p: number, v: Ipv4Addr): void {
	putOctets(p, v.octets, 4);
}

export function ipv6Get(p: number): Ipv6Addr {
	return { octets: octets(p, 16) };
}

export function ipv6Put(p: number, v: Ipv6Addr): void {
	putOctets(p, v.octets, 16);
}

export function ipGet(p: number): IpAddr {
	return dv().getUint8(p) !== 0 ? { tag: "V6", _0: ipv6Get(p + 1) } : { tag: "V4", _0: ipv4Get(p + 1) };
}

export function ipPut(p: number, v: IpAddr): void {
	const { tag, _0 } = v;
	if (tag === "V6") ipv6Put(p + 1, _0);
	else if (tag === "V4") ipv4Put(p + 1, _0);
	else throw new RangeError("an IpAddr is V4 or V6");
	put8(p, tag === "V6" ? 1 : 0);
}

export function socketV4Get(p: number): SocketAddrV4 {
	return { ip: ipv4Get(p), port: dv().getUint16(p + 4, true) };
}

export function socketV4Put(p: number, v: SocketAddrV4): void {
	ipv4Put(p, v.ip);
	put16(p + 4, int(v.port, 0, 0xffff, "port"));
}

export function socketV6Get(p: number): SocketAddrV6 {
	const d = dv();
	return {
		ip: ipv6Get(p),
		port: d.getUint16(p + 16, true),
		flowinfo: d.getUint32(p + 20, true),
		scopeId: d.getUint32(p + 24, true),
	};
}

export function socketV6Put(p: number, v: SocketAddrV6): void {
	ipv6Put(p, v.ip);
	const port = int(v.port, 0, 0xffff, "port");
	const flowinfo = int(v.flowinfo, 0, 0xffffffff, "flow label");
	const scope = int(v.scopeId, 0, 0xffffffff, "scope");
	const d = dv();
	d.setUint16(p + 16, port, true);
	d.setUint32(p + 20, flowinfo, true);
	d.setUint32(p + 24, scope, true);
}

export function socketGet(p: number): SocketAddr {
	return dv().getUint8(p) !== 0 ? { tag: "V6", _0: socketV6Get(p + 4) } : { tag: "V4", _0: socketV4Get(p + 4) };
}

export function socketPut(p: number, v: SocketAddr): void {
	const { tag, _0 } = v;
	if (tag === "V6") socketV6Put(p + 4, _0);
	else if (tag === "V4") socketV4Put(p + 4, _0);
	else throw new RangeError("a SocketAddr is V4 or V6");
	put8(p, tag === "V6" ? 1 : 0);
}

// What JavaScript implements for Rust, each found by its token.

/** Each value Rust holds or a call lends, by token, and what Rust's drop of it calls. */
const tokens = new Map<number, unknown>();
const drops = new Map<number, () => void>();
let token = 0;

// #region web hooks
/** Tell Rust how many tokens JavaScript holds, once that changes: the module has no other thread to call them. */
function relayed(): void {}

/** Tell the runtime whether a call into Rust pends, once that changes: the page lives on regardless. */
function pends(_pending: boolean): void {}
// #endregion web hooks

/** A token for `value`, which Rust frees through `free`, calling `drop` then. */
export function give(value: unknown, drop?: () => void): number {
	const t = ++token;
	tokens.set(t, value);
	relayed();
	if (drop) drops.set(t, drop);
	return t;
}

/** Write at `p` the implementation `value` of a callback interface, which Rust calls through
 * `vtable` and closes once it drops it. */
export function callbackPut(p: number, vtable: Addr, value: { close?(): void }): void {
	const t = give(value, () => value.close?.());
	addrPut(p, vtable);
	lenPut(p + WORD, t);
}

/** A token for `value`, forgotten once the running call returns. */
export function lend(value: unknown): number {
	const t = ++token;
	tokens.set(t, value);
	relayed();
	lent.push(t);
	return t;
}

/** The value of `t`. */
export function find<T>(t: number): T {
	return tokens.get(t) as T;
}

/** The table index of each C function the runtime's values take, as `fns` gives them. */
export const fn: { wake: Addr; wakerDrop: Addr; free: Addr; poll: Addr; read: Addr; seek: Addr } = { wake: NULL, wakerDrop: NULL, free: NULL, poll: NULL, read: NULL, seek: NULL };

/**
 * Run `f`, which JavaScript does for Rust inside a call, aborting the module on what it
 * throws, since Rust has nowhere to unwind to.
 */
export function out<T>(what: string, f: () => T): T {
	// What `f` takes of the stack, it gives back: a value lowered there is read by now.
	const m = top;
	s.d--;
	try {
		return f();
	} catch (e) {
		if (aborted !== undefined) throw e;
		return abort(`${what}: a call into JavaScript threw ${thrown(e)}`);
	} finally {
		s.d++;
		top = m;
	}
}

/** What `e` says, with its stack: V8's begins with its message, which SpiderMonkey's lacks. */
function thrown(e: unknown): string {
	if (!(e instanceof Error) || e.stack === undefined) return String(e);
	return e.stack.startsWith(String(e)) ? e.stack : `${e}\n${e.stack}`;
}

/** Write a waker of `t`'s wakes at `dst`. */
function putWaker(dst: number, t: number): void {
	addrPut(dst, fn.wake);
	addrPut(dst + WORD, fn.wakerDrop);
	lenPut(dst + 2 * WORD, t);
}

/** What each token's wake calls, and what dropping its last waker calls. */
const wakes = new Map<number, () => void>();

/** Write at `dst` a waker Rust holds until it drops it, calling `wake` on a microtask. */
export function wakerOf(dst: number, wake: () => void): void {
	const t = ++token;
	wakes.set(t, wake);
	drops.set(t, () => wakes.delete(t));
	putWaker(dst, t);
}

/** A waker Rust returned at `p`, the caller's to wake through Rust and drop once. */
export function wakerGet(p: number): Waker {
	const wake = addrGet(p);
	const drop = addrGet(p + WORD);
	const data = addrGet(p + 2 * WORD);
	let dropped = false;
	const waker = {
		wake() {
			if (dropped) throw new Error("Waker is closed");
			const m = enter();
			s.d++;
			w[`${space}_call`](wake, data);
			s.d--;
			leave(m);
		},
		[Symbol.dispose]() {
			if (dropped) return;
			dropped = true;
			wakers.unregister(waker);
			const m = enter();
			s.d++;
			w[`${space}_call`](drop, data);
			s.d--;
			leave(m);
		},
	};
	wakers.register(waker, [drop, data], waker);
	return waker;
}

const wakers = new FinalizationRegistry<[Addr, Addr]>(([drop, data]) => {
	if (!live()) return;
	s.d++;
	w[`${space}_call`](drop, data);
	s.d--;
});

/** A future JavaScript gave Rust: its promise's outcome, and the waker of Rust's last poll. */
type Foreign = { done: boolean; failed: unknown; wake: Addr; drop: Addr; data: Addr; call?: Call<unknown> };

/** Subscribe to a future before validating handles or transferring any value to Rust. */
export function futurePrepare(promise: Promise<unknown>): Foreign {
	const f: Foreign = { done: false, failed: undefined, wake: NULL, drop: NULL, data: NULL };
	if (calls.has(promise)) f.call = promise as Call<unknown>;
	promise.then(
		() => settle(f, undefined),
		(e) => settle(f, e ?? new Error("a future given to Rust rejected")),
	);
	return f;
}

/** Transfer a prepared future to Rust without running user JavaScript. */
export function futurePut(dst: number, f: Foreign): void {
	const t = give(f, () => {
		f.done = true;
		release(f);
		f.call?.cancel();
	});
	addrPut(dst, fn.poll);
	addrPut(dst + WORD, fn.free);
	lenPut(dst + 2 * WORD, t);
}

/** Write at `dst` the future Rust polls of `promise`, until it completes. */
export function futureOf(dst: number, promise: Promise<unknown>): void {
	futurePut(dst, futurePrepare(promise));
}

function settle(f: Foreign, failed: unknown): void {
	if (f.done) return;
	if (failed !== undefined) {
		if (!(failed instanceof Cancelled)) abort(`a future given to Rust: it rejected with ${failed}`);
	}
	f.done = true;
	const { wake, data } = f;
	if (data === NULL || aborted !== undefined) return;
	const m = enter();
	s.d++;
	w[`${space}_call`](wake, data);
	s.d--;
	leave(m);
	release(f);
}

/** Drop the waker `f` holds, if any. */
function release(f: Foreign): void {
	if (f.data === NULL || aborted !== undefined) return;
	const { drop, data } = f;
	f.data = NULL;
	const m = enter();
	s.d++;
	w[`${space}_call`](drop, data);
	s.d--;
	leave(m);
}

/** Write at `dst` `reader`, lent to Rust for the running call. */
export function readerOf(dst: number, reader: Reader): void {
	const t = lend(reader);
	addrPut(dst, fn.read);
	addrPut(dst + WORD, fn.seek);
	lenPut(dst + 2 * WORD, t);
}

/** Write the error `e` a reader threw at `err`, as an `io::Error`'s message. */
function readerError(err: number, e: unknown): false {
	copied(err, e instanceof Error ? e.message : String(e), true);
	return false;
}

/** The functions the module imports from `rust2`: the C functions of what JavaScript implements. */
function rust2(extra: Record<string, (...args: number[]) => unknown>): Record<string, unknown> {
	return {
		...extra,
		rust2_wake(t: number) {
			const wake = wakes.get(t);
			if (wake) queueMicrotask(wake);
		},
		rust2_waker_drop(t: number) {
			drops.get(t)?.();
			drops.delete(t);
		},
		rust2_free(t: number) {
			out("a drop", () => {
				const drop = drops.get(t);
				tokens.delete(t);
				relayed();
				drops.delete(t);
				drop?.();
			});
		},
		rust2_future_poll(t: number, waker: number): number {
			return out("a future's poll", () => {
				const f = find<Foreign>(t);
				release(f);
				if (f.done) {
					s.d++;
					w[`${space}_call`](addrGet(waker + WORD), addrGet(waker + 2 * WORD));
					s.d--;
					return 1;
				}
				f.wake = addrGet(waker);
				f.drop = addrGet(waker + WORD);
				f.data = addrGet(waker + 2 * WORD);
				return 0;
			});
		},
		rust2_reader_read(t: number, buf: Addr, len: number, n: number, err: number): number {
			return out("a reader's read", () => {
				const owned = new Uint8Array(len);
				let got: number;
				try {
					got = find<Reader>(t).read(owned);
				} catch (e) {
					return readerError(err, e) ? 1 : 0;
				}
				if (Number.isInteger(got) && got > 0 && got <= len) lentWrite(buf, owned.subarray(0, got));
				lenPut(n, got >>> 0 === got ? got : 0xffffffff);
				return 1;
			});
		},
		rust2_reader_seek(t: number, whence: number, offset: bigint, position: number, err: number): number {
			return out("a reader's seek", () => {
				const from: SeekFrom = whence === 0 ? "Start" : whence === 1 ? "Current" : "End";
				let at: u64;
				try {
					at = find<Reader>(t).seek(from === "Start" ? u64OfBig(offset) : i64OfBig(offset), from);
				} catch (e) {
					return readerError(err, e) ? 1 : 0;
				}
				u64Put(position, at);
				return 1;
			});
		},
	};
}

// Futures: a call into Rust's async API polls its future once as it is made, and again on a
// microtask each time Rust wakes it, never inside the wake.

/** The calls pending in Rust, by their promise. */
const calls = new Map<Promise<unknown>, Pending<unknown>>();

/** A future in Rust that pended, until it finishes. */
class Pending<T> {
	f: number;
	/** The waker's token, whose wakes poll it again. */
	t = 0;
	polling = false;
	cancelled = false;
	resolve!: (value: T) => void;
	reject!: (error: unknown) => void;
	promise!: Call<T>;
	readonly poll: (f: number, waker: number) => [boolean, T];
	readonly free: string;

	constructor(f: number, poll: (f: number, waker: number) => [boolean, T], free: string) {
		this.f = f;
		this.poll = poll;
		this.free = free;
	}

	wake = () => {
		if (this.f !== 0 && !this.polling) this.step();
	};

	step(): void {
		this.polling = true;
		let done: boolean;
		let value: T;
		let m = 0;
		try {
			m = enter();
			const waker = alloc(3 * WORD, WORD);
			putWaker(waker, this.t);
			[done, value] = this.poll(this.f, waker);
		} catch (e) {
			// The promise settles whatever the cleanup throws.
			try {
				if (m !== 0) leave(m);
				this.polling = false;
				this.finish();
			} finally {
				this.reject(e);
			}
			return;
		}
		leave(m);
		this.polling = false;
		if (done) {
			this.finish();
			this.resolve(value);
		} else if (this.cancelled) {
			this.cancel();
		}
	}

	finish(): void {
		if (this.f === 0) return;
		const f = this.f;
		this.f = 0;
		calls.delete(this.promise);
		if (calls.size === 0) pends(false);
		wakes.delete(this.t);
		if (!live()) return;
		const m = enter();
		s.d++;
		w[this.free](f);
		s.d--;
		leave(m);
	}

	cancel(): void {
		this.finish();
		this.promise.catch(() => {});
		this.reject(new Cancelled());
	}
}

/**
 * The `Call` of a future whose first poll gave `first`: its value if ready, or else the
 * handle of the future that pended, which `poll` polls with a waker and `free` frees.
 */
export function pending<T>(f: number, poll: (f: number, waker: number) => [boolean, T], free: string): Call<T> {
	const p = new Pending<T>(f, poll, free);
	const promise = new Promise<T>((resolve, reject) => {
		p.resolve = resolve;
		p.reject = reject;
	}) as Call<T>;
	promise.cancel = () => {
		if (p.f === 0 || p.cancelled) return;
		p.cancelled = true;
		if (!p.polling) p.cancel();
	};
	p.promise = promise;
	calls.set(promise, p as Pending<unknown>);
	if (calls.size === 1) pends(true);
	p.t = ++token;
	wakes.set(p.t, p.wake);
	p.step();
	return promise;
}

/** The `Call` of a future ready at its first poll, with `value`. */
export function ready<T>(value: T): Call<T> {
	const promise = Promise.resolve(value) as Call<T>;
	promise.cancel = () => {};
	return promise;
}

/** The `Call` of a future whose first poll failed with `error`. */
export function failed<T>(error: unknown): Call<T> {
	const promise = Promise.reject(error) as Call<T>;
	promise.cancel = () => {};
	return promise;
}

// #region shared
/** A stream's items, as `for await` takes them, by a name no export's can hide. */
export type Stream<T> = AsyncIterableIterator<T>;

/**
 * A stream's items, as an async iterator calling `next` once for each: it ends where
 * `next` gives `none`, or with `fallible` at an `Ok` of `none`, and yields an `Err` last.
 * It reads nothing ahead, and `return` cancels a pending `next` in Rust.
 */
export function values<T>(next: () => Call<unknown>, none: unknown, fallible: boolean): Stream<T> {
	return new Values<T>(next, none, fallible);
}

const end: IteratorReturnResult<undefined> = { done: true, value: undefined };

class Values<T> implements AsyncIterableIterator<T> {
	readonly #next: () => Call<unknown>;
	readonly #none: unknown;
	readonly #fallible: boolean;
	#done = false;
	/** The `next` pending in Rust, which `return` cancels. */
	#call: Call<unknown> | undefined;
	/** The last `next` asked for, after which the next one runs, so they never overlap. */
	#queue: Promise<unknown> = Promise.resolve();

	constructor(next: () => Call<unknown>, none: unknown, fallible: boolean) {
		this.#next = next;
		this.#none = none;
		this.#fallible = fallible;
	}

	next(): Promise<IteratorResult<T, undefined>> {
		const result = this.#queue.then(() => this.#pull());
		this.#queue = result.catch(() => {});
		return result;
	}

	async #pull(): Promise<IteratorResult<T, undefined>> {
		if (this.#done) return end;
		let out: unknown;
		try {
			const call = this.#next();
			this.#call = call;
			// A `return` from a callback Rust called in the start, before there was a call
			// for it to cancel.
			if (this.#done) call.cancel();
			out = await call;
		} catch (e) {
			this.#done = true;
			if (e instanceof Cancelled) return end;
			throw e;
		} finally {
			this.#call = undefined;
		}
		// A `next` that finished as `return` cancelled it still gives its item, which would
		// otherwise be lost.
		if (this.#fallible) {
			const result = out as Result<unknown, unknown>;
			if (!result.ok) {
				this.#done = true;
				return { done: false, value: result as T };
			}
			if (result.value === this.#none) return this.#finish();
		} else if (out === this.#none) {
			return this.#finish();
		}
		return { done: false, value: out as T };
	}

	#finish(): IteratorReturnResult<undefined> {
		this.#done = true;
		return end;
	}

	async return(): Promise<IteratorReturnResult<undefined>> {
		this.#done = true;
		this.#call?.cancel();
		return end;
	}

	[Symbol.asyncIterator](): AsyncIterableIterator<T> {
		return this;
	}
}
// #endregion shared

// The module, and the web APIs its std imports.

let prefix = "";
/** The namespace of the glue's C names. */
let space = "";

// #region web natives
// #endregion web natives

// #region web load
const text = { out: "", err: "" };

/** A line of what the module wrote to stdout or stderr, to `console.log` or `console.error`. */
/** Stdout's and stderr's decoders, each keeping a character a write splits for the next. */
const streams = [new TextDecoder("utf-8", { ignoreBOM: true }), new TextDecoder("utf-8", { ignoreBOM: true })];

function written(fd: number, chunk: string): void {
	const key = fd === 1 ? "out" : "err";
	let pending = text[key] + chunk;
	for (let at = pending.indexOf("\n"); at >= 0; at = pending.indexOf("\n")) {
		(fd === 1 ? console.log : console.error)(pending.slice(0, at));
		pending = pending.slice(at + 1);
	}
	text[key] = pending;
}

/** What the module's std imports, each the web API standing in for it. */
const wasi = {
	clock_time_get(id: number, _precision: bigint, out: number): number {
		// The realtime clock from `Date.now`'s origin, at `performance.now`'s precision; every
		// other clock is monotonic.
		const ms = id === 0 ? performance.timeOrigin + performance.now() : performance.now();
		const ns = BigInt(Math.floor(ms * 1e6));
		dv().setBigUint64(out, ns, true);
		return 0;
	},
	clock_res_get(_id: number, out: number): number {
		dv().setBigUint64(out, 1000n, true);
		return 0;
	},
	random_get(buf: number, len: number): number {
		// `getRandomValues` fills at most 64 KiB at a time.
		for (let at = 0; at < len; at += 65536) {
			crypto.getRandomValues(mem().subarray(buf + at, buf + Math.min(len, at + 65536)));
		}
		return 0;
	},
	fd_write(fd: number, iovs: number, n: number, nwritten: number): number {
		// A web page has no stream but the console's.
		if (fd !== 1 && fd !== 2) return 8;
		const d = dv();
		let total = 0;
		for (let i = 0; i < n; i++) {
			const ptr = d.getUint32(iovs + 8 * i, true);
			const len = d.getUint32(iovs + 8 * i + 4, true);
			written(fd, streams[fd - 1].decode(mem().subarray(ptr, ptr + len), { stream: true }));
			total += len;
		}
		dv().setUint32(nwritten, total, true);
		return 0;
	},
	// A web page has no environment and no arguments.
	environ_sizes_get(count: number, size: number): number {
		const d = dv();
		d.setUint32(count, 0, true);
		d.setUint32(size, 0, true);
		return 0;
	},
	environ_get(): number {
		return 0;
	},
	args_sizes_get(count: number, size: number): number {
		return wasi.environ_sizes_get(count, size);
	},
	args_get(): number {
		return 0;
	},
	// One thread, with nothing to yield to.
	sched_yield(): number {
		return 0;
	},
	proc_exit(code: number): never {
		aborted ??= new Error(`the module exited with ${code}`);
		throw dead();
	},
};

/**
 * What `load` takes the module from: a compiled module, its bytes, a `Response` such as
 * `fetch` gives, or its URL, which a `file:` URL reads through Node's file system where the
 * runtime has one, as Node's `fetch` refuses that scheme, and any other fetches.
 */
export type Source = WebAssembly.Module | BufferSource | Response | PromiseLike<Response> | URL;

/** Node's file system, where the runtime has one: Node, Bun, and Deno, but no browser. */
type Files = { readFile(path: URL): Promise<Uint8Array<ArrayBuffer>> };

/**
 * The bytes or the response at `url`. Node's file system is found by
 * `process.getBuiltinModule`, a call no bundler follows, so a browser's bundle imports none.
 */
function fetchUrl(url: URL): Promise<BufferSource | Response> {
	const node = (globalThis as { process?: { getBuiltinModule?(id: string): unknown } }).process;
	const files = url.protocol === "file:" ? (node?.getBuiltinModule?.("node:fs/promises") as Files | undefined) : undefined;
	return files ? files.readFile(url) : fetch(url);
}

/** What a binding's `load` passes: its C prefix and glue's namespace, its contract and target, and its imports. */
export type Glue = {
	readonly prefix: string;
	readonly space: string;
	readonly file: string;
	readonly contract: bigint;
	readonly target: number;
	readonly imports: Record<string, (...args: never[]) => unknown>;
};

/**
 * Compile and instantiate the module from `source`, checking it is the build `glue` was
 * generated with, before any call. It refuses an import with no web API standing in for it.
 */
export async function load(source: Source, glue: Glue): Promise<void> {
	if (loading || loaded) throw new Error(`${glue.file}: the module is loaded already`);
	// Taken before the first await, so a second load meanwhile is refused; given back on a
	// failure, so a load after one may succeed.
	loading = true;
	try {
		await instantiate(source, glue);
		loaded = true;
	} catch (e) {
		// What the refused instance left, such as a `proc_exit` in its initializer, is not
		// the next one's.
		aborted = undefined;
		s.d = 0;
		throw e;
	} finally {
		loading = false;
	}
}

let loading = false;
let loaded = false;

async function instantiate(source: Source, glue: Glue): Promise<void> {
	let module: WebAssembly.Module;
	if (source instanceof URL) source = await fetchUrl(source);
	if (source instanceof WebAssembly.Module) module = source;
	else if (source instanceof ArrayBuffer || ArrayBuffer.isView(source)) module = await WebAssembly.compile(source);
	// A promise of its own, as Deno's types take no other thenable.
	else module = await WebAssembly.compileStreaming(Promise.resolve(source as Response | PromiseLike<Response>));
	const rust = rust2(glue.imports as Record<string, (...args: number[]) => unknown>);
	for (const i of WebAssembly.Module.imports(module)) {
		// Its own, not what every object inherits, such as `toString`.
		const known =
			i.module === "wasi_snapshot_preview1" ? Object.hasOwn(wasi, i.name) : i.module === "rust2" && Object.hasOwn(rust, i.name);
		if (!known) {
			throw new Error(`${glue.file}: the module imports ${i.module}.${i.name}, which no web API stands in for`);
		}
	}
	const instance = await WebAssembly.instantiate(module, {
		wasi_snapshot_preview1: wasi,
		rust2: rust as WebAssembly.ModuleImports,
	});
	const exports = instance.exports as Exports;
	memory = exports.memory as WebAssembly.Memory;
	// A refused load's views are of its own memory, which this one never detaches.
	view = new DataView(new ArrayBuffer(0));
	bytes = new Uint8Array(0);
	prefix = glue.prefix;
	space = glue.space;
	exports._initialize?.();
	if (exports[`${prefix}_contract`]() !== BigInt.asIntN(64, glue.contract)) {
		throw new Error(`${glue.file} and the module are from different builds`);
	}
	if (exports[`${prefix}_target`]() !== glue.target) {
		throw new Error(`${glue.file}: the module was built for another target than wasm32-wasip1`);
	}
	w = exports;
	base = exports[`${space}_alloc`](STACK, 16);
	const p = exports[`${space}_fns`]();
	const d = dv();
	const names = ["wake", "wakerDrop", "free", "poll", "read", "seek"] as const;
	names.forEach((name, i) => {
		fn[name] = d.getUint32(p + 4 * i, true);
	});
}
// #endregion web load
