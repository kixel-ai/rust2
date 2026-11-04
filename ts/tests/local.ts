// A thread-local's first value as wasm32's `LazyStorage` makes it, which differential
// tests can't show: native Rust runs each case after a panic on a fresh instance, whose
// thread-locals have no value yet, while the translation's keep theirs.
//
// usage: node local.ts

import { strict as assert } from "node:assert";

import * as rs from "../runtime/rs.ts";

// `set`, as std's calls it: what the key had no value for becomes its value, else sets it.
const set = (key: rs.LocalKey<rs.Cell<number>>, v: number) =>
	rs.localKeyInitializeWith(key, new rs.Cell(v), (init, cell) => {
		if (init !== undefined) rs.cellSet(cell, init.v);
	});

// A lazy key's `set` before its first use doesn't run its expression; a `const` one's
// value is always there.
const made: string[] = [];
const lazy = new rs.LocalKey(() => {
	made.push("lazy");
	return new rs.Cell(1);
}, true);
set(lazy, 2);
assert.equal(lazy.get().v, 2);
set(lazy, 3);
assert.equal(lazy.get().v, 3);
const eager = new rs.LocalKey(() => {
	made.push("eager");
	return new rs.Cell(1);
}, false);
set(eager, 4);
assert.equal(eager.get().v, 4);
assert.deepEqual(made, ["eager"]);

// A first use from within the expression makes its own value, which the outer one's
// replaces, dropping it.
const dropped: number[] = [];
let count = 0;
const nested: rs.LocalKey<number> = new rs.LocalKey(
	() => {
		const n = ++count;
		if (n === 1) assert.equal(nested.get(), 2);
		return n;
	},
	true,
	(n) => dropped.push(n),
);
assert.equal(nested.get(), 1);
assert.equal(nested.get(), 1);
assert.deepEqual(dropped, [2]);

// A use while that drop runs aborts, which here panics.
let calls = 0;
const dying: rs.LocalKey<number> = new rs.LocalKey(
	() => {
		const n = ++calls;
		if (n === 1) dying.get();
		return n;
	},
	true,
	() =>
		assert.throws(
			() => dying.get(),
			(e: Error) => e instanceof rs.Panic && e.message === "Attempted to initialize thread-local while it is being dropped",
		),
);
assert.equal(dying.get(), 1);
assert.equal(calls, 3);
