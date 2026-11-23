//! The api fixture is exported, built as a static library, and driven from C by
//! `tests/api.c`, from C++ by `tests/api.cpp`, from Kotlin by `tests/api.kt` on the JVM
//! and, shrunk by R8, on Android, from Python by `tests/api.py`, from Swift by
//! `tests/api.swift`, from Go by `tests/api.go`, and from Dart by `tests/api.dart`, whose
//! transcripts must match the fixture's own script run natively.
//! The reject fixture's export must fail at each item it cannot export, and generated
//! type shapes round-trip in every language ([`shapes`]).

#[path = "fixtures/corpus.rs"]
mod corpus;
#[path = "fixtures/protocol.rs"]
mod protocol;
#[path = "fixtures/shapes.rs"]
mod shapes;
#[path = "fixtures/step.rs"]
mod step;
#[path = "fixtures/transcript/mod.rs"]
mod transcript;

use std::collections::BTreeSet;
use std::env::consts::EXE_SUFFIX;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

use step::Step;

fn manifest_dir() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn scratch() -> PathBuf {
	PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
}

/// The host's target, which rustc builds for by default.
fn host() -> &'static str {
	static HOST: OnceLock<String> = OnceLock::new();
	HOST.get_or_init(|| {
		let rustc = run(Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into())).arg("-vV"));
		let version = String::from_utf8_lossy(&rustc.stdout).into_owned();
		let host = version.lines().find_map(|line| line.strip_prefix("host: "));
		host.expect("rustc -vV names the host").to_string()
	})
}

/// A wasm32 target, which has an item the host lacks and a variant of an enum.
const WASM: &str = "wasm32-unknown-unknown";

/// The target the api fixture's binding serves besides the host's: wasm32, whose std
/// imports its platform, which the web binding implements with web APIs.
const WEB: &str = "wasm32-wasip1";

/// Cargo, on a `PATH` without what changes each run, as a Cargo shim's session does: build
/// scripts that read `PATH`, such as pyo3's, would otherwise run again each time, and
/// rebuild the glue.
fn cargo() -> Command {
	let mut cmd = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
	cmd.env("PATH", step::path());
	cmd
}

/// The shared library a crate of the lib `name` builds.
fn shared(name: &str) -> String {
	match (cfg!(windows), cfg!(target_os = "macos")) {
		(true, _) => format!("{name}.dll"),
		(_, true) => format!("lib{name}.dylib"),
		_ => format!("lib{name}.so"),
	}
}

fn run(cmd: &mut Command) -> Output {
	let output = output(cmd);
	check(cmd, &output);
	output
}

fn output(cmd: &mut Command) -> Output {
	cmd.output().unwrap_or_else(|err| panic!("running {cmd:?}: {err}"))
}

/// Asserts `cmd` succeeded, printing what it printed if not.
fn check(cmd: &Command, output: &Output) {
	assert!(
		output.status.success(),
		"{cmd:?} failed with {}:\n{}{}",
		output.status,
		String::from_utf8_lossy(&output.stdout),
		String::from_utf8_lossy(&output.stderr)
	);
}

/// Export the fixture `name` into `out` for `targets` into one binding, else the host,
/// with these cargo args, returning what the exporter printed.
fn export(name: &str, out: &Path, targets: &[&str], cargo: &[&str]) -> Output {
	exporter(name, out, targets, &[], cargo)
		.output()
		.expect("running rust2-ffi")
}

/// The export of the fixture `name`, with `args` for rust2-ffi besides.
fn exporter(name: &str, out: &Path, targets: &[&str], args: &[&std::ffi::OsStr], cargo: &[&str]) -> Command {
	let fixture = manifest_dir().join("fixtures").join(name);
	let mut command = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"));
	command
		.arg("--manifest-path")
		.arg(fixture.join("Cargo.toml"))
		.arg("--out")
		.arg(out)
		.args(targets.iter().flat_map(|target| ["--target", target]))
		.args(args)
		.arg("--")
		.args(cargo)
		.env("CARGO_TARGET_DIR", scratch().join(name).join("build"));
	command
}

/// What `Deref` forwards: a member the object or a nearer target has hides one past it, even
/// a target that does not export, a method only a method and a field only a field, and a
/// forwarded field sharing a method's name exports beside it.
#[test]
fn deref_shadowing() {
	let header = std::fs::read_to_string(api().out.join("include").join("api.h")).unwrap();
	for present in [
		"api_Layered_lines(",
		"api_Pad_u16_lines(",
		"api_Pad_u16_title(",
		"api_Pad_u16_title_get(",
		"api_Marker_x(",
		"api_Marker_x_get(",
		"api_Marker_x_set(",
		"api_Marker_y(",
		"api_Marker_y_get(",
	] {
		assert!(header.contains(present), "{present} is not exported");
	}
	assert!(
		!header.contains("api_Layered_describe("),
		"api_Layered_describe( is exported"
	);
}

/// The api fixture, exported and built once for each language's test.
struct Api {
	/// The export.
	out: PathBuf,
	/// Where cargo built the static library and the shared one Kotlin and Dart load.
	lib: PathBuf,
	/// What a program linking the static library links besides, as rustc prints it.
	native: Vec<String>,
	/// The fixture's script, run natively.
	transcript: String,
}

impl Api {
	fn staticlib(&self) -> PathBuf {
		self.lib.join(match cfg!(target_env = "msvc") {
			true => "api.lib",
			false => "libapi.a",
		})
	}
}

/// The fixture's script as data, typed against the export's API, which each language's
/// test generates its own differential script from.
fn transcript() -> &'static transcript::Transcript {
	static TRANSCRIPT: OnceLock<transcript::Transcript> = OnceLock::new();
	TRANSCRIPT.get_or_init(|| {
		transcript::Transcript::load(
			&api().out.join("api.json"),
			&manifest_dir().join("fixtures/api/examples/transcript.rs"),
		)
	})
}

/// The api fixture's checks as the release builder takes them, into `RUST2_TRANSCRIPTS`: a
/// directory each, `python`, `cpp`, `web`, `dart`, `go`, and `swift`, holding the test as
/// `check.<ext>` beside its generated script. Only the export runs, not `api`'s builds.
#[test]
#[ignore = "writes outside the target: just ffi transcripts <dir>"]
fn transcripts() {
	let dir = PathBuf::from(std::env::var_os("RUST2_TRANSCRIPTS").expect("RUST2_TRANSCRIPTS names the directory"));
	let out = scratch().join("transcripts").join("out");
	// The export `api` makes, so each script is the one its test runs.
	let output = export("api", &out, &[host(), WEB], &API_CARGO);
	assert!(
		output.status.success(),
		"exporting failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let t = transcript::Transcript::load(
		&out.join("api.json"),
		&manifest_dir().join("fixtures/api/examples/transcript.rs"),
	);
	let tests = manifest_dir().join("tests");
	// The web script imports the test's hooks by its name, which is `check.ts` here.
	let typescript = transcript::typescript::emit(&t, transcript::typescript::Binding::Web);
	let hooks = "import * as hook from \"./api.ts\";";
	assert_eq!(
		typescript.matches(hooks).count(),
		1,
		"the web script imports the hooks once"
	);
	let typescript = typescript.replace(hooks, "import * as hook from \"./check.ts\";");
	for (language, check, file, script) in [
		("python", "api.py", "transcript.py", transcript::python::emit(&t)),
		("cpp", "api.cpp", "transcript.hpp", transcript::cpp::emit(&t)),
		("web", "api.ts", "transcript.ts", typescript),
		("dart", "api.dart", "transcript.dart", transcript::dart::emit(&t)),
		("go", "api.go", "transcript.go", transcript::go::emit(&t)),
		("swift", "api.swift", "transcript.swift", transcript::swift::emit(&t)),
	] {
		let checks = dir.join(language);
		if checks.exists() {
			std::fs::remove_dir_all(&checks).unwrap();
		}
		let ext = Path::new(check).extension().unwrap();
		step::copy(&tests.join(check), &checks.join("check").with_extension(ext));
		step::write(&checks.join(file), script);
	}
}

/// `contents` written as `file` in a scratch directory of `language`'s, which the
/// language's test puts on its path.
fn generated(language: &str, file: &str, contents: &str) -> PathBuf {
	let dir = scratch().join("api").join(format!("{language}-transcript"));
	// Written only when it changes, so what builds from it stays fresh.
	step::write(&dir.join(file), contents);
	dir
}

/// The directory of `transcript.hpp`, which `tests/api.cpp` includes: the transcript as
/// C++.
fn cpp_transcript() -> PathBuf {
	generated("cpp", "transcript.hpp", &transcript::cpp::emit(transcript()))
}

/// The cargo args each export of the api fixture its tests build takes: a feature of the
/// facade's that enables one in a crate it re-exports, which the C library must build with,
/// and the dev profile its tests build, so its leaves are leaf calls in them.
const API_CARGO: [&str; 4] = ["--profile", "dev", "--features", "doubled"];

fn api() -> &'static Api {
	static API: OnceLock<Api> = OnceLock::new();
	API.get_or_init(|| {
		let out = scratch().join("api").join("out");
		let features = ["--features", "doubled"];
		// One binding for the host and wasm32, which the web binding runs.
		let output = export("api", &out, &[host(), WEB], &API_CARGO);
		assert!(
			output.status.success(),
			"exporting failed:\n{}",
			String::from_utf8_lossy(&output.stderr)
		);
		let target = scratch().join("api").join("glue");
		let build = |target: &Path, feature: &str| {
			run(cargo()
				.arg("build")
				.arg("--quiet")
				.arg("--locked")
				.arg("--manifest-path")
				.arg(out.join("Cargo.toml"))
				.arg("--target-dir")
				.arg(target)
				.args(["--features", feature])
				// Glue the exporter writes builds without warnings.
				.env("RUSTFLAGS", "-D warnings --print=native-static-libs"))
		};
		// The CPython extension's symbols would leave the C programs' links undefined, so
		// it builds apart, before the library they link, and is copied beside the Python.
		// Each build has its own target directory, since cargo keeps one fingerprint of a
		// package across its features, and each build would make the other's stale.
		let python = scratch().join("api").join("glue-python");
		build(&python, "python");
		step::copy(
			&python.join("debug").join(PYTHON_LIB),
			&out.join("python").join(PYTHON_LIB),
		);
		// The shared library Kotlin and Dart load.
		let built = build(&target, "kotlin,dart");
		let native = String::from_utf8_lossy(&built.stderr)
			.lines()
			.find_map(|line| line.strip_prefix("note: native-static-libs: "))
			.expect("rustc prints the static library's native libraries")
			.split_whitespace()
			.map(str::to_string)
			.collect();
		let transcript = run(cargo()
			.args([
				"run",
				"--quiet",
				"--locked",
				"--package",
				"api-fixture",
				"--example",
				"transcript",
			])
			.args(features)
			.current_dir(manifest_dir()));
		Api {
			out,
			lib: target.join("debug"),
			native,
			transcript: String::from_utf8_lossy(&transcript.stdout).into_owned(),
		}
	})
}

/// What a use of a handle lent from another's value prints once that value was borrowed
/// exclusively.
const OUTLIVED: &str = "api_Meter_count: a handle lent from another outlived it";

/// The directory of the C script generated from the transcript, which `tests/api.c`
/// includes.
fn c_transcript() -> PathBuf {
	let header = std::fs::read_to_string(api().out.join("include").join("api.h")).unwrap();
	generated("c", "transcript.c", &transcript::c::emit(transcript(), &header))
}

/// `tests/api.c` built by `CC` against the api fixture's static library.
fn api_c_exe(api: &Api) -> PathBuf {
	let exe = scratch().join("api").join("api-test");
	let cc = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
	let src = manifest_dir().join("tests").join("api.c");
	let include = api.out.join("include");
	let script = c_transcript();
	Step {
		stamp: &step::stamp(&exe),
		reads: &[&src, &include, &script, &api.staticlib()],
		writes: &[&exe],
	}
	.run(
		Command::new(cc)
			.args(["-std=c11", "-pedantic", "-Wall", "-Wextra", "-Werror", "-g", "-I"])
			.arg(&include)
			.arg("-I")
			.arg(&script)
			.arg(&src)
			.arg(api.staticlib())
			.args(&api.native)
			.arg("-o")
			.arg(&exe),
	);
	exe
}

#[test]
fn api_c() {
	let api = api();
	let exe = api_c_exe(api);
	let c = run(&mut Command::new(&exe));
	assert_eq!(
		String::from_utf8_lossy(&c.stdout),
		api.transcript,
		"the C transcript differs from the native one"
	);
	// A non-null status takes each failed check, which changed nothing.
	run(Command::new(&exe).arg("statuses"));
	// A value that is not Send stays on its thread; one freed after that exited leaks.
	run(Command::new(&exe).arg("threads"));
	let orphan = run(Command::new(&exe).arg("orphan"));
	assert_eq!(
		String::from_utf8_lossy(&orphan.stderr),
		"rust2-ffi: api::Local leaks: it is not Send, and its thread exited before it was freed\n"
	);

	// Each broken contract aborts, naming the function and what broke.
	for (case, message) in [
		("moved", "api_Counter_get: a handle's value was moved out"),
		("borrowed", "api_Channel_clear: a handle is already borrowed"),
		("utf8", "api_greet: a string is not UTF-8"),
		(
			"finished",
			"api_Channel_recv_poll: a future is polled after it finished",
		),
		("enum", "api_greet: an enum value is invalid"),
		("bound", "api_count: a bound's kind is invalid"),
		("array", "api_vec_u16_free: an array is not as Rust returned it"),
		("future", "api_after: a future's poll is null"),
		("started", "api_after_start: a future's poll is null"),
		("start", "api_yielded_start: a future's handle pointer is null"),
		("lent", "api_Registry_edit: a lent handle was freed"),
		(
			"lent moved",
			"api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
		),
		("null", "api_Registry_edit: a function is null"),
		("returned", "api_made: a returned buffer is not one buf_copy made"),
		("reader", "api_tail: a reader's function is null"),
		("outlived", OUTLIVED),
		(
			"lent shared",
			"api_Meter_reset: a handle a call lent is borrowed exclusively or moved out",
		),
		("target", "api_wasm_only: it is not on this library's target"),
		(
			"variant",
			"api_pointer_bits: the variant is not on this library's target",
		),
		(
			"plain variant",
			"api_is_family: the variant is not on this library's target",
		),
		("zero", "api_next_hop: a NonZero is zero"),
	] {
		let output = Command::new(&exe).arg(case).output().unwrap();
		failed(&output, case);
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "{case} printed {stderr}");
	}
}

/// Random sequences of the ownership protocol's steps through the C ABI, by
/// `tests/protocol.c`, each matching what the model says each step returns.
#[test]
fn api_protocol() {
	let api = api();
	let exe = protocol_c(
		api,
		&scratch().join("api").join("protocol"),
		&[],
		&api.staticlib(),
		&api.native,
	);
	let scripts = protocol::scripts(0, PROTOCOL_SEEDS, PROTOCOL_STEPS, protocol::Profile::C);
	protocol::covered(&scripts);
	protocol::check(
		&scratch().join("api").join("protocol").join("scripts"),
		&scripts,
		|_| Command::new(&exe),
	);
}

/// The gate's protocol scripts: how many, and their random steps each.
const PROTOCOL_SEEDS: u64 = 500;
const PROTOCOL_STEPS: usize = 100;

/// Longer protocol scripts from other seeds, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_protocol_nightly() {
	let api = api();
	let exe = protocol_c(
		api,
		&scratch().join("api").join("protocol"),
		&[],
		&api.staticlib(),
		&api.native,
	);
	protocol::check(
		&scratch().join("api").join("protocol").join("nightly"),
		&protocol::scripts(PROTOCOL_SEEDS, 20_000, 400, protocol::Profile::C),
		|_| Command::new(&exe),
	);
}

/// `tests/protocol.c` built in `dir` against `lib`, with `flags` to the compiler.
fn protocol_c(api: &Api, dir: &Path, flags: &[&str], lib: &Path, native: &[String]) -> PathBuf {
	std::fs::create_dir_all(dir).unwrap();
	let exe = dir.join(format!("protocol{EXE_SUFFIX}"));
	let cc = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
	let compiler = if flags.is_empty() { cc } else { "clang".into() };
	let src = manifest_dir().join("tests").join("protocol.c");
	let include = api.out.join("include");
	Step {
		stamp: &step::stamp(&exe),
		reads: &[&src, &include, lib],
		writes: &[&exe],
	}
	.run(
		Command::new(compiler)
			.args(["-std=c11", "-pedantic", "-g", "-Wall", "-Wextra", "-Werror"])
			.args(flags)
			.arg("-I")
			.arg(&include)
			.arg(&src)
			.arg(lib)
			.args(native)
			.arg("-o")
			.arg(&exe),
	);
	exe
}

/// The C and C++ scripts against the glue built under AddressSanitizer, whose leak check
/// catches what a cancelled future or a freed handle keeps.
#[test]
#[ignore = "needs clang and the nightly's rust-src: just ffi sanitize"]
fn api_asan() {
	sanitized("address");
}

/// The C and C++ scripts against the glue built under ThreadSanitizer, which catches a
/// race between a wake on one thread and a poll on another.
#[test]
#[ignore = "needs clang and the nightly's rust-src: just ffi sanitize"]
fn api_tsan() {
	sanitized("thread");
}

/// The Go script under Go's race detector, which catches a race between an object's
/// `Close` and the nursery that pushes it out on another goroutine, and with cgocheck2,
/// which catches a Go pointer stored in memory Rust owns.
#[test]
#[ignore = "slow: just ffi sanitize"]
fn api_go_checked() {
	let api = api();
	let dir = scratch().join("api").join("go-checked");
	let module = dir.join("module");
	go_layout(
		api,
		&module,
		&std::fs::read_to_string(manifest_dir().join("tests").join("api.go")).unwrap(),
	);
	// The transcript's script, generated beside it in its package.
	step::write(&module.join("transcript.go"), transcript::go::emit(transcript()));
	let exe = dir.join(format!("api-test{EXE_SUFFIX}"));
	run(go(&module, &["build", "-race", "-o"])
		.arg(&exe)
		.env("GOEXPERIMENT", "cgocheck2"));
	// Every report fails the run.
	let env = [("GORACE", "halt_on_error=1")];
	let output = run(Command::new(&exe).envs(env));
	assert_eq!(
		String::from_utf8_lossy(&output.stdout),
		api.transcript,
		"the Go transcript under the race detector and cgocheck2 differs from the native one"
	);
	for case in ["futures", "owned", "closed", "bytes", "closures", "checks"] {
		run(Command::new(&exe).arg(case).envs(env));
	}
}

/// The C and C++ scripts under Valgrind's memcheck, against the library as the gate builds
/// it: uninstrumented, so it also sees what a sanitizer's own build of std would hide, such
/// as a read of memory nothing wrote. A definite or indirect leak fails the run too.
#[test]
#[ignore = "needs Valgrind: just ffi sanitize"]
fn api_valgrind() {
	let api = api();
	let c = api_c_exe(api);
	let cpp = scratch().join("api").join(format!("api-c++17{EXE_SUFFIX}"));
	let link = Link {
		generated: Some(cpp_transcript()),
		..api.link()
	};
	cxx(&link, 17, true, &manifest_dir().join("tests").join("api.cpp"), &cpp);
	for (exe, cases) in [
		(&c, &["statuses", "threads"][..]),
		(&cpp, &["futures", "lifetimes", "checks"]),
	] {
		for case in std::iter::once(None).chain(cases.iter().copied().map(Some)) {
			let output = run(Command::new("valgrind")
				.args([
					"--quiet",
					"--error-exitcode=99",
					"--exit-on-first-error=yes",
					"--track-origins=yes",
					"--leak-check=full",
					"--errors-for-leak-kinds=definite,indirect",
				])
				.arg(exe)
				.args(case));
			if case.is_none() {
				assert_eq!(
					String::from_utf8_lossy(&output.stdout),
					api.transcript,
					"{} under Valgrind differs from the native transcript",
					exe.display()
				);
			}
		}
	}
}

/// The api fixture's glue built, std included, under the `sanitizer`: its static library,
/// and what a program linking it links besides.
fn sanitized_glue(sanitizer: &str) -> (PathBuf, Vec<String>) {
	let api = api();
	let dir = scratch().join("api").join(sanitizer);
	// On aarch64, the runtime's atomics call `__aarch64_*` helpers, which the instrumented
	// compiler_builtins would define and answer by reading a flag through a shadow check, before
	// the runtime has mapped its shadow: every run would crash in `__asan_init` or `__tsan_init`.
	// Without them the link takes the toolchain's own, which are not instrumented.
	let outline = if cfg!(target_arch = "aarch64") {
		" -Ctarget-feature=-outline-atomics"
	} else {
		""
	};
	let built = run(cargo()
		.args(["build", "--quiet", "--locked", "-Zbuild-std", "--target", host()])
		.arg("--manifest-path")
		.arg(api.out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(dir.join("glue"))
		.env(
			"RUSTFLAGS",
			format!("-D warnings -Zsanitizer={sanitizer} --print=native-static-libs{outline}"),
		));
	let native: Vec<String> = String::from_utf8_lossy(&built.stderr)
		.lines()
		.find_map(|line| line.strip_prefix("note: native-static-libs: "))
		.expect("rustc prints the static library's native libraries")
		.split_whitespace()
		.map(str::to_string)
		.collect();
	(dir.join("glue").join(host()).join("debug").join("libapi.a"), native)
}

/// Every report fails the run, rather than only those the sanitizer deems fatal.
const SANITIZER_ENV: [(&str, &str); 3] = [
	("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=1"),
	("TSAN_OPTIONS", "halt_on_error=1"),
	("UBSAN_OPTIONS", "halt_on_error=1:print_stacktrace=1"),
];

/// Build the api fixture's glue under the `sanitizer`, and run the C and C++ scripts and
/// the protocol scripts' C driver against it, built by clang under the same one.
fn sanitized(sanitizer: &str) {
	let api = api();
	let dir = scratch().join("api").join(sanitizer);
	let (lib, native) = sanitized_glue(sanitizer);
	let tests = manifest_dir().join("tests");
	// UndefinedBehaviorSanitizer rides with AddressSanitizer on the C and C++ side, whose
	// undefined behavior Rust's checks cannot see. `-fno-wrapv` overrides a hardened
	// compiler's, such as Nix's, which would define signed overflow and so hide it.
	let sanitize: &[&str] = match sanitizer {
		"address" => &[
			"-fsanitize=address,undefined",
			"-fno-sanitize-recover=all",
			"-fno-wrapv",
		],
		"thread" => &["-fsanitize=thread"],
		other => panic!("no C flags for the {other} sanitizer"),
	};
	// Each language's generated script, which its test includes.
	let scripts = [c_transcript(), cpp_transcript()];
	for (compiler, std, src, args) in [
		("clang", "-std=c11", "api.c", &["threads"][..]),
		(
			"clang++",
			"-std=c++17",
			"api.cpp",
			&["futures", "lifetimes", "checks"][..],
		),
	] {
		let exe = dir.join(src.replace('.', "-"));
		run(Command::new(compiler)
			.args([std, "-g", "-Wall", "-Wextra", "-Werror"])
			.args(sanitize)
			.arg("-I")
			.arg(api.out.join("include"))
			.args(
				scripts
					.iter()
					.flat_map(|dir| [std::ffi::OsStr::new("-I"), dir.as_os_str()]),
			)
			.arg(tests.join(src))
			.arg(&lib)
			.args(&native)
			.arg("-o")
			.arg(&exe));
		let output = run(Command::new(&exe).envs(SANITIZER_ENV));
		assert_eq!(
			String::from_utf8_lossy(&output.stdout),
			api.transcript,
			"{src} under {sanitizer} differs from the native transcript"
		);
		for arg in args {
			run(Command::new(&exe).arg(arg).envs(SANITIZER_ENV));
		}
	}
	// The protocol scripts, whose steps on two threads free and drop what the other made.
	let exe = protocol_c(api, &dir.join("protocol"), sanitize, &lib, &native);
	protocol::check(
		&dir.join("protocol").join("scripts"),
		&protocol::scripts(0, PROTOCOL_SEEDS, 200, protocol::Profile::C),
		|_| {
			let mut cmd = Command::new(&exe);
			cmd.envs(SANITIZER_ENV);
			cmd
		},
	);
}

/// The glue's `extern "C"` functions driven from Rust under Miri, by `tests/miri/api.rs`
/// and the protocol scripts' driver `tests/miri/protocol.rs`, mounted as test modules of a
/// copy of the export.
#[test]
#[ignore = "needs the nightly's miri: just ffi sanitize"]
fn api_miri() {
	let api = api();
	let copy = scratch().join("api").join("miri");
	let _ = std::fs::remove_dir_all(&copy);
	std::fs::create_dir_all(copy.join("src")).unwrap();
	for file in ["Cargo.toml", "Cargo.lock", "build.rs"] {
		std::fs::copy(api.out.join(file), copy.join(file)).unwrap();
	}
	std::fs::create_dir_all(copy.join("src").join("rt")).unwrap();
	for file in ["rt.rs", "lib.rs"] {
		std::fs::copy(api.out.join("src").join(file), copy.join("src").join(file)).unwrap();
	}
	for entry in std::fs::read_dir(api.out.join("src").join("rt")).unwrap() {
		let entry = entry.unwrap();
		std::fs::copy(entry.path(), copy.join("src").join("rt").join(entry.file_name())).unwrap();
	}
	let mut lib = std::fs::read_to_string(copy.join("src").join("lib.rs")).unwrap();
	for (module, file) in [("miri", "api.rs"), ("protocol", "protocol.rs")] {
		lib.push_str(&format!(
			"\n#[cfg(test)]\n#[path = {:?}]\nmod {module};\n",
			manifest_dir().join("tests").join("miri").join(file)
		));
	}
	std::fs::write(copy.join("src").join("lib.rs"), lib).unwrap();
	run(cargo()
		.args(["miri", "test", "--locked", "--lib", "--manifest-path"])
		.arg(copy.join("Cargo.toml"))
		.env("MIRIFLAGS", "-Zmiri-strict-provenance"));
}

/// The C++ header at each standard, with exceptions and RTTI on and off between them,
/// runs the same script, then checks its futures' semantics and who owns each handle.
#[test]
fn api_cpp() {
	let api = api();
	let link = Link {
		generated: Some(cpp_transcript()),
		..api.link()
	};
	for (standard, plain) in [(17, true), (20, false), (23, true)] {
		let exe = scratch().join("api").join(format!("api-c++{standard}{EXE_SUFFIX}"));
		cxx(
			&link,
			standard,
			plain,
			&manifest_dir().join("tests").join("api.cpp"),
			&exe,
		);
		let cpp = run(&mut Command::new(&exe));
		assert_eq!(
			String::from_utf8_lossy(&cpp.stdout),
			api.transcript,
			"the C++{standard} transcript differs from the native one"
		);
		run(Command::new(&exe).arg("futures"));
		run(Command::new(&exe).arg("lifetimes"));
		run(Command::new(&exe).arg("checks"));
		let rehomed = run(Command::new(&exe).arg("rehomed"));
		assert_eq!(String::from_utf8_lossy(&rehomed.stdout), "scaled by 7\n");
		for (case, message) in [
			("moved", "api_Counter_get: a handle's value was moved out"),
			("moved from", "api_Counter_get: a handle is null"),
			("shared moved from", "api_Gauge_into_level: a handle is null"),
			(
				"shared moved out",
				"api_Gauge_into_level: a handle's value was moved out",
			),
			("lent copied", "a lent object was copied past its call"),
			("value", "value() of a result that holds an error"),
			(
				"finished",
				"api_Channel_recv_poll: a future is polled after it finished",
			),
			(
				"lent moved",
				"api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
			),
			("lent kept", "a lent object was moved out of"),
			("overread", "api_tail: a reader read more than it was asked"),
			("null view", "a view of null bytes has a length"),
			("outlived", OUTLIVED),
			("lent ledger", "api_Ledger_published: a handle is borrowed exclusively"),
		] {
			let output = Command::new(&exe).arg(case).output().unwrap();
			failed(&output, &format!("C++{standard} {case}"));
			let stderr = String::from_utf8_lossy(&output.stderr);
			assert!(stderr.contains(message), "C++{standard} {case} printed {stderr}");
		}
	}

	if !cfg!(target_env = "msvc") {
		cpp_strict(&link);
	}

	// Each crate's header builds alone, needing only the headers it includes.
	let headers = std::fs::read_dir(api.out.join("include").join("api")).unwrap();
	let mut headers: Vec<String> = headers
		.map(|entry| entry.unwrap().file_name().into_string().unwrap())
		.collect();
	headers.sort();
	assert!(headers.len() > 2, "the api fixture has a header per crate: {headers:?}");
	for header in headers {
		let src = scratch().join("api").join(format!("alone-{header}.cpp"));
		step::write(&src, format!("#include \"api/{header}\"\nint main() {{}}\n"));
		let exe = scratch().join("api").join(format!("alone-{header}{EXE_SUFFIX}"));
		cxx(&api.link(), 17, true, &src, &exe);
	}

	// A C++ program whose header names another contract than its library's aborts at load,
	// non-dumpable from a constructor that runs before the header's check, as `broken` is.
	let mismatched = scratch().join("api").join("contract.cpp");
	step::write(
		&mismatched,
		r#"#ifdef __linux__
#include <cstdio>
#include <cstdlib>
#include <sys/prctl.h>
__attribute__((constructor(101))) static void nodump() {
	if (prctl(PR_SET_DUMPABLE, 0, 0, 0, 0) != 0) {
		std::perror("prctl");
		std::_Exit(1);
	}
}
#endif
#include "api.h"
#undef API_CONTRACT
#define API_CONTRACT ((uint64_t)0)
#include "api.hpp"
int main() {}
"#,
	);
	let contract = scratch().join("api").join(format!("contract{EXE_SUFFIX}"));
	cxx(&api.link(), 17, false, &mismatched, &contract);
	let output = Command::new(&contract).output().unwrap();
	failed(&output, "a mismatched contract");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api/rt.hpp and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// Clang's ownership, must-use, and reserved-name checks: the whole script builds clean
/// under them, so neither header declares a name C or C++ reserves, and each case of
/// `tests/api_strict.cpp` fails with its diagnostic. The dev shell's Clang 23,
/// `RUST2_CLANG`, adds `-Wlifetime-safety`; Linux, where the merge gate runs, requires it,
/// and elsewhere without it, as on the nightly's Apple Clang leg, the platform's `clang++`
/// runs every check but that one.
fn cpp_strict(link: &Link) {
	let include = &link.include;
	let generated = link
		.generated
		.as_deref()
		.expect("tests/api.cpp includes its transcript");
	let clang23 = std::env::var_os("RUST2_CLANG");
	assert!(
		clang23.is_some() || !cfg!(target_os = "linux"),
		"RUST2_CLANG names Clang 23's clang++: run this in the dev shell"
	);
	let lifetimes = clang23.is_some();
	let compiler = clang23.unwrap_or_else(|| "clang++".into());
	// What each case printed, kept apart from the others'; its object is not.
	let clang = |src: &Path, case: u8| {
		let mut cmd = Command::new(&compiler);
		cmd.args([
			"-std=c++17",
			"-Wconsumed",
			"-Wdangling",
			"-Wreserved-identifier",
			"-Werror",
		]);
		if lifetimes {
			cmd.arg("-Wlifetime-safety");
		}
		cmd.arg("-c")
			.arg("-I")
			.arg(include)
			.arg("-I")
			.arg(generated)
			.arg(format!("-DCASE={case}"))
			.arg(src)
			.arg("-o")
			.arg(scratch().join("api").join("strict.o"));
		let stamp = scratch()
			.join("api")
			.join("strict")
			.join(format!("{}-{case}.stamp", src.file_stem().unwrap().to_string_lossy()));
		let step = Step {
			stamp: &stamp,
			reads: &[src, include, generated],
			writes: &[],
		};
		// Case 0 must build, so only the failing cases keep a failure.
		match case {
			0 => step.run(&mut cmd),
			_ => step.output(&mut cmd),
		}
	};
	let tests = manifest_dir().join("tests");
	for src in ["api.cpp", "api_strict.cpp"] {
		clang(&tests.join(src), 0);
	}
	for (case, message) in [
		(
			1,
			"invalid invocation of method 'get' on object 'c' while it is in the 'consumed' state",
		),
		(
			2,
			"invalid invocation of method 'get' on object 'c' while it is in the 'consumed' state",
		),
		(
			3,
			"invalid invocation of method 'into_level' on a temporary object while it is in the 'consumed' state",
		),
		(
			4,
			"invalid invocation of method 'into_level' on a temporary object while it is in the 'consumed' state",
		),
		(5, "'this' argument to member function 'into_label' is an lvalue"),
		(6, "no matching member function for call to 'emplace'"),
		(7, "temporary whose address is used as value of local variable 'name'"),
		(8, "no viable conversion from 'texts'"),
		(
			9,
			"no viable conversion from 'std::vector<std::tuple<api::buf, api::Counter>>'",
		),
		(
			10,
			"ignoring return value of type 'task' declared with 'nodiscard' attribute",
		),
		(11, "ignoring return value of type 'result<"),
		(12, "local variable 'bytes' does not live long enough"),
	] {
		if case == 12 && !lifetimes {
			continue;
		}
		let output = clang(&tests.join("api_strict.cpp"), case);
		assert!(!output.status.success(), "case {case} built");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "case {case} printed {stderr}");
	}
}

/// What a program links of an export: its headers, its static library, and what that
/// links besides.
struct Link<'a> {
	include: PathBuf,
	staticlib: PathBuf,
	native: &'a [String],
	/// A directory of headers written for the program, such as its transcript.
	generated: Option<PathBuf>,
}

impl Api {
	fn link(&self) -> Link<'_> {
		Link {
			include: self.out.join("include"),
			staticlib: self.staticlib(),
			native: &self.native,
			generated: None,
		}
	}
}

/// Build the C++ `src` against the `link`ed library into `exe`, at C++`standard`, with
/// exceptions and RTTI off if `plain`, and warnings as errors: by `CXX` or the platform's
/// compiler, given MSVC's flags where the target is MSVC's.
fn cxx(link: &Link, standard: u8, plain: bool, src: &Path, exe: &Path) {
	let mut reads = vec![src, &link.include, &link.staticlib];
	reads.extend(link.generated.as_deref());
	let step = Step {
		stamp: &step::stamp(exe),
		reads: &reads,
		writes: &[exe],
	};
	step.run(&mut cxx_command(link, standard, plain, src, exe, true));
}

/// The C++ compile of `src` against `link` into `exe`, linked or, for a library
/// whose host lacks the symbols it calls, compiled to an object nothing links.
fn cxx_command(link: &Link, standard: u8, plain: bool, src: &Path, exe: &Path, linked: bool) -> Command {
	let include = &link.include;
	if cfg!(target_env = "msvc") {
		// cl from a developer environment. Its C++23 is `c++latest`; its objects go beside `exe`.
		let mut cmd = Command::new(std::env::var_os("CXX").unwrap_or_else(|| "cl".into()));
		let standard = match standard {
			23 => "c++latest".to_string(),
			n => format!("c++{n}"),
		};
		cmd.args([
			"-nologo",
			&format!("-std:{standard}"),
			"-permissive-",
			"-W4",
			"-WX",
			"-MD",
		]);
		cmd.args(match plain {
			true => &["-EHs-c-", "-D_HAS_EXCEPTIONS=0", "-GR-"][..],
			false => &["-EHsc", "-GR"],
		});
		let mut obj = OsString::from("-Fo");
		obj.push(exe.with_extension("obj"));
		cmd.arg("-I").arg(include);
		if let Some(generated) = &link.generated {
			cmd.arg("-I").arg(generated);
		}
		cmd.arg(src).arg(obj);
		if !linked {
			cmd.arg("-c");
			return cmd;
		}
		let mut out = OsString::from("-Fe");
		out.push(exe);
		cmd.arg(out);
		cmd.arg("-link").arg(&link.staticlib).args(link.native);
		return cmd;
	}
	let mut cmd = Command::new(std::env::var_os("CXX").unwrap_or_else(|| "c++".into()));
	cmd.arg(format!("-std=c++{standard}"));
	if plain {
		cmd.args(["-fno-exceptions", "-fno-rtti"]);
	}
	// -Wshadow stands in for MSVC's warnings that a name hides another, and
	// -Wmismatched-tags for C4099, a class declared `struct` or a struct `class`.
	cmd.args([
		"-pedantic",
		"-Wall",
		"-Wextra",
		"-Wshadow",
		"-Wmismatched-tags",
		"-Werror",
		"-I",
	])
	.arg(include);
	if let Some(generated) = &link.generated {
		cmd.arg("-I").arg(generated);
	}
	cmd.arg(src);
	if linked {
		cmd.arg(&link.staticlib).args(link.native);
	} else {
		cmd.arg("-c");
	}
	cmd.arg("-o").arg(exe);
	cmd
}

/// Random sequences of the ownership protocol's steps through the C++ header, by
/// `tests/protocol.cpp`, each matching what the model says each step returns: never a
/// failed check, which aborts in C++.
#[test]
fn api_cpp_protocol() {
	cpp_protocol(&protocol::scripts(
		0,
		PROTOCOL_SEEDS,
		PROTOCOL_STEPS,
		protocol::Profile::CPP,
	));
}

/// Longer protocol scripts through the C++ header, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_cpp_protocol_nightly() {
	cpp_protocol(&protocol::scripts(PROTOCOL_SEEDS, 5000, 400, protocol::Profile::CPP));
}

fn cpp_protocol(scripts: &[protocol::Script]) {
	let api = api();
	let dir = scratch().join("api").join("protocol-cpp");
	std::fs::create_dir_all(&dir).unwrap();
	let exe = dir.join(format!("protocol{EXE_SUFFIX}"));
	cxx(
		&api.link(),
		17,
		true,
		&manifest_dir().join("tests").join("protocol.cpp"),
		&exe,
	);
	protocol::check(&dir.join("scripts"), scripts, |_| Command::new(&exe));
}

/// The generated Kotlin, compiled with `tests/api.kt` against the library's JNI
/// functions, runs its script on the JVM, then refuses a library of another contract.
#[test]
fn api_kotlin() {
	let api = api();
	let classes = kotlin_classes(api);
	let jar = PathBuf::from(coroutines());
	let java = |classpath: &[&Path], args: &[&str]| {
		let mut cmd = Command::new("kotlin");
		cmd.arg("-J-Xcheck:jni")
			.arg(format!("-Djava.library.path={}", api.lib.display()))
			.arg("-cp")
			.arg(std::env::join_paths(classpath.iter().copied().chain([jar.as_path()])).unwrap())
			.arg("ApiKt")
			.args(args);
		cmd
	};
	kotlin(&api.transcript, &|args| java(&[&classes], args));
	// The protocol scripts, through `tests/protocol.kt`, compiled with the rest.
	protocol::check_all(
		&scratch().join("api").join("protocol").join("kotlin"),
		&protocol::scripts(0, KOTLIN_PROTOCOL_SEEDS, PROTOCOL_STEPS, protocol::Profile::KOTLIN),
		|| protocol_kotlin(api, &classes),
	);

	let native_returns = run(cargo()
		.arg("run")
		.arg("--quiet")
		.arg("--locked")
		.arg("--package")
		.arg("api-fixture")
		.arg("--example")
		.arg("returns")
		.current_dir(manifest_dir()));
	// Plain Java calls the same classes: javac refuses a `catch` of an exception no function
	// declares, and a companion's function called on its class.
	let callers = scratch().join("api").join("java");
	let stdlib = kotlin_stdlib();
	let classpath = std::env::join_paths([classes.as_path(), stdlib.as_path()]).unwrap();
	Step {
		stamp: &step::stamp(&callers),
		reads: &[&classes, &manifest_dir().join("tests").join("Caller.java"), &stdlib],
		writes: &[&callers],
	}
	.run(
		Command::new("javac")
			.args(["-Werror", "-Xlint:all", "-cp"])
			.arg(&classpath)
			.arg("-d")
			.arg(&callers)
			.arg(manifest_dir().join("tests").join("Caller.java")),
	);
	let caller = run(Command::new("java")
		.arg(format!("-Djava.library.path={}", api.lib.display()))
		.arg("-cp")
		.arg(std::env::join_paths([callers.as_path(), classes.as_path(), stdlib.as_path()]).unwrap())
		.arg("Caller"));
	assert_eq!(
		String::from_utf8_lossy(&caller.stdout),
		"java\n",
		"plain Java calls the binding"
	);
	jvm_names_exist(api, &classes);

	let kotlin_returns = run(&mut java(&[&classes], &["returns"]));
	assert_eq!(
		kotlin_returns.stdout, native_returns.stdout,
		"Kotlin composite returns differ from native Rust"
	);
	let generated = std::fs::read_to_string(api.out.join("kotlin/api.kt")).expect("generated Kotlin");
	for method in ["divide", "corner", "fingerprint", "return_values"] {
		let declaration = generated
			.lines()
			.find(|line| line.contains(&format!("external fun {method}(")))
			.expect("return method");
		assert!(
			!declaration.ends_with(": ByteArray") && !declaration.ends_with(": ByteArray?"),
			"{method} still returns an encoded array: {declaration}"
		);
	}

	// The same classes, with another contract than the library's in place of theirs,
	// refuse to load it: a _Native.class of its own, ahead on the classpath, leaves the
	// compiled classes as they were.
	let bytes = std::fs::read(classes.join("api").join("_Native.class")).unwrap();
	let contract = kotlin_contract(&api.out.join("kotlin").join("api.kt")).to_be_bytes();
	let at: Vec<usize> = (0..bytes.len() - 8).filter(|&i| bytes[i..i + 8] == contract).collect();
	assert_eq!(at.len(), 1, "_Native.class holds the contract once");
	let mut mismatched = bytes.clone();
	mismatched[at[0]] ^= 0xff;
	let shadow = scratch().join("api").join("kotlin-mismatched");
	step::write(&shadow.join("api").join("_Native.class"), &mismatched);
	let output = java(&[&shadow, &classes], &[]).output().unwrap();
	failed(&output, "a mismatched contract");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.kt and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);

	// A _Native from another build lacking the leaves `__contract` registers throws the
	// JVM's NoSuchMethodError from its initializer, rather than aborting.
	let lacking = scratch().join("api").join("kotlin-lacking");
	let source = lacking.join("src").join("_Native.java");
	step::write(
		&source,
		"package api;\n\npublic final class _Native {\n\tstatic {\n\t\tSystem.loadLibrary(\"api\");\n\t\t__contract();\n\t}\n\n\tstatic native long __contract();\n\n\tpublic static void main(String[] args) {}\n}\n",
	);
	run(Command::new("javac")
		.arg("-d")
		.arg(lacking.join("classes"))
		.arg(&source));
	let output = no_core(
		Command::new("java")
			.arg("-Xcheck:jni")
			.arg(format!("-Djava.library.path={}", api.lib.display()))
			.arg("-cp")
			.arg(lacking.join("classes"))
			.arg("api._Native"),
	)
	.output()
	.unwrap();
	failed(&output, "a _Native lacking its leaves");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		output.status.code().is_some()
			&& stderr.contains("java.lang.NoSuchMethodError")
			&& stderr.contains("at api._Native.__contract(Native Method)"),
		"a _Native lacking its leaves exited with {} and printed {stderr}",
		output.status
	);
}

/// The same script on Android: the classes, shrunk by R8 into an app's dex with only the
/// generated rules keeping what JNI looks up, run on ART under CheckJNI on the device adb
/// reaches against the library the NDK builds for its ABI, both for API 24, the oldest moq's
/// Android library supports, and matched to the script run natively there.
#[test]
#[ignore = "needs the Android SDK and NDK, cargo-ndk, and a device: just ffi android"]
fn api_android() {
	const DEVICE: &str = "/data/local/tmp/rust2-ffi";
	let sdk = PathBuf::from(std::env::var_os("ANDROID_HOME").expect("ANDROID_HOME names the Android SDK"));
	let adb = sdk.join("platform-tools").join("adb");
	let dir = scratch().join("api").join("android");
	let _ = std::fs::remove_dir_all(&dir);
	std::fs::create_dir_all(&dir).unwrap();

	let abi = run(Command::new(&adb).args(["shell", "getprop", "ro.product.cpu.abi"]));
	let abi = String::from_utf8_lossy(&abi.stdout).trim().to_string();
	// Exported for the device's target, the only one its crate builds for.
	let target = match abi.as_str() {
		"arm64-v8a" => "aarch64-linux-android",
		"x86_64" => "x86_64-linux-android",
		"x86" => "i686-linux-android",
		abi => panic!("the android shell's toolchain has no target for the device's ABI {abi}"),
	};
	let out = dir.join("out");
	let output = export("api", &out, &[target, WEB], &API_CARGO);
	assert!(
		output.status.success(),
		"exporting for {target} failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	run(cargo()
		.args(["ndk", "--target", &abi, "--platform", "24", "--output-dir"])
		.arg(dir.join("lib"))
		.args(["build", "--quiet", "--locked", "--manifest-path"])
		.arg(out.join("Cargo.toml"))
		// Apart from the host's glue, whose units share the export's names.
		.arg("--target-dir")
		.arg(dir.join("glue"))
		.args(["--features", "kotlin"])
		.env("RUSTFLAGS", "-D warnings"));

	// The fixture's script, built for the device as the library is, whose run there is what
	// the Kotlin must print: a 32-bit ABI's `usize` differs from the host's.
	let transcript = dir.join("transcript");
	run(cargo()
		.args([
			"ndk",
			"--target",
			&abi,
			"--platform",
			"24",
			"build",
			"--quiet",
			"--locked",
		])
		.args([
			"--package",
			"api-fixture",
			"--example",
			"transcript",
			"--features",
			"doubled",
		])
		.arg("--target-dir")
		.arg(&transcript)
		.current_dir(manifest_dir()));

	// Every segment aligns to the 16 KB pages Google Play requires of a 64-bit library from
	// 2027-02-01, whichever NDK built it: r27 and older align to 4 KB unless told otherwise.
	let lib = dir.join("lib").join(&abi).join("libapi.so");
	let segments = run(Command::new("readelf").args(["--wide", "--program-headers"]).arg(&lib));
	let segments = String::from_utf8_lossy(&segments.stdout);
	let aligns: Vec<u64> = segments
		.lines()
		.filter(|line| line.trim_start().starts_with("LOAD"))
		.map(|line| {
			let align = line.split_whitespace().last().unwrap();
			u64::from_str_radix(align.trim_start_matches("0x"), 16).unwrap()
		})
		.collect();
	assert!(!aligns.is_empty(), "readelf listed no LOAD segment:\n{segments}");
	assert!(
		aligns.iter().all(|&align| align >= 16384),
		"{} is not aligned to 16 KB pages:\n{segments}",
		lib.display()
	);

	// The app: the script's entry point, kept as an app keeps its own, and everything it
	// reaches, which R8 shrinks and renames, keeping only what the rules keep: the
	// generated ones, and those kotlinx-coroutines' jar holds, which an app's build reads
	// from it. Kotlin's annotations are on an app's classpath, not in its dex. The
	// script's functions marked `OwnFrame` are kept too, as R8 would inline them into their
	// caller, whose frame ART's interpreter keeps reaching what they held.
	let jar = dir.join("app.jar");
	run(kotlinc(&out, &kotlin_transcript(), &jar).arg("-include-runtime"));
	run(Command::new("jar")
		.arg("xf")
		.arg(coroutines())
		.arg("META-INF/com.android.tools/r8/coroutines.pro")
		.current_dir(&dir));
	let app = dir.join("app.pro");
	std::fs::write(
		&app,
		format!(
			"-keep class ApiKt {{ public static void main(java.lang.String[]); @OwnFrame static <methods>; }}\n-dontwarn org.jetbrains.annotations.**\n-printusage {}\n",
			dir.join("usage.txt").display()
		),
	)
	.unwrap();
	let dex = dir.join("dex");
	std::fs::create_dir_all(&dex).unwrap();
	run(Command::new("java")
		.arg("-cp")
		.arg(newest(&sdk.join("build-tools")).join("lib").join("d8.jar"))
		.args(["com.android.tools.r8.R8", "--release", "--min-api", "24", "--lib"])
		.arg(newest(&sdk.join("platforms")).join("android.jar"))
		.arg("--pg-conf")
		.arg(out.join("kotlin").join("api.pro"))
		.arg("--pg-conf")
		.arg(dir.join("META-INF/com.android.tools/r8/coroutines.pro"))
		.arg("--pg-conf")
		.arg(&app)
		.arg("--output")
		.arg(&dex)
		.arg(&jar)
		.arg(coroutines()));
	// R8 strips the generated Kotlin the script never reaches.
	let usage = std::fs::read_to_string(dir.join("usage.txt")).unwrap();
	assert!(
		usage.lines().any(|line| line.starts_with("api.")),
		"R8 stripped none of the generated Kotlin:\n{usage}"
	);
	// And keeps every leaf's `@FastNative`, which ART reads off the dex, and the method
	// itself, which `__contract` registers.
	let marked = std::fs::read_to_string(out.join("kotlin").join("api.kt"))
		.unwrap()
		.matches("@dalvik.annotation.optimization.FastNative\n")
		.count();
	assert!(marked > 0, "the api fixture has no leaf to mark");
	let dump = run(Command::new(newest(&sdk.join("build-tools")).join("dexdump"))
		.arg("-a")
		.arg(dex.join("classes.dex")));
	let kept = String::from_utf8_lossy(&dump.stdout)
		.matches("VISIBILITY_BUILD Ldalvik/annotation/optimization/FastNative;")
		.count();
	assert_eq!(kept, marked, "R8 kept {kept} of the {marked} @FastNative methods");

	run(Command::new(&adb)
		.args(["shell", "rm", "-rf", DEVICE, "&&", "mkdir", "-p"])
		.arg(format!("{DEVICE}/dalvik-cache")));
	run(Command::new(&adb)
		.arg("push")
		.arg(dex.join("classes.dex"))
		.arg(dir.join("lib").join(&abi).join("libapi.so"))
		.arg(
			transcript
				.join(target)
				.join("debug")
				.join("examples")
				.join("transcript"),
		)
		.arg(DEVICE));
	let native = run(Command::new(&adb).args(["shell", &format!("{DEVICE}/transcript")]));
	let native = String::from_utf8_lossy(&native.stdout);
	let art = |args: &[&str]| {
		let args: String = args.iter().map(|arg| format!(" '{arg}'")).collect();
		let mut cmd = Command::new(&adb);
		cmd.arg("shell").arg(format!(
			"ANDROID_DATA={DEVICE} dalvikvm -Xcheck:jni -Djava.library.path={DEVICE} -cp {DEVICE}/classes.dex ApiKt{args}"
		));
		cmd
	};
	kotlin(&native, &art);

	// An abort or a panic must reach logcat and bionic's abort message, which the tombstone
	// shows: an app's stderr goes nowhere. The tombstone is logged as the crash dump
	// finishes, after the process died, and one from an earlier run may still arrive, so the
	// case prints its process id, and the read takes the abort message that follows the
	// tombstone of that process.
	for (case, logged, tombstone) in [
		(
			"callback threw",
			"api::Log::log: a call into Kotlin threw",
			"Abort message: 'api::Log::log: a call into Kotlin threw'",
		),
		(
			"panicked",
			"called `Option::unwrap()` on a `None` value",
			"Abort message: 'panicked at",
		),
	] {
		run(Command::new(&adb).args(["logcat", "-c"]));
		let mut cmd = Command::new(&adb);
		cmd.arg("shell").arg(format!(
			"echo $$; ANDROID_DATA={DEVICE} exec dalvikvm -Djava.library.path={DEVICE} -cp {DEVICE}/classes.dex ApiKt '{case}'"
		));
		let output = self::output(&mut cmd);
		failed(&output, case);
		let stdout = String::from_utf8_lossy(&output.stdout);
		let pid = stdout.lines().next().unwrap_or_default().trim().to_string();
		assert!(
			pid.parse::<u32>().is_ok(),
			"{case} printed no process id first: {stdout}"
		);
		let ours = format!("pid: {pid},");
		let log = run(Command::new(&adb).args(["logcat", "-d", "-s", "rust2-ffi:F"]));
		let log = String::from_utf8_lossy(&log.stdout);
		assert!(log.contains(logged), "{case} logged {log}");
		// Read the dump as it arrives, bounded by the device-side timeout.
		let mut reader = Command::new(&adb)
			.args(["shell", "timeout", "120", "logcat", "-s", "DEBUG:F"])
			.stdout(std::process::Stdio::piped())
			.spawn()
			.unwrap();
		let mut found = false;
		let mut dump = String::new();
		for line in std::io::BufRead::lines(std::io::BufReader::new(reader.stdout.take().unwrap())) {
			let line = line.unwrap();
			found |= line.contains(&ours);
			if found && line.contains("Abort message") {
				dump = line;
				break;
			}
		}
		let _ = reader.kill();
		let _ = reader.wait();
		assert!(dump.contains(tombstone), "{case}'s tombstone showed {dump:?}");
	}
}

/// The transcript as Kotlin, written beside the scratch directory's other files, whose
/// `transcript()` `tests/api.kt`'s script calls.
fn kotlin_transcript() -> PathBuf {
	generated("kotlin", "Transcript.kt", &transcript::kotlin::emit(transcript())).join("Transcript.kt")
}

/// The gate's Kotlin protocol scripts, fewer than C's, as each costs the JVM more.
const KOTLIN_PROTOCOL_SEEDS: u64 = 100;

/// `tests/protocol.kt`, compiled into `classes` with the api fixture's Kotlin, on the JVM.
fn protocol_kotlin(api: &Api, classes: &Path) -> Command {
	let mut cmd = Command::new("kotlin");
	cmd.arg(format!("-Djava.library.path={}", api.lib.display()))
		.arg("-cp")
		.arg(std::env::join_paths([classes.as_os_str(), &coroutines()]).unwrap())
		.arg("ProtocolKt");
	cmd
}

/// Longer Kotlin protocol scripts from other seeds, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_kotlin_protocol_nightly() {
	let api = api();
	let classes = kotlin_classes(api);
	protocol::check_all(
		&scratch().join("api").join("protocol").join("kotlin-nightly"),
		&protocol::scripts(PROTOCOL_SEEDS, 2000, 400, protocol::Profile::KOTLIN),
		|| protocol_kotlin(api, &classes),
	);
}

/// The api fixture's Kotlin, `tests/api.kt`, the transcript's script, and
/// `tests/protocol.kt`, compiled.
fn kotlin_classes(api: &Api) -> PathBuf {
	let classes = scratch().join("api").join("kotlin");
	let jar = PathBuf::from(coroutines());
	let script = kotlin_transcript();
	Step {
		stamp: &step::stamp(&classes),
		reads: &[
			&api.out.join("kotlin").join("api.kt"),
			&api.out.join("kotlin").join("FastNative.kt"),
			&manifest_dir().join("tests").join("api.kt"),
			&script,
			&manifest_dir().join("tests").join("protocol.kt"),
			&jar,
		],
		writes: &[&classes],
	}
	.run(&mut kotlinc(&api.out, &script, &classes));
	classes
}

/// Each JVM name the api fixture's lock records is a public method of its class, as javap
/// reads `classes`, the compiled Kotlin: the class of the type the function is a member of,
/// or the file's for one at the top level.
fn jvm_names_exist(api: &Api, classes: &Path) {
	let json: serde_json::Value =
		serde_json::from_slice(&std::fs::read(api.out.join("api.json")).unwrap()).expect("the export's API");
	let names = json["names"]["Kotlin"].as_object().expect("Kotlin's names");
	let jvm: Vec<(String, &str)> = (names.iter())
		.filter_map(|(key, jvm)| {
			let (_, path) = key.strip_suffix(" / jvm")?.split_once(' ')?;
			// The last `::` outside a generic method's arguments, as `api::Area::scaled<api::Point>`'s.
			let mut depth = 0;
			let at = (path.char_indices()).rfind(|&(k, c)| {
				depth += match c {
					'>' => 1,
					'<' => -1,
					_ => 0,
				};
				depth == 0 && path[k..].starts_with("::")
			})?;
			let owner = &path[..at.0];
			let class = match names.get(&format!("type {owner}")) {
				Some(class) => format!("api.{}", class.as_str().unwrap()),
				None => "api.ApiKt".to_string(),
			};
			Some((class, jvm.as_str().unwrap()))
		})
		.collect();
	assert!(jvm.len() > 100, "the lock records JVM names: {}", jvm.len());
	// Every class the generated Kotlin declares, but an anonymous one, such as a lambda's.
	let all: Vec<String> = (std::fs::read_dir(classes.join("api")).unwrap())
		.filter_map(|entry| {
			let file = entry.unwrap().file_name().into_string().unwrap();
			let class = file.strip_suffix(".class")?;
			let anonymous = (class.split('$').skip(1)).any(|part| part.starts_with(|c: char| c.is_ascii_digit()));
			(!anonymous).then(|| format!("api.{class}"))
		})
		.collect();
	let javap = run(Command::new("javap").arg("-public").arg("-cp").arg(classes).args(&all));
	// Each class, by its header, and the public methods and constructors javap lists of it.
	let mut declared: Vec<(String, String, Vec<String>)> = Vec::new();
	for line in String::from_utf8_lossy(&javap.stdout).lines() {
		let words: Vec<&str> = line.split(['(', ' ']).collect();
		if let Some(at) = words.iter().position(|w| ["class", "interface"].contains(w)) {
			let class = words[at + 1].split('<').next().unwrap().to_string();
			declared.push((class, line.to_string(), Vec::new()));
		} else if line.contains('(') {
			declared.last_mut().unwrap().2.push(line.trim().to_string());
		}
	}
	let name = |method: &str| {
		method
			.split_once('(')
			.unwrap()
			.0
			.rsplit(' ')
			.next()
			.unwrap()
			.to_string()
	};
	let methods: BTreeSet<(String, String)> = (declared.iter())
		.flat_map(|(class, _, methods)| methods.iter().map(|m| (class.clone(), name(m))))
		.collect();
	let missing: Vec<&(String, &str)> = (jvm.iter())
		.filter(|(class, name)| !methods.contains(&(class.clone(), name.to_string())))
		.collect();
	assert!(
		missing.is_empty(),
		"JVM names the lock records that no class has: {missing:?}"
	);
	java_reaches(&declared);
}

/// No public method of the api fixture's compiled Kotlin, each class of `declared` by its
/// header and its methods as javap lists them, is one only Kotlin calls, but its own form of
/// one plain Java reaches otherwise: Java names no method that is a keyword or no
/// identifier, such as Kotlin's mangled `count-pVg5ArA`, and calls no constructor taking a
/// `DefaultConstructorMarker`, which Kotlin makes of one taking an inline class.
fn java_reaches(declared: &[(String, String, Vec<String>)]) {
	const KEYWORDS: &[&str] = &[
		"_",
		"abstract",
		"assert",
		"boolean",
		"break",
		"byte",
		"case",
		"catch",
		"char",
		"class",
		"const",
		"continue",
		"default",
		"do",
		"double",
		"else",
		"enum",
		"extends",
		"false",
		"final",
		"finally",
		"float",
		"for",
		"goto",
		"if",
		"implements",
		"import",
		"instanceof",
		"int",
		"interface",
		"long",
		"native",
		"new",
		"null",
		"package",
		"private",
		"protected",
		"public",
		"return",
		"short",
		"static",
		"strictfp",
		"super",
		"switch",
		"synchronized",
		"this",
		"throw",
		"throws",
		"transient",
		"true",
		"try",
		"void",
		"volatile",
		"while",
	];
	let classes: BTreeSet<&str> = declared.iter().map(|(class, ..)| class.as_str()).collect();
	let mut unreachable = Vec::new();
	for (class, header, methods) in declared {
		// The runtime's internals, which only the generated code calls.
		if class.starts_with("api._Native") {
			continue;
		}
		let value = methods.iter().any(|m| m.contains(" box-impl("));
		let name = |method: &str| {
			method
				.split_once('(')
				.unwrap()
				.0
				.rsplit(' ')
				.next()
				.unwrap()
				.to_string()
		};
		let java = |name: &str| {
			name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
				&& !name.starts_with(|c: char| c.is_ascii_digit())
				&& !KEYWORDS.contains(&name)
		};
		// The methods Java calls, each by its name and whether it is static.
		let called: BTreeSet<(String, bool)> = (methods.iter())
			.map(|m| (name(m), m.contains(" static ")))
			.filter(|(name, _)| java(name))
			.collect();
		// Whether Java calls a twin of `method`, the Kotlin form `name`. One the generator pins
		// as `name-kotlin` has a bridge of the same kind named `name`, but `default`'s, which
		// Java reserves, is `default_`. A value class's member, which Kotlin mangles itself, has
		// a bridge named as it is before the mangling, or with the `_`s that keep it apart from
		// another method of its class, or for a getter, as its property is.
		let twin = |method: &str, name: &str| {
			if let Some(base) = name.strip_suffix("-kotlin") {
				let base = if base == "default" { "default_" } else { base };
				return called.contains(&(base.to_string(), method.contains(" static ")));
			}
			let base = name.split('-').next().unwrap();
			let property = (base.strip_prefix("get"))
				.filter(|rest| rest.starts_with(|c: char| c.is_ascii_uppercase()))
				.map(|rest| rest[..1].to_ascii_lowercase() + &rest[1..]);
			let apart = |called: &str, base: &str| {
				called
					.strip_prefix(base)
					.is_some_and(|rest| rest.chars().all(|c| c == '_'))
			};
			(called.iter())
				.any(|(called, _)| apart(called, base) || property.as_ref().is_some_and(|p| apart(called, p)))
		};
		let mut made = false;
		for method in methods {
			let name = name(method);
			let name = name.as_str();
			if name == class {
				made |= !method.ends_with("kotlin.jvm.internal.DefaultConstructorMarker);");
				continue;
			}
			made |= method.contains(" static ") && method.contains(&format!(" {class} of("));
			let base = name.split('-').next().unwrap();
			let kotlin = [
				// A value class's box, and what it overrides of `Object`'s.
				value && ["constructor", "box", "unbox", "equals", "hashCode", "toString"].contains(&base),
				// A value class's member, a function Java reaches through its bridge, or a getter
				// Kotlin mangles, which overrides one, so takes no `@JvmName`.
				(name.ends_with("-impl") || name.ends_with("-kotlin") || name.starts_with("get") || value)
					&& twin(method, name),
				// What Kotlin declares of a data class.
				name.starts_with("component") || name.starts_with("copy-"),
				// A callback interface's method, which Java implements through `Java`.
				method.contains(" abstract ") && classes.contains(format!("{class}$Java").as_str()),
				// The codecs.
				class == "api.ApiKt" && (name.starts_with("w_") || name.starts_with("r_")),
				name.contains('$'),
				// The runtime's sealed interface, whose port Java reads through a static of each
				// class implementing it.
				class == "api.SocketAddr" && name.starts_with("getPort-"),
				// The runtime's conversion to Kotlin's own value class.
				class == "api.Duration" && name.starts_with("toKotlinDuration-"),
			];
			if !java(name) && !kotlin.contains(&true) {
				unreachable.push(format!("{class}: {method}"));
			}
		}
		// A data class or an exception Java makes.
		let data = methods.iter().any(|m| m.contains(" copy(") || m.contains(" copy-"));
		if (data || header.contains(" extends java.lang.Exception")) && !made {
			unreachable.push(format!("{class}: no constructor"));
		}
	}
	assert!(
		unreachable.is_empty(),
		"what plain Java cannot call:\n{}",
		unreachable.join("\n")
	);
}

/// kotlinc compiling the generated Kotlin with `tests/api.kt`, the transcript `script`,
/// and `tests/protocol.kt` into `dest`.
fn kotlinc(out: &Path, script: &Path, dest: &Path) -> Command {
	let mut cmd = Command::new("kotlinc");
	cmd.args(["-Werror", "-cp"])
		.arg(coroutines())
		// The generated Kotlin, `api.kt` and `FastNative.kt`.
		.arg(out.join("kotlin"))
		.arg(manifest_dir().join("tests").join("api.kt"))
		.arg(script)
		.arg(manifest_dir().join("tests").join("protocol.kt"))
		.arg("-d")
		.arg(dest);
	cmd
}

/// The Kotlin standard library's jar, beside the `kotlinc` on `PATH`, which plain Java
/// compiles and runs against.
fn kotlin_stdlib() -> PathBuf {
	let path = std::env::var_os("PATH").expect("PATH is set");
	let kotlinc = std::env::split_paths(&path)
		.map(|dir| dir.join("kotlinc"))
		.find(|bin| bin.is_file())
		.expect("kotlinc is on PATH: run this in the dev shell");
	let home = kotlinc.canonicalize().unwrap();
	let jar = home
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.join("lib")
		.join("kotlin-stdlib.jar");
	assert!(jar.is_file(), "{} is missing", jar.display());
	jar
}

/// kotlinx-coroutines' jar.
fn coroutines() -> OsString {
	std::env::var_os("KOTLINX_COROUTINES")
		.expect("KOTLINX_COROUTINES names kotlinx-coroutines' jar: run this in the dev shell")
}

/// The entry of `dir` of the highest version, as the SDK names its build tools and platforms.
fn newest(dir: &Path) -> PathBuf {
	let version = |path: &PathBuf| -> Vec<u32> {
		let name = path.file_name().unwrap().to_string_lossy().into_owned();
		name.split(|c: char| !c.is_ascii_digit())
			.filter_map(|n| n.parse().ok())
			.collect()
	};
	std::fs::read_dir(dir)
		.unwrap_or_else(|err| panic!("reading {}: {err}", dir.display()))
		.map(|entry| entry.unwrap().path())
		.max_by_key(version)
		.unwrap_or_else(|| panic!("{} is empty", dir.display()))
}

/// `tests/api.kt`'s script, run by `vm` with each argument list, prints the native
/// transcript; then its futures, what the GC frees, what only its types bound, and each
/// contract it fails loud on.
fn kotlin(native: &str, vm: &dyn Fn(&[&str]) -> Command) {
	let kt = run(&mut vm(&[]));
	jni_checked(&kt, "the Kotlin transcript");
	assert_eq!(
		String::from_utf8_lossy(&kt.stdout),
		native,
		"the Kotlin transcript differs from the native one"
	);
	for case in ["futures", "owned", "clash", "checks"] {
		jni_checked(&run(&mut vm(&[case])), case);
	}
	for (case, message) in [
		("moved", "IllegalStateException: Counter is closed"),
		("boxed moved", "IllegalStateException: Node is closed"),
		("lent kept", "Counter is closed"),
		(
			"lent moved",
			"api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
		),
		("panicked", "called `Option::unwrap()` on a `None` value"),
		("callback threw", "api::Log::log: a call into Kotlin threw"),
		("closure threw", "api_made: a call into Kotlin threw"),
		("closure failed", "api_drive: a call into Kotlin threw"),
		("reader threw", "SeekableByteChannel.read: a call into Kotlin threw"),
		("outlived", "api_Meter_count: a handle lent from another outlived it"),
	] {
		// Under a piped core pattern the handler still runs at a limit of 0, so the
		// status reports a core though none is written.
		let output = no_core(&mut vm(&[case])).output().unwrap();
		assert!(!output.status.success(), "Kotlin {case} did not fail");
		// A throwing closure is where a JNI call may follow a pending exception.
		jni_checked(&output, case);
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Kotlin {case} printed {stderr}");
	}
}

/// Asserts the run of `what` drew no warning from HotSpot's CheckJNI, which only warns of
/// a JNI call while an exception may be pending or of a local reference past its frame's
/// capacity, and on stdout. ART's CheckJNI aborts instead.
fn jni_checked(output: &Output, what: &str) {
	for printed in [&output.stdout, &output.stderr] {
		let printed = String::from_utf8_lossy(printed);
		assert!(
			!printed.contains("WARNING in native method") && !printed.contains("WARNING: JNI"),
			"CheckJNI warned in {what}:\n{printed}"
		);
	}
}

/// Asserts the run of `what` failed, as a broken contract must, and on Linux, where the test
/// programs make themselves non-dumpable, without dumping core: each core costs the host a
/// write and a run of its crash handler. Elsewhere the host's own core limit decides.
fn failed(output: &Output, what: &str) {
	assert!(!output.status.success(), "{what} did not fail");
	#[cfg(target_os = "linux")]
	assert!(
		!std::os::unix::process::ExitStatusExt::core_dumped(&output.status),
		"{what} dumped core"
	);
}

/// `cmd` with a core-size limit of 0 on Linux, so a JVM case that aborts writes no core. The
/// JVM cannot mark itself non-dumpable as the other test programs do, since JDK 21 calls C
/// only through a preview API, so the host's crash handler still starts and logs the crash.
fn no_core(cmd: &mut Command) -> &mut Command {
	#[cfg(target_os = "linux")]
	{
		use std::ffi::{c_int, c_ulong};
		use std::os::unix::process::CommandExt;

		#[repr(C)]
		struct Rlimit {
			cur: c_ulong,
			max: c_ulong,
		}
		unsafe extern "C" {
			fn setrlimit(resource: c_int, limit: *const Rlimit) -> c_int;
		}
		const RLIMIT_CORE: c_int = 4;
		// SAFETY: setrlimit is async-signal-safe and touches only the child.
		unsafe {
			cmd.pre_exec(|| {
				if setrlimit(RLIMIT_CORE, &Rlimit { cur: 0, max: 0 }) == 0 {
					Ok(())
				} else {
					Err(std::io::Error::last_os_error())
				}
			});
		}
	}
	cmd
}

/// The library `python/api.py` loads from beside itself.
const PYTHON_LIB: &str = match (cfg!(windows), cfg!(target_os = "macos")) {
	(true, _) => "api.dll",
	(_, true) => "libapi.dylib",
	_ => "libapi.so",
};

/// `tests/api.py`'s script, run by `PYTHON` or `python3`, prints the native transcript; then
/// its futures, what frees what nobody closed, the values Python's types bound, and each
/// contract it fails loud on.
#[test]
fn api_python() {
	python_checks(&interpreter("PYTHON", "python3"), &api().out.join("python"), &[]);
}

/// The CPython the environment variable `var` names, which the dev shell sets by path, as
/// each version's `python3` would shadow another's on `PATH`, or else `default`.
fn interpreter(var: &str, default: &str) -> std::ffi::OsString {
	std::env::var_os(var).unwrap_or_else(|| default.into())
}

/// The api fixture's Python beside its extension built with `python-abi3t`, on `abi3t`.
fn abi3t() -> &'static Path {
	static DIR: OnceLock<PathBuf> = OnceLock::new();
	DIR.get_or_init(|| python_abi3t("python-abi3t", &[], ""))
}

/// The api fixture's Python in `scratch/api/<name>`, beside its extension built on `abi3t`
/// with cargo's `args` and `rustflags`.
fn python_abi3t(name: &str, args: &[&str], rustflags: &str) -> PathBuf {
	let api = api();
	let target = scratch().join("api").join(format!("glue-{name}"));
	run(sysconfig_free(cargo())
		.args(["build", "--quiet", "--locked", "--features", "python-abi3t"])
		.args(args)
		.arg("--manifest-path")
		.arg(api.out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(&target)
		.env("RUSTFLAGS", format!("-D warnings {rustflags}"))
		// As the release builds it, with no CPython to ask.
		.env_remove("PYO3_PYTHON")
		.env("PYO3_NO_PYTHON", "1"));
	// A `--target` puts the build under the target's directory.
	let built = match args.iter().position(|arg| *arg == "--target") {
		Some(at) => target.join(args[at + 1]),
		None => target,
	};
	let dir = scratch().join("api").join(name);
	step::copy(&built.join("debug").join(PYTHON_LIB), &dir.join(PYTHON_LIB));
	step::copy(&api.out.join("python").join("api.py"), &dir.join("api.py"));
	dir
}

/// `cmd` without the sysconfig data a Nix shell names for its own CPython, which another
/// version, or a free-threaded build, has none of.
fn sysconfig_free(mut cmd: Command) -> Command {
	cmd.env_remove("_PYTHON_SYSCONFIGDATA_NAME")
		.env_remove("_PYTHON_HOST_PLATFORM");
	cmd
}

/// `api_python`'s checks through the extension built on `abi3t`, threads included, on the
/// free-threaded CPython 3.15 `PYTHON_315T` names, with the GIL off; then the transcript on
/// the GIL's CPython 3.15, `PYTHON_315`, through that library, which serves it too, and
/// through the `abi3` one, which it loads by the same export.
#[test]
fn api_abi3t() {
	let dir = abi3t();
	python_checks(&interpreter("PYTHON_315T", "python3.15t"), dir, &[]);
	for dir in [dir, &api().out.join("python")] {
		let output = run(sysconfig_free(Command::new(interpreter("PYTHON_315", "python3.15")))
			.args(["-P", "-X", "dev", "-W", "error"])
			.arg(manifest_dir().join("tests").join("api.py"))
			.env("PYTHONPATH", std::env::join_paths([dir, &python_transcript()]).unwrap()));
		assert_eq!(
			String::from_utf8_lossy(&output.stdout),
			api().transcript,
			"the Python transcript from {} on CPython 3.15 differs from the native one",
			dir.display()
		);
	}
}

/// `api_python`'s checks on a debug build of the newest CPython, which asserts what a
/// release build trusts of the runtime's reference counting, then each of them looped,
/// leaving the interpreter's references and memory blocks as they were.
#[test]
#[ignore = "needs a debug CPython, which builds from source: just ffi sanitize"]
fn api_python_debug() {
	let interpreter = std::env::var_os("RUST2_PYTHON_DEBUG")
		.expect("RUST2_PYTHON_DEBUG names a debug CPython: run this in the sanitize shell");
	python_checks(&interpreter, &api().out.join("python"), &[]);
	let output = run(Command::new(&interpreter)
		.args(["-P", "-X", "dev", "-W", "error"])
		.arg(manifest_dir().join("tests").join("api.py"))
		.arg("leaks")
		.env(
			"PYTHONPATH",
			std::env::join_paths([api().out.join("python"), python_transcript()]).unwrap(),
		));
	assert!(
		output.stdout.is_empty(),
		"leaks printed {}",
		String::from_utf8_lossy(&output.stdout)
	);
}

/// `api_python`'s checks, threads included, through the `abi3t` library built under
/// ThreadSanitizer, on the free-threaded CPython 3.15 built under it that `RUST2_PYTHON_TSAN`
/// names, where the GIL serializes nothing: a race in the runtime fails the run.
#[test]
#[ignore = "needs a ThreadSanitizer CPython, which builds from source: just ffi sanitize"]
fn api_python_tsan() {
	let interpreter = std::env::var_os("RUST2_PYTHON_TSAN")
		.expect("RUST2_PYTHON_TSAN names a ThreadSanitizer CPython: run this in the sanitize shell");
	// As `sanitized_glue` builds it, for the same reason.
	let outline = if cfg!(target_arch = "aarch64") {
		" -Ctarget-feature=-outline-atomics"
	} else {
		""
	};
	let dir = python_abi3t(
		"python-tsan",
		&["-Zbuild-std", "--target", host()],
		&format!("-Zsanitizer=thread{outline}"),
	);
	python_checks(&interpreter, &dir, &[("TSAN_OPTIONS", "halt_on_error=1")]);
}

/// The directory of the Python script generated from the transcript, which `tests/api.py`
/// imports.
fn python_transcript() -> PathBuf {
	generated("python", "transcript.py", &transcript::python::emit(transcript()))
}

/// `api_python`'s checks, run by `interpreter` with `env` on the module and library in `dir`.
fn python_checks(interpreter: &std::ffi::OsStr, dir: &Path, env: &[(&str, &str)]) {
	let api = api();
	let scripts = python_transcript();
	let python = |dir: &Path, args: &[&str]| {
		let mut cmd = sysconfig_free(Command::new(interpreter));
		// `-P` keeps `tests/` off the path, where `api.py` is this script; dev mode and
		// warnings as errors fail on what the runtime only warns of.
		cmd.args(["-P", "-X", "dev", "-W", "error"])
			.arg(manifest_dir().join("tests").join("api.py"))
			.args(args)
			.env("PYTHONPATH", std::env::join_paths([dir, &scripts]).unwrap())
			.envs(env.iter().copied());
		cmd
	};
	let py = run(&mut python(dir, &[]));
	assert_eq!(
		String::from_utf8_lossy(&py.stdout),
		api.transcript,
		"the Python transcript differs from the native one"
	);
	for case in [
		"futures", "owned", "values", "calls", "checks", "leaf", "idioms", "threads",
	] {
		run(&mut python(dir, &[case]));
	}
	for (case, message) in [
		("moved", "ValueError: api_Counter_get: a handle's value was moved out"),
		(
			"boxed moved",
			"ValueError: api_Node_sum: a handle's value was moved out",
		),
		("lent kept", "ValueError: Counter is closed"),
		("lent held", "Counter: a call still holds it when its lend ends"),
		(
			"lent moved",
			"RuntimeError: api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
		),
		("callback threw", "api::Log::log: a call into Python raised"),
		("foreign cancelled", "a future Rust polls raised"),
		(
			"outlived",
			"ValueError: api_Meter_count: a handle lent from another outlived it",
		),
		("family", "ValueError: an IPv6 address has no Rust SocketAddrV4"),
		("ip family", "ValueError: an IPv6 address has no Rust Ipv4Addr"),
		(
			"ip scope",
			"ValueError: an IPv6 address with a scope has no Rust IpAddr",
		),
		("kept threw", "api_Spawned_new: a call into Python raised"),
		("returned", "api_made: a call into Python raised"),
		("reader kept", "read: a reader kept a view of its buffer past the read"),
	] {
		let output = python(dir, &[case]).output().unwrap();
		failed(&output, &format!("Python {case}"));
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Python {case} printed {stderr}");
	}

	// The same Python, with another contract than the library's in place of its own,
	// refuses to load it.
	let mismatched = dir.with_file_name(format!("{}-mismatched", dir.file_name().unwrap().display()));
	step::copy(&dir.join(PYTHON_LIB), &mismatched.join(PYTHON_LIB));
	let source = std::fs::read_to_string(dir.join("api.py")).unwrap();
	let at = source
		.find("_n.contract() != 0x")
		.expect("the Python checks its contract")
		+ 19;
	let flipped = match &source[at..at + 1] {
		"0" => "1",
		_ => "0",
	};
	let source = format!("{}{flipped}{}", &source[..at], &source[at + 1..]);
	step::write(&mismatched.join("api.py"), source);
	let output = python(&mismatched, &[]).output().unwrap();
	failed(&output, "a mismatched contract");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.py and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// A CPython refuses a library it cannot run with an `ImportError`, where loading it by its
/// path would crash: a free-threaded one before 3.15, `PYTHON_314T`, any, which `api.py`
/// refuses, as no stable ABI covers it; the free-threaded 3.15, `PYTHON_315T`, the `abi3`
/// library, whose declared ABI it checks; and one before 3.15 the `abi3t` library, whose
/// symbols it lacks.
#[test]
fn api_free_threaded() {
	let probe = |python: &std::ffi::OsStr, dir: &Path, code: &str| {
		let mut cmd = sysconfig_free(Command::new(python));
		// `-P` keeps the working directory off the path; PYTHONPATH replaces the dev shell's,
		// which names the GIL build's packages. An inherited `PYTHON_GIL=1` would turn the GIL
		// on in a free-threaded build.
		cmd.args(["-P", "-c", code])
			.env("PYTHONPATH", dir)
			.env_remove("PYTHON_GIL");
		cmd.output().unwrap_or_else(|err| panic!("running {cmd:?}: {err}"))
	};
	let refused = |python: &std::ffi::OsStr, dir: &Path, message: &str| {
		let output = probe(python, dir, "import api");
		// Exit 1 is an uncaught exception, where a crash has no code.
		assert_eq!(output.status.code(), Some(1), "importing under {python:?}: {output:?}");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "importing under {python:?} printed {stderr}");
	};
	let abi3 = api().out.join("python");
	for python in [
		interpreter("PYTHON_314T", "python3.14t"),
		interpreter("PYTHON_315T", "python3.15t"),
	] {
		// A GIL build would pass for the refusal's test, so check the interpreter first.
		let build = probe(&python, &abi3, "import sys; sys.exit(sys._is_gil_enabled())");
		assert!(
			build.status.success(),
			"{python:?} is not a free-threaded CPython: {}",
			String::from_utf8_lossy(&build.stderr)
		);
	}
	refused(
		&interpreter("PYTHON_314T", "python3.14t"),
		&abi3,
		"ImportError: api.py loads a library for CPython's stable ABI, which a free-threaded CPython has from 3.15",
	);
	refused(
		&interpreter("PYTHON_315T", "python3.15t"),
		&abi3,
		"ImportError: _api: incompatible with free-threaded CPython",
	);
	// The loader refuses the first `abi3t` symbol a CPython before 3.15 lacks, or on Windows
	// the free-threaded `python3t.dll` the library links.
	let missing = match (cfg!(windows), cfg!(target_os = "macos")) {
		(true, _) => "ImportError: DLL load failed while importing".to_string(),
		(_, true) => "symbol not found in flat namespace '_Py".to_string(),
		_ => format!("{PYTHON_LIB}: undefined symbol: Py"),
	};
	refused(&interpreter("PYTHON", "python3"), abi3t(), &missing);

	// The suffixes of the platforms this host is not, appended on the GIL build before 3.15,
	// which loads from beside the module whatever its list holds.
	let gil = |suffix: &str| {
		let mut cmd = sysconfig_free(Command::new(interpreter("PYTHON", "python3")));
		cmd.args(["-P", "-c"])
			.arg(format!(
				"import importlib.machinery as m; m.EXTENSION_SUFFIXES.append('{suffix}'); import api"
			))
			.env("PYTHONPATH", &abi3);
		cmd.output().unwrap_or_else(|err| panic!("running {cmd:?}: {err}"))
	};
	for suffix in [
		".cp314t-win_amd64.pyd",
		"_d.cp314t-win_amd64.pyd",
		".cpython-313t-darwin.so",
		".cpython-314t-aarch64-linux-gnu.so",
		".cpython-314td-x86_64-linux-gnu.so",
	] {
		let output = gil(suffix);
		assert_eq!(output.status.code(), Some(1), "{suffix} was not refused: {output:?}");
		assert!(
			String::from_utf8_lossy(&output.stderr).contains("which a free-threaded CPython has from 3.15"),
			"{suffix} printed {output:?}"
		);
	}
	for suffix in [
		".cp314-win_amd64.pyd",
		"_d.cp314-win_amd64.pyd",
		".cpython-314-darwin.so",
		".cpython-314d-x86_64-linux-gnu.so",
	] {
		let output = gil(suffix);
		assert!(output.status.success(), "{suffix} was refused: {output:?}");
	}
}

/// Random sequences of the ownership protocol's steps through the Python, by
/// `tests/protocol.py`, each matching what the model says each step returns.
#[test]
fn api_python_protocol() {
	python_protocol(&protocol::scripts(0, 40, 100, protocol::Profile::PYTHON), "scripts");
}

/// Longer Python protocol scripts from other seeds, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_python_protocol_nightly() {
	python_protocol(&protocol::scripts(40, 1000, 400, protocol::Profile::PYTHON), "nightly");
}

fn python_protocol(scripts: &[protocol::Script], dir: &str) {
	let api = api();
	let python = std::env::var_os("PYTHON").unwrap_or_else(|| "python3".into());
	protocol::check(
		&scratch().join("api").join("python-protocol").join(dir),
		scripts,
		|_| {
			let mut cmd = Command::new(&python);
			// As `api_python` runs its script: off the path, in dev mode, warnings as errors.
			cmd.args(["-P", "-X", "dev", "-W", "error"])
				.arg(manifest_dir().join("tests").join("protocol.py"))
				.env("PYTHONPATH", api.out.join("python"));
			cmd
		},
	);
}

/// The api fixture's Python, and `tests/api_types.py` calling it as a typed caller does,
/// pass mypy and pyright, both strict, at the oldest CPython its extension loads in, and
/// `tests/bare_builtins.py` finds it reading no builtin by its bare name. Apart from
/// `api_python`, which the runners without the dev shell's checkers run alone.
#[test]
fn api_py_types() {
	let dir = scratch().join("api").join("python-types");
	let module = dir.join("api.py");
	let caller = dir.join("api_types.py");
	step::copy(&api().out.join("python").join("api.py"), &module);
	step::copy(&manifest_dir().join("tests").join("api_types.py"), &caller);
	run(
		Command::new(std::env::var_os("PYTHON").unwrap_or_else(|| "python3".into()))
			.arg("-I")
			.arg(manifest_dir().join("tests").join("bare_builtins.py"))
			.arg(&module),
	);
	// Rooted in `dir`, so `import api` finds the module rather than `tests/api.py`.
	let config = dir.join("pyrightconfig.json");
	step::copy(&manifest_dir().join("tests").join("pyright.json"), &config);
	let mut mypy = Command::new("mypy");
	mypy.args(["--strict", "--python-version", "3.11", "--cache-dir"])
		.arg(scratch().join("mypy"))
		.args([&module, &caller]);
	let mut pyright = Command::new("pyright");
	pyright.arg("-p").arg(&config).args([&module, &caller]);
	for (checker, mut cmd) in [("mypy", mypy), ("pyright", pyright)] {
		let step = Step {
			stamp: &dir.join(format!("{checker}.stamp")),
			reads: &[&module, &caller, &config],
			writes: &[],
		};
		step.run(&mut cmd);
	}
}

/// The contract a Kotlin file checks its library against.
fn kotlin_contract(kotlin: &Path) -> u64 {
	let kotlin = std::fs::read_to_string(kotlin).unwrap();
	let at = kotlin
		.find("__contract().toULong() == 0x")
		.expect("the Kotlin checks its contract");
	let hex = &kotlin[at..].split("0x").nth(1).expect("a hex contract")[..16];
	u64::from_str_radix(hex, 16).unwrap()
}

/// The generated Swift, built as the module `Api` with warnings as errors in Swift 6's
/// language mode and type-checked in Swift 5's, which moq's package builds in, runs
/// `tests/api.swift`'s script, its futures, and what releasing frees, then fails loud on
/// each broken contract and on a library of another contract. Each contract Swift's
/// compiler checks, a move or a borrow, fails to compile instead.
#[test]
fn api_swift() {
	let api = api();
	let SwiftApi {
		dir,
		module,
		include,
		source,
		collections,
		built,
		swiftmodule,
		lib,
	} = swift_api(api);
	let step = Step {
		stamp: &dir.join("typecheck.stamp"),
		reads: &[&module, &include, &built],
		writes: &[],
	};
	step.run(
		swiftc(&module, 13)
			.args(["-swift-version", "5", "-typecheck"])
			.arg(&source),
	);
	// The test's executor preference is macOS 15's.
	let exe = dir.join("api-test");
	let test = manifest_dir().join("tests").join("api.swift");
	let script = swift_transcript();
	// The Clang module the test and the contract program import on Linux.
	let nodump = manifest_dir().join("tests").join("nodump");
	let step = Step {
		stamp: &step::stamp(&exe),
		reads: &[
			&test,
			&script,
			&nodump,
			&module,
			&include,
			&built,
			&swiftmodule,
			&lib,
			&api.staticlib(),
		],
		writes: &[&exe],
	};
	step.run(
		swiftc(&module, 15)
			.args(["-parse-as-library", "-I"])
			.arg(&dir)
			.arg(&test)
			.arg(&script)
			.arg(&lib)
			.args(&collections)
			.arg(api.staticlib())
			.args(&api.native)
			.arg("-o")
			.arg(&exe),
	);
	let swift = run(&mut Command::new(&exe));
	assert_eq!(
		String::from_utf8_lossy(&swift.stdout),
		api.transcript,
		"the Swift transcript differs from the native one"
	);
	run(Command::new(&exe).arg("checks"));
	run(Command::new(&exe).arg("futures"));
	run(Command::new(&exe).arg("owned"));
	run(Command::new(&exe).arg("shared"));
	// What only wasm32 has, a program of its own, which the test leaves out so that it
	// compiles against the release builder's export, which has no wasm32.
	let targets_test = manifest_dir().join("tests").join("targets.swift");
	let targets = dir.join("targets-test");
	let step = Step {
		stamp: &step::stamp(&targets),
		reads: &[
			&targets_test,
			&nodump,
			&module,
			&include,
			&built,
			&swiftmodule,
			&lib,
			&api.staticlib(),
		],
		writes: &[&targets],
	};
	step.run(
		swiftc(&module, 15)
			.args(["-parse-as-library", "-I"])
			.arg(&dir)
			.arg(&targets_test)
			.arg(&lib)
			.args(&collections)
			.arg(api.staticlib())
			.args(&api.native)
			.arg("-o")
			.arg(&targets),
	);
	run(&mut Command::new(&targets));
	// Each in a build of its own, since the first error stops the rest; move checking runs
	// on SIL, which type-checking alone skips.
	for (case, message) in [
		("MOVED", "'c' used after consume"),
		("BOXED_MOVED", "'tail' used after consume"),
		(
			"LENT_MOVED",
			"missing reinitialization of inout parameter 'counter' after consume",
		),
		("LENT_KEPT", "implicit conversion to 'Tally?' is consuming"),
		("SPAN_KEPT", "lifetime-dependent variable 'kept' escapes its scope"),
	] {
		let step = Step {
			stamp: &dir.join(format!("{case}.stamp")),
			reads: &[&test, &script, &nodump, &module, &include, &built, &swiftmodule],
			writes: &[],
		};
		let output = step.output(
			swiftc(&module, 15)
				.args(["-parse-as-library", "-wmo", "-emit-sil", "-o"])
				.arg(dir.join("broken.sil"))
				.args(["-I"])
				.arg(&dir)
				.arg(format!("-D{case}"))
				.arg(&test)
				.arg(&script),
		);
		assert!(!output.status.success(), "Swift {case} compiled");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Swift {case} printed {stderr}");
	}
	for (program, case, message) in [
		(&exe, "outlived", OUTLIVED),
		(
			&targets,
			"variant",
			"api_Platform.Wasm: it is not on this library's target",
		),
		(
			&targets,
			"plain variant",
			"api_Family.Wasm: it is not on this library's target",
		),
	] {
		let output = Command::new(program).arg(case).output().unwrap();
		failed(&output, &format!("Swift {case}"));
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Swift {case} printed {stderr}");
	}

	// The same Swift, with another contract than the library's in place of its own, refuses
	// it at the first call.
	let generated = std::fs::read_to_string(&source).unwrap();
	let at = generated
		.find("api_contract() == 0x")
		.expect("the Swift checks its contract")
		+ "api_contract() == 0x".len();
	let mut mismatched = generated.clone();
	let digit = if &generated[at..at + 1] == "0" { "1" } else { "0" };
	mismatched.replace_range(at..at + 1, digit);
	let contract = dir.join("contract");
	let (swift, main) = (contract.join("api.swift"), contract.join("main.swift"));
	step::write(&swift, mismatched);
	// Non-dumpable before the check fails, as `broken` is.
	step::write(
		&main,
		"#if os(Linux)\nimport Glibc\nimport NoDump\n\nif nodump() != 0 {\n\tperror(\"prctl\")\n\texit(1)\n}\n#endif\n_ = dropped()\n",
	);
	let exe = contract.join("contract");
	let step = Step {
		stamp: &step::stamp(&exe),
		reads: &[&swift, &main, &nodump, &module, &include, &built, &api.staticlib()],
		writes: &[&exe],
	};
	step.run(
		swiftc(&module, 13)
			.args(["-module-name", "Api"])
			.arg(&swift)
			.arg(&main)
			.args(&collections)
			.arg(api.staticlib())
			.args(&api.native)
			.arg("-o")
			.arg(&exe),
	);
	let output = Command::new(&exe).output().unwrap();
	failed(&output, "a mismatched contract");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.swift and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// `tests/api.swift`'s script, its futures, and what releasing frees, under
/// ThreadSanitizer, against the glue built under it: it catches a race between a wake on
/// Rust's thread and the task it resumes, or an executor's job and a release.
#[test]
#[ignore = "needs the nightly's rust-src and Swift's ThreadSanitizer: just ffi sanitize"]
fn api_swift_tsan() {
	let api = api();
	let (glue, native) = sanitized_glue("thread");
	let dir = scratch().join("api").join("swift-thread");
	let module = api.out.join("swift");
	let collections = swift_collections();
	let (swiftmodule, lib) = (dir.join("Api.swiftmodule"), dir.join("libApi.a"));
	std::fs::create_dir_all(&dir).unwrap();
	run(swiftc(&module, 13)
		.args([
			"-sanitize=thread",
			"-swift-version",
			"6",
			"-parse-as-library",
			"-module-name",
			"Api",
		])
		.args(["-emit-module", "-emit-module-path"])
		.arg(&swiftmodule)
		.args(["-emit-library", "-static", "-o"])
		.arg(&lib)
		.arg(module.join("api.swift")));
	let exe = dir.join("api-test");
	run(swiftc(&module, 15)
		.args(["-sanitize=thread", "-parse-as-library", "-I"])
		.arg(&dir)
		.arg(manifest_dir().join("tests").join("api.swift"))
		.arg(swift_transcript())
		.arg(&lib)
		.args(&collections)
		.arg(&glue)
		.args(&native)
		.arg("-o")
		.arg(&exe));
	// What Swift's own runtime and Dispatch do goes unchecked, as Darwin's default has it:
	// they are uninstrumented, so TSan would miss the atomics that order a task's frame
	// before its runtime frees it, and report that free as a race.
	let env = [("TSAN_OPTIONS", "halt_on_error=1:ignore_noninstrumented_modules=1")];
	let output = run(Command::new(&exe).envs(env));
	assert_eq!(
		String::from_utf8_lossy(&output.stdout),
		api.transcript,
		"the Swift transcript under ThreadSanitizer differs from the native one"
	);
	for case in ["checks", "futures", "owned", "shared"] {
		run(Command::new(&exe).arg(case).envs(env));
	}
}

/// The Swift script generated from the transcript, which `tests/api.swift` calls.
fn swift_transcript() -> PathBuf {
	generated("swift", "transcript.swift", &transcript::swift::emit(transcript())).join("transcript.swift")
}

/// The api fixture's Swift, built as the module `Api` in Swift 6's language mode.
struct SwiftApi {
	dir: PathBuf,
	/// The generated Swift's directory, with the C header's module map.
	module: PathBuf,
	include: PathBuf,
	source: PathBuf,
	/// swift-collections' libraries, and where they were built.
	collections: Vec<PathBuf>,
	built: PathBuf,
	swiftmodule: PathBuf,
	lib: PathBuf,
}

fn swift_api(api: &Api) -> SwiftApi {
	let dir = scratch().join("api").join("swift");
	let module = api.out.join("swift");
	let include = api.out.join("include");
	let source = module.join("api.swift");
	let collections = swift_collections();
	let built = collections_dir();
	let (swiftmodule, lib) = (dir.join("Api.swiftmodule"), dir.join("libApi.a"));
	let step = Step {
		stamp: &dir.join("Api.stamp"),
		reads: &[&module, &include, &built],
		writes: &[&swiftmodule, &lib],
	};
	// Once per process: the Swift tests share its outputs, which a second build running
	// beside the first would remove.
	static BUILT: OnceLock<()> = OnceLock::new();
	BUILT.get_or_init(|| {
		std::fs::create_dir_all(&dir).unwrap();
		step.run(
			swiftc(&module, 13)
				.args(["-swift-version", "6", "-parse-as-library", "-module-name", "Api"])
				.args(["-emit-module", "-emit-module-path"])
				.arg(&swiftmodule)
				.args(["-emit-library", "-static", "-o"])
				.arg(&lib)
				.arg(&source),
		);
	});
	SwiftApi {
		dir,
		module,
		include,
		source,
		collections,
		built,
		swiftmodule,
		lib,
	}
}

/// Random sequences of the ownership protocol's steps through the Swift, by
/// `tests/protocol.swift`, each matching what the model says each step returns: none Swift's
/// compiler would refuse, and each future an async call awaited as it is made.
#[test]
fn api_swift_protocol() {
	swift_protocol(&protocol::scripts(
		0,
		PROTOCOL_SEEDS,
		PROTOCOL_STEPS,
		protocol::Profile::SWIFT,
	));
}

/// Longer protocol scripts through the Swift, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_swift_protocol_nightly() {
	swift_protocol(&protocol::scripts(PROTOCOL_SEEDS, 5000, 400, protocol::Profile::SWIFT));
}

fn swift_protocol(scripts: &[protocol::Script]) {
	let api = api();
	let swift = swift_api(api);
	let dir = scratch().join("api").join("protocol-swift");
	let exe = dir.join(format!("protocol{EXE_SUFFIX}"));
	let src = manifest_dir().join("tests").join("protocol.swift");
	let step = Step {
		stamp: &step::stamp(&exe),
		reads: &[
			&src,
			&swift.module,
			&swift.include,
			&swift.built,
			&swift.swiftmodule,
			&swift.lib,
			&api.staticlib(),
		],
		writes: &[&exe],
	};
	std::fs::create_dir_all(&dir).unwrap();
	// Its executor preference is macOS 15's.
	step.run(
		swiftc(&swift.module, 15)
			.args(["-parse-as-library", "-I"])
			.arg(&swift.dir)
			.arg(&src)
			.arg(&swift.lib)
			.args(&swift.collections)
			.arg(api.staticlib())
			.args(&api.native)
			.arg("-o")
			.arg(&exe),
	);
	protocol::check(&dir.join("scripts"), scripts, |_| Command::new(&exe));
}

/// Node's script running the WASI program its first argument names.
const WASI_MAIN: &str = "import { WASI } from 'node:wasi';\n\
	import { readFileSync } from 'node:fs';\n\
	const wasi = new WASI({ version: 'preview1', args: [], env: {}, returnOnExit: false });\n\
	const module = await WebAssembly.compile(readFileSync(process.argv[1]));\n\
	wasi.start(await WebAssembly.instantiate(module, wasi.getImportObject()));\n";

/// The fixture's script compiled for wasm32-wasip1 and run under Node's WASI: native Rust on
/// the target the web binding serves, whose `usize` is 32 bits.
fn wasm_transcript() -> String {
	let built = run(cargo()
		.args([
			"build",
			"--quiet",
			"--locked",
			"--package",
			"api-fixture",
			"--example",
			"transcript",
			"--features",
			"doubled",
			"--target",
			WEB,
			"--target-dir",
		])
		.arg(scratch().join("api").join("transcript-wasm"))
		.current_dir(manifest_dir()));
	assert!(built.status.success());
	let wasm = scratch()
		.join("api")
		.join("transcript-wasm")
		.join(WEB)
		.join("debug")
		.join("examples")
		.join("transcript.wasm");
	let transcript = run(Command::new("node")
		.args(["--no-warnings", "--input-type=module", "-e", WASI_MAIN])
		.arg(wasm));
	String::from_utf8_lossy(&transcript.stdout).into_owned()
}

/// The api fixture's web binding, beside copies of `tests/api.ts` and `tests/protocol.ts`
/// as a caller's project holds them, type-checked with no issue by `tsc` and by Deno, and the module the crate's `web` feature builds
/// for wasm32-wasip1, built once for each runtime's test.
struct Web {
	dir: PathBuf,
	wasm: PathBuf,
	/// What native Rust prints running the script on the binding's target.
	transcript: String,
}

fn web() -> &'static Web {
	static WEB_BUILT: OnceLock<Web> = OnceLock::new();
	WEB_BUILT.get_or_init(|| {
		let api = api();
		let dir = scratch().join("api").join("web");
		let target = scratch().join("api").join("glue-web");
		run(cargo()
			.args(["build", "--quiet", "--locked", "--manifest-path"])
			.arg(api.out.join("Cargo.toml"))
			.arg("--target-dir")
			.arg(&target)
			.args(["--target", WEB, "--features", "web"])
			.env("RUSTFLAGS", "-D warnings"));
		step::mirror(&api.out.join("web"), &dir.join("web"));
		step::copy(&manifest_dir().join("tests").join("api.ts"), &dir.join("api.ts"));
		let scripts = generated(
			"typescript",
			"transcript.ts",
			&transcript::typescript::emit(transcript(), transcript::typescript::Binding::Web),
		);
		step::copy(&scripts.join("transcript.ts"), &dir.join("transcript.ts"));
		step::copy(
			&manifest_dir().join("tests").join("protocol.ts"),
			&dir.join("protocol.ts"),
		);
		step::write(&dir.join("tsconfig.json"), TSCONFIG);
		run(Command::new("tsc").arg("-p").arg(&dir));
		// Deno's own types too, which JSR checks a package with as it publishes.
		run(Command::new("deno")
			.args(["check", "api.ts", "protocol.ts"])
			.current_dir(&dir)
			.env("DENO_DIR", scratch().join("deno"))
			.env("DENO_NO_UPDATE_CHECK", "1")
			.env("NO_COLOR", "1"));
		Web {
			dir,
			wasm: target.join(WEB).join("debug").join("api.wasm"),
			transcript: wasm_transcript(),
		}
	})
}

/// What a run of `tests/api.ts` did: whether it finished, and what it printed.
struct Ran {
	ok: bool,
	stdout: String,
	stderr: String,
}

impl From<Output> for Ran {
	fn from(output: Output) -> Self {
		Self {
			ok: output.status.success(),
			stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
			stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
		}
	}
}

/// The cases of `tests/api.ts` that must pass, after its transcript: `owned` and `bytes`
/// check what the garbage collector frees, so only a runtime exposing it runs them.
const WEB_CASES: [&str; 7] = ["futures", "owned", "closed", "bytes", "bounds", "grown", "checks"];

/// The cases of `tests/api.ts` that break a contract, each failing with its message.
const WEB_BROKEN: [(&str, &str); 9] = [
	("moved", "api_Counter_get: a handle's value was moved out"),
	("boxed moved", "api_Node_sum: a handle's value was moved out"),
	("lent kept", "Counter is closed"),
	(
		"lent moved",
		"a closure: a call into JavaScript threw Error: api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
	),
	(
		"callback threw",
		"api::Log::log: a call into JavaScript threw Error: a log failed",
	),
	(
		"closure threw",
		"a closure: a call into JavaScript threw Error: a poll failed",
	),
	("later poll trapped", "api_tail: a reader read more than it was asked"),
	("reader threw", "api_tail: a reader read more than it was asked"),
	("outlived", OUTLIVED),
];

/// Checks what `tests/api.ts` did on `runtime`, `ran` naming each case's run: it printed
/// what native Rust prints on the binding's target, passed each of `cases`, and failed each
/// broken one with its message.
fn web_checks(runtime: &str, cases: &[&str], ran: &mut dyn FnMut(Option<&str>) -> Ran) {
	let transcript = ran(None);
	assert!(transcript.ok, "{runtime}'s transcript failed:\n{}", transcript.stderr);
	assert_eq!(
		transcript.stdout,
		web().transcript,
		"{runtime}'s transcript differs from native Rust's on {WEB}"
	);
	for &case in cases {
		let run = ran(Some(case));
		assert!(run.ok, "{runtime}'s {case} failed:\n{}{}", run.stdout, run.stderr);
	}
	for (case, message) in WEB_BROKEN {
		let run = ran(Some(case));
		assert!(!run.ok, "{runtime}'s {case} did not fail");
		assert!(
			run.stderr.contains(message),
			"{runtime}'s {case} printed {}",
			run.stderr
		);
		// A case checking what follows the failure says it got that far.
		if case == "later poll trapped" {
			assert!(
				run.stdout.contains("later poll checked"),
				"{runtime}'s {case} printed {}",
				run.stdout
			);
		}
	}
}

/// `tests/api.ts`'s script on `runtime`, Node, Deno, or Bun, with `case`, from beside the
/// binding.
fn web_script(runtime: &str, case: Option<&str>) -> Command {
	let web = web();
	let mut cmd = Command::new(runtime);
	if runtime == "deno" {
		// Its cache in the test's own directory, and its errors without color, as matched.
		cmd.args(["run", "--allow-read", "--allow-env"])
			.env("DENO_DIR", scratch().join("deno"))
			.env("DENO_NO_UPDATE_CHECK", "1")
			.env("NO_COLOR", "1");
	}
	cmd.arg("api.ts")
		.args(case)
		.current_dir(&web.dir)
		.env("RUST2_WASM", &web.wasm);
	cmd
}

/// Random sequences of the ownership protocol's steps through the web binding on Node and
/// on Deno, by `tests/protocol.ts`, each matching what the model says each step returns.
#[test]
fn api_web_protocol() {
	for runtime in ["node", "deno"] {
		web_protocol(runtime, "scripts", 0, PROTOCOL_SEEDS, PROTOCOL_STEPS);
	}
}

/// Longer protocol scripts from other seeds through the web binding on Node, as the
/// nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_web_protocol_nightly() {
	web_protocol("node", "nightly", PROTOCOL_SEEDS, 5000, 400);
}

/// `count` scripts from `first` of `steps` each, through `tests/protocol.ts` in one process
/// of `runtime`, Node or Deno.
fn web_protocol(runtime: &str, name: &str, first: u64, count: u64, steps: usize) {
	let web = web();
	let scripts = protocol::scripts(first, count, steps, protocol::Profile::TS);
	let driver = || {
		let mut cmd = Command::new(runtime);
		match runtime {
			"deno" => cmd
				.args(["run", "--allow-read", "--allow-write", "--allow-env"])
				.env("DENO_DIR", scratch().join("deno"))
				.env("DENO_NO_UPDATE_CHECK", "1")
				.env("NO_COLOR", "1"),
			_ => cmd.arg("--no-warnings"),
		};
		cmd.arg("protocol.ts")
			.current_dir(&web.dir)
			.env("RUST2_WASM", &web.wasm);
		cmd
	};
	let dir = scratch().join("api").join("web-protocol").join(runtime).join(name);
	protocol::check_all(&dir, &scripts, driver);
}

/// The generated TypeScript runs `tests/api.ts`'s script on Node, Deno, and Bun, over the
/// module the crate's `web` feature builds for wasm32-wasip1: it prints what native Rust
/// prints on that target, then checks its futures, what disposing and the garbage collector
/// free, what a closed object refuses, how bytes cross, and what a call lent bytes reads once
/// memory grows, and fails loud on each contract it breaks; a binding of another contract
/// refuses the module.
#[test]
fn api_web() {
	for runtime in ["node", "deno", "bun"] {
		web_checks(runtime, &WEB_CASES, &mut |case| {
			output(&mut web_script(runtime, case)).into()
		});
	}

	// The same binding, with another contract than the module's in place of its own,
	// refuses it as it loads.
	let api = api();
	let web = web();
	let contract = web.dir.join("contract");
	step::mirror(&api.out.join("web"), &contract.join("web"));
	let index = contract.join("web").join("index.ts");
	let source = std::fs::read_to_string(api.out.join("web").join("index.ts")).unwrap();
	let at = source.find("contract: 0x").expect("the binding checks its contract") + "contract: 0x".len();
	let mut mismatched = source.clone();
	let digit = if &source[at..at + 1] == "0" { "1" } else { "0" };
	mismatched.replace_range(at..at + 1, digit);
	step::write(&index, mismatched);
	step::write(
		&contract.join("main.ts"),
		"import { readFileSync } from \"node:fs\";\nimport * as api from \"./web/index.ts\";\n\nawait api.load(readFileSync(process.env.RUST2_WASM!));\n",
	);
	let output = Command::new("node")
		.arg("main.ts")
		.current_dir(&contract)
		.env("RUST2_WASM", &web.wasm)
		.output()
		.unwrap();
	assert!(!output.status.success(), "a mismatched contract loaded");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("index.ts and the module are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// Where the engine lacks `Symbol.dispose`, as stable Safari does, the web runtime defines it
/// as the registry's symbol before its classes key a method on it, which `dispose.ts` checks.
#[test]
fn web_without_dispose() {
	run(Command::new("node")
		.arg(manifest_dir().join("tests").join("dispose.ts"))
		.arg(manifest_dir().join("runtime").join("rt.ts")));
}

/// The web binding as a registry serves it: packed by npm with the module beside it,
/// installed from that tarball into a project of its own, and `tests/api.ts` importing it by
/// the package's name type-checks against its declarations alone, with no `skipLibCheck`.
/// The script runs the package's JavaScript, which `load` finds the module beside by
/// default: every check on Node, which strips no type under `node_modules`, and the
/// transcript on Deno and Bun.
#[test]
fn api_web_package() {
	let api = api();
	let web = web();
	let dir = scratch().join("api").join("web-package");
	let package = dir.join("package");
	step::mirror(&api.out.join("web"), &package);
	step::copy(&web.wasm, &package.join("api.wasm"));
	let manifest: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(package.join("package.json")).unwrap()).unwrap();
	let name = manifest["name"].as_str().expect("package.json names the package");
	let npm = |dir: &Path| {
		let mut cmd = Command::new("npm");
		cmd.current_dir(dir)
			.env("npm_config_cache", scratch().join("npm"))
			.env("npm_config_update_notifier", "false")
			.env("npm_config_fund", "false")
			.env("npm_config_audit", "false");
		cmd
	};
	let packed = run(npm(&package).args(["pack", "--silent", "--pack-destination"]).arg(&dir));
	let tarball = dir.join(String::from_utf8_lossy(&packed.stdout).trim());
	let consumer = dir.join("consumer");
	if consumer.exists() {
		std::fs::remove_dir_all(&consumer).unwrap();
	}
	let dependency = serde_json::json!({ name: format!("file:{}", tarball.display()) });
	step::write(
		&consumer.join("package.json"),
		serde_json::json!({ "type": "module", "private": true, "dependencies": dependency }).to_string(),
	);
	run(npm(&consumer).args(["install", "--offline", "--ignore-scripts"]));
	// What npm installed is the package's JavaScript and declarations, not its TypeScript.
	let installed = consumer.join("node_modules").join(name);
	assert!(installed.join("index.js").exists() && installed.join("index.d.ts").exists());
	assert!(installed.join("api.wasm").exists());
	assert!(
		!installed.join("index.ts").exists(),
		"the npm package ships its TypeScript"
	);
	// Each import of the binding is of the package instead; the script's own two files,
	// `api.ts` and the transcript's, still import each other by path.
	let imports = |script: String| -> String {
		let script = script
			.replace("\"./web/index.ts\"", &format!("\"{name}\""))
			.replace("\"./web/", &format!("\"{name}/"));
		let package = format!("from \"{name}/");
		let script: String = (script.lines())
			.map(|line| match line.contains(&package) {
				true => format!("{}\n", line.replace(".ts\";", "\";")),
				false => format!("{line}\n"),
			})
			.collect();
		assert!(
			!script.contains("./web/"),
			"the script imports the binding by a path the rewrite missed"
		);
		script
	};
	let transcript = generated(
		"typescript",
		"transcript.ts",
		&transcript::typescript::emit(transcript(), transcript::typescript::Binding::Web),
	);
	for file in [
		manifest_dir().join("tests").join("api.ts"),
		transcript.join("transcript.ts"),
	] {
		let script = std::fs::read_to_string(&file).unwrap();
		step::write(&consumer.join(file.file_name().unwrap()), imports(script));
	}
	step::write(&consumer.join("tsconfig.json"), TSCONFIG_PACKAGE);
	run(Command::new("tsc").arg("-p").arg(&consumer));
	let runtime = |runtime: &str, case: Option<&str>| {
		let mut cmd = Command::new(runtime);
		if runtime == "deno" {
			cmd.args(["run", "--allow-read", "--allow-env"])
				.env("DENO_DIR", scratch().join("deno"))
				.env("DENO_NO_UPDATE_CHECK", "1")
				.env("NO_COLOR", "1");
		}
		cmd.arg("api.ts").args(case).current_dir(&consumer);
		Ran::from(output(&mut cmd))
	};
	web_checks("node's package", &WEB_CASES, &mut |case| runtime("node", case));
	for other in ["deno", "bun"] {
		let ran = runtime(other, None);
		assert!(ran.ok, "{other}'s transcript of the package failed:\n{}", ran.stderr);
		assert_eq!(
			ran.stdout, web.transcript,
			"{other}'s transcript of the package differs from native Rust's"
		);
	}
}

/// `api_web`'s checks in headless Chromium, Firefox, and WebKit, which only the bench shell
/// carries, WebKit on Linux alone, where nixpkgs builds it: `tests/browser.ts` bundles the
/// script and runs each case in a frame of its own.
/// Firefox and WebKit expose no garbage collector to a page, so they run neither `owned` nor
/// `bytes`. The script's `using` runs in WebKit as the engine has it, and lowered as `tsc` and
/// esbuild lower it, each of which keys on `Symbol.dispose`. Playwright's WebKit has the symbol,
/// which stable Safari lacks: `web_without_dispose` covers its absence.
#[test]
#[ignore = "the nightly's, in the bench shell: just ffi browsers"]
fn api_browsers() {
	let web = web();
	for (browser, lowering) in [
		("chromium", "bun"),
		("firefox", "bun"),
		("webkit", "native"),
		("webkit", "esbuild"),
		("webkit", "tsc"),
	]
	.into_iter()
	.filter(|(browser, _)| *browser != "webkit" || cfg!(target_os = "linux"))
	{
		let cases: Vec<&str> = match browser {
			"chromium" => WEB_CASES.to_vec(),
			_ => WEB_CASES
				.into_iter()
				.filter(|c| !matches!(*c, "owned" | "bytes"))
				.collect(),
		};
		let all = std::iter::once("-")
			.chain(cases.iter().copied())
			.chain(WEB_BROKEN.iter().map(|(case, _)| *case));
		let output = run(Command::new("bun")
			.arg(manifest_dir().join("tests").join("browser.ts"))
			.args([browser, lowering])
			.arg(&web.dir)
			.arg(&web.wasm)
			.arg(scratch().join("api").join(format!("{browser}-{lowering}")))
			.args(all));
		let mut runs: std::collections::HashMap<String, Ran> = String::from_utf8_lossy(&output.stdout)
			.lines()
			.map(|line| {
				let v: serde_json::Value = serde_json::from_str(line).expect("browser.ts prints a case's JSON");
				let text = |key: &str| v[key].as_str().expect("a case's output is text").to_string();
				let ran = Ran {
					ok: v["ok"].as_bool().expect("a case says whether it passed"),
					stdout: text("stdout"),
					stderr: text("stderr"),
				};
				(text("case"), ran)
			})
			.collect();
		web_checks(&format!("{browser} ({lowering})"), &cases, &mut |case| {
			let case = case.unwrap_or("-");
			runs.remove(case).unwrap_or_else(|| panic!("{browser} ran no {case}"))
		});
	}
}

/// The api fixture's React Native binding on Hermes off a device: `tests/rn_host.cpp`, a
/// host installing the JSI module as React Native's TurboModule does, runs `tests/api.ts`'s
/// script, bundled from the generated TypeScript, and each case a runtime without
/// WebAssembly runs. `RUST2_HERMES` names a Hermes checkout built into its `build` at the
/// tag React Native's release pins, whose library finds ICU on the loader's path.
#[test]
#[ignore = "needs Hermes: RUST2_HERMES=<checkout> just ffi react-native-host"]
fn api_react_native_host() {
	let hermes = PathBuf::from(std::env::var_os("RUST2_HERMES").expect("RUST2_HERMES names a built Hermes checkout"));
	let build = hermes.join("build");
	let api = api();
	let dir = scratch().join("api").join("react-native");
	step::mirror(&api.out.join("react-native"), &dir.join("react-native"));
	step::write(
		&dir.join("transcript.ts"),
		transcript::typescript::emit(transcript(), transcript::typescript::Binding::ReactNative),
	);
	step::write(&dir.join("api.ts"), react_native_check());
	step::copy(&manifest_dir().join("tests").join("react-native.ts"), &dir.join("entry.ts"));
	// What `load` reaches of React Native only where no host installed the module.
	step::write(
		&dir.join("react-native-stub.ts"),
		"export const TurboModuleRegistry = {\n\tgetEnforcing(name: string): never {\n\t\tthrow new Error(`no TurboModule ${name} off React Native`);\n\t},\n};\nexport type TurboModule = object;\n",
	);
	// Hermes takes a script, not modules, and lacks async generators. Strict, as Metro's
	// modules are.
	run(Command::new("esbuild")
		.args([
			"entry.ts",
			"--bundle",
			"--format=iife",
			"--banner:js=\"use strict\";",
			"--target=es2020",
			"--supported:async-generator=false",
			"--alias:react-native=./react-native-stub.ts",
			"--log-level=error",
			"--outfile=bundle.js",
		])
		.current_dir(&dir));
	let host = dir.join("rn-host");
	let cpp = dir.join("react-native").join("cpp");
	let mut cmd = Command::new(std::env::var_os("CXX").unwrap_or_else(|| "c++".into()));
	cmd.args(["-std=c++17", "-g", "-Wall", "-Wextra", "-Werror", "-DRUST2_PREFIX_NS=rust2_api", "-DRUST2_GLOBAL=\"__rust2_api\""])
		.arg("-I")
		.arg(api.out.join("include"))
		.arg("-I")
		.arg(&cpp);
	for include in [hermes.join("API"), hermes.join("API/jsi"), hermes.join("public"), hermes.join("include")] {
		cmd.arg("-isystem").arg(include);
	}
	cmd.arg("-isystem")
		.arg(build.join("lib/config"))
		.arg(cpp.join("api_jsi.cpp"))
		.arg(manifest_dir().join("tests").join("rn_host.cpp"))
		.arg("-o")
		.arg(&host)
		.arg(build.join("lib").join(shared("hermesvm")))
		.arg(build.join("jsi").join("libjsi.a"))
		.arg(api.staticlib())
		.args(&api.native)
		.arg(format!("-Wl,-rpath,{}", build.join("lib").display()));
	run(&mut cmd);
	let script = |case: Option<&str>| -> Ran {
		output(Command::new(&host).arg("bundle.js").args(case).current_dir(&dir)).into()
	};
	let transcript = script(None);
	assert!(transcript.ok, "Hermes's transcript failed:\n{}", transcript.stderr);
	assert_eq!(
		transcript.stdout, api.transcript,
		"Hermes's transcript differs from native Rust's"
	);
	for case in REACT_NATIVE_CASES {
		let run = script(Some(case));
		assert!(run.ok, "Hermes's {case} failed:\n{}{}", run.stdout, run.stderr);
	}
	// Each fails with the web's message, but a panic, which aborts rather than trapping, so
	// `later poll trapped` gets no further.
	for (case, message) in WEB_BROKEN {
		let run = script(Some(case));
		assert!(!run.ok, "Hermes's {case} did not fail");
		assert!(run.stderr.contains(message), "Hermes's {case} printed {}", run.stderr);
	}
}

/// The cases of `tests/api.ts` React Native runs, after its transcript: `grown`'s memory and
/// a load's are WebAssembly's.
const REACT_NATIVE_CASES: [&str; 6] = ["futures", "owned", "closed", "bytes", "bounds", "checks"];

/// `tests/api.ts` importing the React Native binding, its cases exported for
/// `tests/react-native.ts`, which loads the binding as its `main` does the web's.
fn react_native_check() -> String {
	let check = std::fs::read_to_string(manifest_dir().join("tests").join("api.ts")).unwrap();
	let check = check.replace("\"./web/", "\"./react-native/");
	let glue = "import * as g from \"./react-native/glue.ts\";\n";
	assert_eq!(check.matches(glue).count(), 1, "api.ts imports the web's glue once");
	let check = check.replace(glue, "");
	let main = "\nasync function main(): Promise<void> {";
	let at = check.find(main).expect("api.ts has its main");
	check[..=at].to_string()
}

/// What `tsc` checks a web binding and its caller with: strict, with no name unused, on the
/// ES2024 a binding needs, as Node runs it, with no ambient types but the DOM's and Node's.
const TSCONFIG: &str = r#"{
	"compilerOptions": {
		"strict": true,
		"noEmit": true,
		"pretty": false,
		"noImplicitOverride": true,
		"noUnusedLocals": true,
		"noUnusedParameters": true,
		"target": "es2024",
		"lib": ["es2024", "esnext.disposable", "dom"],
		"module": "esnext",
		"moduleResolution": "bundler",
		"allowImportingTsExtensions": true,
		"verbatimModuleSyntax": true,
		"erasableSyntaxOnly": true,
		"skipLibCheck": true,
		"types": []
	},
	"include": ["api.ts", "transcript.ts", "protocol.ts", "web/*.ts"],
	"exclude": ["web/*.d.ts"]
}
"#;

/// What `tsc` checks a caller of the web binding's npm package with: as `TSCONFIG`, but
/// resolving the package as Node does, through its `exports` and declarations alone.
const TSCONFIG_PACKAGE: &str = r#"{
	"compilerOptions": {
		"strict": true,
		"noEmit": true,
		"pretty": false,
		"noUnusedLocals": true,
		"noUnusedParameters": true,
		"target": "es2024",
		"lib": ["es2024", "esnext.disposable", "dom"],
		"module": "nodenext",
		"moduleResolution": "nodenext",
		"allowImportingTsExtensions": true,
		"verbatimModuleSyntax": true,
		"erasableSyntaxOnly": true,
		"types": []
	},
	"include": ["api.ts", "transcript.ts"]
}
"#;

/// The generated Dart of a small API, which uses little of the runtime and of what the
/// generator writes per type, analyzes with no issue as well: nothing it leaves unused stays.
#[test]
fn dart_small_api_lints() {
	let dir = scratch().join("edge").join("dart");
	let out = scratch().join("edge").join("out");
	let output = export("edge", &out, &[WASM, host()], &[]);
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let lint = dir.join("lint");
	step::write(
		&lint.join("analysis_options.yaml"),
		std::fs::read_to_string(manifest_dir().join("tests/dart_lints.yaml")).unwrap(),
	);
	step::write(
		&lint.join("edge.dart"),
		std::fs::read_to_string(out.join("dart/lib/edge.dart")).unwrap(),
	);
	let step = Step {
		stamp: &dir.join("analyze.stamp"),
		reads: &[&lint],
		writes: &[],
	};
	step.run(Command::new("dart").args(["analyze", "--fatal-infos"]).arg(&lint));
}

/// The generated Dart, analyzed with no issue, runs `tests/api.dart`'s script, which prints
/// the native transcript, then its futures, what closing and the garbage collector free,
/// and each contract it fails loud on, and a Dart file of another contract fails to load.
#[test]
fn api_dart() {
	let api = api();
	let dir = scratch().join("api").join("dart");
	let generated = api.out.join("dart/lib");
	// The analyzer reads its options from the analyzed directory, so analyze a copy with them.
	let lint = dir.join("lint");
	step::write(
		&lint.join("analysis_options.yaml"),
		std::fs::read_to_string(manifest_dir().join("tests/dart_lints.yaml")).unwrap(),
	);
	step::write(
		&lint.join("api.dart"),
		std::fs::read_to_string(generated.join("api.dart")).unwrap(),
	);
	let step = Step {
		stamp: &dir.join("analyze.stamp"),
		reads: &[&lint],
		writes: &[],
	};
	step.run(Command::new("dart").args(["analyze", "--fatal-infos"]).arg(&lint));
	// What looks like a leaf but reaches user code, loops, or reads a static is no leaf
	// call. The script catches a hand-written `PartialEq`, method, or `Deref`; a getter's
	// drain it cannot, as a home drains each drop queued for it at once, apart from any
	// call. Nor is one walking a length the caller picks, which would stall the isolate
	// group's collector until it returns.
	let code = std::fs::read_to_string(generated.join("api.dart")).unwrap();
	let leaf = |symbol: &str| code.contains(&format!("(symbol: '{symbol}', isLeaf: true)"));
	assert!(
		[
			"api_Plain_eq",
			"api_Plain_hash",
			"api_Plain_n_get",
			"api_Plain_n_set",
			"api_Plain_count",
			"api_Plain_recount",
			"api_Wrapped_n_get",
			"api_Wrapped_n_set",
			"api_Wrapped_count",
			"api_Wrapped_recount",
			"api_contract"
		]
		.into_iter()
		.all(leaf),
		"a leaf is a leaf call"
	);
	assert!(
		[
			"api_Hooked_eq",
			"api_Forged_eq",
			"api_Pinned_value_get",
			"api_Part_value_get",
			"api_Part_eq",
			"api_Part_hash",
			"api_Labeled_eq",
			"api_Labeled_hash",
			"api_Plain_steps",
			"api_Buffer_blank",
			"api_Plain_locals",
			"api_Hooked_count",
			"api_Guarded_n_get",
			"api_Guarded_count"
		]
		.into_iter()
		.all(|symbol| !leaf(symbol)),
		"a call reaching user code or walking a caller's length is no leaf call"
	);
	let lib = api.lib.join(PYTHON_LIB);
	// The test and its script, as the release builder takes them, and an entry beside them
	// that imports the test, as the script does, so both name the one library.
	let checks = dir.join("checks");
	step::copy(
		&manifest_dir().join("tests").join("api.dart"),
		&checks.join("check.dart"),
	);
	step::write(&checks.join("transcript.dart"), transcript::dart::emit(transcript()));
	let main = checks.join("main.dart");
	step::write(
		&main,
		"import 'check.dart' as test;\n\nFuture<void> main(List<String> args) => test.main(args);\n",
	);
	let exe = dart_compile("api", &dir, &generated, &lib, &main, &[("checks", &checks)]);
	let dart = run(&mut dart_run(&exe, &[]));
	assert_eq!(
		String::from_utf8_lossy(&dart.stdout),
		api.transcript,
		"the Dart transcript differs from the native one"
	);
	// What only wasm32 has, a program of its own, which the test leaves out so that it
	// compiles against the release builder's export, which has no wasm32.
	let targets = dart_compile(
		"api",
		&dir,
		&generated,
		&lib,
		&manifest_dir().join("tests").join("targets.dart"),
		&[],
	);
	run(&mut dart_run(&targets, &[]));
	for case in ["checks", "futures", "owned", "closed", "bytes", "bounds", "leaf"] {
		run(&mut dart_run(&exe, &[case]));
	}
	// The protocol scripts, through `tests/protocol.dart`.
	let protocol = protocol_dart(api, &dir);
	protocol::check_all(
		&scratch().join("api").join("protocol").join("dart"),
		&protocol::scripts(0, DART_PROTOCOL_SEEDS, PROTOCOL_STEPS, protocol::Profile::DART),
		|| dart_run(&protocol, &[]),
	);
	for (case, message) in [
		("moved", "Bad state: api_Counter_get: a handle's value was moved out"),
		("boxed moved", "Bad state: api_Node_sum: a handle's value was moved out"),
		("lent kept", "Bad state: Counter is closed"),
		(
			"lent moved",
			"Bad state: api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
		),
		("callback threw", "api::Log::log: a call into Dart threw"),
		(
			"closure threw",
			"a closure: a call into Dart threw Bad state: a poll failed",
		),
		(
			"reader threw",
			"a reader's seek: a call into Dart threw Bad state: a seek failed",
		),
		(
			"outlived",
			"Bad state: api_Meter_count: a handle lent from another outlived it",
		),
	] {
		let output = dart_run(&exe, &[case]).output().unwrap();
		failed(&output, &format!("Dart {case}"));
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Dart {case} printed {stderr}");
	}

	// The same Dart, with another contract than the library's in place of its own, refuses
	// it at the first call.
	let source = std::fs::read_to_string(generated.join("api.dart")).unwrap();
	let at = source.find("contract() == 0x").expect("the Dart checks its contract") + "contract() == 0x".len();
	let mut mismatched = source.clone();
	let digit = if &source[at..at + 1] == "0" { "1" } else { "0" };
	mismatched.replace_range(at..at + 1, digit);
	// The package apart from the program's directory, which its build writes into.
	let contract = dir.join("contract");
	let package = contract.join("package");
	step::write(&package.join("api.dart"), mismatched);
	let main = contract.join("main.dart");
	step::write(
		&main,
		"import 'package:api/api.dart' as api;\n\nvoid main() => api.dropped();\n",
	);
	let exe = dart_compile("api", &contract, &package, &lib, &main, &[]);
	let output = dart_run(&exe, &[]).output().unwrap();
	failed(&output, "a mismatched contract");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.dart and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// The gate's Dart protocol scripts, fewer than C's, as each step costs Dart more.
const DART_PROTOCOL_SEEDS: u64 = 100;

/// `tests/protocol.dart`, compiled in `dir` against the api fixture's Dart.
fn protocol_dart(api: &Api, dir: &Path) -> PathBuf {
	dart_compile(
		"api",
		dir,
		&api.out.join("dart/lib"),
		&api.lib.join(PYTHON_LIB),
		&manifest_dir().join("tests").join("protocol.dart"),
		&[],
	)
}

/// Longer Dart protocol scripts from other seeds, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_dart_protocol_nightly() {
	let api = api();
	let protocol = protocol_dart(api, &scratch().join("api").join("dart"));
	protocol::check_all(
		&scratch().join("api").join("protocol").join("dart-nightly"),
		&protocol::scripts(PROTOCOL_SEEDS, 2000, 400, protocol::Profile::DART),
		|| dart_run(&protocol, &[]),
	);
}

/// `main` compiled into a kernel snapshot in `dir`, importing the generated Dart in
/// `package` as `package:<name>/<name>.dart`, whose native asset is the shared library
/// `lib`: as a package's build hook would bundle it, which `dart compile` has no flag for.
/// Each of `others` is a package of its own, by its name and its directory.
fn dart_compile(name: &str, dir: &Path, package: &Path, lib: &Path, main: &Path, others: &[(&str, &Path)]) -> PathBuf {
	let (mut cmd, kernel) = dart_kernel(name, dir, package, lib, main, others);
	// The snapshot names the library by path, so a change to it rebuilds nothing.
	let mut reads: Vec<&Path> = vec![main, package, &kernel.config, &kernel.assets, &kernel.version];
	reads.extend(others.iter().map(|(_, root)| *root));
	let step = Step {
		stamp: &step::stamp(&kernel.dill),
		reads: &reads,
		writes: &[&kernel.dill],
	};
	step.run(&mut cmd);
	kernel.dill
}

/// What a Dart kernel compile reads and writes besides its program and package.
struct Kernel {
	config: PathBuf,
	assets: PathBuf,
	/// The SDK's version file, which keys a rebuild on the SDK.
	version: PathBuf,
	dill: PathBuf,
}

/// The compile of `main` to a kernel snapshot in `dir`, against the package `name` at
/// `package` and its native library `lib`, and each of `others`, with the package config
/// and native assets it reads written.
fn dart_kernel(
	name: &str,
	dir: &Path,
	package: &Path,
	lib: &Path,
	main: &Path,
	others: &[(&str, &Path)],
) -> (Command, Kernel) {
	let sdk = std::fs::canonicalize(which("dart"))
		.unwrap()
		.parent()
		.and_then(Path::parent)
		.expect("dart is in the SDK's bin")
		.to_path_buf();
	// Each program's own, as two compiled in one directory at once would race on one.
	let stem = main.file_stem().unwrap().to_string_lossy();
	let config = dir.join(format!("{stem}.package_config.json"));
	let packages: Vec<String> = std::iter::once((name, package))
		.chain(others.iter().copied())
		.map(|(name, root)| {
			let root = url(root);
			format!("{{\"name\": \"{name}\", \"rootUri\": \"{root}/\", \"packageUri\": \"./\"}}")
		})
		.collect();
	step::write(
		&config,
		format!("{{\"configVersion\": 2, \"packages\": [{}]}}\n", packages.join(", ")),
	);
	let os = match std::env::consts::OS {
		"macos" => "macos",
		"windows" => "windows",
		_ => "linux",
	};
	let arch = match std::env::consts::ARCH {
		"aarch64" => "arm64",
		_ => "x64",
	};
	let assets = dir.join(format!("{stem}.native_assets.yaml"));
	step::write(
		&assets,
		format!(
			"format-version: [1, 0, 0]\nnative-assets:\n  {os}_{arch}:\n    \"package:{name}/{name}.dart\": [absolute, {:?}]\n",
			lib.display().to_string()
		),
	);
	let dill = dir.join(format!("{stem}.dill"));
	let mut cmd = Command::new(sdk.join("bin").join(format!("dartaotruntime{EXE_SUFFIX}")));
	cmd.arg(sdk.join("bin/snapshots/gen_kernel_aot.dart.snapshot"))
		.arg("--platform")
		.arg(sdk.join("lib/_internal/vm_platform_strong.dill"))
		.arg("--packages")
		.arg(&config)
		.arg("--native-assets")
		.arg(&assets)
		.arg("-o")
		.arg(&dill)
		.arg(main);
	let kernel = Kernel {
		config,
		assets,
		version: sdk.join("version"),
		dill,
	};
	(cmd, kernel)
}

/// The Dart VM running a kernel snapshot, with asserts on.
fn dart_run(dill: &Path, args: &[&str]) -> Command {
	let mut cmd = Command::new("dart");
	cmd.arg("--enable-asserts").arg(dill).args(args);
	cmd
}

/// A program on `PATH`.
fn which(name: &str) -> PathBuf {
	let path = std::env::var_os("PATH").expect("PATH is set");
	std::env::split_paths(&path)
		.map(|dir| dir.join(format!("{name}{EXE_SUFFIX}")))
		.find(|p| p.is_file())
		.unwrap_or_else(|| panic!("{name} is not on PATH"))
}

/// A directory's `file:` URL.
fn url(dir: &Path) -> String {
	let path = dir.display().to_string().replace('\\', "/");
	match path.starts_with('/') {
		true => format!("file://{path}"),
		false => format!("file:///{path}"),
	}
}

/// The static libraries of swift-collections' modules the generated Swift imports, which
/// `just ffi swift-collections` builds beside their modules, where [`swiftc`] finds them.
/// Built once, since two tests running the recipe at once race on its checkout and build.
fn swift_collections() -> Vec<PathBuf> {
	static BUILT: OnceLock<()> = OnceLock::new();
	BUILT.get_or_init(|| {
		run(Command::new("just")
			.arg("--justfile")
			.arg(manifest_dir().join("justfile"))
			.arg("swift-collections"));
	});
	let build = collections_dir();
	["BasicContainers", "InternalCollectionsUtilities", "SpanPreview"]
		.iter()
		.map(|module| build.join(format!("lib{module}.a")))
		.collect()
}

fn collections_dir() -> PathBuf {
	manifest_dir().join("target").join("swift-collections").join("build")
}

/// swiftc, with warnings as errors, finding the C header's module in `module`,
/// swift-collections' in [`collections_dir`], and `tests/nodump`'s, at this macOS deployment
/// target on macOS: 13 is the oldest the generated `Swift.Duration` allows.
fn swiftc(module: &Path, macos: u8) -> Command {
	let mut cmd = Command::new("swiftc");
	if cfg!(target_os = "macos") {
		let arch = match std::env::consts::ARCH {
			"aarch64" => "arm64",
			arch => arch,
		};
		cmd.arg("-target").arg(format!("{arch}-apple-macos{macos}"));
	}
	cmd.args(["-warnings-as-errors", "-I"])
		.arg(module)
		.arg("-I")
		.arg(collections_dir())
		.arg("-I")
		.arg(manifest_dir().join("tests").join("nodump"));
	cmd
}

/// The generated Go, vetted and built through cgo against the static library, runs
/// `tests/api.go`'s script, its futures, and what closing and the GC free, then fails loud
/// on each broken contract and on a library of another contract.
#[test]
fn api_go() {
	let api = api();
	let dir = scratch().join("api").join("go");
	let module = dir.join("module");
	go_layout(
		api,
		&module,
		&std::fs::read_to_string(manifest_dir().join("tests").join("api.go")).unwrap(),
	);
	// The transcript's script, generated beside it in its package.
	step::write(&module.join("transcript.go"), transcript::go::emit(transcript()));
	// What only wasm32 has, a program of its own, which the test leaves out so that it
	// compiles against the release builder's export, which has no wasm32.
	step::copy(
		&manifest_dir().join("tests").join("targets.go"),
		&module.join("targets").join("main.go"),
	);
	let export = module.join("export");
	for (name, root) in [("vet-export", &export), ("vet", &module)] {
		let step = Step {
			stamp: &dir.join(format!("{name}.stamp")),
			reads: &[&module],
			writes: &[],
		};
		step.run(&mut go(root, &["vet", "./..."]));
	}
	let exe = dir.join(format!("api-test{EXE_SUFFIX}"));
	go_build(api, &module, &exe);
	let output = run(&mut Command::new(&exe));
	assert_eq!(
		String::from_utf8_lossy(&output.stdout),
		api.transcript,
		"the Go transcript differs from the native one"
	);
	run(Command::new(&exe).arg("futures"));
	run(Command::new(&exe).arg("owned"));
	run(Command::new(&exe).arg("closed"));
	run(Command::new(&exe).arg("bytes"));
	run(Command::new(&exe).arg("closures"));
	run(Command::new(&exe).arg("checks"));
	for (case, message) in [
		(
			"lent moved",
			"api_Counter_into_label: a handle a call lent is borrowed exclusively or moved out",
		),
		("nil reader", "a nil io.ReadSeeker"),
		("returned zero", "a zero Moment, which only Rust makes"),
		("outlived", OUTLIVED),
		("callback panicked", "a log failed"),
	] {
		let output = Command::new(&exe).arg(case).output().unwrap();
		failed(&output, &format!("Go {case}"));
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Go {case} printed {stderr}");
	}
	let targets = dir.join(format!("targets-test{EXE_SUFFIX}"));
	let step = Step {
		stamp: &step::stamp(&targets),
		reads: &[&module, &api.staticlib()],
		writes: &[&targets],
	};
	step.run(go(&module, &["build", "-o"]).arg(&targets).arg("./targets"));
	run(&mut Command::new(&targets));
	for (case, message) in [
		("target", "api_wasm_only: it is not on this library's target"),
		("variant", "api_Platform.Wasm: it is not on this library's target"),
		("plain variant", "api_Family.Wasm: it is not on this library's target"),
	] {
		let output = Command::new(&targets).arg(case).output().unwrap();
		failed(&output, &format!("Go {case}"));
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Go {case} printed {stderr}");
	}

	// The same packages, with another contract than the library's in place of their own,
	// refuse it when they load: in a module of their own, which leaves the test's as it was.
	let contract = dir.join("contract");
	go_layout(
		api,
		&contract,
		"package main\n\nimport \"api/api\"\n\nfunc main() {\n\tapi.Dropped()\n}\n",
	);
	let runtime = contract.join("export").join("rt").join("rt.go");
	let generated = std::fs::read_to_string(&runtime).unwrap();
	let at = generated
		.find("api_contract() != 0x")
		.expect("the Go checks its contract")
		+ "api_contract() != 0x".len();
	let mut mismatched = generated.clone();
	let digit = if &generated[at..at + 1] == "0" { "1" } else { "0" };
	mismatched.replace_range(at..at + 1, digit);
	step::write(&runtime, mismatched);
	let exe = dir.join(format!("contract-test{EXE_SUFFIX}"));
	go_build(api, &contract, &exe);
	let output = Command::new(&exe).output().unwrap();
	failed(&output, "a mismatched contract");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("the Go of api and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// Lay out `module` as a Go module whose main package is `main`, requiring the export's
/// module of a package per crate, with the static library its runtime links.
fn go_layout(api: &Api, module: &Path, main: &str) {
	step::mirror(&api.out.join("go"), &module.join("export"));
	go_library(&api.staticlib(), &module.join("export"));
	step::write(&module.join("main.go"), main);
	step::write(
		&module.join("go.mod"),
		"module apitest\n\ngo 1.24\n\nrequire api v0.0.0\n\nreplace api => ./export\n",
	);
}

/// Random sequences of the ownership protocol's steps through the Go, by
/// `tests/protocol.go`, each matching what the model says each step returns.
#[test]
fn api_go_protocol() {
	go_protocol(&protocol::scripts(0, 100, 100, protocol::Profile::GO), "scripts");
}

/// Longer Go protocol scripts from other seeds, as the nightly runs them.
#[test]
#[ignore = "slow: just ffi protocol"]
fn api_go_protocol_nightly() {
	go_protocol(&protocol::scripts(100, 5000, 400, protocol::Profile::GO), "nightly");
}

fn go_protocol(scripts: &[protocol::Script], dir: &str) {
	let api = api();
	let built = scratch().join("api").join("go-protocol");
	let module = built.join("module");
	go_layout(
		api,
		&module,
		&std::fs::read_to_string(manifest_dir().join("tests").join("protocol.go")).unwrap(),
	);
	let exe = built.join(format!("protocol{EXE_SUFFIX}"));
	go_build(api, &module, &exe);
	protocol::check(&built.join(dir), scripts, |_| Command::new(&exe));
}

/// Build the Go `module` into `exe` against the api fixture's static library.
fn go_build(api: &Api, module: &Path, exe: &Path) {
	let step = Step {
		stamp: &step::stamp(exe),
		reads: &[module, &api.staticlib()],
		writes: &[exe],
	};
	step.run(go(module, &["build", "-o"]).arg(exe));
}

/// Under a `go.mod`, the generated packages import each other by its module, as the go
/// command reads it from a block among comments, and the export writes no `go.mod` of its own.
#[test]
fn go_module() {
	let module = scratch().join("go-module");
	let _ = std::fs::remove_dir_all(&module);
	std::fs::create_dir_all(&module).unwrap();
	std::fs::write(
		module.join("go.mod"),
		"// module example.com/commented\nmodule ( // the app\n\t\"example.com/app\" // quoted\n)\n\ngo 1.24\n",
	)
	.unwrap();
	let out = module.join("sub").join("out");
	let output = export("api", &out, &[], &[]);
	assert!(
		output.status.success(),
		"exporting failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	assert!(
		!out.join("go").join("go.mod").exists(),
		"the export wrote a go.mod under one"
	);
	let api = std::fs::read_to_string(out.join("go").join("api").join("api.go")).unwrap();
	assert!(api.contains("\"example.com/app/sub/out/go/rt\""), "{api}");

	// A go.mod in go/ itself makes it a module, as a release's is, which the export keeps
	// as it is, above any other.
	let own = module.join("own");
	let go_mod = "module example.com/own/v2 // a release\n\ngo 1.24\n";
	std::fs::create_dir_all(own.join("go")).unwrap();
	std::fs::write(own.join("go").join("go.mod"), go_mod).unwrap();
	let output = export("api", &own, &[], &[]);
	assert!(
		output.status.success(),
		"exporting failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	assert_eq!(std::fs::read_to_string(own.join("go").join("go.mod")).unwrap(), go_mod);
	let api = std::fs::read_to_string(own.join("go").join("api").join("api.go")).unwrap();
	assert!(api.contains("\"example.com/own/v2/rt\""), "{api}");
	vet(&own.join("go"));
}

/// A crate named as the runtime, a Go keyword, or `main` is the Go package of that name and
/// a trailing `_`, which Go vets.
#[test]
fn go_reserved_packages() {
	for name in ["rt", "func", "main"] {
		let package = package(&format!("go_reserved_{name}"), &[]);
		let manifest = format!(
			"[package]\nname = \"go_reserved_{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"{name}\"\n\n[workspace]\n"
		);
		std::fs::write(package.join("Cargo.toml"), manifest).unwrap();
		let out = package.join("out");
		let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--manifest-path")
			.arg(package.join("Cargo.toml"))
			.arg("--out")
			.arg(&out)
			.env("CARGO_TARGET_DIR", package.join("build"))
			.output()
			.unwrap();
		assert!(
			output.status.success(),
			"exporting lib `{name}` failed:\n{}",
			String::from_utf8_lossy(&output.stderr)
		);
		let go = out.join("go");
		assert!(go.join(format!("{name}_")).join(format!("{name}_.go")).exists());
		vet(&go);
	}
}

/// A crate's package keeps the facade's prefix where stripping it leaves no identifier:
/// beside lib `pkg`, lib `pkg_` is `pkg_`, after the facade's `pkg`, and lib `pkg_2d` is
/// `pkg2d`, which Go vets.
#[test]
fn go_package_names() {
	let package = package("go_package_names", &[]);
	let dep = |lib: &str| {
		(
			format!(
				"[package]\nname = \"go-package-names-{}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"{lib}\"\n",
				lib.replace('_', "-")
			),
			"pub fn one() -> u32 {\n\t1\n}\n",
		)
	};
	let (under, under_lib) = dep("pkg_");
	let (digit, digit_lib) = dep("pkg_2d");
	write_all(
		&package,
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"go_package_names\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"pkg\"\n\n\
				[dependencies]\npkg_ = { package = \"go-package-names-pkg-\", path = \"under\" }\n\
				pkg_2d = { package = \"go-package-names-pkg-2d\", path = \"digit\" }\n\n\
				[workspace]\nexclude = [\"under\", \"digit\"]\n",
			),
			(
				"src/lib.rs",
				"pub fn own() -> u32 {\n\t1\n}\n\npub mod under {\n\tpub use pkg_::one;\n}\n\npub mod digit {\n\tpub use pkg_2d::one;\n}\n",
			),
			("under/Cargo.toml", &under),
			("under/src/lib.rs", under_lib),
			("digit/Cargo.toml", &digit),
			("digit/src/lib.rs", digit_lib),
		],
	);
	let out = package.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", package.join("build"))
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"the export failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let go = out.join("go");
	for name in ["pkg", "pkg_", "pkg2d"] {
		assert!(
			go.join(name).join(format!("{name}.go")).exists(),
			"no Go package {name}"
		);
	}
	vet(&go);
}

/// A checkout at another path, with the export beside it as before, exports the same
/// bytes: the crate's version hashes its manifest, which names the facade and its sibling
/// crates relative to the export, and cargo hashes that version into every symbol.
#[test]
fn export_is_relocatable() {
	let files = [
		(
			"Cargo.toml",
			"[package]\nname = \"relocatable\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
			[dependencies]\nunder = { package = \"relocatable-under\", path = \"under\" }\n\n\
			[workspace]\nexclude = [\"under\"]\n",
		),
		("src/lib.rs", "pub use under::one;\n\npub fn two() -> u32 {\n\t2\n}\n"),
		(
			"under/Cargo.toml",
			"[package]\nname = \"relocatable-under\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"under\"\n",
		),
		("under/src/lib.rs", "pub fn one() -> u32 {\n\t1\n}\n"),
	];
	let export = |checkout: PathBuf| {
		write_all(&checkout, &files);
		let out = checkout.join("out");
		let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--manifest-path")
			.arg(checkout.join("Cargo.toml"))
			.arg("--out")
			.arg(&out)
			.env("CARGO_TARGET_DIR", checkout.join("build"))
			.output()
			.unwrap();
		assert!(
			output.status.success(),
			"the export failed:\n{}",
			String::from_utf8_lossy(&output.stderr)
		);
		let mut found = std::collections::BTreeMap::new();
		let mut dirs = vec![out.clone()];
		while let Some(dir) = dirs.pop() {
			for entry in std::fs::read_dir(&dir).unwrap() {
				let path = entry.unwrap().path();
				match path.is_dir() {
					true => dirs.push(path),
					false => {
						let name = path.strip_prefix(&out).unwrap().to_path_buf();
						found.insert(name, std::fs::read(&path).unwrap());
					}
				}
			}
		}
		found
	};
	// With no symlink in it, which an export's own path must lack to be written relative.
	let root = std::fs::canonicalize(scratch()).unwrap().join("relocatable");
	let near = export(root.join("near"));
	let far = export(root.join("far/deeper/still"));
	let manifest = String::from_utf8(near[Path::new("Cargo.toml")].clone()).unwrap();
	assert!(manifest.contains("path = \"..\""), "{manifest}");
	assert!(
		near.keys().eq(far.keys()) && near == far,
		"an export at another path differs: {:?}",
		near.iter()
			.filter(|(k, v)| far.get(*k) != Some(*v))
			.map(|(k, _)| k)
			.collect::<Vec<_>>()
	);
}

/// A type a signature names that the root does not reach, as moq-net's `kio::Pending`,
/// exports by its public path in its own crate, through a re-export or a module, even from a
/// crate the facade re-exports nothing from; one no public path names stays refused. Such a
/// crate's error still crosses as its message, and its type an `Arc` holds stays an opaque
/// handle, refused bare.
#[test]
fn unreached_types() {
	let package = package(
		"unreached_types",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"unreached_types\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"pkg\"\n\n\
				[features]\nrefused = []\n\n[dependencies]\nkio = { package = \"unreached-kio\", path = \"kio\" }\n\
				net = { package = \"unreached-net\", path = \"net\" }\n\
				conv = { package = \"unreached-conv\", path = \"conv\" }\n\n[workspace]\nexclude = [\"kio\", \"net\", \"conv\"]\n",
			),
			(
				"src/lib.rs",
				"pub use kio::Ready;\n\npub fn pending() -> kio::Pending {\n\tkio::Pending(1)\n}\n\n\
				pub fn version() -> kio::version::Version {\n\tkio::version::Version { major: 2 }\n}\n\n\
				#[cfg(feature = \"refused\")] pub fn seal() -> kio::Sealed {\n\tkio::sealed()\n}\n\n\
				pub fn connect() -> Result<net::Link, net::Fault> {\n\tnet::connect()\n}\n\n\
				pub fn handle() -> std::sync::Arc<net::Handle> {\n\tstd::sync::Arc::new(net::Handle(4))\n}\n\n\
				#[cfg(feature = \"refused\")] pub fn bare(handle: &net::Handle) -> u32 {\n\thandle.0\n}\n\n\
				#[cfg(feature = \"refused\")] pub fn opaque() -> Result<(), net::Opaque> {\n\tErr(net::Opaque)\n}\n\n\
				pub fn closed() -> net::Closed {\n\tnet::closed()\n}\n\n\
				pub enum Failure {\n\tNet(net::Fault),\n}\n\npub fn fail() -> Failure {\n\tFailure::Net(net::Fault)\n}\n\n\
				pub struct Buffer(Vec<u32>);\n\nimpl Buffer {\n\tpub fn new() -> Self {\n\t\tSelf(vec![1])\n\t}\n\n\
				\tpub fn drain(&mut self) -> impl Iterator<Item = u32> + '_ {\n\t\tnet::drain(&mut self.0)\n\t}\n}\n\n\
				pub fn copied(items: &[u32]) -> impl Iterator<Item = u32> + '_ {\n\titems.iter().copied()\n}\n\n\
				#[derive(Clone)]\npub struct Held<'a>(&'a u32);\n\nimpl Held<'_> {\n\tpub fn get(&self) -> u32 {\n\t\t*self.0\n\t}\n}\n\n\
				pub fn wrap(x: &u32) -> impl Clone + '_ {\n\tSome(Held(x))\n}\n\n#[derive(Clone)]\npub struct Own(u32);\n\nimpl Own {\n\tpub fn get(&self) -> u32 {\n\t\tself.0\n\t}\n}\n\npub fn own() -> impl Clone {\n\tSome(Own(1))\n}\n\n\
				#[cfg(feature = \"refused\")] impl Buffer {\n\tpub fn first(&self) -> impl Clone + '_ {\n\t\tOwn(self.0[0])\n\t}\n}\n\n\
				pub struct Counter(u32);\n\nimpl Counter {\n\tpub fn get(&self) -> u32 {\n\t\tself.0\n\t}\n}\n\n\
				impl Buffer {\n\tfn counter(&self) -> impl Into<Counter> + '_ {\n\t\tCounter(self.0[0])\n\t}\n\n\
				\tpub fn best(&self) -> impl Into<Counter> + '_ {\n\t\tself.counter()\n\t}\n}\n\n\
				pub fn copied_again(items: &[u32]) -> impl Iterator<Item = u32> + '_ {\n\tcopied(items)\n}\n\n\
				pub fn take(value: impl Into<conv::Value>) -> u32 {\n\tvalue.into().get()\n}\n",
			),
			(
				"net/Cargo.toml",
				"[package]\nname = \"unreached-net\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"net\"\n",
			),
			(
				"net/src/lib.rs",
				"mod link {\n\tpub struct Link(pub u32);\n\n\timpl Link {\n\t\tpub fn id(&self) -> u32 {\n\t\t\tself.0\n\t\t}\n\t}\n}\n\n\
				pub use link::Link;\n\npub struct Handle(pub u32);\n\n#[derive(Debug)]\npub struct Fault;\n\n\
				impl std::fmt::Display for Fault {\n\tfn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\t\tf.write_str(\"fault\")\n\t}\n}\n\n\
				impl std::error::Error for Fault {}\n\n\
				pub struct Opaque;\n\nmod closed {\n\tpub struct Closed;\n}\n\npub type Closed = closed::Closed;\n\n\
				pub fn closed() -> closed::Closed {\n\tclosed::Closed\n}\n\n\
				pub fn connect() -> Result<Link, Fault> {\n\tOk(Link(1))\n}\n\n\
				pub struct Drain<'a>(&'a mut Vec<u32>);\n\n\
				pub fn drain(items: &mut Vec<u32>) -> Drain<'_> {\n\tDrain(items)\n}\n\n\
				impl Iterator for Drain<'_> {\n\ttype Item = u32;\n\n\tfn next(&mut self) -> Option<u32> {\n\t\tself.0.pop()\n\t}\n}\n",
			),
			(
				"conv/Cargo.toml",
				"[package]\nname = \"unreached-conv\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"conv\"\n",
			),
			(
				"conv/src/lib.rs",
				"pub struct Value(u32);\n\nimpl Value {\n\tpub fn new(n: u32) -> Self {\n\t\tSelf(n)\n\t}\n\n\tpub fn get(&self) -> u32 {\n\t\tself.0\n\t}\n}\n",
			),
			(
				"kio/Cargo.toml",
				"[package]\nname = \"unreached-kio\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"kio\"\n",
			),
			(
				"kio/src/lib.rs",
				"pub struct Ready;\n\nmod pollable {\n\tpub struct Pending(pub u32);\n\n\timpl Pending {\n\t\tpub fn id(&self) -> u32 {\n\t\t\tself.0\n\t\t}\n\t}\n}\n\npub use pollable::Pending;\n\n\
				pub mod version {\n\t#[derive(Clone, Copy)]\n\tpub struct Version {\n\t\tpub major: u32,\n\t}\n}\n\n\
				mod sealed {\n\tpub struct Sealed(pub u32);\n}\n\npub type Sealed = sealed::Sealed;\n\n\
				pub fn sealed() -> sealed::Sealed {\n\tsealed::Sealed(3)\n}\n",
			),
		],
	);
	// The export builds, naming `kio`'s types through its own dependency on `kio`.
	let out = package.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", package.join("build")));
	let glue = std::fs::read_to_string(out.join("src/lib.rs")).unwrap();
	// An `impl Iterator + '_` collected at the call holds no borrow past it.
	assert!(glue.contains("fn pkg_copied("), "{glue}");
	// So does one returning another's, whose hidden type is revealed in full.
	assert!(glue.contains("fn pkg_copied_again("), "{glue}");
	// A conversion target only an `impl Into` names is reached, and taken.
	assert!(
		glue.contains("rt::Object::<::conv_reached::Value>::reserve(p0, takes, w)"),
		"{glue}"
	);
	assert!(glue.contains("::kio_reached::Pending"), "{glue}");
	// `net`'s error crosses as its message, or holds `Failure`, which holds it, an object, and
	// its `Arc` crosses as an opaque handle.
	assert!(
		glue.contains("::net_reached::Link") && !glue.contains("Fault") && !glue.contains("net_reached::Handle"),
		"{glue}"
	);
	// Apart from the export's own build, which cleans the facade each time.
	run(cargo()
		.arg("build")
		.arg("--manifest-path")
		.arg(out.join("Cargo.toml"))
		.env("CARGO_TARGET_DIR", package.join("glue")));
	// Staged on another host, with the checkout elsewhere, the merged export depends on the
	// exporting host's `kio`, not the staging host's, though only the staged target reaches it.
	let staged = package.join("wasm.json");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.args(["--api", staged.to_str().unwrap(), "--target", WASM])
		.env("CARGO_TARGET_DIR", package.join("build")));
	let moved = scratch().join("unreached_types-moved");
	let _ = std::fs::remove_dir_all(&moved);
	for dir in ["src", "kio", "net", "conv"] {
		step::mirror(&package.join(dir), &moved.join(dir));
	}
	std::fs::copy(package.join("Cargo.toml"), moved.join("Cargo.toml")).unwrap();
	let lib = std::fs::read_to_string(package.join("src/lib.rs")).unwrap();
	let (head, rest) = lib.split_once("pub fn pending").unwrap();
	let rest = &rest[rest.find("#[cfg(feature = \"refused\")] pub fn seal").unwrap()..];
	std::fs::write(moved.join("src/lib.rs"), format!("{head}{rest}")).unwrap();
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(moved.join("Cargo.toml"))
		.arg("--out")
		.arg(moved.join("out"))
		.arg("--with")
		.arg(&staged)
		.env("CARGO_TARGET_DIR", moved.join("build")));
	let manifest = std::fs::read_to_string(moved.join("out/Cargo.toml")).unwrap();
	// Relative to the export, or absolute where the scratch path holds a symlink.
	let kio = format!("path = {:?}", moved.join("kio").display().to_string());
	assert!(
		(manifest.contains("path = \"../kio\"") || manifest.contains(&kio))
			&& !manifest.contains(&format!("{:?}", package.join("kio").display().to_string())),
		"{manifest}"
	);
	// The census, with the items that fail the export.
	let census = package.join("census");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--census")
		.arg(&census)
		.args(["--", "--features", "refused"])
		.env("CARGO_TARGET_DIR", package.join("build"))
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let report = std::fs::read_to_string(census.join("unreached_types.md")).unwrap();
	// `Pending`, `Version`, `Link`, `Failure`, `Buffer`, and `conv::Value` export; `Sealed`
	// alone is refused as unreached, though an alias names it. `net`'s error in `Failure` is
	// never reached, so a handle holds it by value, and `Failure`, which holds it, is an object.
	assert!(report.contains("| Types | 6 | 0 |"), "{report}");
	assert!(
		report.contains("- `Failure`: object in a value `net::Fault`"),
		"{report}"
	);
	// A type an `Arc` holds, and a `Result`'s error without `Display`, stay refused as
	// foreign; one no public path names is held by value instead.
	for refused in [
		"fn `bare` at src/lib.rs:23:29: foreign type `net::Handle`",
		"fn `opaque` at src/lib.rs:27:29: foreign type `net::Opaque`",
	] {
		assert!(report.contains(refused), "{report}");
	}
	assert!(!report.contains("`closed`"), "{report}");
	// `impl Iterator + '_` lends a borrow of the receiver, so `drain` is refused and `Drain`
	// left out with it, rather than glue that does not build; so is `wrap`, whose hidden
	// `Option` holds a borrowing `Held`.
	assert!(
		report.contains("- `Buffer::drain`: type C never holds `net::Drain`"),
		"{report}"
	);
	assert!(report.contains("- `wrap`: type C never holds `Held`"), "{report}");
	// So is `best`, whose `impl Into<Counter> + '_` hides another `impl Trait` hiding a
	// `Counter`, an object its handle would hold the receiver's borrow for.
	assert!(
		report.contains("- `Buffer::best`: type C never holds `Counter`"),
		"{report}"
	);
	// The glue cannot destructure a hidden type it cannot name, so only an object crosses
	// through an `impl Trait`: `own`'s `Option` is refused, and `Own` left out with it.
	assert!(report.contains("- `own`: type C never holds `Own`"), "{report}");
	assert!(
		report.contains("fn `seal` at src/lib.rs:11:29: type not reachable from the crate root `kio::sealed::Sealed`"),
		"{report}"
	);
	assert_eq!(report.matches("not reachable").count(), 3, "{report}");
}

/// `go vet` over an export's `go/`, which the header beside it lets cgo check unlinked.
fn vet(go: &Path) {
	run(Command::new("go")
		.args(["vet", "./..."])
		.current_dir(go)
		.env("CGO_ENABLED", "1")
		.env("GOFLAGS", "-buildvcs=false")
		.env("GOTOOLCHAIN", "local"));
}

/// Put the static library `lib` in the Go module `export`'s runtime, as the file the
/// export's one `#cgo LDFLAGS` line links with what else it needs, without the LLVM bitcode
/// the go command refuses there.
fn go_library(lib: &Path, export: &Path) {
	let rt = std::fs::read_to_string(export.join("rt").join("rt.go")).unwrap();
	let mut names = rt.lines().filter_map(|line| {
		let flags = line.strip_prefix("#cgo ")?.split_once(" LDFLAGS: ${SRCDIR}/")?.1;
		Some(flags.split(' ').next().unwrap().to_string())
	});
	let (Some(name), None) = (names.next(), names.next()) else {
		panic!("the Go links no one library for the host: {rt}");
	};
	// Beside the module, so layouts that tests build at once strip apart.
	let stripped = export.parent().unwrap().with_extension("a");
	let objcopy = llvm_objcopy();
	Step {
		stamp: &step::stamp(&stripped),
		reads: &[lib, objcopy],
		writes: &[&stripped],
	}
	.run(
		Command::new(objcopy)
			.args(["--remove-section=.llvmbc", "--remove-section=.llvmcmd"])
			.args(["--remove-section=__LLVM,__bitcode", "--remove-section=__LLVM,__cmdline"])
			.arg(lib)
			.arg(&stripped),
	);
	step::copy(&stripped, &export.join("rt").join(name));
}

/// The nightly's llvm-objcopy, which reads every target's objects.
fn llvm_objcopy() -> &'static Path {
	static OBJCOPY: OnceLock<PathBuf> = OnceLock::new();
	OBJCOPY.get_or_init(|| {
		let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
		let sysroot = run(Command::new(rustc).args(["--print", "sysroot"]));
		let sysroot = PathBuf::from(String::from_utf8_lossy(&sysroot.stdout).trim());
		let bin = sysroot.join("lib").join("rustlib").join(host()).join("bin");
		bin.join(format!("llvm-objcopy{EXE_SUFFIX}"))
	})
}

/// The go command in `module`, whose runtime links the api fixture's static library.
fn go(module: &Path, args: &[&str]) -> Command {
	let mut cmd = Command::new("go");
	cmd.args(args)
		.current_dir(module)
		.env("CGO_ENABLED", "1")
		// The module is no checkout of its own, and must build with the Go on PATH.
		.env("GOFLAGS", "-buildvcs=false")
		.env("GOTOOLCHAIN", "local");
	cmd
}

/// Exporting an unchanged crate again moves no file's mtime, the lockfile's included, so a
/// build of the export stays fresh, and drops a file it no longer writes from a directory
/// it owns.
#[test]
fn reexport() {
	let package = package("reexport", &[]);
	let out = package.join("out");
	let export = || {
		run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--manifest-path")
			.arg(package.join("Cargo.toml"))
			.arg("--out")
			.arg(&out)
			.env("CARGO_TARGET_DIR", package.join("build")))
	};
	export();
	fn files(dir: &Path) -> Vec<PathBuf> {
		let mut found = Vec::new();
		for entry in std::fs::read_dir(dir).unwrap() {
			let path = entry.unwrap().path();
			match path.is_dir() {
				true => found.extend(files(&path)),
				false => found.push(path),
			}
		}
		found.sort();
		found
	}
	let written = files(&out);
	assert!(written.contains(&out.join("Cargo.lock")), "{written:?}");
	// In the past, so a rewrite would move it.
	let past = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1 << 30);
	for file in &written {
		let file = std::fs::File::options().write(true).open(file).unwrap();
		file.set_modified(past).unwrap();
	}
	let stray = out.join("go").join("stray").join("stray.go");
	step::write(&stray, "package stray\n");
	export();
	assert_eq!(files(&out), written, "the second export wrote other files");
	for file in &written {
		let mtime = std::fs::metadata(file).unwrap().modified().unwrap();
		assert_eq!(mtime, past, "the second export rewrote {}", file.display());
	}
	assert!(
		!stray.parent().unwrap().exists(),
		"the export kept what it no longer writes"
	);
}

/// A step replays while what it reads and what it wrote are as it left them, and runs again
/// once either changes; a failure a test expects replays too.
#[test]
fn step_reruns_on_change() {
	let dir = scratch().join("step");
	let _ = std::fs::remove_dir_all(&dir);
	let (input, repo) = (dir.join("input"), dir.join("repo"));
	step::write(&input, "one");
	let stamp = dir.join("init.stamp");
	let step = Step {
		stamp: &stamp,
		reads: &[&input],
		writes: &[&repo],
	};
	let head = repo.join(".git").join("HEAD");
	let past = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1 << 30);
	// Whether the step ran since `head` was last set in the past.
	let ran = |run: &dyn Fn()| {
		let file = std::fs::File::options().write(true).open(&head).unwrap();
		file.set_modified(past).unwrap();
		run();
		std::fs::metadata(&head).unwrap().modified().unwrap() != past
	};
	let init = || {
		step.run(Command::new("git").args(["init", "--quiet"]).arg(&repo));
	};
	init();
	assert!(!ran(&init), "an unchanged step ran again");
	step::write(&input, "two");
	assert!(ran(&init), "a step whose input changed replayed");
	step::write(&repo.join("stray"), "");
	assert!(ran(&init), "a step whose output changed replayed");
	assert!(
		!repo.join("stray").exists(),
		"a step's run kept what an earlier one left"
	);

	let stamp = dir.join("failed.stamp");
	let failed = Step {
		stamp: &stamp,
		reads: &[&input],
		writes: &[],
	};
	let first = failed.output(Command::new("git").arg("--no-such-flag"));
	assert!(
		!first.status.success() && stamp.exists(),
		"an expected failure was not kept"
	);
	let again = failed.output(Command::new("git").arg("--no-such-flag"));
	assert_eq!((again.status, again.stderr), (first.status, first.stderr));
}

/// A relative `CARGO_TARGET_DIR` is the caller's, wherever cargo runs.
#[test]
fn relative_target() {
	let dir = scratch().join("relative");
	std::fs::create_dir_all(&dir).unwrap();
	let census = dir.join("census");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(manifest_dir().join("fixtures/api/Cargo.toml"))
		.arg("--census")
		.arg(&census)
		.current_dir(&dir)
		.env("CARGO_TARGET_DIR", "build")
		.output()
		.expect("running rust2-ffi");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(output.status.success(), "the census failed:\n{stderr}");
	let text = std::fs::read_to_string(census.join("api-fixture.md")).unwrap();
	// The fixture exports whole: every row counts no refusal.
	let refused = (text.lines())
		.take_while(|line| !line.starts_with("Exported types:"))
		.filter_map(|line| line.split(" | ").nth(2)?.parse::<usize>().ok());
	assert_eq!(refused.sum::<usize>(), 0, "the census refuses some:\n{text}");
	for line in [
		"- `Lease`: object no export returns",
		"- `Void`: object no export returns",
		"- `release`: type C never holds `Lease`",
		"- `handover`: type C never holds `tls::held::Handoff`",
		"- `Unnamed`: generic type no export instantiates",
		"- `blame`: type C never holds `tls::held::Invalid`",
		// A type parameter only another crate's trait bounds, which no export implements.
		"- `pump`: bound no type C holds meets `tls::held::Stream`",
	] {
		assert!(text.contains(line), "the census lists no `{line}`:\n{text}");
	}
	// Its error a handle C never holds, `verify` still exports, the error as its message.
	assert!(!text.contains("`verify`"), "the census lists `verify`:\n{text}");
	// Nor does `parse_cert`'s, though it borrows the text.
	assert!(!text.contains("`parse_cert`"), "the census lists `parse_cert`:\n{text}");
	assert!(dir.join("build").exists());
}

#[test]
fn reject() {
	let out = scratch().join("reject").join("out");
	let output = export("reject", &out, &[], &[]);
	assert!(!output.status.success(), "exporting reject succeeded");
	let errors = refusals(&output);
	// Every refusal says how to change the facade.
	for error in &errors {
		assert!(error.contains("\n\thelp: "), "no help for {error}");
	}
	let expected = std::fs::read_to_string(manifest_dir().join("fixtures/reject/expected.txt")).unwrap();
	if std::env::var_os("UPDATE_EXPECT").is_some() {
		std::fs::write(
			manifest_dir().join("fixtures/reject/expected.txt"),
			errors.join("\n") + "\n",
		)
		.unwrap();
		return;
	}
	assert_eq!(errors.join("\n") + "\n", expected);
}

/// A function instantiating itself without end, two ways at each step, refused at the
/// recursion limit as rustc's collector refuses it, rather than walked until memory runs out.
#[test]
fn lacks_recursion() {
	let dir = scratch().join("lacks").join("recursion");
	let output = export("lacks", &dir, &[WEB], &["--features", "recursion"]);
	assert!(!output.status.success(), "exporting lacks succeeded");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("rust2-ffi: reached the recursion limit while instantiating `lacks::nest`"),
		"{stderr}"
	);
}

/// What the web target's host lacks, refused at the call in the crate's own code nearest
/// it, by every route the walk follows. Without the call of an undefined C function, whose
/// module would not link, the module builds anyway: what it imports that the binding does
/// not implement is exactly what the refusals name.
#[test]
fn lacks() {
	let dir = scratch().join("lacks");
	let output = export("lacks", &dir.join("undefined"), &[WEB], &["--features", "undefined"]);
	assert!(!output.status.success(), "exporting lacks succeeded");
	let prefix = format!("{}/", manifest_dir().display());
	let refused: Vec<String> = errors(&output).iter().map(|e| e.replace(&prefix, "")).collect();
	let expected = manifest_dir().join("fixtures/lacks/expected.txt");
	if std::env::var_os("UPDATE_EXPECT").is_some() {
		std::fs::write(&expected, refused.join("\n") + "\n").unwrap();
	} else {
		assert_eq!(refused.join("\n") + "\n", std::fs::read_to_string(&expected).unwrap());
	}

	let out = dir.join("out");
	let output = export("lacks", &out, &[WEB], &[]);
	assert!(!output.status.success(), "exporting lacks succeeded");
	// Each `module.name` a refusal names.
	let named: BTreeSet<String> = (errors(&output).iter())
		.flat_map(|error| {
			error
				.split('`')
				.skip(1)
				.step_by(2)
				.map(str::to_string)
				.collect::<Vec<_>>()
		})
		.filter(|token| {
			token
				.split_once('.')
				.is_some_and(|(m, n)| !m.is_empty() && !n.contains(['.', ' ', ':']))
		})
		.collect();
	let target = dir.join("glue");
	run(cargo()
		.args(["build", "--quiet", "--locked", "--manifest-path"])
		.arg(out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(&target)
		.args(["--target", WEB, "--features", "web"]));
	let wasm = std::fs::read(target.join(WEB).join("debug").join("lacks.wasm")).unwrap();
	// The WASI functions the runtime implements, each a method of its `wasi` object.
	let runtime = std::fs::read_to_string(manifest_dir().join("runtime/rt.ts")).unwrap();
	let wasi = &runtime[runtime.find("\nconst wasi = {\n").unwrap()..];
	let wasi = &wasi[..wasi.find("\n};\n").unwrap()];
	let implemented: Vec<&str> = (wasi.lines())
		.filter_map(|line| line.strip_prefix('\t')?.split_once('(').map(|(name, _)| name))
		.filter(|name| name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
		.collect();
	// The binding's own module's functions: each the generated TypeScript names.
	let binding: String = (std::fs::read_dir(out.join("web")).unwrap())
		.map(|entry| std::fs::read_to_string(entry.unwrap().path()).unwrap())
		.collect();
	let mut lacking = BTreeSet::new();
	for payload in wasmparser::Parser::new(0).parse_all(&wasm) {
		if let wasmparser::Payload::ImportSection(section) = payload.unwrap() {
			for import in section.into_imports() {
				let import = import.unwrap();
				let has = match import.module {
					"rust2" => binding.contains(import.name),
					"wasi_snapshot_preview1" => implemented.contains(&import.name),
					_ => false,
				};
				if !has {
					lacking.insert(format!("{}.{}", import.module, import.name));
				}
			}
		}
	}
	assert!(!lacking.is_empty(), "the module imports nothing the host lacks");
	assert_eq!(
		lacking, named,
		"the module's imports the host lacks, and what the export refused"
	);
}

/// Each of rust2-ffi's errors the export printed, sorted: at its span, then each of its
/// notes at its own.
fn errors(output: &Output) -> Vec<String> {
	diagnostics(output, false)
}

/// [`errors`], each with its `help:` notes after its span notes, which a refusal carries
/// for how to change the facade. A baseline break's names a path, so [`errors`] leaves it out.
fn refusals(output: &Output) -> Vec<String> {
	diagnostics(output, true)
}

fn diagnostics(output: &Output, helps: bool) -> Vec<String> {
	let stderr = String::from_utf8_lossy(&output.stderr);
	let lines: Vec<&str> = stderr.lines().collect();
	let at = |i: usize| {
		lines[i + 1..]
			.iter()
			.take_while(|l| !l.starts_with("error") && !l.starts_with("warning") && !l.starts_with("note: "))
			.find_map(|l| l.trim_start().strip_prefix("--> "))
			.unwrap_or("no span")
	};
	let mut errors: Vec<String> = Vec::new();
	let mut ours = false;
	for (i, line) in lines.iter().enumerate() {
		if let Some(error) = line.strip_prefix("error: rust2-ffi:") {
			errors.push(format!("{}: rust2-ffi:{error}", at(i)));
			ours = true;
		} else if line.starts_with("error") || line.starts_with("warning") {
			ours = false;
		} else if let Some(note) = line.strip_prefix("note: ")
			&& ours
		{
			let error = errors.last_mut().expect("a note follows its error");
			*error += &format!("\n\t{}: note: {note}", at(i));
		} else if helps
			&& ours
			&& let Some(help) = line.trim_start().strip_prefix("= help: ")
		{
			let error = errors.last_mut().expect("a help follows its error");
			*error += &format!("\n\thelp: {help}");
		}
	}
	errors.sort();
	errors
}

/// Cleanup must refuse a baseline anywhere below a replaced directory.
fn lock_baseline_cleanup() {
	let dir = scratch().join("lock-cleanup");
	let base = dir.join("base");
	let output = export("lock", &base, &[], &[]);
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let before = std::fs::read(base.join("api.json")).unwrap();
	for replaced in ["go", "include/lock"] {
		let out = dir.join(replaced.replace('/', "-"));
		let baseline = out.join(replaced).join("nested/baseline.json");
		std::fs::create_dir_all(baseline.parent().unwrap()).unwrap();
		std::fs::write(&baseline, &before).unwrap();
		let args = ["--baseline".as_ref(), baseline.as_os_str()];
		let output = exporter("lock", &out, &[], &args, &[]).output().unwrap();
		assert!(!output.status.success(), "export deleted baseline inside {replaced}");
		assert_eq!(std::fs::read(&baseline).unwrap(), before);
		assert!(String::from_utf8_lossy(&output.stderr).contains("export replaces"));
	}
}

/// Adding a thrown type can remove a Swift overload without changing its Rust function.
fn lock_swift_removed_overload() {
	let dir = scratch().join("lock-overload");
	let base = dir.join("base");
	let output = export("lock", &base, &[], &[]);
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let baseline = base.join("api.json");
	let args = ["--baseline".as_ref(), baseline.as_os_str()];
	let out = dir.join("thrown");
	let _ = std::fs::remove_dir_all(&out);
	let output = exporter("lock", &out, &[], &args, &["--features", "thrown"])
		.output()
		.unwrap();
	assert!(!output.status.success(), "removed Swift overload passed the lock");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains(
			"overload omitting Rust parameter indices 0 (zero-based): its Swift name `debugDescription` is no longer exported"
		),
		"{stderr}"
	);
	assert!(!out.join("api.json").exists());
}

/// The lock fixture exported for wasm32, staged, and the host, then again against that
/// export's API as the baseline: with `next`, which must fail at each break, and with
/// `added`, which must pass.
#[test]
fn lock() {
	lock_baseline_cleanup();
	lock_swift_removed_overload();
	let dir = scratch().join("lock");
	let targets = [WASM, host()];
	let output = export("lock", &dir.join("base"), &targets, &[]);
	assert!(
		output.status.success(),
		"exporting failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let baseline = dir.join("base").join("api.json");
	let before = std::fs::read(&baseline).unwrap();
	let json: serde_json::Value = serde_json::from_slice(&before).unwrap();
	for language in ["C++", "Kotlin", "Swift", "Dart", "Python", "Go"] {
		assert!(
			json["names"][language]["const lock::DEPTH"].is_string(),
			"{language} constant name missing"
		);
	}
	for missing in ["all", "Dart"] {
		let mut bad = json.clone();
		if missing == "all" {
			bad.as_object_mut().unwrap().remove("names");
		} else {
			bad["names"].as_object_mut().unwrap().remove(missing);
		}
		let path = dir.join(format!("missing-{missing}.json"));
		std::fs::write(&path, serde_json::to_vec(&bad).unwrap()).unwrap();
		let args = ["--baseline".as_ref(), path.as_os_str()];
		let out = dir.join(format!("missing-{missing}"));
		let _ = std::fs::remove_dir_all(&out);
		let output = exporter("lock", &out, &targets, &args, &[]).output().unwrap();
		assert!(!output.status.success());
		assert_eq!(
			String::from_utf8_lossy(&output.stderr)
				.matches("baseline lacks")
				.count(),
			1
		);
		assert!(!out.exists(), "a baseline without names wrote {}", out.display());
	}
	// A baseline from before names were kept still has its structure checked.
	let path = dir.join("missing-all.json");
	let args = ["--baseline".as_ref(), path.as_os_str()];
	let out = dir.join("missing-next");
	let _ = std::fs::remove_dir_all(&out);
	let output = exporter("lock", &out, &targets, &args, &["--features", "next"])
		.output()
		.unwrap();
	assert!(!out.exists(), "a refused export wrote {}", out.display());
	assert!(!output.status.success());
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(stderr.contains("baseline lacks binding names"), "{stderr}");
	assert!(
		stderr.contains("the API breaks the baseline at fn `lock::removed`: it is removed"),
		"no structural break against a baseline without names:\n{stderr}"
	);
	let args = ["--baseline".as_ref(), baseline.as_os_str()];
	let itself = exporter("lock", &dir.join("base"), &targets, &args, &[])
		.output()
		.expect("running rust2-ffi");
	assert!(!itself.status.success(), "a baseline the export would overwrite passed");
	assert_eq!(
		std::fs::read(&baseline).unwrap(),
		before,
		"the export overwrote its baseline"
	);
	let linked = dir.join("linked");
	let _ = std::fs::remove_dir_all(&linked);
	std::fs::create_dir_all(&linked).unwrap();
	std::fs::hard_link(&baseline, linked.join("api.json")).unwrap();
	let args = ["--baseline".as_ref(), baseline.as_os_str()];
	let link = exporter("lock", &linked, &targets, &args, &[])
		.output()
		.expect("running rust2-ffi");
	assert!(
		!link.status.success(),
		"a hard-linked baseline the export would overwrite passed"
	);
	assert_eq!(
		std::fs::read(&baseline).unwrap(),
		before,
		"the export overwrote its hard-linked baseline"
	);
	// Every path starts with the library's name, so a baseline of another library breaks.
	let renamed = dir.join("renamed.json");
	std::fs::write(
		&renamed,
		String::from_utf8(before.clone())
			.unwrap()
			.replace("\"lock\"", "\"other\""),
	)
	.unwrap();
	let args = ["--baseline".as_ref(), renamed.as_os_str()];
	let other = exporter("lock", &dir.join("renamed"), &targets, &args, &[])
		.output()
		.expect("running rust2-ffi");
	assert!(!other.status.success(), "a baseline of another library passed");
	let stderr = String::from_utf8_lossy(&other.stderr);
	assert!(
		stderr.contains("library `other`: it is now `lock`"),
		"no library error:\n{stderr}"
	);
	// A baseline an earlier export wrote, before records had their sizes, still reads.
	let older = dir.join("older.json");
	let mut json = String::from_utf8(before.clone()).unwrap();
	while let Some(at) = json.find(",\"sizes\":{") {
		let end = at + json[at..].find('}').unwrap() + 1;
		json.replace_range(at..end, "");
	}
	std::fs::write(&older, json).unwrap();
	let args = ["--baseline".as_ref(), older.as_os_str()];
	let read = exporter("lock", &dir.join("older"), &targets, &args, &[])
		.output()
		.expect("running rust2-ffi");
	assert!(
		read.status.success(),
		"a baseline without sizes failed:\n{}",
		String::from_utf8_lossy(&read.stderr)
	);
	let locked = |feature: &str| {
		let args = ["--baseline".as_ref(), baseline.as_os_str()];
		exporter("lock", &dir.join(feature), &targets, &args, &["--features", feature])
			.output()
			.expect("running rust2-ffi")
	};
	let added = locked("added");
	assert!(
		added.status.success(),
		"an addition broke the baseline:\n{}",
		String::from_utf8_lossy(&added.stderr)
	);
	// Kotlin throws `samples` and `samples_set` apart under one error key, which the
	// transcript, knowing no Kotlin type, refuses to resolve rather than name either class.
	let source = dir.join("added-transcript.rs");
	std::fs::write(&source, "fn main() {}").unwrap();
	let t = transcript::Transcript::load(&dir.join("added").join("api.json"), &source);
	for name in ["samples", "samples_set"] {
		let f = t.api.functions.iter().find(|f| f.path == ["lock", name]).unwrap();
		let err = f.err.as_ref().unwrap();
		let panic = std::panic::catch_unwind(|| t.error("Kotlin", err))
			.err()
			.unwrap_or_else(|| panic!("Kotlin's exception of {name} resolved"));
		let message = panic.downcast_ref::<String>().unwrap();
		assert!(message.contains("cannot tell apart"), "{message}");
	}
	// A baseline from before an exception was known by its native type too, whose key alone
	// names one class of those an addition gives it.
	let original: serde_json::Value = serde_json::from_slice(&before).unwrap();
	let mut legacy = original.clone();
	for names in legacy["names"].as_object_mut().unwrap().values_mut() {
		names.as_object_mut().unwrap().retain(|key, _| !key.contains(" / as "));
	}
	assert_ne!(legacy, original, "the baseline holds no native exception key to drop");
	let path = dir.join("legacy.json");
	std::fs::write(&path, serde_json::to_vec(&legacy).unwrap()).unwrap();
	let args = ["--baseline".as_ref(), path.as_os_str()];
	let output = exporter("lock", &dir.join("legacy"), &targets, &args, &["--features", "added"])
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"an addition broke a baseline without native exception keys:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	// A generator that respells a native type, such as an alias it renames, renames no class.
	let mut respelled = original.clone();
	for names in respelled["names"].as_object_mut().unwrap().values_mut() {
		let names = names.as_object_mut().unwrap();
		*names = std::mem::take(names)
			.into_iter()
			.map(|(key, name)| match key.contains(" / as ") {
				true => (format!("{key}_respelled"), name),
				false => (key, name),
			})
			.collect();
	}
	assert_ne!(
		respelled, original,
		"the baseline holds no native exception key to respell"
	);
	let path = dir.join("respelled.json");
	std::fs::write(&path, serde_json::to_vec(&respelled).unwrap()).unwrap();
	let args = ["--baseline".as_ref(), path.as_os_str()];
	let output = exporter(
		"lock",
		&dir.join("respelled"),
		&targets,
		&args,
		&["--features", "added"],
	)
	.output()
	.unwrap();
	assert!(
		output.status.success(),
		"respelling a native exception type broke the baseline:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let names = locked("names");
	assert!(!names.status.success(), "binding renames passed the lock");
	let stderr = String::from_utf8_lossy(&names.stderr);
	// Dart's lints allow no `_` in a type's name, so it takes a number instead.
	for (language, new) in [
		("Kotlin", "GraphicsEncoder_Finish"),
		("Swift", "GraphicsEncoder_Finish"),
		("Go", "GraphicsEncoder_Finish"),
		("Dart", "GraphicsEncoderFinish2"),
		("Python", "GraphicsEncoder_Finish"),
	] {
		assert!(
			stderr.contains(&format!("its {language} name `GraphicsEncoderFinish` is now `{new}`")),
			"missing class rename for {language}:\n{stderr}"
		);
	}
	for (language, old, new) in [
		("Kotlin", "smoothStep", "smoothStep_"),
		("Swift", "smoothStep", "smoothStep_"),
		("Dart", "smoothStep", "smoothStep_"),
	] {
		assert!(
			stderr.contains(&format!("its {language} name `{old}` is now `{new}`")),
			"missing member rename for {language}:\n{stderr}"
		);
	}
	// Python's and Go's cases spell `smooth_step` alone as its Rust, so it keeps its name.
	for (language, old) in [("Python", "smooth_step"), ("Go", "SmoothStep")] {
		assert!(
			!stderr.contains(&format!("its {language} name `{old}` is now")),
			"a {language} name spelled as its Rust moved:\n{stderr}"
		);
	}

	assert!(!stderr.contains("its C name"), "a C name changed:\n{stderr}");
	assert!(!dir.join("names/api.json").exists());
	let members = locked("members");
	assert!(!members.status.success());
	let stderr = String::from_utf8_lossy(&members.stderr);
	assert!(
		stderr.contains("its C++ name `value` is now `value_`"),
		"C++ member rename missing:\n{stderr}"
	);
	assert!(!stderr.contains("its C name"), "a C name changed:\n{stderr}");
	let wrappers = locked("wrappers");
	assert!(!wrappers.status.success());
	let stderr = String::from_utf8_lossy(&wrappers.stderr);
	for text in [
		"its Kotlin name `StringException` is now `StringException_`",
		"its Python name `StringException` is now `StringException_`",
		"its Swift name `StringError` is now `StringError_`",
		"its Dart name `StringException` is now `StringException2`",
		"its Dart name `ShapeCircle` is now `ShapeCircle2`",
		"its Go name `ModeFast` is now `ModeFast_`",
		"its Go name `GoString` is now `GoString_`",
		"/ overload omitting Rust parameter indices 0 (zero-based): its Swift name `debugDescription` is now `debugDescription_`",
	] {
		assert!(stderr.contains(text), "missing {text}:\n{stderr}");
	}
	assert!(!stderr.contains("its C name"), "a C name changed:\n{stderr}");
	let defaults = locked("defaults");
	assert!(!defaults.status.success(), "a Go Default rename passed the lock");
	let stderr = String::from_utf8_lossy(&defaults.stderr);
	assert!(
		stderr.contains("type `lock::Meter` / Default: its Go name `MeterDefault` is now `MeterDefault_`"),
		"Go Default rename missing:\n{stderr}"
	);
	let closure_base = dir.join("closure-base");
	let output = export("lock", &closure_base, &targets, &["--features", "closures"]);
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let closure_baseline = closure_base.join("api.json");
	let args = ["--baseline".as_ref(), closure_baseline.as_os_str()];
	let output = exporter(
		"lock",
		&dir.join("closure-break"),
		&targets,
		&args,
		&["--features", "closures,wrappers"],
	)
	.output()
	.unwrap();
	assert!(!output.status.success());
	let stderr = String::from_utf8_lossy(&output.stderr);
	for (language, new) in [
		("Kotlin", "StringException_"),
		("Python", "StringException_"),
		("Dart", "StringException2"),
	] {
		assert!(
			stderr.contains(&format!(
				"exception class for `string`: its {language} name `StringException` is now `{new}`"
			)),
			"{stderr}"
		);
	}
	let next = locked("next");
	assert!(!next.status.success(), "breaking the baseline passed");
	assert!(
		!dir.join("next").join("api.json").exists(),
		"a broken export wrote its API"
	);
	let errors = errors(&next).join("\n") + "\n";
	let expected = manifest_dir().join("fixtures/lock/expected.txt");
	if std::env::var_os("UPDATE_EXPECT").is_some() {
		std::fs::write(&expected, errors).unwrap();
		return;
	}
	assert_eq!(errors, std::fs::read_to_string(expected).unwrap());
}

/// The C library builds the facade's dependencies with what the facade enables in them,
/// so a feature enabled directly in one fails loud.
#[test]
fn feature_of_dependency() {
	let out = scratch().join("feature").join("out");
	let output = export("api", &out, &[], &["--features", "bytes/std"]);
	assert!(!output.status.success(), "exporting with bytes/std succeeded");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains(
			"--features bytes/std: the export builds api-fixture's dependencies with only the features it enables in them"
		),
		"{stderr}"
	);
}

/// A one-function package named `name` in its own workspace, written into scratch, with
/// `files` in place of its own or besides: each written once, so a second run moves no
/// mtime and cargo finds the package fresh.
fn package(name: &str, files: &[(&str, &str)]) -> PathBuf {
	let package = scratch().join(name);
	let manifest = format!("[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n");
	let defaults = [
		("Cargo.toml", manifest.as_str()),
		("src/lib.rs", "pub fn one() -> u32 {\n\t1\n}\n"),
	];
	let defaults = defaults
		.into_iter()
		.filter(|(path, _)| files.iter().all(|(file, _)| file != path));
	write_all(&package, &defaults.collect::<Vec<_>>());
	write_all(&package, files);
	package
}

/// A `no_std` facade's API still names the system libraries std links, as the glue's
/// static library links std whatever the facade does: those rustc prints for the api
/// fixture's, whose crates link nothing of their own.
#[test]
fn no_std_links() {
	let package = package(
		"no_std_links",
		&[(
			"src/lib.rs",
			"#![cfg_attr(not(test), no_std)]\n\npub fn one() -> u32 {\n\t1\n}\n",
		)],
	);
	let out = package.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", package.join("build"))
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"exporting failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let exported: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(out.join("api.json")).unwrap()).unwrap();
	let links: Vec<String> = serde_json::from_value(exported["api"]["links"][host()].clone()).unwrap();
	assert_eq!(links, api().native);
}

/// rustc's arguments in an argfile, as a consumer's rustflags name one: inside it, an
/// argument starting with `@` is itself, not another file. Rustflags too long for the
/// environment the check passes them on in, which cargo passes rustc in an argfile of its
/// own, fail the export.
#[test]
fn argfile() {
	let package = package("argfile", &[]);
	let argfile = package.join("flags");
	std::fs::write(&argfile, "--remap-path-prefix\n@a=/\n").unwrap();
	let config = |flags: &str| [(".cargo/config.toml", format!("build.rustflags = [{flags:?}]\n"))];
	let [(path, text)] = config(&format!("@{}", argfile.display()));
	write_all(&package, &[(path, &text)]);
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	// Past what one environment variable holds on Windows, and past what cargo passes rustc
	// without an argfile on any platform. Windows's command line holds no more than its
	// variable, so there cargo's argfile comes first.
	let too_long = "rustflags are too long to pass on";
	for (len, error) in [
		(1 << 16, if cfg!(windows) { too_long } else { "rustflags take" }),
		(1 << 21, too_long),
	] {
		let [(path, text)] = config(&format!("--cfg=a{}", "a".repeat(len)));
		write_all(&package, &[(path, &text)]);
		let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(!output.status.success() && stderr.contains(error), "{stderr}");
	}
}

/// A compiler cache in cargo's `RUSTC_WRAPPER`, as mbx's Cargo shim sets one, wraps every
/// crate: the workspace's own through the exporter's wrapper, and every other crate with
/// the exporter as rustc, the consumer's rustflags and then the export's on its command
/// line, which the cache keys it by.
#[cfg(unix)]
#[test]
fn cached() {
	let package = package(
		"cached",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"cached\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
				[dependencies]\ndep = { path = \"../cached_dep\" }\n\n[workspace]\n",
			),
			("src/lib.rs", "pub use dep::one;\n"),
			// Cargo queries the target once without these, which it reads only from the
			// target's `cfg`s, and again with them.
			(
				".cargo/config.toml",
				"[target.'cfg(all())']\nrustflags = [\"--cfg\", \"consumer\"]\n",
			),
		],
	);
	write_all(
		&package.with_file_name("cached_dep"),
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"dep\"\nversion = \"0.0.0\"\nedition = \"2024\"\n",
			),
			("src/lib.rs", "pub fn one() -> u32 {\n\t1\n}\n"),
		],
	);
	let log = package.join("wrapped.log");
	let wrapper = package.join("wrapper.sh");
	let cargo = package.join("cargo.sh");
	write_all(
		&package,
		&[
			(
				"wrapper.sh",
				&format!("#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec \"$@\"\n", log.display()),
			),
			(
				"cargo.sh",
				&format!(
					"#!/bin/sh\nRUSTC_WRAPPER='{}' exec '{}' \"$@\"\n",
					wrapper.display(),
					std::env::var("CARGO").unwrap()
				),
			),
		],
	);
	for script in [&wrapper, &cargo] {
		std::fs::set_permissions(script, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
	}
	let _ = std::fs::remove_file(&log);
	let _ = std::fs::remove_dir_all(package.join("build"));
	run(census(&package.join("Cargo.toml"), &[]).env("CARGO", &cargo));
	let log = std::fs::read_to_string(&log).unwrap();
	let compiled = |krate: &str| {
		let line = log
			.lines()
			.find(|line| line.contains(&format!("--crate-name {krate} ")));
		let line = line.unwrap_or_else(|| panic!("{krate} was not compiled through the cache:\n{log}"));
		line.split(' ').map(str::to_string).collect::<Vec<_>>()
	};
	let (dep, facade) = (compiled("dep"), compiled("cached"));
	// Cargo passes its rustc to the workspace's wrapper alone.
	assert_ne!(dep[0], facade[0], "{log}");
	assert_eq!(dep[0], facade[1], "{log}");
	let flags = [
		"--cfg",
		"consumer",
		"-Zalways-encode-mir",
		"-Coverflow-checks=on",
		"-Zmir-opt-level=1",
	];
	for krate in [&dep, &facade] {
		assert!(krate.windows(flags.len()).any(|run| run == flags), "{log}");
	}
}

/// A census of the package whose manifest is `manifest` with these cargo args, to run.
fn census(manifest: &Path, cargo: &[&str]) -> Command {
	let dir = manifest.parent().unwrap();
	let mut cmd = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"));
	cmd.arg("--manifest-path")
		.arg(manifest)
		.arg("--census")
		.arg(dir.join("census"))
		.arg("--")
		.args(cargo)
		.env("CARGO_TARGET_DIR", dir.join("build"));
	cmd
}

/// A flag given twice is an error, not the last one silently winning.
#[test]
fn flag_twice() {
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.args(["--census", "census", "--manifest-path", "a/Cargo.toml"])
		.args(["--manifest-path", "b/Cargo.toml"])
		.output()
		.unwrap();
	assert!(!output.status.success());
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(stderr.contains("--manifest-path is given twice"), "{stderr}");
}

/// A refused trait counts in the census's table, so a census whose rows sum to no refusal
/// refuses nothing.
#[test]
fn census_counts_traits() {
	let package = package(
		"census_counts_traits",
		&[("src/lib.rs", "pub unsafe trait Trusted {\n\tfn id(&self) -> u32;\n}\n")],
	);
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let report = std::fs::read_to_string(package.join("census/census_counts_traits.md")).unwrap();
	assert!(report.contains("| Traits | 0 | 1 | 0% |"), "{report}");
}

/// The depth an instance may nest to counts what inference names, not only what the
/// signatures spell: an async function's output, and a parameter's implementor, a
/// method's too, which only an instance of its type names.
#[test]
fn census_depth() {
	let lib = "pub struct Wrap<T>(pub T);\n\n\
		pub async fn make() -> Wrap<u8> {\n\tWrap(0)\n}\n\n\
		pub trait Seed: Copy {\n\tfn seed(&self) -> u8;\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Held<T>(pub T);\n\n\
		impl Seed for Held<u16> {\n\tfn seed(&self) -> u8 {\n\t\t1\n\t}\n}\n\n\
		pub fn plant(seed: impl Seed) -> u8 {\n\tseed.seed()\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Host<T = u8>(pub T);\n\n\
		pub trait Grow: Copy {\n\tfn grow(&self) -> u8;\n}\n\n\
		impl Grow for Held<Host<u16>> {\n\tfn grow(&self) -> u8 {\n\t\t2\n\t}\n}\n\n\
		impl<T> Host<T> {\n\tpub fn plant(&self, seed: impl Grow) -> u8 {\n\t\tseed.grow()\n\t}\n}\n";
	let package = package("census_depth", &[("src/lib.rs", lib)]);
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let report = std::fs::read_to_string(package.join("census/census_depth.md")).unwrap();
	assert!(!report.contains("instance deeper"), "{report}");
	assert!(report.contains("| Functions | 2 | 0 |"), "{report}");
	assert!(
		report.contains("| Methods and fields of exported types | 2 | 0 |"),
		"{report}"
	);
}

/// A method Swift refuses leaves out only itself, not the accessors of a field of its name.
#[test]
fn census_swift_field() {
	let lib = "pub struct Token(u8);\n\n\
		pub struct Gate {\n\tpub open: u8,\n\tshut: u8,\n}\n\n\
		pub fn gate() -> Gate {\n\tGate { open: 0, shut: 0 }\n}\n\n\
		impl Gate {\n\tpub fn open(&self, among: Option<&Token>, and: Option<&Token>, also: Option<&Token>) -> u8 {\n\t\t\
		[among, and, also].into_iter().flatten().map(|t| t.0).sum::<u8>() + self.shut\n\t}\n}\n\n\
		pub fn token() -> Token {\n\tToken(1)\n}\n";
	let package = package("census_swift_field", &[("src/lib.rs", lib)]);
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let report = std::fs::read_to_string(package.join("census/census_swift_field.md")).unwrap();
	// The field, by its getter, exports; the method alone is refused.
	assert!(
		report.contains("| Methods and fields of exported types | 1 | 1 |"),
		"{report}"
	);
	assert!(report.contains("- method `Gate::open`"), "{report}");
}

/// Finding the exports keeps to the check's `--locked`: it neither writes a lockfile the
/// check then accepts, nor passes.
#[test]
fn locked() {
	let package = package("locked", &[]);
	let _ = std::fs::remove_file(package.join("Cargo.lock"));
	let output = census(&package.join("Cargo.toml"), &["--locked"]).output().unwrap();
	assert!(!output.status.success(), "a census without a lockfile passed --locked");
	assert!(!package.join("Cargo.lock").exists(), "the lockfile was written");
}

/// A package reached through a symlinked directory is the same package, though cargo
/// keeps the link in its paths. On Unix: making a symlink on Windows takes a privilege.
#[cfg(unix)]
#[test]
fn symlinked() {
	let package = package("symlinked", &[]);
	let link = scratch().join("symlinked-link");
	if !link.exists() {
		std::os::unix::fs::symlink(&package, &link).unwrap();
	}
	let output = census(&link.join("Cargo.toml"), &[]).output().unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
}

/// Two crates of one crate name export through the facade, which shares their name too,
/// by names it calls them that the glue names too, `bytes` and its own crate name, with
/// their sources remapped.
#[test]
fn lookalike() {
	let package = package(
		"lookalike",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"lookalike\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"shared\"\n\n\
				[dependencies]\nshared = { package = \"lookalike-exp\", path = \"exp\" }\n\
				bytes = { package = \"lookalike-other\", path = \"other\" }\n\n\
				[workspace]\nexclude = [\"exp\", \"other\"]\n",
			),
			(
				"src/lib.rs",
				"pub mod exp {\n\tpub use shared::Size;\n}\n\npub mod other {\n\tpub use bytes::Size;\n}\n\n\
				pub fn total(a: shared::Size, b: bytes::Size) -> u32 {\n\ta.width + b.width\n}\n",
			),
		],
	);
	let write = |path: &str, text: &str| {
		step::write(&package.join(path), text);
	};
	// Both libraries are crate `shared`.
	for name in ["exp", "other"] {
		write(
			&format!("{name}/Cargo.toml"),
			&format!(
				"[package]\nname = \"lookalike-{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"shared\"\n"
			),
		);
		write(
			&format!("{name}/src/lib.rs"),
			"pub struct Size {\n\tpub width: u32,\n}\n\nimpl Size {\n\tpub fn new(width: u32) -> Self {\n\t\tSelf { width }\n\t}\n}\n",
		);
	}
	let out = package.join("out");
	let remap = format!("--remap-path-prefix={}=/src", package.display());
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", package.join("build"))
		.env("RUSTFLAGS", remap)
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"the export failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	run(cargo()
		.args(["build", "--quiet", "--locked", "--manifest-path"])
		.arg(out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(package.join("glue"))
		.env("RUSTFLAGS", "-D warnings"));
}

/// A facade whose lib takes a standard library crate's name exports its own items, which
/// are not the standard library's, such as a `SocketAddrV4` record.
#[test]
fn standard_names() {
	for name in ["core", "alloc", "std"] {
		let package = package(&format!("standard_{name}"), &[]);
		let manifest = format!(
			"[package]\nname = \"standard_{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"{name}\"\n\n[workspace]\n"
		);
		std::fs::write(package.join("Cargo.toml"), manifest).unwrap();
		let lib = "pub struct SocketAddrV4 {\n\tpub port: u16,\n}\n\npub fn port(addr: SocketAddrV4) -> u16 {\n\taddr.port\n}\n";
		std::fs::write(package.join("src/lib.rs"), lib).unwrap();
		let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--manifest-path")
			.arg(package.join("Cargo.toml"))
			.arg("--out")
			.arg(package.join("out"))
			.args(["--", "--offline"])
			.env("CARGO_TARGET_DIR", package.join("build"))
			.output()
			.unwrap();
		assert!(
			output.status.success(),
			"exporting lib `{name}` failed:\n{}",
			String::from_utf8_lossy(&output.stderr)
		);
		let header = std::fs::read_to_string(package.join("out/include").join(format!("{name}.h"))).unwrap();
		assert!(
			header.contains(&format!("{name}_SocketAddrV4 addr")),
			"lib `{name}`'s own SocketAddrV4 is no record:\n{header}"
		);
	}
}

/// A `Copy` type crosses as its bytes only when its fields fill it under any field order
/// a `rustc` may pick, with a byte for an enum's tag where no field has a niche to hold
/// one, in a model no target varies, so the export's targets agree; any other is a handle.
/// A huge array is judged without walking its elements: a
/// zero-sized element repeated `usize::MAX` times, or a 1 TiB array of bytes, which no
/// export returns, so it is left out only after it is judged.
#[test]
fn opaque_values() {
	let lib = "#![allow(dead_code)]\n\n#[derive(Clone, Copy)]\npub struct Marker([(); usize::MAX]);\n\nimpl Marker {\n\tpub fn new() -> Self {\n\t\tSelf([(); usize::MAX])\n\t}\n}\n\npub fn take(marker: Marker) -> u32 {\n\tmarker.0.len().count_ones()\n}\n\n\
		#[cfg(target_pointer_width = \"64\")]\n#[derive(Clone, Copy)]\npub struct Huge([u8; 1 << 40]);\n\n#[cfg(target_pointer_width = \"64\")]\npub fn first(huge: &Huge) -> u8 {\n\thuge.0[0]\n}\n\n\
		#[derive(Clone, Copy)]\n#[repr(C)]\nstruct Pair {\n\ta: u8,\n\tb: u16,\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Pairs([Pair; 3]);\n\npub fn pairs() -> Pairs {\n\tPairs([Pair { a: 1, b: 2 }; 3])\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Flag(bool, u8, [u16; 3]);\n\npub fn flag() -> Flag {\n\tFlag(true, 1, [2; 3])\n}\n\n\
		#[derive(Clone, Copy)]\nenum Two {\n\tA(u8),\n\tB(i8),\n}\n\n#[derive(Clone, Copy)]\npub struct Tagged(Two);\n\npub fn tagged() -> Tagged {\n\tTagged(Two::B(-1))\n}\n\n\
		#[derive(Clone, Copy)]\nenum Uneven {\n\tA(u32),\n\tB(u16),\n}\n\n#[derive(Clone, Copy)]\npub struct Wide(Uneven);\n\npub fn wide() -> Wide {\n\tWide(Uneven::B(1))\n}\n\n\
		#[derive(Clone, Copy)]\nenum Niche {\n\tA(bool, u8),\n\tB(u8, u8),\n}\n\n#[derive(Clone, Copy)]\npub struct Niched(Niche);\n\npub fn niched() -> Niched {\n\tNiched(Niche::A(true, 1))\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Word(usize, u32);\n\npub fn word() -> Word {\n\tWord(1, 2)\n}\n\n\
		#[derive(Clone, Copy)]\n#[repr(C)]\npub struct Mixed(u32, u64, u32);\n\npub fn mixed() -> Mixed {\n\tMixed(1, 2, 3)\n}\n\n\
		#[derive(Clone, Copy)]\n#[repr(C)]\npub struct MixedWord(usize, u64);\n\npub fn mixed_word() -> MixedWord {\n\tMixedWord(1, 2)\n}\n\n\
		#[derive(Clone, Copy)]\n#[repr(C)]\npub struct Words(u64, usize, usize);\n\npub fn words() -> Words {\n\tWords(1, 2, 3)\n}\n\n\
		#[derive(Clone, Copy)]\n#[repr(u8)]\nenum Kind {\n\tA,\n\tB,\n}\n\n#[derive(Clone, Copy)]\n#[repr(C)]\npub struct Ordered(u64, u32, u16, u8, Kind);\n\npub fn ordered() -> Ordered {\n\tOrdered(1, 2, 3, 4, Kind::B)\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Scaled {\n\tvalue: u64,\n\tscale: Option<std::num::NonZero<u64>>,\n}\n\npub fn scaled() -> Scaled {\n\tScaled { value: 1, scale: None }\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Lent(Option<&'static u32>);\n\npub fn lent() -> Lent {\n\tLent(None)\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Nested(Option<Option<bool>>);\n\npub fn nested() -> Nested {\n\tNested(Some(None))\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Chars(Option<char>);\n\npub fn chars() -> Chars {\n\tChars(None)\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Unset(Option<u32>);\n\npub fn unset() -> Unset {\n\tUnset(None)\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Narrow(Option<(u32, std::num::NonZero<u32>)>);\n\npub fn narrow() -> Narrow {\n\tNarrow(None)\n}\n\n\
		#[derive(Clone, Copy)]\npub struct Sliced(Option<&'static [u8]>);\n\npub fn sliced() -> Sliced {\n\tSliced(None)\n}\n";
	let package = package("opaque_values", &[("src/lib.rs", lib)]);
	let out = package.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.args(["--target", WASM, "--target", host(), "--", "--offline"])
		.env("CARGO_TARGET_DIR", package.join("build")));
	build_offline(&out, &package.join("glue"));
	let header = std::fs::read_to_string(package.join("out/include/opaque_values.h")).unwrap();
	assert!(
		header.contains("\tuint8_t opaque[1];\n} opaque_values_Marker;"),
		"Marker is no opaque value:\n{header}"
	);
	// A `bool` beside integers that fill the struct; a tag byte beside each variant's byte;
	// fields in an order `repr(C)` keeps that leaves no gap, a `repr(u8)` tag among them;
	// two `usize`s after a `u64`, which fill it at either pointer width; a `None` that writes
	// its niche's whole scalar, a `u64`'s, a pointer's at either width, a `bool`'s an outer
	// `None` shares with an inner one, or a `char`'s.
	for value in [
		"Flag", "Tagged", "Ordered", "Words", "Scaled", "Lent", "Nested", "Chars",
	] {
		assert!(
			header.contains(&format!("}} opaque_values_{value};")),
			"{value} is no opaque value:\n{header}"
		);
	}
	// Padding after each `u8`; a tag and a `u16` short of a `u32`'s room; a niche the tag
	// may hide in, leaving `B` a byte short; padding after the `u32` on a 64-bit target,
	// though none on wasm32, so a handle on both; a `u64` that `repr(C)` keeps after a `u32`,
	// or after a `usize`, which is a gap on wasm32 alone; a `None` with no niche, writing
	// only its tag, or with a niche narrower than its payload, a `NonZero<u32>`'s beside a
	// `u32`, or a slice's pointer beside its length.
	for handle in [
		"Pairs",
		"Wide",
		"Niched",
		"Word",
		"Mixed",
		"MixedWord",
		"Unset",
		"Narrow",
		"Sliced",
	] {
		assert!(
			header.contains(&format!("void opaque_values_{handle}_free(")),
			"{handle} is no handle:\n{header}"
		);
	}
	let glue = std::fs::read_to_string(package.join("out/src/lib.rs")).unwrap();
	assert!(
		glue.contains("const SIZE: usize = 8;") && !glue.contains("PADDING"),
		"Flag's size is not asserted:\n{glue}"
	);
}

/// A facade whose lib takes the name of a foreign type's crate maps that crate's type
/// through a dependency of its own, apart from the glue's dependency on the facade.
#[test]
fn foreign_lib_name() {
	let package = package(
		"foreign_lib_name",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"foreign_lib_name\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"url\"\n\n\
				[dependencies]\nurl_dep = { package = \"url\", version = \"2\" }\n\n[workspace]\n",
			),
			(
				"src/lib.rs",
				"pub fn echo(url: url_dep::Url) -> url_dep::Url {\n\turl\n}\n",
			),
		],
	);
	export_and_build(&package, &[]);
}

/// A facade's own types keep their names, even where a foreign type's path and role name
/// would spell them: `serde_json::Value` is `Value`, apart from `Json`.
#[test]
fn foreign_role_names() {
	let package = package(
		"foreign_role_names",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"foreign_role_names\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
				[lib]\nname = \"serde_json\"\n\n[workspace]\n",
			),
			(
				"src/lib.rs",
				"pub struct Value {\n\tpub n: u32,\n}\n\npub struct Json {\n\tpub n: u32,\n}\n",
			),
		],
	);
	export_and_build(&package, &[]);
}

/// The glue declares the facade's `rust-version`, raised to the 1.85 its edition needs.
#[test]
fn rust_version() {
	let package = package(
		"rust_version",
		&[(
			"Cargo.toml",
			"[package]\nname = \"rust_version\"\nversion = \"0.0.0\"\nedition = \"2021\"\nrust-version = \"1.70\"\n\n[workspace]\n",
		)],
	);
	let out = package.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", package.join("build")));
	let manifest = std::fs::read_to_string(out.join("Cargo.toml")).unwrap();
	assert!(manifest.contains("rust-version = \"1.85\"\n"), "{manifest}");
}

/// A dependency the facade's dev-dependency enables an optional dependency of, which
/// cargo's resolution holds but neither the check nor the glue builds, moves nothing.
#[test]
fn dev_feature() {
	let package = package("dev_feature", &[]);
	let dep = |name: &str, rest: &str| {
		format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\nedition = \"2024\"\n{rest}")
	};
	write_all(
		&package,
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"dev_feature\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
				[dependencies]\ninner = { path = \"inner\" }\n\n\
				[dev-dependencies]\ninner = { path = \"inner\", features = [\"extra\"] }\n\n\
				[workspace]\nexclude = [\"inner\", \"extra\"]\n",
			),
			(
				"inner/Cargo.toml",
				&dep(
					"inner",
					"\n[dependencies]\nextra = { path = \"../extra\", optional = true }\n\n[features]\nextra = [\"dep:extra\"]\n",
				),
			),
			("inner/src/lib.rs", ""),
			("extra/Cargo.toml", &dep("extra", "")),
			("extra/src/lib.rs", ""),
		],
	);
	export_and_build(&package, &[]);
}

/// A build script that asks the compiler's channel, as `rustversion` does, through cargo's
/// wrapper, gets the channel of the rustc cargo resolved, which the glue builds with, not
/// the nightly the export checks on: a stable rustc exports the stable item alone.
#[test]
fn channel() {
	let package = package(
		"channel",
		&[
			(
				"build.rs",
				"use std::process::Command;\n\n\
				fn main() {\n\
				\tlet rustc = std::env::var_os(\"RUSTC\").unwrap();\n\
				\tlet mut cmd = match std::env::var_os(\"RUSTC_WRAPPER\").filter(|w| !w.is_empty()) {\n\
				\t\tSome(wrapper) => {\n\t\t\tlet mut cmd = Command::new(wrapper);\n\t\t\tcmd.arg(rustc);\n\t\t\tcmd\n\t\t}\n\
				\t\tNone => Command::new(rustc),\n\
				\t};\n\
				\tlet version = cmd.arg(\"--version\").output().unwrap();\n\
				\tassert!(version.status.success());\n\
				\tprintln!(\"cargo::rustc-check-cfg=cfg(nightly)\");\n\
				\tif String::from_utf8(version.stdout).unwrap().contains(\"nightly\") {\n\
				\t\tprintln!(\"cargo::rustc-cfg=nightly\");\n\
				\t}\n\
				}\n",
			),
			(
				"src/lib.rs",
				"#[cfg(nightly)]\npub fn nightly() -> u32 {\n\t1\n}\n\n\
				#[cfg(not(nightly))]\npub fn stable() -> u32 {\n\t2\n}\n",
			),
		],
	);
	let out = package.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("RUSTC", stable_rustc())
		.env("CARGO_TARGET_DIR", package.join("build")));
	let header = std::fs::read_to_string(out.join("include/channel.h")).unwrap();
	assert!(
		header.contains("channel_stable(") && !header.contains("channel_nightly("),
		"{header}"
	);
}

/// A stable rustc from PATH, as the repository's root resolves it: the dev shell puts the
/// root's stable toolchain on PATH behind the exporter's nightly, and rustup picks it from
/// the root's `rust-toolchain.toml` unless the environment names another.
///
/// That root is 1.100 for crabx's binstalk, so in the dev shell these checks prove the glue
/// on 1.100 rather than moq's 1.98.1, until crabx moves to kixel-ai/crabx and the root
/// returns to 1.98.1. Elsewhere, a standalone stable rustc earlier on PATH wins instead.
fn stable_rustc() -> PathBuf {
	let root = manifest_dir().parent().unwrap().to_path_buf();
	let path = std::env::var_os("PATH").unwrap_or_default();
	let rustc = |dir: &Path, arg: &str| {
		let output = Command::new(dir.join(format!("rustc{}", std::env::consts::EXE_SUFFIX)))
			.arg(arg)
			.current_dir(&root)
			.env_remove("RUSTUP_TOOLCHAIN")
			.output()
			.ok()?;
		output
			.status
			.success()
			.then(|| String::from_utf8(output.stdout).unwrap())
	};
	let stable = std::env::split_paths(&path).find(|dir| {
		rustc(dir, "-vV").is_some_and(|v| v.lines().any(|l| l.starts_with("release: ") && !l.contains('-')))
	});
	let stable = stable.expect("no stable rustc on PATH");
	let sysroot = rustc(&stable, "--print=sysroot").unwrap();
	Path::new(sysroot.trim())
		.join("bin")
		.join(format!("rustc{}", std::env::consts::EXE_SUFFIX))
}

/// Each target exports the API it compiles, at its widths, and its crate builds for that
/// target alone: wasm32 has a function the host lacks, lacks one the host has, and a
/// 32-bit `usize`, which Kotlin's 64-bit `ULong` crosses into checked.
#[test]
fn target() {
	let shared = "pub fn halve(n: usize) -> usize {\n\tn / 2\n}\n\n\
		#[cfg(target_arch = \"wasm32\")]\npub fn wasm() -> u32 {\n\t32\n}\n\n\
		#[cfg(not(target_arch = \"wasm32\"))]\npub fn desktop() -> u32 {\n\t64\n}\n";
	// Items whose definitions differ between targets, which one binding cannot serve: a
	// `Copy` type whose array is a `usize` long fills its `repr(C)` layout on a 64-bit
	// target alone, so it is a value there and a handle on wasm32.
	let differing = "pub const MAX: usize = usize::MAX;\n\n\
		#[cfg(target_arch = \"wasm32\")]\npub fn width() -> u32 {\n\t32\n}\n\n\
		#[cfg(not(target_arch = \"wasm32\"))]\npub fn width() -> u64 {\n\t64\n}\n\n\
		#[derive(Clone, Copy)]\n#[repr(C)]\npub struct WordBytes([u8; size_of::<usize>()], u64);\n\n\
		pub fn word_bytes() -> WordBytes {\n\tWordBytes([0; size_of::<usize>()], 2)\n}\n";
	// Each run starts from these, and drops `differing` partway, which rebuilds the glue.
	let package = package("targeted", &[("src/lib.rs", &format!("{shared}\n{differing}"))]);
	let exporter = |args: &[&str]| {
		let mut cmd = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"));
		cmd.arg("--manifest-path")
			.arg(package.join("Cargo.toml"))
			.args(args)
			.env("CARGO_TARGET_DIR", package.join("build"));
		cmd
	};
	let export = |name: &str, args: &[&str]| {
		let out = package.join(name);
		run(exporter(&["--out", out.to_str().unwrap()]).args(args));
		let header = std::fs::read_to_string(out.join("include/targeted.h")).unwrap();
		(out, header)
	};
	let (host_out, header) = export("host", &[]);
	assert!(
		header.contains("targeted_desktop(") && !header.contains("targeted_wasm("),
		"{header}"
	);
	assert!(header.contains(&u64::MAX.to_string()), "{header}");
	// The exports share a target directory, where each export's units are its own though
	// they are the same package at the same path in its workspace: on a second run, with
	// every file's mtime kept, `wasm` would otherwise reuse `merged`'s build script.
	let target = package.join("glue");
	build_offline(&host_out, &target);
	let (wasm, header) = export("wasm", &["--target", WASM]);
	assert!(
		header.contains("targeted_wasm(") && !header.contains("targeted_desktop("),
		"{header}"
	);
	assert!(
		header.contains(&u32::MAX.to_string()) && !header.contains(&u64::MAX.to_string()),
		"{header}"
	);
	let glue = |out: &Path, args: &[&str]| {
		let mut cmd = cargo();
		cmd.args([
			"check",
			"--quiet",
			"--offline",
			"--features",
			"kotlin,dart",
			"--manifest-path",
		])
		.arg(out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(&target)
		.args(args)
		.env("RUSTFLAGS", "-D warnings");
		cmd
	};
	run(&mut glue(&wasm, &["--target", WASM]));
	let output = glue(&wasm, &[]).output().unwrap();
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(!output.status.success(), "wasm32's export built for the host");
	assert!(stderr.contains(&format!("exported for {WASM}, not ")), "{stderr}");

	// One binding cannot serve an item whose definition differs between its targets: each
	// is refused at its span, naming both.
	let merged = package.join("merged");
	// From an earlier run's export.
	let _ = std::fs::remove_dir_all(&merged);
	let output = exporter(&["--out", merged.to_str().unwrap(), "--target", host(), "--target", WASM])
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(!output.status.success(), "differing items exported");
	let (first, second) = match host() < WASM {
		true => (host(), WASM),
		false => (WASM, host()),
	};
	for what in [
		"const `targeted::MAX`",
		"fn `targeted::width`",
		"fn `targeted::word_bytes`",
	] {
		let differs = format!("rust2-ffi: {what} differs between {first} and {second}");
		assert!(stderr.contains(&differs), "{stderr}");
	}
	assert!(!merged.exists(), "a refused export wrote {}", merged.display());

	// Staged on another host, with the checkout elsewhere, an item keeps its span there.
	let staged = package.join("differing.json");
	run(&mut exporter(&["--api", staged.to_str().unwrap(), "--target", WASM]));
	let moved = scratch().join("targeted-moved");
	for file in ["Cargo.toml", "src/lib.rs"] {
		std::fs::create_dir_all(moved.join(file).parent().unwrap()).unwrap();
		std::fs::copy(package.join(file), moved.join(file)).unwrap();
	}
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(moved.join("Cargo.toml"))
		.arg("--out")
		.arg(moved.join("out"))
		.arg("--with")
		.arg(&staged)
		.env("CARGO_TARGET_DIR", moved.join("build"))
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(!output.status.success(), "differing items exported");
	// Each target's span shows its own line of the moved checkout.
	assert!(
		stderr.contains("pub fn width() -> u32") && stderr.contains("pub fn width() -> u64"),
		"{stderr}"
	);

	// Without them, one binding serves both targets, each with only its own items.
	write_all(&package, &[("src/lib.rs", shared)]);
	let (merged, header) = export("merged", &["--target", host(), "--target", WASM]);
	assert!(
		header.contains("targeted_desktop(")
			&& header.contains("targeted_wasm(")
			&& header.contains("targeted_target("),
		"{header}"
	);
	run(&mut glue(&merged, &["--target", WASM]));
	run(&mut glue(&merged, &[]));

	// A target staged on its own host merges into the same binding.
	let staged = package.join("wasm.json");
	run(&mut exporter(&["--api", staged.to_str().unwrap(), "--target", WASM]));
	let (_, joined) = export("joined", &["--with", staged.to_str().unwrap()]);
	assert_eq!(joined, header);

	// A census or a staged API is of one target.
	for output in ["--census", "--api"] {
		let out = package.join("one");
		let output = exporter(&[output, out.to_str().unwrap(), "--target", host(), "--target", WASM])
			.output()
			.unwrap();
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(!output.status.success() && stderr.contains("one --target"), "{stderr}");
	}
}

/// Writes each `(path, text)` into `dir`, leaving a file that already holds its text alone,
/// so cargo finds a package written again fresh.
fn write_all(dir: &Path, files: &[(&str, &str)]) {
	for (path, text) in files {
		step::write(&dir.join(path), text);
	}
}

/// Export the package in `dir` with cargo `args`, then build its glue offline from
/// here, with the lockfile the check resolved.
fn export_and_build(dir: &Path, args: &[&str]) {
	let out = dir.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(dir.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.arg("--")
		.args(args)
		.env("CARGO_TARGET_DIR", dir.join("build"))
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"the export failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	build_offline(&out, &dir.join("glue"));
}

/// Build the export in `out` offline from here, into `target`, once a fetch has brought
/// the registry crates its manifest names, such as `bytes` and `jni-sys`, as any first
/// build would: offline, the build shows it needs none of the config the check read.
fn build_offline(out: &Path, target: &Path) {
	run(cargo()
		.args(["fetch", "--quiet", "--locked", "--manifest-path"])
		.arg(out.join("Cargo.toml")));
	run(cargo()
		.args(["build", "--quiet", "--locked", "--offline", "--manifest-path"])
		.arg(out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(target)
		.env("RUSTFLAGS", "-D warnings"));
}

/// The glue of an API that passes no handle, out-parameter, or callback builds without
/// warnings in every language's module too, writing no import it leaves unused.
#[test]
fn plain() {
	let package = package("plain", &[]);
	export_and_build(&package, &[]);
	run(cargo()
		.args([
			"check",
			"--quiet",
			"--offline",
			"--features",
			"kotlin,python,dart",
			"--manifest-path",
		])
		.arg(package.join("out").join("Cargo.toml"))
		.arg("--target-dir")
		.arg(package.join("glue"))
		.env("RUSTFLAGS", "-D warnings"));
}

/// A facade that reaches `bytes`, whose `Take` its signature names though it re-exports
/// nothing of the crate, builds with the one dependency on it the glue's own `Bytes` shares:
/// cargo refuses two under different names.
#[test]
fn reaches_bytes() {
	let package = package(
		"reaches_bytes",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"reaches_bytes\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
				[dependencies]\nbytes = \"1\"\n\n[workspace]\n",
			),
			(
				"src/lib.rs",
				"pub fn limited(bytes: bytes::Bytes) -> bytes::buf::Take<bytes::Bytes> {\n\tbytes::Buf::take(bytes, 2)\n}\n",
			),
		],
	);
	export_and_build(&package, &["--offline"]);
}

/// A facade whose `bytes` is a `path` dependency that only the host's API reaches: the glue
/// declares `bytes` once, from the facade's source, for every target, never also as the
/// registry's, which cargo refuses as one name from two sources.
#[test]
fn reaches_bytes_on_the_host() {
	// The registry's `bytes`, copied beside the facade to be its `path` dependency.
	let source = package("reaches_bytes_source", &[]);
	write_all(
		&source,
		&[(
			"Cargo.toml",
			"[package]\nname = \"reaches_bytes_source\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
			[dependencies]\nbytes = \"1\"\n\n[workspace]\n",
		)],
	);
	run(cargo()
		.args(["fetch", "--quiet", "--manifest-path"])
		.arg(source.join("Cargo.toml")));
	let metadata = run(cargo()
		.args(["metadata", "--format-version", "1", "--offline", "--manifest-path"])
		.arg(source.join("Cargo.toml")));
	let metadata: serde_json::Value = serde_json::from_slice(&metadata.stdout).unwrap();
	let registry = (metadata["packages"].as_array().unwrap().iter())
		.find(|p| p["name"] == "bytes")
		.map(|p| {
			Path::new(p["manifest_path"].as_str().unwrap())
				.parent()
				.unwrap()
				.to_path_buf()
		})
		.unwrap();
	let package = package(
		"reaches_bytes_host",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"reaches_bytes_host\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
				[dependencies]\nbytes = { path = \"bytes\" }\n\n[workspace]\nexclude = [\"bytes\"]\n",
			),
			(
				"src/lib.rs",
				"#[cfg(not(target_arch = \"wasm32\"))]\npub fn limited(bytes: bytes::Bytes) -> bytes::buf::Take<bytes::Bytes> {\n\tbytes::Buf::take(bytes, 2)\n}\n\n\
				pub fn both() -> u32 {\n\t1\n}\n",
			),
		],
	);
	step::mirror(&registry, &package.join("bytes"));
	let out = package.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(package.join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.args(["--target", WASM, "--target", host(), "--", "--offline"])
		.env("CARGO_TARGET_DIR", package.join("build")));
	let manifest = std::fs::read_to_string(out.join("Cargo.toml")).unwrap();
	let declared: Vec<&str> = manifest.lines().filter(|l| l.starts_with("bytes = ")).collect();
	assert!(
		declared.len() == 1 && declared[0].contains("path"),
		"bytes is declared once, from the facade's source:\n{manifest}"
	);
	build_offline(&out, &package.join("glue"));
}

/// A dependency the facade re-exports from, patched in for a registry dependency, is
/// patched into the glue's build too, though the dependency goes by its library's crate
/// name. So is a dependency of no platform's, which cargo resolves all the same.
#[test]
fn patched() {
	let package = package(
		"patched",
		&[
			(
				"dep/Cargo.toml",
				"[package]\nname = \"rust2-ffi-patched-dep\"\nversion = \"1.0.0\"\nedition = \"2024\"\n\n\
			[lib]\nname = \"rust2_ffi_patched_core\"\n",
			),
			("dep/src/lib.rs", "pub struct Size {\n\tpub width: u32,\n}\n"),
			(
				"inactive/Cargo.toml",
				"[package]\nname = \"rust2-ffi-patched-inactive\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
			),
			("inactive/src/lib.rs", ""),
			(
				"Cargo.toml",
				"[package]\nname = \"patched\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
			[dependencies]\nrust2-ffi-patched-dep = \"1\"\n\n\
			[target.'cfg(any())'.dependencies]\nrust2-ffi-patched-inactive = \"1\"\n\n\
			[patch.crates-io]\nrust2-ffi-patched-dep = { path = \"dep\" }\n\
			rust2-ffi-patched-inactive = { path = \"inactive\" }\n\n\
			[workspace]\nexclude = [\"dep\", \"inactive\"]\n",
			),
			(
				"src/lib.rs",
				"pub use rust2_ffi_patched_core::Size;\n\npub fn width(size: Size) -> u32 {\n\tsize.width\n}\n",
			),
		],
	);
	export_and_build(&package, &["--offline"]);
}

/// A commit of all of `dir`'s files.
const COMMIT: [&str; 8] = [
	"-c",
	"user.name=t",
	"-c",
	"user.email=t@t",
	"commit",
	"--quiet",
	"-am",
	"commit",
];

/// A commit's dates, fixed so a repository made again from the same files has the same
/// commits, whose checkout cargo then reuses.
const GIT_DATES: [(&str, &str); 2] = [
	("GIT_AUTHOR_DATE", "2000-01-01T00:00:00Z"),
	("GIT_COMMITTER_DATE", "2000-01-01T00:00:00Z"),
];

/// The URL of a new git repository in `dir`, whose branch `main` holds `files`.
fn repository(dir: &Path, files: &[(&str, &str)]) -> String {
	let _ = std::fs::remove_dir_all(dir);
	write_all(dir, files);
	for args in [
		&["init", "--quiet", "--initial-branch", "main"][..],
		&["add", "."],
		&COMMIT,
	] {
		run(Command::new("git").args(args).envs(GIT_DATES).current_dir(dir));
	}
	format!("file://{}", dir.display())
}

/// A dependency patched in from a git repository that holds two versions of it, the patch
/// picking one by version, is carried with the version it resolved to.
#[test]
fn git_patched() {
	let dir = scratch().join("git_patched");
	let manifest =
		|version| format!("[package]\nname = \"rust2-ffi-git-dep\"\nversion = \"{version}\"\nedition = \"2024\"\n");
	let stale = repository(
		&dir.join("stale"),
		&[("Cargo.toml", &manifest("1.0.0")), ("src/lib.rs", "")],
	);
	let fresh = repository(
		&dir.join("fresh"),
		&[
			("one/Cargo.toml", &manifest("1.0.0")),
			("one/src/lib.rs", ""),
			("two/Cargo.toml", &manifest("2.0.0")),
			("two/src/lib.rs", ""),
		],
	);
	let text = format!(
		"[package]\nname = \"git_patched\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
		[dependencies]\nrust2-ffi-git-dep = {{ git = \"{stale}\" }}\n\n\
		[patch.\"{stale}\"]\nrust2-ffi-git-dep = {{ git = \"{fresh}\", version = \"1\" }}\n\n\
		[workspace]\nexclude = [\"stale\", \"fresh\"]\n"
	);
	let package = package("git_patched", &[("Cargo.toml", &text)]);
	export_and_build(&package, &[]);
}

/// A dependency the facade re-exports from, which the workspace `[replace]`s by a spec
/// naming no package, is carried as
/// a `[patch]` of it, which resolves alike since a replacement keeps its version. A
/// replacement from the source it replaces is an error, since no patch can carry it.
#[test]
fn replaced() {
	let package = scratch().join("replaced");
	// A git repository the dependency names, whose default branch lacks the field the
	// package reads, named for the package so the spec names it by its URL.
	let manifest = "[package]\nname = \"rust2-ffi-replaced-dep\"\nversion = \"1.0.0\"\nedition = \"2024\"\n";
	let git = package.join("rust2-ffi-replaced-dep");
	let url = repository(
		&git,
		&[
			("Cargo.toml", manifest),
			("src/lib.rs", "pub struct Size {\n\tpub height: u32,\n}\n"),
		],
	);
	run(Command::new("git")
		.args(["checkout", "--quiet", "-b", "fixed"])
		.current_dir(&git));
	write_all(&git, &[("src/lib.rs", "pub struct Size {\n\tpub width: u32,\n}\n")]);
	run(Command::new("git").args(COMMIT).envs(GIT_DATES).current_dir(&git));
	let files = |replacement: &str| {
		write_all(
			&package,
			&[
				("dep/Cargo.toml", manifest),
				("dep/src/lib.rs", "pub struct Size {\n\tpub width: u32,\n}\n"),
				(
					"Cargo.toml",
					&format!(
						"[package]\nname = \"replaced\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
						[dependencies]\nrust2-ffi-replaced-dep = {{ git = \"{url}\", branch = \"main\" }}\n\n\
						[replace]\n\"{url}#1.0.0\" = {replacement}\n\n\
						[workspace]\nexclude = [\"dep\", \"rust2-ffi-replaced-dep\"]\n"
					),
				),
				(
					"src/lib.rs",
					"pub use rust2_ffi_replaced_dep::Size;\n\npub fn width(size: Size) -> u32 {\n\tsize.width\n}\n",
				),
			],
		);
	};
	files("{ path = \"dep\" }");
	export_and_build(&package, &[]);
	files(&format!("{{ git = \"{url}\", branch = \"fixed\" }}"));
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(!output.status.success(), "a replacement from its own source passed");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("rust2-ffi-replaced-dep 1.0.0 is replaced from its own source"),
		"{stderr}"
	);
}

/// Patches in cargo's config, which cargo reads only where the check runs, reach the glue
/// as the check resolved them: an ancestor's file through its `include`, with paths from
/// the directory holding its `.cargo`, outranking the manifest's patch of the same
/// dependency under another spelling of its source, and a `--config` value's `include`,
/// from the package's directory. Both name that source by a registry only the config
/// defines, whose index the environment overrides.
#[test]
fn config_patched() {
	let root = scratch().join("config-patched");
	let write = |path: &str, text: &str| {
		step::write(&root.join(path), text);
	};
	let dep = |dir: &str, name: &str, item: &str| {
		write(
			&format!("{dir}/Cargo.toml"),
			&format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\nedition = \"2024\"\n"),
		);
		write(&format!("{dir}/src/lib.rs"), item);
	};
	dep(
		"pkg/dep",
		"rust2-ffi-config-dep",
		"pub struct Size {\n\tpub width: u32,\n}\n",
	);
	// What the manifest patches in, which lacks the field the package reads.
	dep(
		"pkg/stale",
		"rust2-ffi-config-dep",
		"pub struct Size {\n\tpub height: u32,\n}\n",
	);
	dep(
		"pkg/cli",
		"rust2-ffi-config-cli",
		"pub struct Margin {\n\tpub width: u32,\n}\n",
	);
	write(".cargo/config.toml", "include = [\"patch.toml\"]\n");
	write(
		".cargo/patch.toml",
		"[registries.index]\nindex = \"https://example.com/index\"\n\n\
		[patch.index]\nrust2-ffi-config-dep = { path = \"pkg/dep\" }\n",
	);
	write(
		"pkg/.cargo/cli.toml",
		"[patch.index]\nrust2-ffi-config-cli = { path = \"cli\" }\n",
	);
	write(
		"pkg/Cargo.toml",
		"[package]\nname = \"config-patched\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
		[dependencies]\nrust2-ffi-config-cli = \"1\"\nrust2-ffi-config-dep = \"1\"\n\n\
		[patch.'https://github.com/rust-lang/crates.io-index']\nrust2-ffi-config-dep = { path = \"stale\" }\n\n\
		[workspace]\nexclude = [\"cli\", \"dep\", \"stale\"]\n",
	);
	write(
		"pkg/src/lib.rs",
		"pub use rust2_ffi_config_cli::Margin;\npub use rust2_ffi_config_dep::Size;\n\n\
		pub fn width(size: Size, cli: Margin) -> u32 {\n\tsize.width + cli.width\n}\n",
	);
	let out = root.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--manifest-path")
		.arg(root.join("pkg/Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.args(["--", "--offline", "--config", "include = [\".cargo/cli.toml\"]"])
		.env("CARGO_TARGET_DIR", root.join("build"))
		// What the check resolves `index` by, outranking its config.
		.env(
			"CARGO_REGISTRIES_INDEX_INDEX",
			"https://github.com/rust-lang/crates.io-index",
		)
		.output()
		.unwrap();
	assert!(
		output.status.success(),
		"the export failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	// Built from here, where none of that config is read.
	build_offline(&out, &root.join("glue"));
}

/// A leaf is proven against the profile the export checks, release unless its cargo args pick
/// another, and a build under another makes no leaf call. A method that runs a hook only
/// with debug assertions on is a leaf of a release check; built for dev, Python makes it as
/// any other call, releasing the GIL, which the hook's thread needs. Kept, the call would wait
/// on that thread for good, which the hook fails loud on instead. Every overflow asserts in
/// the check, so arithmetic is no leaf even in release, where a build may still check it. A
/// profile of cargo's config, which the glue does not carry, is an error.
#[test]
fn leaf_profile() {
	let package = package(
		"leaf_profile",
		&[
			(
				"Cargo.toml",
				"[package]\nname = \"leaf_profile\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
				[profile.ship]\ninherits = \"release\"\n\n\
				[profile.bench]\ndebug-assertions = true\n\n[workspace]\n",
			),
			(".cargo/config.toml", "[profile.configured]\ninherits = \"release\"\n"),
			(
				"src/lib.rs",
				"use std::sync::Mutex;\n\n\
				static HOOK: Mutex<Option<Box<dyn Fn() + Send>>> = Mutex::new(None);\n\n\
				pub fn hook(f: impl Fn() + Send + 'static) {\n\t*HOOK.lock().unwrap() = Some(Box::new(f));\n}\n\n\
				fn run_hook() {\n\
				\tlet Some(f) = HOOK.lock().unwrap().take() else {\n\t\treturn;\n\t};\n\
				\tlet (done, ran) = std::sync::mpsc::channel();\n\
				\tstd::thread::spawn(move || {\n\t\tf();\n\t\tlet _ = done.send(f);\n\t});\n\
				\tlet f = ran.recv_timeout(std::time::Duration::from_secs(30)).expect(\"the hook ran: the call kept the GIL\");\n\
				\t*HOOK.lock().unwrap() = Some(f);\n}\n\n\
				pub struct Counter {\n\tn: u64,\n}\n\n\
				impl Counter {\n\
				\tpub fn new(n: u64) -> Self {\n\t\tSelf { n }\n\t}\n\n\
				\tpub fn debugged(&self) -> u64 {\n\t\t#[cfg(debug_assertions)]\n\t\trun_hook();\n\t\tself.n\n\t}\n\n\
				\tpub fn released(&self) -> u64 {\n\t\t#[cfg(not(debug_assertions))]\n\t\trun_hook();\n\t\tself.n\n\t}\n\n\
				\tpub fn next(&self) -> u64 {\n\t\tself.n + 1\n\t}\n}\n",
			),
		],
	);
	let out = package.join("out");
	let export = |cargo: &[&str]| {
		Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--manifest-path")
			.arg(package.join("Cargo.toml"))
			.arg("--out")
			.arg(&out)
			.arg("--")
			.args(cargo)
			.env("CARGO_TARGET_DIR", package.join("build"))
			.output()
			.unwrap()
	};
	// Which of Counter's methods are leaves, and the profile they were proven under: its
	// `PROFILE`, and the directory a build under it alone builds in.
	let exported = |cargo: &[&str]| {
		let output = export(cargo);
		assert!(
			output.status.success(),
			"{cargo:?} failed:\n{}",
			String::from_utf8_lossy(&output.stderr)
		);
		let api: serde_json::Value =
			serde_json::from_str(&std::fs::read_to_string(out.join("api.json")).unwrap()).unwrap();
		let api = &api["api"];
		let methods = api["objects"][0]["methods"].as_array().unwrap();
		let leaves: Vec<&str> = (methods.iter())
			.filter(|m| m["leaf"] == true)
			.map(|m| m["path"].as_array().unwrap().last().unwrap().as_str().unwrap())
			.collect();
		let profile = &api["profiles"][host()];
		(
			leaves.join(" "),
			profile["root"].as_str().unwrap().to_string(),
			profile["dir"].as_str().map(str::to_string),
		)
	};
	let profiled =
		|leaves: &str, root: &str, dir: Option<&str>| (leaves.to_string(), root.to_string(), dir.map(str::to_string));
	// Its build of the Python extension under the profile `dir` names (`test` in `debug`), with `RUSTFLAGS`,
	// cargo's `--config`s, and the environment `with` adds, which says whether its build
	// script found the check's profile, and the Python calling a method with a hook set,
	// which times out if the call kept the GIL.
	let python = |check: &str, dir: &str, rustflags: &str, with: (&[&str], &[(&str, &str)]), name: &str| {
		// Apart for each check, as cargo keeps the library of whichever unit it built last.
		let target = package.join("glue").join(check);
		let (profile, dir) = match dir {
			"debug" => ("dev", "debug"),
			"test" => ("test", "debug"),
			dir => (dir, dir),
		};
		let built = run(cargo()
			.args(["build", "--locked", "--message-format=json", "--manifest-path"])
			.arg(out.join("Cargo.toml"))
			.arg("--target-dir")
			.arg(&target)
			.args(["--profile", profile, "--features", "python"])
			.args(with.0.iter().flat_map(|config| ["--config", config]))
			.envs(with.1.iter().copied())
			.env("RUSTFLAGS", format!("-D warnings {rustflags}")));
		let lib = shared("leaf_profile");
		step::copy(&target.join(dir).join(&lib), &out.join("python").join(&lib));
		let proven = String::from_utf8_lossy(&built.stdout).lines().any(|line| {
			let message: serde_json::Value = serde_json::from_str(line).unwrap();
			message["reason"] == "build-script-executed"
				&& (message["cfgs"].as_array().unwrap().iter())
					.any(|cfg| cfg.as_str().unwrap().starts_with("rust2_ffi_proven"))
		});
		let script = format!(
			"import threading\nimport leaf_profile as m\nran = []\nm.hook(lambda: ran.append(threading.get_ident()))\n\
			assert m.Counter.new(3).{name}() == 3\nassert len(ran) == 1 and ran[0] != threading.get_ident(), ran\n"
		);
		run(
			Command::new(std::env::var_os("PYTHON").unwrap_or_else(|| "python3".into()))
				.args(["-P", "-c", &script])
				.env("PYTHONPATH", out.join("python")),
		);
		proven
	};
	// `bench` sets its own debug assertions on top of `release`, in the directory they
	// share, so a release check proves no build.
	assert_eq!(exported(&[]), profiled("debugged", "release", None));
	// A release flag in a group of short flags picks release too, and the export adds no
	// second one for cargo to refuse.
	assert_eq!(exported(&["-qr"]), profiled("debugged", "release", None));
	assert!(
		!python("release", "debug", "", (&[], &[]), "debugged"),
		"a dev build ran the release check's leaves"
	);
	assert_eq!(
		exported(&["--profile", "dev"]),
		profiled("released", "debug", Some("debug"))
	);
	assert!(
		python("dev", "debug", "", (&[], &[]), "debugged"),
		"a dev build did not run the dev check's leaves"
	);
	// A build whose target cfgs, `DEBUG`, or profile variables are not the check's makes no
	// leaf call, though it builds in the same directory: its build script runs again.
	let api: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(out.join("api.json")).unwrap()).unwrap();
	let debug = api["api"]["profiles"][host()]["debug"].as_bool().unwrap();
	let undebugged = format!("profile.dev.debug={}", !debug);
	for (rustflags, with) in [
		("--cfg rust2_extra --check-cfg cfg(rust2_extra)", (&[][..], &[][..])),
		("", (&[undebugged.as_str()][..], &[][..])),
		(
			"",
			(
				&[][..],
				&[("CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG_ASSERTIONS", "true")][..],
			),
		),
	] {
		assert!(
			!python("dev", "debug", rustflags, with, "debugged"),
			"a build with {rustflags:?} and {with:?} ran the check's leaves"
		);
	}
	// `test` builds in `dev`'s directory, where the check never saw its variables apply.
	assert!(
		!python(
			"dev",
			"test",
			"",
			(&[], &[("CARGO_PROFILE_TEST_BUILD_OVERRIDE_DEBUG_ASSERTIONS", "true")]),
			"debugged"
		),
		"a test build with its own build-override debug assertions ran the dev check's leaves"
	);
	assert!(
		python("dev", "debug", "", (&[], &[]), "debugged"),
		"a dev build did not run the dev check's leaves again"
	);
	// The nightly the export ran lists `cfg`s a stable rustc keeps back, so the check also
	// records them as the nightly lists them acting as a stable release.
	let stable = stable_rustc();
	assert!(
		python(
			"dev-stable",
			"debug",
			"",
			(&[], &[("RUSTC", stable.to_str().unwrap())]),
			"debugged"
		),
		"a stable build did not run the dev check's leaves"
	);
	// Cargo defines `debug` itself, inheriting `dev`'s settings and directory.
	assert_eq!(
		exported(&["--profile", "debug"]),
		profiled("released", "debug", Some("debug"))
	);
	// Debug assertions `RUSTFLAGS` turns off, which the build script may not see, still
	// make no leaf call: `released` runs the hook, and would time out keeping the GIL.
	python(
		"dev-unasserted",
		"debug",
		"-C debug-assertions=off",
		(&[], &[]),
		"released",
	);
	assert_eq!(
		exported(&["--profile", "ship"]),
		profiled("debugged", "release", Some("ship"))
	);
	assert!(
		python("ship", "ship", "", (&[], &[]), "released"),
		"a build under the check's profile did not run its leaves"
	);
	assert!(
		!python("ship", "release", "", (&[], &[]), "released"),
		"a build under another profile of its `PROFILE` ran the check's leaves"
	);
	// The glue's manifest carries no profile cargo's config defines.
	let output = export(&["--profile", "configured"]);
	assert!(!output.status.success(), "a profile of cargo's config passed");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("no profile `configured` inherits from another here"),
		"{stderr}"
	);
}

/// The workspace's profiles reach the glue's build, with an override of a package the
/// library builds, but not one of a package only a test builds; an override naming no
/// package, or `*`, is an error at its line.
#[test]
fn profiled() {
	let package = package("profiled", &[]);
	let files = |spec: &str| {
		write_all(
			&package,
			&[
				(
					"Cargo.toml",
					&format!(
						"[package]\nname = \"profiled\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
						[profile.dev]\nopt-level = 1\n\n\
						[profile.dev.package.\"{spec}\"]\nopt-level = 2\n\n\
						[profile.dev.package.rust2-ffi-profiled-dev]\nopt-level = 3\n\n\
						[dev-dependencies]\nrust2-ffi-profiled-dev = {{ path = \"dev\" }}\n\n\
						[workspace]\nexclude = [\"dev\"]\n"
					),
				),
				("src/lib.rs", "pub fn one() -> u32 {\n\t1\n}\n"),
				(
					"dev/Cargo.toml",
					"[package]\nname = \"rust2-ffi-profiled-dev\"\nversion = \"0.0.0\"\nedition = \"2024\"\n",
				),
				("dev/src/lib.rs", ""),
			],
		);
	};
	files("profiled");
	export_and_build(&package, &[]);
	let manifest = std::fs::read_to_string(package.join("out/Cargo.toml")).unwrap();
	let glue: toml::Table = toml::from_str(&manifest).unwrap();
	assert_eq!(
		glue["profile"]["dev"]["package"]["profiled"]["opt-level"].as_integer(),
		Some(2),
		"{manifest}"
	);
	assert_eq!(
		glue["profile"]["dev"]["package"]["profiled-ffi"]["opt-level"].as_integer(),
		Some(2),
		"{manifest}"
	);
	assert!(!manifest.contains("rust2-ffi-profiled-dev"), "{manifest}");
	files("missing");
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(!output.status.success(), "an override of no package passed");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("Cargo.toml:9: `profile.dev.package.missing` names no package the workspace resolves"),
		"{stderr}"
	);
	files("*");
	let output = census(&package.join("Cargo.toml"), &[]).output().unwrap();
	assert!(!output.status.success(), "a wildcard override passed");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("Cargo.toml:9: `profile.dev.package.\"*\"` would reach"),
		"{stderr}"
	);
}

/// A loop's element is typed among the locals in scope at the loop, not those its body
/// binds over them, as Rust evaluates it before the body runs.
#[test]
fn transcript_loop_scope() {
	let lock = api().out.join("api.json");
	let source = scratch().join("api").join("loop-scope.rs");
	std::fs::write(
		&source,
		"fn main() { let x = 1u8; for n in [x] { match api::divide(7, 2) { \
		 Ok((x, _)) => println!(\"{n} {x}\"), Err(e) => println!(\"{e}\") } } }",
	)
	.unwrap();
	let t = transcript::Transcript::load(&lock, &source);
	let Some(transcript::Step::Match { ok, .. }) = t.steps.last() else {
		panic!("the loop's body matches last")
	};
	let Some(transcript::Step::Print(pieces)) = ok.steps.last() else {
		panic!("the arm prints")
	};
	let names: Vec<&str> = (pieces.iter())
		.filter_map(|p| match p {
			transcript::Piece::Value(
				transcript::Expr {
					kind: transcript::Kind::Var(name),
					..
				},
				_,
			) => Some(name.as_str()),
			_ => None,
		})
		.collect();
	assert_eq!(names, ["x", "x2"]);

	// An element naming the loop's own binding, and a constant naming a later one, resolve
	// as Rust resolves them when an overload is picked.
	std::fs::write(
		&source,
		"const A: u64 = B; const B: u64 = 1; \
		 fn main() { let x = 1u64; for x in [x] { println!(\"{}\", api::same_u64(x)); } println!(\"{}\", api::same_u64(A)); }",
	)
	.unwrap();
	transcript::Transcript::load(&lock, &source);
}

/// What no binding's script could say as Rust does fails to parse, naming its line.
#[test]
fn transcript_refusals() {
	let lock = api().out.join("api.json");
	let source = scratch().join("api").join("refusals.rs");
	// A function `transcript.rs` defines, as each language's hook.
	let hooks = "fn set(n: &mut u32) -> u32 { *n = 1; 1 } fn gap(a: u32, b: u32) -> u32 { b - a }";
	for (body, refusal) in [
		(
			"let mut n = 0u32; println!(\"{}\", api::reach(n, (), set(&mut n)));",
			"one changes n, which another reads",
		),
		(
			"let t = api::Tally::new(); let x: u32 = gap(t.bump(), t.bump()); println!(\"{x}\");",
			"two have an effect",
		),
		(
			"let n = 7u32; api::each(1, |_| { drop(n); });",
			"a drop in a closure of n",
		),
		(
			"let t = api::Token::new(7); for item in [t] { println!(\"inside\"); }",
			"which holds a handle",
		),
		(
			"let mut pair = (api::Score::of(2), 1u8); let n = pair.0.take(); println!(\"{n}\");",
			"a value's &mut self method on anything but a local",
		),
		(
			"let x: Option<Option<Option<u32>>> = None; println!(\"{}\", x.is_none());",
			"an option of an option of an option",
		),
		(
			"let x: Option<Box<Option<Option<u32>>>> = None; println!(\"{}\", x.is_some());",
			"an option of an option of an option",
		),
		(
			"let x: Option<Option<()>> = Some(None); println!(\"{}\", x.is_none());",
			"an option of an option of an option or of ()",
		),
		(
			"for mut n in [7u32] { set(&mut n); println!(\"{n}\"); }",
			"a for loop binding a mutable name",
		),
		("let n = 7i32; println!(\"{n:x}\");", "hex of Int(I32)"),
		(
			"let x: Option<u32> = Some(1); println!(\"{}\", x.map_or(0u32, |_| 7));",
			"a map_or closure's parameter other than a name",
		),
		("{ let x = 1u8; println!(\"{x}\"); }", "a block of its own"),
		(
			"for n in [1u8] { let x = n; println!(\"{x}\"); }",
			"a let in a for loop's body",
		),
		("let api_ = 1u8; println!(\"{api_}\");", "a local named other than"),
		(
			"let n: usize = usize::MAX - 1; println!(\"{n}\");",
			"a pointer-sized value past 32 bits",
		),
		(
			"let mut s = api::Score::of(2); println!(\"{} {}\", s.take(), s.count());",
			"a value's &mut self method",
		),
		(
			"let mut s = api::Score::of(2); println!(\"{} {}\", api::Score::take(&mut s), s.count());",
			"a value's &mut self method",
		),
		(
			"println!(\"{}\", api::same_u64(usize::MAX as u64));",
			"a pointer-sized value past 32 bits",
		),
		(
			"match api::divide(7, 2) { Ok(_) if true => println!(\"a\"), Ok(_) => println!(\"b\"), Err(_) => println!(\"c\") }",
			"an arm other than Ok, Err, Some, or None",
		),
		(
			"match api::divide(7, 2) { Ok(_) => println!(\"a\"), Ok(_) => println!(\"b\"), Err(_) => println!(\"c\") }",
			"a second Ok arm",
		),
		(
			"let mut s = api::Score::of(2); let t = api::Score::of(3); \
			 for n in [s, s] { s.add(t); println!(\"{}\", n.count()); }",
			"an element reading a local its body changes",
		),
		(
			"let t = api::Tally::new(); println!(\"{}\", api::reach(t.bump(), (), t.bump()));",
			"two have an effect",
		),
		(
			"let mut n = 0u32; println!(\"{}\", api::reach(n, (), api::each(1, |_| { api::Clash::listed(vec![7], &mut n); })));",
			"one changes n, which another reads",
		),
		(
			"let mut p = api::Packaged { api: 2 }; \
			 for n in [p, p] { api::Clash::listed(vec![7], &mut p.api); println!(\"{}\", n.api); }",
			"an element reading a local its body changes",
		),
	] {
		std::fs::write(&source, format!("{hooks} fn main() {{ {body} }}")).unwrap();
		let panic = std::panic::catch_unwind(|| transcript::Transcript::load(&lock, &source))
			.err()
			.unwrap_or_else(|| panic!("{body} parsed"));
		let message = panic.downcast_ref::<String>().unwrap();
		assert!(
			message.starts_with("transcript.rs:1:") && message.contains(refusal),
			"{body}: {message}"
		);
	}
}
