// `crates_dep::Slots`, which writes into the array it is given.

import * as rs from "../rs.ts";

export type Slots = { base: rs.i16 };

export function withBase(base: rs.i16): Slots {
	return { base };
}

export function fill(self: Slots, out: rs.Slice<rs.i16>): undefined {
	out.data[out.start] = self.base;
	out.data[out.start + 1] = rs.i16Sub(self.base, 1 as rs.i16);
}
