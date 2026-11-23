// The api fixture's React Native test: tests/api.ts's script and cases through the React
// Native binding, which tests/fixtures.rs bundles with a copy of api.ts importing it. It
// runs on Hermes, in tests/rn_host.cpp off a device, whose `process` and `print` stand in
// for Node's, and exits as api.ts's `main` does: with an argument, the case of that name.

import * as api from "./react-native/index.ts";
import * as check from "./api.ts";

declare const process: { argv: string[]; exit(code: number): never; stderr: { write(text: string): boolean } };

const cases: Record<string, () => void | Promise<void>> = {
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
