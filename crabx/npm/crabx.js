#!/usr/bin/env node
// Runs the crabx binary from the platform package npm installed beside this one.
"use strict";

const { spawn } = require("node:child_process");
const { constants } = require("node:os");
const path = require("node:path");

// npm's `os` and `cpu` names for each release target; release.py builds the packages from it.
const TARGETS = {
	"linux x64": "x86_64-unknown-linux-gnu",
	"linux arm64": "aarch64-unknown-linux-gnu",
	"darwin x64": "x86_64-apple-darwin",
	"darwin arm64": "aarch64-apple-darwin",
	"win32 x64": "x86_64-pc-windows-msvc",
	"win32 arm64": "aarch64-pc-windows-msvc",
};

// The platform package's name: Linux packages carry glibc builds only.
function packageName(platform, arch) {
	return `@crabx/${platform}-${arch}${platform === "linux" ? "-gnu" : ""}`;
}

function fail(message) {
	process.stderr.write(`crabx: ${message}\n`);
	process.exit(1);
}

function binary() {
	const { platform, arch } = process;
	if (!TARGETS[`${platform} ${arch}`]) {
		fail(`no prebuilt crabx for ${platform} ${arch}; install it with \`cargo install crabx\``);
	}
	if (platform === "linux") {
		// The report's network section costs ~170 ms; the header alone names the runtime's libc.
		process.report.excludeNetwork = true;
		if (!process.report.getReport().header.glibcVersionRuntime) {
			fail("crabx's Linux builds need glibc, and this system has none; install it with `cargo install crabx`");
		}
	}
	const name = packageName(platform, arch);
	let manifest;
	try {
		manifest = require.resolve(`${name}/package.json`);
	} catch {
		fail(`${name} is not installed; reinstall crabx without omitting optional dependencies`);
	}
	return path.join(path.dirname(manifest), "bin", platform === "win32" ? "crabx.exe" : "crabx");
}

function main() {
	const child = spawn(binary(), process.argv.slice(2), { stdio: "inherit" });
	// A terminal's Ctrl-C and Ctrl-\ already reach the whole foreground group, crabx included,
	// so this process only waits for crabx to exit. A supervisor's signals reach only this one.
	const ignore = () => {};
	const forward = (signal) => child.kill(signal);
	// Windows delivers Ctrl-C to every process on the console, and has no signals to forward.
	const handlers =
		process.platform === "win32"
			? { SIGINT: ignore }
			: { SIGINT: ignore, SIGQUIT: ignore, SIGTERM: forward, SIGHUP: forward };
	for (const [signal, handler] of Object.entries(handlers)) process.on(signal, handler);
	child.on("error", (error) => fail(`cannot run ${child.spawnfile}: ${error.message}`));
	child.on("exit", (code, signal) => {
		if (signal) {
			// Die of the same signal, so the caller sees what crabx saw. Node ignores some, such
			// as SIGPIPE, so the shell's code for the signal stands in when this one survives.
			for (const [name, handler] of Object.entries(handlers)) process.off(name, handler);
			process.exitCode = 128 + constants.signals[signal];
			process.kill(process.pid, signal);
		} else {
			process.exit(code);
		}
	});
}

if (require.main === module) {
	main();
}

module.exports = { TARGETS, packageName };
