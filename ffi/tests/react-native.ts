// The api fixture's React Native test: tests/api.ts's script and cases through the React
// Native binding, which tests/fixtures.rs bundles with a copy of api.ts importing it. It
// runs on Hermes, in tests/rn_host.cpp off a device, whose `process` and `print` stand in
// for Node's, and exits as api.ts's `main` does: with an argument, the case of that name.

import * as api from "./react-native/index.ts";
import * as check from "./api.ts";
import * as Channel from "./react-native/Channel.ts";
import * as rt from "./react-native/rt.ts";
import * as Spawned from "./react-native/Spawned.ts";

declare const process: { argv: string[]; exit(code: number): never; stderr: { write(text: string): boolean } };

/**
 * A typed array crosses as the engine holds it, whatever its own properties or prototype
 * claim: a shadowed length reads no further than its bytes, and another kind relabeled is
 * refused, since the JSI module reads its memory as the kind it claims.
 */
function typed(): void {
	const shadowed = Uint8Array.of(0x61);
	Object.defineProperties(shadowed, { byteLength: { value: 4096 }, byteOffset: { value: 8 }, length: { value: 4096 } });
	const read = api.prefixed(shadowed, undefined).slice();
	if (read.length !== 1 || read[0] !== 0x61) throw new Error(`a shadowed length read ${read.length} bytes`);
	const relabeled = new Int8Array(3);
	Object.setPrototypeOf(relabeled, Int32Array.prototype);
	try {
		api.sumLists(relabeled as unknown as Int32Array, undefined, undefined);
	} catch (e) {
		if (e instanceof TypeError) return;
		throw e;
	}
	throw new Error("a relabeled typed array crossed");
}

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
	Channel.send(channel, new Uint8Array(100));
	// Past 64 bytes, Rust's own buffer, lent.
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
	if (lent.at(99) !== 0) throw new Error("bytes refused as a handle were touched");
	lent[Symbol.dispose]();
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

/** A 64-bit target's `usize` constant crosses whole, as its parameters do. */
function word(): void {
	if (rt.u64ToBig(api.WORD) !== 0xffffffffffffffffn) throw new Error(`usize::MAX is ${String(api.WORD)}`);
}

const cases: Record<string, () => void | Promise<void>> = {
	typed,
	word,
	interleaved,
	foreign,
	faked,
	futures: check.futures,
	owned: check.owned,
	closed: check.closedObjects,
	bytes: check.lent,
	bounds: check.bounds,
	checks: check.checks,
};

async function main(): Promise<void> {
	await api.load();
	const which = process.argv[2];
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
