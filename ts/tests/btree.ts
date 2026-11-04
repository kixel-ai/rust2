// The runtime's B-tree through a script of random operations, large enough to split, merge,
// and rebalance its nodes at every height, and to empty it again: inserts, removals, pops,
// lookups, ranges and iterators walked from both ends, `retain`, `extract_if` read part way,
// and `clone`. Prints what
// each returns, which `runtime.rs` compares with std's `BTreeMap` running the same script.
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

const cmp = (a: number, b: number): rs.Ordering => (a < b ? -1 : a > b ? 1 : 0);
const out: string[] = [];
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
 * The tree's shape, which std's has no counterpart of: every leaf at height `h`, each
 * node's keys in order and within its parent's bounds, the leaves linked in order, the
 * count right, and every node but the root at least half as full as the fullest, so its
 * height is logarithmic.
 */
function check(m: rs.BTreeMap<number, number>): void {
	type Node = { k: number[]; c?: Node[]; prev?: Node; next?: Node };
	const tree = m as unknown as { t: Node; h: number; n: number };
	const leaves: Node[] = [];
	let fewest = Infinity;
	let most = 0;
	const fail = (why: string) => {
		throw new Error(`B-tree of ${tree.n} at height ${tree.h}: ${why}`);
	};
	const visit = (node: Node, h: number, lo: number, hi: number, root: boolean) => {
		const size = h === 0 ? node.k.length : (node.c as Node[]).length;
		if (!root) fewest = Math.min(fewest, size);
		most = Math.max(most, size);
		if (h === 0) {
			node.k.forEach((key, i) => {
				if (key < lo || key >= hi || (i > 0 && key <= node.k[i - 1])) fail(`leaf key ${key} out of order`);
			});
			leaves.push(node);
			return;
		}
		const c = node.c as Node[];
		if (c.length !== node.k.length) fail("a branch's bounds and children differ in number");
		c.forEach((child, i) => {
			const from = i === 0 ? lo : node.k[i];
			const to = i + 1 < c.length ? node.k[i + 1] : hi;
			if (from < lo || to > hi || from >= to) fail(`branch bound ${from} out of order`);
			visit(child, h - 1, from, to, false);
		});
	};
	visit(tree.t, tree.h, -Infinity, Infinity, true);
	leaves.forEach((l, i) => {
		if (l.prev !== leaves[i - 1] || l.next !== leaves[i + 1]) fail(`leaf ${i} is linked out of order`);
	});
	if (leaves.reduce((n, l) => n + l.k.length, 0) !== tree.n) fail("the count is off");
	if (fewest * 2 < most) fail(`a node of ${fewest} beside one of ${most}`);
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
		if (phase === 0 || (phase === 1 && k % 2 === 0)) out.push(`insert ${k} ${show(rs.btreeInsert(m, k, step))}`);
		else out.push(`remove ${k} ${show(rs.btreeRemove(m, k))}`);
	} else if (op === 5) {
		out.push(`get ${k} ${show(rs.btreeGet(m, k))}`);
	} else if (op === 6) {
		const j = rand() % keys;
		const [lo, hi] = k < j ? [k, j] : [j, k];
		const inclusive = rand() & 1;
		const r = inclusive ? new rs.RangeInclusive(lo, hi) : { start: lo, end: hi };
		out.push(`range ${lo} ${hi} ${inclusive} ${walk(rs.btreeRange(m, r))}`);
	} else if (op === 7) {
		const last = (rand() & 1) === 1;
		out.push(`pop ${last} ${rs.btreePop(m, last)?.join(" ") ?? "-"}`);
	} else if (op === 8) {
		out.push(`remove ${k} ${show(rs.btreeRemove(m, k))}`);
	} else if (step % 97 === 0) {
		const it = rs.btreeIter(m);
		const seen = walk(it);
		out.push(`iter ${rs.btreeLen(m)} ${it.len()} ${seen}`);
	} else if (step % 1009 === 0) {
		const copy = rs.btreeClone(m);
		rs.btreeRetain(m, (key) => key % 7 !== k % 7);
		out.push(`retain ${rs.btreeLen(m)} ${rs.btreeFirst(m)?.join(" ") ?? "-"} ${rs.btreeFirst(m, true)?.join(" ") ?? "-"}`);
		m = rand() & 1 ? copy : m;
		out.push(`kept ${rs.btreeLen(m)}`);
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
		out.push(`extract ${lo} ${hi} ${rs.btreeLen(m)} ${taken.join(",")}`);
		check(m);
		m = rand() & 1 ? copy : m;
	}
	if (step % 499 === 0) check(m);
}
out.push(`end ${rs.btreeLen(m)} ${rs.iterToVec(rs.btreeKeys(m)).items.join(",")}`);
// Emptied from both ends, down to its root leaf.
const drained: number[] = [];
while (rs.btreeLen(m) > 0) {
	drained.push((rs.btreePop(m, (rand() & 1) === 1) as [number, number])[0]);
	if (drained.length % 37 === 0) check(m);
}
check(m);
out.push(`drain ${drained.join(",")}`);
console.log(out.join("\n"));
