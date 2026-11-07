// The runtime's B-tree through a script of random operations, large enough to split, merge,
// and rebalance its nodes at every height, and to empty it again: inserts, removals, pops,
// lookups, ranges and iterators walked from both ends, `retain`, `extract_if` read part way,
// `clone`, `split_off`, and `collect`, and drops whole trees. Prints what each returns, which
// keys each compared, and which values each whole drop dropped in what order, which
// `runtime.rs` compares with std's `BTreeMap` running the same script.
//
// usage: node btree.ts

import * as rs from "../runtime/rs.ts";

// xorshift32, as `runtime.rs` runs it.
let state = 0x2545f491;
function rand(): number {
	state ^= state << 13;
	state >>>= 0;
	state ^= state >>> 17;
	state ^= state << 5;
	state >>>= 0;
	return state;
}

// Each call of the keys' `cmp`, counted and hashed by its keys, which `runtime.rs` logs alike.
let calls = 0;
let hash = 0;
const cmp = (a: number, b: number): rs.Ordering => {
	calls++;
	hash = (Math.imul(hash, 31) + Math.imul(a, 65537) + b) >>> 0;
	return a < b ? -1 : a > b ? 1 : 0;
};
const out: string[] = [];
/** `line`, and the calls of `cmp` since the last. */
const say = (line: string) => {
	out.push(`${line} #${calls} ${hash}`);
	calls = 0;
	hash = 0;
};
/** How `f` drops values through the dropper it is given: how many, and their hash, as `runtime.rs` logs them. */
const logged = (f: (v: (value: number) => void) => void) => {
	let n = 0;
	let h = 0;
	f((value) => {
		n++;
		h = (Math.imul(h, 31) + value) >>> 0;
	});
	return `drop ${n} ${h}`;
};
const show = (v: number | undefined) => (v === undefined ? "-" : String(v));
const walk = (it: rs.Iter<[number, number]>) => {
	const seen: number[] = [];
	for (let i = 0; i < 40; i++) {
		const item = rand() & 1 ? it.next() : it.nextBack();
		if (item === rs.DONE) break;
		seen.push(item[0]);
	}
	return seen.join(",");
};

/**
 * The tree's shape, as std's keeps it: every leaf at height `h`, its keys in order, each
 * node but the root holding 5 to 11, each child linked to its parent, and the count right.
 */
function check(m: rs.BTreeMap<number, number>): void {
	type Node = { k: number[]; e?: Node[]; p?: Node; i: number };
	const tree = m as unknown as { t: Node; h: number; n: number };
	const keys: number[] = [];
	const fail = (why: string) => {
		throw new Error(`B-tree of ${tree.n} at height ${tree.h}: ${why}`);
	};
	const visit = (node: Node, h: number, root: boolean) => {
		if (node.k.length > 11 || (!root && node.k.length < 5)) fail(`a node of ${node.k.length}`);
		if ((h === 0) !== (node.e === undefined)) fail("a leaf off the bottom");
		node.k.forEach((key, i) => {
			if (node.e !== undefined) visit(node.e[i], h - 1, false);
			keys.push(key);
		});
		if (node.e === undefined) return;
		if (node.e.length !== node.k.length + 1) fail("an internal node's keys and children differ in number");
		node.e.forEach((child, i) => {
			if (child.p !== node || child.i !== i) fail(`child ${i} is linked to the wrong parent`);
		});
		visit(node.e[node.k.length], h - 1, false);
	};
	if (tree.t.p !== undefined) fail("the root has a parent");
	visit(tree.t, tree.h, true);
	keys.forEach((key, i) => {
		if (i > 0 && key <= keys[i - 1]) fail(`key ${key} out of order`);
	});
	if (keys.length !== tree.n) fail("the count is off");
	if (tree.n === 0 && tree.h !== 0) fail("an empty tree above its root leaf");
}

let m = rs.btreeNew<number, number>(cmp);
const steps = 120_000;
for (let step = 0; step < steps; step++) {
	const keys = 65_536;
	// Inserts in the first third, both in the second, and removals in the last.
	const phase = Math.floor((3 * step) / steps);
	const op = rand() % 10;
	const k = rand() % keys;
	if (op < 5) {
		if (phase === 0 || (phase === 1 && k % 2 === 0)) say(`insert ${k} ${show(rs.btreeInsert(m, k, step))}`);
		else say(`remove ${k} ${show(rs.btreeRemove(m, k))}`);
	} else if (op === 5) {
		say(`get ${k} ${show(rs.btreeGet(m, k))}`);
	} else if (op === 6) {
		const j = rand() % keys;
		const [lo, hi] = k < j ? [k, j] : [j, k];
		const inclusive = rand() & 1;
		const r = inclusive ? new rs.RangeInclusive(lo, hi) : { start: lo, end: hi };
		say(`range ${lo} ${hi} ${inclusive} ${walk(rs.btreeRange(m, r))}`);
	} else if (op === 7) {
		const last = (rand() & 1) === 1;
		say(`pop ${last} ${rs.btreePop(m, last)?.join(" ") ?? "-"}`);
	} else if (op === 8) {
		say(`remove ${k} ${show(rs.btreeRemove(m, k))}`);
	} else if (step % 97 === 0) {
		const it = rs.btreeIter(m);
		const seen = walk(it);
		say(`iter ${rs.btreeLen(m)} ${it.len()} ${seen}`);
	} else if (step % 1009 === 0) {
		const copy = rs.btreeClone(m);
		rs.btreeRetain(m, (key) => key % 7 !== k % 7);
		say(`retain ${rs.btreeLen(m)} ${rs.btreeFirst(m)?.join(" ") ?? "-"} ${rs.btreeFirst(m, true)?.join(" ") ?? "-"}`);
		const [kept, unused] = rand() & 1 ? [copy, m] : [m, copy];
		m = kept;
		say(`kept ${rs.btreeLen(m)} ${logged((v) => rs.btreeDrop(unused, undefined, v))}`);
	} else if (step % 1301 === 0) {
		const right = rs.btreeSplitOff(m, k);
		say(`split ${k} ${rs.btreeLen(m)} ${rs.btreeLen(right)}`);
		check(right);
		const [kept, unused] = rand() & 1 ? [right, m] : [m, right];
		m = kept;
		say(logged((v) => rs.btreeClear(unused, undefined, v)));
	} else if (step % 2003 === 0) {
		const entries = rs.iterToVec(rs.btreeIter(m)).items;
		const old = m;
		m = rs.btreeCollect(rs.btreeNew<number, number>(cmp), entries, 8, true, (a, b) => a === b);
		say(`collect ${rs.btreeLen(m)} ${logged((v) => rs.btreeDrop(old, undefined, v))}`);
	} else if (step % 211 === 0) {
		const j = rand() % keys;
		const [lo, hi] = k < j ? [k, j] : [j, k];
		const limit = rand() % 400;
		const copy = rs.btreeClone(m);
		const it = rs.btreeExtractIf(m, { start: lo, end: hi }, (key, value) => (key ^ value) % 3 !== 0);
		const taken: number[] = [];
		for (let i = 0; i < limit; i++) {
			const entry = it.next();
			if (entry === rs.DONE) break;
			taken.push(entry[0]);
		}
		say(`extract ${lo} ${hi} ${rs.btreeLen(m)} ${taken.join(",")}`);
		check(m);
		const [kept, unused] = rand() & 1 ? [copy, m] : [m, copy];
		m = kept;
		say(logged((v) => rs.btreeDrop(unused, undefined, v)));
	}
	if (step % 499 === 0) check(m);
}
say(`end ${rs.btreeLen(m)} ${rs.iterToVec(rs.btreeKeys(m)).items.join(",")}`);
// Emptied from both ends, down to its root leaf.
const drained: number[] = [];
while (rs.btreeLen(m) > 0) {
	drained.push((rs.btreePop(m, (rand() & 1) === 1) as [number, number])[0]);
	if (drained.length % 37 === 0) check(m);
}
check(m);
say(`drain ${drained.join(",")}`);
console.log(out.join("\n"));
