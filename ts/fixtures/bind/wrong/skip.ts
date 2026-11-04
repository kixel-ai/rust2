// `raw::skip`, bound wrong: it returns the new position, where Rust returns `()`.

import type * as rs from "../rs.ts";

export function skip(bytes: rs.ByteSlice, at: rs.Place<rs.usize>): rs.usize {
	while (at.v < bytes.length && bytes.data[bytes.start + at.v] === 0) at.v++;
	return at.v;
}
