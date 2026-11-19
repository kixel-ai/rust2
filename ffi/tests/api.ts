// The api fixture's web test: the script fixtures/api/examples/transcript.rs runs as
// native Rust does on wasm32-wasip1, through the generated TypeScript on Node or Deno, from
// beside the binding's `web` directory, or in a browser, through tests/browser.ts's
// `process`. RUST2_WASM is the path of the module, built with the crate's `web` feature.
// tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks the futures' semantics, what disposing and the
// garbage collector free, what a closed object refuses, how returned bytes cross, or what
// a call lent bytes or text reads once memory grows, or breaks a contract, which must fail.

import * as api from "./web/index.ts";
import * as rt from "./web/rt.ts";
import * as g from "./web/glue.ts";
import * as Area from "./web/Area.ts";
import * as Atlas from "./web/Atlas.ts";
import * as Batch from "./web/Batch.ts";
import * as Batch2 from "./web/Batch2.ts";
import * as Berth from "./web/Berth.ts";
import * as Board from "./web/Board.ts";
import * as Cancelled from "./web/Cancelled.ts";
import * as Channel from "./web/Channel.ts";
import * as ClashSpan from "./web/ClashSpan.ts";
import * as ClashStore from "./web/ClashStore.ts";
import * as ClashTone from "./web/ClashTone.ts";
import * as ClashTone_ from "./web/ClashTone_.ts";
import * as Codecs from "./web/Codecs.ts";
import * as Counter from "./web/Counter.ts";
import * as Cursor from "./web/Cursor.ts";
import * as Der from "./web/Der.ts";
import * as Digest from "./web/Digest.ts";
import * as Draft from "./web/Draft.ts";
import * as Error_ from "./web/Error_.ts";
import type * as Event from "./web/Event.ts";
import * as Extent from "./web/Extent.ts";
import * as Gap from "./web/Gap.ts";
import * as Hits from "./web/Hits.ts";
import * as Instant from "./web/Instant.ts";
import * as Json from "./web/Json.ts";
import * as Key from "./web/Key.ts";
import * as Label from "./web/Label.ts";
import * as Labelled from "./web/Labelled.ts";
import * as Layered from "./web/Layered.ts";
import * as Ledger from "./web/Ledger.ts";
import * as Local from "./web/Local.ts";
import type { Log } from "./web/Log.ts";
import * as Logger from "./web/Logger.ts";
import * as Mark from "./web/Mark.ts";
import * as Marker from "./web/Marker.ts";
import * as Measure from "./web/Measure.ts";
import * as Message from "./web/Message.ts";
import * as Meter from "./web/Meter.ts";
import * as Meters from "./web/Meters.ts";
import * as Moment from "./web/Moment.ts";
import * as Name from "./web/Name.ts";
import * as Node from "./web/Node.ts";
import * as Numbers from "./web/Numbers.ts";
import * as OuterInnerU8 from "./web/OuterInnerU8.ts";
import * as PadU16 from "./web/PadU16.ts";
import * as PadU8 from "./web/PadU8.ts";
import * as PairSizeU8 from "./web/PairSizeU8.ts";
import * as PairU32 from "./web/PairU32.ts";
import * as PairU8Size from "./web/PairU8Size.ts";
import * as Peer from "./web/Peer.ts";
import * as Point from "./web/Point.ts";
import * as Receipt from "./web/Receipt.ts";
import * as Registry from "./web/Registry.ts";
import * as Retry from "./web/Retry.ts";
import * as Score from "./web/Score.ts";
import * as Seat from "./web/Seat.ts";
import * as Shape from "./web/Shape.ts";
import * as Site from "./web/Site.ts";
import * as SlotU8 from "./web/SlotU8.ts";
import * as Span from "./web/Span.ts";
import * as Stack from "./web/Stack.ts";
import * as Stamp from "./web/Stamp.ts";
import * as Tallies from "./web/Tallies.ts";
import * as Tally from "./web/Tally.ts";
import * as Ticket from "./web/Ticket.ts";
import * as TlsConfig from "./web/TlsConfig.ts";
import * as TlsSession from "./web/TlsSession.ts";
import * as Token from "./web/Token.ts";
import * as Trail from "./web/Trail.ts";
import * as Url from "./web/Url.ts";
import * as Verbosity from "./web/Verbosity.ts";
import * as Waiter from "./web/Waiter.ts";
import * as Weighted from "./web/Weighted.ts";
import * as Window from "./web/Window.ts";

// Node's globals, which the binding's types leave out.
declare const process: {
	readonly argv: string[];
	readonly env: Record<string, string | undefined>;
	exit(code: number): never;
	readonly stderr: { write(text: string): boolean };
	getBuiltinModule(id: "node:fs"): { readFileSync(path: string): Uint8Array<ArrayBuffer> };
	getBuiltinModule(id: "node:v8"): { setFlagsFromString(flags: string): void };
	getBuiltinModule(id: "node:vm"): { runInNewContext(code: string): () => void };
};

const print = (line: string) => console.log(line);

/** A `u64` in decimal. */
const dec = (v: rt.u64) => (typeof v === "number" ? `${v}` : `${rt.u64ToBig(v)}`);

/** A `u64` as a number, which it must be. */
function num(v: rt.u64): number {
	if (typeof v !== "number") throw new RangeError(`${dec(v)} is past a safe integer`);
	return v;
}

const U64_MAX = rt.u64FromBig(0xffffffffffffffffn);

/** A `u128` in hex, as transcript.rs prints it. */
function hex128(v: rt.u128): string {
	return rt.u128ToBig(v).toString(16).padStart(32, "0");
}

const encoder = new TextEncoder();
const decoder = new TextDecoder();
const enc = (text: string) => encoder.encode(text);
const text = (data: Uint8Array | rt.Bytes) => decoder.decode(data.slice());
const bytes = (...values: number[]) => new Uint8Array(values);

/** A list as Rust's `Debug` prints one. */
function list(values: Iterable<unknown>): string {
	return `[${[...values].map((v) => `${v}`).join(", ")}]`;
}

/** Bytes in hex, as transcript.rs prints them. */
function hex(data: Uint8Array): string {
	return [...data].map((b) => b.toString(16).padStart(2, "0")).join("");
}

/** A socket address as its IP's bytes in hex, its port, flow label, and scope, each IPv6's alone. */
function socketHex(addr: rt.SocketAddr): string {
	if (addr.tag === "V4") return `${hex(addr._0.ip.octets)}:${addr._0.port}`;
	const v6 = addr._0;
	return `${hex(v6.ip.octets)}:${v6.port}:${v6.flowinfo}:${v6.scopeId}`;
}

const quoted = (values: string[]) => `[${values.map((v) => `"${v}"`).join(", ")}]`;

/** A path's bytes as Rust's `escape_ascii` prints them. */
function escaped(path: rt.PathBuf): string {
	let out = "";
	for (const b of path.bytes) {
		if (b === 0x09) out += "\\t";
		else if (b === 0x0d) out += "\\r";
		else if (b === 0x0a) out += "\\n";
		else if (b === 0x5c || b === 0x27 || b === 0x22) out += `\\${String.fromCharCode(b)}`;
		else if (b >= 0x20 && b < 0x7f) out += String.fromCharCode(b);
		else out += `\\x${b.toString(16).padStart(2, "0")}`;
	}
	return out;
}

type Bound = ReturnType<typeof api.end>;

function bound(b: Bound): string {
	return b.tag === "Unbounded" ? "Unbounded" : `${b.tag}(${dec(b._0)})`;
}

/** Each float's bits, as transcript.rs prints them. */
const bits32 = (v: Float32Array) => list(new Uint32Array(v.buffer, v.byteOffset, v.length));
const bits64 = (v: Float64Array) => list(new BigUint64Array(v.buffer, v.byteOffset, v.length));

/** The floats of `bits`, each never a JavaScript number, whose NaN an engine may make its
 * own: Deno's V8 does, storing one in an array literal. */
const floats64 = (...bits: bigint[]) => new Float64Array(new BigUint64Array(bits).buffer);

/** Print each of `n`'s lists, its floats as their bits, as transcript.rs does. */
function numbers(what: string, n: Numbers.Numbers): void {
	print(`${what} u16 ${list(n.u16s)}`);
	print(`${what} u32 ${list(n.u32s)}`);
	print(`${what} u64 ${list(n.u64s)}`);
	print(`${what} i8 ${list(n.i8s)}`);
	print(`${what} i16 ${list(n.i16s)}`);
	print(`${what} i32 ${list(n.i32s)}`);
	print(`${what} i64 ${list(n.i64s)}`);
	print(`${what} f32 ${bits32(n.f32s)}`);
	print(`${what} f64 ${bits64(n.f64s)}`);
	print(`${what} some ${n.some === undefined ? "None" : `Some(${list(n.some)})`}`);
}

/** The value of a result that must be `Ok`. */
function unwrap<T, E>(r: rt.Result<T, E>): T {
	if (!r.ok) throw new Error(`an unexpected Err: ${String(r.error)}`);
	return r.value;
}

/** The Log callback: keeps even levels. */
class PrintLog implements Log {
	log(level: number, line: string): boolean {
		print(`log ${level} ${line}`);
		return level % 2 === 0;
	}

	// Rust closes it once it drops the implementation.
	close(): void {
		print("freed");
	}
}

/** A Log that counts the lines it keeps and its closes, running `then` on each line. */
class CountLog implements Log {
	lines = 0;
	closes = 0;
	readonly then: (line: string) => void;

	constructor(then: (line: string) => void = () => {}) {
		this.then = then;
	}

	log(_level: number, line: string): boolean {
		this.lines++;
		this.then(line);
		return true;
	}

	close(): void {
		this.closes++;
	}
}

const close = (handle: Disposable) => handle[Symbol.dispose]();

function check(ok: boolean, what: string): void {
	if (!ok) {
		process.stderr.write(`failed: ${what}\n`);
		process.exit(1);
	}
}

/** `f` throws an error of `type`, whose message `message` accepts. */
function throws(f: () => unknown, type: new (...args: never[]) => Error, what: string, message?: (m: string) => boolean): void {
	try {
		f();
	} catch (e) {
		check(e instanceof type && (message === undefined || message(e.message)), `${what}: ${String(e)}`);
		return;
	}
	check(false, what);
}

/** `f` throws the error of a closed `type`: the binding's own, for a value whose calls only
 * borrow it, or else Rust's check of a moved value, naming the function. */
function closed(f: () => unknown, type: string, what: string): void {
	throws(
		f,
		Error,
		what,
		(m) => m === `${type} is closed` || (m.startsWith("api_") && m.endsWith(": a handle's value was moved out")),
	);
}

/** `f` throws the error `message` of a close under a borrow. */
function borrowed(f: () => unknown, message: string, what: string): void {
	throws(f, Error, what, (m) => m === message);
}

/** Let the event loop run what is ready. */
const step = () => new Promise<void>((resolve) => setTimeout(resolve, 0));

/** Run the event loop until `done()`, failing after a deadline. */
async function until(done: () => boolean, what: string): Promise<void> {
	const deadline = Date.now() + 5000;
	while (!done()) {
		check(Date.now() < deadline, what);
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

/** `promise` rejects with `rt.Cancelled`. */
async function cancelled(promise: Promise<unknown>, what: string): Promise<void> {
	try {
		await within(promise, what);
	} catch (e) {
		check(e instanceof rt.Cancelled, `${what}: ${String(e)}`);
		return;
	}
	check(false, what);
}

/** Bytes a reader of `data` reads and seeks, as Rust's `Cursor` does. */
class BytesReader implements rt.Reader {
	at = 0n;
	readonly data: Uint8Array;
	readonly then: () => void;

	constructor(data: Uint8Array, then: () => void = () => {}) {
		this.data = data;
		this.then = then;
	}

	read(buffer: Uint8Array): number {
		this.then();
		const start = Number(this.at < BigInt(this.data.length) ? this.at : BigInt(this.data.length));
		const n = Math.min(buffer.length, this.data.length - start);
		buffer.set(this.data.subarray(start, start + n));
		this.at += BigInt(n);
		return n;
	}

	seek(offset: rt.u64 | rt.i64, from: rt.SeekFrom): rt.u64 {
		const by = from === "Start" ? rt.u64ToBig(offset as rt.u64) : rt.i64ToBig(offset as rt.i64);
		const base = from === "Start" ? 0n : from === "Current" ? this.at : BigInt(this.data.length);
		if (base + by < 0n) throw new Error("invalid seek to a negative or overflowing position");
		this.at = base + by;
		return rt.u64FromBig(this.at);
	}
}

/** A reader whose reads fail, as the transcript's: each seek lands at 0. */
class Broken implements rt.Reader {
	read(_buffer: Uint8Array): number {
		throw new Error("broken");
	}

	seek(_offset: rt.u64 | rt.i64, _from: rt.SeekFrom): rt.u64 {
		return 0;
	}
}

/** A reader of nothing, which seeks anywhere from its start, as Rust's `Cursor` does. */
class Far implements rt.Reader {
	read(_buffer: Uint8Array): number {
		return 0;
	}

	seek(offset: rt.u64 | rt.i64, _from: rt.SeekFrom): rt.u64 {
		return offset as rt.u64;
	}
}

async function script(): Promise<void> {
	print(`limit ${dec(api.LIMIT)}`);
	check(`${rt.i64ToBig(api.FAR)}` === "9223372036854775807", "a constant past a safe integer keeps its type");

	const p = Point.offset({ x: 1, y: 2 }, 2, 3);
	print(`offset ${p.x} ${p.y}`);
	print(`debug ${Point.debug(p)}`);
	print(`eq ${Point.eq(p, { x: 3, y: 5 })}`);
	const d = Point.default();
	print(`default ${d.x} ${d.y}`);

	const c = Counter.new("c", 5);
	print(`get ${dec(Counter.get(c))}`);
	print(`name ${Counter.name(c)}`);
	print(`add ok ${dec(unwrap(Counter.add(c, 10)))}`);
	const over = Counter.add(c, 2000);
	if (!over.ok) print(`add err ${Error_.toString(over.error)} ${Error_.debug(over.error)}`);
	const under = Counter.sub(c, 20);
	if (!under.ok) print(`sub err ${Error_.toString(under.error)} ${Error_.debug(under.error)}`);
	Counter.rename(c, "d");
	print(`name ${Counter.name(c)}`);
	print(`is d ${Counter.is(c, "d")}`);
	const copy = Counter.clone(c);
	Counter.add(copy, 1);
	print(`clone ${dec(Counter.get(copy))} original ${dec(Counter.get(c))}`);
	close(copy);
	print(`debug ${Counter.debug(c)}`);
	const label = Counter.intoLabel(c, p);
	print(`label ${Label.debug(label)}`);
	print(`note ${api.hasText({ text: "" })} ${api.hasText({ text: undefined })}`);

	const [below, above] = Counter.split(Counter.new("s", 7), 5);
	print(`split ${Counter.name(below)} ${dec(Counter.get(below))} ${Counter.name(above)} ${dec(Counter.get(above))}`);
	const [only] = api.only(Counter.new("o", 3));
	print(`only ${Counter.name(only)} ${dec(Counter.get(only))}`);
	const [w8, w16, w32, w64] = api.widths(98305);
	print(`widths ${w8} ${w16} ${w32} ${dec(w64)}`);
	for (const b of [2, 0]) {
		const r = api.divide(7, b);
		print(r.ok ? `divide ${dec(r.value[0])} ${dec(r.value[1])}` : `divide err ${Error_.toString(r.error)}`);
	}
	for (const n of [98305, 0]) {
		const spread = api.spread(n);
		print(spread === undefined ? "spread none" : `spread ${spread[0]} ${spread[1]} ${spread[2]} ${dec(spread[3])}`);
	}
	for (const needle of ["lo", "x"]) {
		const found = api.find(enc("hello"), enc(needle));
		print(found === undefined ? "find none" : `find ${found[0]} ${found[1]}`);
	}
	const statRows = api.rows([
		{ x: 1, y: 2 },
		{ x: 3, y: 3 },
	]);
	print(`rows ${statRows.map(([n, s, at]) => `${n} ${Shape.debug(s)} ${at.x} ${at.y}`).join(", ")}`);
	// Each handle in a tuple moves in, as one taken by value does, and the counters a board
	// returns build another, each moved back in.
	const first = Board.new([
		["low", Counter.new("l", 1)],
		["high", Counter.new("h", 9)],
	]);
	const board = Board.new(Board.intoCounters(first));
	const ranked = Board.ranked(board);
	print(`ranked ${ranked.map(([n, r]) => `${n} ${Counter.name(r)} ${dec(Counter.get(r))}`).join(", ")}`);
	const best = Board.best(board);
	print(`best ${Counter.name(best)} ${dec(Counter.get(best))}`);
	close(best);
	close(board);
	// A map crosses as its pairs and a set as its items, which Rust collects in order, each
	// key once, the last kept.
	const atlas = Atlas.with(
		{
			points: [
				["b", { x: 1, y: 1 }],
				["ab", { x: 2, y: 3 }],
				["b", { x: 4, y: 4 }],
			],
			ids: [3, 1, 3],
		},
		"c",
		{ x: 5, y: 6 },
	);
	print(`atlas ${atlas.points.map(([n, at]) => `${n} ${at.x} ${at.y}`).join(", ")}; ids ${atlas.ids.join(" ")}`);
	// Another crate's error crosses as its message.
	for (const t of ["80", "x"]) {
		const r = api.port(t);
		print(r.ok ? `port ${r.value}` : `port err ${r.error}`);
	}
	const span: Span.Span = { bounds: [2, 10], named: ["s", "Square"] };
	const [lowerHalf, upperHalf] = api.halves(span);
	print(`halves ${lowerHalf[0]} ${lowerHalf[1]} ${upperHalf[0]} ${upperHalf[1]} ${Span.debug(span)}`);
	const [swappedN, swappedText] = api.swapped(["t", 4]);
	print(`swapped ${swappedN} ${swappedText}`);
	for (const value of [undefined, [undefined], [3]] as ([number | undefined] | undefined)[]) {
		const doubled = api.doubled(value);
		if (doubled === undefined) print("doubled None");
		else print(`doubled Some((${doubled[0] === undefined ? "None" : `Some(${doubled[0]})`},))`);
	}
	// A borrowed or boxed tuple of one is a tuple, not its element.
	const [lentOne] = api.lentBack([bytes(1, 2)]);
	const [boxedOne] = api.boxedTuple(bytes(3));
	print(`lent back ${list(lentOne)} boxed ${list(boxedOne)}`);
	// A borrowed string and an owned one share a tuple struct.
	print(
		`title Some(("${api.title(true)![0]}",)) ${api.title(false) === undefined ? "None" : "Some"} ` +
			`named Some(("${api.named("x")![0]}",))`,
	);

	// Each lent counter is a new handle to a clone.
	const registry = Registry.new(Counter.new("first", 1));
	print(`last ${Registry.last(registry) !== undefined}`);
	Registry.push(registry, Counter.new("last", 2));
	const lentFirst = Registry.firstGet(registry);
	Counter.add(lentFirst, 10);
	print(`first ${dec(Counter.get(Registry.firstGet(registry)))} lent ${dec(Counter.get(lentFirst))}`);
	const last = Registry.last(registry)!;
	Counter.add(last, 10);
	print(`last ${dec(Counter.get(Registry.last(registry)!))} lent ${dec(Counter.get(last))}`);
	// A closure lent for the call edits the first counter through a handle.
	const by = 5;
	const edited = Registry.edit(registry, (counter) => {
		Counter.add(counter, by);
		return Counter.edited(counter) && Counter.get(counter) === 6;
	});
	print(`edit ${edited} ${dec(Counter.get(Registry.firstGet(registry)))}`);
	// A setter moves an object in, as assignment does, closing it.
	Registry.firstSet(registry, Counter.new("second", 3));
	print(`set first ${dec(Counter.get(Registry.firstGet(registry)))} ${Counter.name(Registry.firstGet(registry))}`);
	print(`walk ${dec(api.walk(3, (cursor) => num(Cursor.at(cursor)) * 2))}`);
	print(`each ${api.each(2, (at) => print(`visit ${at}`))}`);
	print(`offset ${api.offset(() => 7, 3)}`);
	print(`shift ${api.shift(() => 7, 4)}`);

	const name = Name.owned("a");
	const joined = Name.join(name, Name.owned("b"));
	print(`joined ${Name.asStr(joined)} ${Name.debug(joined)} ${Name.eq(joined, name)}`);
	Name.rename(joined, "c");
	print(`renamed ${Name.asStr(joined)}`);
	const long = api.longer(joined, Name.owned("dd"));
	print(`longer ${Name.asStr(long)}`);
	print(`unnamed [${Name.asStr(api.unnamed())}]`);
	const lit = Name.borrowed("lit");
	print(`borrowed ${Name.asStr(lit)} ${Name.shares(lit, Name.borrowed("lit"))} ${Name.shares(lit, Name.borrowed("other"))}`);

	print(`total ${dec(api.total({ start: 2, end: 5 }))}`);
	print(`reach ${api.reach(2, 5)}`);
	const scaled = [
		api.scaled(rt.u128FromBig((1n << 128n) - 1n), 1),
		api.scaled(rt.u128FromBig((1n << 64n) + 1n), rt.u64FromBig(1n << 63n)),
		api.scaled(rt.u128FromBig(1n << 127n), 2),
	];
	print(`scaled ${scaled.map((v) => (v === undefined ? "none" : hex128(v))).join(" ")}`);
	const digest = Digest.new();
	Digest.sumSet(digest, unwrap(api.fingerprint("deadbeef")));
	const unparsed = api.fingerprint("zz");
	print(
		`digest ${hex(Digest.sumGet(digest))} ${hex(Digest.flipped(digest, bytes(0xff, 0, 0xff, 0)))} ` +
			`${unparsed.ok ? "" : unparsed.error}`,
	);
	close(digest);
	const batch = Batch.new();
	const batch2 = api.pair();
	let accepted = 0;
	for (let v = 1; v <= 5; v++) if (Batch.push(batch, v)) accepted++;
	let paired = 0;
	for (let v = 1; v <= 3; v++) if (Batch2.push(batch2, v)) paired++;
	const emptied = api.drain4(batch);
	print(
		`batch ${Batch.capacity(batch)} ${accepted} ${dec(emptied)} ${dec(api.drain4(batch))} ` +
			`${Batch2.capacity(batch2)} ${paired} ${dec(api.drain2(batch2))}`,
	);
	print(`capacities ${api.capacities42(batch, batch2)} ${api.capacities22(batch2, batch2)}`);
	close(batch);
	close(batch2);
	const wide = api.widen({ start: 2, end: 5 }, 1);
	print(`widen ${dec(wide.start)} ${dec(wide.end)}`);
	const counts = (
		[
			[{ tag: "Included", _0: 2 }, { tag: "Excluded", _0: 5 }],
			[{ tag: "Included", _0: 2 }, { tag: "Included", _0: 5 }],
			[{ tag: "Included", _0: 3 }, { tag: "Unbounded" }],
			[{ tag: "Unbounded" }, { tag: "Unbounded" }],
			[{ tag: "Excluded", _0: 2 }, { tag: "Included", _0: 4 }],
		] as [Bound, Bound][]
	).map(([start, end]) => api.count({ start, end }));
	print(`count ${counts.map(dec).join(" ")}`);
	print(`end ${bound(api.end(3))} ${bound(api.end(0))}`);
	const [endsStart, endsEnd] = api.ends(3);
	print(`ends ${bound(endsStart)} ${bound(endsEnd)}`);
	for (const n of [2, 5]) {
		const r = api.below(n, 3);
		print(r.ok ? `below ${dec(r.value)}` : `below err ${bound(r.error.start)} ${bound(r.error.end)}`);
	}
	const window: Window.Window = { range: { start: 2, end: 8 }, end: api.end(5) };
	print(`window ${Window.contains(window, 4)} ${Window.contains(window, 6)} ${Window.debug(window)}`);

	print(api.greet("x", "Square"));
	print(`len ${api.len(enc("abc"))}`);
	const binary = bytes(0xff, 0, 0x61);
	print(`hex ${api.hex(binary)}`);
	print(`timed hex ${api.timedHex(binary)}`);
	print(`sample hex ${api.sampleHex(binary)}`);
	print(`queued hex ${api.queuedHex(binary)}`);
	print(`tick ${api.tickName({ name: "tick", at: undefined, ttl: undefined })}`);
	print(`rooted ${api.rootedName({ name: "rooted", at: undefined, ttl: undefined })}`);
	print(`stored ${api.storedText("stored")}`);
	print(`layer ${api.layerName({ name: "layer", at: undefined, ttl: undefined })}`);
	for (const event of [{ tag: "Moved", _0: p }, { tag: "Named", name: "n" }, { tag: "Done" }] as const) {
		print(api.describe(event));
	}

	// A `Copy` value passed by value is copied, so it stays usable.
	const stamp = Stamp.fromMillis(2500);
	print(`secs ${dec(Stamp.secs(stamp))}`);
	print(`later ${Stamp.later(Stamp.fromMillis(3000), stamp)}`);
	print(`secs ${dec(Stamp.secs(stamp))}`);
	print(`whole ${Stamp.debug(Stamp.whole(stamp))}`);
	const message = Message.new(stamp, enc("body"));
	print(`payload ${text(Message.payloadGet(message))}`);
	print(`stamp ${Stamp.debug(Message.stampGet(message)!)}`);
	print(`sent ${Message.sent(message)}`);
	// Setters assign in place: the bytes are copied in, and a `Copy` value copied.
	const reset = Stamp.fromMillis(4000);
	Message.stampSet(message, reset);
	Message.payloadSet(message, enc("edited"));
	print(`payload ${text(Message.payloadGet(message))}`);
	print(`stamp ${Stamp.debug(Message.stampGet(message)!)}`);
	print(`secs ${dec(Stamp.secs(reset))}`);
	Message.stampSet(message, undefined);
	print(`stamp ${Message.stampGet(message) !== undefined}`);

	// A value past a word, copied across: a closure takes and returns one by value.
	const moment = Moment.new(1500, 1000)!;
	const Forged = rt.Opaque as unknown as new (bytes: Uint8Array) => Moment.Moment;
	throws(() => new Forged(new Uint8Array(16)), TypeError, "a caller's opaque value");
	throws(() => Moment.millis({ bytes: new Uint8Array(16) } as unknown as Moment.Moment), RangeError, "a lookalike");
	print(`moment ${dec(Moment.millis(moment))} ${Moment.new(1, 0) === undefined}`);
	const shifted = api.shifted(moment, (m) => Moment.after(m, 500));
	print(`shifted ${dec(Moment.millis(shifted))} ${Moment.eq(shifted, Moment.after(moment, 500))}`);
	print(`wrapped ${dec(Moment.millis(Moment.after(moment, U64_MAX)))} ${Moment.eq(moment, shifted)}`);

	// A value with padding, or bytes that vary, is a handle, as no table says where padding is.
	const gap = Gap.new(7, 3);
	print(`gap ${dec(Gap.sum(gap))} ${Gap.eq(gap, Gap.new(7, 3))}`);
	const berth = Berth.new(4);
	print(`berth Some(${Berth.at(berth)}) ${Berth.at(Berth.new(undefined)) === undefined ? "None" : "Some"}`);

	// A `Copy` type changed in place is a value, which a `&mut self` method returns as the
	// call left it, Rust's own return beside it: a change reaches only the copy it replaces.
	let score = Score.of(2);
	const increment = Score.of(3);
	score = Score.add(score, increment);
	score = Score.add(score, increment);
	print(`score ${dec(Score.count(score))} ${dec(Score.count(Score.doubled(increment)))} ${dec(Score.count(increment))}`);
	let [scoreCopy, taken] = Score.take(score);
	print(`taken ${dec(taken)} ${dec(Score.count(scoreCopy))} ${dec(Score.count(score))}`);
	const otherScore = { v: increment };
	const [absorbed, before0, before1] = Score.absorb(scoreCopy, otherScore);
	scoreCopy = absorbed;
	print(
		`absorbed ${dec(before0)} ${dec(before1)} ${dec(Score.count(scoreCopy))} ` +
			`${dec(Score.count(otherScore.v))} ${dec(Score.count(increment))}`,
	);
	let hits = Hits.of(1, 2);
	const keptHits = hits;
	hits = Hits.add(hits, Hits.of(10, 20));
	hits = Hits.hitsSet(hits, 7);
	print(
		`hits ${dec(Hits.hitsGet(hits))} ${dec(Hits.missesGet(hits))} ${dec(Hits.hitsGet(keptHits))} ` +
			`${dec(Hits.missesGet(keptHits))} ${Hits.eq(hits, keptHits)}`,
	);
	const [counted, countedSame] = Hits.eq_(keptHits, keptHits);
	print(`counted ${countedSame} ${dec(Hits.hitsGet(counted))}`);
	let extent = Extent.new(5, 3);
	const extentStart = extent;
	const [slid, old] = Extent.slide(extent, 10);
	extent = slid;
	extent = Extent.startSet(extent, num(Extent.startGet(extent)) + 1);
	print(
		`extent ${dec(old)} ${dec(Extent.startGet(extent))} ${dec(Extent.end(extent))} ` +
			`${dec(Extent.startGet(extentStart))} ${dec(Extent.end(extentStart))}`,
	);
	const weighted = { score: increment, weight: 4 };
	print(`weighted ${dec(Weighted.total(weighted))} ${num(Score.count(weighted.score)) + num(Score.count(increment))}`);
	// A checked mark beside bytes in a tuple and in a record lent, and bytes beside a move.
	const mark = Mark.of(1);
	Mark.add(mark, 2);
	const parcel = { mark, payload: bytes(97, 98, 99) };
	print(`heft ${dec(api.heft([mark, bytes(97, 98)], parcel, [Counter.new("w", 1), bytes(97, 98, 99, 100)]))}`);
	// A value's `&mut self` beside a checked handle.
	const [noted, notedBefore] = Score.note(Score.of(2), Mark.of(5));
	print(`noted ${dec(notedBefore)} ${dec(Score.count(noted))}`);

	print(`kept ${api.emit(new PrintLog(), bytes(1, 2, 3, 4))}`);
	const logger = Logger.new(new PrintLog());
	print(`write ${Logger.write(logger, "hi")}`);
	close(logger);
	print("dropped");
	const threshold = 2;
	const filtered = api.filter(bytes(1, 2, 3, 4), (level, line) => {
		print(`filter ${level} ${line}`);
		return level > threshold;
	});
	print(`filtered ${filtered}`);

	const channel = Channel.new();
	let wakes = 0;
	// Polled at the call, it pends, and the wake the send posts resumes it from the event loop.
	let got: rt.Bytes | undefined;
	const recv = Channel.recv(channel);
	void recv.then((payload) => (got = payload));
	await step();
	print(`recv pending ${got === undefined}`);
	Channel.send(channel, enc("hi"));
	check(got === undefined, "recv never resumes inside the wake");
	await recv;
	wakes++;
	print(`woken ${wakes}`);
	print(`recv ${text(got!)}`);
	Channel.send(channel, new Uint8Array(0));
	const nonempty = await Channel.recvNonempty(channel);
	print(nonempty.ok ? `nonempty ${nonempty.value.length}` : `nonempty err ${Error_.toString(nonempty.error)}`);
	const fill4 = Batch.new();
	const fill2 = api.pair();
	print(`fill ${[...(await api.fill4(fill4, 9)), ...(await api.fill2(fill2, 1))].join(" ")}`);
	close(fill4);
	close(fill2);
	// The send happens at the call: the future is dropped unawaited.
	void Channel.sendLater(channel, enc("eager"));
	print(`recv ${text(await Channel.recv(channel))}`);
	// The send happens at the call, and the future borrows the channel and the tag.
	print(`tagged ${await Channel.tagged(channel, "tag")}`);
	print(`itemized ${await Channel.itemized(channel, new Uint32Array([1, 2, 3]))}`);
	print(`byte_len ${await api.byteLen(enc("abc"))}`);
	Channel.send(channel, enc("sized"));
	const [payload, size] = await Channel.recvSized(channel);
	print(`sized ${text(payload)} ${size}`);
	// The waker moves into Rust, which keeps it until the send wakes it.
	Channel.notify(channel, () => wakes++);
	Channel.send(channel, enc("rung"));
	await until(() => wakes === 2, "the send wakes the waker Rust keeps");
	print(`woken ${wakes}`);
	// A poll with no async twin is a future.
	print(`take ${text(await Channel.pollTake(channel))}`);
	Channel.send(channel, enc("measured"));
	print(`measure ${unwrap(await Channel.pollMeasure(channel, 2))}`);
	// A context is made of the waker, which Rust keeps until the send wakes it.
	Channel.listen(channel, () => wakes++);
	Channel.send(channel, enc("heard"));
	await until(() => wakes === 3, "the send wakes the context Rust keeps");
	print(`woken ${wakes}`);
	// A returned waker wakes the waiter's task through Rust until it is disposed, after which
	// a wake throws.
	const waiter = Waiter.new(() => wakes++);
	const waker = Waiter.waker(waiter);
	const context = Waiter.context(waiter);
	waker.wake();
	context.wake();
	close(waiter);
	close(waker);
	close(context);
	close(context);
	throws(() => waker.wake(), Error, "a closed waker throws", (m) => m === "Waker is closed");
	await until(() => wakes === 5, "a returned waker wakes the waiter");
	print(`woken ${wakes}`);

	const nodes = Node.new(1, undefined);
	Node.push(nodes, 2);
	print(`node ${dec(Node.sum(nodes))}`);

	const tally = Tally.new();
	Tally.bump(tally);
	print(`tally ${Tally.bump(tally)} ${Tally.eq(tally, tally)}`);
	print(`both ${api.bumpBoth(tally, tally)}`);
	print(`peek ${api.peek(tally, (lent) => Tally.bump(lent))}`);

	const area = api.grow(Area.new({ width: 2, height: 3 }), 1);
	const areaSize = Area.size(area);
	print(`area ${areaSize.width} ${areaSize.height} ${dec(Area.pixels(area))}`);

	// Generic types, each at the instances it infers, named by their arguments.
	const pair = { left: 2, right: 5 };
	const swapped = PairU32.swap(pair);
	print(`pair ${swapped.left} ${swapped.right} sum ${PairU32.sum(pair)} ${PairU32.debug(pair)} ${PairU32.eq(pair, swapped)}`);
	print(`even ${PairU32.even(pair)} ${PairU32.even({ left: 7, right: 7 })}`);
	// An instance renamed alone, beside another of `Pair` that keeps its name.
	const sized = api.tag({ width: 1, height: 2 }, 3);
	const flipped = PairSizeU8.swap(sized);
	print(`sized ${flipped.left} ${flipped.right.width} ${PairU8Size.debug(flipped)}`);
	const beside = PairU32.besideSize({ left: 258, right: 9 }, sized.left);
	print(`beside ${beside.left} ${PairU8Size.debug(beside)}`);
	// At its default, named as the type itself.
	const stack = Stack.new();
	["a", "b"].forEach((n, i) => Stack.push(stack, Counter.new(n, i + 1)));
	const top = dec(Counter.get(Stack.pop(stack)!));
	print(`stack ${Stack.depth(stack)} Some(${top}) ${Stack.pop(stack) !== undefined} ${Stack.pop(stack) === undefined}`);
	// Generic functions and methods, at each implementation of the trait bounding them, by
	// reference, which leaves JavaScript its object.
	const fromPoint = Counter.seededPoint("p", { x: -3, y: 4 });
	const fromSize = Counter.seededSize("s", { width: 5, height: 6 });
	const fromCounter = Counter.seededCounter("c", fromPoint);
	const fromPolar = Counter.seededPolarPoint("q", { turns: 1, distance: 2 });
	print(
		`seeded ${dec(Counter.get(fromPoint))} ${dec(Counter.get(fromSize))} ${dec(Counter.get(fromCounter))} ` +
			`${dec(Counter.get(fromPolar))} past ${dec(Stack.depthPastPoint(stack, { x: 1, y: 2 }))} ` +
			`seeds ${dec(api.seedsPointSize({ width: 2, height: 3 }))}`,
	);

	// A trait implemented for `Point` and `&Point` apart: one instance.
	print(`weigh ${api.weighPoint({ x: -1, y: 2 })}`);
	// A function named as the type it returns, and one named as a type it does not.
	print(
		`polar ${api.polarPoint(2)!.distance} ${api.polarPoint(0) === undefined} ` +
			`area ${dec(api.area({ width: 2, height: 3 }))}`,
	);
	// Two types whose paths join alike, the second kept apart where its path splits.
	print(`wind ${api.wind({ whole: 2 }, { part: 3 })}`);
	// Names that clash in some language, each kept apart there.
	const clashSpan = { width: 4 };
	const clashStore = ClashStore.new(7);
	print(
		`clash ${ClashTone.debug({ hz: 440 })} ${ClashTone_.debug({ db: 3 })} ${Cancelled.debug({ step: 2 })} ` +
			`span ${ClashSpan.width(clashSpan)} ${ClashSpan.toString_(clashSpan)} ${ClashSpan.toString(clashSpan)} ` +
			`store ${ClashStore.free(clashStore)} ${ClashStore.close(clashStore)} ` +
			`audio ${api.clashAudioEncoderFinish()} ${api.clashAudioEncoderFinish_()} ` +
			`nested ${api.clashNested([[1, 2], 3], [[1, 2, 3]])}`,
	);
	close(clashStore);
	// Instances that name each other, and one made only by an impl of it.
	const cross = api.cross({ value: { value: 1 } }, { value: { value: 2 } });
	print(`cross ${cross} outer ${OuterInnerU8.get(OuterInnerU8.new(4))}`);
	// A bound that outlives the call takes a value, and an instance only a constant names.
	print(`kept ${dec(api.keptPoint({ x: 1, y: -2 }))} boxed ${api.BOXED().value}`);
	print(`keep ${api.keepPoint({ x: 1, y: -2 })}`);
	const five = { x: 5, y: 2 };
	print(`kept or ${api.keptOrPoint(five)} ${api.keptOrPoint(undefined)} keeper ${api.keeperPoint(five)}`);
	// Inferred instances taking a lent closure and a foreign implementation, and one only its
	// impl of a trait names.
	const sides = { left: 2, right: 5 };
	print(`apply ${api.applyU32(sides, (side) => side * 3)}`);
	print(`report ${api.reportU32(sides, new PrintLog())}`);
	print(`wrapped ${dec(api.keptWrapU32({ value: 4 }))}`);
	print(`keeps ${api.keeps(new PrintLog()).value}`);

	// Its Drop runs when JavaScript disposes it, not before.
	const token = Token.new(7);
	print(`token ${Token.idGet(token)} dropped ${api.dropped()}`);
	close(token);
	print(`dropped ${api.dropped()}`);

	// A field and the method of its name, each its own member.
	const retry = Retry.new(5);
	const limit = Retry.limitGet(retry);
	print(`retry ${limit === undefined ? 0 : 1} ${limit ?? 0} ${Retry.limit(retry)} ${Retry.delay(retry)}`);
	// The renamed field's setter, named as its getter.
	Retry.limitSet(retry, undefined);
	print(`retry ${Retry.limit(retry)}`);
	print(`slot ${SlotU8.value(SlotU8.new(200))}`);

	// Foreign types: a URL parsed from text, socket addresses, OS paths, and lists.
	const base = unwrap(Url.parse("https://example.com/a/b"));
	const host = api.host(base);
	print(`url ${Url.toString(base)} host ${host === undefined ? "None" : `Some("${host}")`}`);
	const url = unwrap(api.resolve(Url.clone(base), "c?d"));
	print(`resolve ${Url.toString(url)} ${Url.eq(url, base)}`);
	close(url);
	const unparsedUrl = Url.parse("no scheme");
	if (!unparsedUrl.ok) print(`parse err ${unparsedUrl.error}`);
	const v4: rt.SocketAddrV4 = { ip: { octets: bytes(127, 0, 0, 1) }, port: 8080 };
	const v6: rt.SocketAddrV6 = { ip: { octets: ipv6(1) }, port: 443, flowinfo: 0, scopeId: 3 };
	print(`ports ${list(api.ports([{ tag: "V4", _0: v4 }, { tag: "V6", _0: v6 }]))}`);
	const peer = Peer.next({ addr: { tag: "V6", _0: v6 }, names: ["a", "b"] }, "c");
	print(`peer ${Peer.debug(peer)} ${quoted(Peer.aliases(peer))}`);
	const file = api.file(rt.pathFrom("dir"), "f\\'\"\t");
	print(`file ${escaped(file)}`);
	print(`parents ${api.parents([file, rt.pathFrom("/")]).map((at) => (at === undefined ? "none" : escaped(at))).join(" ")}`);
	const words = api.words(" one two  three ");
	print(`words ${words.length} ${quoted(words)}`);
	print(`sentence ${api.sentence(words)} ${api.sentence(undefined)}`);
	print(`sentence ${api.sentence([])}|`);
	print(`either ${api.either("abc")} ${api.either(undefined)}`);
	print(`prefixed ${text(api.prefixed(enc("cd"), enc("ab")))} ${text(api.prefixed(enc("cd"), undefined))}`);
	print(`coordinates ${api.coordinates({ x: 2, y: 3 })} ${api.coordinates(undefined)}`);
	const two = Counter.new("two", 2);
	const three = Counter.new("three", 3);
	print(`plus ${dec(Counter.plus(two, three))} ${dec(Counter.plus(two, undefined))}`);
	print(`add_to ${Counter.addTo(two, three)} ${Counter.addTo(two, undefined)} ${dec(Counter.get(three))}`);
	close(two);
	close(three);
	const encoded = { v: enc("0,") };
	api.putNumber(1, encoded);
	api.putNumber(23, encoded);
	print(`put_number ${text(encoded.v)}`);
	const entry: rt.Place<[Uint8Array, number]> = { v: [enc("ab"), 1] };
	const stamps = api.pushCounted(entry, 0x63);
	print(`push_counted ${stamps} ${text(entry.v[0])} ${entry.v[1]}`);
	const digits = { v: enc("7x9") };
	const firstDigit = unwrap(api.digit(digits));
	const skipped = !api.digit(digits).ok;
	print(`digit ${firstDigit} ${skipped} ${unwrap(api.digit(digits))} ${digits.v.length}`);
	const nudged = { v: { x: 1, y: 2 } };
	api.nudge(nudged, 3);
	print(`nudge ${nudged.v.x} ${nudged.v.y}`);
	const config = api.secure("h3");
	print(`secure ${api.alpnLent(config)} ${api.alpn(TlsConfig.clone(config))}`);
	const cert = api.certify(enc("der"));
	const certLen = api.certLen(cert);
	print(`cert ${certLen} ${text(api.certDer(cert))} ${api.chainLen([api.certify(enc("a")), api.certify(enc("bc"))])}`);
	close(config);
	// Another crate's trait objects: an `Arc`, taken back, and `'static` ones, held by value.
	print(`resolver ${api.resolved(api.resolver("r1"))}`);
	const codecs = Codecs.new();
	print(`codecs ${Codecs.ids(codecs)}`);
	Codecs.listSet(codecs, Codecs.listGet(codecs).reverse());
	print(`codecs reversed ${Codecs.ids(codecs)}`);
	close(codecs);
	// A standard library value no type maps, in a handle.
	const failure = api.failure("boom");
	print(`failure ${api.failureMessage(failure)} ${api.failureKind(failure)}`);
	// Iterators, passed as the arrays of their items.
	print(`sum_all ${api.sumAll([1, 2, 3])}`);
	print(`weigh_all ${dec(api.weighAllCounter([Counter.new("w", 3)]))}`);
	print(`offered_len ${api.offeredLen([enc("h3"), enc("moq-00")])}`);
	print(`join_words ${api.joinWords(["a", "b", "c"])}`);
	// Bytes that must be `'static`, interned.
	for (const what of ["der", "der again"]) {
		const der = Der.fromSlice(enc("der"));
		print(`${what} ${text(Der.bytes(der))}`);
		close(der);
	}
	const session = unwrap(api.resume("h3", 60));
	print(`resume ${TlsSession.alpn(session)} ${TlsSession.lifetime(session).seconds}`);
	close(session);
	const unresumed = api.resume("", 60);
	if (!unresumed.ok) print(`resume err ${unresumed.error}`);
	const meters = Meters.new();
	const lentMeter = Meters.first(meters);
	Meter.add(lentMeter, 2);
	print(`lent ${dec(Meter.add(lentMeter, 3))} ${dec(Meter.count(Meters.first(meters)))}`);
	close(lentMeter);
	Meters.clear(meters);
	print(`cleared ${dec(Meter.count(Meters.first(meters)))}`);
	const lastMeter = Meters.lastGet(meters);
	Meter.add(lastMeter, 4);
	print(`field lent ${dec(Meter.add(lastMeter, 1))}`);
	close(lastMeter);
	// An optional field that is not Clone, lent as a handle.
	print(`spare ${Meters.spareGet(meters) !== undefined}`);
	Meters.spareSet(meters, Meter.default());
	const spare = Meters.spareGet(meters)!;
	Meter.add(spare, 5);
	print(`spare lent ${dec(Meter.add(spare, 1))}`);
	close(spare);
	close(meters);
	const tallies = Tallies.new();
	Tally.bump(Tallies.first(tallies));
	print(`tallied ${Tally.bump(Tallies.first(tallies))}`);
	close(tallies);
	// A draft the ledger's `edit` lends holds the ledger borrowed exclusively until it closes,
	// and its sheet's members are reached by Deref and DerefMut: the ledger refuses any other
	// use meanwhile.
	const ledger = Ledger.new();
	const edit = (): Draft.Draft => {
		const edited = Ledger.edit(ledger);
		if (!edited.ok) throw new Error("an unsealed ledger lends a draft");
		return edited.value;
	};
	let draft = edit();
	Draft.titleSet(draft, "minutes");
	Draft.write(draft, "first");
	print(`draft ${Draft.titleGet(draft)} ${Draft.lines(draft)} ${Draft.describe(draft)} ${Draft.edited(draft)}`);
	borrowed(() => Ledger.published(ledger), "api_Ledger_published: a handle is borrowed exclusively", "a lent ledger refuses a read");
	print(`committed ${Draft.commit(draft)}`);
	close(draft);
	print(`ledger ${Ledger.title(ledger)} ${Ledger.published(ledger)}`);
	draft = edit();
	print(`read ${Draft.lines(draft)} ${Draft.edited(draft)}`);
	close(draft);
	draft = edit();
	Draft.write(draft, "second");
	close(draft);
	print(`dropped ${Ledger.published(ledger)}`);
	const receipt = Ledger.receipt(ledger);
	print(`receipt ${Receipt.count(receipt)}`);
	close(receipt);
	{
		const small = PadU8.small();
		const wide = PadU16.wide();
		const layered = Layered.new$();
		const marker = Marker.new$("m", 3, 5);
		Marker.ySet(marker, 7);
		print(
			`forwarded ${PadU16.title(wide)} ${Layered.lines(layered)} ${Marker.xGet(marker)} ${Marker.yGet(marker)} ` +
				`[${PadU8.describe(small)}] [${PadU16.describe(wide)}]`,
		);
		for (const handle of [small, wide, layered, marker]) close(handle);
	}
	Ledger.seal(ledger);
	const sealed = Ledger.edit(ledger);
	if (!sealed.ok) print(`sealed ${Error_.toString(sealed.error)}`);
	close(ledger);
	print(`chunks ${list(api.chunks(enc("abcde"), 2).map(list))}`);
	const counters = api.counters(["x", "y"]);
	print(`counters ${counters.map((n) => `${Counter.name(n)} ${dec(Counter.get(n))}`).join(", ")}`);
	// Each counter moves in, so a later call throws.
	print(`worth ${dec(api.worth([counters[0]], [counters[1]]))} ${dec(api.worth(undefined, []))}`);
	for (const counter of counters) closed(() => Counter.get(counter), "Counter", "a counter moved in is closed");
	const held = [api.heldCount([Counter.new("h", 4)]), api.heldCount(undefined)];
	print(`held ${held.map((n) => (n === undefined ? "None" : `Some(${dec(n)})`)).join(" ")}`);
	print(
		`boxed Some("${api.boxedName(api.boxedCounter("b"))}") ` +
			`${api.boxedName(undefined) === undefined ? "None" : "Some"} ${api.boxedCounter(undefined) === undefined}`,
	);
	print(`less ${api.less(5, 2)}`);
	print(`sums ${list(api.sums([new Uint32Array([1, 2]), new Uint32Array(0), new Uint32Array([3])]))}`);
	print(`least ${api.least([[3, 1], [], [2]]).map(list).join(" ")}`);
	numbers("edges", Numbers.edges());
	numbers("reversed", Numbers.reversed(Numbers.edges()));
	const edges = Numbers.edges();
	const nan = Measure.new(NaN, false);
	print(
		`float equal ${Numbers.eq(edges, edges)} ${Numbers.eq(edges, Numbers.edges())} ` +
			`${Numbers.eq(Numbers.zero(true), Numbers.zero(false))} ${Measure.eq(nan, nan)} ` +
			`${Measure.eq(Measure.new(-0, true), Measure.new(0, false))}`,
	);
	print(`sum u32 ${dec(api.sumU32(new Uint32Array(0)))} ${dec(api.sumU32(new Uint32Array([0xffffffff, 0xffffffff, 2])))}`);
	print(`doubled i64 ${list(api.doubledI64(new BigInt64Array([-(1n << 63n), -1n, 0n, (1n << 63n) - 1n])))}`);
	const floats = api.boxedFloats(floats64(1n << 63n, 0x7ff8000000000001n));
	print(`boxed floats ${bits64(floats)}`);
	const summed = api.summed([
		{ x: 1, y: 2 },
		{ x: -4, y: 6 },
	]);
	const noneSummed = api.summed([]);
	print(`summed ${summed.x} ${summed.y} ${noneSummed.x} ${noneSummed.y}`);
	print(`longest ${api.longest(["a", "abc"])} ${api.longest([])}`);
	print(`first of Some("${api.firstOf(["a", "b"])}") ${api.firstOf([]) ?? "None"}`);
	const limits = api.byteLimits(enc("a"), enc("bc"));
	const waits = api.breadth({ low: { secs: 0, nanos: 5_000_000 }, high: { secs: 2, nanos: 0 } });
	print(`limits ${text(limits.low)} ${text(limits.high)} breadth ${num(waits.secs) * 1000 + Math.floor(waits.nanos / 1e6)}`);
	const parts = text(api.joined([enc("ab"), new Uint8Array(0), enc("c")]));
	print(`joined ${parts} ${api.joined([]).length} read ${dec(api.readLen(enc("hello")))}`);
	const doc = unwrap(Json.parse('{"b": [1, 2.5, null], "a": "é"}'));
	print(`json ${api.encode(doc)}`);
	print(`timed text ${api.timedText(doc)}`);
	const tagged = api.tagged(doc, "t");
	print(`tagged ${Json.toText(tagged)}`);
	close(tagged);
	// A JSON object's entries cross as pairs of a key and its document, each moved in.
	const entries = api.entries(
		[
			["b", unwrap(Json.parse("[1]"))],
			["a", unwrap(Json.parse("2"))],
		],
		"t",
	);
	print(`entries ${entries.map(([k, v]) => `${k} ${Json.toText(v)}`).join(", ")}`);
	for (const [, v] of entries) close(v);
	const unparsedJson = Json.parse('{"a":');
	if (!unparsedJson.ok) print(`json parse err ${unparsedJson.error}`);
	const site = Site.new(base);
	print(`site ${Url.toString(Site.homeGet(site))} ${Url.toString(Site.homeGet(site))}`);

	// Slices, passed in and returned as clones, and the socket address of each family.
	const shapes = (values: Shape.Shape[]) => list(values.map(Shape.debug));
	const points = (values: Point.Point[]) => list(values.map(Point.debug));
	const labels = (values: Label.Label[]) => list(values.map(Label.debug));
	print(`shapes ${shapes(Shape.ALL())} greetings ${quoted(api.GREETINGS())}`);
	print(`both ${shapes(Shape.BOTH())} primes ${list(api.PRIMES())}`);
	const trail = Trail.new(
		[
			{ x: 1, y: 2 },
			{ x: 3, y: 4 },
		],
		["a", "b"],
	);
	print(`trail ${points(Trail.points(trail))} ${labels(Trail.labels(trail))}`);
	// Returned iterators, each collected at the call: drained labels are gone after.
	print(`iterated ${points(Trail.iter(trail))} ${quoted(Trail.names(trail))} ${list(Trail.bytes(trail))}`);
	print(`drained ${labels(Trail.drain(trail))} then ${labels(Trail.labels(trail))}`);
	// A filter, collected at the call.
	print(`beyond ${Trail.beyond(trail, 1).map((at) => `${at.x} ${at.y}`).join(", ")}`);
	close(trail);
	const numbered = api.numbered(["p", "q"]);
	print(`numbered ${numbered.map((n) => `${Counter.name(n)} ${dec(Counter.get(n))}`).join(", ")}`);
	const [next4, next6] = api.nexts(v4, v6);
	print(`nexts ${next4.ip.octets.join(".")} ${next4.port} ${next6.port} ${next6.scopeId}`);
	const [nextIp4, nextIp6] = api.nextIps({ octets: bytes(10, 0, 0, 255) }, { octets: ipv6(0xffff) });
	const mapped = api.flip({ tag: "V4", _0: { octets: bytes(127, 0, 0, 1) } });
	const ips = [nextIp4, nextIp6, mapped._0, api.flip(mapped)._0, api.flip({ tag: "V6", _0: { octets: ipv6(1) } })._0];
	print(`ips ${ips.map((ip) => hex(ip.octets)).join(" ")}`);

	// Lossless round trips: each type's edge values, passed through Rust and back.
	const durations = (
		[
			{ secs: 0, nanos: 0 },
			{ secs: 0, nanos: 1 },
			{ secs: 9223372036, nanos: 854775808 },
			{ secs: U64_MAX, nanos: 999999999 },
		] as rt.Duration[]
	).map(api.sameDuration);
	print(`lossless duration ${durations.map((t) => `${dec(t.secs)}.${`${t.nanos}`.padStart(9, "0")}`).join(" ")}`);
	const u64s = [0, rt.u64FromBig(1n << 63n), U64_MAX];
	print(`lossless u64 ${u64s.map((v) => dec(api.sameU64(v))).join(" ")}`);
	// wasm32's `usize` and `isize` are 32 bits.
	print(`lossless usize ${[0, 0xffffffff].map(api.sameUsize).join(" ")}`);
	print(`lossless isize ${api.sameIsize(-0x80000000)} ${api.sameIsize(0x7fffffff)}`);
	const u128s = [0n, 1n << 64n, (1n << 128n) - 1n].map(rt.u128FromBig);
	print(`lossless u128 ${u128s.map((v) => hex128(api.sameU128(v))).join(" ")}`);
	print(`lossless char ${[0, 0xd7ff, 0xe000, 0x10ffff].map((v) => api.sameChar(v).toString(16)).join(" ")}`);
	// A path no Rust string spells: bytes that are not UTF-8.
	const paths = [rt.pathFrom("a/é"), { bytes: bytes(0xff) }];
	print(`lossless path ${paths.map((at) => hex(api.samePath(at).bytes)).join(" ")}`);
	const sockV4 = api.sameSocketAddrV4({ ip: { octets: bytes(255, 255, 255, 255) }, port: 65535 });
	const sockV6 = api.sameSocketAddrV6({
		ip: { octets: bytes(0, 1, ...new Array<number>(13).fill(0), 2) },
		port: 3,
		flowinfo: 0xffffffff,
		scopeId: 0xffffffff,
	});
	const socks = [api.sameSocketAddr({ tag: "V4", _0: sockV4 }), api.sameSocketAddr({ tag: "V6", _0: sockV6 })];
	print(
		`lossless socket ${socketHex({ tag: "V4", _0: sockV4 })} ${socketHex({ tag: "V6", _0: sockV6 })} ` +
			`${socks.map(socketHex).join(" ")}`,
	);
	// A key hashes by Rust's `Hash`: equal keys alike, and these two apart.
	const hash = (n: string) => dec(Key.hash(Key.new(n)));
	print(`key ${hash("a") === hash("a")} ${hash("a") === hash("b")}`);
	const sameIps = [
		api.sameIpAddr({ tag: "V4", _0: { octets: bytes(255, 255, 255, 255) } }),
		api.sameIpAddr({ tag: "V6", _0: { octets: bytes(0xff, 0xff, ...new Array<number>(13).fill(0), 1) } }),
	];
	print(`lossless ip ${sameIps.map((ip) => hex(ip._0.octets)).join(" ")}`);

	// serde_json's map iterator, collected as pairs of a key and its document.
	const sections = api.sections([
		["b", unwrap(Json.parse("[1]"))],
		["a", unwrap(Json.parse("2"))],
	]);
	print(`sections ${sections.map(([k, v]) => `${k} ${Json.toText(v)}`).join(", ")}`);
	for (const [, v] of sections) close(v);
	// An instant only Rust makes, which each call copies out of its handle, and compares.
	const began = api.now();
	const ended = api.later(began, { secs: 1, nanos: 500_000_000 });
	const same = api.later(began, { secs: 0, nanos: 0 });
	const millis = (t: rt.Duration) => num(t.secs) * 1000 + Math.floor(t.nanos / 1e6);
	print(
		`instants ${millis(api.since(ended, began))} ${millis(api.since(began, ended))} ` +
			`${Instant.eq(began, same)} ${Instant.eq(began, ended)}`,
	);
	print(`has_instant ${api.hasInstant(began)} ${api.hasInstant(undefined)}`);
	// tracing's level, and its filter as an optional level whose undefined is `OFF`.
	const verbosity = Verbosity.new("Warn");
	print(`verbosity ${api.spell(Verbosity.filter(verbosity))} ${api.spell(api.louder(Verbosity.filter(verbosity)))}`);
	Verbosity.levelSet(verbosity, "Trace");
	print(
		`verbosity ${Verbosity.levelGet(verbosity) === "Trace"} ${api.spell(Verbosity.filter(verbosity))} ` +
			`${api.spell(api.louder(Verbosity.filter(verbosity)))} ${api.spell(api.louder(undefined))}`,
	);
	close(verbosity);

	// A closure's string, which Rust takes.
	print(`made ${api.made(3, (n) => `${n} times`)}`);
	// A closure taking and returning `Copy` values copies them, never running their `Clone`.
	const nine = Ticket.new(9);
	const renewed = Ticket.number(api.reissue(Ticket.new(7), (t) => Ticket.new(Ticket.number(t) + 1)));
	const copied = Ticket.number(api.reissue(Ticket.new(7), () => nine));
	print(`reissued ${renewed} ${copied}, cloned ${Ticket.clones()}`);
	// A record's traits borrow the checked handle it holds.
	const nines = Mark.of(9);
	const front = api.seat(nines, 1);
	const back = api.seat(nines, 2);
	print(`seat ${Seat.debug(front)} ${Seat.eq(front, api.seat(nines, 1))} ${Seat.eq(front, back)}`);
	// A kept closure: called again at the instant it asks for, then once its waiter wakes,
	// failing with Rust's error.
	const start = Moment.new(0, 1000)!;
	let calls = 0;
	const stopped = await within(
		api.drive(start, (now, waiter) => {
			calls++;
			if (calls === 1) return rt.Ok(Moment.after(now, 5));
			if (calls === 2) {
				print(`drive ${num(Moment.millis(now)) - num(Moment.millis(start))}`);
				const woken = Waiter.waker(waiter);
				woken.wake();
				close(woken);
				return rt.Ok(undefined);
			}
			return rt.Err(`stopped after ${calls}`);
		}),
		"a kept closure wakes its future",
	);
	print(`drive ${stopped}`);
	// A reader the caller implements, then one whose reads fail.
	for (const reader of [new BytesReader(enc("hello world")), new Broken()]) {
		const r = api.tail(reader, 5);
		print(r.ok ? `tail ${text(r.value[0])} ${dec(r.value[1])}` : `tail error ${r.error}`);
	}
	// A seek from the start reaches the reader whole, past `i64::MAX`, as `Cursor` takes one.
	const sought = api.seekTo(new Far(), U64_MAX);
	print(sought.ok ? `sought ${dec(sought.value)}` : `sought error ${sought.error}`);
	print(`non zeros ${api.describeNonZeros(api.nonZeros())}`);
	for (const hop of [undefined, 1, U64_MAX]) {
		const next = api.nextHop(hop);
		print(next === undefined ? "hop None" : `hop Some(${dec(next)})`);
	}
	print(`farther hop ${api.fartherHop({ left: 3, right: 0xffff })}`);
	const local = Local.new();
	const shared = api.localOrNew(local);
	const bumped = Local.bump(local);
	print(`local ${bumped} ${Local.bump(shared)}`);
	print(`local fresh ${Local.bump(api.localOrNew(undefined))}`);
	print(`local either ${api.bumpEither(undefined, shared)}`);
	print(`local later ${Local.bump(await api.localOrNewLater(local))}`);
	print(`local later fresh ${Local.bump(await api.localOrNewLater(undefined))}`);
}

/** The IPv6 address whose last 16 bits are `last`. */
function ipv6(last: number): Uint8Array {
	const octets = new Uint8Array(16);
	octets[14] = last >> 8;
	octets[15] = last & 0xff;
	return octets;
}

/** The futures' semantics beyond the transcript, each checked by an assertion. wasm32 has no
 * threads, so none of the native script's from another thread is here. */
async function futures(): Promise<void> {
	const channel = Channel.new();

	// A wake that lands while the future polls, before it returns pending, as each of
	// `yielded`'s does, is kept: the future polls again rather than waiting for a wake that
	// already happened.
	check((await within(api.yielded(7), "a wake during the poll polls again")) === 7, "yielded returns its value");

	// A value that is not Send stays on JavaScript's one thread, so its check never fails:
	// its futures poll, a callback calls back into it, and disposing it drops it there.
	{
		const local = Local.new();
		const shared = Local.share(local);
		Local.bump(local);
		check(Local.bump(shared) === 2, "a shared local counts together");
		check((await within(Local.later(local), "a local's future")) === 3, "a local's future polls on its thread");
		check((await within(api.localLater(5), "a future that is not Send")) === 10, "a future that is not Send polls");
		check((await Local.pollCount(local)) === 4, "a local's poll counts");
		check(Local.visit(local, () => Local.bump(local)) === 5, "a callback calls back into its local");
		check(Local.onOwnThread(local), "a local stays on its thread");
		const before = api.localsDropped();
		close(local);
		check(api.localsDropped() === before + 1, "a disposed local drops");
		close(shared);
	}

	// A future's output of None is undefined, and its error is its Err.
	check((await api.halved(3)) === undefined, "a future ready with None returns undefined");
	check((await api.halved(4)) === 2, "a future ready with Some returns its value");
	const corner = await api.cornerLater();
	check(corner[0] === 1 && corner[1] === 1, "a future's borrowed tuple returns whole");
	// A future's error borrowing its argument crosses as its message, as `parseCert`'s does.
	const parsed = await api.parseLater("cert");
	check(parsed.ok && parsed.value === 4, "a future with a borrowing error returns Ok");
	const unparsed = await api.parseLater("");
	check(!unparsed.ok && unparsed.error === "malformed: ", "a future's borrowing error is its message");
	Channel.send(channel, new Uint8Array(0));
	const empty = await Channel.recvNonempty(channel);
	check(!empty.ok && empty.error.tag === "Underflow" && empty.error.by === 1, "a future's error reads back");

	// Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	// for the next.
	const woken = Channel.recv(channel);
	Channel.send(channel, enc("kept"));
	woken.cancel();
	await cancelled(woken, "a receive cancelled after its wake rejects with Cancelled");
	// Its wake, still queued, finds it gone.
	await step();
	check(text(await within(Channel.recv(channel), "the next receive")) === "kept", "a cancelled receive loses nothing");

	// Cancelling a receive that waits for no wake ends it.
	const waiting = Channel.recv(channel);
	await step();
	waiting.cancel();
	waiting.cancel();
	await cancelled(waiting, "a waiting receive cancelled rejects with Cancelled");

	// Cancelling a finished future does nothing.
	const done = api.halved(4);
	check((await done) === 2, "halved returns its value");
	done.cancel();
	check((await done) === 2, "a finished future cancelled keeps its value");

	// Disposing a handle a call awaiting on it borrows throws, as Rust refuses a drop under a
	// borrow, and changes nothing: the call ends as it would, and then the dispose passes.
	const other = Channel.new();
	const held = Channel.recv(other);
	const sender = Channel.clone(other);
	borrowed(() => close(other), "api_Channel_drop: a handle is already borrowed", "a dispose under an awaiting call");
	Channel.send(sender, enc("held"));
	check(text(await within(held, "a receive the dispose left")) === "held", "the awaiting call ends as it would");
	close(other);
	closed(() => Channel.clear(other), "Channel", "a call after the dispose");
	close(sender);

	// A future holding an implementation drops it as it finishes, closing it.
	const log = new CountLog();
	check(await api.logLater(log), "the future returns what its log kept");
	check(log.lines === 1 && log.closes === 1, "a finished future closes the implementation it held");

	// Promises Rust polls, whose settling wakes it.
	let ran = false;
	const first = step().then(() => {
		ran = true;
	});
	check((await api.after(first, 4)) === 8 && ran, "Rust awaits a promise");
	const gate = Promise.withResolvers<void>();
	let after = 0;
	const doubled = api.after(gate.promise, 5).then((value) => (after = value));
	await step();
	await step();
	check(after === 0, "Rust waits on a promise");
	gate.resolve();
	await within(doubled, "a promise's settling wakes Rust");
	check(after === 10, "a promise's settling wakes Rust");

	// Given up on, a call into Rust passed back as a promise is cancelled.
	const kept = Channel.new();
	const recv = Channel.recv(kept);
	const gaveUp = await api.within(0, recv);
	check(!gaveUp.ok && gaveUp.error.tag === "Overflow", "Rust gives up on a promise");
	await cancelled(recv, "Rust's drop of a call into Rust it was given cancels it");
	close(kept);

	// A promise that rejects once Rust dropped it aborts nothing.
	const late = Promise.withResolvers<void>();
	const lateGaveUp = await api.within(0, late.promise);
	check(!lateGaveUp.ok && lateGaveUp.error.tag === "Overflow", "Rust gives up on a promise");
	late.reject(new Error("failed after Rust dropped it"));
	await step();
	check(api.halved(4) !== undefined, "a call after a dropped promise rejected");

	// A poll of a waiter, of a promise, which its future owns.
	const gated = Promise.withResolvers<void>();
	let polled = false;
	const polling = Waiter.pollFuture(gated.promise).then(() => (polled = true));
	await step();
	await step();
	check(!polled, "a poll of a pending promise pends");
	gated.resolve();
	await within(polling, "a promise's settling wakes the poll of it");
	// Cancelled before it finishes, it drops the future it owns: here a call into Rust,
	// which is cancelled in turn.
	const inner = Waiter.pollFuture(Promise.withResolvers<void>().promise);
	const outer = Waiter.pollFuture(inner);
	await step();
	outer.cancel();
	await cancelled(outer, "a poll of a promise cancels");
	await cancelled(inner, "a cancelled poll drops the future it owns");
	close(channel);
	print("futures ok");
}

/** Node's garbage collector, which a flag exposes. */
let gc: (() => void) | undefined;

/** Run the garbage collector until `done()`, failing past a deadline. */
async function collect(done: () => boolean, what: string): Promise<void> {
	if (gc === undefined) {
		process.getBuiltinModule("node:v8").setFlagsFromString("--expose-gc");
		gc = process.getBuiltinModule("node:vm").runInNewContext("gc");
	}
	const deadline = Date.now() + 10000;
	while (!done()) {
		check(Date.now() < deadline, what);
		gc!();
		await step();
	}
}

/** Collect a few times, and let what the collector found finalize. */
async function settle(): Promise<void> {
	for (let i = 0; i < 3; i++) await collect(() => true, "a collection");
	for (let i = 0; i < 3; i++) {
		gc!();
		await step();
	}
}

/** What frees an object nobody disposed: disposing at once, or the garbage collector once it
 * is unreachable, and only what disposing did not free already. */
async function owned(): Promise<void> {
	for (let n = 65535; n <= 65537; n++) {
		const payload = new Uint8Array(n).map((_, i) => i % 256);
		const packet = api.nextPacket({ seq: 1, payload });
		check(
			packet.seq === 2 && packet.payload.length === n && packet.payload.every((b, i) => b === i % 256),
			`a record's ${n} bytes cross both ways`,
		);
	}
	const before = api.dropped();
	const token = Token.new(1);
	close(token);
	check(api.dropped() === before + 1, "disposing frees at once");
	close(token);
	check(api.dropped() === before + 1, "disposing again frees nothing");
	closed(() => Token.idGet(token), "Token", "a disposed token refuses a read");

	unclosed();
	await collect(() => api.dropped() >= before + 2, "the garbage collector frees what nobody disposed");
	await settle();
	check(api.dropped() === before + 2, "the garbage collector frees each once");

	// An implementation an object the garbage collector frees held is closed.
	const log = new CountLog();
	unclosedLogger(log);
	await collect(() => log.closes === 1, "the garbage collector frees what an object it frees holds");
	// So is a returned waker nobody disposed, once.
	let wakes = 0;
	unclosedWaker(() => wakes++);
	await settle();
	check(wakes === 0, "a collected waker never wakes");
	targets();
	print("owned ok");
}

function unclosed(): void {
	Token.new(3);
}

function unclosedLogger(log: Log): void {
	Logger.new(log);
}

function unclosedWaker(wake: () => void): void {
	Waiter.waker(Waiter.new(wake));
}

/** The binding serves wasm32 alone, whose items are each here. */
function targets(): void {
	check(api.pointerBits(api.platform()) === 32, "wasm32's variant crosses");
	check(api.placement().tag === "Anywhere", "a variant wasm32 lacks is left out with its record");
	const labelled = api.sameLabelled({ tag: "Tagged", tag$: 7 });
	check(Labelled.debug(labelled) === "Tagged { tag: 7 }", "a variant's field named `tag` crosses as Rust reads it");
	check(api.pointerBits({ tag: "Native", _0: 64 }) === 64, "the native variant crosses too");
	check(api.isFamily("Wasm") && !api.isFamily("Native"), "wasm32's plain variant crosses");
	check(api.wasmOnly() === 32, "an item only wasm32 has");
	const taken = api.wasmTake(Counter.new("t", 5));
	check(taken.ok && taken.value === 5, "an item only wasm32 has takes a counter");
	const log = new CountLog();
	api.wasmLog(log);
	check(log.lines === 1 && log.closes === 1, "an item only wasm32 has takes an implementation");
}

/** Each call taking a closed object throws, however it was closed, and moves nothing. */
async function closedObjects(): Promise<void> {
	const c = Counter.new("c", 1);
	close(c);
	closed(() => Counter.get(c), "Counter", "a method of a disposed object");
	// `using` disposes at the end of its scope, as stable Safari, which lacks `Symbol.dispose`,
	// needs the runtime's own symbol for, however the bundler lowers it. No garbage collector
	// is involved, so every browser runs it.
	let scoped: ReturnType<typeof Counter.new> | undefined;
	{
		using inner = Counter.new("u", 3);
		scoped = inner;
		check(Counter.get(inner) === 3, "a `using` object is open in its scope");
	}
	closed(() => Counter.get(scoped!), "Counter", "a `using` object after its scope");
	closed(() => Counter.clone(c), "Counter", "clone of a disposed object");
	closed(() => Counter.debug(c), "Counter", "debug of a disposed object");
	const url = unwrap(Url.parse("https://example.com"));
	close(url);
	closed(() => Url.toString(url), "Url", "toString of a disposed object");
	const tally = Tally.new();
	close(tally);
	closed(() => Tally.eq(tally, Tally.new()), "Tally", "eq of a disposed object");
	closed(() => Tally.eq(Tally.new(), tally), "Tally", "eq with a disposed object");
	const registry = Registry.new(Counter.new("first", 1));
	close(registry);
	closed(() => Registry.firstGet(registry), "Registry", "a field's getter of a disposed object");
	const second = Counter.new("second", 2);
	closed(() => Registry.firstSet(registry, second), "Registry", "a field's setter of a disposed object");
	check(Counter.get(second) === 2, "a call that throws moves nothing in");

	// An argument, alone, or in a list, beside others the call then moves none of.
	closed(() => api.bumpBoth(Tally.new(), tally), "Tally", "a disposed argument");
	const open = Counter.new("open", 3);
	closed(() => api.worth([open, c], []), "Counter", "a disposed object in a list");
	closed(() => api.worth([open], [c]), "Counter", "a disposed object in a later list");
	// Disposed while a later argument lowers, after its pointer was passed: the call fails
	// Rust's check of it, and its handle is freed only once the call lets it go, so no other
	// handle made meanwhile takes its place.
	const early = Counter.new("early", 4);
	const kept: Counter.Counter[] = [];
	const disposing = new Proxy([] as Counter.Counter[], {
		get(target, key, receiver) {
			if (key === "length") {
				close(early);
				kept.push(Counter.new("meanwhile", 5));
			}
			return Reflect.get(target, key, receiver);
		},
	});
	closed(() => api.worth([early], disposing), "Counter", "an object disposed while a later argument lowers");
	check(Counter.get(kept[0]!) === 5, "a handle made meanwhile is its own");
	close(kept[0]!);
	closed(
		() =>
			Board.new([
				["open", open],
				["closed", c],
			]),
		"Counter",
		"a disposed object in a tuple",
	);
	check(Counter.get(open) === 3, "a call that throws moves none of its other arguments");

	// Nor keeps an implementation beside an argument that throws: one the binding refuses
	// Rust never takes, and one Rust's check refuses Rust drops.
	const spared = new CountLog();
	const digest = Digest.new();
	throws(() => Digest.flipped(digest, bytes(1, 2, 3)), RangeError, "3 bytes are no [u8; 4]");
	close(digest);
	throws(() => api.logCounts(spared, 256, open, []), RangeError, "an integer out of range beside an implementation");
	closed(() => api.logCounts(spared, 0, c, []), "Counter", "a disposed object beside an implementation");
	check(spared.closes === 0, "a call the binding refuses gives Rust no implementation");
	// A moved value's handle stays open, so Rust's own check refuses it.
	const taken = Counter.new("taken", 1);
	Counter.intoLabel(taken, undefined);
	closed(() => api.logCounts(spared, 0, taken, []), "Counter", "a moved object beside an implementation");
	closed(() => api.logCounts(spared, 0, open, [taken]), "Counter", "a moved object in a list beside an implementation");
	check(spared.closes === 2, "a call whose check throws drops the implementation it gave Rust");
	check(api.onFirstThread(), "a call after one that threw");
	check(api.logCounts(spared, 0, open, []) === 1, "an implementation a call that threw gave none of");
	check(spared.lines === 1 && spared.closes === 3, "an implementation given once is closed once");

	// An async call.
	const channel = Channel.new();
	close(channel);
	await closedCall(() => Channel.recv(channel), "Channel", "an async call of a disposed object");

	// Closed by a call that moved it, plain or boxed, or once the closure it was lent to
	// returned.
	const moved = Counter.new("m", 0);
	Counter.intoLabel(moved, undefined);
	closed(() => Counter.get(moved), "Counter", "a method of a moved object");
	const tail = Node.new(1, undefined);
	Node.new(2, tail);
	closed(() => Node.sum(tail), "Node", "a method of a boxed moved object");
	let lent: Counter.Counter | undefined;
	Registry.edit(Registry.new(Counter.new("c", 0)), (counter) => {
		lent = counter;
		return false;
	});
	closed(() => Counter.get(lent!), "Counter", "a lent object kept past its closure");

	// Disposed from the closure a call lends while the call borrows it, an object throws, as
	// Rust refuses a drop under a borrow, and changes nothing; disposed after, it closes.
	const editing = Registry.new(Counter.new("e", 4));
	const edited = Registry.edit(editing, (counter) => {
		borrowed(() => close(editing), "api_Registry_drop: a handle is already borrowed", "a dispose in a closure the call lends");
		return Counter.get(counter) === 4;
	});
	check(edited, "the call whose object a dispose failed on runs on");
	close(editing);
	closed(() => Registry.firstGet(editing), "Registry", "an object disposed after the call");
	// The same for one the binding counts, as the drop of what it holds shows: once, at the
	// dispose.
	let logger: Logger.Logger | undefined;
	const log = new CountLog(() =>
		borrowed(() => close(logger!), "Logger: a handle is already borrowed", "a dispose during a call it is held by"),
	);
	logger = Logger.new(log);
	check(Logger.write(logger, "x"), "the call whose object a dispose failed on returns");
	check(log.closes === 0, "a dispose that failed frees nothing");
	close(logger);
	check(log.closes === 1, "a dispose after the call frees it");
	close(logger);
	check(log.closes === 1, "disposing it again frees nothing");
	closed(() => Logger.write(logger!, "x"), "Logger", "a call after the dispose");
	print("closed ok");
}

/** The call `f` throws, or rejects with, the error of a closed `type`, as [closed] says. */
async function closedCall(f: () => Promise<unknown>, type: string, what: string): Promise<void> {
	let call: Promise<unknown>;
	try {
		call = f();
	} catch {
		closed(f, type, what);
		return;
	}
	try {
		await within(call, what);
	} catch (e) {
		closed(() => {
			throw e;
		}, type, what);
		return;
	}
	check(false, what);
}

/** Bytes a call returns up to 64 are a copy, Rust's freed at the call; past that they lend
 * Rust's buffer, freed once disposed and no call views it, or by the garbage collector, and
 * passed back to Rust they share it. Either reads the same, and refuses a read once disposed. */
async function lent(): Promise<void> {
	let before = api.dropped();
	const small = api.tokenBytes(1, 64);
	check(api.dropped() === ++before, "64 bytes are copied and Rust's freed at the call");
	check(small.length === 64 && [...small].every((b) => b === 1), "copied bytes read back");
	close(small);
	closed(() => small.at(0), "Bytes", "disposed copied bytes refuse a read");
	for (const n of [65, 65537]) {
		const big = api.tokenBytes(2, n);
		check(api.dropped() === before, `${n} bytes lend Rust's buffer`);
		const copy = big.slice();
		check(big.length === n && copy.length === n && copy.every((b) => b === 2), `${n} lent bytes read back`);
		check(big.at(-1) === 2 && big.at(n) === undefined && big.slice(1, 3).length === 2, "lent bytes read as a Uint8Array's do");
		// A copy out is the caller's, which reaches nothing of Rust's.
		copy[0] = 3;
		check(big.at(0) === 2, "a copy out of lent bytes is the caller's");
		close(big);
		check(api.dropped() === ++before, `disposing ${n} lent bytes frees Rust's buffer`);
		close(big);
		check(api.dropped() === before, "disposing them again frees nothing");
		closed(() => big.slice(), "Bytes", "disposed lent bytes refuse a read");
		closed(() => api.len(big), "Bytes", "disposed lent bytes refuse a call");
	}
	// Passed back, lent bytes cross in place: viewed for a call, or a second owner of Rust's
	// buffer for one that keeps them, so the buffer outlives the bytes disposed after.
	const sent = api.tokenBytes(5, 65537);
	check(api.len(sent) === 65537, "lent bytes viewed for a call");
	const channel = Channel.new();
	Channel.send(channel, sent);
	close(sent);
	check(api.dropped() === before, "lent bytes sent share Rust's buffer");
	const got = await within(Channel.recv(channel), "a receive of what was sent");
	check(got.length === 65537 && got.at(0) === 5 && got.at(-1) === 5, "lent bytes sent read back");
	close(got);
	check(api.dropped() === ++before, "the last owner of lent bytes frees them");
	close(channel);
	// Disposed while a later argument lowers, lent bytes stay Rust's for the call it is in,
	// and are freed once it returns.
	const viewed = api.tokenBytes(6, 100);
	const prefix = new Proxy(enc("ab"), {
		get(target, key) {
			if (key === "length") close(viewed);
			return Reflect.get(target, key);
		},
	});
	const joined = api.prefixed(viewed, prefix).slice();
	check(joined.length === 102 && text(joined.subarray(0, 2)) === "ab" && joined.subarray(2).every((b) => b === 6), "bytes a call views read whole");
	check(api.dropped() === ++before, "bytes disposed during a call are freed once it returns");
	// A Bytes' length is its own: no caller's write or definition changes what Rust reads.
	const fixed = api.tokenBytes(11, 100);
	throws(() => {
		(fixed as { length: number }).length = 1 << 20;
	}, TypeError, "a Bytes' length is read-only");
	Object.defineProperty(fixed, "length", { value: 1 << 20 });
	check(api.len(fixed) === 100 && fixed.slice().length === 100, "a Bytes' length shadowed reaches no read");
	close(fixed);
	check(api.dropped() === ++before, "bytes whose length was shadowed are freed whole");
	// Bytes a call takes by value, which Rust gets a second owner of, disposed while a later
	// argument lowers, read whole and are freed once Rust drops them.
	const given = api.tokenBytes(10, 100);
	const high = new Proxy(enc("bc"), {
		get(target, key) {
			if (key === "length") close(given);
			return Reflect.get(target, key);
		},
	});
	const limits = api.byteLimits(given, high);
	check(limits.low.length === 100 && limits.low.every((b) => b === 10) && text(limits.high) === "bc", "bytes given and disposed during a call read whole");
	check(api.dropped() === ++before, "bytes given and disposed during a call are freed once Rust drops them");
	// Bytes a call returned pass as an optional borrowed argument too.
	const optional = api.tokenBytes(8, 100);
	const suffixed = api.prefixed(enc("cd"), optional).slice();
	check(suffixed.length === 102 && suffixed.subarray(0, 100).every((b) => b === 8) && text(suffixed.subarray(100)) === "cd", "returned bytes pass as an optional argument");
	close(optional);
	check(api.dropped() === ++before, "returned bytes passed as an optional argument stay the caller's");
	// The garbage collector frees lent bytes nobody disposed.
	unclosedBytes();
	await collect(() => api.dropped() > before, "the garbage collector frees lent bytes nobody disposed");
	await settle();
	check(api.dropped() === ++before, "the garbage collector frees lent bytes once");
	// Past a quarter of memory lent and not yet freed, bytes are copied, so bytes nobody
	// disposed, which the collector frees only once its own heap prompts it, grow memory by
	// no more than a third.
	const kept: rt.Bytes[] = [];
	for (let lending = true; lending; ) {
		const dropped = api.dropped();
		kept.push(api.tokenBytes(7, 1 << 20));
		lending = api.dropped() === dropped;
		check(kept.length < 1000, "lent bytes copy past a quarter of memory");
	}
	check((kept.length - 1) << 20 <= rt.mem().buffer.byteLength / 4, "at most a quarter of memory is lent");
	before = api.dropped();
	for (const b of kept) close(b);
	check(api.dropped() === before + kept.length - 1, "disposing frees each lent");
	// A slice keeps its whole buffer, which its length does not show, so lending also stops
	// once memory has grown by a third since none were lent.
	const slices: rt.Bytes[] = [];
	for (let lending = true; lending; ) {
		const dropped = api.dropped();
		slices.push(api.tokenSlice(9, 100, 1 << 20));
		lending = api.dropped() === dropped;
		check(slices.length < 1000, "slices of lent buffers copy once memory grows a third");
	}
	check(slices.length > 1 && slices[0]!.at(99) === 9, "a slice of a buffer lends");
	before = api.dropped();
	for (const b of slices) close(b);
	check(api.dropped() === before + slices.length - 1, "disposing frees each slice lent");
	print("bytes ok");
}

/** Lent bytes, left for the garbage collector. */
function unclosedBytes(): void {
	api.tokenBytes(8, 1000);
}

/** Values at the edges of what Rust takes and returns, and each value Rust's type lacks,
 * which throws the language's own error rather than reaching Rust. */
function bounds(): void {
	// A char is a Unicode scalar value: a surrogate, or a number past one, is none.
	check(api.nextChar(0x61) === 0x62 && api.nextChar(0x1f600) === 0x1f601, "a char");
	for (const c of [0xd800, 0xdfff, 0x110000, -1, 1.5]) throws(() => api.nextChar(c), RangeError, `${c} is no char`);
	check(api.nextChar(0xd7ff) === undefined, "the scalar before the surrogates");
	throws(() => rt.u128FromBig(1n << 128n), RangeError, "an int past u128");
	throws(() => rt.u128FromBig(-1n), RangeError, "a negative int for a u128");

	// Text is UTF-8 in Rust, which a lone surrogate has no form in.
	check(api.sameString("\u{1f600}é\u{10ffff}") === "\u{1f600}é\u{10ffff}", "a pair of surrogates");
	check(api.sameString("\u{feff}hi") === "\u{feff}hi", "a leading U+FEFF is text");
	check(rt.pathText(api.samePath(rt.pathFrom("\u{feff}a"))) === "\u{feff}a", "a path's leading U+FEFF is text");
	for (const lone of ["\ud800", "a\udc00", "\udbff\ud800", "\udc00\ud800", "é\ud800"]) {
		throws(() => api.sameString(lone), RangeError, `a lone surrogate in ${[...lone].map((u) => u.charCodeAt(0))}`);
	}
	throws(() => rt.pathFrom("\udc00"), RangeError, "a path of a lone surrogate");
	throws(() => api.nextHop(0), RangeError, "a zero NonZero");
	throws(() => api.fartherHop({ left: 0, right: 1 }), RangeError, "a zero NonZero field");

	// An integer is a number of its range, or for one past 2^53, its limbs.
	const max = rt.u64FromBig(0xffffffffffffffffn);
	check(rt.u64ToBig(api.sameU64(max)) === 0xffffffffffffffffn, "u64::MAX crosses whole");
	check(api.sameU64(2 ** 53 - 1) === 2 ** 53 - 1, "the last safe integer is a number");
	check(rt.u64ToBig(api.sameU64(rt.u64FromBig(2n ** 53n))) === 2n ** 53n, "the first unsafe one is limbs");
	for (const v of [-1, 1.5, 2 ** 53, NaN]) throws(() => api.sameU64(v), RangeError, `${v} is no u64`);
	throws(() => rt.u64FromBig(1n << 64n), RangeError, "a bigint past u64");
	throws(() => rt.u64FromBig(-1n), RangeError, "a negative bigint for a u64");
	throws(() => rt.i64FromBig(1n << 63n), RangeError, "a bigint past i64");
	check(rt.i64ToBig(rt.i64FromBig(-(1n << 63n))) === -(1n << 63n), "i64::MIN as its limbs");
	for (const v of [-1, 2 ** 32]) throws(() => api.sameUsize(v), RangeError, `${v} is no usize`);
	for (const v of [-(2 ** 31) - 1, 2 ** 31]) throws(() => api.sameIsize(v), RangeError, `${v} is no isize`);
	throws(() => Point.offset({ x: 2 ** 31, y: 0 }, 0, 0), RangeError, "a field past i32");
	throws(() => api.greet("x", "Triangle" as Shape.Shape), RangeError, "no variant of a plain enum");
	throws(() => api.describe({ tag: "Gone" } as unknown as Event.Event), RangeError, "no variant of a data enum");
	throws(() => Score.count(Moment.new(1, 1)! as unknown as Score.Score), RangeError, "a value Rust made of another type");

	// A Rust Duration is its seconds and nanoseconds, each checked.
	const longest = { secs: max, nanos: 999999999 };
	const same = api.extended(longest, { secs: 0, nanos: 0 })!;
	check(rt.u64ToBig(same.secs) === 0xffffffffffffffffn && same.nanos === 999999999, "the longest Duration crosses whole");
	check(api.extended(longest, { secs: 0, nanos: 1 }) === undefined, "past the longest");
	for (const nanos of [1000000000, -1, 0.5]) {
		throws(() => api.sameDuration({ secs: 0, nanos }), RangeError, `${nanos} nanoseconds`);
	}

	// A path is its bytes, which a string may not hold.
	const stray = { bytes: bytes(0xff) };
	const back = api.samePath(stray);
	check(back.bytes.length === 1 && back.bytes[0] === 0xff, "a path that is not UTF-8");
	check(rt.pathText(back) === "�", "a path's text replaces what is not UTF-8");
	check(rt.pathText(rt.pathFrom("a/é")) === "a/é", "a UTF-8 path as text");

	// An address is its family's octets, and a port, flow label, and scope their ranges.
	throws(() => api.sameIpAddr({ tag: "V4", _0: { octets: bytes(1, 2, 3) } }), RangeError, "three octets");
	throws(() => api.sameIpAddr({ tag: "V6", _0: { octets: bytes(1, 2, 3, 4) } }), RangeError, "four octets for IPv6");
	const ip = { octets: ipv6(1) };
	const v6 = (port: number, flowinfo: number, scopeId: number) => ({ ip, port, flowinfo, scopeId });
	check(api.ports([{ tag: "V6", _0: v6(65535, 0, 0) }])[0] === 65535, "the last port");
	throws(() => api.sameSocketAddrV6(v6(65536, 0, 0)), RangeError, "a port past u16");
	throws(() => api.sameSocketAddrV6(v6(1, 0, -1)), RangeError, "a negative scope");
	throws(() => api.sameSocketAddrV6(v6(1, 2 ** 32, 0)), RangeError, "a flow label past u32");

	// A key hashes by Rust's Hash, so its hash keys a Set.
	const hashes = new Set(["a", "a", "b"].map((n) => dec(Key.hash(Key.new(n)))));
	check(hashes.size === 2, "a set of keys' hashes");

	// A block crosses whole: an empty one stays apart from none, and a u64 keeps its bits.
	const blocks = (some: Int32Array | undefined): Numbers.Numbers => ({
		u16s: new Uint16Array(0),
		u32s: new Uint32Array(0),
		u64s: new BigUint64Array([0xffffffffffffffffn]),
		i8s: new Int8Array(0),
		i16s: new Int16Array(0),
		i32s: new Int32Array(0),
		i64s: new BigInt64Array(0),
		f32s: new Float32Array(0),
		f64s: new Float64Array(0),
		some,
	});
	check(Numbers.reversed(blocks(undefined)).some === undefined, "no block crosses as none");
	check(Numbers.reversed(blocks(new Int32Array(0))).some?.length === 0, "an empty block crosses as itself");
	check(Numbers.reversed(blocks(undefined)).u64s[0] === 0xffffffffffffffffn, "u64::MAX crosses whole in a block");
	const thousand = new Uint32Array(1000).map((_, i) => i);
	check(api.sumU32(thousand) === 499500, "a thousand u32s cross whole");
	check(api.sumU32(thousand.subarray(1, 3)) === 3, "a view of a block crosses as itself");
	print("ok");
}

/** The bytes the last [grow] asked Rust for. */
let growth = 0;

/** Grow memory from a call into JavaScript, by asking Rust for more bytes than it has free:
 * at first as many as memory holds, then a MiB more each time. */
function grow(): void {
	const was = rt.mem().buffer.byteLength;
	growth = Math.max(growth + (1 << 20), was);
	check(api.tokenBytes(2, growth).length === growth, "the bytes asked for");
	check(rt.mem().buffer.byteLength > was, "the call grew memory");
}

/** What a callback or closure is lent, bytes or text, reads the same once it grows memory,
 * and what it returns or writes lands. */
function grown(): void {
	let lines = 0;
	const log = new CountLog((line) => {
		grow();
		check(line === `line ${++lines}`, `a callback's lent text after memory grew: ${line}`);
	});
	check(api.emit(log, bytes(1, 2, 3)) === 3, "a callback that grew memory returns");
	const filtered = api.filter(bytes(1, 2, 3), (level, line) => {
		grow();
		check(line === `line ${level}`, `a closure's lent text after memory grew: ${line}`);
		return level > 1;
	});
	check(filtered === 2, "a closure that grew memory returns");
	const registry = Registry.new(Counter.new("grown", 3));
	const edited = Registry.edit(registry, (counter) => {
		grow();
		return Counter.name(counter) === "grown" && unwrap(Counter.add(counter, 1)) === 4;
	});
	check(edited && Counter.get(Registry.firstGet(registry)) === 4, "a lent handle after memory grew");
	close(registry);
	check(api.made(2, (n) => (grow(), `${n} grown`)) === "2 grown", "a closure's text returned after memory grew");
	// A reader fills the buffer Rust lends it after growing memory.
	const tail = api.tail(new BytesReader(enc("hello world"), grow), 5);
	check(tail.ok && text(tail.value[0]) === "world" && tail.value[1] === 6, "a reader's read after memory grew");
	// Lent bytes read Rust's buffer through memory as it is now, and pass back in place.
	const held = api.tokenBytes(4, 1000);
	grow();
	check(held.at(999) === 4 && held.slice().every((b) => b === 4), "lent bytes read after memory grew");
	check(api.len(held) === 1000, "lent bytes viewed for a call after memory grew");
	const channel = Channel.new();
	Channel.send(channel, held);
	grow();
	// What grew memory dropped tokens of its own.
	const before = api.dropped();
	Channel.clear(channel);
	close(channel);
	check(api.dropped() === before && held.at(0) === 4, "lent bytes shared after memory grew");
	close(held);
	check(api.dropped() === before + 1, "lent bytes disposed after memory grew free Rust's buffer");
	print("grown ok");
}

/** A reader whose seek throws. */
class Thrower implements rt.Reader {
	read(_buffer: Uint8Array): number {
		return 0;
	}

	seek(_offset: rt.u64 | rt.i64, _from: rt.SeekFrom): rt.u64 {
		throw new Error("a seek failed");
	}
}

/** A reader that reads more than it was asked. */
class Overreader implements rt.Reader {
	read(buffer: Uint8Array): number {
		return buffer.length + 1;
	}

	seek(_offset: rt.u64 | rt.i64, _from: rt.SeekFrom): rt.u64 {
		return 0;
	}
}

/** Each breaks a contract, which must fail with a message naming it. */
async function broken(which: string): Promise<void> {
	switch (which) {
		case "moved": {
			const c = Counter.new("c", 0);
			Counter.intoLabel(c, undefined);
			Counter.get(c);
			break;
		}
		case "boxed moved": {
			const tail = Node.new(1, undefined);
			Node.new(2, tail);
			Node.sum(tail);
			break;
		}
		case "outlived": {
			const meters = Meters.new();
			const first = Meters.first(meters);
			Meters.clear(meters);
			Meter.count(first);
			break;
		}
		case "lent moved":
			Registry.edit(Registry.new(Counter.new("c", 0)), (counter) => {
				Counter.intoLabel(counter, undefined);
				return false;
			});
			break;
		case "lent kept": {
			let kept: Counter.Counter | undefined;
			Registry.edit(Registry.new(Counter.new("c", 0)), (counter) => {
				kept = counter;
				return false;
			});
			Counter.get(kept!);
			break;
		}
		case "callback threw":
			api.emit(
				new CountLog(() => {
					throw new Error("a log failed");
				}),
				bytes(1),
			);
			break;
		case "later poll trapped": {
			// A trap in a later poll, Rust's own with nothing of JavaScript's thrown, rejects its
			// call, and a call woken behind it rejects too.
			// Each pends, keeping its last waker, until both are woken at once.
			let woken = false;
			const wakers: { wake(): void }[] = [];
			const pend = (fail: boolean) =>
				api.drive(Moment.new(0, 1000)!, (_now, waiter) => {
					if (!woken) {
						wakers[Number(fail)] = Waiter.waker(waiter);
						return rt.Ok(undefined);
					}
					if (fail) api.tail(new Overreader(), 1);
					return rt.Err("woken");
				});
			const settled = (p: Promise<unknown>) => p.then(() => "resolved", (e: unknown) => e);
			const failing = settled(pend(true));
			const queued = settled(pend(false));
			setTimeout(() => {
				woken = true;
				wakers[1].wake();
				wakers[0].wake();
			});
			const [first, second] = await Promise.all([failing, queued]);
			// Each engine words a trap its own way, as V8's "unreachable" and SpiderMonkey's
			// "unreachable executed".
			check(first instanceof WebAssembly.RuntimeError, `a later poll's trap rejects: ${String(first)}`);
			const message = second instanceof Error ? second.message : String(second);
			check(message === "the module aborted, so no call into it runs", `a call woken behind it rejects: ${message}`);
			print("later poll checked");
			throw new Error(message);
		}
		case "closure threw":
			await api.drive(Moment.new(0, 1000)!, () => {
				throw new Error("a poll failed");
			});
			break;
		// What a reader throws is its io::Error, as rt.Reader says, so what breaks its contract
		// is a read of more than it was asked.
		case "reader threw": {
			const r = api.tail(new Thrower(), 1);
			check(!r.ok && r.error === "a seek failed", "a reader's throw is its error");
			api.tail(new Overreader(), 1);
			break;
		}
		default:
			check(false, `no case ${which}`);
	}
}

async function main(): Promise<void> {
	const path = process.env["RUST2_WASM"];
	check(path !== undefined, "RUST2_WASM names the module");
	const module = await WebAssembly.compile(process.getBuiltinModule("node:fs").readFileSync(path!));
	// A load that fails leaves the next free to succeed, and a load while one runs is refused.
	const glue = { prefix: "api", space: "api_web", file: "stale.ts", contract: 1n, target: 0, imports: g.imports };
	const message = (p: Promise<void>) => p.then(() => "", (e: unknown) => (e instanceof Error ? e.message : String(e)));
	check((await message(rt.load(module, glue))) === "stale.ts and the module are from different builds", "a stale load");
	const first = api.load(module);
	check((await message(api.load(module))) === "index.ts: the module is loaded already", "a load meanwhile");
	await first;
	const which = process.argv[2];
	switch (which) {
		case undefined:
			return script();
		case "futures":
			return futures();
		case "owned":
			return owned();
		case "closed":
			return closedObjects();
		case "bytes":
			return lent();
		case "bounds":
			return bounds();
		case "grown":
			return grown();
		default:
			return broken(which);
	}
}

await main();
