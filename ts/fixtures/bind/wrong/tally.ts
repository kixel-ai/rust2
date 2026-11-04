// `raw::Tally`, bound wrong: it drops, but has no `[Symbol.dispose]`.

import type * as rs from "../rs.ts";

export interface Tally {
	count: rs.Arc<rs.u32>;
}

function make(count: rs.Arc<rs.u32>): Tally {
	return { count };
}
export { make as new };

export function get(self: Tally): rs.u32 {
	return self.count.value;
}
