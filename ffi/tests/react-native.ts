// The api fixture's React Native test: tests/api.ts's script and cases through the React
// Native binding, which tests/fixtures.rs bundles with a copy of api.ts importing it. It
// runs on Hermes, in tests/rn_host.cpp off a device, whose `process` and `print` stand in
// for Node's, and exits as api.ts's `main` does: with an argument, the case of that name.

import * as api from "./react-native/index.ts";
import * as check from "./api.ts";
import * as Channel from "./react-native/Channel.ts";
import * as Epilogue from "./react-native/Epilogue.ts";
import * as rt from "./react-native/rt.ts";
import * as Spawned from "./react-native/Spawned.ts";
import { TurboModuleRegistry } from "react-native";

declare const process: { argv: string[]; exit(code: number): never; stderr: { write(text: string): boolean } };

/**
 * A thread that takes the runtime while the JavaScript thread waits in a call into Rust gives
 * it back before that call returns, even while its own JavaScript waits in a call into Rust:
 * two threads' JavaScript never interleaves. The spawned thread's task takes the runtime
 * during one of the JavaScript thread's short calls, then makes a long one of its own.
 */
function interleaved(): void {
	let inside = false;
	let done = false;
	const spawned = Spawned.new(() => {
		inside = true;
		api.filter(new Uint8Array(1000), () => true);
		inside = false;
		done = true;
		return 0;
	});
	while (!done) {
		api.filter(new Uint8Array(10), () => true);
		if (inside) throw new Error("a call into Rust returned while another thread's JavaScript was in one");
	}
	Spawned.join(spawned);
}

/**
 * The runtime's natives refuse a value that is not theirs with a TypeError, rather than read
 * it as their own: a handle's dispose, a waker's, and Rust's bytes' free, whatever `this` or
 * argument JavaScript passes.
 */
async function foreign(): Promise<void> {
	const channel = Channel.new();
	Channel.send(channel, new Uint8Array(4097));
	// Past 4 KiB, Rust's own buffer, lent.
	const lent = await Channel.recv(channel);
	const dispose = rt.Handle.prototype[Symbol.dispose];
	const cases: [string, () => unknown][] = [
		["an object disposed as a handle", () => Reflect.apply(dispose, {}, [])],
		["a handle's prototype disposed", () => Reflect.apply(dispose, Object.create(rt.Handle.prototype), [])],
		["bytes disposed as a handle", () => Reflect.apply(dispose, lent, [])],
		["a number disposed", () => rt.n.$dispose(1)],
		["a handle woken as a waker", () => rt.n.$wake(channel)],
		["a handle dropped as a waker", () => rt.n.$wakerDrop(channel)],
		["a handle freed as bytes", () => rt.n.$bytesFree(channel)],
		["nothing freed as bytes", () => rt.n.$bytesFree()],
	];
	for (const [what, f] of cases) {
		try {
			f();
		} catch (e) {
			if (e instanceof TypeError) continue;
			throw e;
		}
		throw new Error(`${what} passed`);
	}
	if (lent.at(4096) !== 0) throw new Error("bytes refused as a handle were touched");
	lent[Symbol.dispose]();
	channel[Symbol.dispose]();
}

/**
 * Bytes' `slice` reads Rust's buffer through nothing a caller can replace, so no caller's
 * code sees a view of it or frees it under the read, and bounds it as `Uint8Array`'s
 * `slice` would.
 */
async function sliced(): Promise<void> {
	const channel = Channel.new();
	const sent = new Uint8Array(4097).map((_, i) => i);
	const recv = async (length: number) => {
		Channel.send(channel, sent.subarray(0, length));
		return Channel.recv(channel);
	};
	// Past 4 KiB, Rust's own buffer, lent; up to it, a copy.
	const lent = await recv(4097);
	const copied = await recv(4096);
	const typed = Object.getPrototypeOf(Uint8Array.prototype);
	const slice = typed.slice;
	const set = typed.set;
	const reached = function (this: Uint8Array): never {
		throw new Error(`a replaced method reached a view of ${this.length} bytes`);
	};
	typed.slice = reached;
	typed.set = reached;
	try {
		const bounds = [undefined, 0, 1, 4095, 4096, 4097, 4098, -1, -4096, -4097, -4098, 2.7, -2.7, NaN, Infinity, -Infinity];
		for (const [bytes, length] of [[lent, 4097], [copied, 4096]] as const) {
			for (const start of bounds) {
				for (const end of bounds) {
					const got = bytes.slice(start, end);
					const want = Reflect.apply(slice, sent.subarray(0, length), [start, end]) as Uint8Array;
					if (got.length !== want.length || got.some((b, i) => b !== want[i])) {
						throw new Error(`${length} bytes' slice(${start}, ${end}) read [${got}], not [${want}]`);
					}
				}
			}
		}
	} finally {
		typed.slice = slice;
		typed.set = set;
	}
	lent[Symbol.dispose]();
	copied[Symbol.dispose]();

	// Each step a slice could take that a caller can replace disposes the bytes when reached:
	// the slice then fails as closed, never reading what was freed under it.
	const length = Object.getOwnPropertyDescriptor(typed, "length")!;
	const hooks: [string, (dispose: () => void) => () => void][] = [
		...(["max", "min", "trunc"] as const).map((key): [string, (dispose: () => void) => () => void] => [
			`Math.${key}`,
			(dispose) => {
				const original = Math[key];
				Math[key] = (...args: number[]) => (dispose(), original(...args));
				return () => void (Math[key] = original);
			},
		]),
		[
			"a typed array's length",
			(dispose) => {
				Object.defineProperty(typed, "length", {
					configurable: true,
					get(this: Uint8Array) {
						dispose();
						return Reflect.apply(length.get!, this, []);
					},
				});
				return () => Object.defineProperty(typed, "length", length);
			},
		],
	];
	for (const [what, hook] of hooks) {
		for (const [start, end] of [[0, 80], [-80, undefined]]) {
			const bytes = await recv(4097);
			let disposed = false;
			const restore = hook(() => {
				if (!disposed) bytes[Symbol.dispose]();
				disposed = true;
			});
			try {
				bytes.slice(start, end);
				if (disposed) throw new Error(`${what} disposed bytes that slice(${start}, ${end}) then read`);
			} catch (e) {
				if (!disposed || !(e instanceof Error) || e.message !== "Bytes is closed") throw e;
			} finally {
				restore();
			}
		}
	}
	channel[Symbol.dispose]();
}

/**
 * A reader's read crosses as the bytes it wrote into its chunk, whatever the chunk's
 * properties claim once it returns: a `buffer` redefined to other bytes is never read.
 */
function faked(): void {
	const data = Uint8Array.of(1, 2, 3, 4, 5, 6, 7, 8);
	let at = 0;
	const reader: rt.Reader = {
		read(chunk: Uint8Array): number {
			const n = Math.min(chunk.length, data.length - at);
			chunk.set(data.subarray(at, at + n));
			at += n;
			Object.defineProperty(chunk, "buffer", { value: new Uint8Array(chunk.length).fill(0xee).buffer });
			return n;
		},
		seek(offset: rt.u64 | rt.i64, from: rt.SeekFrom): rt.u64 {
			const by = from === "Start" ? rt.u64ToBig(offset as rt.u64) : rt.i64ToBig(offset as rt.i64);
			at = Number((from === "Start" ? 0n : from === "Current" ? BigInt(at) : BigInt(data.length)) + by);
			return rt.u64FromBig(BigInt(at));
		},
	};
	const r = api.tail(reader, 8);
	if (!r.ok) throw new Error(`tail failed: ${r.error}`);
	const read = Array.from(r.value[0]).join(" ");
	if (read !== "1 2 3 4 5 6 7 8") throw new Error(`a faked buffer read ${read}`);
}

/**
 * A typed array a call takes is Rust's own while it runs: a closure the call runs writes
 * the caller's array, never what Rust reads, as Rust holds it as a slice no one writes.
 */
function written(): void {
	const payload = Uint8Array.of(1, 2, 3);
	const seen = api.visitChunks(payload, 1, (chunk) => {
		payload[2] = 9;
		return chunk?.[0] ?? 0;
	});
	if (Array.from(seen).join(" ") !== "1 2 3 0") throw new Error(`a write during the call reached Rust: ${seen.join(" ")}`);
}

/**
 * A closure Rust keeps and a promise Rust polls later are checked as the call converts them:
 * one that is not callable, or a thenable whose `then` is not, throws a TypeError there,
 * rather than abort once Rust calls or polls it.
 */
function refused(): void {
	const cases: [string, () => unknown][] = [
		["an object kept as a closure", () => api.hook({} as unknown as () => void)],
		["a thenable whose then is no function", () => api.after({ then: 1 } as unknown as Promise<unknown>, 1)],
	];
	for (const [what, f] of cases) {
		let made: unknown;
		try {
			made = f();
		} catch (e) {
			if (e instanceof TypeError) continue;
			throw e;
		}
		// Thrown as the call converts it, not rejected later.
		if (made instanceof Promise) made.catch(() => {});
		throw new Error(`${what} passed`);
	}
}

/**
 * Bytes lent from Rust's buffer are made with no JavaScript a caller replaced: a global
 * `Uint8Array` or an `ArrayBuffer` `byteLength` getter never sees Rust's buffer, so nothing
 * keeps a view of it past its free.
 */
async function replaced(): Promise<void> {
	const channel = Channel.new();
	Channel.send(channel, new Uint8Array(4097));
	const seen: unknown[] = [];
	const g = globalThis as { Uint8Array: Uint8ArrayConstructor };
	const was = g.Uint8Array;
	const length = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength")!;
	g.Uint8Array = new Proxy(was, {
		construct(target, args) {
			seen.push(args[0]);
			return Reflect.construct(target, args);
		},
	});
	Object.defineProperty(ArrayBuffer.prototype, "byteLength", {
		configurable: true,
		get(this: ArrayBuffer) {
			seen.push(this);
			return length.get!.call(this);
		},
	});
	let lent: rt.Bytes;
	try {
		// Past 4 KiB, Rust's own buffer, lent.
		lent = await Channel.recv(channel);
	} finally {
		g.Uint8Array = was;
		Object.defineProperty(ArrayBuffer.prototype, "byteLength", length);
	}
	if (seen.length !== 0) throw new Error(`a replaced global saw ${seen.length} values as Rust's bytes were made`);
	if (lent.length !== 4097 || lent.at(4096) !== 0) throw new Error(`lent bytes read ${lent.length}`);
	lent[Symbol.dispose]();
	channel[Symbol.dispose]();
}

/**
 * A call given back to Rust as a future is cancelled as Rust drops it, as the web's is: a
 * receive whose wait timed out takes nothing sent after it.
 */
async function cancelled(): Promise<void> {
	const channel = Channel.new();
	const recv = Channel.recv(channel);
	const timed = await api.within(0, recv);
	if (timed.ok) throw new Error("a receive on an empty channel finished");
	Channel.send(channel, Uint8Array.of(7));
	const outcome = await recv.then(
		() => "received",
		(e: unknown) => (e instanceof rt.Cancelled ? "cancelled" : e),
	);
	if (outcome !== "cancelled") throw new Error(`a timed-out receive ended ${String(outcome)}`);
	const got = await Channel.recv(channel);
	if (got.length !== 1 || got.at(0) !== 7) throw new Error("what was sent after a timed-out receive was lost");
	got[Symbol.dispose]();
	channel[Symbol.dispose]();
}

/**
 * A waker that JavaScript its own wake runs disposes is dropped in Rust once that wake
 * returns, not under it, which still borrows it.
 */
function woken(): void {
	const events: string[] = [];
	let waker: rt.Waker | undefined;
	waker = api.logWaker({
		log(level: number): boolean {
			events.push(`log ${level}`);
			if (level === 1) {
				waker![Symbol.dispose]();
				events.push("disposed");
			}
			return true;
		},
	});
	waker.wake();
	events.push("returned");
	const seen = events.join(", ");
	if (seen !== "log 1, disposed, log 0, returned") throw new Error(`a waker disposed in its wake: ${seen}`);
}

/**
 * A value the collector frees is dropped on a posted job, so a drop that calls into
 * JavaScript runs outside the collection. One still held as the runtime goes is leaked, as
 * no JavaScript is left to call: the host runs this twice with `--reload`.
 */
async function collected(): Promise<void> {
	const lines: string[] = [];
	const log = {
		log(_level: number, line: string): boolean {
			lines.push(line);
			return true;
		},
	};
	(() => Epilogue.new$(log, 1))();
	await check.collect(() => lines.length > 0, "the collector drops an epilogue nobody disposed");
	if (lines.join(", ") !== "closed") throw new Error(`a collected epilogue logged ${lines.join(", ")}`);
	(globalThis as { kept?: unknown }).kept = Epilogue.new$(log, 2);
}

/** What installs the module: the host's global off a device, else the TurboModule. */
function installer(): { install(kit: object): boolean } {
	const global = (globalThis as Record<string, unknown>).__rust2_api as { install(kit: object): boolean } | undefined;
	return global ?? TurboModuleRegistry.getEnforcing<{ install(kit: object): boolean } & object>("Rust2Api");
}

/** The module installs once in a runtime: again, with any kit, it throws. */
function reinstalled(): void {
	try {
		installer().install({ natives: {} });
	} catch (e) {
		if (e instanceof Error && e.message.includes("installed in this runtime already")) return;
		throw e;
	}
	throw new Error("the module installed twice");
}

/**
 * A kit forged before the binding loads, whose helpers return what the module cannot read,
 * makes each call that reads one throw a TypeError, rather than read it as what it should be.
 */
function forged(): void {
	const natives: Record<string, (...args: unknown[]) => unknown> = {};
	const no = () => 0;
	installer().install({
		...Object.fromEntries(["pending", "bytes", "int", "char", "u128Words", "f32Bits", "bytesCopy", "opaqueMade", "opaqueBytes", "isCancelled", "cancelCall", "cancelled", "ready"].map((k) => [k, no])),
		typedTag: () => "Int32Array",
		u64Halves: () => 0,
		i64Halves: () => [2 ** 40, 0],
		f64Bits: () => [1, 2, 3],
		Uint8Array,
		typedArrays: {},
		RangeError,
		TypeError,
		Error,
		None: undefined,
		Handle: {},
		Waker: {},
		natives,
	});
	const cases: [string, () => unknown][] = [
		["a u64's halves as a number", () => natives.same_u64({ hi: 1, lo: 0 })],
		["an i64's high half past 32 bits", () => natives.same_isize({ hi: -1, lo: 0 })],
		["an f64's three halves", () => natives.same({ hi: 0x7ff00000, lo: 1 }, 0)],
		["an object its kit calls an Int32Array", () => natives.sum_lists({}, undefined, undefined)],
	];
	for (const [what, f] of cases) {
		try {
			f();
		} catch (e) {
			if (e instanceof TypeError) continue;
			throw e;
		}
		throw new Error(`${what} passed`);
	}
}

/** A 64-bit target's `usize` constant crosses whole, as its parameters do. */
function word(): void {
	if (rt.u64ToBig(api.WORD) !== 0xffffffffffffffffn) throw new Error(`usize::MAX is ${String(api.WORD)}`);
}

const cases: Record<string, () => void | Promise<void>> = {
	word,
	interleaved,
	foreign,
	sliced,
	faked,
	written,
	refused,
	replaced,
	cancelled,
	woken,
	collected,
	reinstalled,
	futures: check.futures,
	owned: check.owned,
	closed: check.closedObjects,
	bytes: check.lent,
	bounds: check.bounds,
	checks: check.checks,
};

async function main(): Promise<void> {
	const which = process.argv[2];
	// Before the binding loads, which it then refuses to.
	if (which === "forged") return forged();
	await api.load();
	if (which === undefined) return check.script();
	return (cases[which] ?? (() => check.broken(which)))();
}

main().then(
	() => process.exit(0),
	(e: unknown) => {
		// The host's `process.exit` throws this to stop the script, having exited already.
		if (e instanceof Error && e.message === "__exited") return;
		process.stderr.write(`uncaught: ${e instanceof Error ? (e.stack ?? e.message) : String(e)}\n`);
		process.exit(1);
	},
);
