// The app tests/fixtures.rs runs the api fixture's check in, launched with the case in the
// intent's `case` extra: each line it prints is logged as `rust2| <line>`, what it writes to
// stderr as `rust2! <text>`, and its exit as `rust2| exit <code>`.
import { AppRegistry } from "react-native";

function Check({ case: which }) {
	// The transcript's `console.log` and api.ts's `print` alike, apart from React Native's own.
	const log = console.log;
	console.log = (...args) => log(`rust2| ${args.join(" ")}`);
	globalThis.print = (line) => console.log(line);
	Object.assign(globalThis.process, {
		argv: ["react-native", "check", ...(which ? [which] : [])],
		exit(code) {
			log(`rust2| exit ${code}`);
			throw new Error("__exited");
		},
		stderr: {
			write(text) {
				log(`rust2! ${text}`);
				return true;
			},
		},
	});
	require("./check/entry.ts");
	return null;
}

AppRegistry.registerComponent("Consumer", () => Check);
