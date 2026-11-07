// `rand`'s thread-local generator, for rust2-ts output: each value comes from the
// platform's cryptographic source, `crypto.getRandomValues`, as `rand::rng()` seeds from
// the OS's. The values are uniform as `rand`'s are, but not its stream: no seed reproduces
// them on either side. `rs.ts` re-exports all of it.

import { panic, type RangeInclusive } from "../rs.ts";
import { intFromBig, intToBig, type Limbs } from "./int.ts";
import type { Location } from "./location.ts";

/** `rand::rngs::ThreadRng`: a handle to the platform's source, which holds no state. */
export class ThreadRng {}

/** `rand::rng()`. A handle is made each time, as a bundler keeps a constant object. */
export function randRng(): ThreadRng {
	return new ThreadRng();
}

/** `bits` uniform random bits, as a nonnegative bigint. */
function randomBits(bits: number): bigint {
	const words = crypto.getRandomValues(new Uint32Array(Math.ceil(bits / 32)));
	let v = 0n;
	for (const word of words) v = (v << 32n) | BigInt(word);
	return v & ((1n << BigInt(bits)) - 1n);
}

/** A uniform value below `span`, which is positive: the fewest bits that hold it, redrawn past it. */
function below(span: bigint): bigint {
	const bits = (span - 1n).toString(2).length;
	for (;;) {
		const v = randomBits(bits);
		if (v < span) return v;
	}
}

/** A draw of `randBelow`, reused so a draw makes no array. */
const WORD = /* @__PURE__ */ new Uint32Array(1);

/** A uniform `u32` below `n`, which is positive: a word redrawn past the last whole multiple of `n` below 2^32. */
export function randBelow(n: number): number {
	const limit = 2 ** 32 - (2 ** 32 % n);
	for (;;) {
		crypto.getRandomValues(WORD);
		if (WORD[0] < limit) return WORD[0] % n;
	}
}

/** `random::<T>()` of an integer type of `bits`, signed or not: each value equally likely. */
export function randInt(bits: number, signed: boolean): number | Limbs {
	const v = randomBits(bits);
	return intFromBig(signed && v >> BigInt(bits - 1) ? v - (1n << BigInt(bits)) : v, bits, signed);
}

/** `random::<bool>()`. */
export function randBool(): boolean {
	return randomBits(1) === 1n;
}

/** `random_range(lo..hi)` of an integer type of `bits`, panicking on an empty range as `rand` does. */
export function randRange(lo: number | Limbs, hi: number | Limbs, bits: number, signed: boolean, at?: Location): number | Limbs {
	const [l, h] = [intToBig(lo), intToBig(hi)];
	if (l >= h) panic("cannot sample empty range", at);
	return intFromBig(l + below(h - l), bits, signed);
}

/** `random_range(lo..=hi)`, from its bounds alone, as `rand` reads even an exhausted range. */
export function randRangeInclusive<T extends number | Limbs>(range: RangeInclusive<T>, bits: number, signed: boolean, at?: Location): number | Limbs {
	const [l, h] = [intToBig(range.lo), intToBig(range.hi)];
	if (l > h) panic("cannot sample empty range", at);
	return intFromBig(l + below(h - l + 1n), bits, signed);
}

/** `random_range(..=hi)` of an unsigned integer type of `bits`, which is never empty. */
export function randRangeToInclusive(hi: number | Limbs, bits: number): number | Limbs {
	return intFromBig(below(intToBig(hi) + 1n), bits, false);
}
