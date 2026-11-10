// Runs a fixture's harness both ways in one headless Chromium page and fails on any
// difference: natively, as the fixture's wasm32 build through wasm-bindgen's CLI glue for
// the web, and as its TypeScript translation, which Bun bundles for the page. Each side's
// `run()` gives a promise of its report, which the page posts back here.
//
// usage: bun browser.ts <glue.js> <harness.ts> <out-dir>

import { mkdir, rm } from "node:fs/promises";
import { basename, dirname, join, resolve } from "node:path";

const [gluePath, harnessPath, outArg] = process.argv.slice(2);
if (!gluePath || !harnessPath || !outArg) throw new Error("usage: browser.ts <glue.js> <harness.ts> <out-dir>");
const out = resolve(outArg);
await rm(out, { recursive: true, force: true });
await mkdir(out, { recursive: true });

const bundled = await Bun.build({ entrypoints: [harnessPath], target: "browser", format: "esm" });
if (!bundled.success) throw new AggregateError(bundled.logs, "bundling the translation failed");
await Bun.write(join(out, "translated.js"), bundled.outputs[0]!);

const glue = basename(gluePath);
const page = `<!doctype html><script type="module">
const post = (path, body) => fetch(path, { method: "POST", body });
try {
	const native = await import("/glue/${glue}");
	await native.default();
	const want = await native.run();
	const got = await (await import("/translated.js")).run();
	await post("/result", JSON.stringify({ want, got }));
} catch (err) {
	await post("/error", String(err?.stack ?? err));
}
</script>`;
await Bun.write(join(out, "index.html"), page);

// A page that posts nothing for this long has hung: it encodes three tiny frames.
const stallMs = 60_000;
const { promise, resolve: done, reject } = Promise.withResolvers<{ want: string; got: string }>();
const server = Bun.serve({
	hostname: "127.0.0.1",
	port: 0,
	async fetch(req) {
		const path = new URL(req.url).pathname;
		if (req.method === "POST") {
			const body = await req.text();
			if (path === "/result") done(JSON.parse(body));
			else reject(new Error(`the page threw: ${body}`));
			return new Response();
		}
		const file = Bun.file(
			path.startsWith("/glue/") ? join(dirname(gluePath), path.slice("/glue/".length)) : join(out, path === "/" ? "index.html" : path.slice(1)),
		);
		if (!(await file.exists())) return new Response(null, { status: 404 });
		return new Response(file);
	},
});

// No sandbox: the page is our own, and CI runners refuse the user namespaces it needs.
const profile = join(out, "profile");
const log = join(out, "chromium.log");
const proc = Bun.spawn(["chromium", "--headless", "--no-sandbox", "--no-first-run", `--user-data-dir=${profile}`, server.url.href], {
	stdout: "ignore",
	stderr: Bun.file(log),
});
proc.exited.then(async (code) => reject(new Error(`chromium exited with ${code}:\n${await Bun.file(log).text()}`)));
const stall = setTimeout(() => reject(new Error(`chromium posted nothing for ${stallMs / 1000} s`)), stallMs);

let failed = false;
try {
	const { want, got } = await promise;
	if (want === got) console.log(want);
	else {
		failed = true;
		console.error(`native\n${want}\nts\n${got}`);
	}
} finally {
	clearTimeout(stall);
	proc.kill();
	await server.stop(true);
}
if (failed) process.exit(1);
