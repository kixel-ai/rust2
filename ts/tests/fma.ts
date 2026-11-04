// The `floats` fixture's `mul_add` allowance: its exact rounding agrees with the runtime's,
// and it accepts native a last place off only where the translation is exact.
//
// usage: node fma.ts

import { strict as assert } from "node:assert";

import { exactMulAdd, mulAddsWithin } from "../fixtures/lang/floats/fma.ts";
import * as rt from "../runtime/rt.ts";

function double(bits: bigint): number {
	const view = new DataView(new ArrayBuffer(8));
	view.setBigUint64(0, bits);
	return view.getFloat64(0);
}

function bits(x: number): bigint {
	const view = new DataView(new ArrayBuffer(8));
	view.setFloat64(0, x);
	return view.getBigUint64(0);
}

/** `mul_adds`' output for one set: its operands, then `result`. */
function set(a: number, b: number, c: number, result: number): Uint8Array {
	const out = new Uint8Array(32);
	const view = new DataView(out.buffer);
	[a, b, c, result].forEach((x, i) => view.setFloat64(i * 8, x));
	return out;
}

/** The double `n` places above `x`, below if negative. */
const step = (x: number, n: bigint) => double(bits(x) + n);

// Exact rounding at the edges: a tie broken by the product's error alone, an overflow the
// sum brings back, subnormal results, and signed zeros.
const eps = 2 ** -52;
assert.equal(exactMulAdd(1 + eps, 1 + eps, -1), 2 * eps + eps * eps);
assert.equal(exactMulAdd(1 + 2 ** -27, 1 - 2 ** -27, -1), -(2 ** -54));
assert.equal(exactMulAdd(eps / 2 + (eps * eps) / 2, 1 - eps / 2, 1), 1 + eps);
assert.equal(exactMulAdd(Number.MAX_VALUE, 2, -Number.MAX_VALUE), Number.MAX_VALUE);
assert.equal(exactMulAdd(Number.MAX_VALUE, 1 + eps, 0), Infinity);
assert.equal(exactMulAdd(5e-324, 0.5, 5e-324), 1e-323);
assert.equal(exactMulAdd(5e-324, 1.5, 0), 1e-323);
assert.ok(Object.is(exactMulAdd(-0, 1, -0), -0));
assert.ok(Object.is(exactMulAdd(0, -1, 0), 0));
assert.ok(Object.is(exactMulAdd(1, -1, 1), 0));
assert.equal(exactMulAdd(Infinity, 1, 1), undefined);

// It agrees with the runtime's `f64MulAdd` over operands of any bits, and near-cancelling ones.
let state = 0x9e3779b97f4a7c15n;
function next(): bigint {
	state ^= (state << 13n) & 0xffffffffffffffffn;
	state ^= state >> 7n;
	state ^= (state << 17n) & 0xffffffffffffffffn;
	return state;
}
for (let i = 0; i < 20000; i++) {
	const [a, b] = [double(next()), double(next())];
	const c = i % 2 === 0 ? double(next()) : -(a * b);
	const exact = exactMulAdd(a, b, c);
	if (exact === undefined) continue;
	assert.ok(Object.is(exact, rt.f64MulAdd(a, b, c)), `${a}, ${b}, ${c}`);
}

// A product of half `c`'s last place that only its error moves off the tie, which wasm32's
// `fma` rounds to `c` (rust-lang/compiler-builtins#1353).
const [a, b, c] = [double(0xbe8c27a8edc66e22n), double(0xbd522f6292409f88n), double(0x3f41e8176d8609aan)];
const exact = exactMulAdd(a, b, c) as number;
assert.equal(bits(exact), 0x3f41e8176d8609abn);
const right = set(a, b, c, exact);
assert.ok(mulAddsWithin(set(a, b, c, c), right), "native's own result");
assert.ok(mulAddsWithin(set(a, b, c, step(exact, 1n)), right), "native a place above");
assert.ok(mulAddsWithin(set(a, b, c, step(exact, -1n)), right), "native a place below");
assert.ok(mulAddsWithin(right, right));
// Native two places off.
assert.ok(!mulAddsWithin(set(a, b, c, step(exact, 2n)), right));
// The translation is the side a place off.
assert.ok(!mulAddsWithin(right, set(a, b, c, step(exact, 1n))));
// Both off, a place apart.
assert.ok(!mulAddsWithin(set(a, b, c, step(exact, 1n)), set(a, b, c, step(exact, 2n))));
// Operands that differ.
assert.ok(!mulAddsWithin(set(a, b, c, step(exact, 1n)), set(a, b, step(c, 1n), exact)));
// A set past the first, and outputs of different lengths.
assert.ok(!mulAddsWithin(new Uint8Array([...right, ...set(a, b, c, step(exact, 2n))]), new Uint8Array([...right, ...right])));
assert.ok(!mulAddsWithin(set(a, b, c, step(exact, 1n)), new Uint8Array([...right, ...right])));
// Native infinite beside the largest double.
const max = Number.MAX_VALUE;
assert.ok(!mulAddsWithin(set(max, 1, 0, Infinity), set(max, 1, 0, max)));
