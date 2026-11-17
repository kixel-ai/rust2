// Builds each fixture's routes from its Rust source, timing each step as a consumer's first
// build: from an emptied target directory, without the dev shell's compiler cache, in wall
// and CPU seconds, its processes' children included.

import { copyFile, rm } from "node:fs/promises";
import { join } from "node:path";

/** A step's cost, in seconds. */
export interface Step {
	step: string;
	wall: number;
	cpu: number;
}

/** What `build` made, as paths, and what each step cost. */
export interface Built {
	/** The wasm32 build at the profile moq's WASM shipped with. */
	wasm: string;
	/** The same at `wasm-checked`, which keeps the translation's checks. */
	checked: string;
	/** `checked` as wasm2js lowered it: an ES module of its exports. */
	lowered: string;
	/** rust2-ffi's export of the fixture's facade, whose `web/` the rust2-wasm route imports. */
	export: string;
	/** The export's module, built at `wasm-checked`. */
	exported: string;
	/** The translation. */
	translated: string;
	steps: Step[];
}

/** The translators, and where the builds go. */
export interface Tools {
	/** rust2-ts's release binary. */
	ts: string;
	/** rust2-ffi's release binary. */
	ffi: string;
	/** ts/, which `fixtures/` is in. */
	root: string;
	/** Where the routes' files go. */
	out: string;
	/** The target directory, emptied before each step. */
	scratch: string;
}

// The drivers link the toolchain's own libraries, as rustc does.
const sysroot = Bun.spawnSync(["rustc", "--print", "sysroot"], { stderr: "inherit" }).stdout.toString().trim();
const libpath = process.platform === "darwin" ? "DYLD_LIBRARY_PATH" : "LD_LIBRARY_PATH";

// wasm2js validates against the MVP unless told what rustc turns on for wasm32.
const features = [
	"--enable-bulk-memory",
	"--enable-bulk-memory-opt",
	"--enable-nontrapping-float-to-int",
	"--enable-sign-ext",
	"--enable-mutable-globals",
	"--enable-reference-types",
	"--enable-multivalue",
];

/**
 * Builds the routes of `fixtures/<name>` (`codec` or `task`), the crate `<name>-fixture` and
 * its facade in `web/`, into `tools.out`, and its translation into the directory `translated`.
 */
export async function build(name: string, translated: string, tools: Tools): Promise<Built> {
	const { root, out, scratch } = tools;
	const steps: Step[] = [];
	const step = async (step: string, cmd: string[]) => steps.push({ step, ...(await timed(cmd, root, scratch)) });
	const built: Built = {
		wasm: join(out, `${name}.wasm`),
		checked: join(out, `${name}-checked.wasm`),
		lowered: join(out, `${name}-wasm2js.mjs`),
		export: join(out, `${name}-web`),
		exported: join(out, `${name}-web.wasm`),
		translated,
		steps,
	};

	const wasm = (profile: string) =>
		step(profile === "wasm-release" ? "wasm" : "wasm checked", [
			"cargo",
			"build",
			"--locked",
			"--quiet",
			"--package",
			`${name}-fixture`,
			"--features",
			"bench",
			"--target",
			"wasm32-unknown-unknown",
			"--profile",
			profile,
		]);
	await wasm("wasm-release");
	await copyFile(join(scratch, "wasm32-unknown-unknown/wasm-release", `${name}_fixture.wasm`), built.wasm);
	await wasm("wasm-checked");
	await copyFile(join(scratch, "wasm32-unknown-unknown/wasm-checked", `${name}_fixture.wasm`), built.checked);
	await step("wasm2js", ["wasm2js", "-O", ...features, built.checked, "-o", built.lowered]);

	const facade = join(root, "fixtures", name, "web");
	await rm(built.export, { recursive: true, force: true });
	await step("rust2-ffi", [tools.ffi, "--config", join(facade, "ffi.toml"), "--target", "wasm32-wasip1", "--out", built.export]);
	await step("export's build", [
		"cargo",
		"build",
		"--locked",
		"--quiet",
		"--manifest-path",
		join(built.export, "Cargo.toml"),
		"--target",
		"wasm32-wasip1",
		"--profile",
		"wasm-checked",
		"--features",
		"web",
	]);
	await copyFile(join(scratch, "wasm32-wasip1/wasm-checked", `${name}_web.wasm`), built.exported);

	await rm(built.translated, { recursive: true, force: true });
	await step("rust2-ts", [tools.ts, "--config", join(root, "fixtures", name, "ts.toml"), "--out", built.translated]);
	return built;
}

/** Wall and CPU seconds of `cmd` in `cwd`, from an emptied `scratch` target, uncached. */
async function timed(cmd: string[], cwd: string, scratch: string): Promise<Omit<Step, "step">> {
	// Removed, not emptied: cargo cleans only a target directory it made.
	await rm(scratch, { recursive: true, force: true });
	const env: Record<string, string | undefined> = { ...process.env, CARGO_TARGET_DIR: scratch, [libpath]: join(sysroot, "lib") };
	// mbx's Cargo shim keeps a compiler cache that emptying the target does not empty: the
	// wrapper is its, and CARGO would name the shim to the drivers.
	const shim = process.env.RUST2_CARGO_SHIM;
	env.PATH = (process.env.PATH ?? "")
		.split(":")
		.filter((dir) => dir !== shim)
		.join(":");
	delete env.RUSTC_WRAPPER;
	delete env.CARGO;
	const start = performance.now();
	const proc = Bun.spawn(cmd, { cwd, env, stdout: "inherit", stderr: "inherit" });
	if ((await proc.exited) !== 0) throw new Error(`${cmd.join(" ")} failed`);
	const wall = (performance.now() - start) / 1000;
	const usage = proc.resourceUsage();
	if (!usage) throw new Error(`${cmd[0]} reported no resource usage`);
	return { wall, cpu: Number(usage.cpuTime.total) / 1e6 };
}
