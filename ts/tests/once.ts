// A `OnceLock` initialized from within its own initialization, or a `LazyLock` or
// `LazyCell` forced from within its function, panics with std's message, which native
// Rust can't show: wasm32's `Once` panics, which traps there, and the host's
// deadlocks.
//
// usage: node once.ts

import { strict as assert } from "node:assert";

import * as rs from "../runtime/rs.ts";

const recursive = (f: () => void) => assert.throws(f, (e: Error) => e instanceof rs.Panic && e.message === "one-time initialization may not be performed recursively");

const lock = new rs.OnceLock<string>(undefined);
const made = rs.onceGetOrInit(lock, () => {
	recursive(() => rs.onceSet(lock, "set"));
	recursive(() => rs.onceGetOrInit(lock, () => "inner"));
	return "outer";
});
assert.equal(made, "outer");

// Once initialized, neither panics, and `set` gives its value back.
assert.deepEqual(rs.onceSet(lock, "again"), rs.Err("again"));

// A lazy cell's function runs once, so a `force` from within it has no value to give.
const lazyLock: rs.LazyLock<string> = new rs.LazyLock(() => {
	recursive(() => lazyLock.force());
	return "made";
});
assert.equal(lazyLock.force(), "made");
const lazyCell: rs.LazyCell<string> = new rs.LazyCell(() => {
	assert.throws(
		() => lazyCell.force(),
		(e: Error) => e instanceof rs.Panic && e.message === "LazyCell instance has previously been poisoned",
	);
	return "made";
});
assert.equal(lazyCell.force(), "made");
