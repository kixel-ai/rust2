// The api fixture's React Native test: tests/api.ts's script and cases through the React
// Native binding, which tests/fixtures.rs bundles with a copy of api.ts importing it. It
// runs on Hermes, in tests/rn_host.cpp off a device, whose `process` and `print` stand in
// for Node's, and exits as api.ts's `main` does: with an argument, the case of that name.

import * as api from "./react-native/index.ts";
import * as check from "./api.ts";
import * as rt from "./react-native/rt.ts";

declare const process: { argv: string[]; exit(code: number): never; stderr: { write(text: string): boolean } };

/**
 * A typed array crosses as the engine holds it, whatever its own properties or prototype
 * claim: a shadowed length reads no further than its bytes, and another kind relabeled is
 * refused, since the JSI module reads its memory as the kind it claims.
 */
function typed(): void {
	const shadowed = Uint8Array.of(0x61);
	Object.defineProperties(shadowed, { byteLength: { value: 4096 }, byteOffset: { value: 8 }, length: { value: 4096 } });
	const read = api.prefixed(shadowed, undefined).slice();
	if (read.length !== 1 || read[0] !== 0x61) throw new Error(`a shadowed length read ${read.length} bytes`);
	const relabeled = new Int8Array(3);
	Object.setPrototypeOf(relabeled, Int32Array.prototype);
	try {
		api.sumLists(relabeled as unknown as Int32Array, undefined, undefined);
	} catch (e) {
		if (e instanceof TypeError) return;
		throw e;
	}
	throw new Error("a relabeled typed array crossed");
}

/** A 64-bit target's `usize` constant crosses whole, as its parameters do. */
function word(): void {
	if (rt.u64ToBig(api.WORD) !== 0xffffffffffffffffn) throw new Error(`usize::MAX is ${String(api.WORD)}`);
}

const cases: Record<string, () => void | Promise<void>> = {
	typed,
	word,
	futures: check.futures,
	owned: check.owned,
	closed: check.closedObjects,
	bytes: check.lent,
	bounds: check.bounds,
	checks: check.checks,
};

async function main(): Promise<void> {
	await api.load();
	const which = process.argv[2];
	if (which === undefined) return check.script();
	return (cases[which] ?? (() => check.broken(which)))();
}

main().then(
	() => process.exit(0),
	(e: unknown) => {
		// The host's `process.exit` throws this to stop the script, having exited already.
		if (e instanceof Error && e.message === "__exited") return;
		process.stderr.write(`uncaught: ${e instanceof Error ? (e.stack ?? e.message) : String(e)}\n`);
		process.exit(1);
	},
);
