// The inputs `drops`' cases name, which run with the generated ones.

import { Logged } from "./drops.Logged.ts";
import { op } from "./lang.ts";
import * as rs from "./rs.ts";

export { ops, run } from "./lang.ts";

/** `drops::drops`: each shape, by each pick of its variants and each way it drops. */
const dropsCases = Array.from({ length: 12 * 16 * 12 }, (_, i) => Uint8Array.of(...op("drops::drops"), i % 12, Math.floor(i / 12) % 16, Math.floor(i / 192)));

/** `drops::if_lets`: each element type, by each variant of the temporary. */
const ifLetsCases = Array.from({ length: 3 * 5 }, (_, i) => Uint8Array.of(...op("drops::if_lets"), i % 3, Math.floor(i / 3)));

export const cases: Uint8Array[] = [...dropsCases, ...ifLetsCases];

// A consumer's `using` runs the drop the translation defines, as `Symbol.dispose` is on an engine
// that has it and on one that lacks it, as stable Safari does: a drop at the end of the block, once.
{
	const log = new rs.RefCell(new rs.ByteVec());
	const logged = (): number => log.value.length;
	{
		using _logged = new Logged(7, log);
		if (logged() !== 0) throw new Error("a `using` value dropped before its scope ended");
	}
	if (logged() !== 1 || log.value.data[0] !== 7) throw new Error("a `using` value did not drop at the end of its scope");
}
