// `try_reserve` of a byte vector or deque gives std's `AllocError`, of the layout it asked
// for, where JS can't make the bytes, and leaves the collection as it was. No allocation
// fails on demand natively, so `Uint8Array` throws here as JS does past what it can make.
//
// usage: node reserve.ts

import { strict as assert } from "node:assert";

import * as rt from "../runtime/rt.ts";

/** The length past which a `Uint8Array` throws, as V8's does out of memory. */
const LIMIT = 1 << 20;

const Native = Uint8Array;
globalThis.Uint8Array = class extends Native {
	constructor(...args: unknown[]) {
		if (typeof args[0] === "number" && args[0] >= LIMIT) throw new RangeError("Array buffer allocation failed");
		super(...(args as []));
	}
} as Uint8ArrayConstructor;

const debug = (e: rt.TryReserveError, spec: rt.Spec = {}) => rt.fmtToString(rt.fmtSpec(spec, (f) => rt.fmtTryReserveErrorDebug(e, f)));
const display = (e: rt.TryReserveError) => rt.fmtToString((f) => rt.fmtTryReserveErrorDisplay(e, f));
const failed = (r: rt.Result<undefined, rt.TryReserveError>, size: number): rt.TryReserveError => {
	assert.ok(!r.ok);
	assert.deepEqual(r.error.layout, { size, align: 1 });
	return r.error;
};

// Grown as `reserve` grows it, to double or to what it needs; exactly to what it needs.
const v = rt.byteVecWithCapacity(LIMIT / 2);
rt.byteVecPush(v, 7);
const data = v.data;
const grown = failed(rt.byteVecTryReserve(v, LIMIT / 2 + 1), LIMIT);
failed(rt.byteVecTryReserveExact(v, LIMIT), LIMIT + 1);
assert.equal(v.data, data);
assert.deepEqual([...rt.byteVecView(v)], [7]);
assert.deepEqual(rt.byteVecTryReserve(v, LIMIT / 2 - 1), rt.OK);

assert.equal(debug(grown), "TryReserveError { kind: AllocError { layout: Layout { size: 1048576, align: 1 (1 << 0) }, non_exhaustive: () } }");
assert.equal(debug(grown, { debugHex: "x" }), "TryReserveError { kind: AllocError { layout: Layout { size: 100000, align: 1 (1 << 0) }, non_exhaustive: () } }");
assert.equal(display(grown), "memory allocation failed because the memory allocator returned an error");
const overflow = rt.byteVecTryReserve(v, 0x7fffffff);
assert.ok(!overflow.ok);
assert.ok(rt.tryReserveErrorEq(grown, new rt.TryReserveError({ size: LIMIT, align: 1 })));
assert.ok(!rt.tryReserveErrorEq(grown, overflow.error));
assert.ok(!rt.tryReserveErrorEq(grown, new rt.TryReserveError({ size: LIMIT + 1, align: 1 })));

// A deque of bytes, wrapped around its end, keeps its items in place.
const q = rt.dequeWithCapacity<number>(4, 1, true);
for (const b of [1, 2, 3, 4]) rt.dequePushBack(q, b);
rt.dequePopFront(q);
rt.dequePushBack(q, 5);
const slots = q.b;
failed(rt.dequeTryReserve(q, LIMIT), LIMIT + 4);
failed(rt.dequeTryReserveExact(q, LIMIT - 4), LIMIT);
assert.equal(q.b, slots);
assert.deepEqual(rt.iterToVec(rt.dequeIter(q)).d, [2, 3, 4, 5]);
