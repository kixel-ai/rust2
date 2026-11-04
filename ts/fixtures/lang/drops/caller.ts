// The inputs `drops`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** `drops::drops`: each shape, by each pick of its variants and each way it drops. */
const dropsCases = Array.from({ length: 12 * 16 * 12 }, (_, i) => Uint8Array.of(...op("drops::drops"), i % 12, Math.floor(i / 12) % 16, Math.floor(i / 192)));

export const cases: Uint8Array[] = [...dropsCases];
