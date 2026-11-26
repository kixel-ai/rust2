// Division of 64- and 128-bit limbs by a divisor under 2^32, against bigints: each width and
// sign, a dividend of every bit length, and divisors either side of the 2^16 and 2^21 steps the
// long division switches at.
//
// usage: node limbs.ts

import { strict as assert } from "node:assert";

import * as rs from "../runtime/rs.ts";

// xorshift64*, seeded, so a failure replays.
let state = 0x9e3779b97f4a7c15n;
function next(): bigint {
	state ^= state >> 12n;
	state ^= (state << 25n) & 0xffffffffffffffffn;
	state ^= state >> 27n;
	return (state * 0x2545f4914f6cdd1dn) & 0xffffffffffffffffn;
}

/** A random value of `bits` bits at most, a bit length first so the short ones are as common as the long. */
function random(bits: number): bigint {
	const length = Number(next() % BigInt(bits + 1));
	let v = 0n;
	for (let i = 0; i < bits; i += 64) v = (v << 64n) | next();
	return v & ((1n << BigInt(length)) - 1n);
}

const types = [
	{ name: "u64", bits: 64, signed: false, div: rs.u64Div, rem: rs.u64Rem },
	{ name: "i64", bits: 64, signed: true, div: rs.i64Div, rem: rs.i64Rem },
	{ name: "u128", bits: 128, signed: false, div: rs.u128Div, rem: rs.u128Rem },
	{ name: "i128", bits: 128, signed: true, div: rs.i128Div, rem: rs.i128Rem },
] as const;

const edges = [1n, 2n, 3n, 10n, 0xffffn, 0x10000n, 0x10001n, 0x1fffffn, 0x200000n, 0x200001n, 0x7fffffffn, 0x80000000n, 0xffffffffn, 90_000n, 1_000_000n];

for (const { name, bits, signed, div, rem } of types) {
	const b = BigInt(bits);
	const [min, max] = signed ? [-(1n << (b - 1n)), (1n << (b - 1n)) - 1n] : [0n, (1n << b) - 1n];
	const dividends = [min, min + 1n, max, max - 1n, (1n << 53n) - 1n, 1n << 53n, (1n << 53n) + 1n, 0xffffffffffffffffn & max, 1n << 64n & max, (1n << 64n) - 1n & max];
	for (let i = 0; i < 3000; i++) {
		const v = random(signed ? bits - 1 : bits);
		dividends.push(signed && next() & 1n ? -v : v);
	}
	for (const n of dividends) {
		const divisors = [...edges, random(32) || 1n, random(21) || 1n, random(16) || 1n];
		for (const d of divisors) {
			for (const s of signed ? [d, -d] : [d]) {
				if (s === -1n && n === min) continue;
				const a = rs.intFromBig(n, bits, signed) as never;
				const c = rs.intFromBig(s, bits, signed) as never;
				const where = `${name}: ${n} / ${s}`;
				assert.equal(rs.intToBig(div(a, c)), n / s, `${where} quotient`);
				assert.equal(rs.intToBig(rem(a, c)), n % s, `${where} remainder`);
			}
		}
	}
}
