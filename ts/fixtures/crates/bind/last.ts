// `crates_dep::last`, which reads through a raw pointer.

import * as rs from "../rs.ts";

export function last(bytes: rs.ByteSlice): rs.u8 {
	if (bytes.length === 0) rs.panic("no last byte");
	return bytes.data[bytes.start + bytes.length - 1] as rs.u8;
}
