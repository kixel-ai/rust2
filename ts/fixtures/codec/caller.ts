// The harness, with its cases by name for `differential.ts`.

import { cases, run } from "./harness.ts";

export { run };

/** Each case's name and the byte that picks it, its place in `cases`. */
export const ops = new Map(cases().items.map(([name], i): [string, Uint8Array] => [name, Uint8Array.of(i)]));

/** Its cases read full-width values. */
export const wide = true;
