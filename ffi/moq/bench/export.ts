// bench/export.c's cases through the generated TypeScript, so the two differ by what the
// binding and the engine add: all but the threaded ones, since JavaScript runs Rust on one
// thread. Each round runs twice, the first to warm the JIT. Each case prints its wall time
// and, on Node, the process's CPU time, its own threads included; a browser has no clock of
// that, so it prints NaN. Its arguments are the module's path, which bench/browser.ts's page
// serves in a browser, and on Node, moq/bench/codspeed-node.c's addon, under CodSpeed's CPU
// simulation, which then counts one round.

import * as api from "./web/index.ts";
import * as BroadcastInfo from "./web/BroadcastInfo.ts";
import * as BroadcastProducer from "./web/BroadcastProducer.ts";
import * as FrameFrame from "./web/FrameFrame.ts";
import * as GroupConsumer from "./web/GroupConsumer.ts";
import * as GroupDemand from "./web/GroupDemand.ts";
import * as GroupProducer from "./web/GroupProducer.ts";
import * as Timescale from "./web/Timescale.ts";
import * as Timestamp from "./web/Timestamp.ts";
import * as TrackConsumer from "./web/TrackConsumer.ts";
import * as TrackInfo from "./web/TrackInfo.ts";
import * as TrackProducer from "./web/TrackProducer.ts";
import * as TrackSubscriber from "./web/TrackSubscriber.ts";
import type * as rt from "./web/rt.ts";

// Node's globals, which the binding's types leave out; a browser's page supplies the first two.
declare const process: {
	readonly argv: string[];
	getBuiltinModule(id: "node:fs"): { readFileSync(path: string): Uint8Array<ArrayBuffer> };
	getBuiltinModule(id: "node:module"): { createRequire(from: string): (id: string) => Hooks };
	cpuUsage?(): { user: number; system: number };
};

/** moq/bench/codspeed.h's functions, as the addon gives them. */
type Hooks = { init(bench: string): boolean; start(): void; stop(name: string): void };

/** A tenth under CodSpeed, whose simulation counts every call exactly. */
let groups = 10000;
const frames = 100;
let n = groups * frames;
const sizes = [100, 1024, 16384, 65536, 1048576];
/** The payload rows' sizes: those of throughput, and between them, where a copy's cost turns. */
const payloads = [16, 64, 100, 256, 1024, 4096, 16384, 65536, 262144, 1048576];

/** Wall and CPU nanoseconds: a point, or a span per call. */
type Span = { wall: number; cpu: number };

function now(): Span {
	const cpu = process.cpuUsage?.();
	return { wall: performance.now() * 1e6, cpu: cpu === undefined ? NaN : (cpu.user + cpu.system) * 1e3 };
}

/** Nanoseconds per call since `start`, over `calls` calls. */
function per(start: Span, calls = n): Span {
	const at = now();
	return { wall: (at.wall - start.wall) / calls, cpu: (at.cpu - start.cpu) / calls };
}

const addon = process.argv[3];
const hooks = addon === undefined ? undefined : process.getBuiltinModule("node:module").createRequire(import.meta.url)(addon);

/**
 * The garbage collector, which `--expose-gc` exposes. Under CodSpeed each case starts with
 * it collected, so the collections a case makes depend on what the case allocates alone.
 */
const gc = (globalThis as { gc?: () => void }).gc;
let measured = false;

/** A case's start: now, and CodSpeed counting from here. */
function begin(): Span {
	if (measured) gc!();
	const start = now();
	hooks?.start();
	return start;
}

/** The case `name`'s nanoseconds per call since `start`, over `calls` calls. */
function end(start: Span, name: string, calls = n): Span {
	hooks?.stop(name);
	return per(start, calls);
}

/** What the loops sum, kept so the JIT cannot drop the calls. */
let sink = 0;

function ok<T, E>(r: rt.Result<T, E>, what: string): T {
	if (!r.ok) throw new Error(`${what} failed`);
	return r.value;
}

function some<T>(v: T | undefined, what: string): T {
	if (v === undefined) throw new Error(`${what} returned none`);
	return v;
}

/** A group's end: finished, and its producer disposed. */
function finish(group: GroupProducer.GroupProducer): void {
	ok(GroupProducer.finish(group), "finish");
	group[Symbol.dispose]();
}

/** Permits one side gives and the other waits for, as Dart's stream of them. */
class Permits {
	#queued = 0;
	#waiting: (() => void) | undefined;

	give(): void {
		const waiting = this.#waiting;
		this.#waiting = undefined;
		if (waiting === undefined) this.#queued++;
		else waiting();
	}

	take(): Promise<void> | undefined {
		if (this.#queued > 0) {
			this.#queued--;
			return undefined;
		}
		return new Promise((resolve) => (this.#waiting = resolve));
	}
}

/**
 * Groups of 100 frames of `size` bytes written and each read back, its payload's first byte
 * touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
 */
async function throughput(
	track: TrackProducer.TrackProducer,
	subscriber: TrackSubscriber.TrackSubscriber,
	at: Timestamp.Timestamp,
	size: number,
): Promise<Span> {
	const count = Math.min(Math.max(Math.floor((64 << 20) / (size * frames)), 3), 2000);
	const frame = new Uint8Array(size);
	const start = begin();
	for (let g = 0; g < count; g++) {
		const group = ok(TrackProducer.appendGroup(track), "appendGroup");
		const reader = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			ok(GroupProducer.writeFrame(group, at, frame), "writeFrame");
			const read = some(ok(await GroupConsumer.readFrame(reader), "readFrame"), "readFrame");
			const bytes = FrameFrame.payloadGet(read);
			sink += bytes.at(0)!;
			bytes[Symbol.dispose]();
			read[Symbol.dispose]();
		}
		reader[Symbol.dispose]();
		finish(group);
	}
	const ns = end(start, `throughput[${size}]`, count * frames * size);
	return { wall: 1e3 / ns.wall, cpu: 1e3 / ns.cpu };
}

/** `frame.payload` of one frame of `size` bytes, read many times, a byte of each touched. */
async function payload(
	track: TrackProducer.TrackProducer,
	subscriber: TrackSubscriber.TrackSubscriber,
	at: Timestamp.Timestamp,
	size: number,
): Promise<Span> {
	// A call a frame, at most a GiB of them copied.
	const calls = Math.min(n, Math.max(1000, Math.floor((1 << 30) / size)));
	const group = ok(TrackProducer.appendGroup(track), "appendGroup");
	const reader = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
	ok(GroupProducer.writeFrame(group, at, new Uint8Array(size)), "writeFrame");
	const read = some(ok(await GroupConsumer.readFrame(reader), "readFrame"), "readFrame");
	reader[Symbol.dispose]();
	finish(group);
	const start = begin();
	for (let i = 0; i < calls; i++) {
		const bytes = FrameFrame.payloadGet(read);
		sink += bytes.at(0)!;
		bytes[Symbol.dispose]();
	}
	const ns = end(start, `payload[${size}]`, calls);
	read[Symbol.dispose]();
	return ns;
}

async function round(): Promise<Map<string, Span>> {
	const out = new Map<string, Span>();
	const broadcast = BroadcastInfo.produce(BroadcastInfo.new());
	const info = TrackInfo.withTimescale(TrackInfo.default(), Timescale.MICRO());
	const track = ok(BroadcastProducer.createTrack(broadcast, "bench", info), "createTrack");
	const frame = new Uint8Array(100);

	let start = begin();
	for (let i = 0; i < n; i++) {
		if (TrackProducer.isClosed(track)) sink++;
	}
	out.set("call", end(start, "call"));

	const demanded = ok(TrackProducer.appendGroup(track), "appendGroup");
	const demand = GroupProducer.demand(demanded);
	start = begin();
	for (let i = 0; i < n; i++) {
		sink += GroupDemand.sequence(demand) as number;
	}
	out.set("getter", end(start, "getter"));
	demand[Symbol.dispose]();
	finish(demanded);

	start = begin();
	for (let i = 0; i < n; i++) {
		if (TrackProducer.largest(track) !== undefined) sink++;
	}
	out.set("record", end(start, "record"));

	start = begin();
	for (let i = 0; i < n; i++) {
		TrackProducer.info(track)[Symbol.dispose]();
	}
	out.set("object", end(start, "object"));

	// Each frame stamped through a fresh timestamp, as a caller holding a number must.
	let micros = 0;
	start = begin();
	for (let g = 0; g < groups; g++) {
		const group = ok(TrackProducer.appendGroup(track), "appendGroup");
		for (let f = 0; f < frames; f++) {
			ok(GroupProducer.writeFrame(group, ok(Timestamp.fromMicros(micros++), "fromMicros"), frame), "writeFrame");
		}
		finish(group);
	}
	out.set("stamped", end(start, "stamped"));

	const at = ok(Timestamp.fromMicros(micros), "fromMicros");
	start = begin();
	for (let g = 0; g < groups; g++) {
		const group = ok(TrackProducer.appendGroup(track), "appendGroup");
		for (let f = 0; f < frames; f++) {
			ok(GroupProducer.writeFrame(group, at, frame), "writeFrame");
		}
		finish(group);
	}
	out.set("write", end(start, "write"));

	const consumer = TrackProducer.consume(track);
	const subscriber = ok(await TrackConsumer.subscribe(consumer, undefined), "subscribe");
	consumer[Symbol.dispose]();

	// Each frame written, then read through a call whose future is ready at its first poll,
	// so it never waits for a wake.
	let last: FrameFrame.FrameFrame | undefined;
	start = begin();
	for (let g = 0; g < groups; g++) {
		const group = ok(TrackProducer.appendGroup(track), "appendGroup");
		const reader = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			ok(GroupProducer.writeFrame(group, at, frame), "writeFrame");
			last?.[Symbol.dispose]();
			last = some(ok(await GroupConsumer.readFrame(reader), "readFrame"), "readFrame");
		}
		reader[Symbol.dispose]();
		finish(group);
	}
	out.set("ready", end(start, "ready"));

	// Each read's promise made before its frame is written, so its first poll pends, and the
	// write's wake polls it again on a microtask.
	start = begin();
	for (let g = 0; g < groups; g++) {
		const group = ok(TrackProducer.appendGroup(track), "appendGroup");
		const reader = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			const pending = GroupConsumer.readFrame(reader);
			ok(GroupProducer.writeFrame(group, at, frame), "writeFrame");
			some(ok(await pending, "readFrame"), "readFrame")[Symbol.dispose]();
		}
		reader[Symbol.dispose]();
		finish(group);
	}
	out.set("woken", end(start, "woken"));

	// One writer and one reader; one outstanding permit bounds the stream.
	start = begin();
	const permits = new Permits();
	const writer = (async () => {
		for (let g = 0; g < groups; g++) {
			const group = ok(TrackProducer.appendGroup(track), "appendGroup");
			for (let f = 0; f < frames; f++) {
				await permits.take();
				ok(GroupProducer.writeFrame(group, at, frame), "writeFrame");
			}
			// The reader's acknowledgement of the group, before the next.
			await permits.take();
			finish(group);
		}
	})();
	for (let g = 0; g < groups; g++) {
		const reader = some(ok(await TrackSubscriber.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			permits.give();
			some(ok(await GroupConsumer.readFrame(reader), "readFrame"), "readFrame")[Symbol.dispose]();
		}
		reader[Symbol.dispose]();
		permits.give();
	}
	await writer;
	out.set("loop", end(start, "loop"));

	for (const size of sizes) out.set(`mbps:${size}`, await throughput(track, subscriber, at, size));

	// The last frame's 100 bytes, handed to JavaScript, one of them read, and disposed.
	start = begin();
	for (let i = 0; i < n; i++) {
		const bytes = FrameFrame.payloadGet(last!);
		sink += bytes.at(0)!;
		bytes[Symbol.dispose]();
	}
	out.set("payload", end(start, "payload"));
	last![Symbol.dispose]();

	for (const size of payloads) out.set(`payload:${size}`, await payload(track, subscriber, at, size));
	subscriber[Symbol.dispose]();
	track[Symbol.dispose]();
	broadcast[Symbol.dispose]();
	return out;
}

const { readFileSync } = process.getBuiltinModule("node:fs");
await api.load(readFileSync(process.argv[2]!));
// Under CodSpeed, one round, which prints nothing: its simulation counts each case.
measured = hooks?.init("ffi/moq/bench/export.ts::ts") ?? false;
if (measured) {
	if (gc === undefined) throw new Error("under CodSpeed, Node runs with --expose-gc");
	groups /= 10;
	n = groups * frames;
	await round();
} else {
	await round();
	for (const [key, value] of await round()) console.log(`${key}\t${value.wall}\t${value.cpu}`);
}
// Read once, so no engine drops the loops as dead.
if (sink === -1) console.log("");
