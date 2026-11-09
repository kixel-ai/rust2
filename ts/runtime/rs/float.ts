// `f64` and `f32`, for rust2-ts output. An `f64` is a number, which is the same IEEE 754
// double, so its operators are JS's. An `f32` is a number that a single holds exactly,
// each operation's result rounded to one by `Math.fround`, which rounds as Rust does: a
// double holds the exact sum, product, or quotient of two singles closely enough that
// rounding it again gives the single nearest the exact value. These are their methods
// where JS differs from std or has none, each its own function, `f64Round`, so a bundler
// drops every one translated code never calls. `rs.ts` re-exports all of it.
//
// JS doesn't specify which NaN an operation makes, so a NaN's payload and sign are the
// engine's: `to_bits` reads what it keeps. An `f32` NaN is the double NaN with its sign
// and its payload at the top, which a signaling one keeps, where JS's conversion from a
// single would quiet it.

import { Err, type FmtPart, type FmtResult, type Formatter, fmtOwned, fmtPadFormatted, Ok, panicFmt, type Result } from "../rs.ts";
import type { Ordering } from "./collections.ts";
import { type f32, type f64, type u32, type u64, u64FromHalves } from "./int.ts";

/** Eight bytes that hold a double while its bits are read or written, made on first use so a bundler drops them unused. */
let scratch: DataView | undefined;
const bytes = (): DataView => (scratch ??= new DataView(new ArrayBuffer(8)));
const TWO32 = 4294967296;
const TWO52 = 4503599627370496;
/** `f64::MIN_POSITIVE`, the least normal double. */
const MIN_POSITIVE = 2.2250738585072014e-308;

/** `f64::round`: to the nearest integer, half away from zero. */
export function f64Round(x: f64): f64 {
	const t = Math.trunc(x);
	// `x - t` is exact, and NaN for an infinity, which `t` already is.
	return Math.abs(x - t) >= 0.5 ? t + Math.sign(x) : t;
}

/** `f64::round_ties_even`: to the nearest integer, half to even. */
export function f64RoundTiesEven(x: f64): f64 {
	const a = Math.abs(x);
	// Every double from 2^52 up is an integer; below it, adding 2^52 rounds to one, ties to even.
	if (!(a < TWO52)) return x;
	const r = a + TWO52 - TWO52;
	return x < 0 ? -r : x === 0 ? x : r;
}

/** `f64::fract`: `x - x.trunc()`. */
export const f64Fract = (x: f64): f64 => x - Math.trunc(x);

/** `f64::is_sign_negative`: whether the sign bit is set, as it is for -0.0. */
export function f64IsSignNegative(x: f64): boolean {
	if (x === x) return x < 0 || 1 / x < 0;
	const view = bytes();
	view.setFloat64(0, x);
	return view.getUint8(0) >= 0x80;
}

/** `f64::is_sign_positive`: whether the sign bit is clear. */
export const f64IsSignPositive = (x: f64): boolean => !f64IsSignNegative(x);

/** `f64::signum`: 1.0 or -1.0 by the sign bit, so -1.0 for -0.0, and NaN for NaN. */
export const f64Signum = (x: f64): f64 => (x !== x ? NaN : f64IsSignNegative(x) ? -1 : 1);

/** `f64::copysign`: `x`'s magnitude with `y`'s sign bit. */
export function f64Copysign(x: f64, y: f64): f64 {
	const negative = f64IsSignNegative(y);
	if (x === x) return negative ? -Math.abs(x) : Math.abs(x);
	const view = bytes();
	view.setFloat64(0, x);
	const top = view.getUint8(0) & 0x7f;
	view.setUint8(0, negative ? top | 0x80 : top);
	return view.getFloat64(0);
}

/** `f64::is_infinite`. */
export const f64IsInfinite = (x: f64): boolean => Math.abs(x) === Infinity;

/** `f64::is_normal`: neither zero, subnormal, infinite, nor NaN. */
export function f64IsNormal(x: f64): boolean {
	const a = Math.abs(x);
	return a >= MIN_POSITIVE && a < Infinity;
}

/** `f64::is_subnormal`. */
export function f64IsSubnormal(x: f64): boolean {
	const a = Math.abs(x);
	return a > 0 && a < MIN_POSITIVE;
}

/** `x`'s bits moved by one toward +infinity if `up`, from a finite or infinite non-NaN. */
function step(x: number, up: boolean): number {
	if (x === 0) return up ? 5e-324 : -5e-324;
	const view = bytes();
	view.setFloat64(0, x);
	let hi = view.getUint32(0);
	let lo = view.getUint32(4);
	// The magnitude grows if it moves away from zero.
	if (up === x > 0) {
		lo = (lo + 1) >>> 0;
		if (lo === 0) hi++;
	} else {
		if (lo === 0) hi--;
		lo = (lo - 1) >>> 0;
	}
	view.setUint32(0, hi);
	view.setUint32(4, lo);
	return view.getFloat64(0);
}

/** `f64::next_up`: the least double greater than `x`. */
export const f64NextUp = (x: f64): f64 => (x !== x || x === Infinity ? x : step(x, true));

/** `f64::next_down`: the greatest double less than `x`. */
export const f64NextDown = (x: f64): f64 => (x !== x || x === -Infinity ? x : step(x, false));

/** `f64::max`: the greater, ignoring a NaN. */
export const f64Max = (a: f64, b: f64): f64 => (a !== a ? b : b !== b ? a : Math.max(a, b));

/** `f64::min`: the lesser, ignoring a NaN. */
export const f64Min = (a: f64, b: f64): f64 => (a !== a ? b : b !== b ? a : Math.min(a, b));

/** `f64::midpoint`: halved before the sum where the sum could overflow. */
export function f64Midpoint(a: f64, b: f64): f64 {
	const hi = Number.MAX_VALUE * 0.5;
	return Math.abs(a) <= hi && Math.abs(b) <= hi ? (a + b) * 0.5 : a * 0.5 + b * 0.5;
}

/** `f64::to_bits`. */
export function f64ToBits(x: f64): u64 {
	const view = bytes();
	view.setFloat64(0, x);
	return u64FromHalves(view.getUint32(0), view.getUint32(4));
}

/** `f64::from_bits`. */
export function f64FromBits(v: u64): f64 {
	const view = bytes();
	view.setUint32(0, typeof v === "number" ? Math.floor(v / TWO32) : v.hi);
	view.setUint32(4, typeof v === "number" ? v >>> 0 : v.lo);
	return view.getFloat64(0);
}

function toBytes(x: number, little: boolean): Uint8Array {
	const out = new Uint8Array(8);
	new DataView(out.buffer).setFloat64(0, x, little);
	return out;
}

function fromBytes(bytes: Uint8Array, little: boolean): number {
	return new DataView(bytes.buffer, bytes.byteOffset, 8).getFloat64(0, little);
}

/** `f64::to_be_bytes`. */
export const f64ToBeBytes = (x: f64): Uint8Array => toBytes(x, false);

/** `f64::to_le_bytes`, and `to_ne_bytes`, as wasm32 is little-endian. */
export const f64ToLeBytes = (x: f64): Uint8Array => toBytes(x, true);

/** `f64::from_be_bytes`. */
export const f64FromBeBytes = (bytes: Uint8Array): f64 => fromBytes(bytes, false);

/** `f64::from_le_bytes`, and `from_ne_bytes`, as wasm32 is little-endian. */
export const f64FromLeBytes = (bytes: Uint8Array): f64 => fromBytes(bytes, true);

/** `f64::total_cmp`: IEEE 754's total order, by the bits, with a negative's flipped. */
export function f64TotalCmp(a: f64, b: f64): Ordering {
	const view = bytes();
	view.setFloat64(0, a);
	let ah = view.getInt32(0);
	let al = view.getUint32(4);
	if (ah < 0) {
		ah ^= 0x7fffffff;
		al = ~al >>> 0;
	}
	view.setFloat64(0, b);
	let bh = view.getInt32(0);
	let bl = view.getUint32(4);
	if (bh < 0) {
		bh ^= 0x7fffffff;
		bl = ~bl >>> 0;
	}
	return ah < bh ? -1 : ah > bh ? 1 : al < bl ? -1 : al > bl ? 1 : 0;
}

/** `f64`'s `PartialOrd::partial_cmp`: `undefined`, which is `None`, if either is NaN. */
export const f64PartialCmp = (a: f64, b: f64): Ordering | undefined => (a < b ? -1 : a > b ? 1 : a === b ? 0 : undefined);

/** The identity, as an `f64` rounds what JS computes. */
const double = (x: number): number => x;

/** `powi`, multiplied by squaring as compiler-builtins' `__powidf2` is, each product rounded by `round`. */
function powi(x: number, n: number, round: (x: number) => number): number {
	let a = x;
	let pow = Math.abs(n);
	let mul = 1;
	for (;;) {
		if (pow & 1) mul = round(mul * a);
		// `>>>` keeps 2^31, the magnitude of `i32::MIN`, unsigned.
		pow >>>= 1;
		if (pow === 0) break;
		a = round(a * a);
	}
	return n < 0 ? round(1 / mul) : mul;
}

/** `f64::powi`. */
export const f64Powi = (x: f64, n: number): f64 => powi(x, n, double);

/** libm's `pow`: 1 for a base of 1, and for -1 to an infinite power, where `Math.pow` is NaN. */
const pow = (x: number, y: number): number => (x === 1 || (x === -1 && Math.abs(y) === Infinity) ? 1 : Math.pow(x, y));

/** `f64::powf`. */
export const f64Powf = (x: f64, y: f64): f64 => pow(x, y);

/** `f64::sin_cos`. */
export const f64SinCos = (x: f64): [f64, f64] => [Math.sin(x), Math.cos(x)];

/** `atanh`, as std computes it rather than libm, each step rounded by `round`. */
const atanh = (x: number, round: (x: number) => number): number => round(0.5 * round(Math.log1p(round(round(2 * x) / round(1 - x)))));

/** `f64::atanh`. */
export const f64Atanh = (x: f64): f64 => atanh(x, double);

function divEuclid(x: number, rhs: number, round: (x: number) => number): number {
	const q = Math.trunc(round(x / rhs));
	if (x % rhs < 0) return round(rhs > 0 ? q - 1 : q + 1);
	return q;
}

/** `f64::div_euclid`. */
export const f64DivEuclid = (x: f64, rhs: f64): f64 => divEuclid(x, rhs, double);

function remEuclid(x: number, rhs: number, round: (x: number) => number): number {
	const r = x % rhs;
	return r < 0 ? round(r + Math.abs(rhs)) : r;
}

/** `f64::rem_euclid`. */
export const f64RemEuclid = (x: f64, rhs: f64): f64 => remEuclid(x, rhs, double);

/** `f64`'s `+`, which `Iterator::sum` folds with. */
export const f64Add = (a: f64, b: f64): f64 => a + b;

/** `f64`'s `*`, which `Iterator::product` folds with. */
export const f64Mul = (a: f64, b: f64): f64 => a * b;

/** 2^27 + 1, which splits a double into two halves whose products are exact: Veltkamp's split. */
const SPLIT = 134217729;

/**
 * `f64::mul_add`: `a * b + c` rounded once, as compiler-builtins' `fma` is. Dekker's
 * product and Knuth's TwoSum make it exact as three doubles, which Boldo and Melquiond's
 * emulation adds with one rounding. Where a step could overflow or lose bits to
 * subnormals, the significands' exact sum rounds instead.
 */
export function f64MulAdd(a: f64, b: f64, c: f64): f64 {
	const p = a * b;
	const m = Math.abs(p);
	// The split doesn't overflow, the product's exponents sum to at least -970 so its error
	// is a double, and `p + c` doesn't overflow. NaN fails each.
	if (Math.abs(a) < 2 ** 996 && Math.abs(b) < 2 ** 996 && m >= 2 ** -968 && m < 2 ** 1021 && Math.abs(c) < 2 ** 1021) {
		let t = a * SPLIT;
		const ah = t - (t - a);
		const al = a - ah;
		t = b * SPLIT;
		const bh = t - (t - b);
		const bl = b - bh;
		// `p + pl` is `a * b`, and `s + sl` is `p + c`.
		const pl = ah * bh - p + ah * bl + al * bh + al * bl;
		const s = p + c;
		const sl = error(p, c, s);
		const l = sl + pl;
		// Where `s` is inexact, `c` didn't cancel half of `p`, so `s`'s last place is far
		// above the subnormals' and `l`'s odd bit sticks below it.
		return s + odd(l, error(sl, pl, l));
	}
	return mulAddExact(a, b, c);
}

/** The rounding error of `s`, the double nearest `a + b`, exactly: Knuth's TwoSum. */
function error(a: number, b: number, s: number): number {
	const bb = s - a;
	return a - (s - bb) + (b - bb);
}

/**
 * `s + e` rounded to odd, from `s`, its nearest double, and `e`, the exact error: `s`'s odd
 * neighbor toward `e` if `s` is even and `e` isn't 0. The odd last bit records that bits
 * were lost, so a rounding to at least two fewer bits gives what rounding `s + e` would.
 */
function odd(s: number, e: number): number {
	if (e === 0) return s;
	const view = bytes();
	view.setFloat64(0, s);
	return view.getUint32(4) & 1 ? s : step(s, e > 0);
}

/** `a * b + c` rounded once from the operands' exact significands. */
function mulAddExact(a: number, b: number, c: number): number {
	// A product that is infinite, NaN, or exactly zero adds as JS's does, and an infinite
	// `c` wins over a product that would only overflow.
	if (!Number.isFinite(a) || !Number.isFinite(b) || a === 0 || b === 0) return a * b + c;
	if (!Number.isFinite(c)) return c;
	const [ma, ea] = significand(a);
	const [mb, eb] = significand(b);
	const [mc, ec] = significand(c);
	const sign = (x: number) => (x < 0 ? -1n : 1n);
	// The sum in units of the lesser term's last place.
	const e = Math.min(ea + eb, ec);
	const n = sign(a) * sign(b) * ((ma * mb) << BigInt(ea + eb - e)) + sign(c) * (mc << BigInt(ec - e));
	// Terms that cancel exactly make +0.0, as rounding to nearest does.
	if (n === 0n) return 0;
	const magnitude = n < 0n ? -n : n;
	const x = e >= 0 ? quotient(magnitude << BigInt(e), 1n, F64) : quotient(magnitude, 1n << BigInt(-e), F64);
	return n < 0n ? -x : x;
}

/** `clamp`, which panics with both bounds' `Debug` unless `min <= max`. */
function clamp(x: number, min: number, max: number, debug: (x: number, f: Formatter) => FmtResult, key: string): number {
	if (!(min <= max)) panicFmt(["min > max, or either was NaN. min = ", fmtOwned(min, debug), ", max = ", fmtOwned(max, debug)], key);
	return x < min ? min : x > max ? max : x;
}

/** `f64::clamp`. */
export const f64Clamp = (x: f64, min: f64, max: f64): f64 => clamp(x, min, max, f64Debug, "f64_clamp");

// `f32`'s methods: an `f64`'s where they give the same single, else rounded as std's.

/** `f32::MIN_POSITIVE`, the least normal single. */
const MIN_POSITIVE_F32 = 1.1754943508222875e-38;

/** `f32::round`. */
export const f32Round: (x: f32) => f32 = f64Round as never;
/** `f32::round_ties_even`. */
export const f32RoundTiesEven: (x: f32) => f32 = f64RoundTiesEven as never;
/** `f32::fract`. */
export const f32Fract: (x: f32) => f32 = f64Fract as never;
/** `f32::is_sign_negative`. */
export const f32IsSignNegative: (x: f32) => boolean = f64IsSignNegative as never;
/** `f32::is_sign_positive`. */
export const f32IsSignPositive: (x: f32) => boolean = f64IsSignPositive as never;
/** `f32::signum`. */
export const f32Signum: (x: f32) => f32 = f64Signum as never;
/** `f32::copysign`. */
export const f32Copysign: (x: f32, y: f32) => f32 = f64Copysign as never;
/** `f32::is_infinite`. */
export const f32IsInfinite: (x: f32) => boolean = f64IsInfinite as never;
/** `f32::max`. */
export const f32Max: (a: f32, b: f32) => f32 = f64Max as never;
/** `f32::min`. */
export const f32Min: (a: f32, b: f32) => f32 = f64Min as never;
/** `f32::total_cmp`, which orders singles' bits as their doubles' order. */
export const f32TotalCmp: (a: f32, b: f32) => Ordering = f64TotalCmp as never;
/** `f32`'s `PartialOrd::partial_cmp`. */
export const f32PartialCmp: (a: f32, b: f32) => Ordering | undefined = f64PartialCmp as never;

/** `f32::is_normal`. */
export function f32IsNormal(x: f32): boolean {
	const a = Math.abs(x);
	return a >= MIN_POSITIVE_F32 && a < Infinity;
}

/** `f32::is_subnormal`. */
export function f32IsSubnormal(x: f32): boolean {
	const a = Math.abs(x);
	return a > 0 && a < MIN_POSITIVE_F32;
}

/** `x`'s bits as a single moved by one toward +infinity if `up`, from a finite or infinite non-NaN. */
function step32(x: number, up: boolean): number {
	if (x === 0) return up ? 1.401298464324817e-45 : -1.401298464324817e-45;
	const view = bytes();
	view.setFloat32(0, x);
	// The magnitude grows if it moves away from zero.
	view.setUint32(0, view.getUint32(0) + (up === x > 0 ? 1 : -1));
	return view.getFloat32(0);
}

/** `f32::next_up`: the least single greater than `x`. */
export const f32NextUp = (x: f32): f32 => (x !== x || x === Infinity ? x : step32(x, true));

/** `f32::next_down`: the greatest single less than `x`. */
export const f32NextDown = (x: f32): f32 => (x !== x || x === -Infinity ? x : step32(x, false));

/** `f32::midpoint`, which std computes in `f64` on wasm32. */
export const f32Midpoint = (a: f32, b: f32): f32 => Math.fround((a + b) * 0.5);

/** `f32::to_bits`: a NaN's sign and payload are the top of its double's. */
export function f32ToBits(x: f32): u32 {
	const view = bytes();
	if (x === x) {
		view.setFloat32(0, x);
		return view.getUint32(0);
	}
	view.setFloat64(0, x);
	const hi = view.getUint32(0);
	return ((hi & 0x80000000) | 0x7f800000 | ((hi & 0xfffff) << 3) | (view.getUint32(4) >>> 29)) >>> 0;
}

/** `f32::from_bits`: a NaN's sign and payload go to the top of its double's, which `getFloat32` would quiet. */
export function f32FromBits(v: u32): f32 {
	const view = bytes();
	if ((v & 0x7f800000) !== 0x7f800000 || (v & 0x7fffff) === 0) {
		view.setUint32(0, v);
		return view.getFloat32(0);
	}
	view.setUint32(0, (v & 0x80000000) | 0x7ff00000 | ((v & 0x7fffff) >>> 3));
	view.setUint32(4, v << 29);
	return view.getFloat64(0);
}

function toBytes32(x: number, little: boolean): Uint8Array {
	const out = new Uint8Array(4);
	new DataView(out.buffer).setUint32(0, f32ToBits(x), little);
	return out;
}

function fromBytes32(bytes: Uint8Array, little: boolean): number {
	return f32FromBits(new DataView(bytes.buffer, bytes.byteOffset, 4).getUint32(0, little));
}

/** `f32::to_be_bytes`. */
export const f32ToBeBytes = (x: f32): Uint8Array => toBytes32(x, false);

/** `f32::to_le_bytes`, and `to_ne_bytes`, as wasm32 is little-endian. */
export const f32ToLeBytes = (x: f32): Uint8Array => toBytes32(x, true);

/** `f32::from_be_bytes`. */
export const f32FromBeBytes = (bytes: Uint8Array): f32 => fromBytes32(bytes, false);

/** `f32::from_le_bytes`, and `from_ne_bytes`, as wasm32 is little-endian. */
export const f32FromLeBytes = (bytes: Uint8Array): f32 => fromBytes32(bytes, true);

/** `f32::powi`, multiplied by squaring in singles as compiler-builtins' `__powisf2` is. */
export const f32Powi = (x: f32, n: number): f32 => powi(x, n, Math.fround);

/** `f32::powf`: libm's `powf`, within its ULPs. */
export const f32Powf = (x: f32, y: f32): f32 => Math.fround(pow(x, y));

/** `f32::sin_cos`. */
export const f32SinCos = (x: f32): [f32, f32] => [Math.fround(Math.sin(x)), Math.fround(Math.cos(x))];

/** `f32::atanh`, as std computes it, each step in singles. */
export const f32Atanh = (x: f32): f32 => atanh(x, Math.fround);

/** `f32::div_euclid`. */
export const f32DivEuclid = (x: f32, rhs: f32): f32 => divEuclid(x, rhs, Math.fround);

/** `f32::rem_euclid`. */
export const f32RemEuclid = (x: f32, rhs: f32): f32 => remEuclid(x, rhs, Math.fround);

/** `f32`'s `+`, which `Iterator::sum` folds with. */
export const f32Add = (a: f32, b: f32): f32 => Math.fround(a + b);

/** `f32`'s `*`, which `Iterator::product` folds with. */
export const f32Mul = (a: f32, b: f32): f32 => Math.fround(a * b);

/**
 * `f32::mul_add`, rounded once as compiler-builtins' `fmaf` is: a double holds `a * b`
 * exactly, and the sum rounded to odd has 29 more bits than a single, so `Math.fround`
 * rounds it as it would the exact sum.
 */
export function f32MulAdd(a: f32, b: f32, c: f32): f32 {
	const p = a * b;
	const s = p + c;
	return Math.fround(Number.isFinite(s) ? odd(s, error(p, c, s)) : s);
}

/** `f32::clamp`. */
export const f32Clamp = (x: f32, min: f32, max: f32): f32 => clamp(x, min, max, f32Debug, "f32_clamp");

// Formatting, as `core::fmt::float` does it. A finite nonzero magnitude is digits and the
// power of ten that `0.digits` scales by, as `flt2dec` makes them.

/** A float's shortest digits that read back as it: the digits, and the power of ten that `0.digits` scales by. */
type Shortest = (x: number) => [string, number];

/** `x`'s shortest digits that read back as it, the closest to it of those, and the greater of two as close. */
function shortest(x: number): [string, number] {
	// JS picks the shortest and then the closest too, but the even one of two as close.
	const [m, e] = x.toExponential().split("e");
	const digits = m.replace(".", "");
	const exp = Number(e) + 1;
	// Two are as close only where `x` ends half a digit past them: one more binary digit
	// past the point than they have decimal ones.
	const places = digits.length - exp + 1;
	if (places < 1 || fractionBits(x) !== places) return [digits, exp];
	const [all, pos] = exact(x);
	// The greater ending in 0 would be shorter, so it can't read back as `x`.
	if (all !== digits + "5" || pos !== exp || digits.endsWith("9")) return [digits, exp];
	const up = digits.slice(0, -1) + String.fromCharCode(digits.charCodeAt(digits.length - 1) + 1);
	return nearest(up, exp - up.length, F64) === x ? [up, exp] : [digits, exp];
}

/**
 * `x`'s shortest digits that read back as the single it is, as Dragon finds them: at the
 * first length where the digits cut short of `x` or the next ones up read back, the one
 * that does, or the closer of two, the greater if `x` is half a digit past the lesser.
 */
function shortest32(x: number): [string, number] {
	const [all, pos] = exact(x);
	for (let n = 1; n < all.length; n++) {
		const down = all.slice(0, n);
		const nines = end(down, "9");
		const [up, upAt] = nines === 0 ? ["1", pos + 1] : [down.slice(0, nines - 1) + String.fromCharCode(down.charCodeAt(nines - 1) + 1), pos];
		const fitsDown = nearest(down, pos - n, F32) === x;
		const fitsUp = nearest(up, upAt - up.length, F32) === x;
		if (fitsUp && (!fitsDown || all[n] >= "5")) return [up, upAt];
		if (fitsDown) return [down, pos];
	}
	return [all, pos];
}

/** How many binary digits `x`, finite and nonzero, has past the point. */
function fractionBits(x: number): number {
	const view = bytes();
	view.setFloat64(0, x);
	const hi = view.getUint32(0);
	const lo = view.getUint32(4);
	const biased = (hi >>> 20) & 0x7ff;
	// The significand's lowest set bit, in its low word or else its high one.
	const word = lo !== 0 ? lo : (hi & 0xfffff) | (biased === 0 ? 0 : 0x100000);
	const zeros = 31 - Math.clz32(word & -word) + (lo !== 0 ? 0 : 32);
	return Math.max(0, 1075 - Math.max(biased, 1) - zeros);
}

/** A finite double's magnitude as its integer significand and the power of two that scales it. */
function significand(x: number): [bigint, number] {
	const view = bytes();
	view.setFloat64(0, x);
	const bits = view.getBigUint64(0) & 0x7fff_ffff_ffff_ffffn;
	const biased = Number(bits >> 52n);
	return [biased === 0 ? bits : (bits & 0xf_ffff_ffff_ffffn) | 0x10_0000_0000_0000n, Math.max(biased, 1) - 1075];
}

/** `x`'s exact digits, without trailing zeros, which a double, and so a single, always has: its significand times a power of two. */
function exact(x: number): [string, number] {
	const [m, e] = significand(x);
	// m * 2^-k is m * 5^k / 10^k.
	const all = (e >= 0 ? m << BigInt(e) : m * 5n ** BigInt(-e)).toString();
	return [all.slice(0, end(all, "0")), all.length + Math.min(e, 0)];
}

/** Where the run of `c` that ends `s` starts, scanned from the end, which a regex would retry from each `c` before it. */
function end(s: string, c: string): number {
	let i = s.length;
	while (i > 0 && s[i - 1] === c) i--;
	return i;
}

/** The first `n` of `digits`, rounded half to even on the rest, as `format_exact` does; none if they round to zero. */
function round(digits: string, exp: number, n: number): [string, number] {
	if (n >= digits.length) return [digits, exp];
	if (n < 0) return ["", exp];
	const kept = digits.slice(0, n);
	const next = digits[n];
	const half = next === "5" && /^0*$/.test(digits.slice(n + 1));
	// A tie with no digit kept rounds to the zero before it, which is even.
	const odd = n > 0 && kept.charCodeAt(n - 1) % 2 === 1;
	if (next < "5" || (half && !odd)) return [kept, exp];
	const nines = end(kept, "9");
	if (nines === 0) return ["1".padEnd(Math.max(n, 1), "0"), exp + 1];
	const carried = String.fromCharCode(kept.charCodeAt(nines - 1) + 1);
	return [kept.slice(0, nines - 1) + carried + "0".repeat(n - nines), exp];
}

/** `digits_to_dec_str`: at least `frac` digits past the point. */
function decimal(digits: string, exp: number, frac: number): FmtPart[] {
	if (exp <= 0) {
		const parts: FmtPart[] = ["0.", -exp, digits];
		if (frac > digits.length && frac - digits.length > -exp) parts.push(frac - digits.length + exp);
		return parts;
	}
	if (exp < digits.length) {
		const parts: FmtPart[] = [digits.slice(0, exp), ".", digits.slice(exp)];
		if (frac > digits.length - exp) parts.push(frac - (digits.length - exp));
		return parts;
	}
	return frac > 0 ? [digits, exp - digits.length, ".", frac] : [digits, exp - digits.length];
}

/** `digits_to_exp_str`: at least `frac` digits past the point. */
function exponential(digits: string, exp: number, frac: number, e: "e" | "E"): FmtPart[] {
	const parts: FmtPart[] = [digits[0]];
	const actual = digits.length - 1;
	if (actual > 0 || frac > 0) {
		parts.push(".");
		if (actual > 0) parts.push(digits.slice(1));
		if (frac > actual) parts.push(frac - actual);
	}
	parts.push(exp - 1 < 0 ? `${e}-` : e, String(Math.abs(exp - 1)));
	return parts;
}

/** `0`, with `frac` zeros past the point. */
const zero = (frac: number): FmtPart[] => (frac > 0 ? ["0.", frac] : ["0"]);

/** `to_shortest_str`: the shortest digits, with at least `frac` past the point. */
function shortestFixed(x: number, frac: number, shortest: Shortest): FmtPart[] {
	if (x === 0) return zero(frac);
	const [digits, exp] = shortest(x);
	return decimal(digits, exp, frac);
}

/**
 * The most digits `format_exact` writes of `x`, a double or a `single`: std's
 * `estimate_max_buf_len` of its decoded binary exponent. It writes zeros past the exact
 * digits up to what it is asked for, within that.
 */
function maxDigits(x: number, single: boolean): number {
	const view = bytes();
	let biased: number;
	let fraction: boolean;
	if (single) {
		view.setFloat32(0, x);
		const b = view.getUint32(0);
		biased = (b >>> 23) & 0xff;
		fraction = (b & 0x7fffff) !== 0;
	} else {
		view.setFloat64(0, x);
		const hi = view.getUint32(0);
		biased = (hi >>> 20) & 0x7ff;
		fraction = (hi & 0xfffff) !== 0 || view.getUint32(4) !== 0;
	}
	const bias = single ? 150 : 1075;
	// A power of two has its lower neighbor closer, so `decode` scales it once more.
	const exp = biased === 0 ? -bias : biased - bias - (fraction ? 1 : 2);
	return 21 + ((exp < 0 ? -12 * exp : 5 * exp) >> 4);
}

/** `to_exact_fixed_str`: rounded to `frac` digits past the point. */
function exactFixed(x: number, frac: number, single: boolean): FmtPart[] {
	if (x === 0) return zero(frac);
	const [all, pos] = exact(x);
	const [digits, exp] = round(all, pos, pos + frac);
	if (digits === "") return zero(frac);
	return decimal(digits.padEnd(Math.min(maxDigits(x, single), exp + frac), "0"), exp, frac);
}

/** `to_shortest_exp_str` without a precision, else `to_exact_exp_str`: `precision` digits past the point. */
function scientific(x: number, precision: number | undefined, e: "e" | "E", shortest: Shortest): FmtPart[] {
	if (x === 0) return precision ? ["0.", precision, `${e}0`] : [`0${e}0`];
	if (precision === undefined) {
		const [digits, exp] = shortest(x);
		return exponential(digits, exp, 0, e);
	}
	const [all, pos] = exact(x);
	const [digits, exp] = round(all, pos, precision + 1);
	return exponential(digits.padEnd(Math.min(maxDigits(x, shortest === shortest32), precision + 1), "0"), exp, precision, e);
}

/** Write `x` padded, its magnitude's parts by `finite`: a NaN has no sign, and -0.0 has one. */
function write(x: number, f: Formatter, finite: (magnitude: number) => FmtPart[]): FmtResult {
	if (x !== x) return fmtPadFormatted(f, "", ["NaN"]);
	const sign = f64IsSignNegative(x) ? "-" : f.spec.plus ? "+" : "";
	const magnitude = Math.abs(x);
	return fmtPadFormatted(f, sign, magnitude === Infinity ? ["inf"] : finite(magnitude));
}

/** `Display`: the shortest digits, or rounded to the precision. */
function display(x: number, f: Formatter, shortest: Shortest): FmtResult {
	const precision = f.spec.precision;
	return write(x, f, (m) => (precision === undefined ? shortestFixed(m, 0, shortest) : exactFixed(m, precision, shortest === shortest32)));
}

/** `Debug`: as `Display`, but with a point, and an exponent below 1e-4 and from 1e16, each as the type has it. */
function debug(x: number, f: Formatter, shortest: Shortest, tiny: number, huge: number): FmtResult {
	const precision = f.spec.precision;
	return write(x, f, (m) => {
		if (precision !== undefined) return exactFixed(m, precision, shortest === shortest32);
		return (m !== 0 && m < tiny) || m >= huge ? scientific(m, undefined, "e", shortest) : shortestFixed(m, 1, shortest);
	});
}

/** `f64`'s `Display`. */
export const f64Display = (x: f64, f: Formatter): FmtResult => display(x, f, shortest);

/** `f64`'s `Debug`. */
export const f64Debug = (x: f64, f: Formatter): FmtResult => debug(x, f, shortest, 1e-4, 1e16);

/** `f64`'s `LowerExp`. */
export const f64LowerExp = (x: f64, f: Formatter): FmtResult => write(x, f, (m) => scientific(m, f.spec.precision, "e", shortest));

/** `f64`'s `UpperExp`. */
export const f64UpperExp = (x: f64, f: Formatter): FmtResult => write(x, f, (m) => scientific(m, f.spec.precision, "E", shortest));

/** `f32`'s `Display`. */
export const f32Display = (x: f32, f: Formatter): FmtResult => display(x, f, shortest32);

/** `f32`'s `Debug`, whose bounds are 1e-4 and 1e16 rounded to singles. */
export const f32Debug = (x: f32, f: Formatter): FmtResult => debug(x, f, shortest32, 0.00009999999747378752, 10000000272564224);

/** `f32`'s `LowerExp`. */
export const f32LowerExp = (x: f32, f: Formatter): FmtResult => write(x, f, (m) => scientific(m, f.spec.precision, "e", shortest32));

/** `f32`'s `UpperExp`. */
export const f32UpperExp = (x: f32, f: Formatter): FmtResult => write(x, f, (m) => scientific(m, f.spec.precision, "E", shortest32));

// Parsing, as `core::num::dec2flt` does it.

/** `core::num::ParseFloatError`: whether the string was empty or not a float. */
export type ParseFloatError = { readonly kind: FloatErrorKind };

/** `core::num::FloatErrorKind`: why a float didn't parse, by variant name. */
export type FloatErrorKind = "Empty" | "Invalid";

/** Every power of ten a double holds exactly. */
const POWERS = [1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22];

/**
 * A binary float type: its significand's bits, the implicit one's included, and its
 * exponent's bias; the most digits and the greatest power of ten Clinger's fast path
 * takes exactly; and the digits and exponent past which a decimal is at least 10^`over`
 * or less than 10^`under`, both of which round past the type's range.
 */
type Binary = { readonly bits: number; readonly bias: number; readonly digits: number; readonly powers: number; readonly over: number; readonly under: number };

const F64: Binary = { bits: 53, bias: 1023, digits: 15, powers: 22, over: 310, under: -324 };
const F32: Binary = { bits: 24, bias: 127, digits: 7, powers: 10, over: 40, under: -45 };

/** `str::parse::<f64>`: std's grammar, without JS's whitespace, hex, or `Infinity`, rounded to the nearest double, half to even. */
export const f64FromStr = (s: string): Result<f64, ParseFloatError> => fromStr(s, F64);

/** `str::parse::<f32>`: rounded once to the nearest single, never through a double. */
export const f32FromStr = (s: string): Result<f32, ParseFloatError> => fromStr(s, F32);

function fromStr(s: string, binary: Binary): Result<number, ParseFloatError> {
	if (s === "") return Err({ kind: "Empty" });
	const signed = s[0] === "-" || s[0] === "+";
	const rest = signed ? s.slice(1) : s;
	const number = /^([0-9]*)(?:\.([0-9]*))?(?:[eE]([+-]?)([0-9]+))?$/.exec(rest);
	let x: number;
	if (number !== null && number[1].length + (number[2]?.length ?? 0) > 0) {
		const [, int, frac = "", sign, digits = ""] = number;
		// std stops adding exponent digits once past 0x10000.
		let exp = 0;
		for (let i = 0; i < digits.length; i++) if (exp < 0x10000) exp = exp * 10 + digits.charCodeAt(i) - 48;
		x = nearest(int + frac, (sign === "-" ? -exp : exp) - frac.length, binary);
	} else if (/^(?:inf|infinity)$/i.test(rest)) {
		x = Infinity;
	} else if (/^nan$/i.test(rest)) {
		x = NaN;
	} else {
		return Err({ kind: "Invalid" });
	}
	return Ok(s[0] === "-" ? -x : x);
}

/** The `binary` float nearest `digits * 10^exp`, half to even. */
function nearest(digits: string, exp: number, binary: Binary): number {
	const last = end(digits, "0");
	const trimmed = digits.slice(0, last).replace(/^0+/, "");
	if (trimmed === "") return 0;
	exp += digits.length - last;
	const n = trimmed.length;
	// Both operands exact in the type, so the one operation rounds once, as Clinger's fast
	// path does. A double's quotient or product of two singles, rounded to one, is the
	// single nearest the exact value.
	if (n <= binary.digits && Math.abs(exp) <= binary.powers) {
		const x = exp < 0 ? Number(trimmed) / POWERS[-exp] : Number(trimmed) * POWERS[exp];
		return binary === F32 ? Math.fround(x) : x;
	}
	if (n + exp > binary.over) return Infinity;
	if (n + exp < binary.under) return 0;
	const big = BigInt(trimmed);
	return exp < 0 ? quotient(big, 10n ** BigInt(-exp), binary) : quotient(big * 10n ** BigInt(exp), 1n, binary);
}

/** The `binary` float nearest `num / den`, half to even, built from its bits so nothing rounds twice. */
function quotient(num: bigint, den: bigint, binary: Binary): number {
	const { bits, bias } = binary;
	const implicit = 1n << BigInt(bits - 1);
	// The power of two that leaves a full significand, or the subnormals' one.
	let e = Math.max(num.toString(2).length - den.toString(2).length - bits, 2 - bias - bits);
	const divide = (): [bigint, bigint, bigint] => {
		const [n, d] = e < 0 ? [num << BigInt(-e), den] : [num, den << BigInt(e)];
		return [n / d, n % d, d];
	};
	let [q, r, d] = divide();
	if (q >= implicit << 1n) {
		e++;
		[q, r, d] = divide();
	}
	if (2n * r > d || (2n * r === d && (q & 1n) === 1n)) q++;
	if (q === implicit << 1n) {
		q = implicit;
		e++;
	}
	const biased = e + bias + bits - 1;
	if (biased >= 2 * bias + 1) return Infinity;
	const field = q < implicit ? q : (BigInt(biased) << BigInt(bits - 1)) | (q - implicit);
	const view = bytes();
	if (binary === F32) {
		view.setUint32(0, Number(field));
		return view.getFloat32(0);
	}
	view.setBigUint64(0, field);
	return view.getFloat64(0);
}
