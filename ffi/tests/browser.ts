// Runs tests/api.ts's cases in one headless browser, Chromium, Firefox, or WebKit, as Node
// runs them: the script and its binding are bundled for the page, and each case runs in a
// frame of its own, a fresh realm, against a `process` that serves the module's bytes, the
// case, and the garbage collector where the browser exposes it. Each case posts whether it
// finished and what it printed, which this prints as a line of JSON.
//
// How `using` in the script reaches the page is the bundle's: `bun` is Bun's bundler, which
// lowers it; `native` leaves it to the engine; `esbuild` and `tsc` lower it as a consumer's
// build would, to the helper that falls back to `Symbol.for("Symbol.dispose")` and to the one
// that throws without `Symbol.dispose`, which stable Safari lacks.
//
// usage: bun browser.ts <chromium|firefox|webkit> <bun|native|esbuild|tsc> <script-dir> <module.wasm> <out-dir> <case>...
// where a case is `-` for the transcript.

import { mkdir, rm } from "node:fs/promises";
import { join, resolve } from "node:path";

const [browser, lowering, dirArg, wasmPath, outArg, ...cases] = process.argv.slice(2);
if (!browser || !lowering || !dirArg || !wasmPath || !outArg || cases.length === 0) {
	throw new Error("usage: browser.ts <chromium|firefox|webkit> <bun|native|esbuild|tsc> <script-dir> <module.wasm> <out-dir> <case>...");
}
const out = resolve(outArg);
await rm(out, { recursive: true, force: true });
await mkdir(out, { recursive: true });

/** Runs a build tool, failing with its output. */
async function build(...command: string[]): Promise<void> {
	const tool = Bun.spawn(command, { stdout: "pipe", stderr: "pipe" });
	const [stdout, stderr, code] = await Promise.all([tool.stdout.text(), tool.stderr.text(), tool.exited]);
	if (code !== 0) throw new Error(`${command.join(" ")} exited with ${code}:\n${stdout}${stderr}`);
}

const entry = join(dirArg, "api.ts");
const bundle = join(out, "api.js");
switch (lowering) {
	case "bun": {
		const bundled = await Bun.build({ entrypoints: [entry], target: "browser", format: "esm" });
		if (!bundled.success) throw new AggregateError(bundled.logs, "bundling api.ts failed");
		await Bun.write(bundle, bundled.outputs[0]!);
		break;
	}
	case "native":
		await build("esbuild", entry, "--bundle", "--format=esm", "--target=esnext", `--outfile=${bundle}`);
		break;
	case "esbuild":
		await build("esbuild", entry, "--bundle", "--format=esm", "--target=es2022", `--outfile=${bundle}`);
		break;
	case "tsc": {
		// The script's own `tsc` emit, lowered to ES2022 as a project's build would, then bundled.
		const emitted = join(out, "tsc");
		await build(
			"tsc",
			"-p",
			dirArg,
			"--noEmit",
			"false",
			"--outDir",
			emitted,
			"--target",
			"es2022",
			"--rewriteRelativeImportExtensions",
			"--allowImportingTsExtensions",
			"false",
		);
		await build("esbuild", join(emitted, "api.js"), "--bundle", "--format=esm", "--target=esnext", `--outfile=${bundle}`);
		break;
	}
	default:
		throw new Error(`no lowering ${lowering}`);
}

// The frame of one case: a `process` as api.ts reads Node's, then the script. What it
// printed, and whether it finished, go back once it settles; an exit or a throw ends it.
const frame = `<!doctype html><script type="module">
const which = new URL(location.href).searchParams.get("case");
const stdout = [];
const stderr = [];
// An error's message and stack, which V8's stack begins with and SpiderMonkey's lacks.
const shown = (a) => (!(a instanceof Error) || a.stack === undefined ? String(a) : a.stack.startsWith(String(a)) ? a.stack : \`\${a}\\n\${a.stack}\`);
const text = (args) => args.map(shown).join(" ");
console.log = (...args) => stdout.push(text(args));
console.error = (...args) => stderr.push(text(args));
class Exit extends Error {}
let code = 0;
addEventListener("error", (e) => {
	if (!(e.error instanceof Exit)) stderr.push(text([e.error ?? e.message])), (code ||= 1);
});
addEventListener("unhandledrejection", (e) => {
	if (!(e.reason instanceof Exit)) stderr.push(text([e.reason])), (code ||= 1);
});
const bytes = new Uint8Array(await (await fetch("/api.wasm")).arrayBuffer());
globalThis.process = {
	argv: ["browser", "api.js", ...(which === "-" ? [] : [which])],
	env: { RUST2_WASM: "api.wasm" },
	exit(c) {
		code = c;
		throw new Exit(\`exit \${c}\`);
	},
	stderr: { write: (t) => (stderr.push(t.replace(/\\n$/, "")), true) },
	getBuiltinModule(id) {
		if (id === "node:fs") return { readFileSync: () => bytes };
		if (id === "node:v8") return { setFlagsFromString() {} };
		if (id === "node:vm") return { runInNewContext: () => globalThis.gc };
		throw new Error(\`no \${id} in a browser\`);
	},
};
try {
	await import("/api.js");
} catch (e) {
	if (!(e instanceof Exit)) stderr.push(text([e])), (code ||= 1);
}
// A rejection the script left unhandled surfaces by the next task.
await new Promise((r) => setTimeout(r));
await fetch("/result", {
	method: "POST",
	body: JSON.stringify({ case: which, ok: code === 0, stdout: stdout.map((l) => l + "\\n").join(""), stderr: stderr.join("\\n") }),
});
parent.postMessage("done", "*");
</script>`;
await Bun.write(join(out, "case.html"), frame);

// The page: each case's frame in turn, so no two share a realm or overlap.
const page = `<!doctype html><script type="module">
const cases = ${JSON.stringify(cases)};
for (const c of cases) {
	const f = document.createElement("iframe");
	const done = new Promise((r) => addEventListener("message", r, { once: true }));
	f.src = "/case.html?case=" + encodeURIComponent(c);
	document.body.append(f);
	await done;
	f.remove();
}
await fetch("/done", { method: "POST" });
</script><body></body>`;
await Bun.write(join(out, "index.html"), page);

// A case that posts nothing for this long has hung: the slowest grows memory to 64 MiB.
const stallMs = 60_000;
const { promise, resolve: done, reject } = Promise.withResolvers<void>();
let stall = setTimeout(() => {}, 0);
const posted = () => {
	clearTimeout(stall);
	stall = setTimeout(() => reject(new Error(`${browser} posted nothing for ${stallMs / 1000} s`)), stallMs);
};
posted();
const results: string[] = [];
const server = Bun.serve({
	hostname: "127.0.0.1",
	port: 0,
	async fetch(req) {
		const path = new URL(req.url).pathname;
		if (req.method === "POST") {
			posted();
			if (path === "/result") results.push(await req.text());
			else done();
			return new Response();
		}
		const file = Bun.file(path === "/api.wasm" ? wasmPath : join(out, path === "/" ? "index.html" : path.slice(1)));
		if (!(await file.exists())) return new Response(null, { status: 404 });
		return new Response(file);
	},
});

const profile = join(out, "profile");
await mkdir(profile, { recursive: true });
const command = (() => {
	switch (browser) {
		// No sandbox: the page is our own, and CI runners refuse the user namespaces it needs.
		// Its V8 exposes `gc`, as Node's `--expose-gc` does, for the case that checks what the
		// garbage collector frees.
		case "chromium":
			return [
				"chromium",
				"--headless",
				"--no-sandbox",
				"--no-first-run",
				"--js-flags=--expose-gc",
				`--user-data-dir=${profile}`,
				server.url.href,
			];
		case "firefox":
			return ["firefox", "--headless", "--no-remote", "--profile", profile, server.url.href];
		// The `webkit-headless` the bench shell carries: Playwright's WPE build, which exposes
		// no garbage collector, as Firefox does not.
		case "webkit":
			return ["webkit-headless", server.url.href];
		default:
			throw new Error(`no browser ${browser}`);
	}
})();
// A first run's pages and checks stay out of the way of the one the command opens.
await Bun.write(
	join(profile, "user.js"),
	[
		'user_pref("browser.shell.checkDefaultBrowser", false);',
		'user_pref("browser.startup.homepage_override.mstone", "ignore");',
		'user_pref("datareporting.policy.dataSubmissionEnabled", false);',
		'user_pref("toolkit.telemetry.reportingpolicy.firstRun", false);',
		'user_pref("app.update.disabledForTesting", true);',
	].join("\n"),
);
const log = join(out, `${browser}.log`);
const proc = Bun.spawn(command, { stdout: "ignore", stderr: Bun.file(log) });
proc.exited.then(async (code) => reject(new Error(`${browser} exited with ${code}:\n${await Bun.file(log).text()}`)));

try {
	await promise;
	for (const result of results) console.log(result);
} finally {
	clearTimeout(stall);
	proc.kill();
	await server.stop(true);
}
