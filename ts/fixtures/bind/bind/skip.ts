// `raw::skip`, which reads through a raw pointer.

import type * as rs from "../rs.ts";

export function skip(bytes: rs.ByteSlice, at: rs.Place<rs.usize>): undefined {
	while (at.v < bytes.length && bytes.data[bytes.start + at.v] === 0) at.v++;
}
