// Runs a fixture's harness both ways on the same inputs and fails on any difference:
// natively, as the fixture's wasm32 build, and as its TypeScript translation.
//
// usage: node differential.ts <fixture.wasm> <harness.ts> [cases] [glue.js]
//
// A harness, or a hand-written caller wrapping one, exports `run(input: rs.ByteSlice):
// Vec<u8>`, what `inputs.ts` reads to pick its inputs, and may export `within(input,
// native, translated)`, which accepts outputs that differ only where Rust leaves the
// result to the platform, as libm's are. [cases] random inputs run, or that many per case
// of a harness that names its cases. One whose panic hooks write a note exports `note()`,
// and its wasm build `note` and `note_len`, which the two compare after each case.
// The wasm build exports `input`, `run`, and `output` around the crate's entry, which
// `wasm.ts` calls. A panic traps natively and throws `Panic` in
// TypeScript, and the two agree only if their messages do. An abort traps natively
// without a panic message, as wasm32 has nowhere to print std's, and throws `Abort` in
// TypeScript, so the two agree that it aborted but not on why. Either leaves the native
// side a fresh instance, so the translation's hook is reset after one. A module that
// wasm-bindgen's CLI processed runs with [glue.js], the `_bg.js` of its glue, and a JS
// exception thrown through either side agrees only if its text does.

import { readFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";

import { inputs } from "./inputs.ts";
import { type Glue, type Outcome, Wasm } from "./wasm.ts";

const [wasmPath, harnessPath, countArg, gluePath] = process.argv.slice(2);
if (!wasmPath || !harnessPath) throw new Error("usage: differential.ts <fixture.wasm> <harness.ts> [cases] [glue.js]");
const count = Number(countArg ?? "20000");

const harness = await import(pathToFileURL(harnessPath).href);
// The translation's runtime, beside the harness, which makes the input its slice type.
const rs = await import(pathToFileURL(join(dirname(harnessPath), "rs.ts")).href);
// The glue's module, loaded anew for each instance, whose memory it caches views of.
let loads = 0;
const glue = async (): Promise<Glue | undefined> => (gluePath ? import(`${pathToFileURL(gluePath).href}?instance=${loads++}`) : undefined);
const wasm = new Wasm(await WebAssembly.compile(await readFile(wasmPath)), await glue());

function translated(input: Uint8Array): Outcome {
	try {
		const out = harness.run(rs.byteSliceFrom(input));
		return out.data.slice(0, out.length);
	} catch (err) {
		if (gluePath && !(err instanceof Error && (err.name === "Abort" || err.name === "Panic"))) return { thrown: `${err}` };
		if (!(err instanceof Error) || (err.name !== "Abort" && err.name !== "Panic")) throw err;
		rs.takeHook();
		return err.name === "Abort" ? "abort" : { panic: err.message };
	}
}

function show(outcome: Outcome): string {
	if (outcome === "abort") return outcome;
	if ("thrown" in outcome) return `threw ${outcome.thrown}`;
	return outcome instanceof Uint8Array ? Buffer.from(outcome).toString("hex") : `panic ${JSON.stringify(outcome.panic)}`;
}

/** An outcome to read: as text if it is lines of printable ASCII, as a log is. */
function display(outcome: Outcome): string {
	if (outcome instanceof Uint8Array && outcome.every((b) => b === 0x0a || (b >= 0x20 && b < 0x7f))) {
		return `\n${Buffer.from(outcome).toString()}`;
	}
	return show(outcome);
}

const cases = inputs(harness, count);

let failures = 0;
let panics = 0;
let aborts = 0;
for (const input of cases) {
	if (wasm.stale) wasm.renew((await glue()) as Glue);
	const want = wasm.call(input);
	const wantNote = wasm.note;
	let got: Outcome;
	let gotNote = "";
	try {
		got = translated(input);
		gotNote = harness.note?.() ?? "";
	} catch (err) {
		console.error(`input ${show(input)}: the translation threw ${err}`);
		failures++;
		continue;
	}
	if (want === "abort") aborts++;
	else if (!(want instanceof Uint8Array) && "panic" in want) panics++;
	const close = want instanceof Uint8Array && got instanceof Uint8Array && harness.within?.(input, want, got) === true;
	if ((show(want) !== show(got) && !close) || wantNote !== gotNote) {
		failures++;
		if (failures <= 20) {
			const note = (note: string) => (note ? `\nnoted\n${note}` : "");
			console.error(`input  ${show(input)}\nnative ${display(want)}${note(wantNote)}\nts     ${display(got)}${note(gotNote)}\n`);
		}
	}
}

console.log(`${cases.length} cases, ${panics} panics, ${aborts} aborts, ${failures} differences`);
if (failures > 0) process.exit(1);
