// The inputs `drops`' cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/** `drops::drops`: each shape, by each pick of its variants and each way it drops. */
const dropsCases = Array.from({ length: 12 * 16 * 12 }, (_, i) => Uint8Array.of(...op("drops::drops"), i % 12, Math.floor(i / 12) % 16, Math.floor(i / 192)));

/** `drops::if_lets`: each element type, by each variant of the temporary. */
const ifLetsCases = Array.from({ length: 3 * 5 }, (_, i) => Uint8Array.of(...op("drops::if_lets"), i % 3, Math.floor(i / 3)));

export const cases: Uint8Array[] = [...dropsCases, ...ifLetsCases];
