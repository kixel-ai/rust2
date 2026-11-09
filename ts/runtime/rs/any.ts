// `core::any::TypeId`, for rust2-ts output. `rs.ts` re-exports all of it.

import { type FmtResult, type Formatter, fmtPadIntegral, fmtSpec, fmtWriteFmt, type Spec } from "../rs.ts";
import { cmpStr, type Ordering } from "./collections.ts";
import { type Hasher, hash64 } from "./hash.ts";
import { u64FromHalves } from "./int.ts";

/**
 * `TypeId`: rustc's 128-bit hash of the type, which std's holds at run time, as 32
 * lowercase hex digits, so two are equal by `===`.
 */
export type TypeId = string;

/**
 * `Ord::cmp`, derived over the pointer-sized words std lays the hash out in, lowest first,
 * as wasm32's are: each is 8 hex digits, which compare as their numbers do.
 */
export function typeIdCmp(a: TypeId, b: TypeId): Ordering {
	for (let end = 32; end > 0; end -= 8) {
		const order = cmpStr(a.slice(end - 8, end), b.slice(end - 8, end));
		if (order !== 0) return order;
	}
	return 0;
}

/** `Hash::hash`: the hash's high 64 bits, the second half of its bytes, as one `u64`. */
export function typeIdHash(id: TypeId, s: Hasher): void {
	hash64(u64FromHalves(parseInt(id.slice(0, 8), 16), parseInt(id.slice(8, 16), 16)), s);
}

/**
 * `Debug`, as std's `write!(f, "TypeId({:#034x})", ..)`: `TypeId(`, `0x`, a zero for each
 * leading zero digit, the rest, and `)`, each its own write.
 */
export function fmtTypeIdDebug(id: TypeId, f: Formatter): FmtResult {
	const digits = id.replace(/^0+(?=.)/, "");
	return fmtWriteFmt(f, ["TypeId(", fmtSpec(HASH, (g) => fmtPadIntegral(g, true, "0x", digits)), ")"]);
}

/** `{:#034x}`. */
const HASH: Spec = { alternate: true, zero: true, width: 34 };
