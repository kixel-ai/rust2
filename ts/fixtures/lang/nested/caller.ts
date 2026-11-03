// The inputs `nested`'s cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `nested::consumers`, whose first byte is also `nth`'s index: of no items, one `Some` or
 * `None`, `None` before `Some`, `nth` past the end, only `None`s, keys all tied, and
 * `find` stopping early.
 */
const consumers = [[], [6], [5], [1, 2], [7, 2], [1, 3, 5], [2, 6, 10, 14], [4, 3, 0x90, 0x92, 5]].map((bytes) => Uint8Array.of(...op("nested::nested"), 3, ...bytes));

/**
 * `nested::closures`, whose bytes `pick` each `None` (0 mod 3), `Some(None)` (1), or
 * `Some(Some(_))` (2): of no bytes, only `None`s, only `Some(None)`s, `Some(None)` before
 * a `None` and after it, a seed of each, and a `successors` whose size hint reserves past
 * `isize::MAX`.
 */
const closures = [[], [3], [0, 3], [1], [4, 7, 1], [1, 2, 0, 4, 5], [4, 0, 1, 3, 5], [2, 1, 4, 3, 8], [5, 4], [0, 1, 2], [0xf0], [0xf1]].map((bytes) => Uint8Array.of(...op("nested::nested"), 5, ...bytes));

/**
 * `nested::elements`, whose first byte is also the index and whose odd bytes are each a
 * `None` element: of no elements, one of each, only `None`s, and an index in range and
 * past the end of each mix.
 */
const elements = [[], [1], [2], [1, 3], [3, 1, 5], [0, 1, 2], [2, 6, 1, 8], [5, 4, 3, 0x10, 7, 0x40], [7, 0x22, 9, 0x22, 1, 0x80, 3, 6]].map((bytes) => Uint8Array.of(...op("nested::nested"), 6, ...bytes));

export const cases: Uint8Array[] = [...consumers, ...closures, ...elements];
