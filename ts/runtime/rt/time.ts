// `core::time::Duration`, for rust2-ts output: std's whole seconds and nanoseconds, with
// each method its own function, `durationAdd`, so a bundler drops every one translated
// code never calls. `rt.ts` re-exports all of it.
//
// A `Duration` never changes once made, so values share it. Its arithmetic is std's: a
// sum past `u64::MAX` seconds is `None`, saturates, or panics with std's message.

import { Err, type FmtResult, type Formatter, fmtDebugStructFields, fmtPad, Ok, panic, type Result } from "../rt.ts";
import { cmp, type Ordering } from "./collections.ts";
import { type Hasher, hash32, hash64 } from "./hash.ts";
import { f64ToBits } from "./float.ts";
import { type f32, type f64, type u32, type u64, type u128, u64CheckedAdd, u64CheckedMul, u64CheckedSub, u64Div, u64Eq, u64Of, u64Rem, u64ToF32, u64ToF64, u64TryFrom, u128Add, u128Cast, u128Div, u128Mul, u128Rem } from "./int.ts";
import { type Iter, iterFold } from "./iter.ts";
import type { Location } from "./location.ts";

/** `core::time::Duration`: whole seconds, and the nanoseconds past them, below 10^9. */
export interface Duration {
	readonly secs: u64;
	readonly nanos: u32;
}

const NANOS_PER_SEC = 1_000_000_000;
const MAX_SAFE = 0x1fffffffffffff;

/** `Duration::MAX`, whose seconds are `u64::MAX`'s limbs. A function, as a bundler keeps a constant object. */
function max(): Duration {
	return { secs: { hi: 0xffffffff, lo: 0xffffffff } as u64, nanos: NANOS_PER_SEC - 1 };
}

/** `Duration::new`: `nanos` past a second carries into `secs`, panicking past `u64::MAX`. */
export function durationNew(secs: u64, nanos: u32): Duration {
	if (nanos < NANOS_PER_SEC) return { secs, nanos };
	const carried = u64CheckedAdd(secs, Math.floor(nanos / NANOS_PER_SEC));
	if (carried === undefined) panic("adding carry-over seconds from nanoseconds should not exceed u64::MAX", "duration_new", true);
	return { secs: carried, nanos: nanos % NANOS_PER_SEC };
}

/** `Duration::from_secs`. */
export function durationFromSecs(secs: u64): Duration {
	return { secs, nanos: 0 };
}

/** `Duration::from_millis`. */
export function durationFromMillis(millis: u64, at?: Location): Duration {
	return { secs: u64Div(millis, 1000, at), nanos: (u64Rem(millis, 1000, at) as number) * 1_000_000 };
}

/** `Duration::from_micros`. */
export function durationFromMicros(micros: u64, at?: Location): Duration {
	return { secs: u64Div(micros, 1_000_000, at), nanos: (u64Rem(micros, 1_000_000, at) as number) * 1000 };
}

/** `Duration::from_nanos`. */
export function durationFromNanos(nanos: u64, at?: Location): Duration {
	return { secs: u64Div(nanos, NANOS_PER_SEC, at), nanos: u64Rem(nanos, NANOS_PER_SEC, at) as number };
}

/** `Duration::from_mins`, panicking past `u64::MAX` seconds. */
export function durationFromMins(mins: u64): Duration {
	return { secs: u64CheckedMul(mins, 60 as u64) ?? panic("overflow in Duration::from_mins", "duration_from_mins"), nanos: 0 };
}

/** `Duration::from_hours`, panicking past `u64::MAX` seconds. */
export function durationFromHours(hours: u64): Duration {
	return { secs: u64CheckedMul(hours, 3600 as u64) ?? panic("overflow in Duration::from_hours", "duration_from_hours"), nanos: 0 };
}

/** `Duration::from_nanos_u128`, panicking past `u64::MAX` seconds. */
export function durationFromNanosU128(nanos: u128, at?: Location): Duration {
	const secs = u64TryFrom(u128Div(nanos, NANOS_PER_SEC, at));
	if (!secs.ok) panic("overflow in `Duration::from_nanos_u128`", at);
	return { secs: secs.value, nanos: u128Rem(nanos, NANOS_PER_SEC, at) as number };
}

/** `Duration::is_zero`. */
export function durationIsZero(d: Duration): boolean {
	return d.secs === 0 && d.nanos === 0;
}

/** `Duration::as_secs`: the whole seconds. */
export function durationAsSecs(d: Duration): u64 {
	return d.secs;
}

/** `Duration::subsec_millis`. */
export function durationSubsecMillis(d: Duration): u32 {
	return Math.floor(d.nanos / 1_000_000);
}

/** `Duration::subsec_micros`. */
export function durationSubsecMicros(d: Duration): u32 {
	return Math.floor(d.nanos / 1000);
}

/** `Duration::subsec_nanos`. */
export function durationSubsecNanos(d: Duration): u32 {
	return d.nanos;
}

/** `secs * per + sub` as a `u128`, a number while that is a safe integer. */
function total(secs: u64, per: number, sub: number, at?: Location): u128 {
	if (typeof secs === "number") {
		// Exact up to 2^53, and past it rounded to no less than 2^53.
		const n = secs * per + sub;
		if (n <= MAX_SAFE) return n;
	}
	return u128Add(u128Mul(u128Cast(secs), per, at), sub, at);
}

/** `Duration::as_millis`. */
export function durationAsMillis(d: Duration, at?: Location): u128 {
	return total(d.secs, 1000, Math.floor(d.nanos / 1_000_000), at);
}

/** `Duration::as_micros`. */
export function durationAsMicros(d: Duration, at?: Location): u128 {
	return total(d.secs, 1_000_000, Math.floor(d.nanos / 1000), at);
}

/** `Duration::as_nanos`. */
export function durationAsNanos(d: Duration, at?: Location): u128 {
	return total(d.secs, NANOS_PER_SEC, d.nanos, at);
}

/** `Duration::checked_add`: `a + b`, or `undefined` past `u64::MAX` seconds. */
export function durationCheckedAdd(a: Duration, b: Duration): Duration | undefined {
	let secs = u64CheckedAdd(a.secs, b.secs);
	if (secs === undefined) return undefined;
	let nanos = a.nanos + b.nanos;
	if (nanos >= NANOS_PER_SEC) {
		nanos -= NANOS_PER_SEC;
		secs = u64CheckedAdd(secs, 1);
		if (secs === undefined) return undefined;
	}
	return { secs, nanos };
}

/** `Duration::checked_sub`: `a - b`, or `undefined` below zero. */
export function durationCheckedSub(a: Duration, b: Duration): Duration | undefined {
	let secs = u64CheckedSub(a.secs, b.secs);
	if (secs === undefined) return undefined;
	if (a.nanos >= b.nanos) return { secs, nanos: a.nanos - b.nanos };
	secs = u64CheckedSub(secs, 1);
	if (secs === undefined) return undefined;
	return { secs, nanos: a.nanos + NANOS_PER_SEC - b.nanos };
}

/** `Duration::checked_mul` by a `u32`: `d * n`, or `undefined` past `u64::MAX` seconds. */
export function durationCheckedMul(d: Duration, n: u32): Duration | undefined {
	// The nanoseconds' product is below 2^62, past a safe integer only for large factors.
	let carry: number;
	let nanos: number;
	const product = d.nanos * n;
	if (product <= MAX_SAFE) {
		carry = Math.floor(product / NANOS_PER_SEC);
		nanos = product % NANOS_PER_SEC;
	} else {
		const exact = BigInt(d.nanos) * BigInt(n);
		carry = Number(exact / 1_000_000_000n);
		nanos = Number(exact % 1_000_000_000n);
	}
	const secs = u64CheckedMul(d.secs, n as u64);
	if (secs === undefined) return undefined;
	const carried = u64CheckedAdd(secs, carry);
	if (carried === undefined) return undefined;
	return { secs: carried, nanos };
}

/** `Duration::checked_div` by a `u32`: `d / n`, truncated, or `undefined` if `n` is zero. */
export function durationCheckedDiv(d: Duration, n: u32, at?: Location): Duration | undefined {
	if (n === 0) return undefined;
	const secs = u64Div(d.secs, n as u64, at);
	// What `secs` leaves is below `n`, so its nanoseconds are below 2^62.
	const left = u64Rem(d.secs, n as u64, at) as number;
	const rest = left * NANOS_PER_SEC + (d.nanos % n);
	const nanos = Math.floor(d.nanos / n) + (rest <= MAX_SAFE ? Math.floor(rest / n) : Number((BigInt(left) * 1_000_000_000n + BigInt(d.nanos % n)) / BigInt(n)));
	return { secs, nanos };
}

/** `Duration::saturating_add`: `a + b`, or `Duration::MAX` past it. */
export function durationSaturatingAdd(a: Duration, b: Duration): Duration {
	return durationCheckedAdd(a, b) ?? max();
}

/** `Duration::saturating_sub`: `a - b`, or zero below it. */
export function durationSaturatingSub(a: Duration, b: Duration): Duration {
	return durationCheckedSub(a, b) ?? { secs: 0, nanos: 0 };
}

/** `Duration::saturating_mul` by a `u32`: `d * n`, or `Duration::MAX` past it. */
export function durationSaturatingMul(d: Duration, n: u32): Duration {
	return durationCheckedMul(d, n) ?? max();
}

/** `Duration::abs_diff`: the distance between `a` and `b`. */
export function durationAbsDiff(a: Duration, b: Duration): Duration {
	return durationCheckedSub(a, b) ?? (durationCheckedSub(b, a) as Duration);
}

/** `core::time::TryFromFloatSecsError`: why seconds made no `Duration`. */
export type TryFromFloatSecsError = { readonly kind: TryFromFloatSecsErrorKind };

/** `core::time::TryFromFloatSecsErrorKind`: why a `Duration` wasn't made, by variant name. */
export type TryFromFloatSecsErrorKind = "Negative" | "OverflowOrNan";

/** `Duration::as_secs_f64`. */
export function durationAsSecsF64(d: Duration): f64 {
	return u64ToF64(d.secs) + d.nanos / NANOS_PER_SEC;
}

/** `Duration::div_duration_f64`: their nanoseconds, each rounded to an `f64`, divided. */
export function durationDivDurationF64(a: Duration, b: Duration): f64 {
	return (u64ToF64(a.secs) * NANOS_PER_SEC + a.nanos) / (u64ToF64(b.secs) * NANOS_PER_SEC + b.nanos);
}

/** `n`, the nanoseconds past a second times 2^`shift`, rounded to the nearest, half to even. */
function roundNanos(n: bigint, shift: bigint): number {
	const nanos = Number(n >> shift);
	const rem = n & ((1n << shift) - 1n);
	const half = 1n << (shift - 1n);
	return rem > half || (rem === half && (nanos & 1) === 1) ? nanos + 1 : nanos;
}

/**
 * `Duration::try_from_secs_f64`: `secs` to the nearest nanosecond, half to even, by
 * std's integer algorithm on its bits.
 */
export function durationTryFromSecsF64(secs: f64): Result<Duration, TryFromFloatSecsError> {
	if (secs < 0) return Err({ kind: "Negative" });
	const bits = f64Bits(secs);
	const mantMask = (1n << 52n) - 1n;
	const mant = (bits & mantMask) | (mantMask + 1n);
	const exp = Number((bits >> 52n) & 0x7ffn) - 1023;
	let whole: bigint;
	let nanos: number;
	if (exp < -31) {
		// Less than a nanosecond, which can't round up to one.
		[whole, nanos] = [0n, 0];
	} else if (exp < 0) {
		nanos = roundNanos(BigInt(NANOS_PER_SEC) * (mant << BigInt(44 + exp)), 96n);
		[whole, nanos] = nanos === NANOS_PER_SEC ? [1n, 0] : [0n, nanos];
	} else if (exp < 52) {
		whole = mant >> BigInt(52 - exp);
		nanos = roundNanos(BigInt(NANOS_PER_SEC) * ((mant << BigInt(exp)) & mantMask), 52n);
		if (nanos === NANOS_PER_SEC) [whole, nanos] = [whole + 1n, 0];
	} else if (exp < 64) {
		[whole, nanos] = [mant << BigInt(exp - 52), 0];
	} else {
		return Err({ kind: "OverflowOrNan" });
	}
	return Ok({ secs: u64Of(whole), nanos });
}

/** A double's bits. */
function f64Bits(x: number): bigint {
	const v = f64ToBits(x);
	return typeof v === "number" ? BigInt(v) : (BigInt(v.hi) << 32n) | BigInt(v.lo);
}

/** `Duration::from_secs_f64`, panicking where `try_from_secs_f64` errs. */
export function durationFromSecsF64(secs: f64, at?: Location): Duration {
	const r = durationTryFromSecsF64(secs);
	return r.ok ? r.value : panic(tryFromFloatSecsErrorText(r.error), at, true);
}

/** `Duration::mul_f64`: through seconds as an `f64`, as std computes it. */
export const durationMulF64 = (d: Duration, rhs: f64, at?: Location): Duration => durationFromSecsF64(rhs * durationAsSecsF64(d), at);

/** `Duration::div_f64`: through seconds as an `f64`, as std computes it. */
export const durationDivF64 = (d: Duration, rhs: f64, at?: Location): Duration => durationFromSecsF64(durationAsSecsF64(d) / rhs, at);

/** `Duration::as_secs_f32`, each step rounded to a single as std's. */
export function durationAsSecsF32(d: Duration): f32 {
	return Math.fround(u64ToF32(d.secs) + Math.fround(Math.fround(d.nanos) / NANOS_PER_SEC));
}

/** `Duration::div_duration_f32`: their nanoseconds, each made in singles as std's, divided. */
export function durationDivDurationF32(a: Duration, b: Duration): f32 {
	const nanos = (d: Duration) => Math.fround(Math.fround(u64ToF32(d.secs) * NANOS_PER_SEC) + Math.fround(d.nanos));
	return Math.fround(nanos(a) / nanos(b));
}

/**
 * `Duration::try_from_secs_f32`, which rounds the exact value to the nearest nanosecond,
 * half to even, and errs where `try_from_secs_f64` of the same value does.
 */
export const durationTryFromSecsF32: (secs: f32) => Result<Duration, TryFromFloatSecsError> = durationTryFromSecsF64 as never;

/** `Duration::from_secs_f32`, with `from_secs_f64`'s panic. */
export const durationFromSecsF32: (secs: f32, at?: Location) => Duration = durationFromSecsF64 as never;

/** `Duration::mul_f32`, which std computes as `mul_f64`. */
export const durationMulF32: (d: Duration, rhs: f32, at?: Location) => Duration = durationMulF64 as never;

/** `Duration::div_f32`, which std computes as `div_f64`. */
export const durationDivF32: (d: Duration, rhs: f32, at?: Location) => Duration = durationDivF64 as never;

function tryFromFloatSecsErrorText(e: TryFromFloatSecsError): string {
	const why = e.kind === "Negative" ? "value is negative" : "value is either too big or NaN";
	return `cannot convert float seconds to Duration: ${why}`;
}

/** `Display` of `TryFromFloatSecsError`. */
export const fmtTryFromFloatSecsErrorDisplay = (e: TryFromFloatSecsError, f: Formatter): FmtResult => fmtPad(f, tryFromFloatSecsErrorText(e));

/** `Debug` of `TryFromFloatSecsError`. */
export const fmtTryFromFloatSecsErrorDebug = (e: TryFromFloatSecsError, f: Formatter): FmtResult =>
	fmtDebugStructFields(f, "TryFromFloatSecsError", ["kind"], [(f) => f.out.writeStr(e.kind)]);

/** `a + b`, panicking past `u64::MAX` seconds. */
export function durationAdd(a: Duration, b: Duration): Duration {
	return durationCheckedAdd(a, b) ?? panic("adding durations should not exceed u64::MAX", "duration_add", true);
}

/** `a - b`, panicking below zero. */
export function durationSub(a: Duration, b: Duration): Duration {
	return durationCheckedSub(a, b) ?? panic("subtracting durations should not be negative", "duration_sub", true);
}

/** `d * n` by a `u32`, panicking past `u64::MAX` seconds. */
export function durationMul(d: Duration, n: u32): Duration {
	return durationCheckedMul(d, n) ?? panic("multiplying duration by scalar should not exceed u64::MAX", "duration_mul", true);
}

/** `n * d`, which std computes as `d * n`. */
export function durationTimes(n: u32, d: Duration): Duration {
	return durationMul(d, n);
}

/** `d / n` by a `u32`, truncated, panicking if `n` is zero. */
export function durationDiv(d: Duration, n: u32, at?: Location): Duration {
	return durationCheckedDiv(d, n, at) ?? panic("divisor should not be zero", at, true);
}

/**
 * `Sum`, as std sums: the seconds as it goes, and the nanoseconds in a `u64` that carries
 * into them only when it would overflow and at the end, each overflow past `u64::MAX`
 * seconds a panic.
 */
export function durationSum(it: Iter<Duration>, drops = false, at?: Location): Duration {
	const overflow = "sum over duration iterator should not overflow u64::MAX";
	let secs = 0 as u64;
	let nanos = 0 as u64;
	iterFold<Duration, void>(
		it,
		undefined,
		(_, d) => {
			secs = u64CheckedAdd(secs, d.secs) ?? panic(overflow, at, true);
			const sum = u64CheckedAdd(nanos, d.nanos as u64);
			if (sum !== undefined) {
				nanos = sum;
			} else {
				secs = u64CheckedAdd(secs, u64Div(nanos, NANOS_PER_SEC, at)) ?? panic(overflow, at, true);
				nanos = (u64Rem(nanos, NANOS_PER_SEC, at) as number) + d.nanos;
			}
		},
		drops,
	);
	secs = u64CheckedAdd(secs, u64Div(nanos, NANOS_PER_SEC, at)) ?? panic(overflow, at, true);
	return { secs, nanos: u64Rem(nanos, NANOS_PER_SEC, at) as number };
}

/** `PartialEq::eq`. */
export function durationEq(a: Duration, b: Duration): boolean {
	return a.nanos === b.nanos && u64Eq(a.secs, b.secs);
}

/** `Ord::cmp`: by seconds, then nanoseconds. */
export function durationCmp(a: Duration, b: Duration): Ordering {
	return cmp(a.secs, b.secs) || cmp(a.nanos, b.nanos);
}

/** `Hash::hash`: its seconds, then its nanoseconds, as the derive hashes them. */
export function durationHash(d: Duration, s: Hasher): void {
	hash64(d.secs, s);
	hash32(d.nanos, s);
}

/** `Debug`, as std writes it: `1.5s`, `20ms`, `3µs`, or `0ns`, padded to the width. */
export function durationDebug(d: Duration, f: Formatter): FmtResult {
	const prefix = f.spec.plus ? "+" : "";
	if (d.secs !== 0) return decimal(f, d.secs, d.nanos, NANOS_PER_SEC / 10, prefix, "s");
	if (d.nanos >= 1_000_000) return decimal(f, Math.floor(d.nanos / 1_000_000), d.nanos % 1_000_000, 100_000, prefix, "ms");
	if (d.nanos >= 1000) return decimal(f, Math.floor(d.nanos / 1000), d.nanos % 1000, 100, prefix, "µs");
	return decimal(f, d.nanos as u64, 0, 1, prefix, "ns");
}

/**
 * std's `fmt_decimal`: `whole`, then the digits of `fraction` from `divisor` down, rounded
 * half to even to the precision or else up to the last nonzero one.
 */
function decimal(f: Formatter, whole: u64, fraction: number, divisor: number, prefix: string, unit: string): FmtResult {
	const precision = f.spec.precision;
	let digits = "";
	while (fraction > 0 && digits.length < (precision ?? 9)) {
		digits += Math.floor(fraction / divisor);
		fraction %= divisor;
		divisor /= 10;
	}
	let carry = false;
	if (fraction > 0 && fraction >= divisor * 5) {
		const last = digits.length > 0 ? Number(digits[digits.length - 1]) : typeof whole === "number" ? whole : whole.lo;
		if (fraction > divisor * 5 || last % 2 === 1) {
			const nines = /9*$/.exec(digits)![0].length;
			const pos = digits.length - nines - 1;
			carry = pos < 0;
			digits = (carry ? "" : digits.slice(0, pos) + (Number(digits[pos]) + 1)) + "0".repeat(nines);
		}
	}
	const big = typeof whole === "number" ? BigInt(whole) : (BigInt(whole.hi) << 32n) | BigInt(whole.lo);
	const int = carry ? String(big + 1n) : String(big);
	const point = Math.min(precision ?? digits.length, 9) > 0 ? `.${digits.padEnd(precision ?? digits.length, "0")}` : "";
	// A width pads the whole text, which the precision has already cut.
	const uncut = precision === undefined ? f : { out: f.out, spec: { ...f.spec, precision: undefined } };
	return fmtPad(uncut, `${prefix}${int}${point}${unit}`);
}
