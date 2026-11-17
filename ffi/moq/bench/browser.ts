// Runs export.ts in one headless browser, Chromium or Firefox, as Node runs it: Bun bundles
// it and its binding for a page that serves the module's bytes through a `process` as
// export.ts reads Node's, then posts what it printed, which this prints. The page is
// cross-origin isolated, so its clock ticks in microseconds rather than a tenth of a
// millisecond.
//
// usage: bun browser.ts <chromium|firefox> <module.wasm> <out-dir>

import { mkdir, rm } from "node:fs/promises";
import { join, resolve } from "node:path";

const [browser, wasmPath, outArg] = process.argv.slice(2);
if (!browser || !wasmPath || !outArg) throw new Error("usage: browser.ts <chromium|firefox> <module.wasm> <out-dir>");
const out = resolve(outArg);
await rm(out, { recursive: true, force: true });
await mkdir(out, { recursive: true });

const bundled = await Bun.build({ entrypoints: [join(import.meta.dir, "export.ts")], target: "browser", format: "esm" });
if (!bundled.success) throw new AggregateError(bundled.logs, "bundling export.ts failed");
await Bun.write(join(out, "export.js"), bundled.outputs[0]!);

// What the script prints, or why it failed, goes back once it settles.
const page = `<!doctype html><script type="module">
const lines = [];
console.log = (...args) => lines.push(args.join(" "));
let error;
const bytes = new Uint8Array(await (await fetch("/moq.wasm")).arrayBuffer());
globalThis.process = {
	argv: ["browser", "export.js", "moq.wasm"],
	getBuiltinModule(id) {
		if (id === "node:fs") return { readFileSync: () => bytes };
		throw new Error(\`no \${id} in a browser\`);
	},
};
try {
	await import("/export.js");
} catch (e) {
	error = e instanceof Error ? \`\${e}\\n\${e.stack}\` : String(e);
}
await fetch("/result", { method: "POST", body: JSON.stringify({ lines, error }) });
</script>`;
await Bun.write(join(out, "index.html"), page);

// The two rounds take about a minute; one that posts nothing for ten has hung.
const limitMs = 600_000;
const { promise, resolve: done, reject } = Promise.withResolvers<{ lines: string[]; error?: string }>();
const limit = setTimeout(() => reject(new Error(`${browser} posted nothing for ${limitMs / 1000} s`)), limitMs);
const isolated = { "Cross-Origin-Opener-Policy": "same-origin", "Cross-Origin-Embedder-Policy": "require-corp" };
const server = Bun.serve({
	hostname: "127.0.0.1",
	port: 0,
	async fetch(req) {
		const path = new URL(req.url).pathname;
		if (req.method === "POST") {
			done(await req.json());
			return new Response();
		}
		const file = Bun.file(path === "/moq.wasm" ? wasmPath : join(out, path === "/" ? "index.html" : path.slice(1)));
		if (!(await file.exists())) return new Response(null, { status: 404 });
		return new Response(file, { headers: isolated });
	},
});

const profile = join(out, "profile");
await mkdir(profile, { recursive: true });
const command = (() => {
	switch (browser) {
		// No sandbox: the page is our own, and CI runners refuse the user namespaces it needs.
		case "chromium":
			return ["chromium", "--headless", "--no-sandbox", "--no-first-run", `--user-data-dir=${profile}`, server.url.href];
		case "firefox":
			return ["firefox", "--headless", "--no-remote", "--profile", profile, server.url.href];
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
	const result = await promise;
	if (result.error !== undefined) throw new Error(`export.ts failed in ${browser}: ${result.error}`);
	for (const line of result.lines) console.log(line);
} finally {
	clearTimeout(limit);
	proc.kill();
	await server.stop(true);
}
