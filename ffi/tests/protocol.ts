// Runs scripts tests/protocol/model.rs writes through the api fixture's web binding, from
// beside its `web` directory: each script named on the command line, writing what each step
// returned a line to the script's path with `.out` added, which tests/fixtures.rs compares
// with the model's transcript. One process runs them all, as loading the module costs more
// than a script. RUST2_WASM is the path of the module, built with the crate's `web` feature.
// tests/fixtures.rs runs it through Node's binding too, importing `./node/` in place of
// `./web/` and loading the addon beside it.
//
// A dispose is the script's drop, and a future's `cancel()` its free; the garbage collector
// frees in its own time, so no step frees an object. Every step runs before any microtask
// but an await's, so a future progresses only as the script awaits.

import * as api from "./web/index.ts";
import * as rt from "./web/rt.ts";
import * as Counter from "./web/Counter.ts";
import * as Draft from "./web/Draft.ts";
import * as Ledger from "./web/Ledger.ts";
import * as Local from "./web/Local.ts";
import * as Meter from "./web/Meter.ts";
import * as Meters from "./web/Meters.ts";
import * as Receipt from "./web/Receipt.ts";
import * as Tally from "./web/Tally.ts";

// Node's globals, which the binding's types leave out.
declare const process: {
	readonly argv: string[];
	readonly env: Record<string, string | undefined>;
	getBuiltinModule(id: "node:fs"): {
		readFileSync(path: string): Uint8Array<ArrayBuffer>;
		readFileSync(path: string, encoding: "utf8"): string;
		writeFileSync(path: string, text: string): void;
	};
};

/** A failed check's message, after the function's name, by the status the model prints. */
const STATUSES: [string, string][] = [
	["a handle's value was moved out", "moved"],
	["a handle is borrowed exclusively", "exclusive"],
	["a handle is already borrowed", "borrowed"],
	["a handle is borrowed on another thread", "thread"],
	["a handle a call lent is borrowed exclusively or moved out", "lent"],
	["a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed", "outlived"],
	["a value that is not Send is used on another thread than the one that made it", "not_send"],
];

/**
 * The status a failed call's error names: a closed handle's is the binding's own. Any other
 * error is the step's outcome as it is, so the transcript parts from the model there.
 */
function status(e: unknown): string {
	const message = e instanceof Error ? e.message : String(e);
	if (/^\w+ is closed$/.test(message)) return "moved";
	for (const [text, name] of STATUSES) {
		if (message.endsWith(`: ${text}`)) return name;
	}
	return `threw ${message}`;
}

const show = (v: rt.u64 | number | boolean) =>
	typeof v === "boolean" ? (v ? "1" : "0") : typeof v === "number" ? String(v) : rt.u64ToBig(v).toString();

type Value = rt.Handle | rt.Call<rt.u64 | number>;

async function run(text: string): Promise<string> {
	const lines = text.split("\n").filter((line) => line !== "");
	const kinds = lines[0]!.split(" ");
	const slots: (Value | undefined)[] = [];
	const at = <T extends Value>(i: string): T => slots[Number(i)] as T;
	const opt = (i: string) => (i === "-" ? undefined : at<Counter.Counter>(i));
	let out = "";
	for (const line of lines.slice(1)) {
		const [thread, op, a = "", b = "", c = ""] = line.split(" ");
		if (thread !== "0") throw new Error(`no second thread on wasm32: ${line}`);
		let result: string;
		try {
			// Only an await yields to the microtask queue, which polls the pending futures.
			const stepped = step(op!, a, b, c);
			result = typeof stepped === "string" ? stepped : await stepped;
		} catch (e) {
			result = status(e);
		}
		out += `${result}\n`;
	}
	return out;

	function step(op: string, a: string, b: string, c: string): string | Promise<string> {
		const made = (v: Value) => {
			slots[Number(a)] = v;
			return "ok";
		};
		switch (op) {
			case "counter_new":
				return made(Counter.new$("c", Number(b)));
			case "counter_get":
				return show(Counter.get(at(a)));
			case "counter_add": {
				const r = Counter.add(at(a), Number(b));
				return r.ok ? show(r.value) : "error";
			}
			case "counter_into_sum":
				return show(Counter.intoSum(at(a), Number(b)));
			case "counter_plus":
				return show(Counter.plus(at(a), opt(b)));
			case "counter_add_to":
				return show(Counter.addTo(at(a), opt(b)));
			case "counter_clone":
				return made(Counter.clone(at(b)));
			case "plus_later":
				return made(Counter.plusLater(at(b), opt(c)));
			case "ledger_new":
				return made(Ledger.new$());
			case "ledger_edit": {
				const r = Ledger.edit(at(b));
				if (!r.ok) return "error";
				made(r.value);
				return "1";
			}
			case "ledger_published":
				return show(Ledger.published(at(a)));
			case "ledger_seal":
				Ledger.seal(at(a));
				return "ok";
			case "ledger_receipt":
				return made(Ledger.receipt(at(b)));
			case "draft_write":
				Draft.write(at(a), "line");
				return "ok";
			case "draft_lines":
				return show(Draft.lines(at(a)));
			case "draft_commit":
				return show(Draft.commit(at(a)));
			case "receipt_count":
				return show(Receipt.count(at(a)));
			case "receipt_clone":
				return made(Receipt.clone(at(b)));
			case "tallied":
				return show(api.tallied(at(a)));
			case "stored":
				return show(api.stored(at(a)));
			case "tally_new":
				return made(Tally.new$());
			case "tally_bump":
				return show(Tally.bump(at(a)));
			case "bump_later":
				return made(Tally.bumpLater(at(b)));
			case "local_new":
				// The binding makes it where its profile says: on wasm32, which has no homes,
				// here, and through Node's addon, on a home.
				return made(Local.new$());
			case "local_bump":
				return show(Local.bump(at(a)));
			case "local_share":
				return made(Local.share(at(b)));
			case "local_later":
				return made(Local.later(at(b)));
			case "meters_new":
				return made(Meters.new$());
			case "meters_first":
				return made(Meters.first(at(b)));
			case "meters_clear":
				Meters.clear(at(a));
				return "ok";
			case "meter_new":
				return made(Meter.default$());
			case "meter_add":
				return show(Meter.add(at(a), Number(b)));
			case "meter_reset":
				Meter.reset(at(a));
				return "ok";
			case "drop":
				at<rt.Handle>(a)[Symbol.dispose]();
				return "ok";
			case "free": {
				if (!kinds[Number(a)]!.includes("_")) throw new Error(`only a future frees: ${a}`);
				at<rt.Call<number>>(a).cancel();
				return "ok";
			}
			case "await":
				return at<rt.Call<rt.u64 | number>>(a).then(show);
			case "drain":
				return "ok";
			default:
				throw new Error(`no op ${op}`);
		}
	}
}

const fs = process.getBuiltinModule("node:fs");
await api.load(await WebAssembly.compile(fs.readFileSync(process.env["RUST2_WASM"]!)));
for (const path of process.argv.slice(2)) {
	fs.writeFileSync(`${path}.out`, await run(fs.readFileSync(path, "utf8")));
}
