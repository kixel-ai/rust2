// `key::Nibble`, a byte that compares, hashes, and orders by its low four bits, which the
// `===` of its number would not.

import * as rs from "../rs.ts";

export type Nibble = rs.u8;

function make(byte: rs.u8): Nibble {
	return byte;
}
export { make as new };

export function get(self: Nibble): rs.u8 {
	return self;
}

export function eq$cmp_PartialEq(self: Nibble, other: Nibble): boolean {
	return (self & 0xf) === (other & 0xf);
}

export function hash$Hash(self: Nibble, state: rs.Hasher): undefined {
	rs.hash8(self & 0xf, state);
	return undefined;
}

export function partialCmp$cmp_PartialOrd(self: Nibble, other: Nibble): rs.Ordering {
	return cmp$cmp_Ord(self, other);
}

export function cmp$cmp_Ord(self: Nibble, other: Nibble): rs.Ordering {
	return rs.cmp(self & 0xf, other & 0xf);
}
