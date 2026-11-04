// `retain` and `dedup_by` over every keep mask of up to 6 items: prints what a B-tree, a
// deque, and a `Vec` hold after, and what each drops, which `runtime.rs` compares with
// std's. A hash map's visiting order is unspecified, so it is checked here instead: whole,
// holding each entry `f` kept.
//
// usage: node retain.ts

import * as rt from "../runtime/rt.ts";

for (let n = 0; n <= 6; n++) {
	for (let mask = 0; mask < 1 << n; mask++) {
		const keep = (i: number) => ((mask >> i) & 1) === 1;

		const tree = rt.btreeNew<number, number>(rt.cmp);
		for (let i = n - 1; i >= 0; i--) rt.btreeInsert(tree, i, i * 10);
		rt.btreeRetain(tree, (k) => keep(k));

		// Half pushed at the front, so the ring wraps.
		const deque = rt.dequeNew<number>();
		for (let i = (n >> 1) - 1; i >= 0; i--) rt.dequePushFront(deque, i);
		for (let i = n >> 1; i < n; i++) rt.dequePushBack(deque, i);
		rt.dequeRetain(deque, keep);

		const vec = rt.vecFrom([...Array(n).keys()]);
		const dropped: number[] = [];
		rt.vecRetain(vec, keep, (i) => dropped.push(i));

		// A run ends at each item the mask clears.
		const runs = rt.vecFrom<[number, number]>([]);
		for (let i = 0, run = 0; i < n; i++) runs.d.push([i, keep(i) ? run : ++run]);
		const deduped: number[] = [];
		rt.vecDedupBy(
			runs,
			(a, b) => a[1] === b[1],
			(r) => deduped.push(r[0]),
		);

		const entries = rt.iterToVec(rt.btreeIter(tree)).d;
		const items = rt.iterToVec(rt.dequeIter(deque)).d;
		const ran = runs.d.map((r) => r[0]);
		console.log(
			`${n} ${mask} btree ${entries.join(";")} deque ${items.join(",")} vec ${vec.d.join(",")} dropped ${dropped.join(",")} dedup ${ran.join(",")} dropped ${deduped.join(",")}`,
		);

		// Keys that collide by twos, so `retain` unlinks within chains.
		type Key = { k: number };
		const hashed = rt.hashNew<Key, number>(
			rt.hashing<Key>(
				(key, s) => rt.hash32(key.k >> 1, s),
				(a, b) => a.k === b.k,
			),
		);
		for (let i = 0; i < n; i++) rt.hashInsert(hashed, { k: i }, i);
		rt.hashRetain(hashed, (key) => keep(key.k));
		const held = rt.iterToVec(rt.hashIter(hashed)).d.map(([key, v]) => {
			if (rt.hashGet(hashed, key) !== v) throw new Error(`${key.k} is not found`);
			return key.k;
		});
		const want = [...Array(n).keys()].filter(keep);
		if (rt.hashLen(hashed) !== held.length || held.sort().join() !== want.join()) {
			throw new Error(`hash map of ${n} by ${mask}: ${held} of ${rt.hashLen(hashed)}`);
		}
	}
}
