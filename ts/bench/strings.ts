// The `str/` workloads' hand-written route: `bench.rs`'s ops 19 to 22, as moq's TypeScript
// codes a path through its Writer and Cursor, joins it with a template literal, and keys a
// `Map` by it.

import { Cursor, type StreamVersion, Writer } from "moq/js/net/src/stream.ts";

const NAMES = ["demo/room-42/alice/camera", "démo/salle-42/アリス/📷"];
const TRACKS = ["video", "audio", "catalog.json", "字幕"];
const encoder = new TextEncoder();

/** Op 19: the `selector`th name, as many times as `data`'s count, each size-prefixed. */
export function encodeStrings(selector: number, data: Uint8Array, version: StreamVersion): Promise<Uint8Array> {
	const count = Number(u64(data, 0));
	const name = NAMES[selector % NAMES.length] as string;
	return Writer.collect(version, async (w) => {
		for (let i = 0; i < count; i++) await w.string(name);
	});
}

/** Op 20: every string on the wire. */
export function decodeStrings(wire: Uint8Array, version: StreamVersion): Uint8Array {
	const cursor = new Cursor(wire, version);
	let count = 0;
	let total = 0;
	let last = "";
	while (cursor.remaining > 0) {
		const start = cursor.offset;
		last = cursor.string();
		total += cursor.offset - start;
		count++;
	}
	const size = encoder.encode(last).byteLength;
	// Each prefix is the one byte of a QUIC varint under 64.
	return u64s(count, total - count, size);
}

/** Op 21: each path joined and stripped of its name. */
export function join(selector: number, data: Uint8Array): Uint8Array {
	const count = Math.min(Number(u64(data, 0)), 1 << 16);
	const name = NAMES[selector % NAMES.length] as string;
	// Rust counts UTF-8 bytes: each part's, measured once.
	const nameLen = encoder.encode(name).byteLength;
	const trackLens = TRACKS.map((t) => encoder.encode(t).byteLength);
	let total = 0;
	let stripped = 0;
	for (let i = 0; i < count; i++) {
		const track = TRACKS[i % TRACKS.length] as string;
		const path = `${name}/${track}`;
		const trackLen = trackLens[i % TRACKS.length] as number;
		total += nameLen + 1 + trackLen;
		if (path.startsWith(name) && path[name.length] === "/") stripped += trackLen;
	}
	return u64s(total, stripped);
}

/** Op 22: each path a key of a `Map`, looked up four times. */
export function keyed(selector: number, data: Uint8Array): Uint8Array {
	const count = Math.min(Number(u64(data, 0)), 1 << 16);
	const name = NAMES[selector % NAMES.length] as string;
	const keys = Array.from({ length: count }, (_, i) => `${name}/${i}`);
	const map = new Map<string, number>();
	keys.forEach((key, i) => map.set(key, i));
	let sum = 0;
	for (let round = 0; round < 4; round++) for (const key of keys) sum += map.get(key) ?? 0;
	return u64s(sum, map.size);
}

function u64(data: Uint8Array, i: number): bigint {
	const bytes = new Uint8Array(8);
	bytes.set(data.subarray(i * 8, i * 8 + 8));
	return new DataView(bytes.buffer).getBigUint64(0);
}

function u64s(...values: number[]): Uint8Array {
	const out = new Uint8Array(values.length * 8);
	const view = new DataView(out.buffer);
	values.forEach((v, i) => view.setBigUint64(i * 8, BigInt(v)));
	return out;
}
