// `raw::Tally`, which holds its `Arc` by a raw pointer, and drops it where Rust drops
// the tally.

import * as rs from "../rs.ts";

export class Tally implements Disposable {
	readonly count: rs.Arc<rs.u32>;

	constructor(count: rs.Arc<rs.u32>) {
		this.count = rs.arcClone(count);
	}

	[Symbol.dispose](): void {
		rs.arcDrop(this.count);
	}
}

function make(count: rs.Arc<rs.u32>): Tally {
	return new Tally(count);
}
export { make as new };

export function get(self: Tally): rs.u32 {
	return self.count.value;
}
