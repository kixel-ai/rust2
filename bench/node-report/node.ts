// ffi/moq/bench/export.ts's cases, but the loop, over a route node.ts is given: `napi`
// through addon.c, or `nf` through node:ffi. Its arguments are the route and the directory
// holding its libraries; it prints each case as bench-run reads it.

import type { Handle, Result, Route } from "./route.ts";

declare const process: { readonly argv: string[]; cpuUsage(): { user: number; system: number } };

const [, , name, dir] = process.argv;
const api: Route =
	name === "napi" ? (await import("./napi.ts")).load(`${dir}/addon.node`) : name === "nf" ? (await import("./nf.ts")).load(dir!) : undefined!;
if (api === undefined) throw new Error(`no route ${name}`);

const groups = 10000;
const frames = 100;
const n = groups * frames;
const sizes = [100, 1024, 16384, 65536, 1048576];
const payloads = [16, 64, 100, 256, 1024, 4096, 16384, 65536, 262144, 1048576];

type Span = { wall: number; cpu: number };

function now(): Span {
	const cpu = process.cpuUsage();
	return { wall: performance.now() * 1e6, cpu: (cpu.user + cpu.system) * 1e3 };
}

function end(start: Span, calls = n): Span {
	const at = now();
	return { wall: (at.wall - start.wall) / calls, cpu: (at.cpu - start.cpu) / calls };
}

let sink = 0;

function ok<T>(r: Result<T>, what: string): T {
	if (!r.ok) throw new Error(`${what} failed`);
	return r.value;
}

function some<T>(v: T | undefined, what: string): T {
	if (v === undefined) throw new Error(`${what} returned none`);
	return v;
}

function finish(group: Handle): void {
	ok(api.finish(group), "finish")[Symbol.dispose]();
	group[Symbol.dispose]();
}

let micros = 0;

function next(): Uint8Array {
	return ok(api.fromMicros(micros++), "fromMicros");
}

function append(track: Handle, at: Uint8Array): Handle {
	return ok(api.appendGroup(track, at), "appendGroup");
}

function createTrack(broadcast: Handle, name: string): Handle {
	return ok(api.createTrack(broadcast, name), "createTrack");
}

async function subscribe(track: Handle): Promise<Handle> {
	const consumer = api.consume(track);
	const subscriber = ok(await api.subscribe(consumer), "subscribe");
	consumer[Symbol.dispose]();
	return subscriber;
}

async function throughput(broadcast: Handle, size: number): Promise<Span> {
	const count = Math.min(Math.max(Math.floor((64 << 20) / (size * frames)), 3), 2000);
	const frame = new Uint8Array(size);
	const track = createTrack(broadcast, `throughput[${size}]`);
	const subscriber = await subscribe(track);
	const start = now();
	for (let g = 0; g < count; g++) {
		const at = next();
		const group = append(track, at);
		const reader = some(ok(await api.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			ok(api.writeFrame(group, at, frame), "writeFrame");
			const read = some(ok(await api.readFrame(reader), "readFrame"), "readFrame");
			const bytes = api.payload(read);
			sink += bytes.at(0)!;
			bytes[Symbol.dispose]();
			read[Symbol.dispose]();
		}
		reader[Symbol.dispose]();
		finish(group);
	}
	const ns = end(start, count * frames * size);
	subscriber[Symbol.dispose]();
	track[Symbol.dispose]();
	return { wall: 1e3 / ns.wall, cpu: 1e3 / ns.cpu };
}

async function payload(broadcast: Handle, size: number): Promise<Span> {
	const calls = Math.min(n, Math.max(1000, Math.floor((1 << 30) / size)));
	const track = createTrack(broadcast, `payload[${size}]`);
	const subscriber = await subscribe(track);
	const at = next();
	const group = append(track, at);
	const reader = some(ok(await api.recvGroup(subscriber), "recvGroup"), "recvGroup");
	ok(api.writeFrame(group, at, new Uint8Array(size)), "writeFrame");
	const read = some(ok(await api.readFrame(reader), "readFrame"), "readFrame");
	reader[Symbol.dispose]();
	finish(group);
	subscriber[Symbol.dispose]();
	track[Symbol.dispose]();
	const start = now();
	for (let i = 0; i < calls; i++) {
		const bytes = api.payload(read);
		sink += bytes.at(0)!;
		bytes[Symbol.dispose]();
	}
	const ns = end(start, calls);
	read[Symbol.dispose]();
	return ns;
}

async function round(): Promise<Map<string, Span>> {
	const out = new Map<string, Span>();
	const broadcast = api.broadcast();
	let track = createTrack(broadcast, "bench");
	const frame = new Uint8Array(100);
	micros = 0;

	let start = now();
	for (let i = 0; i < n; i++) {
		if (api.isClosed(track)) sink++;
	}
	out.set("call", end(start));

	const demanded = append(track, next());
	const demand = api.demand(demanded);
	start = now();
	for (let i = 0; i < n; i++) {
		sink += api.demandStart(demand) as number;
	}
	out.set("getter", end(start));
	demand[Symbol.dispose]();
	finish(demanded);

	start = now();
	for (let i = 0; i < n; i++) {
		if (api.largest(track) !== undefined) sink++;
	}
	out.set("record", end(start));

	start = now();
	for (let i = 0; i < n; i++) {
		api.info(track)[Symbol.dispose]();
	}
	out.set("object", end(start));

	const info = api.info(track);
	start = now();
	for (let i = 0; i < n; i++) {
		sink += api.priority(info);
	}
	out.set("field", end(start));
	info[Symbol.dispose]();

	track[Symbol.dispose]();
	track = createTrack(broadcast, "stamped");
	start = now();
	for (let g = 0; g < groups; g++) {
		const group = append(track, ok(api.fromMicros(micros), "fromMicros"));
		for (let f = 0; f < frames; f++) {
			ok(api.writeFrame(group, next(), frame), "writeFrame");
		}
		finish(group);
	}
	out.set("stamped", end(start));

	track[Symbol.dispose]();
	track = createTrack(broadcast, "write");
	start = now();
	for (let g = 0; g < groups; g++) {
		const at = next();
		const group = append(track, at);
		for (let f = 0; f < frames; f++) {
			ok(api.writeFrame(group, at, frame), "writeFrame");
		}
		finish(group);
	}
	out.set("write", end(start));

	track[Symbol.dispose]();
	track = createTrack(broadcast, "ready");
	let subscriber = await subscribe(track);
	let last: Handle | undefined;
	start = now();
	for (let g = 0; g < groups; g++) {
		const at = next();
		const group = append(track, at);
		const reader = some(ok(await api.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			ok(api.writeFrame(group, at, frame), "writeFrame");
			last?.[Symbol.dispose]();
			last = some(ok(await api.readFrame(reader), "readFrame"), "readFrame");
		}
		reader[Symbol.dispose]();
		finish(group);
	}
	out.set("ready", end(start));

	subscriber[Symbol.dispose]();
	track[Symbol.dispose]();
	track = createTrack(broadcast, "woken");
	subscriber = await subscribe(track);
	start = now();
	for (let g = 0; g < groups; g++) {
		const at = next();
		const group = append(track, at);
		const reader = some(ok(await api.recvGroup(subscriber), "recvGroup"), "recvGroup");
		for (let f = 0; f < frames; f++) {
			const pending = api.readFrame(reader);
			ok(api.writeFrame(group, at, frame), "writeFrame");
			some(ok(await pending, "readFrame"), "readFrame")[Symbol.dispose]();
		}
		reader[Symbol.dispose]();
		finish(group);
	}
	out.set("woken", end(start));
	subscriber[Symbol.dispose]();
	track[Symbol.dispose]();

	for (const size of sizes) out.set(`mbps:${size}`, await throughput(broadcast, size));

	start = now();
	for (let i = 0; i < n; i++) {
		const bytes = api.payload(last!);
		sink += bytes.at(0)!;
		bytes[Symbol.dispose]();
	}
	out.set("payload", end(start));
	last![Symbol.dispose]();

	for (const size of payloads) out.set(`payload:${size}`, await payload(broadcast, size));
	broadcast[Symbol.dispose]();
	return out;
}

await round();
for (const [key, value] of await round()) console.log(`${key}\t${value.wall}\t${value.cpu}`);
if (sink === -1) console.log("");
