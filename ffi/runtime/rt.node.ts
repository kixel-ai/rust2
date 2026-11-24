// Node's regions of rt.ts: each `// #region node <name>` stands in for rt.ts's region
// `// #region web <name>`, which rust2-ffi replaces as it writes Node's runtime. The rest
// of rt.ts, and so the generated TypeScript, is the web's: values lower into memory the
// addon lends, its scratch memory, as they lower into the module's, but at the native C
// ABI's layouts, whose pointers and `usize` are 64 bits. A pointer the TypeScript reads is
// Rust's, which only the addon's natives read through. Each native function of the C ABI
// holds its handles' indices, throws the status it fails a check with, and converts its
// integers, so a call of numbers and handles alone is one call of it.

// #region node live
/**
 * Whether Rust may run: no call aborted. A call into the addon cannot trap, as a module's
 * can, since a panic there aborts the process, so `s.d` counts nothing here.
 */
function live(): boolean {
	return aborted === undefined;
}
// #endregion node live

// #region node memory
/** The scratch memory: one reservation, which never moves, so its views never detach. */
let view = new DataView(new ArrayBuffer(0));
let bytes: Uint8Array<ArrayBuffer> = new Uint8Array(0);

/** A view of the scratch memory. */
export function dv(): DataView {
	return view;
}

/** The scratch memory as bytes. */
export function mem(): Uint8Array<ArrayBuffer> {
	return bytes;
}

/** An address in Rust's memory: a `bigint`. */
export type Addr = bigint;
/** The null address. */
const NULL: Addr = 0n;
/** A pointer's size, and so a word's. */
const WORD = 8;

/** The scratch memory's address: its low and high 32 bits. */
let baseLo = 0;
let baseHi = 0;

/** The word at `p`, as a length. */
function lenGet(p: number): number {
	return view.getUint32(p, true) + view.getUint32(p + 4, true) * TWO32;
}

/** Write the word `n`, a length or a token, at `p`. */
function lenPut(p: number, n: number): void {
	view.setUint32(p, n % TWO32, true);
	view.setUint32(p + 4, Math.floor(n / TWO32), true);
}

/** The address at `p`. */
function addrGet(p: number): Addr {
	return view.getBigUint64(p, true);
}

/** Write the address `a` at `p`. */
function addrPut(p: number, a: Addr): void {
	view.setBigUint64(p, a, true);
}

/** Write at `p` the address of `at`, a place the stack took. */
function placePut(p: number, at: number): void {
	const lo = baseLo + at;
	view.setUint32(p, lo % TWO32, true);
	view.setUint32(p + 4, baseHi + Math.floor(lo / TWO32), true);
}

/**
 * A copy of the `len` bytes at `ptr`. Up to [LEND], through the scratch memory, as a native
 * call taking only numbers costs half one reading a typed array's place; past it, straight
 * into the copy, so the scratch memory, which stays committed, grows no larger than that.
 */
function lentCopy(ptr: Addr, len: number): Uint8Array<ArrayBuffer> {
	if (len > LEND) {
		const out = new Uint8Array(len);
		w.$into(out, ptr);
		return out;
	}
	const m = top;
	try {
		const at = alloc(len, 1);
		if (len > 0) w.$copy(at, ptr, len);
		return bytes.slice(at, at + len);
	} finally {
		top = m;
	}
}

/** The text of the `len` bytes of UTF-8 at `ptr`, as `decoder` decodes it, through the scratch memory as [lentCopy] copies. */
function lentText(ptr: Addr, len: number, decoder: TextDecoder): string {
	if (len > LEND) return decoder.decode(lentCopy(ptr, len));
	const m = top;
	try {
		const at = alloc(len, 1);
		w.$copy(at, ptr, len);
		return decoder.decode(bytes.subarray(at, at + len));
	} finally {
		top = m;
	}
}

/** The byte at `ptr + i`. */
function lentByte(ptr: Addr, i: number): number {
	return w.$byte(ptr, i);
}

/** Write `data` at `ptr`. */
function lentWrite(ptr: Addr, data: Uint8Array): void {
	w.$write(ptr, data);
}
// #endregion node memory

// #region node stack
/**
 * The first offset the stack takes: the words below hold how many tokens JavaScript holds,
 * which the addon reads, and keep any place's address apart from null.
 */
const BOTTOM = 16;
let top = BOTTOM;
/** How far the scratch memory is committed, which the addon extends. */
let committed = 0;
// #endregion node stack

// #region node alloc
/** `size` bytes aligned to `align`, a power of two, until the running call returns. */
export function alloc(size: number, align: number): number {
	// Not by bits, which are 32: the scratch memory is as large as wasm32's.
	const at = Math.ceil(top / align) * align;
	const end = at + size;
	if (end > committed) committed = w.$commit(end);
	top = end;
	return at;
}
// #endregion node alloc

// #region node handles
/**
 * What each object type's handles do: its name, its free's index among the addon's, and its
 * drop, if Rust checks it.
 */
export type Kind = {
	readonly name: string;
	readonly free: number;
	readonly drop?: string;
};

/** The index of a closed handle, which each native function refuses as closed. */
const CLOSED = -1;

/** The handles the garbage collector closes, of each that left the nursery undisposed. */
const collected = new FinalizationRegistry<[number, number]>(([i, free]) => {
	w.$close(i, free, true);
});

/**
 * A handle to a Rust object, by its index in the addon's table, which counts the calls
 * holding it. An owned one of a value Rust checks is Rust's to account: disposing it drops
 * the value as Rust's own drop does, failing while a call borrows it, so a call after it
 * throws. Any other throws if disposed while a call holds it. Either is freed once closed
 * and no call holds it, by the addon, or once the garbage collector finds it unreachable,
 * if it was never disposed. A lent one's free forgets its index alone.
 */
export class Handle<T extends string = string> {
	/** The index each call passes, which the addon holds for the call: [CLOSED] once closed. */
	p: number;
	/** Its index, which outlives its close until the last call holding it returns. */
	readonly i: number;
	readonly k: Kind;
	readonly owned: boolean;
	/** Its slot in the nursery, or whether it registered once it left. */
	n: number;

	constructor(p: number, k: Kind, owned: boolean) {
		this.p = p;
		this.i = p;
		this.k = k;
		this.owned = owned;
		this.n = nurse(this);
	}

	/** Leave the nursery, registering unless closed. */
	evict(): void {
		if (this.p === CLOSED) {
			this.n = UNREGISTERED;
			return;
		}
		collected.register(this, [this.i, this.owned ? this.k.free : 0], this);
		this.n = REGISTERED;
	}

	/** Release the hold [putHandle] took for a call. */
	release(): void {
		w.$release(this.i);
	}

	/** Close it as `Handle` says; again, it does nothing. */
	[Symbol.dispose](): void {
		if (this.p === CLOSED) return;
		// Dropped, so a call that still holds it fails Rust's check, and none can hold it
		// again: it is freed with its last hold.
		const checked = this.owned && this.k.drop !== undefined;
		if (checked) w[this.k.drop!](this.p);
		if (!w.$close(this.p, this.owned ? this.k.free : 0, checked)) {
			throw new Error(`${this.k.name}: a handle is already borrowed`);
		}
		this.p = CLOSED;
		if (this.n >= 0) nursery[this.n] = undefined;
		else if (this.n === REGISTERED) collected.unregister(this);
		this.n = UNREGISTERED;
	}
}
// #endregion node handles

// #region node lends
/** Whether bytes of `len` a call returned are lent: past [LEND], as the addon tells V8 what they hold. */
function lends(len: number): boolean {
	if (len <= LEND) return false;
	w.$external(len);
	return true;
}

/** A copy of the bytes from `from` up to `to` of the `len` at `ptr`, as `Uint8Array`'s `slice`: of those alone. */
function lentSlice(ptr: Addr, len: number, from?: number, to?: number): Uint8Array<ArrayBuffer> {
	// `slice`'s relative index: from the end where negative, clamped to the bytes.
	const at = (index: number | undefined, unset: number): number => {
		if (index === undefined) return unset;
		const i = Math.trunc(index) || 0;
		return i < 0 ? Math.max(len + i, 0) : Math.min(i, len);
	};
	const start = at(from, 0);
	return lentCopy(ptr + BigInt(start), Math.max(at(to, len) - start, 0));
}

/** Bytes of `len` [lends] lent are freed. */
function unlends(len: number): void {
	w.$external(-len);
}
// #endregion node lends

// #region node hooks
/** Tell the addon how many tokens JavaScript holds, once that changes, as a call it makes reads. */
function relayed(): void {
	if (loaded) view.setUint32(0, tokens.size, true);
}

/** Keep the event loop alive while a call into Rust pends, which another thread may wake. */
function pends(pending: boolean): void {
	w.$pends(pending);
}
// #endregion node hooks

// #region node natives
/** The library's target, by its index in the API's targets. */
let target = 0n;

/** Throw unless the library's target has `name`, which the targets of `mask` have. */
export function on(mask: bigint, name: string): void {
	if (((mask >> target) & 1n) === 0n) throw new Error(`${name}: the library's target lacks it`);
}

/** The word at `p`: a list's length, at `p + 8` of the list. */
export function lenAt(p: number): number {
	return lenGet(p);
}

/** A copy on the stack of the `n` bytes the pointer at `p` points to, aligned to `align`. */
export function mirror(p: number, n: number, align: number): number {
	const at = alloc(Math.max(n, 1), align);
	if (n > 0) w.$copy(at, addrGet(p), n);
	return at;
}

/** An array the generated code reads a list's block into: a typed array, which `$into` takes, never a `DataView`. */
type Block =
	| Uint8Array
	| Int8Array
	| Uint16Array
	| Int16Array
	| Uint32Array
	| Int32Array
	| BigUint64Array
	| BigInt64Array
	| Float32Array
	| Float64Array;

/** Copy into `out` the bytes the pointer at `p` points to, as many as it holds. */
export function readInto(p: number, out: Block): void {
	if (out.byteLength > 0) w.$into(out, addrGet(p));
}

/** Write at `p` the list of `n` elements at `at`, a place the stack took, or `None` at 0. */
export function putList(p: number, at: number, n: number): void {
	if (at === 0) addrPut(p, NULL);
	else placePut(p, at);
	lenPut(p + WORD, n);
	lenPut(p + 2 * WORD, 0);
}

/** The index of the handle at `p`, which the addon's table holds until it is freed. */
export function handleAt(p: number): number {
	return w.$handle(p);
}

/** Write at `p` the handle `h`, or null for none, held until the running call returns: a closed one throws. */
export function putHandle(p: number, h: Handle | undefined): void {
	if (h === undefined) return addrPut(p, NULL);
	if (h.p === CLOSED) throw new Error(`${h.k.name} is closed`);
	w.$put(p, h.p);
	held.push(h);
}
// #endregion node natives

// #region node load
/** What a binding's `load` passes: its C prefix and the glue's namespace, its contract and the targets it serves, and its imports. */
export type Glue = {
	readonly prefix: string;
	readonly space: string;
	readonly file: string;
	readonly contract: bigint;
	readonly targets: readonly number[];
	readonly imports: Record<string, (...args: never[]) => unknown>;
};

/** What `load` needs of Node's modules, which Node, Bun, and Deno each have. */
type Process = {
	getBuiltinModule(id: "node:module"): { createRequire(from: string | URL): (id: string) => unknown };
	getBuiltinModule(id: "node:url"): { fileURLToPath(url: URL): string };
	getBuiltinModule(id: "node:path"): { resolve(path: string): string };
};

/**
 * Load the addon at `path`, a URL or a path from the working directory, checking it is the
 * build `glue` was generated with, for a target it serves, before any call. The first
 * environment to load it keeps it for the process's life: any other's load throws, even
 * once the first has exited.
 */
export async function load(path: string | URL, glue: Glue): Promise<void> {
	if (loading || loaded) throw new Error(`${glue.file}: the addon is loaded already`);
	loading = true;
	try {
		instantiate(path, glue);
		loaded = true;
		view.setUint32(0, tokens.size, true);
	} finally {
		loading = false;
	}
}

let loading = false;
let loaded = false;

function instantiate(path: string | URL, glue: Glue): void {
	const process = (globalThis as unknown as { process: Process }).process;
	// A relative path from the working directory, as a caller means it, not from this module,
	// as `require` reads one.
	const file =
		path instanceof URL ? process.getBuiltinModule("node:url").fileURLToPath(path) : process.getBuiltinModule("node:path").resolve(path);
	const exports = process.getBuiltinModule("node:module").createRequire(import.meta.url)(file) as Exports;
	if (exports[`${glue.prefix}_contract`]() !== BigInt.asIntN(64, glue.contract)) {
		throw new Error(`${glue.file} and the addon are from different builds`);
	}
	const built = exports[`${glue.prefix}_target`]() as number;
	if (!glue.targets.includes(built)) {
		throw new Error(`${glue.file}: the addon was built for a target the binding does not serve`);
	}
	const imports = {
		...rust2(glue.imports as Record<string, (...args: number[]) => unknown>),
		// The event loop's run of what Rust's other threads queued, which the addon calls
		// around it to learn whether the environment is torn down.
		rust2_idle(_entering: number) {},
		// What the addon makes of a 64-bit integer that is no safe one, or throws for.
		rust2_u64: u64Big,
		rust2_i64: i64Big,
	};
	const memory = exports.$init(imports) as ArrayBuffer;
	const base = exports.$base() as bigint;
	baseLo = Number(base & 0xffffffffn);
	baseHi = Number(base >> 32n);
	view = new DataView(memory);
	bytes = new Uint8Array(memory);
	prefix = glue.prefix;
	space = glue.space;
	target = BigInt(built);
	w = exports;
	committed = 0;
	top = BOTTOM;
	const p = alloc(6 * WORD, WORD);
	exports.$fns(p);
	const names = ["wake", "wakerDrop", "free", "poll", "read", "seek"] as const;
	names.forEach((name, i) => {
		fn[name] = addrGet(p + WORD * i);
	});
	top = BOTTOM;
}
// #endregion node load
