// The harness, with its cases by name for `differential.ts`.

import { cases, run } from "./harness.ts";

// The class `imports::Vendored` names by its vendor prefix alone, defined before either
// side runs, as a browser defines its prefixed globals.
Object.assign(globalThis, {
	webkitRustVendored: class {
		name() {
			return "webkit";
		}
	},
});

export { run };

/** Each case's name and the byte that picks it, its place in `cases`. */
export const ops = new Map(cases().items.map(([name], i): [string, Uint8Array] => [name, Uint8Array.of(i)]));
