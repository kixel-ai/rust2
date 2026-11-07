// What `panics`' hooks wrote, which `differential.ts` compares after each case.

import { note as noted } from "./lib.ts";

export { ops, run } from "./lang.ts";

/** What the hooks wrote since the last call, as a JS string. */
export const note = (): string => noted().v;
