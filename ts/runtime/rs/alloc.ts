// `std::alloc`, for rust2-ts output: a `Layout`, and memory from `alloc` that a cast of its
// pointer to one to `T` makes a JS array of `T`s, as many as fit. `rs.ts` re-exports all of it.
//
// JS keeps values, not bytes, so the memory is untyped until that cast, and translation
// refuses a read or write of its bytes, and a pointer to them leaving the body that made
// it. `realloc` keeps the values in a new array, and `dealloc` lets the array go; a pointer
// into it after either reads what Rust leaves undefined.

import { Abort, Err, type Fmt, type FmtResult, type Formatter, fmtDebugStructFields, fmtPad, fmtWriteFmt, Ok, type Place, type Result, violated } from "../rs.ts";
import { AtIndex } from "./collections.ts";
import { type u32, u32Debug, type usize, usizeDebug } from "./int.ts";

/** `core::alloc::Layout`: a size, and an alignment that is a power of two. */
export interface Layout {
	readonly size: usize;
	readonly align: usize;
}

/** `core::alloc::LayoutError`, which holds nothing. */
export class LayoutError {}

/** The one `LayoutError`. */
export const LAYOUT_ERROR = /* @__PURE__ */ new LayoutError();

/** The largest size a layout of `align` may have: wasm32's `isize::MAX`, less the padding up to `align`. */
function maxSize(align: number): number {
	return 0x7fffffff - (align - 1);
}

/** `Layout::from_size_align`: an error unless `align` is a power of two and `size` rounded up to it fits in `isize`. */
export function layoutFromSizeAlign(size: usize, align: usize): Result<Layout, LayoutError> {
	if (align === 0 || (align & (align - 1)) !== 0 || size > maxSize(align)) return Err(LAYOUT_ERROR);
	return Ok({ size, align });
}

/** `Layout::array::<T>(n)`, of `T`s of `size` bytes aligned to `align`. */
export function layoutArray(n: usize, size: usize, align: usize): Result<Layout, LayoutError> {
	if (size !== 0 && n > Math.floor(maxSize(align) / size)) return Err(LAYOUT_ERROR);
	return Ok({ size: size * n, align });
}

/** `Debug` of a `Layout`, as std's: its size by the formatter's flags, and its alignment also as a power of two, by none. */
export function fmtLayoutDebug(l: Layout, f: Formatter): FmtResult {
	return fmtDebugStructFields(
		f,
		"Layout",
		["size", "align"],
		[(f) => usizeDebug(l.size, f), (f) => fmtWriteFmt(f, [(f) => usizeDebug(l.align, f), " (1 << ", (f) => u32Debug((31 - Math.clz32(l.align)) as u32, f), ")"])],
	);
}

/** `Debug` of a `LayoutError`. */
export function fmtLayoutErrorDebug(_: LayoutError, f: Formatter): FmtResult {
	return f.out.writeStr("LayoutError");
}

/** `Display` of a `LayoutError`. */
export function fmtLayoutErrorDisplay(_: LayoutError, f: Formatter): FmtResult {
	return f.out.writeStr("invalid parameters to Layout::from_size_align");
}

/**
 * `alloc::collections::TryReserveError`, never changed once made: why `try_reserve` failed,
 * the capacity past what the collection holds, or no `layout` for it.
 */
export class TryReserveError {
	/** The layout whose allocation failed, or none where the capacity overflowed. */
	readonly layout: Layout | undefined;

	constructor(layout?: Layout) {
		this.layout = layout;
	}
}

/** `Err` of the capacity's overflow, which every `try_reserve` shares. */
export const CAPACITY_OVERFLOW: Result<never, TryReserveError> = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Err(/* @__PURE__ */ new TryReserveError()));

/** `Err` of the allocation of `size` bytes, if `e` is what JS throws for one it can't make; else `e` again. */
export function allocError(e: unknown, size: usize): Result<never, TryReserveError> {
	if (e instanceof RangeError) return Err(new TryReserveError({ size, align: 1 }));
	throw e;
}

/** `PartialEq` of `TryReserveError`s. */
export function tryReserveErrorEq(a: TryReserveError, b: TryReserveError): boolean {
	if (a.layout === undefined || b.layout === undefined) return a.layout === b.layout;
	return a.layout.size === b.layout.size && a.layout.align === b.layout.align;
}

/** `Debug` of a `TryReserveError`, derived as std's is, of its `TryReserveErrorKind`. */
export function fmtTryReserveErrorDebug(e: TryReserveError, f: Formatter): FmtResult {
	const layout = e.layout;
	const kind: Fmt =
		layout === undefined
			? (f) => f.out.writeStr("CapacityOverflow")
			: (f) => fmtDebugStructFields(f, "AllocError", ["layout", "non_exhaustive"], [(f) => fmtLayoutDebug(layout, f), (f) => fmtPad(f, "()")]);
	return fmtDebugStructFields(f, "TryReserveError", ["kind"], [kind]);
}

/** `Display` of a `TryReserveError`, unpadded as std's. */
export function fmtTryReserveErrorDisplay(e: TryReserveError, f: Formatter): FmtResult {
	const reason =
		e.layout === undefined ? " because the computed capacity exceeded the collection's maximum" : " because the memory allocator returned an error";
	const r = f.out.writeStr("memory allocation failed");
	return r.ok ? f.out.writeStr(reason) : r;
}

/**
 * Memory from `alloc`: `size` bytes, which hold the values of type `type`, `elem` bytes
 * each, in `array` once a cast of a pointer to it types it. A pointer to its bytes is the block.
 */
export class Block implements Place<number> {
	readonly size: number;
	array: unknown[] | undefined;
	elem = 0;
	type = "";

	constructor(size: number) {
		this.size = size;
	}

	get v(): number {
		return bytes();
	}

	set v(_: number) {
		bytes();
	}
}

function bytes(): never {
	throw new Error("rust2-ts: a read or write of the bytes of memory from `alloc`, which translation refuses");
}

/** The block each array a cast of one's pointer made is in. */
const BLOCKS = /* @__PURE__ */ new WeakMap<unknown[], Block>();

/** `alloc::alloc`: `layout`'s bytes, untyped until a cast of the pointer to them. */
export function alloc(layout: Layout): Block {
	return new Block(layout.size);
}

/**
 * The pointer `p` to the bytes of a block, cast to one to the values of Rust type `type`,
 * `elem` bytes each, it holds: the first of a new array of as many as fit, or of the array
 * it holds already, of values of that type.
 */
export function allocAs<T>(p: Place<number>, elem: number, type: string): Place<T> {
	// Translation casts only a pointer the body shows `alloc` or `realloc` returned.
	if (!(p instanceof Block)) {
		throw new Error("rust2-ts: a pointer cast from one to bytes to one to values, which isn't memory from `alloc`");
	}
	// As many as fit, past which bytes nothing can read are left over.
	if (!p.array) {
		p.array = new Array(Math.floor(p.size / elem));
		p.elem = elem;
		p.type = type;
		BLOCKS.set(p.array, p);
	}
	// Translation refuses casting memory from one `alloc` to two types, and from `realloc` to another than its argument's.
	if (p.type !== type) throw new Error(`rust2-ts: memory from \`alloc\` holding \`${p.type}\`s, as \`${type}\`s`);
	return new AtIndex(p.array as T[], 0);
}

/**
 * The pointer `p` to the first of the values of Rust type `type`, `elem` bytes each, that
 * memory from `alloc`, or a `Vec`, holds, cast to one to its bytes: the block, which only
 * `dealloc` and `realloc` take.
 */
export function allocBytes(p: Place<unknown>, elem: number, type: string): Block {
	if (!(p instanceof AtIndex) || p.index !== 0 || !Array.isArray(p.array)) {
		throw new Error("rust2-ts: a pointer cast to one to bytes, which isn't to the start of memory from `alloc` or a `Vec`'s");
	}
	const array = p.array as unknown[];
	let block = BLOCKS.get(array);
	// A `Vec`'s, which `Layout::array` laid out as one block's is.
	if (!block) {
		block = new Block(array.length * elem);
		block.array = array;
		block.elem = elem;
		block.type = type;
		BLOCKS.set(array, block);
	}
	if (block.type !== type) throw new Error(`rust2-ts: memory from \`alloc\` holding \`${block.type}\`s, as \`${type}\`s`);
	return block;
}

/** The block the pointer `p` to bytes, which `dealloc` or `realloc` takes, is. */
function blockOf(p: Place<number>): Block {
	if (!(p instanceof Block)) violated("dealloc and realloc require memory that alloc gave");
	return p;
}

/** `alloc::dealloc` of the block `p`, whose values JS frees once nothing holds them. */
export function dealloc(p: Place<number>): void {
	const block = blockOf(p);
	if (block.array) BLOCKS.delete(block.array);
	block.array = undefined;
}

/**
 * `alloc::realloc` of the block `p` to `size` bytes: a new block holding the values the old one
 * did, as many as fit, and holes past them, which Rust leaves uninitialized.
 */
export function realloc(p: Place<number>, size: usize): Block {
	const block = blockOf(p);
	const next = new Block(size);
	const array = block.array;
	if (array) {
		const n = Math.floor(size / block.elem);
		next.array = array.slice(0, n);
		next.array.length = n;
		next.elem = block.elem;
		next.type = block.type;
		BLOCKS.set(next.array, next);
		dealloc(block);
	}
	return next;
}

/** `alloc::handle_alloc_error`, which wasm32's std aborts by, running no panic hook. */
export function handleAllocError(layout: Layout): never {
	throw new Abort(`memory allocation of ${layout.size} bytes failed`);
}
