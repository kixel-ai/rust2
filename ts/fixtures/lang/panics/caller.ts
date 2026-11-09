// What `panics`' hooks wrote, which `differential.ts` compares after each case, and the
// inputs its cases name, which run with the generated ones.

import { ops } from "./lang.ts";
import { note as noted } from "./lib.ts";

export { ops, run } from "./lang.ts";

/** What the hooks wrote since the last call, as a JS string. */
export const note = (): string => noted().v;

/** Each of std's sites `hooked::sited` panics at, by kind, which random inputs may miss, where a hook is set. */
const sited = ops.get("hooked::sited");
export const cases: Uint8Array[] = sited === undefined ? [] : Array.from({ length: 87 }, (_, kind) => Uint8Array.of(...sited, kind, 1));
