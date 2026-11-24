// The api fixture's Node test: tests/api.ts's script and cases through Node's binding, over
// the addon the crate's `node` feature builds, which tests/fixtures.rs runs with a copy of
// api.ts importing it, on Node, Bun, and Deno, from the directory holding node/. With an
// argument, it runs the case of that name, `threads` among them: what Rust calls from its
// other threads, which the web's module has none of.

import * as api from "./node/index.ts";
// A second copy of the binding, as its `.js` beside its `.ts` would be.
import * as copy from "./copy/index.ts";
import * as check from "./api.ts";
import * as rt from "./node/rt.ts";
import * as Channel from "./node/Channel.ts";
import * as Counter from "./node/Counter.ts";
import type { Log } from "./node/Log.ts";
import * as Logger from "./node/Logger.ts";
import * as LogThread from "./node/LogThread.ts";
import * as Spawned from "./node/Spawned.ts";

declare const process: {
	readonly argv: string[];
	exit(code: number): never;
	readonly stderr: { write(text: string): boolean };
};

function ok(holds: boolean, what: string): void {
	if (!holds) {
		process.stderr.write(`failed: ${what}\n`);
		process.exit(1);
	}
}

/** Let the event loop run what is ready. */
const step = () => new Promise<void>((resolve) => setTimeout(resolve, 0));

/** Run the event loop until `done()`, failing after a deadline. */
async function until(done: () => boolean, what: string): Promise<void> {
	const deadline = Date.now() + 5000;
	while (!done()) {
		ok(Date.now() < deadline, what);
		await step();
	}
}

/** `promise`'s value, failing past a deadline rather than waiting for a wake that never lands. */
async function within<T>(promise: Promise<T>, what: string): Promise<T> {
	let timer: ReturnType<typeof setTimeout> | undefined;
	const late = new Promise<never>((_, reject) => {
		timer = setTimeout(() => reject(new Error(`timed out: ${what}`)), 5000);
	});
	try {
		return await Promise.race([promise, late]);
	} finally {
		clearTimeout(timer);
	}
}

/** A log counting its lines and closes, and the thread each line was logged on. */
class CountLog implements Log {
	lines = 0;
	closes = 0;
	log(_level: number, _line: string): boolean {
		this.lines++;
		return true;
	}
	close(): void {
		this.closes++;
	}
}

const text = (data: Uint8Array | rt.Bytes) => new TextDecoder().decode(data.slice());

/**
 * What Rust calls from its other threads reaches JavaScript on its own thread: a call the
 * JavaScript thread waits on, from that wait; one made while it idles, from its event loop;
 * a wake, and a drop, from either.
 */
async function threads(): Promise<void> {
	// A call refused before it reached Rust gives Rust no implementation, so none keeps a
	// token and the next call runs on JavaScript's thread, as the first did.
	ok(api.onFirstThread(), "the first call runs on JavaScript's thread");
	const disposed = Counter.new$("c", 0);
	disposed[Symbol.dispose]();
	let refused = false;
	try {
		api.logCounts(new CountLog(), 0, disposed, []);
	} catch {
		refused = true;
	}
	ok(refused, "a call of a disposed object throws");
	ok(api.onFirstThread(), "a call after a refused one runs on JavaScript's thread");

	// The addon holds a call's handles before it converts the rest, which may run
	// JavaScript: a getter of a `u64`'s limbs that disposes the receiver and makes another
	// object leaves the call its own receiver, whose value was moved out, not the new one.
	const victim = Counter.new$("victim", 1);
	let replacement: Counter.Counter | undefined;
	const limbs = {
		get hi(): number {
			victim[Symbol.dispose]();
			replacement = Counter.new$("replacement", 100);
			return 0x200000;
		},
		lo: 0,
	} as unknown as rt.u64;
	let moved = "";
	try {
		Counter.offset(victim, limbs, 0);
	} catch (e) {
		moved = (e as Error).message;
	}
	ok(moved === "api_Counter_offset: a handle's value was moved out", `a receiver disposed while held: ${moved}`);
	ok(Counter.get(replacement!) === 100, "the object made meanwhile is untouched");

	// A handle's id read before a reentrant call disposed it, and another object took its
	// slot, is refused once the addon reads it, never taken for the other object: what any
	// JavaScript between a call's read of an id and the addon's hold would pass.
	const stale = Counter.new$("stale", 1);
	const id = stale.p;
	let taker: Counter.Counter | undefined;
	const reentrant = Logger.new(
		new (class extends CountLog {
			override log(level: number, line: string): boolean {
				stale[Symbol.dispose]();
				taker = Counter.new$("taker", 100);
				return super.log(level, line);
			}
		})(),
	);
	Logger.write(reentrant, "dispose");
	reentrant[Symbol.dispose]();
	ok(taker!.p % 2 ** 32 === id % 2 ** 32, "the new object took the disposed one's slot");
	let stalled = "";
	try {
		rt.w.api_Counter_get(id);
	} catch (e) {
		stalled = (e as Error).message;
	}
	ok(stalled === "Counter is closed", `a stale id: ${stalled}`);
	ok(Counter.get(taker!) === 100, "the object that took its slot is untouched");

	// A kept closure, called on another thread after the call that took it returned.
	const spawned = Spawned.new$(() => 7);
	ok(Spawned.join(spawned) === 7, "a kept closure runs on another thread");
	// A lent closure, called on another thread the call joins.
	let kept = 0;
	ok(api.filter(Uint8Array.of(1, 2, 3, 4), (level) => (kept++, level % 2 === 0)) === 2, "a lent closure on another thread");
	ok(kept === 4, "each line reached the closure");

	// The send lands from another thread while the receive waits, and so does the wake of a
	// waker Rust keeps.
	const channel = Channel.new$();
	const threaded = Channel.recv(channel);
	Channel.sendFromThread(channel, new TextEncoder().encode("threaded"));
	ok(text(await within(threaded, "a wake from another thread")) === "threaded", "recv returns what another thread sent");
	let wakes = 0;
	Channel.notify(channel, () => wakes++);
	Channel.sendFromThread(channel, new TextEncoder().encode("rung"));
	await until(() => wakes === 1, "a waker woken from another thread calls back");
	Channel.clear(channel);
	channel[Symbol.dispose]();

	// An implementation Rust calls from another thread, which the call waits on, runs on
	// JavaScript's thread meanwhile, and is closed once dropped there.
	const threadLog = new CountLog();
	ok(api.emitFromThread(threadLog, Uint8Array.of(1, 2, 3)) === 3, "a call into JavaScript from a thread the call joins");
	ok(threadLog.lines === 3, "each line logged");
	await until(() => threadLog.closes === 1, "an implementation dropped on another thread is closed");
	const laterLog = new CountLog();
	ok((await within(api.emitLater(laterLog, Uint8Array.of(1, 2)), "emitLater")) === 4, "an async call and its poll into JavaScript from a joined thread");
	ok(laterLog.lines === 4, "each line logged, eagerly and once polled");

	// An implementation Rust holds, which a later call waits on another thread to call.
	const heldLog = new CountLog();
	const logger = Logger.new$(heldLog);
	ok(Logger.writeFromThread(logger, "held"), "a call into a held implementation from a joined thread");
	ok(heldLog.lines === 1, "the held implementation logged");
	logger[Symbol.dispose]();
	ok(heldLog.closes === 1, "closing the object closes the implementation it held");

	// A call another thread makes while JavaScript idles reaches it from its event loop,
	// and one it makes while JavaScript waits in a call, from that wait.
	const postedLog = new CountLog();
	const posting = LogThread.new$(postedLog, "posted");
	await until(() => postedLog.lines === 1, "a call into JavaScript while its thread idles");
	ok(LogThread.join(posting), "the posted call was kept");
	await until(() => postedLog.closes === 1, "an implementation its thread dropped is closed");
	const waitedLog = new CountLog();
	ok(LogThread.join(LogThread.new$(waitedLog, "waited")), "a call into JavaScript while its thread waits in Rust");
	ok(waitedLog.lines === 1, "the waited call logged");

	// A returned waker, from its wake and from the close that drops it.
	const wakeLog = new CountLog();
	const logWaker = api.logWaker(wakeLog);
	logWaker.wake();
	ok(wakeLog.lines === 1, "a wake calls into JavaScript from a joined thread");
	logWaker[Symbol.dispose]();
	ok(wakeLog.lines === 2, "a returned waker's close calls into JavaScript from a joined thread");
	await until(() => wakeLog.closes === 1, "the implementation a closed waker held is closed");
	const dropLog = new CountLog();
	const dropping = api.logOnDrop(dropLog);
	await step();
	dropping.cancel();
	ok(dropLog.lines === 1, "a cancel calls into the implementation its future's drop holds");
	try {
		await dropping;
		ok(false, "a cancelled future completed");
	} catch (e) {
		ok(e instanceof rt.Cancelled, `a cancelled future rejects with Cancelled: ${e}`);
	}
	ok(api.onFirstThread(), "a call while Rust holds no implementation runs on JavaScript's thread");
}

async function main(): Promise<void> {
	// A path from the working directory, not from the binding's modules, as a caller means it.
	await api.load("node/api.node");
	const message = await api.load().then(
		() => "",
		(e: unknown) => (e instanceof Error ? e.message : String(e)),
	);
	ok(message === "index.ts: the addon is loaded already", "a second load");
	const copied = await copy.load("node/api.node").then(
		() => "",
		(e: unknown) => (e instanceof Error ? e.message : String(e)),
	);
	ok(copied.includes("by another copy of its binding"), `a second copy's load: ${copied}`);
	const which = process.argv[2];
	switch (which) {
		case undefined:
			return check.script();
		case "threads":
			return threads();
		case "futures":
			return check.futures();
		case "owned":
			return check.owned();
		case "closed":
			return check.closedObjects();
		case "bytes":
			return check.lent();
		case "bounds":
			return check.bounds();
		case "checks":
			return check.checks();
		default:
			return check.broken(which);
	}
}

await main();
