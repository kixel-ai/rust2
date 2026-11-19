// `just ts bench`: builds each route from the fixtures' Rust, timing each build, bundles and
// minifies the JS routes, times every workload natively and in Node, Bun, Chrome, and
// Firefox, and reports the numbers with each route's size, build, and cold start.
//
// usage: bun main.ts --rust2-ts <bin> --rust2-ffi <bin> --native <bin> --task-native <bin>
//                    --moq <checkout> --out <dir>
//                    [--sample-ms 2] [--rounds 25] [--seed <u32>] [--route <name>=<dir>] [--filter <prefix>[,<prefix>...]]
//
// The task fixture's async handoff runs beside the codec, against moq's way with promises.
// `--route` times the translations in another run's `--out` as one more route, for A/B runs.

import { mkdir, rm } from "node:fs/promises";
import { cpus, release, totalmem } from "node:os";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import type { BunPlugin } from "bun";

import { Wasm } from "../tests/wasm.ts";
import { type Built, build, type Step } from "./build.ts";
import type { Config } from "./entry.ts";
import { type Cold, hex, type Timing } from "./time.ts";
import { handoffs, type Workload, workloads } from "./workloads.ts";

interface Engine {
	engine: string;
	timings: Timing[];
	cold: { codec: Cold[]; handoff: Cold[] };
}

const { values: args } = parseArgs({
	options: {
		"rust2-ts": { type: "string" },
		"rust2-ffi": { type: "string" },
		native: { type: "string" },
		moq: { type: "string" },
		"task-native": { type: "string" },
		out: { type: "string" },
		"sample-ms": { type: "string", default: "2" },
		rounds: { type: "string", default: "25" },
		seed: { type: "string" },
		route: { type: "string" },
		filter: { type: "string" },
	},
	strict: true,
});
function arg(name: keyof typeof args): string {
	const value = args[name];
	if (value === undefined) throw new Error(`--${name} is required`);
	return resolve(value);
}
const checkout = arg("moq");
const out = arg("out");
const sampleMs = Number(args["sample-ms"]);
const rounds = Number(args.rounds);
// Each run orders its rounds anew unless `--seed` replays one.
const seed = args.seed === undefined ? crypto.getRandomValues(new Uint32Array(1))[0] : Number(args.seed);
if (!Number.isInteger(seed) || seed < 0 || seed >= 2 ** 32) throw new Error(`--seed takes a u32, not ${args.seed}`);

const wasmRoutes = ["wasm", "wasm copy-in", "wasm copy-out", "wasm checked", "wasm2js", "rust2-wasm"];
const codecRoutes = ["native", ...wasmRoutes, "hand-written", "translated", "A/A"];
const handoffRoutes = ["native", ...wasmRoutes.filter((r) => !r.includes("copy")), "hand-written", "translated", "spawned", "A/A"];
const extra = args.route === undefined ? undefined : await extraRoute(args.route);

/** `--route`'s name, and the bundles of the translations in its directory, which must both be there. */
async function extraRoute(arg: string): Promise<{ name: string; codec: Uint8Array<ArrayBuffer>; task: Uint8Array<ArrayBuffer> }> {
	const at = arg.indexOf("=");
	const name = arg.slice(0, at);
	if (at < 1 || at === arg.length - 1) throw new Error(`--route takes name=dir, not ${arg}`);
	if ([...codecRoutes, ...handoffRoutes].includes(name)) throw new Error(`--route ${name} is already a route`);
	const dir = resolve(arg.slice(at + 1));
	const codec = join(dir, "translated");
	const task = join(dir, "task");
	for (const file of [join(codec, "bench.ts"), join(task, "model.ts")]) {
		if (!(await Bun.file(file).exists())) throw new Error(`--route ${name}: ${file} is missing`);
	}
	return { name, codec: await bundle(await codecEntry(codec)), task: await bundle(await taskEntry(task)) };
}

// `moq/` is the checkout, whose own imports resolve through its install.
const moq: BunPlugin = {
	name: "moq",
	setup(build) {
		build.onResolve({ filter: /^moq\// }, ({ path }) => ({ path: join(checkout, path.slice("moq/".length)) }));
	},
};

/**
 * `entry` bundled and minified, as a consumer ships it: each route is timed and sized as
 * this. Its wall and CPU time go to `steps` as the step `step`.
 */
async function bundle(entry: string, plugins: BunPlugin[] = [], steps?: Step[], step = ""): Promise<Uint8Array<ArrayBuffer>> {
	const start = performance.now();
	const cpu = process.cpuUsage();
	const result = await Bun.build({ entrypoints: [entry], target: "browser", external: ["node:*"], minify: true, plugins });
	const used = process.cpuUsage(cpu);
	steps?.push({ step, wall: (performance.now() - start) / 1000, cpu: (used.user + used.system) / 1e6 });
	if (result.outputs.length !== 1) throw new Error(`bundling ${entry} made ${result.outputs.length} files`);
	return new Uint8Array(await result.outputs[0].arrayBuffer());
}

async function read(path: string): Promise<Uint8Array<ArrayBuffer>> {
	return new Uint8Array(await Bun.file(path).arrayBuffer());
}

// The page reports an entry that fails to load or throws before it can report itself.
const page = `<!doctype html>
<script>
	addEventListener("error", (e) => fetch("/error", { method: "POST", body: e.message ?? \`\${e.target.src} failed to load\` }), true);
</script>
<script type="module" src="entry.js"></script>
`;

const handWritten = join(import.meta.dir, "moq.ts");

// The codec fixture's translation, whose `bench::run` takes its input as a byte slice.
async function codecEntry(dir: string): Promise<string> {
	const entry = join(dir, "bench-entry.ts");
	await Bun.write(
		entry,
		`import * as rs from "./rs.ts";
import { run as bench } from "./bench.ts";

export const run = (input: Uint8Array) => bench(rs.byteSliceFrom(input));
`,
	);
	return entry;
}

// The task fixture's translation, as `bench/task.ts` drives it.
async function taskEntry(dir: string): Promise<string> {
	const entry = join(dir, "bench-entry.ts");
	await Bun.write(
		entry,
		`import * as rs from "./rs.ts";
import * as Channel from "./model.Channel.ts";
import { drain } from "./model.ts";
import { polled, spawned } from ${JSON.stringify(join(import.meta.dir, "task.ts"))};

const fixture = { Pending: rs.Pending, spawn: rs.spawn, Channel, drain };
export const run = polled(fixture);
export const spawn = spawned(fixture);
`,
	);
	return entry;
}

// A fixture's export through rust2-ffi, as a consumer calls it: the module, `wasm` beside
// the bundle, loaded once, and `bench::run` through the generated binding, its output
// copied out of the bytes Rust returns, which are then freed.
async function webEntry(built: Built, wasm: string): Promise<string> {
	const entry = join(built.export, "bench-entry.ts");
	await Bun.write(
		entry,
		`import { load, run as call } from "./web/index.ts";
import { fresh } from ${JSON.stringify(join(import.meta.dir, "fresh.ts"))};

const url = new URL(${JSON.stringify(wasm)}, import.meta.url);
const bytes = url.protocol === "file:" ? (await import("node:fs/promises")).readFile(url) : fetch(url).then((r) => r.arrayBuffer());
await load(fresh(new Uint8Array(await bytes)));

export function run(input: Uint8Array): Uint8Array {
	const out = call(input);
	try {
		return out.slice();
	} finally {
		out[Symbol.dispose]();
	}
}
`,
	);
	return entry;
}

// Each fixture's routes, built from its Rust source, each step timed.
const tools = { ts: arg("rust2-ts"), ffi: arg("rust2-ffi"), root: resolve(import.meta.dir, ".."), out: join(out, "build"), scratch: join(out, "scratch") };
await mkdir(tools.out, { recursive: true });
const codec = await build("codec", join(out, "translated"), tools);
const task = await build("task", join(out, "task"), tools);
await rm(tools.scratch, { recursive: true, force: true });

/** A fixture's routes as they ship, each its files, and what made them. */
interface Shipped {
	wasm: Uint8Array<ArrayBuffer>;
	checked: Uint8Array<ArrayBuffer>;
	lowered: Uint8Array<ArrayBuffer>;
	exported: Uint8Array<ArrayBuffer>;
	binding: Uint8Array<ArrayBuffer>;
	hand: Uint8Array<ArrayBuffer>;
	translation: Uint8Array<ArrayBuffer>;
	steps: Step[];
}

async function ship(built: Built, prefix: string, hand: string, translated: string, plugins: BunPlugin[]): Promise<Shipped> {
	const { steps } = built;
	return {
		wasm: await read(built.wasm),
		checked: await read(built.checked),
		lowered: await bundle(built.lowered, [], steps, "wasm2js's bundle"),
		exported: await read(built.exported),
		binding: await bundle(await webEntry(built, `${prefix}rust2-wasm.wasm`), [], steps, "rust2-wasm's bundle"),
		hand: await bundle(hand, plugins, steps, "hand-written's bundle"),
		translation: await bundle(translated, [], steps, "translated's bundle"),
		steps,
	};
}
const codecShip = await ship(codec, "", handWritten, await codecEntry(codec.translated), [moq]);
// The task fixture: its channel, its async functions, and their runtime, as each route ships them.
const taskShip = await ship(task, "task-", join(import.meta.dir, "task.ts"), await taskEntry(task.translated), []);

await Promise.all([
	Bun.write(join(out, "entry.js"), await bundle(join(import.meta.dir, "entry.ts"))),
	...files("bench", "", codecShip),
	...files("task", "task-", taskShip),
	Bun.write(join(out, "moq.js"), codecShip.hand),
	Bun.write(join(out, "translated.js"), codecShip.translation),
	Bun.write(join(out, "translated-aa.js"), codecShip.translation),
	Bun.write(join(out, "task-hand.js"), taskShip.hand),
	Bun.write(join(out, "task-translated.js"), taskShip.translation),
	Bun.write(join(out, "task-spawned.js"), taskShip.translation),
	Bun.write(join(out, "task-translated-aa.js"), taskShip.translation),
	Bun.write(join(out, "package.json"), '{ "type": "module" }\n'),
	Bun.write(join(out, "index.html"), page),
	extra && Bun.write(join(out, "route.js"), extra.codec),
	extra && Bun.write(join(out, "task-route.js"), extra.task),
]);

/** The wasm32 builds' files, as `entry.ts` names them. */
function files(wasm: string, prefix: string, s: Shipped): Promise<number>[] {
	return [
		Bun.write(join(out, `${wasm}.wasm`), s.wasm),
		Bun.write(join(out, `${wasm}-checked.wasm`), s.checked),
		Bun.write(join(out, `${wasm}-wasm2js.js`), s.lowered),
		Bun.write(join(out, `${prefix}rust2-wasm.js`), s.binding),
		Bun.write(join(out, `${prefix}rust2-wasm.wasm`), s.exported),
	];
}

/** Each route's files, and the build steps that make them. */
function shipped(s: Shipped, extraFile?: Uint8Array<ArrayBuffer>) {
	return [
		{ route: "wasm", files: [s.wasm], steps: ["wasm"] },
		{ route: "wasm checked", files: [s.checked], steps: ["wasm checked"] },
		{ route: "wasm2js", files: [s.lowered], steps: ["wasm checked", "wasm2js", "wasm2js's bundle"] },
		{ route: "rust2-wasm", files: [s.exported, s.binding], steps: ["rust2-ffi", "export's build", "rust2-wasm's bundle"] },
		{ route: "hand-written", files: [s.hand], steps: ["hand-written's bundle"] },
		{ route: "translated", files: [s.translation], steps: ["rust2-ts", "translated's bundle"] },
		...(extra && extraFile ? [{ route: extra.name, files: [extraFile], steps: [] }] : []),
	].map(({ route, files, steps }) => {
		const sum = (f: (b: Uint8Array<ArrayBuffer>) => number) => files.reduce((n, b) => n + f(b), 0);
		const cost = steps.map((name) => {
			const step = s.steps.find((t) => t.step === name);
			if (!step) throw new Error(`no build step ${name}`);
			return step;
		});
		return {
			route,
			raw: sum((b) => b.byteLength),
			gzip: sum((b) => Bun.gzipSync(b, { level: 9 }).byteLength),
			wall: cost.reduce((n, t) => n + t.wall, 0),
			cpu: cost.reduce((n, t) => n + t.cpu, 0),
			built: steps.length > 0,
		};
	});
}
const sizes = shipped(codecShip, extra?.codec);
const taskSizes = shipped(taskShip, extra?.task);

// The workloads, made by the wasm32 build, which also gives each its expected output.
const wasm = new Wasm(await WebAssembly.compile(codecShip.wasm));
function expected(input: Uint8Array): Uint8Array {
	const output = wasm.call(input);
	if (!(output instanceof Uint8Array)) throw new Error(`native Rust traps on ${hex(input)}`);
	return output;
}
const prefixes = (args.filter ?? "").split(",");
const list = workloads(expected).filter((w) => prefixes.some((p) => w.name.startsWith(p)));
if (list.length === 0) throw new Error(`--filter ${args.filter} matches no codec workloads`);
const handoffList = handoffs();
const config: Config = {
	sampleMs,
	rounds,
	seed,
	workloads: list.map((w) => ({ name: w.name, op: w.op, items: w.items, input: hex(w.input) })),
	handoffs: handoffList.map((w) => ({ name: w.name, op: w.op, items: w.items, input: hex(w.input) })),
	route: extra?.name,
};
await Bun.write(join(out, "workloads.json"), JSON.stringify(config));

/** The native route of `list`: `bin` times each on the host, which must match `expected`. */
async function nativeRoute(bin: string, list: Workload[], expected: (input: Uint8Array) => Uint8Array): Promise<Timing[]> {
	// Names are one word on the binary's lines.
	const key = (w: Workload) => `${w.name}/${w.op}`.replaceAll(" ", "-");
	const stdin = Buffer.from(list.map((w) => `${key(w)} ${hex(w.input)}\n`).join(""));
	const proc = Bun.spawn([bin, String(sampleMs), String(rounds)], { stdin, stderr: "inherit" });
	const lines = (await new Response(proc.stdout).text()).trim().split("\n");
	if ((await proc.exited) !== 0) throw new Error(`the native benchmark ${bin} failed`);
	return lines.map((line, i): Timing => {
		const [name, calls, ns, output] = line.split(" ");
		const w = list[i];
		if (name !== key(w)) throw new Error(`the native benchmark printed ${name} for ${key(w)}`);
		if (output !== hex(expected(w.input))) throw new Error(`native Rust on the host differs from wasm32 on ${name}`);
		return { workload: w.name, op: w.op, route: "native", calls: Number(calls), ns: ns.split(",").map((n) => Number(n) / w.items) };
	});
}

async function native(): Promise<Timing[]> {
	const taskWasm = new Wasm(await WebAssembly.compile(taskShip.wasm));
	const handoff = (input: Uint8Array) => {
		const output = taskWasm.call(input);
		if (!(output instanceof Uint8Array)) throw new Error(`native Rust traps on ${hex(input)}`);
		return output;
	};
	return [...(await nativeRoute(arg("native"), list, expected)), ...(await nativeRoute(arg("task-native"), handoffList, handoff))];
}

async function script(command: string): Promise<Engine> {
	const proc = Bun.spawn([command, join(out, "entry.js")], { stderr: "inherit" });
	const result = await new Response(proc.stdout).text();
	if ((await proc.exited) !== 0) throw new Error(`${command} failed`);
	return JSON.parse(result);
}

// The page's longest step is a round's warm-up, a few samples of each of a few hundred
// bodies: a few seconds at the default 2 ms, so this long a silence is a hang.
const stallMs = Math.max(30_000, 1000 * sampleMs);

/** A browser `browser()` drives: `argv` opens `url` headless on a fresh `profile`. */
interface Browser {
	name: string;
	command: string;
	argv(url: string, profile: string): string[];
	/** Writes what `profile` needs before the first launch. */
	setup?(profile: string): Promise<void>;
}

// No sandbox: the page is our own, and CI runners refuse the user namespaces it needs.
const chromium: Browser = {
	name: "Chrome",
	command: "chromium",
	argv: (url, profile) => ["--headless", "--no-sandbox", "--no-first-run", `--user-data-dir=${profile}`, url],
};

// `--no-remote` keeps a Firefox the user has open from taking the page. The prefs keep a
// fresh profile from opening a first-run tab beside it.
const firefox: Browser = {
	name: "Firefox",
	command: "firefox",
	argv: (url, profile) => ["--headless", "--no-remote", "--profile", profile, url],
	async setup(profile) {
		const prefs = {
			"browser.aboutwelcome.enabled": false,
			"browser.shell.checkDefaultBrowser": false,
			"browser.startup.homepage_override.mstone": "ignore",
			"datareporting.policy.dataSubmissionPolicyBypassNotification": true,
		};
		const lines = Object.entries(prefs).map(([k, v]) => `user_pref(${JSON.stringify(k)}, ${JSON.stringify(v)});\n`);
		await Bun.write(join(profile, "user.js"), lines.join(""));
	},
};

async function browser({ name, command, argv, setup }: Browser): Promise<Engine> {
	const { promise, resolve, reject } = Promise.withResolvers<Engine>();
	let step = "loading the page";
	let stall: Timer | undefined;
	const progress = () => {
		clearTimeout(stall);
		stall = setTimeout(() => fail(`stalled for ${stallMs / 1000} s while ${step}`), stallMs);
	};
	const server = Bun.serve({
		hostname: "127.0.0.1",
		port: 0,
		async fetch(req) {
			const path = new URL(req.url).pathname;
			if (req.method === "POST") {
				const body = await req.text();
				if (path === "/progress") {
					step = body;
					progress();
				} else if (path === "/result") resolve(JSON.parse(body));
				else reject(new Error(`${name}: ${body}`));
				return new Response();
			}
			const file = Bun.file(join(out, path === "/" ? "index.html" : path.slice(1)));
			if (!(await file.exists())) return new Response(null, { status: 404 });
			// Cross-origin isolated, so performance.now() is precise to 5µs in Chrome and 20µs
			// in Firefox, rather than 100µs and 1 ms. Never stored, so each worker's load of a
			// route is cold: no engine caches code it compiled for an earlier one by the response.
			const headers = {
				"Cross-Origin-Opener-Policy": "same-origin",
				"Cross-Origin-Embedder-Policy": "require-corp",
				"Cache-Control": "no-store",
			};
			return new Response(file, { headers });
		},
	});
	// A fresh profile each run, so no state from the last one carries over.
	const profile = join(out, command);
	await rm(profile, { recursive: true, force: true });
	await mkdir(profile, { recursive: true });
	await setup?.(profile);
	// Its stderr goes to a file, which a child left behind cannot hold open as it can a pipe.
	const log = join(out, `${command}.log`);
	const proc = Bun.spawn([command, ...argv(server.url.href, profile)], { stdout: "ignore", stderr: Bun.file(log) });
	const fail = async (why: string) => {
		proc.kill("SIGKILL");
		reject(new Error(`${name} ${why}:\n${await Bun.file(log).text()}`));
	};
	proc.exited.then((code) => fail(`exited with ${code}`));
	progress();
	try {
		// The user agent reduces the version to its major number.
		return { ...(await promise), engine: text([command, "--version"]) };
	} finally {
		proc.kill();
		// Closing the server first drops any progress still in flight.
		await server.stop(true);
		clearTimeout(stall);
	}
}

function text(cmd: string[]): string {
	const proc = Bun.spawnSync(cmd, { stderr: "inherit" });
	if (!proc.success) throw new Error(`${cmd.join(" ")} failed`);
	return proc.stdout.toString().trim();
}

const host = await native();
const engines: Engine[] = [];
for (const run of [() => script("node"), () => script(process.execPath), () => browser(chromium), () => browser(firefox)]) {
	engines.push(await run());
}

const report = render();
await Bun.write(join(out, "report.md"), report);
await Bun.write(join(out, "report.json"), JSON.stringify({ seed, native: host, engines, sizes, taskSizes }, null, "\t"));
console.log(report);

function render(): string {
	const cpu = cpus();
	// Node names no Arm CPU on Linux; lscpu decodes its part number.
	const model =
		process.platform === "linux" ? (text(["lscpu"]).match(/^Model name:\s*(.+)$/m)?.[1] ?? "unknown") : cpu[0].model;
	const lines = [
		"# rust2-ts benchmark",
		"",
		`- Host: ${model}, ${cpu.length} threads, ${(totalmem() / 2 ** 30).toFixed(0)} GiB, ${process.platform} ${release()} ${process.arch}`,
		`- Rust: ${text(["rustc", "-V"])}`,
		`- ${text(["wasm2js", "--version"])}`,
		`- moq: ${text(["git", "-C", checkout, "rev-parse", "HEAD"])}`,
		...engines.map((e) => `- ${e.engine}`),
		"",
		"Each cell is nanoseconds per item (a varint, a frame header, a message, a byte iterated, a key, a timestamp converted, added, or compared, a hash round, a line formatted, a step, a frame, or a string coded, joined, or keyed): the median",
		`of ${rounds} samples of at least ${sampleMs} ms, ± half their interquartile range. Each round runs`,
		`the JS routes and workloads in a new random order, from seed ${seed}. \`native\` is a release`,
		"build on the host, its workloads in a fixed order. `wasm` is the same",
		"Rust at the size-optimized profile moq's WASM shipped with, run with its input already in memory;",
		"copying the input in and the output out are timed apart. `wasm checked` is it at the",
		"`wasm-checked` profile, with the overflow checks and debug assertions the translation keeps,",
		"and `wasm2js` that build lowered to JavaScript by Binaryen's wasm2js, each run as `wasm` is.",
		"`rust2-wasm` is the fixture's `bench::run` exported by rust2-ffi, built at `wasm-checked`, and",
		"called through its generated TypeScript, which copies the input in and the output out. So",
		"`translated`, `rust2-wasm`, `wasm2js`, and `wasm checked` run the same checks: `rust2-wasm`",
		"against `wasm checked` plus `wasm`'s copies is the binding's cost, `wasm2js` against `wasm checked`",
		"the lowering's, and `wasm checked` against `wasm` the checks'. `hand-written` is moq's",
		"`js/net`, called as its own benchmarks call it, for `iter/` the same chain with ES",
		"iterator helpers (`bench/iter.ts`), for `btree/` a sorted array (`bench/sorted.ts`), for `hash/` a `Map` (`bench/hashed.ts`), and",
		"for `time/` conversions `bigint`, as moq's `js/hang` converts timescales (`bench/rescale.ts`),",
		"for `time/checked-add` and `time/ord` moq's own `Timestamp` (`bench/timestamp.ts`), for `fmt/` a template literal (`bench/fmt.ts`), for `parts/` a plain array (`bench/parts.ts`), for `bytes/` subarrays (`bench/frames.ts`), and for `str/`",
		"moq's Writer and Cursor, a template literal, and a `Map` (`bench/strings.ts`), and for `int/` `bigint` wrapped to 64 bits (`bench/mixed.ts`).",
		"Each route runs alone in a worker of its own, a fresh one each round. `A/A` is the translation again: its",
		"gap from `translated` is the noise. A `reads` decode is the frame headers above as they",
		"arrive that many bytes at a time, so a header cut by a read comes up short and is",
		"decoded again on the next: its gap from the whole decode is the error path's price.",
		...(extra ? [`\`${extra.name}\` is the translation in another run's output, given by \`--route\`.`] : []),
		"",
		"## Size and build",
		"",
		"Bytes of each route's own files for the same workloads, as timed: minified by Bun where JS,",
		"and for `rust2-wasm` its `.wasm`, its binding, and the loader that fetches the module, while",
		"the hand-boundary wasm32 rows are the `.wasm` alone, without the few lines of JS a caller",
		"needs to copy its input in and its output out. Each build step is timed from an emptied target",
		"directory of its own, without a compiler cache, its dependencies included, in wall and CPU",
		"seconds, so a route of two cargo steps pays for its dependencies twice where a consumer's",
		"one pipeline would share them: `wasm2js`'s includes `wasm checked`'s build, `rust2-wasm`'s",
		"rust2-ffi's export and the export's build, `translated`'s rust2-ts's translation, and every",
		"JS route's its bundling by Bun.",
		"",
		"| route | raw | gzip | build wall s | build CPU s |",
		"| --- | ---: | ---: | ---: | ---: |",
		...sizes.map(sizeRow),
	];

	const routes = [...codecRoutes, ...(extra ? [extra.name] : [])];
	for (const engine of engines) {
		const all = [...host, ...engine.timings];
		const median = (w: Workload, route: string) => stats(find(all, w, route).ns).median;
		const ratio = (w: Workload) => median(w, "A/A") / median(w, "translated");
		const aa = list.map((w) => Math.abs(ratio(w) - 1)).sort((a, b) => a - b);
		// A family is a workload's first name segment and its op, such as `varint decode`.
		const families = Map.groupBy(list, (w) => `${w.name.split("/")[0]} ${w.op}`);
		const geomean = (ws: Workload[]) => Math.exp(ws.reduce((sum, w) => sum + Math.log(ratio(w)), 0) / ws.length);
		const family = Math.max(...[...families.values()].map((ws) => Math.abs(geomean(ws) - 1)));
		lines.push(
			"",
			`## ${engine.engine}`,
			"",
			`A/A differs from translated by ${percent(aa[aa.length >> 1])} at the median workload, ${percent(aa[aa.length - 1])} at most, and ${percent(family)} at most over a family's geometric mean.`,
			"",
			`| workload | op | ${routes.join(" | ")} | translated ÷ rust2-wasm | translated ÷ hand-written |`,
			`| --- | --- |${" ---: |".repeat(routes.length + 2)}`,
			...list.map((w) => {
				const cells = routes.map((r) => cell(find(all, w, r).ns));
				const over = (route: string) => (median(w, "translated") / median(w, route)).toFixed(2);
				return `| ${w.name} | ${w.op} | ${cells.join(" | ")} | ${over("rust2-wasm")} | ${over("hand-written")} |`;
			}),
		);
	}
	lines.push(...renderCold(), ...renderHandoffs());
	return `${lines.join("\n")}\n`;
}

function sizeRow(s: ReturnType<typeof shipped>[number]): string {
	const build = s.built ? `${s.wall.toFixed(2)} | ${s.cpu.toFixed(2)}` : " | ";
	return `| ${s.route} | ${s.raw} | ${s.gzip} | ${build} |`;
}

/** Each route's cold start in each engine. */
function renderCold(): string[] {
	const lines = [
		"",
		"## Cold start",
		"",
		"Milliseconds from a fresh worker's load of a route to its first output, the codec's first",
		"workload and the handoff's: fetching or reading its files, compiling a `.wasm` or",
		"evaluating its JavaScript, and the first call, the median of a round's worker each, ± half",
		"their interquartile range. Each `.wasm` is compiled with a custom section of its own, and",
		"a browser is told to store nothing, so no engine reuses code it compiled for another worker.",
	];
	for (const engine of engines) {
		const of = (cold: Cold[], route: string) => {
			const found = cold.find((c) => c.route === route);
			return found ? cell(found.ms) : "";
		};
		const routes = [...new Set([...engine.cold.codec, ...engine.cold.handoff].map((c) => c.route))];
		lines.push(
			"",
			`### ${engine.engine}`,
			"",
			"| route | codec | handoff |",
			"| --- | ---: | ---: |",
			...routes.map((r) => `| ${r} | ${of(engine.cold.codec, r)} | ${of(engine.cold.handoff, r)} |`),
		);
	}
	return lines;
}

/** The async handoff on every route, and each route's size. */
function renderHandoffs(): string[] {
	const routes = [...handoffRoutes, ...(extra ? [extra.name] : [])];
	const lines = [
		"",
		"## Async handoff",
		"",
		"Nanoseconds per handoff: a value sent to a task parked on a kio-shaped channel, which",
		"wakes it, polls it through `depth` async functions to the channel, and parks it again.",
		"`native` and the wasm32 routes run the task fixture's `bench.rs`, which polls the task at",
		"once on each wake, and `translated` its translation polled the same way. `spawned` runs the",
		"translation on the runtime's `spawn`, which polls woken tasks from a microtask.",
		"`hand-written` is the handoff with promises and `async` functions, as moq's TypeScript",
		"writes it, where each handoff waits for the reader to park again. Sizes are of",
		"the channel, the async functions, and their runtime, and builds as above.",
		"`rust2-wasm` can run the handoff faster than `wasm checked`, the same Rust at the same",
		"profile. Neither its target nor its allocator does it, since the hand-boundary build",
		"for `wasm32-wasip1` runs as the `wasm32-unknown-unknown` one, nor this harness, since",
		"both reproduce called directly. The fixture is the cdylib's root in one and a dependency",
		"behind the export's `extern \"C\"` function in the other, and the gap narrows at",
		"`opt-level = 3`, so it reads as their code generation differing, not pinned down further.",
		"",
		"| route | raw | gzip | build wall s | build CPU s |",
		"| --- | ---: | ---: | ---: | ---: |",
		...taskSizes.map(sizeRow),
	];
	for (const engine of engines) {
		const all = [...host, ...engine.timings];
		lines.push(
			"",
			`### ${engine.engine}`,
			"",
			`| depth | ${routes.join(" | ")} | translated ÷ rust2-wasm | translated ÷ hand-written |`,
			`| --- |${" ---: |".repeat(routes.length + 2)}`,
			...handoffList.map((w) => {
				const cells = routes.map((r) => cell(find(all, w, r).ns));
				const median = (route: string) => stats(find(all, w, route).ns).median;
				const over = (route: string) => (median("translated") / median(route)).toFixed(2);
				return `| ${w.name.replace("depth ", "")} | ${cells.join(" | ")} | ${over("rust2-wasm")} | ${over("hand-written")} |`;
			}),
		);
	}
	return lines;
}

function find(timings: Timing[], w: Workload, route: string): Timing {
	const timing = timings.find((t) => t.workload === w.name && t.op === w.op && t.route === route);
	if (!timing) throw new Error(`no ${route} timing for ${w.name} ${w.op}`);
	return timing;
}

function stats(ns: number[]) {
	const sorted = [...ns].sort((a, b) => a - b);
	const at = (q: number) => sorted[Math.round(q * (sorted.length - 1))];
	return { median: at(0.5), spread: (at(0.75) - at(0.25)) / 2 / at(0.5) };
}

function cell(ns: number[]): string {
	const { median, spread } = stats(ns);
	return `${Number(median.toPrecision(3))} ±${percent(spread)}`;
}

function percent(x: number): string {
	return `${(x * 100).toFixed(x < 0.1 ? 1 : 0)}%`;
}
