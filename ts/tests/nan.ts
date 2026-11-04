// A NaN's payload and sign, which JS leaves to the engine: what this engine keeps through
// `from_bits` and `to_bits`, the sign tests, and `copysign`, and that it quiets a
// signaling NaN stored in an array, as V8 does, which the README notes.
//
// usage: node nan.ts

import { strict as assert } from "node:assert";

import * as rs from "../runtime/rs.ts";

/** A `u64`'s value. */
function big(v: rs.u64): bigint {
	return typeof v === "number" ? BigInt(v) : (BigInt(v.hi) << 32n) + BigInt(v.lo);
}

const QUIET = 0x0008000000000000n;
for (const bits of [0x7ff8000000000000n, 0xfff8000000000000n, 0x7ff8000000000123n, 0xfffc0000deadbeefn, 0x7ff0000000000001n, 0xfff4000000000abcn]) {
	const x = rs.f64FromBits(rs.u64Of(bits));
	const negative = bits >> 63n === 1n;
	assert.equal(big(rs.f64ToBits(x)), bits, `${bits.toString(16)} round trips`);
	assert.equal(rs.f64IsSignNegative(x), negative);
	assert.equal(rs.f64IsSignPositive(x), !negative);
	assert.equal(big(rs.f64ToBits(rs.f64Copysign(x, -1))), bits | (1n << 63n));
	assert.equal(big(rs.f64ToBits(rs.f64Copysign(x, 1))), bits & ~(1n << 63n));
	// A double array holds a NaN quieted, its payload otherwise kept.
	const held = [x, 1.5];
	assert.equal(big(rs.f64ToBits(held[0])), bits | QUIET, `${bits.toString(16)} held in an array`);
}

// An `f32` NaN is a double NaN with the single's sign and payload at the top, which a
// signaling one keeps through `from_bits`, `to_bits`, its bytes, and `copysign`.
for (const bits of [0x7fc00000, 0xffc00000, 0x7fc00123, 0xffe0beef, 0x7f800001, 0xffa00abc]) {
	const x = rs.f32FromBits(bits);
	const negative = bits >>> 31 === 1;
	assert.equal(rs.f32ToBits(x), bits, `${bits.toString(16)} round trips`);
	assert.equal(rs.f32IsSignNegative(x), negative);
	assert.equal(rs.f32ToBits(rs.f32Copysign(x, -1)), (bits | 0x80000000) >>> 0);
	assert.equal(rs.f32ToBits(rs.f32Copysign(x, 1)), bits & 0x7fffffff);
	assert.equal(rs.f32ToBits(rs.f32FromBeBytes(rs.f32ToBeBytes(x))), bits);
	assert.equal(rs.f32ToBits(rs.f32FromLeBytes(rs.f32ToLeBytes(x))), bits);
}
