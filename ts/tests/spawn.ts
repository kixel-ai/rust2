// `rs.spawn` on futures that misbehave, and cancelled: one that keeps waking itself yields to a timer
// queued before it once a batch spends its budget of 128 polls, so it cannot starve
// timers and I/O, and one that panics as it drops rejects its promise.
//
// usage: node spawn.ts

import * as rs from "../runtime/rs.ts";

let polls = 0;
let beforeTimer: number | undefined;
const selfWaking: rs.Future<number> = {
	poll(cx: rs.Context): rs.Poll<number> {
		if (++polls === 1000) return polls;
		cx.wakeByRef();
		return rs.Pending;
	},
	[Symbol.dispose](): void {},
};
setTimeout(() => {
	beforeTimer = polls;
}, 0);
await rs.spawn(selfWaking);
if (beforeTimer !== 128) throw new Error(`${beforeTimer} polls ran before the timer, not 128`);

const panicsOnDrop: rs.Future<number> = {
	poll: () => 1,
	[Symbol.dispose]: () => rs.panic("drop failed"),
};
const settled = await outcome(rs.spawn(panicsOnDrop));
if (settled !== "panic drop failed") throw new Error(`a panic in drop settled the promise as ${settled}`);

// Cancelling through an `AbortSignal`: the future drops as Rust drops one, at once and
// never polled again, or as the poll the abort comes from returns pending, and the
// promise rejects with the signal's reason; one ready in that poll resolves instead.
const log: string[] = [];
/** The waker of the last poll that returned pending. */
let parked: rs.Waker | undefined;

/** A future that logs each poll and its drop, ready on poll `ready`, and runs `during` in each poll. */
function logged(name: string, ready = Infinity, during?: () => void): rs.Future<string> {
	let polls = 0;
	return {
		poll(cx: rs.Context): rs.Poll<string> {
			log.push(`${name} poll`);
			during?.();
			if (++polls >= ready) return name;
			parked = cx.clone();
			return rs.Pending;
		},
		[Symbol.dispose](): void {
			log.push(`${name} drop`);
		},
	};
}

/** How `promise` settles. */
function outcome(promise: Promise<unknown>): Promise<string> {
	return promise.then(
		(value) => `resolved ${value}`,
		(err: unknown) => (err instanceof rs.Panic ? `panic ${err.message}` : `rejected ${err}`),
	);
}

async function expect(name: string, run: () => Promise<string>, expected: string[]): Promise<void> {
	log.length = 0;
	log.push(await run());
	if (log.join("; ") !== expected.join("; ")) throw new Error(`${name}: ${log.join("; ")}, not ${expected.join("; ")}`);
}

await expect("aborted before", async () => outcome(rs.spawn(logged("a"), AbortSignal.abort("early"))), ["a drop", "rejected early"]);
await expect(
	"aborted before its first poll",
	async () => {
		const abort = new AbortController();
		const done = rs.spawn(logged("a"), abort.signal);
		abort.abort("now");
		log.push("aborted");
		return outcome(done);
	},
	["a drop", "aborted", "rejected now"],
);
await expect(
	"aborted while pending",
	async () => {
		const abort = new AbortController();
		const done = rs.spawn(logged("a"), abort.signal);
		await Promise.resolve();
		abort.abort("later");
		log.push("aborted");
		// A wake after the drop polls nothing.
		parked?.wake();
		await Promise.resolve();
		return outcome(done);
	},
	["a poll", "a drop", "aborted", "rejected later"],
);
await expect(
	"aborted in its own poll",
	async () => {
		const abort = new AbortController();
		return outcome(rs.spawn(logged("a", Infinity, () => abort.abort("inside")), abort.signal));
	},
	["a poll", "a drop", "rejected inside"],
);
await expect(
	"aborted in the poll it is ready in",
	async () => {
		const abort = new AbortController();
		return outcome(rs.spawn(logged("a", 1, () => abort.abort("inside")), abort.signal));
	},
	["a poll", "a drop", "resolved a"],
);
await expect(
	"aborted after it returned",
	async () => {
		const abort = new AbortController();
		const done = await outcome(rs.spawn(logged("a", 1), abort.signal));
		abort.abort("late");
		return done;
	},
	["a poll", "a drop", "resolved a"],
);
await expect(
	"aborted with a drop that panics",
	async () => {
		const abort = new AbortController();
		const future = logged("a");
		const done = rs.spawn({ poll: (cx) => future.poll(cx), [Symbol.dispose]: () => rs.panic("drop failed") }, abort.signal);
		abort.abort("now");
		return outcome(done);
	},
	["panic drop failed"],
);
