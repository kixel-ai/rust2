// `try_reserve` of a byte vector or deque gives std's `AllocError`, of the layout it asked
// for, where JS can't make the bytes, and leaves the collection as it was. No allocation
// fails on demand natively, so `Uint8Array` throws here as JS does past what it can make.
//
// usage: node reserve.ts

import { strict as assert } from "node:assert";

import * as rs from "../runtime/rs.ts";

/** The length past which a `Uint8Array` throws, as V8's does out of memory. */
const LIMIT = 1 << 20;

const Native = Uint8Array;
globalThis.Uint8Array = class extends Native {
	constructor(...args: unknown[]) {
		if (typeof args[0] === "number" && args[0] >= LIMIT) throw new RangeError("Array buffer allocation failed");
		super(...(args as []));
	}
} as Uint8ArrayConstructor;

const debug = (e: rs.TryReserveError, spec: rs.Spec = {}) => rs.fmtToString(rs.fmtSpec(spec, (f) => rs.fmtTryReserveErrorDebug(e, f))).v;
const display = (e: rs.TryReserveError) => rs.fmtToString((f) => rs.fmtTryReserveErrorDisplay(e, f)).v;
const failed = (r: rs.Result<undefined, rs.TryReserveError>, size: number): rs.TryReserveError => {
	assert.ok(!r.ok);
	assert.deepEqual(r.error.layout, { size, align: 1 });
	return r.error;
};

// Grown as `reserve` grows it, to double or to what it needs; exactly to what it needs.
const v = rs.byteVecWithCapacity(LIMIT / 2);
rs.byteVecPush(v, 7);
const data = v.data;
const grown = failed(rs.byteVecTryReserve(v, LIMIT / 2 + 1), LIMIT);
failed(rs.byteVecTryReserveExact(v, LIMIT), LIMIT + 1);
assert.equal(v.data, data);
assert.deepEqual([...rs.byteVecView(v)], [7]);
assert.deepEqual(rs.byteVecTryReserve(v, LIMIT / 2 - 1), rs.OK);

assert.equal(debug(grown), "TryReserveError { kind: AllocError { layout: Layout { size: 1048576, align: 1 (1 << 0) }, non_exhaustive: () } }");
assert.equal(debug(grown, { debugHex: "x" }), "TryReserveError { kind: AllocError { layout: Layout { size: 100000, align: 1 (1 << 0) }, non_exhaustive: () } }");
assert.equal(display(grown), "memory allocation failed because the memory allocator returned an error");
const overflow = rs.byteVecTryReserve(v, 0x7fffffff);
assert.ok(!overflow.ok);
assert.ok(rs.tryReserveErrorEq(grown, new rs.TryReserveError({ size: LIMIT, align: 1 })));
assert.ok(!rs.tryReserveErrorEq(grown, overflow.error));
assert.ok(!rs.tryReserveErrorEq(grown, new rs.TryReserveError({ size: LIMIT + 1, align: 1 })));

// A deque of bytes, wrapped around its end, keeps its items in place.
const q = rs.dequeWithCapacity<number>(4, 1, true);
for (const b of [1, 2, 3, 4]) rs.dequePushBack(q, b);
rs.dequePopFront(q);
rs.dequePushBack(q, 5);
const slots = q.b;
failed(rs.dequeTryReserve(q, LIMIT), LIMIT + 4);
failed(rs.dequeTryReserveExact(q, LIMIT - 4), LIMIT);
assert.equal(q.b, slots);
assert.deepEqual(rs.iterToVec(rs.dequeIter(q)).items, [2, 3, 4, 5]);
