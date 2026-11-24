// Node's regions of rt.ts: each `// #region node <name>` stands in for rt.ts's region
// `// #region web <name>`, which rust2-ffi replaces as it writes Node's runtime. The rest
// of rt.ts, and so the generated TypeScript, is the web's: values lower into memory the
// addon lends, its scratch memory, as they lower into the module's, but at the native C
// ABI's layouts, whose pointers and `usize` are 64 bits. A pointer the TypeScript reads is
// Rust's, which only the addon's natives read through.

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

/** The text of the `len` bytes of UTF-8 at `ptr`, as `decoder` decodes it. */
function lentText(ptr: Addr, len: number, decoder: TextDecoder): string {
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
/** Whether a handle Rust lends is freed too, through [FORGET]: its index in the addon's table. */
const FORGETS = true;
/** What frees a lent handle's index. */
const FORGET = "$forget";
// #endregion node handles

// #region node lends
/** Whether bytes of `len` a call returned are lent: past [LEND], as the addon tells V8 what they hold. */
function lends(len: number): boolean {
	if (len <= LEND) return false;
	w.$external(len);
	return true;
}

/** A copy of the bytes from `from` up to `to` of the `len` at `ptr`, as `Uint8Array`'s `slice`. */
function lentSlice(ptr: Addr, len: number, from?: number, to?: number): Uint8Array<ArrayBuffer> {
	const all = lentCopy(ptr, len);
	return from === undefined && to === undefined ? all : all.slice(from, to);
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

/** The length of the list at `p - 8`, the word after its pointer. */
export function lenAt(p: number): number {
	return lenGet(p);
}

/** A copy on the stack of the `n` bytes the pointer at `p` points to, aligned to `align`. */
export function mirror(p: number, n: number, align: number): number {
	const at = alloc(Math.max(n, 1), align);
	if (n > 0) w.$copy(at, addrGet(p), n);
	return at;
}

/** Copy into `out` the bytes the pointer at `p` points to, as many as it holds. */
export function readInto(p: number, out: ArrayBufferView): void {
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

/** Write at `p` the handle of index `h`. */
export function putHandle(p: number, h: number): void {
	w.$put(p, h);
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
};

/**
 * Load the addon at `path`, checking it is the build `glue` was generated with, for a
 * target it serves, before any call. Its runtime serves one at a time: a worker's load
 * while another holds it throws.
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
	const file = path instanceof URL ? process.getBuiltinModule("node:url").fileURLToPath(path) : path;
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
		// The event loop's run of what Rust's other threads queued counts as a call into Rust,
		// as each it runs reaches JavaScript as one from Rust does.
		rust2_idle(entering: number) {
			s.d += entering;
		},
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
