// Integers, for rust2-ts output: each operation of each type is its own function,
// `u8Add` or `u64Shr`, so a bundler drops every one translated code never calls.
// `rs.ts` re-exports all of it.
//
// Integers are numbers, but a 64- or 128-bit one past 2^53 is an immutable object of
// 32-bit limbs. Arithmetic is checked as a debug build checks it: overflow panics, it
// never wraps silently. Every panic's message is the one native Rust panics with.

import { Err, type Formatter, type FmtResult, fmtOwned, fmtPadFormatted, fmtPadIntegral, Ok, panic, panicFmt, type Result } from "../rs.ts";
import type { Location } from "./location.ts";

declare const brand: unique symbol;

/** A `u64` past 2^53: its high and low 32 bits, each a uint32. */
export type U64Limbs = { readonly hi: number; readonly lo: number; readonly [brand]: "u64" };
/** An `i64` past 2^53 in magnitude: its high 32 bits as an int32, and its low as a uint32. */
export type I64Limbs = { readonly hi: number; readonly lo: number; readonly [brand]: "i64" };
/** A `u128` past 2^53: four 32-bit limbs, `hi`, `w2`, `w1`, and `lo`, each a uint32. */
export type U128Limbs = { readonly hi: number; readonly w2: number; readonly w1: number; readonly lo: number; readonly [brand]: "u128" };
/** An `i128` past 2^53 in magnitude: as a `u128`'s, but `hi` is an int32. */
export type I128Limbs = { readonly hi: number; readonly w2: number; readonly w1: number; readonly lo: number; readonly [brand]: "i128" };
/** Limbs, whose value is theirs with `hi` read in the type's sign. */
export type Limbs = U64Limbs | I64Limbs | U128Limbs | I128Limbs;

/**
 * A number of the Rust integer type `T`. Its brand is type-only, so `tsc` refuses one
 * type's value where another's goes, while a plain number, such as a literal, passes
 * for any.
 */
type Num<T extends string> = number & { readonly [brand]?: T };

/** Rust's `u8`. */
export type u8 = Num<"u8">;
/** Rust's `u16`. */
export type u16 = Num<"u16">;
/** Rust's `u32`. */
export type u32 = Num<"u32">;
/** 32 bits, as on wasm32. */
export type usize = Num<"usize">;
/** Rust's `i8`. */
export type i8 = Num<"i8">;
/** Rust's `i16`. */
export type i16 = Num<"i16">;
/** Rust's `i32`. */
export type i32 = Num<"i32">;
/** 32 bits, as on wasm32. */
export type isize = Num<"isize">;
/** Rust's `f64`: a number, which is the same IEEE 754 double. */
export type f64 = Num<"f64">;
/** Rust's `f32`: a number that is exactly an IEEE 754 single, as `Math.fround` makes one. */
export type f32 = Num<"f32">;
/** Rust's `char`: a Unicode scalar value, which is at most 0x10ffff and no surrogate. */
export type char = Num<"char">;
/**
 * A `u64`: a number while it is a safe integer, at most 2^53 - 1, and its limbs past
 * that. Each value has one form, so two numbers compare with `===`, and a number never
 * equals limbs.
 */
export type u64 = Num<"u64"> | U64Limbs;
/** An `i64`: a number while it is a safe integer, and its limbs past that. */
export type i64 = Num<"i64"> | I64Limbs;
/** A `u128`: a number while it is a safe integer, and its limbs past that. */
export type u128 = Num<"u128"> | U128Limbs;
/** An `i128`: a number while it is a safe integer, and its limbs past that. */
export type i128 = Num<"i128"> | I128Limbs;

/**
 * How a range of one integer type steps: `n + by`, wrapped into the type, and whether it
 * wrapped. Ranges of wider types take theirs to step past 2^53.
 */
export type Step<T> = (n: T, by: number) => [T, boolean];

/**
 * `NonZero<T>`'s `Step` of an unsigned `T`'s, `int`: within 1 and `max`, past which it
 * stops at that end where `T`'s wraps.
 */
export function nonZeroStep<T>(int: Step<T>, max: T): Step<T> {
	return (n, by) => {
		const [v, over] = int(n, by);
		if (over || v === 0) return [by > 0 ? max : (1 as T), true];
		return [v, false];
	};
}

/** `core::num::TryFromIntError`: which way the value was out of range. */
export type TryFromIntError = { readonly kind: "PosOverflow" | "NegOverflow" };

/** `core::num::IntErrorKind`: why an integer didn't parse, by variant name. */
export type IntErrorKind = "Empty" | "InvalidDigit" | "PosOverflow" | "NegOverflow" | "Zero" | "NotAPowerOfTwo";

/** `core::num::ParseIntError`: why an integer didn't parse. */
export type ParseIntError = { readonly kind: IntErrorKind };

/**
 * `char`'s `Step`: `n` moved by `by` past the surrogates, which no `char` is, and whether
 * that left the chars, past which it stays at the last or first, as std's does.
 */
export const charStep: Step<char> = (n, by) => {
	let v = n + by;
	if (by > 0 && n < 0xd800 && v >= 0xd800) v += 0x800;
	else if (by < 0 && n >= 0xe000 && v < 0xe000) v -= 0x800;
	if (v > 0x10ffff) return [0x10ffff as char, true];
	if (v < 0) return [0 as char, true];
	return [v as char, false];
};

/** `core::char::CharErrorKind`: why a `char` didn't parse, by variant name. */
export type CharErrorKind = "EmptyString" | "TooManyChars";

/** `core::char::ParseCharError`: why a `char` didn't parse. */
export type ParseCharError = { readonly kind: CharErrorKind };

/** `char`'s `FromStr`: the string's only char, or why it holds none or more. */
export function charFromStr(s: string): Result<char, ParseCharError> {
	if (s === "") return Err({ kind: "EmptyString" });
	const c = s.codePointAt(0) as char;
	return s.length === (c > 0xffff ? 2 : 1) ? Ok(c) : Err({ kind: "TooManyChars" });
}

const TWO32 = 4294967296;
const MAX_SAFE = 0x1fffffffffffff;

/** A wide integer's value as a number: exact up to 2^53 in magnitude, rounded past it. */
function approx(v: Limbs): number {
	return "w1" in v ? ((v.hi * TWO32 + v.w2) * TWO32 + v.w1) * TWO32 + v.lo : v.hi * TWO32 + v.lo;
}

/** A wide integer's exact value. */
function exact(v: Limbs): bigint {
	if ("w1" in v) return (((((BigInt(v.hi) << 32n) + BigInt(v.w2)) << 32n) + BigInt(v.w1)) << 32n) + BigInt(v.lo);
	return (BigInt(v.hi) << 32n) + BigInt(v.lo);
}

function safe(n: number): boolean {
	return n <= MAX_SAFE && n >= -MAX_SAFE;
}

function shift(b: number | Limbs, bits: number, direction: string, at?: Location): number {
	const n = typeof b === "number" ? b : approx(b);
	if (n < 0 || n >= bits) panic(`attempt to shift ${direction} with overflow`, at);
	return n;
}

/**
 * `a < b` of two integers of one type: numbers, or a wide type's limbs. Each value has one
 * form, so limbs lie past every number, below them if negative and above them if not.
 */
export function lt(a: number | Limbs, b: number | Limbs): boolean {
	if (typeof a === "number") return typeof b === "number" ? a < b : b.hi >= 0;
	return typeof b === "number" ? a.hi < 0 : compare(a, b) < 0;
}

/** `PartialOrd::le` of integers, by `lt`. */
export function le(a: number | Limbs, b: number | Limbs): boolean {
	return !lt(b, a);
}

/** `PartialOrd::gt` of integers, by `lt`. */
export function gt(a: number | Limbs, b: number | Limbs): boolean {
	return lt(b, a);
}

/** `PartialOrd::ge` of integers, by `lt`. */
export function ge(a: number | Limbs, b: number | Limbs): boolean {
	return !lt(a, b);
}

/** The larger of two integers. */
export function max(a: number, b: number): number {
	return a >= b ? a : b;
}

/** The smaller of two integers. */
export function min(a: number, b: number): number {
	return a <= b ? a : b;
}

/** `Ord::clamp` of integers in numbers: `a` within `[min, max]`, panicking unless `min <= max`. */
export function clamp(a: number, min: number, max: number, at?: Location): number {
	if (min > max) panicFmt(["min > max. min = ", fmtOwned(min, display), ", max = ", fmtOwned(max, display)], at);
	return a < min ? min : a > max ? max : a;
}

// Integers of 32 bits or less, in numbers. Sums, differences, and quotients are exact; a
// product is inexact only past 2^53, far outside every type's range, so its check still
// fails. JS's 32-bit operators do the rest. Each type's functions pass its bounds to
// these as literals, which engines inline.

function checked(value: number, min: number, max: number, op: string, at?: Location): number {
	if (value < min || value > max) panic(`attempt to ${op} with overflow`, at);
	return value;
}

function inRange(value: number, min: number, max: number): number | undefined {
	return value >= min && value <= max ? value : undefined;
}

function saturate(value: number, min: number, max: number): number {
	return value < min ? min : value > max ? max : value;
}

/** An unsigned `a / b`, rounded up, panicking on a zero divisor. */
function ceilDivide(a: number, b: number, at?: Location): number {
	if (b === 0) panic("attempt to divide by zero", at);
	return a % b === 0 ? a / b : Math.trunc(a / b) + 1;
}

/** Whether `a` is a multiple of `b`: of 0, only 0 is. */
function multipleOf(a: number, b: number): boolean {
	return b === 0 ? a === 0 : a % b === 0;
}

function divide(a: number, b: number, min: number, max: number, at?: Location): number {
	if (b === 0) panic("attempt to divide by zero", at);
	return checked(Math.trunc(a / b), min, max, "divide", at);
}

function remainder(a: number, b: number, min: number, at?: Location): number {
	if (b === 0) panic("attempt to calculate the remainder with a divisor of zero", at);
	if (b === -1 && a === min && min < 0) panic("attempt to calculate the remainder with overflow", at);
	return a % b;
}

/** Big-endian bytes, as an int32 of the last four. */
function beWord(bytes: Uint8Array): number {
	let value = 0;
	for (let i = 0; i < bytes.length; i++) value = (value << 8) | bytes[i];
	return value;
}

/** The big-endian 32-bit word of `b` at `i`, a uint32. */
function word(b: Uint8Array, i: number): number {
	return ((b[i] << 24) | (b[i + 1] << 16) | (b[i + 2] << 8) | b[i + 3]) >>> 0;
}

function beBytes(a: number, n: number): Uint8Array {
	const out = new Uint8Array(n);
	for (let i = n - 1; i >= 0; i--) {
		out[i] = a;
		a >>= 8;
	}
	return out;
}

function narrowTryFrom(value: number | Limbs, min: number, max: number): Result<number, TryFromIntError> {
	// Rounding past 2^53 keeps the order, so the bounds still test exactly.
	const n = typeof value === "number" ? value : approx(value);
	if (n >= min && n <= max) return Ok(n);
	return Err({ kind: n < min ? "NegOverflow" : "PosOverflow" });
}

/** `n + by` within `[min, max]`, wrapped by `wrap` past them; `by` may be past the type's range. */
function narrowStep(n: number, by: number, min: number, max: number, wrap: (v: number) => number): [number, boolean] {
	const v = n + by;
	return v >= min && v <= max ? [v, false] : [wrap(v), true];
}

/** `Display` of an integer in a number. */
function display(a: number, f: Formatter): FmtResult {
	return fmtPadIntegral(f, a >= 0, "", String(a < 0 ? -a : a));
}

/** `Debug` of `a`, an integer in a number whose bits as unsigned are `bits`. */
function debug(a: number, bits: number, f: Formatter): FmtResult {
	const hexDigits = f.spec.debugHex;
	return hexDigits === undefined ? display(a, f) : hex(bits, f, hexDigits === "X");
}

/** `LowerHex` or `UpperHex` of `bits`, a value's bits as unsigned. */
function hex(bits: number | bigint, f: Formatter, upper: boolean): FmtResult {
	const digits = bits.toString(16);
	return fmtPadIntegral(f, true, "0x", upper ? digits.toUpperCase() : digits);
}

/** `Binary` or `Octal` of `bits`, a value's bits as unsigned. */
function radix(bits: number | bigint, f: Formatter, base: 2 | 8, prefix: "0b" | "0o"): FmtResult {
	return fmtPadIntegral(f, true, prefix, bits.toString(base));
}

/**
 * `LowerExp` or `UpperExp` of an integer, from its sign and its magnitude's decimal
 * `digits`: rounded half to even to the precision, else without trailing zeros.
 */
function exp(nonnegative: boolean, digits: string, f: Formatter, letter: "e" | "E"): FmtResult {
	let exponent = digits.length - 1;
	// The significant digits, the first before the point, and the zeros after them.
	let coef = digits;
	let zeros = 0;
	const precision = f.spec.precision;
	if (precision === undefined) {
		coef = digits.replace(/(?<=.)0+$/, "");
	} else if (precision >= exponent) {
		zeros = precision - exponent;
	} else {
		coef = digits.slice(0, precision + 1);
		const over = digits.slice(precision + 1);
		const tie = over[0] === "5" && !/[1-9]/.test(over.slice(1));
		const up = tie ? Number(coef[coef.length - 1]) % 2 === 1 : over[0]! >= "5";
		if (up) coef = String(BigInt(coef) + 1n);
		// Rounding 9s up carries into one more digit: a power of ten higher.
		if (coef.length > precision + 1) {
			coef = coef.slice(0, -1);
			exponent++;
		}
	}
	const point = coef.length > 1 || zeros > 0 ? `.${coef.slice(1)}` : "";
	if (zeros === 0) return fmtPadIntegral(f, nonnegative, "", `${coef[0]}${point}${letter}${exponent}`);
	// The precision's zeros past the digits are parts of their own, as are the sign's.
	const sign = nonnegative ? (f.spec.plus ? "+" : "") : "-";
	return fmtPadFormatted(f, sign, [`${coef[0]}${point}`, zeros, `${letter}${exponent}`]);
}

const wrapU8 = (v: number) => v & 0xff;
const wrapU16 = (v: number) => v & 0xffff;
const wrapU32 = (v: number) => v >>> 0;
const wrapI8 = (v: number) => (v << 24) >> 24;
const wrapI16 = (v: number) => (v << 16) >> 16;
const wrapI32 = (v: number) => v | 0;

/** `u8`'s `a + b`, panicking on overflow. */
export const u8Add = (a: u8, b: u8, at?: Location): u8 => checked(a + b, 0, 0xff, "add", at);
/** `u8`'s `a - b`, panicking on overflow. */
export const u8Sub = (a: u8, b: u8, at?: Location): u8 => checked(a - b, 0, 0xff, "subtract", at);
/** `u8`'s `a * b`, panicking on overflow. */
export const u8Mul = (a: u8, b: u8, at?: Location): u8 => checked(a * b, 0, 0xff, "multiply", at);
/** `u8`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const u8Div = (a: u8, b: u8, at?: Location): u8 => divide(a, b, 0, 0xff, at);
/** `u8`'s `a % b`, panicking on a zero divisor or overflow. */
export const u8Rem = (a: u8, b: u8, at?: Location): u8 => remainder(a, b, 0, at);
/** `u8`'s `a << b`, panicking on a shift by the width or more. */
export const u8Shl = (a: u8, b: number | Limbs, at?: Location): u8 => (a << shift(b, 8, "left", at)) & 0xff;
/** `u8`'s `a >> b`, panicking on a shift by the width or more. */
export const u8Shr = (a: u8, b: number | Limbs, at?: Location): u8 => a >>> shift(b, 8, "right", at);
/** `u8`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const u8CheckedAdd = (a: u8, b: u8): u8 | undefined => inRange(a + b, 0, 0xff);
/** `u8`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const u8CheckedSub = (a: u8, b: u8): u8 | undefined => inRange(a - b, 0, 0xff);
/** `u8`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const u8CheckedMul = (a: u8, b: u8): u8 | undefined => inRange(a * b, 0, 0xff);
/** `u8`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const u8SaturatingAdd = (a: u8, b: u8): u8 => saturate(a + b, 0, 0xff);
/** `u8`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const u8SaturatingSub = (a: u8, b: u8): u8 => saturate(a - b, 0, 0xff);
/** `u8`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const u8WrappingAdd = (a: u8, b: u8): u8 => (a + b) & 0xff;
/** `u8`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const u8WrappingSub = (a: u8, b: u8): u8 => (a - b) & 0xff;
/** `u8`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const u8SaturatingMul = (a: u8, b: u8): u8 => saturate(a * b, 0, 0xff);
/** `u8`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const u8WrappingMul = (a: u8, b: u8): u8 => Math.imul(a, b) & 0xff;
/** `u8`'s `div_ceil`: `a / b`, rounded up, panicking on a zero divisor. */
export const u8DivCeil = (a: u8, b: u8, at?: Location): u8 => ceilDivide(a, b, at);
/** `u8`'s `is_multiple_of`: whether `b` divides `a`; of 0, only 0 is a multiple. */
export const u8IsMultipleOf = (a: u8, b: u8): boolean => multipleOf(a, b);
/** `u8`'s `leading_ones`: the count of leading one bits. */
export const u8LeadingOnes = (a: u8): number => Math.clz32(~(a << 24));
/** `u8`'s `leading_zeros`: the count of leading zero bits. */
export const u8LeadingZeros = (a: u8): number => Math.clz32(a) - 24;
/** `u8`'s `trailing_zeros`: the count of trailing zero bits. */
export const u8TrailingZeros = (a: u8): number => trailingZeros32(a, 8);
/** `u8`'s `from_be_bytes`: a value from its big-endian bytes. */
export const u8FromBeBytes = (bytes: Uint8Array): u8 => beWord(bytes) & 0xff;
/** `u8`'s `to_be_bytes`: its big-endian bytes. */
export const u8ToBeBytes = (a: u8): Uint8Array => beBytes(a, 1);
/** `u8`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const u8TryFrom = (value: number | Limbs): Result<u8, TryFromIntError> => narrowTryFrom(value, 0, 0xff);
/** `u8`'s `LowerHex`, `{:x}`. */
export const u8LowerHex = (a: u8, f: Formatter): FmtResult => hex(a, f, false);
/** `u8`'s `UpperHex`, `{:X}`. */
export const u8UpperHex = (a: u8, f: Formatter): FmtResult => hex(a, f, true);
/** `u8`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const u8Debug = (a: u8, f: Formatter): FmtResult => debug(a, a, f);
/** `u8`'s `Binary`, `{:b}`. */
export const u8Binary = (a: u8, f: Formatter): FmtResult => radix(a, f, 2, "0b");
/** `u8`'s `Octal`, `{:o}`. */
export const u8Octal = (a: u8, f: Formatter): FmtResult => radix(a, f, 8, "0o");
/** `u8`'s `LowerExp`, `{:e}`. */
export const u8LowerExp = (a: u8, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "e");
/** `u8`'s `UpperExp`, `{:E}`. */
export const u8UpperExp = (a: u8, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "E");
/** `u8`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const u8Step: Step<u8> = (n, by) => narrowStep(n, by, 0, 0xff, wrapU8);

/** `u16`'s `a + b`, panicking on overflow. */
export const u16Add = (a: u16, b: u16, at?: Location): u16 => checked(a + b, 0, 0xffff, "add", at);
/** `u16`'s `a - b`, panicking on overflow. */
export const u16Sub = (a: u16, b: u16, at?: Location): u16 => checked(a - b, 0, 0xffff, "subtract", at);
/** `u16`'s `a * b`, panicking on overflow. */
export const u16Mul = (a: u16, b: u16, at?: Location): u16 => checked(a * b, 0, 0xffff, "multiply", at);
/** `u16`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const u16Div = (a: u16, b: u16, at?: Location): u16 => divide(a, b, 0, 0xffff, at);
/** `u16`'s `a % b`, panicking on a zero divisor or overflow. */
export const u16Rem = (a: u16, b: u16, at?: Location): u16 => remainder(a, b, 0, at);
/** `u16`'s `a << b`, panicking on a shift by the width or more. */
export const u16Shl = (a: u16, b: number | Limbs, at?: Location): u16 => (a << shift(b, 16, "left", at)) & 0xffff;
/** `u16`'s `a >> b`, panicking on a shift by the width or more. */
export const u16Shr = (a: u16, b: number | Limbs, at?: Location): u16 => a >>> shift(b, 16, "right", at);
/** `u16`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const u16CheckedAdd = (a: u16, b: u16): u16 | undefined => inRange(a + b, 0, 0xffff);
/** `u16`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const u16CheckedSub = (a: u16, b: u16): u16 | undefined => inRange(a - b, 0, 0xffff);
/** `u16`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const u16CheckedMul = (a: u16, b: u16): u16 | undefined => inRange(a * b, 0, 0xffff);
/** `u16`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const u16SaturatingAdd = (a: u16, b: u16): u16 => saturate(a + b, 0, 0xffff);
/** `u16`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const u16SaturatingSub = (a: u16, b: u16): u16 => saturate(a - b, 0, 0xffff);
/** `u16`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const u16WrappingAdd = (a: u16, b: u16): u16 => (a + b) & 0xffff;
/** `u16`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const u16WrappingSub = (a: u16, b: u16): u16 => (a - b) & 0xffff;
/** `u16`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const u16SaturatingMul = (a: u16, b: u16): u16 => saturate(a * b, 0, 0xffff);
/** `u16`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const u16WrappingMul = (a: u16, b: u16): u16 => Math.imul(a, b) & 0xffff;
/** `u16`'s `div_ceil`: `a / b`, rounded up, panicking on a zero divisor. */
export const u16DivCeil = (a: u16, b: u16, at?: Location): u16 => ceilDivide(a, b, at);
/** `u16`'s `is_multiple_of`: whether `b` divides `a`; of 0, only 0 is a multiple. */
export const u16IsMultipleOf = (a: u16, b: u16): boolean => multipleOf(a, b);
/** `u16`'s `leading_ones`: the count of leading one bits. */
export const u16LeadingOnes = (a: u16): number => Math.clz32(~(a << 16));
/** `u16`'s `leading_zeros`: the count of leading zero bits. */
export const u16LeadingZeros = (a: u16): number => Math.clz32(a) - 16;
/** `u16`'s `trailing_zeros`: the count of trailing zero bits. */
export const u16TrailingZeros = (a: u16): number => trailingZeros32(a, 16);
/** `u16`'s `from_be_bytes`: a value from its big-endian bytes. */
export const u16FromBeBytes = (bytes: Uint8Array): u16 => beWord(bytes) & 0xffff;
/** `u16`'s `to_be_bytes`: its big-endian bytes. */
export const u16ToBeBytes = (a: u16): Uint8Array => beBytes(a, 2);
/** `u16`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const u16TryFrom = (value: number | Limbs): Result<u16, TryFromIntError> => narrowTryFrom(value, 0, 0xffff);
/** `u16`'s `LowerHex`, `{:x}`. */
export const u16LowerHex = (a: u16, f: Formatter): FmtResult => hex(a, f, false);
/** `u16`'s `UpperHex`, `{:X}`. */
export const u16UpperHex = (a: u16, f: Formatter): FmtResult => hex(a, f, true);
/** `u16`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const u16Debug = (a: u16, f: Formatter): FmtResult => debug(a, a, f);
/** `u16`'s `Binary`, `{:b}`. */
export const u16Binary = (a: u16, f: Formatter): FmtResult => radix(a, f, 2, "0b");
/** `u16`'s `Octal`, `{:o}`. */
export const u16Octal = (a: u16, f: Formatter): FmtResult => radix(a, f, 8, "0o");
/** `u16`'s `LowerExp`, `{:e}`. */
export const u16LowerExp = (a: u16, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "e");
/** `u16`'s `UpperExp`, `{:E}`. */
export const u16UpperExp = (a: u16, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "E");
/** `u16`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const u16Step: Step<u16> = (n, by) => narrowStep(n, by, 0, 0xffff, wrapU16);

/** `u32`'s `a + b`, panicking on overflow. */
export const u32Add = (a: u32, b: u32, at?: Location): u32 => checked(a + b, 0, 0xffffffff, "add", at);
/** `u32`'s `a - b`, panicking on overflow. */
export const u32Sub = (a: u32, b: u32, at?: Location): u32 => checked(a - b, 0, 0xffffffff, "subtract", at);
/** `u32`'s `a * b`, panicking on overflow. */
export const u32Mul = (a: u32, b: u32, at?: Location): u32 => checked(a * b, 0, 0xffffffff, "multiply", at);
/** `u32`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const u32Div = (a: u32, b: u32, at?: Location): u32 => divide(a, b, 0, 0xffffffff, at);
/** `u32`'s `a % b`, panicking on a zero divisor or overflow. */
export const u32Rem = (a: u32, b: u32, at?: Location): u32 => remainder(a, b, 0, at);
/** `u32`'s `a << b`, panicking on a shift by the width or more. */
export const u32Shl = (a: u32, b: number | Limbs, at?: Location): u32 => (a << shift(b, 32, "left", at)) >>> 0;
/** `u32`'s `a >> b`, panicking on a shift by the width or more. */
export const u32Shr = (a: u32, b: number | Limbs, at?: Location): u32 => a >>> shift(b, 32, "right", at);
/** `u32`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const u32CheckedAdd = (a: u32, b: u32): u32 | undefined => inRange(a + b, 0, 0xffffffff);
/** `u32`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const u32CheckedSub = (a: u32, b: u32): u32 | undefined => inRange(a - b, 0, 0xffffffff);
/** `u32`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const u32CheckedMul = (a: u32, b: u32): u32 | undefined => inRange(a * b, 0, 0xffffffff);
/** `u32`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const u32SaturatingAdd = (a: u32, b: u32): u32 => saturate(a + b, 0, 0xffffffff);
/** `u32`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const u32SaturatingSub = (a: u32, b: u32): u32 => saturate(a - b, 0, 0xffffffff);
/** `u32`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const u32WrappingAdd = (a: u32, b: u32): u32 => (a + b) >>> 0;
/** `u32`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const u32WrappingSub = (a: u32, b: u32): u32 => (a - b) >>> 0;
/** `u32`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const u32SaturatingMul = (a: u32, b: u32): u32 => saturate(a * b, 0, 0xffffffff);
/** `u32`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const u32WrappingMul = (a: u32, b: u32): u32 => Math.imul(a, b) >>> 0;
/** `u32`'s `div_ceil`: `a / b`, rounded up, panicking on a zero divisor. */
export const u32DivCeil = (a: u32, b: u32, at?: Location): u32 => ceilDivide(a, b, at);
/** `u32`'s `is_multiple_of`: whether `b` divides `a`; of 0, only 0 is a multiple. */
export const u32IsMultipleOf = (a: u32, b: u32): boolean => multipleOf(a, b);
/** `u32`'s `leading_ones`: the count of leading one bits. */
export const u32LeadingOnes = (a: u32): number => Math.clz32(~a);
/** `u32`'s `leading_zeros`: the count of leading zero bits. */
export const u32LeadingZeros = (a: u32): number => Math.clz32(a);
/** `u32`'s `trailing_zeros`: the count of trailing zero bits. */
export const u32TrailingZeros = (a: u32): number => trailingZeros32(a, 32);
/** `u32`'s `from_be_bytes`: a value from its big-endian bytes. */
export const u32FromBeBytes = (bytes: Uint8Array): u32 => word(bytes, 0);
/** `u32`'s `to_be_bytes`: its big-endian bytes. */
export const u32ToBeBytes = (a: u32): Uint8Array => beBytes(a, 4);
/** `u32`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const u32TryFrom = (value: number | Limbs): Result<u32, TryFromIntError> => narrowTryFrom(value, 0, 0xffffffff);
/** `u32`'s `LowerHex`, `{:x}`. */
export const u32LowerHex = (a: u32, f: Formatter): FmtResult => hex(a, f, false);
/** `u32`'s `UpperHex`, `{:X}`. */
export const u32UpperHex = (a: u32, f: Formatter): FmtResult => hex(a, f, true);
/** `u32`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const u32Debug = (a: u32, f: Formatter): FmtResult => debug(a, a, f);
/** `u32`'s `Binary`, `{:b}`. */
export const u32Binary = (a: u32, f: Formatter): FmtResult => radix(a, f, 2, "0b");
/** `u32`'s `Octal`, `{:o}`. */
export const u32Octal = (a: u32, f: Formatter): FmtResult => radix(a, f, 8, "0o");
/** `u32`'s `LowerExp`, `{:e}`. */
export const u32LowerExp = (a: u32, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "e");
/** `u32`'s `UpperExp`, `{:E}`. */
export const u32UpperExp = (a: u32, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "E");
/** `u32`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const u32Step: Step<u32> = (n, by) => narrowStep(n, by, 0, 0xffffffff, wrapU32);

/** `i8`'s `a + b`, panicking on overflow. */
export const i8Add = (a: i8, b: i8, at?: Location): i8 => checked(a + b, -0x80, 0x7f, "add", at);
/** `i8`'s `a - b`, panicking on overflow. */
export const i8Sub = (a: i8, b: i8, at?: Location): i8 => checked(a - b, -0x80, 0x7f, "subtract", at);
/** `i8`'s `a * b`, panicking on overflow. */
export const i8Mul = (a: i8, b: i8, at?: Location): i8 => checked(a * b, -0x80, 0x7f, "multiply", at);
/** `i8`'s `-a`, panicking on overflow. */
export const i8Neg = (a: i8, at?: Location): i8 => checked(-a, -0x80, 0x7f, "negate", at);
/** `i8`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const i8Div = (a: i8, b: i8, at?: Location): i8 => divide(a, b, -0x80, 0x7f, at);
/** `i8`'s `a % b`, panicking on a zero divisor or overflow. */
export const i8Rem = (a: i8, b: i8, at?: Location): i8 => remainder(a, b, -0x80, at);
/** `i8`'s `a << b`, panicking on a shift by the width or more. */
export const i8Shl = (a: i8, b: number | Limbs, at?: Location): i8 => ((a << shift(b, 8, "left", at)) << 24) >> 24;
/** `i8`'s `a >> b`, panicking on a shift by the width or more. */
export const i8Shr = (a: i8, b: number | Limbs, at?: Location): i8 => a >> shift(b, 8, "right", at);
/** `i8`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const i8CheckedAdd = (a: i8, b: i8): i8 | undefined => inRange(a + b, -0x80, 0x7f);
/** `i8`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const i8CheckedSub = (a: i8, b: i8): i8 | undefined => inRange(a - b, -0x80, 0x7f);
/** `i8`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const i8CheckedMul = (a: i8, b: i8): i8 | undefined => inRange(a * b, -0x80, 0x7f);
/** `i8`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const i8SaturatingAdd = (a: i8, b: i8): i8 => saturate(a + b, -0x80, 0x7f);
/** `i8`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const i8SaturatingSub = (a: i8, b: i8): i8 => saturate(a - b, -0x80, 0x7f);
/** `i8`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const i8WrappingAdd = (a: i8, b: i8): i8 => ((a + b) << 24) >> 24;
/** `i8`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const i8WrappingSub = (a: i8, b: i8): i8 => ((a - b) << 24) >> 24;
/** `i8`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const i8SaturatingMul = (a: i8, b: i8): i8 => saturate(a * b, -0x80, 0x7f);
/** `i8`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const i8WrappingMul = (a: i8, b: i8): i8 => (Math.imul(a, b) << 24) >> 24;
/** `i8`'s `wrapping_div`: `a / b`, truncated and wrapped into the type's range, panicking on a zero divisor. */
export const i8WrappingDiv = (a: i8, b: i8, at?: Location): i8 => (b === -1 ? i8WrappingSub(0, a) : i8Div(a, b, at));
/** `i8`'s `wrapping_rem`: `a % b`, 0 where it would overflow, panicking on a zero divisor. */
export const i8WrappingRem = (a: i8, b: i8, at?: Location): i8 => (b === -1 ? 0 : i8Rem(a, b, at));
/** `i8`'s `saturating_div`: `a / b`, truncated and clamped to the type's range, panicking on a zero divisor. */
export const i8SaturatingDiv = (a: i8, b: i8, at?: Location): i8 => (b === -1 ? i8SaturatingSub(0, a) : i8Div(a, b, at));
/** `i8`'s `leading_ones`: the count of leading one bits. */
export const i8LeadingOnes = (a: i8): number => Math.clz32(~(a << 24));
/** `i8`'s `leading_zeros`: the count of leading zero bits. */
export const i8LeadingZeros = (a: i8): number => Math.clz32(a & 0xff) - 24;
/** `i8`'s `trailing_zeros`: the count of trailing zero bits. */
export const i8TrailingZeros = (a: i8): number => trailingZeros32(a, 8);
/** `i8`'s `from_be_bytes`: a value from its big-endian bytes. */
export const i8FromBeBytes = (bytes: Uint8Array): i8 => (beWord(bytes) << 24) >> 24;
/** `i8`'s `to_be_bytes`: its big-endian bytes. */
export const i8ToBeBytes = (a: i8): Uint8Array => beBytes(a, 1);
/** `i8`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const i8TryFrom = (value: number | Limbs): Result<i8, TryFromIntError> => narrowTryFrom(value, -0x80, 0x7f);
/** `i8`'s `LowerHex`, `{:x}`. */
export const i8LowerHex = (a: i8, f: Formatter): FmtResult => hex(a & 0xff, f, false);
/** `i8`'s `UpperHex`, `{:X}`. */
export const i8UpperHex = (a: i8, f: Formatter): FmtResult => hex(a & 0xff, f, true);
/** `i8`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const i8Debug = (a: i8, f: Formatter): FmtResult => debug(a, a & 0xff, f);
/** `i8`'s `Binary`, `{:b}`. */
export const i8Binary = (a: i8, f: Formatter): FmtResult => radix(a & 0xff, f, 2, "0b");
/** `i8`'s `Octal`, `{:o}`. */
export const i8Octal = (a: i8, f: Formatter): FmtResult => radix(a & 0xff, f, 8, "0o");
/** `i8`'s `LowerExp`, `{:e}`. */
export const i8LowerExp = (a: i8, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "e");
/** `i8`'s `UpperExp`, `{:E}`. */
export const i8UpperExp = (a: i8, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "E");
/** `i8`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const i8Step: Step<i8> = (n, by) => narrowStep(n, by, -0x80, 0x7f, wrapI8);

/** `i16`'s `a + b`, panicking on overflow. */
export const i16Add = (a: i16, b: i16, at?: Location): i16 => checked(a + b, -0x8000, 0x7fff, "add", at);
/** `i16`'s `a - b`, panicking on overflow. */
export const i16Sub = (a: i16, b: i16, at?: Location): i16 => checked(a - b, -0x8000, 0x7fff, "subtract", at);
/** `i16`'s `a * b`, panicking on overflow. */
export const i16Mul = (a: i16, b: i16, at?: Location): i16 => checked(a * b, -0x8000, 0x7fff, "multiply", at);
/** `i16`'s `-a`, panicking on overflow. */
export const i16Neg = (a: i16, at?: Location): i16 => checked(-a, -0x8000, 0x7fff, "negate", at);
/** `i16`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const i16Div = (a: i16, b: i16, at?: Location): i16 => divide(a, b, -0x8000, 0x7fff, at);
/** `i16`'s `a % b`, panicking on a zero divisor or overflow. */
export const i16Rem = (a: i16, b: i16, at?: Location): i16 => remainder(a, b, -0x8000, at);
/** `i16`'s `a << b`, panicking on a shift by the width or more. */
export const i16Shl = (a: i16, b: number | Limbs, at?: Location): i16 => ((a << shift(b, 16, "left", at)) << 16) >> 16;
/** `i16`'s `a >> b`, panicking on a shift by the width or more. */
export const i16Shr = (a: i16, b: number | Limbs, at?: Location): i16 => a >> shift(b, 16, "right", at);
/** `i16`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const i16CheckedAdd = (a: i16, b: i16): i16 | undefined => inRange(a + b, -0x8000, 0x7fff);
/** `i16`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const i16CheckedSub = (a: i16, b: i16): i16 | undefined => inRange(a - b, -0x8000, 0x7fff);
/** `i16`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const i16CheckedMul = (a: i16, b: i16): i16 | undefined => inRange(a * b, -0x8000, 0x7fff);
/** `i16`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const i16SaturatingAdd = (a: i16, b: i16): i16 => saturate(a + b, -0x8000, 0x7fff);
/** `i16`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const i16SaturatingSub = (a: i16, b: i16): i16 => saturate(a - b, -0x8000, 0x7fff);
/** `i16`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const i16WrappingAdd = (a: i16, b: i16): i16 => ((a + b) << 16) >> 16;
/** `i16`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const i16WrappingSub = (a: i16, b: i16): i16 => ((a - b) << 16) >> 16;
/** `i16`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const i16SaturatingMul = (a: i16, b: i16): i16 => saturate(a * b, -0x8000, 0x7fff);
/** `i16`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const i16WrappingMul = (a: i16, b: i16): i16 => (Math.imul(a, b) << 16) >> 16;
/** `i16`'s `wrapping_div`: `a / b`, truncated and wrapped into the type's range, panicking on a zero divisor. */
export const i16WrappingDiv = (a: i16, b: i16, at?: Location): i16 => (b === -1 ? i16WrappingSub(0, a) : i16Div(a, b, at));
/** `i16`'s `wrapping_rem`: `a % b`, 0 where it would overflow, panicking on a zero divisor. */
export const i16WrappingRem = (a: i16, b: i16, at?: Location): i16 => (b === -1 ? 0 : i16Rem(a, b, at));
/** `i16`'s `saturating_div`: `a / b`, truncated and clamped to the type's range, panicking on a zero divisor. */
export const i16SaturatingDiv = (a: i16, b: i16, at?: Location): i16 => (b === -1 ? i16SaturatingSub(0, a) : i16Div(a, b, at));
/** `i16`'s `leading_ones`: the count of leading one bits. */
export const i16LeadingOnes = (a: i16): number => Math.clz32(~(a << 16));
/** `i16`'s `leading_zeros`: the count of leading zero bits. */
export const i16LeadingZeros = (a: i16): number => Math.clz32(a & 0xffff) - 16;
/** `i16`'s `trailing_zeros`: the count of trailing zero bits. */
export const i16TrailingZeros = (a: i16): number => trailingZeros32(a, 16);
/** `i16`'s `from_be_bytes`: a value from its big-endian bytes. */
export const i16FromBeBytes = (bytes: Uint8Array): i16 => (beWord(bytes) << 16) >> 16;
/** `i16`'s `to_be_bytes`: its big-endian bytes. */
export const i16ToBeBytes = (a: i16): Uint8Array => beBytes(a, 2);
/** `i16`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const i16TryFrom = (value: number | Limbs): Result<i16, TryFromIntError> => narrowTryFrom(value, -0x8000, 0x7fff);
/** `i16`'s `LowerHex`, `{:x}`. */
export const i16LowerHex = (a: i16, f: Formatter): FmtResult => hex(a & 0xffff, f, false);
/** `i16`'s `UpperHex`, `{:X}`. */
export const i16UpperHex = (a: i16, f: Formatter): FmtResult => hex(a & 0xffff, f, true);
/** `i16`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const i16Debug = (a: i16, f: Formatter): FmtResult => debug(a, a & 0xffff, f);
/** `i16`'s `Binary`, `{:b}`. */
export const i16Binary = (a: i16, f: Formatter): FmtResult => radix(a & 0xffff, f, 2, "0b");
/** `i16`'s `Octal`, `{:o}`. */
export const i16Octal = (a: i16, f: Formatter): FmtResult => radix(a & 0xffff, f, 8, "0o");
/** `i16`'s `LowerExp`, `{:e}`. */
export const i16LowerExp = (a: i16, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "e");
/** `i16`'s `UpperExp`, `{:E}`. */
export const i16UpperExp = (a: i16, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "E");
/** `i16`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const i16Step: Step<i16> = (n, by) => narrowStep(n, by, -0x8000, 0x7fff, wrapI16);

/** `i32`'s `a + b`, panicking on overflow. */
export const i32Add = (a: i32, b: i32, at?: Location): i32 => checked(a + b, -0x80000000, 0x7fffffff, "add", at);
/** `i32`'s `a - b`, panicking on overflow. */
export const i32Sub = (a: i32, b: i32, at?: Location): i32 => checked(a - b, -0x80000000, 0x7fffffff, "subtract", at);
/** `i32`'s `a * b`, panicking on overflow. */
export const i32Mul = (a: i32, b: i32, at?: Location): i32 => checked(a * b, -0x80000000, 0x7fffffff, "multiply", at);
/** `i32`'s `-a`, panicking on overflow. */
export const i32Neg = (a: i32, at?: Location): i32 => checked(-a, -0x80000000, 0x7fffffff, "negate", at);
/** `i32`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const i32Div = (a: i32, b: i32, at?: Location): i32 => divide(a, b, -0x80000000, 0x7fffffff, at);
/** `i32`'s `a % b`, panicking on a zero divisor or overflow. */
export const i32Rem = (a: i32, b: i32, at?: Location): i32 => remainder(a, b, -0x80000000, at);
/** `i32`'s `a << b`, panicking on a shift by the width or more. */
export const i32Shl = (a: i32, b: number | Limbs, at?: Location): i32 => a << shift(b, 32, "left", at);
/** `i32`'s `a >> b`, panicking on a shift by the width or more. */
export const i32Shr = (a: i32, b: number | Limbs, at?: Location): i32 => a >> shift(b, 32, "right", at);
/** `i32`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const i32CheckedAdd = (a: i32, b: i32): i32 | undefined => inRange(a + b, -0x80000000, 0x7fffffff);
/** `i32`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const i32CheckedSub = (a: i32, b: i32): i32 | undefined => inRange(a - b, -0x80000000, 0x7fffffff);
/** `i32`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const i32CheckedMul = (a: i32, b: i32): i32 | undefined => inRange(a * b, -0x80000000, 0x7fffffff);
/** `i32`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const i32SaturatingAdd = (a: i32, b: i32): i32 => saturate(a + b, -0x80000000, 0x7fffffff);
/** `i32`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const i32SaturatingSub = (a: i32, b: i32): i32 => saturate(a - b, -0x80000000, 0x7fffffff);
/** `i32`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const i32WrappingAdd = (a: i32, b: i32): i32 => (a + b) | 0;
/** `i32`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const i32WrappingSub = (a: i32, b: i32): i32 => (a - b) | 0;
/** `i32`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const i32SaturatingMul = (a: i32, b: i32): i32 => saturate(a * b, -0x80000000, 0x7fffffff);
/** `i32`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const i32WrappingMul = (a: i32, b: i32): i32 => Math.imul(a, b);
/** `i32`'s `wrapping_div`: `a / b`, truncated and wrapped into the type's range, panicking on a zero divisor. */
export const i32WrappingDiv = (a: i32, b: i32, at?: Location): i32 => (b === -1 ? i32WrappingSub(0, a) : i32Div(a, b, at));
/** `i32`'s `wrapping_rem`: `a % b`, 0 where it would overflow, panicking on a zero divisor. */
export const i32WrappingRem = (a: i32, b: i32, at?: Location): i32 => (b === -1 ? 0 : i32Rem(a, b, at));
/** `i32`'s `saturating_div`: `a / b`, truncated and clamped to the type's range, panicking on a zero divisor. */
export const i32SaturatingDiv = (a: i32, b: i32, at?: Location): i32 => (b === -1 ? i32SaturatingSub(0, a) : i32Div(a, b, at));
/** `i32`'s `leading_ones`: the count of leading one bits. */
export const i32LeadingOnes = (a: i32): number => Math.clz32(~a);
/** `i32`'s `leading_zeros`: the count of leading zero bits. */
export const i32LeadingZeros = (a: i32): number => Math.clz32(a);
/** `i32`'s `trailing_zeros`: the count of trailing zero bits. */
export const i32TrailingZeros = (a: i32): number => trailingZeros32(a, 32);
/** `i32`'s `from_be_bytes`: a value from its big-endian bytes. */
export const i32FromBeBytes = (bytes: Uint8Array): i32 => word(bytes, 0) | 0;
/** `i32`'s `to_be_bytes`: its big-endian bytes. */
export const i32ToBeBytes = (a: i32): Uint8Array => beBytes(a, 4);
/** `i32`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const i32TryFrom = (value: number | Limbs): Result<i32, TryFromIntError> => narrowTryFrom(value, -0x80000000, 0x7fffffff);
/** `i32`'s `LowerHex`, `{:x}`. */
export const i32LowerHex = (a: i32, f: Formatter): FmtResult => hex(a >>> 0, f, false);
/** `i32`'s `UpperHex`, `{:X}`. */
export const i32UpperHex = (a: i32, f: Formatter): FmtResult => hex(a >>> 0, f, true);
/** `i32`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const i32Debug = (a: i32, f: Formatter): FmtResult => debug(a, a >>> 0, f);
/** `i32`'s `Binary`, `{:b}`. */
export const i32Binary = (a: i32, f: Formatter): FmtResult => radix(a >>> 0, f, 2, "0b");
/** `i32`'s `Octal`, `{:o}`. */
export const i32Octal = (a: i32, f: Formatter): FmtResult => radix(a >>> 0, f, 8, "0o");
/** `i32`'s `LowerExp`, `{:e}`. */
export const i32LowerExp = (a: i32, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "e");
/** `i32`'s `UpperExp`, `{:E}`. */
export const i32UpperExp = (a: i32, f: Formatter): FmtResult => exp(a >= 0, String(Math.abs(a)), f, "E");
/** `i32`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const i32Step: Step<i32> = (n, by) => narrowStep(n, by, -0x80000000, 0x7fffffff, wrapI32);

// `usize` and `isize` are 32 bits, as on wasm32: `u32`'s and `i32`'s functions.
export {
	display as u8Display,
	display as u16Display,
	display as u32Display,
	display as usizeDisplay,
	display as i8Display,
	display as i16Display,
	display as i32Display,
	display as isizeDisplay,
};
/** `usize`'s `a + b`, panicking on overflow. */
export const usizeAdd: (a: usize, b: usize, at?: Location) => usize = u32Add as never;
/** `usize`'s `a - b`, panicking on overflow. */
export const usizeSub: (a: usize, b: usize, at?: Location) => usize = u32Sub as never;
/** `usize`'s `a * b`, panicking on overflow. */
export const usizeMul: (a: usize, b: usize, at?: Location) => usize = u32Mul as never;
/** `usize`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const usizeDiv: (a: usize, b: usize, at?: Location) => usize = u32Div as never;
/** `usize`'s `a % b`, panicking on a zero divisor or overflow. */
export const usizeRem: (a: usize, b: usize, at?: Location) => usize = u32Rem as never;
/** `usize`'s `a << b`, panicking on a shift by the width or more. */
export const usizeShl: (a: usize, b: number | Limbs, at?: Location) => usize = u32Shl as never;
/** `usize`'s `a >> b`, panicking on a shift by the width or more. */
export const usizeShr: (a: usize, b: number | Limbs, at?: Location) => usize = u32Shr as never;
/** `usize`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const usizeCheckedAdd: (a: usize, b: usize) => usize | undefined = u32CheckedAdd as never;
/** `usize`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const usizeCheckedSub: (a: usize, b: usize) => usize | undefined = u32CheckedSub as never;
/** `usize`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const usizeCheckedMul: (a: usize, b: usize) => usize | undefined = u32CheckedMul as never;
/** `usize`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const usizeSaturatingAdd: (a: usize, b: usize) => usize = u32SaturatingAdd as never;
/** `usize`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const usizeSaturatingSub: (a: usize, b: usize) => usize = u32SaturatingSub as never;
/** `usize`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const usizeWrappingAdd: (a: usize, b: usize) => usize = u32WrappingAdd as never;
/** `usize`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const usizeWrappingSub: (a: usize, b: usize) => usize = u32WrappingSub as never;
/** `usize`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const usizeSaturatingMul: (a: usize, b: usize) => usize = u32SaturatingMul as never;
/** `usize`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const usizeWrappingMul: (a: usize, b: usize) => usize = u32WrappingMul as never;
/** `usize`'s `div_ceil`: `a / b`, rounded up, panicking on a zero divisor. */
export const usizeDivCeil: (a: usize, b: usize, at?: Location) => usize = u32DivCeil as never;
/** `usize`'s `is_multiple_of`: whether `b` divides `a`; of 0, only 0 is a multiple. */
export const usizeIsMultipleOf: (a: usize, b: usize) => boolean = u32IsMultipleOf as never;
/** `usize`'s `leading_ones`: the count of leading one bits. */
export const usizeLeadingOnes: (a: usize) => number = u32LeadingOnes as never;
/** `usize`'s `leading_zeros`: the count of leading zero bits. */
export const usizeLeadingZeros: (a: usize) => number = u32LeadingZeros as never;
/** `usize`'s `trailing_zeros`: the count of trailing zero bits. */
export const usizeTrailingZeros: (a: usize) => number = u32TrailingZeros as never;
/** `usize`'s `from_be_bytes`: a value from its big-endian bytes. */
export const usizeFromBeBytes: (bytes: Uint8Array) => usize = u32FromBeBytes as never;
/** `usize`'s `to_be_bytes`: its big-endian bytes. */
export const usizeToBeBytes: (a: usize) => Uint8Array = u32ToBeBytes as never;
/** `usize`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const usizeTryFrom: (value: number | Limbs) => Result<usize, TryFromIntError> = u32TryFrom as never;
/** `usize`'s `LowerHex`, `{:x}`. */
export const usizeLowerHex: (a: usize, f: Formatter) => FmtResult = u32LowerHex as never;
/** `usize`'s `UpperHex`, `{:X}`. */
export const usizeUpperHex: (a: usize, f: Formatter) => FmtResult = u32UpperHex as never;
/** `usize`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const usizeDebug: (a: usize, f: Formatter) => FmtResult = u32Debug as never;
/** `usize`'s `Binary`, `{:b}`. */
export const usizeBinary: (a: usize, f: Formatter) => FmtResult = u32Binary as never;
/** `usize`'s `Octal`, `{:o}`. */
export const usizeOctal: (a: usize, f: Formatter) => FmtResult = u32Octal as never;
/** `usize`'s `LowerExp`, `{:e}`. */
export const usizeLowerExp: (a: usize, f: Formatter) => FmtResult = u32LowerExp as never;
/** `usize`'s `UpperExp`, `{:E}`. */
export const usizeUpperExp: (a: usize, f: Formatter) => FmtResult = u32UpperExp as never;
/** `usize`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const usizeStep: Step<usize> = u32Step as never;
/** `isize`'s `a + b`, panicking on overflow. */
export const isizeAdd: (a: isize, b: isize, at?: Location) => isize = i32Add as never;
/** `isize`'s `a - b`, panicking on overflow. */
export const isizeSub: (a: isize, b: isize, at?: Location) => isize = i32Sub as never;
/** `isize`'s `a * b`, panicking on overflow. */
export const isizeMul: (a: isize, b: isize, at?: Location) => isize = i32Mul as never;
/** `isize`'s `-a`, panicking on overflow. */
export const isizeNeg: (a: isize, at?: Location) => isize = i32Neg as never;
/** `isize`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const isizeDiv: (a: isize, b: isize, at?: Location) => isize = i32Div as never;
/** `isize`'s `a % b`, panicking on a zero divisor or overflow. */
export const isizeRem: (a: isize, b: isize, at?: Location) => isize = i32Rem as never;
/** `isize`'s `a << b`, panicking on a shift by the width or more. */
export const isizeShl: (a: isize, b: number | Limbs, at?: Location) => isize = i32Shl as never;
/** `isize`'s `a >> b`, panicking on a shift by the width or more. */
export const isizeShr: (a: isize, b: number | Limbs, at?: Location) => isize = i32Shr as never;
/** `isize`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const isizeCheckedAdd: (a: isize, b: isize) => isize | undefined = i32CheckedAdd as never;
/** `isize`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const isizeCheckedSub: (a: isize, b: isize) => isize | undefined = i32CheckedSub as never;
/** `isize`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const isizeCheckedMul: (a: isize, b: isize) => isize | undefined = i32CheckedMul as never;
/** `isize`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const isizeSaturatingAdd: (a: isize, b: isize) => isize = i32SaturatingAdd as never;
/** `isize`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const isizeSaturatingSub: (a: isize, b: isize) => isize = i32SaturatingSub as never;
/** `isize`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const isizeWrappingAdd: (a: isize, b: isize) => isize = i32WrappingAdd as never;
/** `isize`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const isizeWrappingSub: (a: isize, b: isize) => isize = i32WrappingSub as never;
/** `isize`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const isizeSaturatingMul: (a: isize, b: isize) => isize = i32SaturatingMul as never;
/** `isize`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const isizeWrappingMul: (a: isize, b: isize) => isize = i32WrappingMul as never;
/** `isize`'s `wrapping_div`: `a / b`, truncated and wrapped into the type's range, panicking on a zero divisor. */
export const isizeWrappingDiv: (a: isize, b: isize, at?: Location) => isize = i32WrappingDiv as never;
/** `isize`'s `wrapping_rem`: `a % b`, 0 where it would overflow, panicking on a zero divisor. */
export const isizeWrappingRem: (a: isize, b: isize, at?: Location) => isize = i32WrappingRem as never;
/** `isize`'s `saturating_div`: `a / b`, truncated and clamped to the type's range, panicking on a zero divisor. */
export const isizeSaturatingDiv: (a: isize, b: isize, at?: Location) => isize = i32SaturatingDiv as never;
/** `isize`'s `leading_ones`: the count of leading one bits. */
export const isizeLeadingOnes: (a: isize) => number = i32LeadingOnes as never;
/** `isize`'s `leading_zeros`: the count of leading zero bits. */
export const isizeLeadingZeros: (a: isize) => number = i32LeadingZeros as never;
/** `isize`'s `trailing_zeros`: the count of trailing zero bits. */
export const isizeTrailingZeros: (a: isize) => number = i32TrailingZeros as never;
/** `isize`'s `from_be_bytes`: a value from its big-endian bytes. */
export const isizeFromBeBytes: (bytes: Uint8Array) => isize = i32FromBeBytes as never;
/** `isize`'s `to_be_bytes`: its big-endian bytes. */
export const isizeToBeBytes: (a: isize) => Uint8Array = i32ToBeBytes as never;
/** `isize`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const isizeTryFrom: (value: number | Limbs) => Result<isize, TryFromIntError> = i32TryFrom as never;
/** `isize`'s `LowerHex`, `{:x}`. */
export const isizeLowerHex: (a: isize, f: Formatter) => FmtResult = i32LowerHex as never;
/** `isize`'s `UpperHex`, `{:X}`. */
export const isizeUpperHex: (a: isize, f: Formatter) => FmtResult = i32UpperHex as never;
/** `isize`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const isizeDebug: (a: isize, f: Formatter) => FmtResult = i32Debug as never;
/** `isize`'s `Binary`, `{:b}`. */
export const isizeBinary: (a: isize, f: Formatter) => FmtResult = i32Binary as never;
/** `isize`'s `Octal`, `{:o}`. */
export const isizeOctal: (a: isize, f: Formatter) => FmtResult = i32Octal as never;
/** `isize`'s `LowerExp`, `{:e}`. */
export const isizeLowerExp: (a: isize, f: Formatter) => FmtResult = i32LowerExp as never;
/** `isize`'s `UpperExp`, `{:E}`. */
export const isizeUpperExp: (a: isize, f: Formatter) => FmtResult = i32UpperExp as never;
/** `isize`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const isizeStep: Step<isize> = i32Step as never;

// 64- and 128-bit integers: numbers while they are safe integers, whose arithmetic stays
// in numbers while its result does, and limbs past that. Each function takes the type's
// sign, and its width where both widths share the function, which each type's own passes
// as literals. A type's own takes the number path itself where it is hot, small enough
// for a JIT to inline: through the shared function alone, V8 took up to 1.7x as long on
// the codec's frames. Limbs compute on their 32-bit words, several times faster than
// through bigints: sums, differences, comparisons, casts, products of 64-bit values or of
// two numbers, and division by a divisor below 2^32. The rest goes through bigints.

type Wide = number | Limbs;

/** Whether a number is a safe integer of the sign's range. */
function fits(n: number, signed: boolean): boolean {
	return n <= MAX_SAFE && n >= (signed ? -MAX_SAFE : 0);
}

/** Whether `v` is in range of the type. */
function fitsBig(v: bigint, bits: number, signed: boolean): boolean {
	return (signed ? BigInt.asIntN(bits, v) : BigInt.asUintN(bits, v)) === v;
}

/** `v`'s limbs, wrapped into the type's range. */
function box(v: bigint, bits: number, signed: boolean): Limbs {
	const top = v >> BigInt(bits - 32);
	const hi = Number(signed ? BigInt.asIntN(32, top) : BigInt.asUintN(32, top));
	const lo = Number(BigInt.asUintN(32, v));
	if (bits === 64) return { hi, lo } as Limbs;
	return { hi, w2: Number(BigInt.asUintN(32, v >> 64n)), w1: Number(BigInt.asUintN(32, v >> 32n)), lo } as Limbs;
}

/** The one form of a value: a number if it is a safe integer. */
function one(v: Limbs): Wide {
	const n = approx(v);
	return safe(n) ? n : v;
}

/** The high half's value from any 32 bits. */
function high(h: number, signed: boolean): number {
	return signed ? h | 0 : h >>> 0;
}

function halves(hi: number, lo: number): Limbs {
	return { hi, lo } as Limbs;
}

/** The one form of a 64-bit value from its halves: `hi` in the type's sign, `lo` a uint32. */
function join(hi: number, lo: number): Wide {
	// Rounding keeps the order, so a value past the safe integers still tests past them.
	const n = hi * TWO32 + lo;
	return safe(n) ? n : halves(hi, lo);
}

/** A 64-bit value's high half, in the type's sign, read without boxing a number. */
function hiOf(v: Wide): number {
	return typeof v === "number" ? Math.floor(v / TWO32) : v.hi;
}

/** A 64-bit value's low half, a uint32. */
function loOf(v: Wide): number {
	return typeof v === "number" ? v >>> 0 : v.lo;
}

/** A value's limbs, from a safe integer wrapped into range. */
function limbs(v: Wide, bits: number, signed: boolean): Limbs {
	if (typeof v !== "number") return v;
	const hi = Math.floor(v / TWO32);
	if (bits === 64) return halves(high(hi, signed), v >>> 0);
	const sign = v < 0 ? -1 : 0;
	return { hi: high(sign, signed), w2: sign >>> 0, w1: hi >>> 0, lo: v >>> 0 } as Limbs;
}

/** `v` in range, as limbs; past it, a panic naming `op`. */
function bigChecked(v: bigint, bits: number, signed: boolean, op: string, at?: Location): Limbs {
	if (!fitsBig(v, bits, signed)) panic(`attempt to ${op} with overflow`, at);
	return box(v, bits, signed);
}

function compare(a: Limbs, b: Limbs): number {
	if (a.hi !== b.hi) return a.hi < b.hi ? -1 : 1;
	if ("w1" in a && "w1" in b) {
		if (a.w2 !== b.w2) return a.w2 < b.w2 ? -1 : 1;
		if (a.w1 !== b.w1) return a.w1 < b.w1 ? -1 : 1;
	}
	return a.lo === b.lo ? 0 : a.lo < b.lo ? -1 : 1;
}

// The high half of the last `mul32`.
let mulHi = 0;

/** The full product of two uint32s: its low half, with the high one in `mulHi`. */
function mul32(a: number, b: number): number {
	const a0 = a & 0xffff;
	const a1 = a >>> 16;
	const b0 = b & 0xffff;
	const b1 = b >>> 16;
	const p00 = a0 * b0;
	const p01 = a0 * b1;
	const p10 = a1 * b0;
	const mid = (p00 >>> 16) + (p01 & 0xffff) + (p10 & 0xffff);
	mulHi = a1 * b1 + (p01 >>> 16) + (p10 >>> 16) + (mid >>> 16);
	return (((mid & 0xffff) << 16) | (p00 & 0xffff)) >>> 0;
}

// Which way the last sum left the type's range: 1 past its maximum, -1 below its
// minimum, and 0 not at all.
let overflow = 0;

// A sum of each width is its own function, so a bundle of 64-bit sums leaves out the
// 128-bit limbs. Each limb's carry or borrow is the floor of its sum over 2^32.

/** A 64-bit `a + b`, or `a - b` if `sign` is -1, wrapped into the type's range, setting `overflow`. */
function sum64(a: Wide, b: Wide, sign: number, signed: boolean): Wide {
	const lo = loOf(a) + sign * loOf(b);
	const hi = hiOf(a) + sign * hiOf(b) + Math.floor(lo / TWO32);
	overflow = outside(hi, signed);
	return join(high(hi, signed), lo >>> 0);
}

/** A 128-bit `a + b`, or `a - b` if `sign` is -1, wrapped into the type's range, setting `overflow`. */
function sum128(a: Wide, b: Wide, sign: number, signed: boolean): Wide {
	const x = limbs(a, 128, signed) as U128Limbs;
	const y = limbs(b, 128, signed) as U128Limbs;
	const lo = x.lo + sign * y.lo;
	const w1 = x.w1 + sign * y.w1 + Math.floor(lo / TWO32);
	const w2 = x.w2 + sign * y.w2 + Math.floor(w1 / TWO32);
	const hi = x.hi + sign * y.hi + Math.floor(w2 / TWO32);
	overflow = outside(hi, signed);
	return one({ hi: high(hi, signed), w2: w2 >>> 0, w1: w1 >>> 0, lo: lo >>> 0 } as Limbs);
}

/** Which way a top limb, before it wraps, leaves the sign's range: 1 past it, -1 below it, or 0. */
function outside(hi: number, signed: boolean): number {
	return hi > (signed ? 0x7fffffff : 0xffffffff) ? 1 : hi < (signed ? -0x80000000 : 0) ? -1 : 0;
}

/** The type's maximum if `top`, else its minimum. */
function extreme(top: boolean, bits: number, signed: boolean): Wide {
	if (!top && !signed) return 0;
	const hi = signed ? (top ? 0x7fffffff : -0x80000000) : 0xffffffff;
	const rest = top ? 0xffffffff : 0;
	return (bits === 64 ? { hi, lo: rest } : { hi, w2: rest, w1: rest, lo: rest }) as Limbs;
}

/** The last sum, `v`, or a panic naming `op` if it overflowed. */
function strict(v: Wide, op: string, at?: Location): Wide {
	if (overflow !== 0) panic(`attempt to ${op} with overflow`, at);
	return v;
}

/** The last sum, `v`, or `undefined` if it overflowed. */
function optional(v: Wide): Wide | undefined {
	return overflow === 0 ? v : undefined;
}

/** The last sum, `v`, clamped to the type's range. */
function saturated(v: Wide, bits: number, signed: boolean): Wide {
	return overflow === 0 ? v : extreme(overflow > 0, bits, signed);
}

/** Whether a wide value is negative: only a signed type's can be. */
function negative(v: Wide): boolean {
	return typeof v === "number" ? v < 0 : v.hi < 0;
}

/** `a * b`, clamped to the type's range: past it, toward the sign the product has. */
function saturatingProduct(a: Wide, b: Wide, bits: number, signed: boolean): Wide {
	return product(a, b, bits, signed) ?? extreme(negative(a) === negative(b), bits, signed);
}

/** `a * b`, wrapped into the type's range. */
function wrappingProduct(a: Wide, b: Wide, bits: number, signed: boolean): Wide {
	return product(a, b, bits, signed) ?? one(box(exact(limbs(a, bits, signed)) * exact(limbs(b, bits, signed)), bits, signed));
}

/** Whether `a` is a multiple of `b`, unsigned: of 0, only 0 is. */
function multiple(a: Wide, b: Wide, bits: number, at?: Location): boolean {
	return b === 0 ? a === 0 : rem(a, b, bits, false, at) === 0;
}

/** `Ord::clamp` of wide values: `a` within `[min, max]`, panicking unless `min <= max`. */
function clampWide(a: Wide, min: Wide, max: Wide, at?: Location): Wide {
	if (lt(max, min)) panicFmt(["min > max. min = ", fmtOwned(min, wideDisplay), ", max = ", fmtOwned(max, wideDisplay)], at);
	return lt(a, min) ? min : lt(max, a) ? max : a;
}

function mul(a: Wide, b: Wide, bits: number, signed: boolean, at?: Location): Wide {
	const v = product(a, b, bits, signed);
	if (v === undefined) panic("attempt to multiply with overflow", at);
	return v;
}

/**
 * `a * b`, or `undefined` past the type's range. Neither factor is boxed: 64-bit ones
 * multiply by their halves, as do two safe integers at 128 bits.
 */
function product(a: Wide, b: Wide, bits: number, signed: boolean): Wide | undefined {
	// `+ 0` turns -0 into 0.
	if (typeof a === "number" && typeof b === "number" && safe(a * b)) return a * b + 0;
	if (bits === 64) return product64(a, b, signed);
	if (typeof a === "number" && typeof b === "number") return product128(a, b);
	const v = exact(limbs(a, bits, signed)) * exact(limbs(b, bits, signed));
	return fitsBig(v, bits, signed) ? one(box(v, bits, signed)) : undefined;
}

/** A 64-bit product on halves: of the magnitudes, then signed. */
function product64(a: Wide, b: Wide, signed: boolean): Wide | undefined {
	let xh = hiOf(a);
	let xl = loOf(a);
	let yh = hiOf(b);
	let yl = loOf(b);
	const negative = signed && xh < 0 !== yh < 0;
	if (signed && xh < 0) {
		xl = -xl >>> 0;
		xh = (~xh + (xl === 0 ? 1 : 0)) >>> 0;
	}
	if (signed && yh < 0) {
		yl = -yl >>> 0;
		yh = (~yh + (yl === 0 ? 1 : 0)) >>> 0;
	}
	// Two high halves pass 2^64. Otherwise one factor is below 2^32, and its product with
	// the other's high half must be too: past 2^53 that sum rounds, but stays past it.
	if (xh !== 0 && yh !== 0) return undefined;
	const lo = mul32(xl, yl);
	const hi = mulHi + xh * yl + xl * yh;
	if (!signed) return hi > 0xffffffff ? undefined : join(hi, lo);
	if (!negative) return hi > 0x7fffffff ? undefined : join(hi, lo);
	if (hi > 0x80000000 || (hi === 0x80000000 && lo !== 0)) return undefined;
	const nlo = -lo >>> 0;
	return join((~hi + (nlo === 0 ? 1 : 0)) | 0, nlo);
}

/**
 * The product of two safe integers past 2^53, as 128-bit limbs: of their magnitudes'
 * halves, each partial product below 2^53 and so exact, then signed: negative only in an
 * `i128`. Under 2^106, it fits either 128-bit type.
 */
function product128(a: number, b: number): Limbs {
	const x = Math.abs(a);
	const y = Math.abs(b);
	const xh = Math.floor(x / TWO32);
	const xl = x >>> 0;
	const yh = Math.floor(y / TWO32);
	const yl = y >>> 0;
	const w0 = mul32(xl, yl);
	const m1 = xh * yl;
	const m2 = xl * yh;
	const s1 = mulHi + (m1 >>> 0) + (m2 >>> 0);
	const s2 = xh * yh + Math.floor(m1 / TWO32) + Math.floor(m2 / TWO32) + Math.floor(s1 / TWO32);
	const w1 = s1 >>> 0;
	const w2 = s2 >>> 0;
	const w3 = Math.floor(s2 / TWO32);
	if (a < 0 === b < 0) return { hi: w3, w2, w1, lo: w0 } as Limbs;
	// Two's complement: each limb inverted, plus the carry out of those below it.
	const lo = -w0 >>> 0;
	const n1 = (~w1 + (lo === 0 ? 1 : 0)) >>> 0;
	const n2 = (~w2 + (lo === 0 && n1 === 0 ? 1 : 0)) >>> 0;
	const n3 = ~w3 + (lo === 0 && n1 === 0 && n2 === 0 ? 1 : 0);
	return { hi: n3 | 0, w2: n2, w1: n1, lo } as Limbs;
}

/** An unsigned 64-bit `a / b`, rounded up, panicking on a zero divisor. */
function ceil64(a: Wide, b: Wide, at?: Location): Wide {
	const q = div(a, b, 64, false, at);
	return rem(a, b, 64, false, at) === 0 ? q : sum64(q, 1, 1, false);
}

/** An unsigned 128-bit `a / b`, rounded up, panicking on a zero divisor. */
function ceil128(a: Wide, b: Wide, at?: Location): Wide {
	const q = div(a, b, 128, false, at);
	return rem(a, b, 128, false, at) === 0 ? q : sum128(q, 1, 1, false);
}

function div(a: Wide, b: Wide, bits: number, signed: boolean, at?: Location): Wide {
	if (b === 0) panic("attempt to divide by zero", at);
	return quotient(a, b, bits, signed, false, at);
}

function rem(a: Wide, b: Wide, bits: number, signed: boolean, at?: Location): Wide {
	if (b === 0) panic("attempt to calculate the remainder with a divisor of zero", at);
	return quotient(a, b, bits, signed, true, at);
}

/** `a / b`, truncated, or `a % b` if `remainder`, by a divisor other than 0. */
function quotient(a: Wide, b: Wide, bits: number, signed: boolean, remainder: boolean, at?: Location): Wide {
	// Below 2^53, a quotient of doubles truncates to the exact one.
	if (typeof a === "number" && typeof b === "number") return (remainder ? a % b : Math.trunc(a / b)) + 0;
	// Limbs pass every number in magnitude.
	if (typeof a === "number") return remainder ? a : 0;
	if (typeof b === "number" && b < TWO32 && b > -TWO32) return divideWords(a, b, bits, signed, remainder, at);
	const n = exact(a);
	const d = exact(limbs(b, bits, signed));
	if (remainder) return one(box(n % d, bits, signed));
	return one(bigChecked(n / d, bits, signed, "divide", at));
}

// A magnitude's 32-bit words, most significant first, which `divideWords` divides in place.
const words = /* @__PURE__ */ new Uint32Array(4);

/** Negates the 128 bits in `words`. */
function negateWords() {
	let carry = 1;
	for (let i = 3; i >= 0; i--) {
		const w = (~words[i] >>> 0) + carry;
		words[i] = w;
		carry = w > 0xffffffff ? 1 : 0;
	}
}

/**
 * `a / b` or `a % b` of limbs by a number below 2^32 in magnitude: a long division of
 * their magnitudes in 16-bit digits, each step exact, then signed as Rust's truncation is.
 */
function divideWords(a: Limbs, b: number, bits: number, signed: boolean, remainder: boolean, at?: Location): Wide {
	const negative = a.hi < 0;
	if (b === -1 && a.hi === -0x80000000 && a.lo === 0 && (!("w1" in a) || (a.w2 === 0 && a.w1 === 0))) {
		panic(remainder ? "attempt to calculate the remainder with overflow" : "attempt to divide with overflow", at);
	}
	if ("w1" in a) {
		words[0] = a.hi;
		words[1] = a.w2;
		words[2] = a.w1;
	} else {
		words[0] = words[1] = negative ? -1 : 0;
		words[2] = a.hi;
	}
	words[3] = a.lo;
	if (negative) negateWords();
	const d = Math.abs(b);
	let r = 0;
	for (let i = bits === 64 ? 2 : 0; i < 4; i++) {
		const w = words[i];
		const t1 = r * 65536 + (w >>> 16);
		const q1 = Math.floor(t1 / d);
		const t0 = (t1 - q1 * d) * 65536 + (w & 0xffff);
		const q0 = Math.floor(t0 / d);
		r = t0 - q0 * d;
		words[i] = q1 * 65536 + q0;
	}
	if (remainder) return negative ? -r + 0 : r;
	if (negative !== b < 0) negateWords();
	if (bits === 64) return join(high(words[2], signed), words[3]);
	return one({ hi: high(words[0], signed), w2: words[1], w1: words[2], lo: words[3] } as Limbs);
}

// The shifts and bitwise operators compute 64 bits on limbs and 128 through bigints, each
// width its own function, so a bundle keeps only the widths it calls.

function shl64(a: Wide, b: number | Limbs, signed: boolean, at?: Location): Wide {
	const n = shift(b, 64, "left", at);
	if (typeof a === "number" && safe(a * 2 ** n)) return a * 2 ** n;
	const x = limbs(a, 64, signed);
	if (n === 0) return one(x);
	if (n < 32) return join(high((x.hi << n) | (x.lo >>> (32 - n)), signed), (x.lo << n) >>> 0);
	return join(high(x.lo << (n - 32), signed), 0);
}

function shl128(a: Wide, b: number | Limbs, signed: boolean, at?: Location): Wide {
	const n = shift(b, 128, "left", at);
	if (typeof a === "number" && safe(a * 2 ** n)) return a * 2 ** n;
	return one(box(exact(limbs(a, 128, signed)) << BigInt(n), 128, signed));
}

function shr64(a: Wide, b: number | Limbs, signed: boolean, at?: Location): Wide {
	const n = shift(b, 64, "right", at);
	if (typeof a === "number") return Math.floor(a / 2 ** n) + 0;
	if (n === 0) return one(a);
	if (signed) {
		if (n < 32) return join(a.hi >> n, ((a.lo >>> n) | (a.hi << (32 - n))) >>> 0);
		return join(a.hi >> 31, (a.hi >> (n - 32)) >>> 0);
	}
	if (n < 32) return join(a.hi >>> n, ((a.lo >>> n) | (a.hi << (32 - n))) >>> 0);
	return join(0, a.hi >>> (n - 32));
}

function shr128(a: Wide, b: number | Limbs, signed: boolean, at?: Location): Wide {
	const n = shift(b, 128, "right", at);
	if (typeof a === "number") return Math.floor(a / 2 ** n) + 0;
	return one(box((signed ? exact(a) : BigInt.asUintN(128, exact(a))) >> BigInt(n), 128, signed));
}

// On two safe integers, a type's own computes the bitwise operators a 32-bit half at a
// time, which can make -2^53 in a signed type, the one result past the safe integers;
// past that, these compute them.

function wordAnd(a: number, b: number): number {
	return (Math.floor(a / TWO32) & Math.floor(b / TWO32)) * TWO32 + ((a & b) >>> 0);
}

function wordOr(a: number, b: number): number {
	return (Math.floor(a / TWO32) | Math.floor(b / TWO32)) * TWO32 + ((a | b) >>> 0);
}

function wordXor(a: number, b: number): number {
	return (Math.floor(a / TWO32) ^ Math.floor(b / TWO32)) * TWO32 + ((a ^ b) >>> 0);
}

function and64(a: Wide, b: Wide, signed: boolean): Wide {
	const x = limbs(a, 64, signed);
	const y = limbs(b, 64, signed);
	return join(high(x.hi & y.hi, signed), (x.lo & y.lo) >>> 0);
}

function and128(a: Wide, b: Wide, signed: boolean): Wide {
	return one(box(exact(limbs(a, 128, signed)) & exact(limbs(b, 128, signed)), 128, signed));
}

function or64(a: Wide, b: Wide, signed: boolean): Wide {
	const x = limbs(a, 64, signed);
	const y = limbs(b, 64, signed);
	return join(high(x.hi | y.hi, signed), (x.lo | y.lo) >>> 0);
}

function or128(a: Wide, b: Wide, signed: boolean): Wide {
	return one(box(exact(limbs(a, 128, signed)) | exact(limbs(b, 128, signed)), 128, signed));
}

function xor64(a: Wide, b: Wide, signed: boolean): Wide {
	const x = limbs(a, 64, signed);
	const y = limbs(b, 64, signed);
	return join(high(x.hi ^ y.hi, signed), (x.lo ^ y.lo) >>> 0);
}

function xor128(a: Wide, b: Wide, signed: boolean): Wide {
	return one(box(exact(limbs(a, 128, signed)) ^ exact(limbs(b, 128, signed)), 128, signed));
}

function not64(a: Wide, signed: boolean): Wide {
	if (signed && typeof a === "number" && a !== MAX_SAFE) return -a - 1;
	const x = limbs(a, 64, signed);
	return join(high(~x.hi, signed), ~x.lo >>> 0);
}

function not128(a: Wide, signed: boolean): Wide {
	if (signed && typeof a === "number" && a !== MAX_SAFE) return -a - 1;
	return one(box(~exact(limbs(a, 128, signed)), 128, signed));
}

/** The count of trailing zero bits of `a`, a value of a type `bits` wide: all of them if `a` is 0. */
function trailingZeros32(a: number, bits: number): number {
	return a === 0 ? bits : 31 - Math.clz32(a & -a);
}

function trailingZeros64(a: Wide, signed: boolean): number {
	const x = limbs(a, 64, signed);
	return x.lo !== 0 ? trailingZeros32(x.lo, 32) : 32 + trailingZeros32(x.hi, 32);
}

function trailingZeros128(a: Wide, signed: boolean): number {
	const v = BigInt.asUintN(128, exact(limbs(a, 128, signed)));
	if (v === 0n) return 128;
	let n = 0;
	while (!((v >> BigInt(n)) & 1n)) n++;
	return n;
}

function leadingZeros64(a: Wide, signed: boolean): number {
	const x = limbs(a, 64, signed);
	const hi = x.hi >>> 0;
	return hi !== 0 ? Math.clz32(hi) : 32 + Math.clz32(x.lo);
}

function leadingOnes64(a: Wide, signed: boolean): number {
	const x = limbs(a, 64, signed);
	const hi = ~x.hi >>> 0;
	return hi !== 0 ? Math.clz32(hi) : 32 + Math.clz32(~x.lo >>> 0);
}

/** The number of leading ones of a 128-bit value's bits. */
function leading128(v: bigint): number {
	let n = 0;
	for (let bit = 127n; bit >= 0n && (v >> bit) & 1n; bit--) n++;
	return n;
}

function leadingZeros128(a: Wide, signed: boolean): number {
	return leading128(BigInt.asUintN(128, ~exact(limbs(a, 128, signed))));
}

function leadingOnes128(a: Wide, signed: boolean): number {
	return leading128(BigInt.asUintN(128, exact(limbs(a, 128, signed))));
}

function fromBeBytes64(b: Uint8Array, signed: boolean): Wide {
	return join(high(word(b, 0), signed), word(b, 4));
}

function fromBeBytes128(b: Uint8Array, signed: boolean): Wide {
	// A 32-bit word at a time: one bigint step per word, not per byte.
	const v = (((((BigInt(word(b, 0)) << 32n) | BigInt(word(b, 4))) << 32n) | BigInt(word(b, 8))) << 32n) | BigInt(word(b, 12));
	return one(box(v, 128, signed));
}

/** `w`'s big-endian bytes, into `out` at `i`. */
function putWord(out: Uint8Array, i: number, w: number): void {
	out[i] = w >>> 24;
	out[i + 1] = w >>> 16;
	out[i + 2] = w >>> 8;
	out[i + 3] = w;
}

function toBeBytes64(a: Wide, signed: boolean): Uint8Array {
	const x = limbs(a, 64, signed);
	const out = new Uint8Array(8);
	putWord(out, 0, x.hi);
	putWord(out, 4, x.lo);
	return out;
}

function toBeBytes128(a: Wide, signed: boolean): Uint8Array {
	const x = limbs(a, 128, signed) as U128Limbs | I128Limbs;
	const out = new Uint8Array(16);
	putWord(out, 0, x.hi);
	putWord(out, 4, x.w2);
	putWord(out, 8, x.w1);
	putWord(out, 12, x.lo);
	return out;
}

function tryFrom(value: number | Limbs, bits: number, signed: boolean): Result<Wide, TryFromIntError> {
	const v = typeof value === "number" ? BigInt(value) : exact(value);
	if (fitsBig(v, bits, signed)) return Ok(one(box(v, bits, signed)));
	return Err({ kind: v < 0n ? "NegOverflow" : "PosOverflow" });
}

function wideDisplay(a: Wide, f: Formatter): FmtResult {
	if (typeof a === "number") return display(a, f);
	const v = exact(a);
	return fmtPadIntegral(f, v >= 0n, "", (v < 0n ? -v : v).toString());
}

function wideDebug(a: Wide, bits: number, signed: boolean, f: Formatter): FmtResult {
	const hexDigits = f.spec.debugHex;
	return hexDigits === undefined ? wideDisplay(a, f) : wideHex(a, bits, signed, f, hexDigits === "X");
}

/** A value's bits as unsigned. */
function wideBits(a: Wide, bits: number, signed: boolean): bigint {
	return BigInt.asUintN(bits, exact(limbs(a, bits, signed)));
}

function wideHex(a: Wide, bits: number, signed: boolean, f: Formatter, upper: boolean): FmtResult {
	return hex(wideBits(a, bits, signed), f, upper);
}

function wideExp(a: Wide, f: Formatter, letter: "e" | "E"): FmtResult {
	const v = typeof a === "number" ? BigInt(a) : exact(a);
	return exp(v >= 0n, (v < 0n ? -v : v).toString(), f, letter);
}

/** A value from a safe integer, or from a bigint of any size, wrapped into range. */
function of(value: number | bigint, bits: number, signed: boolean): Wide {
	if (typeof value === "number") {
		if (!Number.isSafeInteger(value)) throw new RangeError(`rust2-ts: ${value} is not a safe integer`);
		if (fits(value, signed)) return value;
		return one(limbs(value, bits, signed));
	}
	return one(box(value, bits, signed));
}

/** An integer of `bits` from a bigint in its range: a number up to 32 bits, else as `of` makes it. */
export function intFromBig(value: bigint, bits: number, signed: boolean): number | Limbs {
	return bits <= 32 ? Number(value) : of(value, bits, signed);
}

/** Any integer's exact value. */
export function intToBig(value: number | Limbs): bigint {
	return typeof value === "number" ? BigInt(value) : exact(value);
}

/** `value as T`, from any integer: its low bits, read in the type's sign. */
function cast(value: number | Limbs, bits: number, signed: boolean): Wide {
	if (typeof value === "number") return fits(value, signed) ? value : one(limbs(value, bits, signed));
	if (bits === 64) return join(high("w1" in value ? value.w1 : value.hi, signed), value.lo);
	if ("w1" in value) return one({ hi: high(value.hi, signed), w2: value.w2, w1: value.w1, lo: value.lo } as Limbs);
	// 64-bit limbs extend in their own sign, which their `hi` holds.
	const sign = value.hi < 0 ? -1 : 0;
	return one({ hi: high(sign, signed), w2: sign >>> 0, w1: value.hi >>> 0, lo: value.lo } as Limbs);
}

function low(a: Wide): number {
	return typeof a === "number" ? a >>> 0 : a.lo;
}

function eq(a: Wide, b: Wide): boolean {
	return a === b || (typeof a !== "number" && typeof b !== "number" && compare(a, b) === 0);
}

/** `Step`'s result from the last sum, `v`, which moved by `by`'s magnitude. */
function stepped(v: Wide): [Wide, boolean] {
	return [v, overflow !== 0];
}

/** `u64`'s `a + b`, panicking on overflow. */
export const u64Add = (a: u64, b: u64, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : strict(sum64(a, b, 1, false), "add", at)) as u64;
/** `u64`'s `a - b`, panicking on overflow. */
export const u64Sub = (a: u64, b: u64, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : strict(sum64(a, b, -1, false), "subtract", at)) as u64;
/** `u64`'s `a * b`, panicking on overflow. */
export const u64Mul = (a: u64, b: u64, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : mul(a, b, 64, false, at)) as u64;
/** `u64`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const u64Div = (a: u64, b: u64, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && b !== 0 ? Math.trunc(a / b) + 0 : div(a, b, 64, false, at)) as u64;
/** `u64`'s `a % b`, panicking on a zero divisor or overflow. */
export const u64Rem = (a: u64, b: u64, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && b !== 0 ? (a % b) + 0 : rem(a, b, 64, false, at)) as u64;
/** `u64`'s `a << b`, panicking on a shift by the width or more. */
export const u64Shl = (a: u64, b: number | Limbs, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 64 && safe(a * 2 ** b) ? a * 2 ** b : shl64(a, b, false, at)) as u64;
/** `u64`'s `a >> b`, panicking on a shift by the width or more. */
export const u64Shr = (a: u64, b: number | Limbs, at?: Location): u64 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 64 ? Math.floor(a / 2 ** b) + 0 : shr64(a, b, false, at)) as u64;
/** `u64`'s `a & b`. */
export const u64And = (a: u64, b: u64): u64 => {
	const n = typeof a === "number" && typeof b === "number" ? wordAnd(a, b) : NaN;
	return (fits(n, false) ? n : and64(a, b, false)) as u64;
};
/** `u64`'s `a | b`. */
export const u64Or = (a: u64, b: u64): u64 => {
	const n = typeof a === "number" && typeof b === "number" ? wordOr(a, b) : NaN;
	return (fits(n, false) ? n : or64(a, b, false)) as u64;
};
/** `u64`'s `a ^ b`. */
export const u64Xor = (a: u64, b: u64): u64 => {
	const n = typeof a === "number" && typeof b === "number" ? wordXor(a, b) : NaN;
	return (fits(n, false) ? n : xor64(a, b, false)) as u64;
};
/** `u64`'s `!a`. */
export const u64Not = (a: u64): u64 => not64(a, false) as u64;
/** `u64`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const u64CheckedAdd = (a: u64, b: u64): u64 | undefined => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : optional(sum64(a, b, 1, false))) as u64 | undefined;
/** `u64`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const u64CheckedSub = (a: u64, b: u64): u64 | undefined => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : optional(sum64(a, b, -1, false))) as u64 | undefined;
/** `u64`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const u64CheckedMul = (a: u64, b: u64): u64 | undefined => product(a, b, 64, false) as u64 | undefined;
/** `u64`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const u64SaturatingAdd = (a: u64, b: u64): u64 => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : saturated(sum64(a, b, 1, false), 64, false)) as u64;
/** `u64`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const u64SaturatingSub = (a: u64, b: u64): u64 => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : saturated(sum64(a, b, -1, false), 64, false)) as u64;
/** `u64`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const u64WrappingAdd = (a: u64, b: u64): u64 => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : sum64(a, b, 1, false)) as u64;
/** `u64`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const u64WrappingSub = (a: u64, b: u64): u64 => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : sum64(a, b, -1, false)) as u64;
/** `u64`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const u64SaturatingMul = (a: u64, b: u64): u64 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : saturatingProduct(a, b, 64, false)) as u64;
/** `u64`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const u64WrappingMul = (a: u64, b: u64): u64 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : wrappingProduct(a, b, 64, false)) as u64;
/** `u64`'s `div_ceil`: `a / b`, rounded up, panicking on a zero divisor. */
export const u64DivCeil = (a: u64, b: u64, at?: Location): u64 => (typeof a === "number" && typeof b === "number" ? ceilDivide(a, b, at) : ceil64(a, b, at)) as u64;
/** `u64`'s `is_multiple_of`: whether `b` divides `a`; of 0, only 0 is a multiple. */
export const u64IsMultipleOf = (a: u64, b: u64, at?: Location): boolean => (typeof a === "number" && typeof b === "number" ? multipleOf(a, b) : multiple(a, b, 64, at));
/** `u64`'s `leading_ones`: the count of leading one bits. */
export const u64LeadingOnes = (a: u64): number => leadingOnes64(a, false);
/** `u64`'s `leading_zeros`: the count of leading zero bits. */
export const u64LeadingZeros = (a: u64): number => leadingZeros64(a, false);
/** `u64`'s `trailing_zeros`: the count of trailing zero bits. */
export const u64TrailingZeros = (a: u64): number => trailingZeros64(a, false);
/** `u64`'s `from_be_bytes`: a value from its big-endian bytes. */
export const u64FromBeBytes = (bytes: Uint8Array): u64 => fromBeBytes64(bytes, false) as u64;
/** `u64`'s `to_be_bytes`: its big-endian bytes. */
export const u64ToBeBytes = (a: u64): Uint8Array => toBeBytes64(a, false);
/** `u64`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const u64TryFrom = (value: number | Limbs): Result<u64, TryFromIntError> => tryFrom(value, 64, false) as Result<u64, TryFromIntError>;
/** `u64`'s `Display`: its decimal digits, padded by the formatter's spec. */
export const u64Display = (a: u64, f: Formatter): FmtResult => wideDisplay(a, f);
/** `u64`'s `LowerHex`, `{:x}`. */
export const u64LowerHex = (a: u64, f: Formatter): FmtResult => wideHex(a, 64, false, f, false);
/** `u64`'s `UpperHex`, `{:X}`. */
export const u64UpperHex = (a: u64, f: Formatter): FmtResult => wideHex(a, 64, false, f, true);
/** `u64`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const u64Debug = (a: u64, f: Formatter): FmtResult => wideDebug(a, 64, false, f);
/** `u64`'s `Binary`, `{:b}`. */
export const u64Binary = (a: u64, f: Formatter): FmtResult => radix(wideBits(a, 64, false), f, 2, "0b");
/** `u64`'s `Octal`, `{:o}`. */
export const u64Octal = (a: u64, f: Formatter): FmtResult => radix(wideBits(a, 64, false), f, 8, "0o");
/** `u64`'s `LowerExp`, `{:e}`. */
export const u64LowerExp = (a: u64, f: Formatter): FmtResult => wideExp(a, f, "e");
/** `u64`'s `UpperExp`, `{:E}`. */
export const u64UpperExp = (a: u64, f: Formatter): FmtResult => wideExp(a, f, "E");
/** A `u64` from a safe integer, or from a bigint of any size, wrapped into range. */
export const u64Of = (value: number | bigint): u64 => of(value, 64, false) as u64;
/** `value as u64`. */
export const u64Cast = (value: number | Limbs): u64 => (typeof value === "number" && fits(value, false) ? value : cast(value, 64, false)) as u64;
/** The low 32 bits, as a uint32. */
export const u64Low = (a: u64): number => low(a);
/** `(hi as u64) << 32 | lo as u64`: a `u64` from its high and low 32 bits. */
export const u64FromHalves = (hi: u32, lo: u32): u64 => join(hi, lo) as u64;
/** `u64`'s `a == b`. */
export const u64Eq = (a: u64, b: u64): boolean => eq(a, b);
/** `u64`'s `a != b`. */
export const u64Ne = (a: u64, b: u64): boolean => !eq(a, b);
/** `u64`'s `a < b`. */
export const u64Lt = (a: u64, b: u64): boolean => typeof a === "number" && typeof b === "number" ? a < b : lt(a, b);
/** `u64`'s `a <= b`. */
export const u64Le = (a: u64, b: u64): boolean => typeof a === "number" && typeof b === "number" ? a <= b : !lt(b, a);
/** `u64`'s `a > b`. */
export const u64Gt = (a: u64, b: u64): boolean => typeof a === "number" && typeof b === "number" ? a > b : lt(b, a);
/** `u64`'s `a >= b`. */
export const u64Ge = (a: u64, b: u64): boolean => typeof a === "number" && typeof b === "number" ? a >= b : !lt(a, b);
/** `u64`'s `Ord::max`: the greater, `b` if they are equal. */
export const u64Max = (a: u64, b: u64): u64 => (lt(b, a) ? a : b);
/** `u64`'s `Ord::min`: the lesser, `a` if they are equal. */
export const u64Min = (a: u64, b: u64): u64 => (lt(b, a) ? b : a);
/** `u64`'s `Ord::clamp`: `a` within `[min, max]`, panicking unless `min <= max`. */
export const u64Clamp = (a: u64, min: u64, max: u64, at?: Location): u64 => clampWide(a, min, max, at) as u64;
/** `u64`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const u64Step: Step<u64> = (n, by) => stepped(sum64(n, Math.abs(by), by < 0 ? -1 : 1, false)) as [u64, boolean];

/** `i64`'s `a + b`, panicking on overflow. */
export const i64Add = (a: i64, b: i64, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : strict(sum64(a, b, 1, true), "add", at)) as i64;
/** `i64`'s `a - b`, panicking on overflow. */
export const i64Sub = (a: i64, b: i64, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : strict(sum64(a, b, -1, true), "subtract", at)) as i64;
/** `i64`'s `a * b`, panicking on overflow. */
export const i64Mul = (a: i64, b: i64, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : mul(a, b, 64, true, at)) as i64;
/** `i64`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const i64Div = (a: i64, b: i64, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && b !== 0 ? Math.trunc(a / b) + 0 : div(a, b, 64, true, at)) as i64;
/** `i64`'s `a % b`, panicking on a zero divisor or overflow. */
export const i64Rem = (a: i64, b: i64, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && b !== 0 ? (a % b) + 0 : rem(a, b, 64, true, at)) as i64;
/** `i64`'s `-a`, panicking on overflow. */
export const i64Neg = (a: i64, at?: Location): i64 => (typeof a === "number" ? -a + 0 : strict(sum64(0, a, -1, true), "negate", at)) as i64;
/** `i64`'s `a << b`, panicking on a shift by the width or more. */
export const i64Shl = (a: i64, b: number | Limbs, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 64 && safe(a * 2 ** b) ? a * 2 ** b : shl64(a, b, true, at)) as i64;
/** `i64`'s `a >> b`, panicking on a shift by the width or more. */
export const i64Shr = (a: i64, b: number | Limbs, at?: Location): i64 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 64 ? Math.floor(a / 2 ** b) + 0 : shr64(a, b, true, at)) as i64;
/** `i64`'s `a & b`. */
export const i64And = (a: i64, b: i64): i64 => {
	const n = typeof a === "number" && typeof b === "number" ? wordAnd(a, b) : NaN;
	return (fits(n, true) ? n : and64(a, b, true)) as i64;
};
/** `i64`'s `a | b`. */
export const i64Or = (a: i64, b: i64): i64 => {
	const n = typeof a === "number" && typeof b === "number" ? wordOr(a, b) : NaN;
	return (fits(n, true) ? n : or64(a, b, true)) as i64;
};
/** `i64`'s `a ^ b`. */
export const i64Xor = (a: i64, b: i64): i64 => {
	const n = typeof a === "number" && typeof b === "number" ? wordXor(a, b) : NaN;
	return (fits(n, true) ? n : xor64(a, b, true)) as i64;
};
/** `i64`'s `!a`. */
export const i64Not = (a: i64): i64 => not64(a, true) as i64;
/** `i64`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const i64CheckedAdd = (a: i64, b: i64): i64 | undefined => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : optional(sum64(a, b, 1, true))) as i64 | undefined;
/** `i64`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const i64CheckedSub = (a: i64, b: i64): i64 | undefined => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : optional(sum64(a, b, -1, true))) as i64 | undefined;
/** `i64`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const i64CheckedMul = (a: i64, b: i64): i64 | undefined => product(a, b, 64, true) as i64 | undefined;
/** `i64`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const i64SaturatingAdd = (a: i64, b: i64): i64 => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : saturated(sum64(a, b, 1, true), 64, true)) as i64;
/** `i64`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const i64SaturatingSub = (a: i64, b: i64): i64 => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : saturated(sum64(a, b, -1, true), 64, true)) as i64;
/** `i64`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const i64WrappingAdd = (a: i64, b: i64): i64 => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : sum64(a, b, 1, true)) as i64;
/** `i64`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const i64WrappingSub = (a: i64, b: i64): i64 => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : sum64(a, b, -1, true)) as i64;
/** `i64`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const i64SaturatingMul = (a: i64, b: i64): i64 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : saturatingProduct(a, b, 64, true)) as i64;
/** `i64`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const i64WrappingMul = (a: i64, b: i64): i64 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : wrappingProduct(a, b, 64, true)) as i64;
/** `i64`'s `wrapping_div`: `a / b`, truncated and wrapped into the type's range, panicking on a zero divisor. */
export const i64WrappingDiv = (a: i64, b: i64, at?: Location): i64 => (b === -1 ? i64WrappingSub(0, a) : i64Div(a, b, at));
/** `i64`'s `wrapping_rem`: `a % b`, 0 where it would overflow, panicking on a zero divisor. */
export const i64WrappingRem = (a: i64, b: i64, at?: Location): i64 => (b === -1 ? 0 : i64Rem(a, b, at));
/** `i64`'s `saturating_div`: `a / b`, truncated and clamped to the type's range, panicking on a zero divisor. */
export const i64SaturatingDiv = (a: i64, b: i64, at?: Location): i64 => (b === -1 ? i64SaturatingSub(0, a) : i64Div(a, b, at));
/** `i64`'s `leading_ones`: the count of leading one bits. */
export const i64LeadingOnes = (a: i64): number => leadingOnes64(a, true);
/** `i64`'s `leading_zeros`: the count of leading zero bits. */
export const i64LeadingZeros = (a: i64): number => leadingZeros64(a, true);
/** `i64`'s `trailing_zeros`: the count of trailing zero bits. */
export const i64TrailingZeros = (a: i64): number => trailingZeros64(a, true);
/** `i64`'s `from_be_bytes`: a value from its big-endian bytes. */
export const i64FromBeBytes = (bytes: Uint8Array): i64 => fromBeBytes64(bytes, true) as i64;
/** `i64`'s `to_be_bytes`: its big-endian bytes. */
export const i64ToBeBytes = (a: i64): Uint8Array => toBeBytes64(a, true);
/** `i64`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const i64TryFrom = (value: number | Limbs): Result<i64, TryFromIntError> => tryFrom(value, 64, true) as Result<i64, TryFromIntError>;
/** `i64`'s `Display`: its decimal digits, padded by the formatter's spec. */
export const i64Display = (a: i64, f: Formatter): FmtResult => wideDisplay(a, f);
/** `i64`'s `LowerHex`, `{:x}`. */
export const i64LowerHex = (a: i64, f: Formatter): FmtResult => wideHex(a, 64, true, f, false);
/** `i64`'s `UpperHex`, `{:X}`. */
export const i64UpperHex = (a: i64, f: Formatter): FmtResult => wideHex(a, 64, true, f, true);
/** `i64`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const i64Debug = (a: i64, f: Formatter): FmtResult => wideDebug(a, 64, true, f);
/** `i64`'s `Binary`, `{:b}`. */
export const i64Binary = (a: i64, f: Formatter): FmtResult => radix(wideBits(a, 64, true), f, 2, "0b");
/** `i64`'s `Octal`, `{:o}`. */
export const i64Octal = (a: i64, f: Formatter): FmtResult => radix(wideBits(a, 64, true), f, 8, "0o");
/** `i64`'s `LowerExp`, `{:e}`. */
export const i64LowerExp = (a: i64, f: Formatter): FmtResult => wideExp(a, f, "e");
/** `i64`'s `UpperExp`, `{:E}`. */
export const i64UpperExp = (a: i64, f: Formatter): FmtResult => wideExp(a, f, "E");
/** A `i64` from a safe integer, or from a bigint of any size, wrapped into range. */
export const i64Of = (value: number | bigint): i64 => of(value, 64, true) as i64;
/** `value as i64`. */
export const i64Cast = (value: number | Limbs): i64 => (typeof value === "number" && fits(value, true) ? value : cast(value, 64, true)) as i64;
/** The low 32 bits, as a uint32. */
export const i64Low = (a: i64): number => low(a);
/** `i64`'s `a == b`. */
export const i64Eq = (a: i64, b: i64): boolean => eq(a, b);
/** `i64`'s `a != b`. */
export const i64Ne = (a: i64, b: i64): boolean => !eq(a, b);
/** `i64`'s `a < b`. */
export const i64Lt = (a: i64, b: i64): boolean => typeof a === "number" && typeof b === "number" ? a < b : lt(a, b);
/** `i64`'s `a <= b`. */
export const i64Le = (a: i64, b: i64): boolean => typeof a === "number" && typeof b === "number" ? a <= b : !lt(b, a);
/** `i64`'s `a > b`. */
export const i64Gt = (a: i64, b: i64): boolean => typeof a === "number" && typeof b === "number" ? a > b : lt(b, a);
/** `i64`'s `a >= b`. */
export const i64Ge = (a: i64, b: i64): boolean => typeof a === "number" && typeof b === "number" ? a >= b : !lt(a, b);
/** `i64`'s `Ord::max`: the greater, `b` if they are equal. */
export const i64Max = (a: i64, b: i64): i64 => (lt(b, a) ? a : b);
/** `i64`'s `Ord::min`: the lesser, `a` if they are equal. */
export const i64Min = (a: i64, b: i64): i64 => (lt(b, a) ? b : a);
/** `i64`'s `Ord::clamp`: `a` within `[min, max]`, panicking unless `min <= max`. */
export const i64Clamp = (a: i64, min: i64, max: i64, at?: Location): i64 => clampWide(a, min, max, at) as i64;
/** `i64`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const i64Step: Step<i64> = (n, by) => stepped(sum64(n, Math.abs(by), by < 0 ? -1 : 1, true)) as [i64, boolean];

/** `u128`'s `a + b`, panicking on overflow. */
export const u128Add = (a: u128, b: u128, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : strict(sum128(a, b, 1, false), "add", at)) as u128;
/** `u128`'s `a - b`, panicking on overflow. */
export const u128Sub = (a: u128, b: u128, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : strict(sum128(a, b, -1, false), "subtract", at)) as u128;
/** `u128`'s `a * b`, panicking on overflow. */
export const u128Mul = (a: u128, b: u128, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : mul(a, b, 128, false, at)) as u128;
/** `u128`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const u128Div = (a: u128, b: u128, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && b !== 0 ? Math.trunc(a / b) + 0 : div(a, b, 128, false, at)) as u128;
/** `u128`'s `a % b`, panicking on a zero divisor or overflow. */
export const u128Rem = (a: u128, b: u128, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && b !== 0 ? (a % b) + 0 : rem(a, b, 128, false, at)) as u128;
/** `u128`'s `a << b`, panicking on a shift by the width or more. */
export const u128Shl = (a: u128, b: number | Limbs, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 128 && safe(a * 2 ** b) ? a * 2 ** b : shl128(a, b, false, at)) as u128;
/** `u128`'s `a >> b`, panicking on a shift by the width or more. */
export const u128Shr = (a: u128, b: number | Limbs, at?: Location): u128 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 128 ? Math.floor(a / 2 ** b) + 0 : shr128(a, b, false, at)) as u128;
/** `u128`'s `a & b`. */
export const u128And = (a: u128, b: u128): u128 => {
	const n = typeof a === "number" && typeof b === "number" ? wordAnd(a, b) : NaN;
	return (fits(n, false) ? n : and128(a, b, false)) as u128;
};
/** `u128`'s `a | b`. */
export const u128Or = (a: u128, b: u128): u128 => {
	const n = typeof a === "number" && typeof b === "number" ? wordOr(a, b) : NaN;
	return (fits(n, false) ? n : or128(a, b, false)) as u128;
};
/** `u128`'s `a ^ b`. */
export const u128Xor = (a: u128, b: u128): u128 => {
	const n = typeof a === "number" && typeof b === "number" ? wordXor(a, b) : NaN;
	return (fits(n, false) ? n : xor128(a, b, false)) as u128;
};
/** `u128`'s `!a`. */
export const u128Not = (a: u128): u128 => not128(a, false) as u128;
/** `u128`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const u128CheckedAdd = (a: u128, b: u128): u128 | undefined => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : optional(sum128(a, b, 1, false))) as u128 | undefined;
/** `u128`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const u128CheckedSub = (a: u128, b: u128): u128 | undefined => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : optional(sum128(a, b, -1, false))) as u128 | undefined;
/** `u128`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const u128CheckedMul = (a: u128, b: u128): u128 | undefined => product(a, b, 128, false) as u128 | undefined;
/** `u128`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const u128SaturatingAdd = (a: u128, b: u128): u128 => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : saturated(sum128(a, b, 1, false), 128, false)) as u128;
/** `u128`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const u128SaturatingSub = (a: u128, b: u128): u128 => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : saturated(sum128(a, b, -1, false), 128, false)) as u128;
/** `u128`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const u128WrappingAdd = (a: u128, b: u128): u128 => (typeof a === "number" && typeof b === "number" && fits(a + b, false) ? a + b : sum128(a, b, 1, false)) as u128;
/** `u128`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const u128WrappingSub = (a: u128, b: u128): u128 => (typeof a === "number" && typeof b === "number" && fits(a - b, false) ? a - b : sum128(a, b, -1, false)) as u128;
/** `u128`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const u128SaturatingMul = (a: u128, b: u128): u128 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : saturatingProduct(a, b, 128, false)) as u128;
/** `u128`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const u128WrappingMul = (a: u128, b: u128): u128 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : wrappingProduct(a, b, 128, false)) as u128;
/** `u128`'s `div_ceil`: `a / b`, rounded up, panicking on a zero divisor. */
export const u128DivCeil = (a: u128, b: u128, at?: Location): u128 => (typeof a === "number" && typeof b === "number" ? ceilDivide(a, b, at) : ceil128(a, b, at)) as u128;
/** `u128`'s `is_multiple_of`: whether `b` divides `a`; of 0, only 0 is a multiple. */
export const u128IsMultipleOf = (a: u128, b: u128, at?: Location): boolean => (typeof a === "number" && typeof b === "number" ? multipleOf(a, b) : multiple(a, b, 128, at));
/** `u128`'s `leading_ones`: the count of leading one bits. */
export const u128LeadingOnes = (a: u128): number => leadingOnes128(a, false);
/** `u128`'s `leading_zeros`: the count of leading zero bits. */
export const u128LeadingZeros = (a: u128): number => leadingZeros128(a, false);
/** `u128`'s `trailing_zeros`: the count of trailing zero bits. */
export const u128TrailingZeros = (a: u128): number => trailingZeros128(a, false);
/** `u128`'s `from_be_bytes`: a value from its big-endian bytes. */
export const u128FromBeBytes = (bytes: Uint8Array): u128 => fromBeBytes128(bytes, false) as u128;
/** `u128`'s `to_be_bytes`: its big-endian bytes. */
export const u128ToBeBytes = (a: u128): Uint8Array => toBeBytes128(a, false);
/** `u128`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const u128TryFrom = (value: number | Limbs): Result<u128, TryFromIntError> => tryFrom(value, 128, false) as Result<u128, TryFromIntError>;
/** `u128`'s `Display`: its decimal digits, padded by the formatter's spec. */
export const u128Display = (a: u128, f: Formatter): FmtResult => wideDisplay(a, f);
/** `u128`'s `LowerHex`, `{:x}`. */
export const u128LowerHex = (a: u128, f: Formatter): FmtResult => wideHex(a, 128, false, f, false);
/** `u128`'s `UpperHex`, `{:X}`. */
export const u128UpperHex = (a: u128, f: Formatter): FmtResult => wideHex(a, 128, false, f, true);
/** `u128`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const u128Debug = (a: u128, f: Formatter): FmtResult => wideDebug(a, 128, false, f);
/** `u128`'s `Binary`, `{:b}`. */
export const u128Binary = (a: u128, f: Formatter): FmtResult => radix(wideBits(a, 128, false), f, 2, "0b");
/** `u128`'s `Octal`, `{:o}`. */
export const u128Octal = (a: u128, f: Formatter): FmtResult => radix(wideBits(a, 128, false), f, 8, "0o");
/** `u128`'s `LowerExp`, `{:e}`. */
export const u128LowerExp = (a: u128, f: Formatter): FmtResult => wideExp(a, f, "e");
/** `u128`'s `UpperExp`, `{:E}`. */
export const u128UpperExp = (a: u128, f: Formatter): FmtResult => wideExp(a, f, "E");
/** A `u128` from a safe integer, or from a bigint of any size, wrapped into range. */
export const u128Of = (value: number | bigint): u128 => of(value, 128, false) as u128;
/** `value as u128`. */
export const u128Cast = (value: number | Limbs): u128 => (typeof value === "number" && fits(value, false) ? value : cast(value, 128, false)) as u128;
/** The low 32 bits, as a uint32. */
export const u128Low = (a: u128): number => low(a);
/** `u128`'s `a == b`. */
export const u128Eq = (a: u128, b: u128): boolean => eq(a, b);
/** `u128`'s `a != b`. */
export const u128Ne = (a: u128, b: u128): boolean => !eq(a, b);
/** `u128`'s `a < b`. */
export const u128Lt = (a: u128, b: u128): boolean => typeof a === "number" && typeof b === "number" ? a < b : lt(a, b);
/** `u128`'s `a <= b`. */
export const u128Le = (a: u128, b: u128): boolean => typeof a === "number" && typeof b === "number" ? a <= b : !lt(b, a);
/** `u128`'s `a > b`. */
export const u128Gt = (a: u128, b: u128): boolean => typeof a === "number" && typeof b === "number" ? a > b : lt(b, a);
/** `u128`'s `a >= b`. */
export const u128Ge = (a: u128, b: u128): boolean => typeof a === "number" && typeof b === "number" ? a >= b : !lt(a, b);
/** `u128`'s `Ord::max`: the greater, `b` if they are equal. */
export const u128Max = (a: u128, b: u128): u128 => (lt(b, a) ? a : b);
/** `u128`'s `Ord::min`: the lesser, `a` if they are equal. */
export const u128Min = (a: u128, b: u128): u128 => (lt(b, a) ? b : a);
/** `u128`'s `Ord::clamp`: `a` within `[min, max]`, panicking unless `min <= max`. */
export const u128Clamp = (a: u128, min: u128, max: u128, at?: Location): u128 => clampWide(a, min, max, at) as u128;
/** `u128`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const u128Step: Step<u128> = (n, by) => stepped(sum128(n, Math.abs(by), by < 0 ? -1 : 1, false)) as [u128, boolean];

/** `i128`'s `a + b`, panicking on overflow. */
export const i128Add = (a: i128, b: i128, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : strict(sum128(a, b, 1, true), "add", at)) as i128;
/** `i128`'s `a - b`, panicking on overflow. */
export const i128Sub = (a: i128, b: i128, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : strict(sum128(a, b, -1, true), "subtract", at)) as i128;
/** `i128`'s `a * b`, panicking on overflow. */
export const i128Mul = (a: i128, b: i128, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : mul(a, b, 128, true, at)) as i128;
/** `i128`'s `a / b`, truncated, panicking on a zero divisor or overflow. */
export const i128Div = (a: i128, b: i128, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && b !== 0 ? Math.trunc(a / b) + 0 : div(a, b, 128, true, at)) as i128;
/** `i128`'s `a % b`, panicking on a zero divisor or overflow. */
export const i128Rem = (a: i128, b: i128, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && b !== 0 ? (a % b) + 0 : rem(a, b, 128, true, at)) as i128;
/** `i128`'s `-a`, panicking on overflow. */
export const i128Neg = (a: i128, at?: Location): i128 => (typeof a === "number" ? -a + 0 : strict(sum128(0, a, -1, true), "negate", at)) as i128;
/** `i128`'s `a << b`, panicking on a shift by the width or more. */
export const i128Shl = (a: i128, b: number | Limbs, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 128 && safe(a * 2 ** b) ? a * 2 ** b : shl128(a, b, true, at)) as i128;
/** `i128`'s `a >> b`, panicking on a shift by the width or more. */
export const i128Shr = (a: i128, b: number | Limbs, at?: Location): i128 => (typeof a === "number" && typeof b === "number" && b >= 0 && b < 128 ? Math.floor(a / 2 ** b) + 0 : shr128(a, b, true, at)) as i128;
/** `i128`'s `a & b`. */
export const i128And = (a: i128, b: i128): i128 => {
	const n = typeof a === "number" && typeof b === "number" ? wordAnd(a, b) : NaN;
	return (fits(n, true) ? n : and128(a, b, true)) as i128;
};
/** `i128`'s `a | b`. */
export const i128Or = (a: i128, b: i128): i128 => {
	const n = typeof a === "number" && typeof b === "number" ? wordOr(a, b) : NaN;
	return (fits(n, true) ? n : or128(a, b, true)) as i128;
};
/** `i128`'s `a ^ b`. */
export const i128Xor = (a: i128, b: i128): i128 => {
	const n = typeof a === "number" && typeof b === "number" ? wordXor(a, b) : NaN;
	return (fits(n, true) ? n : xor128(a, b, true)) as i128;
};
/** `i128`'s `!a`. */
export const i128Not = (a: i128): i128 => not128(a, true) as i128;
/** `i128`'s `checked_add`: `a + b`, or `undefined` on overflow. */
export const i128CheckedAdd = (a: i128, b: i128): i128 | undefined => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : optional(sum128(a, b, 1, true))) as i128 | undefined;
/** `i128`'s `checked_sub`: `a - b`, or `undefined` on overflow. */
export const i128CheckedSub = (a: i128, b: i128): i128 | undefined => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : optional(sum128(a, b, -1, true))) as i128 | undefined;
/** `i128`'s `checked_mul`: `a * b`, or `undefined` on overflow. */
export const i128CheckedMul = (a: i128, b: i128): i128 | undefined => product(a, b, 128, true) as i128 | undefined;
/** `i128`'s `saturating_add`: `a + b`, clamped to the type's range. */
export const i128SaturatingAdd = (a: i128, b: i128): i128 => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : saturated(sum128(a, b, 1, true), 128, true)) as i128;
/** `i128`'s `saturating_sub`: `a - b`, clamped to the type's range. */
export const i128SaturatingSub = (a: i128, b: i128): i128 => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : saturated(sum128(a, b, -1, true), 128, true)) as i128;
/** `i128`'s `wrapping_add`: `a + b`, wrapped into the type's range. */
export const i128WrappingAdd = (a: i128, b: i128): i128 => (typeof a === "number" && typeof b === "number" && fits(a + b, true) ? a + b : sum128(a, b, 1, true)) as i128;
/** `i128`'s `wrapping_sub`: `a - b`, wrapped into the type's range. */
export const i128WrappingSub = (a: i128, b: i128): i128 => (typeof a === "number" && typeof b === "number" && fits(a - b, true) ? a - b : sum128(a, b, -1, true)) as i128;
/** `i128`'s `saturating_mul`: `a * b`, clamped to the type's range. */
export const i128SaturatingMul = (a: i128, b: i128): i128 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : saturatingProduct(a, b, 128, true)) as i128;
/** `i128`'s `wrapping_mul`: `a * b`, wrapped into the type's range. */
export const i128WrappingMul = (a: i128, b: i128): i128 => (typeof a === "number" && typeof b === "number" && safe(a * b) ? a * b + 0 : wrappingProduct(a, b, 128, true)) as i128;
/** `i128`'s `wrapping_div`: `a / b`, truncated and wrapped into the type's range, panicking on a zero divisor. */
export const i128WrappingDiv = (a: i128, b: i128, at?: Location): i128 => (b === -1 ? i128WrappingSub(0, a) : i128Div(a, b, at));
/** `i128`'s `wrapping_rem`: `a % b`, 0 where it would overflow, panicking on a zero divisor. */
export const i128WrappingRem = (a: i128, b: i128, at?: Location): i128 => (b === -1 ? 0 : i128Rem(a, b, at));
/** `i128`'s `saturating_div`: `a / b`, truncated and clamped to the type's range, panicking on a zero divisor. */
export const i128SaturatingDiv = (a: i128, b: i128, at?: Location): i128 => (b === -1 ? i128SaturatingSub(0, a) : i128Div(a, b, at));
/** `i128`'s `leading_ones`: the count of leading one bits. */
export const i128LeadingOnes = (a: i128): number => leadingOnes128(a, true);
/** `i128`'s `leading_zeros`: the count of leading zero bits. */
export const i128LeadingZeros = (a: i128): number => leadingZeros128(a, true);
/** `i128`'s `trailing_zeros`: the count of trailing zero bits. */
export const i128TrailingZeros = (a: i128): number => trailingZeros128(a, true);
/** `i128`'s `from_be_bytes`: a value from its big-endian bytes. */
export const i128FromBeBytes = (bytes: Uint8Array): i128 => fromBeBytes128(bytes, true) as i128;
/** `i128`'s `to_be_bytes`: its big-endian bytes. */
export const i128ToBeBytes = (a: i128): Uint8Array => toBeBytes128(a, true);
/** `i128`'s `try_from` another integer: it, or a `TryFromIntError` past the type's range. */
export const i128TryFrom = (value: number | Limbs): Result<i128, TryFromIntError> => tryFrom(value, 128, true) as Result<i128, TryFromIntError>;
/** `i128`'s `Display`: its decimal digits, padded by the formatter's spec. */
export const i128Display = (a: i128, f: Formatter): FmtResult => wideDisplay(a, f);
/** `i128`'s `LowerHex`, `{:x}`. */
export const i128LowerHex = (a: i128, f: Formatter): FmtResult => wideHex(a, 128, true, f, false);
/** `i128`'s `UpperHex`, `{:X}`. */
export const i128UpperHex = (a: i128, f: Formatter): FmtResult => wideHex(a, 128, true, f, true);
/** `i128`'s `Debug`, `Display` unless `{:x?}` or `{:X?}` makes it hex. */
export const i128Debug = (a: i128, f: Formatter): FmtResult => wideDebug(a, 128, true, f);
/** `i128`'s `Binary`, `{:b}`. */
export const i128Binary = (a: i128, f: Formatter): FmtResult => radix(wideBits(a, 128, true), f, 2, "0b");
/** `i128`'s `Octal`, `{:o}`. */
export const i128Octal = (a: i128, f: Formatter): FmtResult => radix(wideBits(a, 128, true), f, 8, "0o");
/** `i128`'s `LowerExp`, `{:e}`. */
export const i128LowerExp = (a: i128, f: Formatter): FmtResult => wideExp(a, f, "e");
/** `i128`'s `UpperExp`, `{:E}`. */
export const i128UpperExp = (a: i128, f: Formatter): FmtResult => wideExp(a, f, "E");
/** A `i128` from a safe integer, or from a bigint of any size, wrapped into range. */
export const i128Of = (value: number | bigint): i128 => of(value, 128, true) as i128;
/** `value as i128`. */
export const i128Cast = (value: number | Limbs): i128 => (typeof value === "number" && fits(value, true) ? value : cast(value, 128, true)) as i128;
/** The low 32 bits, as a uint32. */
export const i128Low = (a: i128): number => low(a);
/** `i128`'s `a == b`. */
export const i128Eq = (a: i128, b: i128): boolean => eq(a, b);
/** `i128`'s `a != b`. */
export const i128Ne = (a: i128, b: i128): boolean => !eq(a, b);
/** `i128`'s `a < b`. */
export const i128Lt = (a: i128, b: i128): boolean => typeof a === "number" && typeof b === "number" ? a < b : lt(a, b);
/** `i128`'s `a <= b`. */
export const i128Le = (a: i128, b: i128): boolean => typeof a === "number" && typeof b === "number" ? a <= b : !lt(b, a);
/** `i128`'s `a > b`. */
export const i128Gt = (a: i128, b: i128): boolean => typeof a === "number" && typeof b === "number" ? a > b : lt(b, a);
/** `i128`'s `a >= b`. */
export const i128Ge = (a: i128, b: i128): boolean => typeof a === "number" && typeof b === "number" ? a >= b : !lt(a, b);
/** `i128`'s `Ord::max`: the greater, `b` if they are equal. */
export const i128Max = (a: i128, b: i128): i128 => (lt(b, a) ? a : b);
/** `i128`'s `Ord::min`: the lesser, `a` if they are equal. */
export const i128Min = (a: i128, b: i128): i128 => (lt(b, a) ? b : a);
/** `i128`'s `Ord::clamp`: `a` within `[min, max]`, panicking unless `min <= max`. */
export const i128Clamp = (a: i128, min: i128, max: i128, at?: Location): i128 => clampWide(a, min, max, at) as i128;
/** `i128`'s `Step`, which ranges step by: `n` moved by `by`, and whether that overflowed. */
export const i128Step: Step<i128> = (n, by) => stepped(sum128(n, Math.abs(by), by < 0 ? -1 : 1, true)) as [i128, boolean];

// Every width's other inherent methods: one function each, which the call passes the
// type's width and sign as literals, computing on the value's exact bigint. None is on a
// hot path, so a type gets no number path of its own. Where std gives a value of another
// type, as `checked_add_signed` takes, it is any integer.

/** Any integer: a number, or a wide type's limbs. */
type Int = number | Limbs;

/** The type's least value. */
function least(bits: number, signed: boolean): bigint {
	return signed ? -(1n << BigInt(bits - 1)) : 0n;
}

/** The type's greatest value. */
function greatest(bits: number, signed: boolean): bigint {
	return (1n << BigInt(signed ? bits - 1 : bits)) - 1n;
}

/** The exact `v`, wrapped into the type's range. */
function wrappingOf(v: bigint, bits: number, signed: boolean): Int {
	return intFromBig(signed ? BigInt.asIntN(bits, v) : BigInt.asUintN(bits, v), bits, signed);
}

/** The exact `v`, or a panic naming `op` past the type's range. */
function strictOf(v: bigint, bits: number, signed: boolean, op: string, at?: Location): Int {
	if (!fitsBig(v, bits, signed)) panic(`attempt to ${op} with overflow`, at);
	return intFromBig(v, bits, signed);
}

/** The exact `v`, or `undefined` past the type's range. */
function checkedOf(v: bigint, bits: number, signed: boolean): Int | undefined {
	return fitsBig(v, bits, signed) ? intFromBig(v, bits, signed) : undefined;
}

/** The exact `v`, clamped to the type's range. */
function saturatingOf(v: bigint, bits: number, signed: boolean): Int {
	const min = least(bits, signed);
	const max = greatest(bits, signed);
	return intFromBig(v < min ? min : v > max ? max : v, bits, signed);
}

/** The exact `v` wrapped into the type's range, and whether it was past it. */
function overflowingOf(v: bigint, bits: number, signed: boolean): [Int, boolean] {
	return [wrappingOf(v, bits, signed), !fitsBig(v, bits, signed)];
}

/**
 * `a ** exp`: exact while it may be in the type's range, and past that, a value past the
 * range on the side the exact one is, which wraps to the same bits. Past `bits`, an
 * exponent leaves the range unless `a` is -1, 0, or 1.
 */
function power(a: Int, exp: number, bits: number): bigint {
	const v = intToBig(a);
	if (exp < bits || (v >= -1n && v <= 1n)) return v ** BigInt(exp);
	let low = 1n;
	let base = BigInt.asUintN(bits, v);
	for (let e = exp; e > 0; e = Math.floor(e / 2)) {
		if (e % 2 === 1) low = BigInt.asUintN(bits, low * base);
		base = BigInt.asUintN(bits, base * base);
	}
	return v < 0n && exp % 2 === 1 ? low - (2n << BigInt(bits)) : low + (1n << BigInt(bits));
}

/** `a / b`, truncated, or for `euclid` rounded so the remainder is not negative, panicking on a zero divisor. */
function quotientOf(a: Int, b: Int, euclid: boolean, at?: Location): bigint {
	const x = intToBig(a);
	const y = intToBig(b);
	if (y === 0n) panic("attempt to divide by zero", at);
	const q = x / y;
	return euclid && x % y < 0n ? (y > 0n ? q - 1n : q + 1n) : q;
}

/**
 * `a % b`, of `a`'s sign, or for `euclid` not negative, and whether it overflowed: only
 * `MIN % -1` does, whose remainder is 0. Panics on a zero divisor.
 */
function remainderOf(a: Int, b: Int, bits: number, signed: boolean, euclid: boolean, at?: Location): [bigint, boolean] {
	const x = intToBig(a);
	const y = intToBig(b);
	if (y === 0n) panic("attempt to calculate the remainder with a divisor of zero", at);
	const r = x % y;
	const over = y === -1n && x === least(bits, signed);
	return [euclid && r < 0n ? r + (y < 0n ? -y : y) : r, over];
}

/** `|v|`. */
function magnitude(v: bigint): bigint {
	return v < 0n ? -v : v;
}

/** An integer's bits, as unsigned. */
function bitsOf(a: Int, bits: number): bigint {
	return BigInt.asUintN(bits, intToBig(a));
}

/** How many bits `v`, not negative, takes: 0 for 0. */
function bitLength(v: bigint): number {
	return v === 0n ? 0 : v.toString(2).length;
}

/** The index of `v`'s lowest one bit; `v` is not 0. */
function lowestOne(v: bigint): number {
	return bitLength(v & -v) - 1;
}

/** How many one bits `v`, not negative, has. */
function popcount(v: bigint): number {
	let n = 0;
	for (; v !== 0n; v >>= 32n) {
		let w = Number(v & 0xffffffffn);
		w -= (w >>> 1) & 0x55555555;
		w = (w & 0x33333333) + ((w >>> 2) & 0x33333333);
		n += Math.imul((w + (w >>> 4)) & 0x0f0f0f0f, 0x01010101) >>> 24;
	}
	return n;
}

/** `v`'s integer square root; `v` is not negative. */
function sqrt(v: bigint): bigint {
	if (v < 2n) return v;
	// Newton's method from a value at or above the root, which falls to it.
	let x = 1n << BigInt(Math.ceil(bitLength(v) / 2));
	for (;;) {
		const y = (x + v / x) >> 1n;
		if (y >= x) return x;
		x = y;
	}
}

/** `ilog`'s, `ilog2`'s, or `ilog10`'s panic unless `v` is positive. */
function positive(v: bigint, at?: Location): bigint {
	if (v <= 0n) panic("argument of integer logarithm must be positive", at);
	return v;
}

/** The floor of `v`'s logarithm in `base`; `v` is positive and `base` at least 2. */
function log(v: bigint, base: bigint): number {
	let n = 0;
	for (let r = base; r <= v; r *= base) n++;
	return n;
}

/** The next power of two at or above the unsigned `a`, exactly. */
function nextPower(a: Int): bigint {
	const v = intToBig(a);
	return v <= 1n ? 1n : 1n << BigInt(bitLength(v - 1n));
}

/** `pow`: `a ** exp`, panicking on overflow, as a debug build's `pow` does. */
export const intPow = <T extends Int>(a: T, exp: number, bits: number, signed: boolean, at?: Location): T => strictOf(power(a, exp, bits), bits, signed, "exponentiate", at) as T;
/** `checked_pow`: `a ** exp`, or `undefined` on overflow. */
export const intCheckedPow = <T extends Int>(a: T, exp: number, bits: number, signed: boolean): T | undefined => checkedOf(power(a, exp, bits), bits, signed) as T | undefined;
/** `wrapping_pow`: `a ** exp`, wrapped into the type's range. */
export const intWrappingPow = <T extends Int>(a: T, exp: number, bits: number, signed: boolean): T => wrappingOf(power(a, exp, bits), bits, signed) as T;
/** `saturating_pow`: `a ** exp`, clamped to the type's range. */
export const intSaturatingPow = <T extends Int>(a: T, exp: number, bits: number, signed: boolean): T => saturatingOf(power(a, exp, bits), bits, signed) as T;
/** `overflowing_pow`: `a ** exp` wrapped, and whether it overflowed. */
export const intOverflowingPow = <T extends Int>(a: T, exp: number, bits: number, signed: boolean): [T, boolean] => overflowingOf(power(a, exp, bits), bits, signed) as [T, boolean];

/** `strict_add_signed` and `strict_add_unsigned`: `a + b`, panicking on overflow. */
export const intStrictAdd = <T extends Int>(a: T, b: Int, bits: number, signed: boolean, at?: Location): T => strictOf(intToBig(a) + intToBig(b), bits, signed, "add", at) as T;
/** `checked_add_signed` and `checked_add_unsigned`: `a + b`, or `undefined` on overflow. */
export const intCheckedAdd = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): T | undefined => checkedOf(intToBig(a) + intToBig(b), bits, signed) as T | undefined;
/** `wrapping_add_signed` and `wrapping_add_unsigned`: `a + b`, wrapped into the type's range. */
export const intWrappingAdd = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): T => wrappingOf(intToBig(a) + intToBig(b), bits, signed) as T;
/** `saturating_add_signed` and `saturating_add_unsigned`: `a + b`, clamped to the type's range. */
export const intSaturatingAdd = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): T => saturatingOf(intToBig(a) + intToBig(b), bits, signed) as T;
/** `overflowing_add` and its `_signed` and `_unsigned` forms: `a + b` wrapped, and whether it overflowed. */
export const intOverflowingAdd = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): [T, boolean] => overflowingOf(intToBig(a) + intToBig(b), bits, signed) as [T, boolean];
/** `strict_sub_signed` and `strict_sub_unsigned`: `a - b`, panicking on overflow. */
export const intStrictSub = <T extends Int>(a: T, b: Int, bits: number, signed: boolean, at?: Location): T => strictOf(intToBig(a) - intToBig(b), bits, signed, "subtract", at) as T;
/** `checked_sub_signed` and `checked_sub_unsigned`: `a - b`, or `undefined` on overflow. */
export const intCheckedSub = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): T | undefined => checkedOf(intToBig(a) - intToBig(b), bits, signed) as T | undefined;
/** `wrapping_sub_signed` and `wrapping_sub_unsigned`: `a - b`, wrapped into the type's range. */
export const intWrappingSub = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): T => wrappingOf(intToBig(a) - intToBig(b), bits, signed) as T;
/** `saturating_sub_signed` and `saturating_sub_unsigned`: `a - b`, clamped to the type's range. */
export const intSaturatingSub = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): T => saturatingOf(intToBig(a) - intToBig(b), bits, signed) as T;
/** `overflowing_sub` and its `_signed` and `_unsigned` forms: `a - b` wrapped, and whether it overflowed. */
export const intOverflowingSub = <T extends Int>(a: T, b: Int, bits: number, signed: boolean): [T, boolean] => overflowingOf(intToBig(a) - intToBig(b), bits, signed) as [T, boolean];
/** `overflowing_mul`: `a * b` wrapped, and whether it overflowed. */
export const intOverflowingMul = <T extends Int>(a: T, b: T, bits: number, signed: boolean): [T, boolean] => overflowingOf(intToBig(a) * intToBig(b), bits, signed) as [T, boolean];
/** `carrying_add`: `a + b + carry` wrapped, and whether it carried out. */
export const intCarryingAdd = <T extends Int>(a: T, b: T, carry: boolean, bits: number, signed: boolean): [T, boolean] => overflowingOf(intToBig(a) + intToBig(b) + (carry ? 1n : 0n), bits, signed) as [T, boolean];
/** `borrowing_sub`: `a - b - borrow` wrapped, and whether it borrowed. */
export const intBorrowingSub = <T extends Int>(a: T, b: T, borrow: boolean, bits: number, signed: boolean): [T, boolean] => overflowingOf(intToBig(a) - intToBig(b) - (borrow ? 1n : 0n), bits, signed) as [T, boolean];
/** `carrying_mul_add` of an unsigned type: `a * b + c + d`, as its low and high halves. */
export const intCarryingMulAdd = <T extends Int>(a: T, b: T, c: T, d: T, bits: number, signed: boolean): [T, T] => {
	const v = intToBig(a) * intToBig(b) + intToBig(c) + intToBig(d);
	return [wrappingOf(v, bits, signed), intFromBig(v >> BigInt(bits), bits, signed)] as [T, T];
};
/** `carrying_mul` of an unsigned type: `a * b + c`, as its low and high halves. */
export const intCarryingMul = <T extends Int>(a: T, b: T, c: T, bits: number, signed: boolean): [T, T] => intCarryingMulAdd(a, b, c, 0 as T, bits, signed);

/** `checked_div`: `a / b`, truncated, or `undefined` on a zero divisor or overflow. */
export const intCheckedDiv = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T | undefined => (b === 0 ? undefined : (checkedOf(quotientOf(a, b, false, at), bits, signed) as T | undefined));
/** `overflowing_div`: `a / b` truncated and wrapped, and whether it overflowed, panicking on a zero divisor. */
export const intOverflowingDiv = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): [T, boolean] => overflowingOf(quotientOf(a, b, false, at), bits, signed) as [T, boolean];
/** `div_euclid`: the quotient whose remainder is not negative, panicking on a zero divisor or overflow. */
export const intDivEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T => strictOf(quotientOf(a, b, true, at), bits, signed, "divide", at) as T;
/** `checked_div_euclid`: `div_euclid`, or `undefined` on a zero divisor or overflow. */
export const intCheckedDivEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T | undefined => (b === 0 ? undefined : (checkedOf(quotientOf(a, b, true, at), bits, signed) as T | undefined));
/** `wrapping_div_euclid`: `div_euclid`, wrapped into the type's range, panicking on a zero divisor. */
export const intWrappingDivEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T => wrappingOf(quotientOf(a, b, true, at), bits, signed) as T;
/** `overflowing_div_euclid`: `div_euclid` wrapped, and whether it overflowed, panicking on a zero divisor. */
export const intOverflowingDivEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): [T, boolean] => overflowingOf(quotientOf(a, b, true, at), bits, signed) as [T, boolean];
/** `checked_rem`: `a % b`, or `undefined` on a zero divisor or overflow. */
export const intCheckedRem = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T | undefined => {
	if (b === 0) return undefined;
	const [r, over] = remainderOf(a, b, bits, signed, false, at);
	return over ? undefined : (intFromBig(r, bits, signed) as T);
};
/** `overflowing_rem`: `a % b`, 0 where it overflows, and whether it did, panicking on a zero divisor. */
export const intOverflowingRem = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): [T, boolean] => {
	const [r, over] = remainderOf(a, b, bits, signed, false, at);
	return [intFromBig(r, bits, signed) as T, over];
};
/** `rem_euclid`: the remainder that is not negative, panicking on a zero divisor or overflow. */
export const intRemEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T => {
	const [r, over] = remainderOf(a, b, bits, signed, true, at);
	if (over) panic("attempt to calculate the remainder with overflow", at);
	return intFromBig(r, bits, signed) as T;
};
/** `checked_rem_euclid`: `rem_euclid`, or `undefined` on a zero divisor or overflow. */
export const intCheckedRemEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T | undefined => {
	if (b === 0) return undefined;
	const [r, over] = remainderOf(a, b, bits, signed, true, at);
	return over ? undefined : (intFromBig(r, bits, signed) as T);
};
/** `wrapping_rem_euclid`: `rem_euclid`, 0 where it overflows, panicking on a zero divisor. */
export const intWrappingRemEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T => intFromBig(remainderOf(a, b, bits, signed, true, at)[0], bits, signed) as T;
/** `overflowing_rem_euclid`: `rem_euclid`, 0 where it overflows, and whether it did, panicking on a zero divisor. */
export const intOverflowingRemEuclid = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): [T, boolean] => {
	const [r, over] = remainderOf(a, b, bits, signed, true, at);
	return [intFromBig(r, bits, signed) as T, over];
};
/** `next_multiple_of` of an unsigned type: the least multiple of `b` at or above `a`, panicking on a zero `b` or overflow. */
export const intNextMultipleOf = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T => {
	const [r] = remainderOf(a, b, bits, signed, false, at);
	return (r === 0n ? a : strictOf(intToBig(a) + intToBig(b) - r, bits, signed, "add", at)) as T;
};
/** `checked_next_multiple_of` of an unsigned type: `next_multiple_of`, or `undefined` on a zero `b` or overflow. */
export const intCheckedNextMultipleOf = <T extends Int>(a: T, b: T, bits: number, signed: boolean, at?: Location): T | undefined => {
	if (b === 0) return undefined;
	const [r] = remainderOf(a, b, bits, signed, false, at);
	return (r === 0n ? a : checkedOf(intToBig(a) + intToBig(b) - r, bits, signed)) as T | undefined;
};
/** `midpoint`: `(a + b) / 2`, rounded toward zero, without overflow. */
export const intMidpoint = <T extends Int>(a: T, b: T, bits: number, signed: boolean): T => intFromBig((intToBig(a) + intToBig(b)) / 2n, bits, signed) as T;
/** `abs_diff`: `|a - b|`, in the unsigned type of the width. */
export const intAbsDiff = (a: Int, b: Int, bits: number, _signed: boolean): Int => intFromBig(magnitude(intToBig(a) - intToBig(b)), bits, false);
/** `checked_signed_diff` of an unsigned type: `a - b` in the signed type of the width, or `undefined` past its range. */
export const intCheckedSignedDiff = (a: Int, b: Int, bits: number, _signed: boolean): Int | undefined => checkedOf(intToBig(a) - intToBig(b), bits, true);

/** `strict_neg`: `-a`, panicking on overflow. */
export const intStrictNeg = <T extends Int>(a: T, bits: number, signed: boolean, at?: Location): T => strictOf(-intToBig(a), bits, signed, "negate", at) as T;
/** `checked_neg`: `-a`, or `undefined` on overflow. */
export const intCheckedNeg = <T extends Int>(a: T, bits: number, signed: boolean): T | undefined => checkedOf(-intToBig(a), bits, signed) as T | undefined;
/** `wrapping_neg`: `-a`, wrapped into the type's range. */
export const intWrappingNeg = <T extends Int>(a: T, bits: number, signed: boolean): T => wrappingOf(-intToBig(a), bits, signed) as T;
/** `overflowing_neg`: `-a` wrapped, and whether it overflowed. */
export const intOverflowingNeg = <T extends Int>(a: T, bits: number, signed: boolean): [T, boolean] => overflowingOf(-intToBig(a), bits, signed) as [T, boolean];
/** `abs`: `|a|`, panicking on overflow, as a debug build's `abs` does. */
export const intAbs = <T extends Int>(a: T, bits: number, signed: boolean, at?: Location): T => strictOf(magnitude(intToBig(a)), bits, signed, "negate", at) as T;
/** `checked_abs`: `|a|`, or `undefined` on overflow. */
export const intCheckedAbs = <T extends Int>(a: T, bits: number, signed: boolean): T | undefined => checkedOf(magnitude(intToBig(a)), bits, signed) as T | undefined;
/** `wrapping_abs`: `|a|`, wrapped into the type's range. */
export const intWrappingAbs = <T extends Int>(a: T, bits: number, signed: boolean): T => wrappingOf(magnitude(intToBig(a)), bits, signed) as T;
/** `saturating_abs`: `|a|`, clamped to the type's range. */
export const intSaturatingAbs = <T extends Int>(a: T, bits: number, signed: boolean): T => saturatingOf(magnitude(intToBig(a)), bits, signed) as T;
/** `overflowing_abs`: `|a|` wrapped, and whether it overflowed. */
export const intOverflowingAbs = <T extends Int>(a: T, bits: number, signed: boolean): [T, boolean] => overflowingOf(magnitude(intToBig(a)), bits, signed) as [T, boolean];
/** `unsigned_abs`: `|a|`, in the unsigned type of the width. */
export const intUnsignedAbs = (a: Int, bits: number, _signed: boolean): Int => intFromBig(magnitude(intToBig(a)), bits, false);
/** `signum`: -1, 0, or 1, as `a` is negative, zero, or positive. */
export const intSignum = <T extends Int>(a: T, _bits: number, _signed: boolean): T => {
	const v = intToBig(a);
	return (v < 0n ? -1 : v > 0n ? 1 : 0) as T;
};
/** `isqrt`: the integer square root, panicking on a negative `a`. */
export const intIsqrt = <T extends Int>(a: T, bits: number, signed: boolean, at?: Location): T => {
	const v = intToBig(a);
	if (v < 0n) panic("argument of integer square root cannot be negative", at);
	return intFromBig(sqrt(v), bits, signed) as T;
};
/** `checked_isqrt`: the integer square root, or `undefined` for a negative `a`. */
export const intCheckedIsqrt = <T extends Int>(a: T, bits: number, signed: boolean): T | undefined => {
	const v = intToBig(a);
	return v < 0n ? undefined : (intFromBig(sqrt(v), bits, signed) as T);
};
/** `ilog`: the floor of `a`'s logarithm in `base`, panicking unless `base` is at least 2 and `a` is positive. */
export const intIlog = (a: Int, base: Int, _bits: number, _signed: boolean, at?: Location): number => {
	const b = intToBig(base);
	if (b < 2n) panic("base of integer logarithm must be at least 2", at);
	return log(positive(intToBig(a), at), b);
};
/** `ilog2`: the floor of `a`'s base-2 logarithm, panicking unless `a` is positive. */
export const intIlog2 = (a: Int, _bits: number, _signed: boolean, at?: Location): number => bitLength(positive(intToBig(a), at)) - 1;
/** `ilog10`: the floor of `a`'s base-10 logarithm, panicking unless `a` is positive. */
export const intIlog10 = (a: Int, _bits: number, _signed: boolean, at?: Location): number => String(positive(intToBig(a), at)).length - 1;
/** `checked_ilog`: `ilog`, or `undefined` unless `base` is at least 2 and `a` is positive. */
export const intCheckedIlog = (a: Int, base: Int, _bits: number, _signed: boolean): number | undefined => {
	const v = intToBig(a);
	const b = intToBig(base);
	return v <= 0n || b <= 1n ? undefined : log(v, b);
};
/** `checked_ilog2`: `ilog2`, or `undefined` unless `a` is positive. */
export const intCheckedIlog2 = (a: Int, _bits: number, _signed: boolean): number | undefined => {
	const v = intToBig(a);
	return v <= 0n ? undefined : bitLength(v) - 1;
};
/** `checked_ilog10`: `ilog10`, or `undefined` unless `a` is positive. */
export const intCheckedIlog10 = (a: Int, _bits: number, _signed: boolean): number | undefined => {
	const v = intToBig(a);
	return v <= 0n ? undefined : String(v).length - 1;
};

/** `checked_shl`: `a << n`, or `undefined` for a shift by the width or more. */
export const intCheckedShl = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T | undefined => (n < bits ? (wrappingOf(intToBig(a) << BigInt(n), bits, signed) as T) : undefined);
/** `checked_shr`: `a >> n`, or `undefined` for a shift by the width or more. */
export const intCheckedShr = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T | undefined => (n < bits ? (intFromBig(intToBig(a) >> BigInt(n), bits, signed) as T) : undefined);
/** `wrapping_shl`: `a << n`, by `n` masked to the width. */
export const intWrappingShl = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T => wrappingOf(intToBig(a) << BigInt(n & (bits - 1)), bits, signed) as T;
/** `wrapping_shr`: `a >> n`, by `n` masked to the width. */
export const intWrappingShr = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T => intFromBig(intToBig(a) >> BigInt(n & (bits - 1)), bits, signed) as T;
/** `overflowing_shl`: `wrapping_shl`, and whether `n` was the width or more. */
export const intOverflowingShl = <T extends Int>(a: T, n: number, bits: number, signed: boolean): [T, boolean] => [intWrappingShl(a, n, bits, signed), n >= bits];
/** `overflowing_shr`: `wrapping_shr`, and whether `n` was the width or more. */
export const intOverflowingShr = <T extends Int>(a: T, n: number, bits: number, signed: boolean): [T, boolean] => [intWrappingShr(a, n, bits, signed), n >= bits];
/** `unbounded_shl`: `a << n`, 0 for a shift by the width or more. */
export const intUnboundedShl = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T => (n < bits ? (wrappingOf(intToBig(a) << BigInt(n), bits, signed) as T) : (0 as T));
/** `unbounded_shr`: `a >> n`, past the width all sign bits: 0, or -1 for a negative `a`. */
export const intUnboundedShr = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T => intFromBig(intToBig(a) >> BigInt(Math.min(n, bits)), bits, signed) as T;
/** `funnel_shl` of an unsigned type, as `unchecked_funnel_shl` checks it: the high half of `a:b` shifted left by `n`, or `undefined` unless `n` is below the width. */
export const intCheckedFunnelShl = <T extends Int>(a: T, b: T, n: number, bits: number, signed: boolean): T | undefined =>
	n < bits ? (wrappingOf((((intToBig(a) << BigInt(bits)) | intToBig(b)) << BigInt(n)) >> BigInt(bits), bits, signed) as T) : undefined;
/** `funnel_shr` of an unsigned type, as `unchecked_funnel_shr` checks it: the low half of `a:b` shifted right by `n`, or `undefined` unless `n` is below the width. */
export const intCheckedFunnelShr = <T extends Int>(a: T, b: T, n: number, bits: number, signed: boolean): T | undefined =>
	n < bits ? (wrappingOf(((intToBig(a) << BigInt(bits)) | intToBig(b)) >> BigInt(n), bits, signed) as T) : undefined;
/** `funnel_shl` of an unsigned type, panicking unless `n` is below the width. */
export const intFunnelShl = <T extends Int>(a: T, b: T, n: number, bits: number, signed: boolean, at?: Location): T =>
	intCheckedFunnelShl(a, b, n, bits, signed) ?? panic("attempt to funnel shift left with overflow", at);
/** `funnel_shr` of an unsigned type, panicking unless `n` is below the width. */
export const intFunnelShr = <T extends Int>(a: T, b: T, n: number, bits: number, signed: boolean, at?: Location): T =>
	intCheckedFunnelShr(a, b, n, bits, signed) ?? panic("attempt to funnel shift right with overflow", at);
/** `rotate_left`: `a`'s bits rotated left by `n`, modulo the width. */
export const intRotateLeft = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T => {
	const v = bitsOf(a, bits);
	const k = BigInt(n % bits);
	return wrappingOf((v << k) | (v >> (BigInt(bits) - k)), bits, signed) as T;
};
/** `rotate_right`: `a`'s bits rotated right by `n`, modulo the width. */
export const intRotateRight = <T extends Int>(a: T, n: number, bits: number, signed: boolean): T => {
	const v = bitsOf(a, bits);
	const k = BigInt(n % bits);
	return wrappingOf((v >> k) | (v << (BigInt(bits) - k)), bits, signed) as T;
};
/** `swap_bytes`: `a`'s bytes in reverse order, as `to_be` and `from_be` are on wasm32. */
export const intSwapBytes = <T extends Int>(a: T, bits: number, signed: boolean): T => {
	let v = bitsOf(a, bits);
	let r = 0n;
	for (let i = 0; i < bits; i += 8, v >>= 8n) r = (r << 8n) | (v & 0xffn);
	return wrappingOf(r, bits, signed) as T;
};
/** `reverse_bits`: `a`'s bits in reverse order. */
export const intReverseBits = <T extends Int>(a: T, bits: number, signed: boolean): T => {
	let v = bitsOf(a, bits);
	let r = 0n;
	for (let i = 0; i < bits; i++, v >>= 1n) r = (r << 1n) | (v & 1n);
	return wrappingOf(r, bits, signed) as T;
};
/** `count_ones`: how many one bits `a` has. */
export const intCountOnes = (a: Int, bits: number, _signed: boolean): number => popcount(bitsOf(a, bits));
/** `count_zeros`: how many zero bits `a` has. */
export const intCountZeros = (a: Int, bits: number, _signed: boolean): number => bits - popcount(bitsOf(a, bits));
/** `trailing_ones`: how many one bits `a` ends in. */
export const intTrailingOnes = (a: Int, bits: number, _signed: boolean): number => {
	const v = BigInt.asUintN(bits, ~intToBig(a));
	return v === 0n ? bits : lowestOne(v);
};
/** `bit_width`: how many bits `a` takes: the width less its leading zeros. */
export const intBitWidth = (a: Int, bits: number, _signed: boolean): number => bitLength(bitsOf(a, bits));
/** `highest_one`: the index of `a`'s highest one bit, or `undefined` for 0. */
export const intHighestOne = (a: Int, bits: number, _signed: boolean): number | undefined => {
	const v = bitsOf(a, bits);
	return v === 0n ? undefined : bitLength(v) - 1;
};
/** `lowest_one`: the index of `a`'s lowest one bit, or `undefined` for 0. */
export const intLowestOne = (a: Int, bits: number, _signed: boolean): number | undefined => {
	const v = bitsOf(a, bits);
	return v === 0n ? undefined : lowestOne(v);
};
/** `isolate_highest_one`: `a` with only its highest one bit kept. */
export const intIsolateHighestOne = <T extends Int>(a: T, bits: number, signed: boolean): T => {
	const v = bitsOf(a, bits);
	return (v === 0n ? 0 : wrappingOf(1n << BigInt(bitLength(v) - 1), bits, signed)) as T;
};
/** `isolate_lowest_one`: `a` with only its lowest one bit kept. */
export const intIsolateLowestOne = <T extends Int>(a: T, bits: number, signed: boolean): T => {
	const v = bitsOf(a, bits);
	return wrappingOf(v & -v, bits, signed) as T;
};
/** `is_power_of_two` of an unsigned type. */
export const intIsPowerOfTwo = (a: Int, bits: number, _signed: boolean): boolean => popcount(bitsOf(a, bits)) === 1;
/** `next_power_of_two` of an unsigned type: the least power of two at or above `a`, panicking past the type's range, as a debug build's does. */
export const intNextPowerOfTwo = <T extends Int>(a: T, bits: number, signed: boolean, at?: Location): T => strictOf(nextPower(a), bits, signed, "add", at) as T;
/** `checked_next_power_of_two` of an unsigned type: `next_power_of_two`, or `undefined` past the type's range. */
export const intCheckedNextPowerOfTwo = <T extends Int>(a: T, bits: number, signed: boolean): T | undefined => checkedOf(nextPower(a), bits, signed) as T | undefined;
/** `wrapping_next_power_of_two` of an unsigned type: `next_power_of_two`, or 0 past the type's range. */
export const intWrappingNextPowerOfTwo = <T extends Int>(a: T, bits: number, signed: boolean): T => wrappingOf(nextPower(a), bits, signed) as T;

// Casts between integers and floats. An integer of at most 32 bits is already its `f64`;
// a wider one rounds to the nearest, ties to even, once, and so does one to an `f32`. A
// float to an integer truncates toward zero, saturates at the type's bounds, and sends
// NaN to 0; an `f32` does so as the `f64` it is exactly.

/** `x as T` into a type of at most 32 bits, whose bounds are int32s or uint32s. */
function narrowFromF64(x: number, min: number, max: number): number {
	// NaN fails both comparisons; `+ 0` turns a truncated -0 into 0.
	return x >= min ? (x <= max ? Math.trunc(x) + 0 : max) : x < min ? min : 0;
}

/** `x as T` into a 64- or 128-bit type. */
function wideFromF64(x: number, bits: number, signed: boolean): Wide {
	const t = Math.trunc(x) + 0;
	if (fits(t, signed)) return t;
	if (t !== t) return 0;
	// Every integer past 2^53 is exact in a double, and so is each type's bound.
	const bound = 2 ** (signed ? bits - 1 : bits);
	const min = signed ? -bound : 0;
	if (t >= bound) return of((1n << BigInt(signed ? bits - 1 : bits)) - 1n, bits, signed);
	if (t < min) return of(BigInt(min), bits, signed);
	if (bits === 64) {
		const hi = Math.floor(t / TWO32);
		return join(hi, t - hi * TWO32);
	}
	return of(BigInt(t), bits, signed);
}

/** `x as u8`. */
export const u8FromF64 = (x: f64): u8 => narrowFromF64(x, 0, 0xff);
/** `x as u16`. */
export const u16FromF64 = (x: f64): u16 => narrowFromF64(x, 0, 0xffff);
/** `x as u32`. */
export const u32FromF64 = (x: f64): u32 => narrowFromF64(x, 0, 0xffffffff);
/** `x as usize`. */
export const usizeFromF64: (x: f64) => usize = u32FromF64 as never;
/** `x as i8`. */
export const i8FromF64 = (x: f64): i8 => narrowFromF64(x, -0x80, 0x7f);
/** `x as i16`. */
export const i16FromF64 = (x: f64): i16 => narrowFromF64(x, -0x8000, 0x7fff);
/** `x as i32`. */
export const i32FromF64 = (x: f64): i32 => narrowFromF64(x, -0x80000000, 0x7fffffff);
/** `x as isize`. */
export const isizeFromF64: (x: f64) => isize = i32FromF64 as never;
/** `x as u64`. */
export const u64FromF64 = (x: f64): u64 => wideFromF64(x, 64, false) as u64;
/** `x as i64`. */
export const i64FromF64 = (x: f64): i64 => wideFromF64(x, 64, true) as i64;
/** `x as u128`. */
export const u128FromF64 = (x: f64): u128 => wideFromF64(x, 128, false) as u128;
/** `x as i128`. */
export const i128FromF64 = (x: f64): i128 => wideFromF64(x, 128, true) as i128;
/** `a as f64`: limbs' one sum is exact before it rounds. */
export const u64ToF64 = (a: u64): f64 => (typeof a === "number" ? (a as number) : a.hi * TWO32 + a.lo);
/** `a as f64`: limbs' one sum is exact before it rounds. */
export const i64ToF64 = (a: i64): f64 => (typeof a === "number" ? (a as number) : a.hi * TWO32 + a.lo);
/** `a as f64`: a bigint converts to the nearest double. */
export const u128ToF64 = (a: u128): f64 => (typeof a === "number" ? (a as number) : Number(exact(a)));
/** `a as f64`: a bigint converts to the nearest double. */
export const i128ToF64 = (a: i128): f64 => (typeof a === "number" ? (a as number) : Number(exact(a)));

/**
 * `v as f32` past 2^53: its top 53 bits, the last set if any bit below them is, which
 * `Math.fround` then rounds once as if from the exact value, where the nearest double
 * would round twice.
 */
function wideToF32(v: bigint): f32 {
	const m = v < 0n ? -v : v;
	const drop = BigInt(Math.max(0, m.toString(2).length - 53));
	const odd = (m >> drop) | (m & ((1n << drop) - 1n) ? 1n : 0n);
	const x = Math.fround(Number(odd << drop));
	return v < 0n ? -x : x;
}

/** `a as f32`: a safe integer is exact as a double, so rounds once. */
export const u64ToF32 = (a: u64): f32 => (typeof a === "number" ? Math.fround(a) : wideToF32(exact(a)));
/** `a as f32`. */
export const i64ToF32 = (a: i64): f32 => (typeof a === "number" ? Math.fround(a) : wideToF32(exact(a)));
/** `a as f32`. */
export const u128ToF32 = (a: u128): f32 => (typeof a === "number" ? Math.fround(a) : wideToF32(exact(a)));
/** `a as f32`. */
export const i128ToF32 = (a: i128): f32 => (typeof a === "number" ? Math.fround(a) : wideToF32(exact(a)));

/** The 32 bits of an integer from `shift`, a multiple of 32, without making a byte array. */
export function intWord(value: number | Limbs, shift: number): number {
	if (typeof value === "number") return Math.floor(value / 2 ** shift) | 0;
	if (shift === 0) return value.lo;
	return "w1" in value ? (shift === 32 ? value.w1 : shift === 64 ? value.w2 : value.hi) : value.hi;
}
