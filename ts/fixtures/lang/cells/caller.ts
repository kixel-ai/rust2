// The inputs `cells`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `cells` scripts of (op, operand) pairs on every kind, so each cell and value type: reads
 * and sets, a `None` and a `()` held, initializing from within initialization and
 * panicking in it, empty and full, and a cell replaced; clones, `Debug`, and `get_mut`,
 * empty and full; and `get_or_try_init` failing, succeeding, set, and set or initialized
 * again from within, failing after or not.
 */
const cellsOp = op("cells::cells");
const cellScripts = [
	[0, 0, 1, 5, 0, 0, 1, 6, 2, 7, 4, 0, 0, 0, 4, 0, 2, 9, 0, 0],
	[1, 3, 0, 0, 1, 4, 4, 0, 1, 6, 2, 3],
	[3, 4, 3, 5],
	[5, 3],
	[6, 3],
	[7, 1],
	[2, 1, 5, 3, 6, 3, 7, 1],
	[8, 4, 0, 0, 8, 5, 0, 0, 1, 3, 8, 6],
	[9, 0, 9, 1, 0, 0],
	[10, 0, 10, 1, 11, 2, 0, 0],
	[11, 1, 2, 1, 11, 4, 0, 0, 4, 0, 11, 5],
	[12, 0, 12, 6, 0, 0, 12, 1],
	[12, 1, 12, 2],
	[12, 2],
	[12, 3, 4, 0, 12, 7],
	[12, 11],
	[12, 7, 12, 4],
	[],
];
const cells = Array.from({ length: 256 }, (_, kind) => cellScripts.map((script) => Uint8Array.of(...cellsOp, kind, ...script))).flat();

export const cases: Uint8Array[] = [...cells];
