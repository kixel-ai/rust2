// `retain` and `dedup_by` over every keep mask of up to 6 items: prints what a B-tree, a
// deque, and a `Vec` hold after, and what each drops, which `runtime.rs` compares with
// std's. A hash map's visiting order is unspecified, so it is checked here instead: whole,
// holding each entry `f` kept.
//
// usage: node retain.ts

import * as rs from "../runtime/rs.ts";

for (let n = 0; n <= 6; n++) {
	for (let mask = 0; mask < 1 << n; mask++) {
		const keep = (i: number) => ((mask >> i) & 1) === 1;

		const tree = rs.btreeNew<number, number>(rs.cmp);
		for (let i = n - 1; i >= 0; i--) rs.btreeInsert(tree, i, i * 10);
		rs.btreeRetain(tree, (k) => keep(k));

		// Half pushed at the front, so the ring wraps.
		const deque = rs.dequeNew<number>();
		for (let i = (n >> 1) - 1; i >= 0; i--) rs.dequePushFront(deque, i);
		for (let i = n >> 1; i < n; i++) rs.dequePushBack(deque, i);
		rs.dequeRetain(deque, keep);

		const vec = rs.vecFrom([...Array(n).keys()]);
		const dropped: number[] = [];
		rs.vecRetain(vec, keep, (i) => dropped.push(i));

		// A run ends at each item the mask clears.
		const runs = rs.vecFrom<[number, number]>([]);
		for (let i = 0, run = 0; i < n; i++) runs.items.push([i, keep(i) ? run : ++run]);
		const deduped: number[] = [];
		rs.vecDedupBy(
			runs,
			(a, b) => a[1] === b[1],
			(r) => deduped.push(r[0]),
		);

		const entries = rs.iterToVec(rs.btreeIter(tree)).items;
		const items = rs.iterToVec(rs.dequeIter(deque)).items;
		const ran = runs.items.map((r) => r[0]);
		console.log(
			`${n} ${mask} btree ${entries.join(";")} deque ${items.join(",")} vec ${vec.items.join(",")} dropped ${dropped.join(",")} dedup ${ran.join(",")} dropped ${deduped.join(",")}`,
		);

		// Keys that collide by twos, so `retain` unlinks within chains.
		type Key = { k: number };
		const hashed = rs.hashNew<Key, number>(
			rs.hashing<Key>(
				(key, s) => rs.hash32(key.k >> 1, s),
				(a, b) => a.k === b.k,
			),
		);
		for (let i = 0; i < n; i++) rs.hashInsert(hashed, { k: i }, i);
		rs.hashRetain(hashed, (key) => keep(key.k));
		const held = rs.iterToVec(rs.hashIter(hashed)).items.map(([key, v]) => {
			if (rs.hashGet(hashed, key) !== v) throw new Error(`${key.k} is not found`);
			return key.k;
		});
		const want = [...Array(n).keys()].filter(keep);
		if (rs.hashLen(hashed) !== held.length || held.sort().join() !== want.join()) {
			throw new Error(`hash map of ${n} by ${mask}: ${held} of ${rs.hashLen(hashed)}`);
		}
	}
}
