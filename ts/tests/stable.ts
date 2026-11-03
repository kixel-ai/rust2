// Each case's generated inputs are its own: adding, removing, or reordering a case
// changes no other case's, and a task step's scripts mix in only the drivers.
//
// usage: node stable.ts

import { strict as assert } from "node:assert";

import { byCase } from "./inputs.ts";
import * as task from "./task.ts";

const COUNT = 50;

/** Cases picked by their place in `names`, as a harness's `op` picks them. */
function placed(names: string[]): Map<string, Uint8Array> {
	return new Map(names.map((name, i) => [name, Uint8Array.of(i >> 8, i & 0xff)]));
}

const hex = (inputs: Uint8Array[]) => inputs.map((input) => Buffer.from(input).toString("hex"));

const names = ["slices", "made", "buf::reads", "buf::writes", "maps::map"];
const base = byCase({ ops: placed(names), wide: true }, COUNT);

/** What follows each input's two-byte op. */
const tails = (inputs: Uint8Array[]) => hex(inputs.map((input) => input.subarray(2)));

// Added at the end, a case moves no other's op either, so every other input stays byte for byte.
const added = byCase({ ops: placed([...names, "buf::write_all"]), wide: true }, COUNT);
for (const name of names) assert.deepEqual(hex(added.get(name)!), hex(base.get(name)!), `adding a case changed ${name}'s inputs`);
for (const name of names) {
	assert.notDeepEqual(tails(added.get("buf::write_all")!.slice(-COUNT)), tails(base.get(name)!.slice(-COUNT)), `a new case drew ${name}'s random inputs`);
}

// Added first, removed, or reordered, a case moves the others' ops, but nothing after them.
for (const moved of [["buf::write_all", ...names], names.filter((name) => name !== "made"), [...names].reverse()]) {
	const inputs = byCase({ ops: placed(moved), wide: true }, COUNT);
	for (const name of moved.filter((name) => names.includes(name))) {
		assert.deepEqual(tails(inputs.get(name)!), tails(base.get(name)!), `ordering cases as ${moved} changed ${name}'s inputs`);
	}
}

// A step's scripts run only it and the drivers, so no other step's existence or place reaches them.
const drivers = task.drivers.map((name) => task.ops.get(name)![0]);
for (const [name, scripts] of byCase(task, COUNT)) {
	const own = task.ops.get(name)![0];
	for (const script of scripts) {
		for (let i = 0; i < script.length; i += 2) {
			assert(script[i] === own || drivers.includes(script[i]), `${name}'s script ${Buffer.from(script).toString("hex")} runs another step`);
		}
	}
}
