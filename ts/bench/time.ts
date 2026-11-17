// Times every workload on every route in one engine, after checking that each route's
// output matches the wasm32 build's, which is native Rust's. A wasm32 build's route times
// its run alone, and the reference's also copying the input in and the output out.
//
// Each route runs alone in a realm of its own, a worker, which `measure` drives one
// sample at a time. Routes in one realm share its state: JavaScriptCore merges the type
// profiles of modules with the same text, a module two routes call sees both routes'
// types, and V8 ran a route 2-12% slower loaded after an identical copy than before it.
// Each round's worker is fresh, so its first result is the route's cold start.

import { type Exports, Wasm } from "../tests/wasm.ts";
import { fresh } from "./fresh.ts";
import type { Workload } from "./workloads.ts";

/** What one realm times: a route's bundle, or a wasm32 build through `fixtures/wasm.rs`. */
export type Subject =
	| {
			route: string;
			/** The bundle, beside `entry.js`. */
			file: string;
			/** The export that runs a workload. */
			exported: string;
			/** Whether `exported` returns the translation's `{ data, length }` rather than bytes. */
			slice: boolean;
	  }
	| {
			route: string;
			/** The `.wasm`, or with `lowered`, the bundle of wasm2js's lowering of one. */
			wasm: string;
			lowered: boolean;
			/** Whether copying the input in and the output out are timed apart, as `<route> copy-in` and `copy-out`. */
			copies: boolean;
	  };

/** What a realm is told first: what to load, and where later requests arrive. */
export interface Load {
	subject: Subject;
	workloads: Workload[];
	sampleMs: number;
	/**
	 * Each later request: a count `measure` bumps, the op, and its argument. The realm
	 * blocks on it in `Atomics.wait` rather than going back to its event loop, where the
	 * engine's idle-time work spread Firefox's samples of one body up to 2.5x apart.
	 */
	control: Int32Array;
}

/** A realm's replies: the bodies it loaded, then one per request. */
export type Reply = { value: unknown } | { error: string };

/** A worker, as `measure` drives it. */
export interface Link {
	post(load: Load): void;
	/** The next reply, rejected if the worker fails. */
	reply(): Promise<unknown>;
	close(): void;
}

const enum Op {
	/** The route's output on a workload, in hex. */
	Output,
	/** Warm a body up, answering with its calls per sample. */
	Warm,
	/** One sample of a body, in milliseconds. */
	Sample,
}

export interface Options {
	/** How long one sample runs at least, in milliseconds. */
	sampleMs: number;
	/** Rounds, each one sample of every body from fresh workers. */
	rounds: number;
	/** Seeds the order of the bodies in each round. */
	seed: number;
	/** Told what runs next, between timed regions. */
	progress(step: string): void;
}

export interface Timing {
	workload: string;
	op: string;
	route: string;
	/** Calls per sample, as the first round's warm-up found. */
	calls: number;
	/** Nanoseconds per item, one per round. */
	ns: number[];
}

/** A route's cold starts: milliseconds from a fresh worker's load of it to its first output, one per round. */
export interface Cold {
	route: string;
	ms: number[];
}

/** What `measure` found. */
export interface Measured {
	timings: Timing[];
	cold: Cold[];
}

interface Body {
	workload: Workload;
	route: string;
	/** Run before each sample, untimed. */
	setup?: () => void;
	call: () => unknown;
	async: boolean;
	calls: number;
}

// Holds the last result, so no engine can drop a call as dead.
let sink: unknown;

export function hex(bytes: Uint8Array): string {
	return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

/**
 * Times `workloads` on each subject, each round in a fresh worker per subject that `spawn`
 * starts. A worker keeps the inlining and deopts it settles into for as long as it runs,
 * which put identical copies of a route up to 1.6x apart, so a round is one sample of each
 * body from fresh workers, as Kalibera and Jones's "Rigorous Benchmarking in Reasonable
 * Time" advises when runs vary more than iterations within one.
 */
export async function measure(
	wasm: Wasm,
	subjects: Subject[],
	workloads: Workload[],
	spawn: () => Link,
	options: Options,
): Promise<Measured> {
	const { progress, sampleMs } = options;
	const want = workloads.map((workload) => {
		const output = wasm.call(workload.input);
		if (!(output instanceof Uint8Array)) throw new Error(`${workload.name} ${workload.op} traps in native Rust`);
		return hex(output);
	});
	// Each subject's timings, one per body, which every round's worker lists alike.
	const timings: Timing[][] = [];
	const cold: Cold[] = subjects.map(({ route }) => ({ route, ms: [] }));
	const span = `${workloads[0].name} through ${workloads[workloads.length - 1].name}`;
	const random = prng(options.seed);
	for (let round = 0; round < options.rounds; round++) {
		const of = `round ${round + 1} of ${options.rounds} on ${span}`;
		const links: Link[] = [];
		try {
			type Realm = (op: Op, arg: number) => Promise<unknown>;
			const bodies: { realm: Realm; index: number; timing: Timing; items: number; calls: number }[] = [];
			for (const [s, subject] of subjects.entries()) {
				progress(`loading ${"wasm" in subject ? subject.wasm : subject.file} for ${of}`);
				const link = spawn();
				links.push(link);
				const control = new Int32Array(new SharedArrayBuffer(12));
				link.post({ subject, workloads, sampleMs, control });
				const loaded = (await link.reply()) as { bodies: { route: string; workload: number }[]; cold: number };
				const list = loaded.bodies;
				cold[s].ms.push(loaded.cold);
				const realm = (op: Op, arg: number) => {
					control[1] = op;
					control[2] = arg;
					Atomics.add(control, 0, 1);
					Atomics.notify(control, 0);
					return link.reply();
				};
				timings[s] ??= list.map(({ route, workload: w }) => ({ workload: workloads[w].name, op: workloads[w].op, route, calls: 0, ns: [] }));
				list.forEach(({ workload: w }, index) => {
					bodies.push({ realm, index, timing: timings[s][index], items: workloads[w].items, calls: 0 });
				});
				progress(`checking ${subject.route} for ${of}`);
				for (const [i, workload] of workloads.entries()) {
					const got = await realm(Op.Output, i);
					if (got !== want[i]) {
						throw new Error(`${subject.route} differs from native Rust on ${workload.name} ${workload.op}:\nnative ${want[i]}\ngot    ${got}`);
					}
				}
			}

			// Warm-up and sampling each take the bodies in a new random order, so drift spreads
			// over all of them and no body keeps a slot: a fixed order runs identical code slower
			// in one slot than in another. Progress is per step: a report per sample slows
			// Chrome's samples by about 10%.
			progress(`warming up ${of}`);
			shuffle(bodies, random);
			for (const body of bodies) {
				body.calls = (await body.realm(Op.Warm, body.index)) as number;
				body.timing.calls ||= body.calls;
			}
			progress(`timing ${of}`);
			shuffle(bodies, random);
			for (const body of bodies) {
				const ms = (await body.realm(Op.Sample, body.index)) as number;
				body.timing.ns.push((ms * 1e6) / (body.calls * body.items));
			}
		} finally {
			for (const link of links) link.close();
		}
	}
	return { timings: timings.flat(), cold };
}

/**
 * A realm's side of `measure`: loads `load.subject`, importing a bundle with `bundle` and
 * compiling a `.wasm`'s bytes from `wasm`, then answers each request through `reply`
 * until the worker is closed. Its first reply gives the cold start: from the load to the
 * first workload's output.
 */
export async function serve(
	load: Load,
	reply: (reply: Reply) => void,
	bundle: (file: string) => Promise<Record<string, unknown>>,
	wasm: (file: string) => Promise<Uint8Array>,
): Promise<void> {
	const { subject, workloads, sampleMs, control } = load;
	const bodies: Body[] = [];
	const body = (workload: Workload, route: string, call: () => unknown, setup?: () => void) =>
		bodies.push({ workload, route, setup, call, async: false, calls: 1 });
	let output: (workload: number) => Promise<Uint8Array>;
	let cold: number;
	try {
		const start = performance.now();
		if ("wasm" in subject) {
			const instance = subject.lowered
				? new Wasm((await bundle(subject.wasm)) as unknown as Exports)
				: new Wasm(await WebAssembly.compile(fresh(await wasm(subject.wasm))));
			output = async (i) => {
				const out = instance.call(workloads[i].input);
				if (!(out instanceof Uint8Array)) throw new Error(`${subject.route} traps: ${JSON.stringify(out)}`);
				return out;
			};
			for (const workload of workloads) {
				const { input } = workload;
				let len = 0;
				body(workload, subject.route, () => instance.run(), () => instance.input(input));
				if (!subject.copies) continue;
				body(workload, `${subject.route} copy-in`, () => instance.input(input));
				body(workload, `${subject.route} copy-out`, () => instance.output(len), () => {
					instance.input(input);
					len = instance.run();
				});
			}
		} else {
			const run = (await bundle(subject.file))[subject.exported] as (input: Uint8Array) => unknown;
			const view = subject.slice
				? (out: { data: Uint8Array; length: number }) => out.data.subarray(0, out.length)
				: (out: Uint8Array) => out;
			output = async (i) => view((await run(workloads[i].input)) as never);
			for (const workload of workloads) body(workload, subject.route, () => run(workload.input));
		}
		await output(0);
		cold = performance.now() - start;
	} catch (err) {
		return reply(failure(err));
	}
	reply({ value: { bodies: bodies.map((b) => ({ route: b.route, workload: workloads.indexOf(b.workload) })), cold } });

	async function answer(op: Op, arg: number): Promise<unknown> {
		switch (op) {
			case Op.Output:
				return hex(await output(arg));
			case Op.Warm: {
				const body = bodies[arg];
				body.setup?.();
				const first = body.call();
				body.async = first instanceof Promise;
				await first;
				while ((await sample(body)) < sampleMs) body.calls *= 2;
				return body.calls;
			}
			case Op.Sample:
				return sample(bodies[arg]);
		}
	}

	// Each wait blocks the worker in this loop, and each `await` goes back only to the
	// microtask queue, which is all an async body needs: the event loop never runs again.
	for (let seen = 0; ; ) {
		Atomics.wait(control, 0, seen);
		seen = Atomics.load(control, 0);
		try {
			reply({ value: await answer(control[1], control[2]) });
		} catch (err) {
			reply(failure(err));
		}
	}
}

function failure(err: unknown): Reply {
	return { error: err instanceof Error ? (err.stack ?? err.message) : String(err) };
}

/** Milliseconds for one sample of `body.calls` calls. */
async function sample(body: Body): Promise<number> {
	body.setup?.();
	const { call, calls } = body;
	const start = performance.now();
	if (body.async) {
		for (let i = 0; i < calls; i++) sink = await call();
	} else {
		for (let i = 0; i < calls; i++) sink = call();
	}
	return performance.now() - start;
}

/** Shuffles `items` in place: Fisher-Yates. */
function shuffle<T>(items: T[], random: () => number) {
	for (let i = items.length - 1; i > 0; i--) {
		const j = Math.floor(random() * (i + 1));
		[items[i], items[j]] = [items[j], items[i]];
	}
}

/** Uniform numbers in [0, 1) from a 32-bit seed: mulberry32. */
function prng(seed: number): () => number {
	return () => {
		seed = (seed + 0x6d2b79f5) | 0;
		let t = Math.imul(seed ^ (seed >>> 15), seed | 1);
		t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
		return ((t ^ (t >>> 14)) >>> 0) / 2 ** 32;
	};
}
