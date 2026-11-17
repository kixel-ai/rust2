// One engine's run of the benchmark, bundled by `main.ts` beside the files it loads: in
// Node and Bun it prints the result, and in a browser it posts it back to `main.ts`. Each
// route runs in a worker of its own, which loads this bundle again to serve `measure`.

import { Wasm } from "../tests/wasm.ts";
import { type Link, type Load, measure, type Reply, type Subject, serve } from "./time.ts";
import type { Workload } from "./workloads.ts";

type Workloads = { name: string; op: Workload["op"]; items: number; input: string }[];

export interface Config {
	sampleMs: number;
	rounds: number;
	seed: number;
	workloads: Workloads;
	/** The async handoff's, on the task fixture. */
	handoffs: Workloads;
	/** The name of `--route`'s translations, bundled as `route.js` and `task-route.js`. */
	route?: string;
}

// A page has a `window` and its workers a `WorkerGlobalScope`; Node and Bun have neither.
const browser = typeof window !== "undefined" || "WorkerGlobalScope" in globalThis;
const threads = browser ? undefined : await import("node:worker_threads");
const worker = browser ? typeof window === "undefined" : !threads?.isMainThread;

// Tells a browser's `main.ts`, which fails the run when it stops hearing, what runs next. A
// beacon leaves no response for the page to handle between samples.
const progress = browser ? (step: string) => void navigator.sendBeacon("/progress", step) : () => {};

async function load(name: string): Promise<Uint8Array> {
	const url = new URL(name, import.meta.url);
	if (url.protocol === "file:") {
		const fs = await import("node:fs/promises");
		return new Uint8Array(await fs.readFile(url));
	}
	return new Uint8Array(await (await fetch(url)).arrayBuffer());
}

/** A worker running this bundle, which serves `measure`. */
function spawn(): Link {
	const arrived: Reply[] = [];
	const waiting: PromiseWithResolvers<unknown>[] = [];
	let failed: Error | undefined;
	const settle = () => {
		while (waiting.length > 0 && (failed || arrived.length > 0)) {
			const next = waiting.shift() as PromiseWithResolvers<unknown>;
			const reply = arrived.shift();
			if (!reply) next.reject(failed);
			else if ("error" in reply) next.reject(new Error(reply.error));
			else next.resolve(reply.value);
		}
	};
	const receive = (reply: Reply) => {
		arrived.push(reply);
		settle();
	};
	const fail = (err: Error) => {
		failed = err;
		settle();
	};
	if (threads) {
		const w = new threads.Worker(new URL(import.meta.url));
		w.on("message", receive);
		w.on("error", fail);
		return { post: (load) => w.postMessage(load), reply: () => next(), close: () => void w.terminate() };
	}
	const w = new Worker(import.meta.url, { type: "module" });
	w.onmessage = (e: MessageEvent<Reply>) => receive(e.data);
	w.onerror = (e) => fail(new Error(`a worker failed: ${e.message}`));
	return { post: (load) => w.postMessage(load), reply: () => next(), close: () => w.terminate() };

	function next(): Promise<unknown> {
		const next = Promise.withResolvers<unknown>();
		waiting.push(next);
		settle();
		return next.promise;
	}
}

/** A worker's side: it loads one route, then times it as `measure` asks. */
function work() {
	// Without isolation, a browser coarsens performance.now() to as much as a millisecond.
	if (browser && !crossOriginIsolated) throw new Error("the worker is not cross-origin isolated");
	const reply = (reply: Reply) => (threads ? threads.parentPort?.postMessage(reply) : postMessage(reply));
	const start = (request: Load) =>
		serve(
			request,
			reply,
			(file) => import(new URL(file, import.meta.url).href),
			load,
		);
	if (threads) threads.parentPort?.once("message", start);
	else onmessage = (e: MessageEvent<Load>) => start(e.data);
}

function engine(): string {
	if (browser) return navigator.userAgent;
	const bun = (globalThis as { Bun?: { version: string } }).Bun;
	if (bun) return `Bun ${bun.version} (JavaScriptCore ${process.versions.webkit})`;
	return `Node ${process.version} (V8 ${process.versions.v8})`;
}

function parse(workloads: Workloads): Workload[] {
	return workloads.map((w) => ({ ...w, input: Uint8Array.from(w.input.match(/../g) ?? [], (b) => parseInt(b, 16)) }));
}

async function main() {
	// Without isolation, a browser coarsens performance.now() to as much as a millisecond.
	if (browser && !crossOriginIsolated) throw new Error("the page is not cross-origin isolated");
	progress("loading workloads.json");
	const config: Config = JSON.parse(new TextDecoder().decode(await load("workloads.json")));
	const options = { ...config, progress };
	// Each route is a file of its own, so no cache keyed by URL serves one route's code to
	// another, and runs in a worker of its own. The A/A route is the translation again: its
	// spread from the first is the measurement's noise.
	const route = (route: string, file: string, exported = "run", slice = false): Subject => ({ route, file, exported, slice });
	// The wasm32 builds: the reference, the build with the translation's checks, and that
	// build lowered to JavaScript.
	const builds = (prefix: string): Subject[] => [
		{ route: "wasm", wasm: `${prefix}.wasm`, lowered: false, copies: true },
		{ route: "wasm checked", wasm: `${prefix}-checked.wasm`, lowered: false, copies: false },
		{ route: "wasm2js", wasm: `${prefix}-wasm2js.js`, lowered: true, copies: false },
	];
	const wasm = new Wasm(await WebAssembly.compile(await load("bench.wasm")));
	const routes = [
		...builds("bench"),
		route("rust2-wasm", "rust2-wasm.js"),
		route("hand-written", "moq.js"),
		route("translated", "translated.js", "run", true),
		route("A/A", "translated-aa.js", "run", true),
	];
	if (config.route) routes.push(route(config.route, "route.js", "run", true));
	const codec = await measure(wasm, routes, parse(config.workloads), spawn, options);

	// The async handoff: the translation polled at once by a one-task executor, as the
	// native and WASM routes are, and on the runtime's microtask executor.
	const handoffRoutes = [
		...builds("task"),
		route("rust2-wasm", "task-rust2-wasm.js"),
		route("hand-written", "task-hand.js", "handWritten"),
		route("translated", "task-translated.js"),
		route("spawned", "task-spawned.js", "spawn"),
		route("A/A", "task-translated-aa.js"),
	];
	if (config.route) handoffRoutes.push(route(config.route, "task-route.js"));
	const taskWasm = new Wasm(await WebAssembly.compile(await load("task.wasm")));
	const handoff = await measure(taskWasm, handoffRoutes, parse(config.handoffs), spawn, options);
	return { engine: engine(), timings: [...codec.timings, ...handoff.timings], cold: { codec: codec.cold, handoff: handoff.cold } };
}

if (worker) {
	work();
} else if (browser) {
	const post = (path: string, body: string) => fetch(path, { method: "POST", body });
	main().then(
		(result) => post("/result", JSON.stringify(result)),
		(err: unknown) => post("/error", err instanceof Error ? (err.stack ?? err.message) : String(err)),
	);
} else {
	console.log(JSON.stringify(await main()));
}
