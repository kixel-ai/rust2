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

// A namespace a getter makes anew each time it is read, whose function says whether it was
// called on the namespace it was read from: by its arguments' count, or -1 where not.
Object.defineProperty(globalThis, "rustFresh", {
	get() {
		const fresh = {
			of(...args: unknown[]) {
				return this === fresh ? args.length : -1;
			},
		};
		return fresh;
	},
});

export { run };

/** Each case's name and the byte that picks it, its place in `cases`. */
export const ops = new Map(cases().items.map(([name], i): [string, Uint8Array] => [name, Uint8Array.of(i)]));
