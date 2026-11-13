//! The api fixture is exported, built as a static library, and driven from C by
//! `tests/api.c`, from C++ by `tests/api.cpp`, from Kotlin by `tests/api.kt` on the JVM
//! and, shrunk by R8, on Android, from Python by `tests/api.py`, from Swift by
//! `tests/api.swift`, from Go by `tests/api.go`, and from Dart by `tests/api.dart`, whose
//! transcripts must match the fixture's own script run natively.
//! The reject fixture's export must fail at each item it cannot export.

use std::env::consts::EXE_SUFFIX;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::OnceLock;

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

/// The target the api fixture's binding serves besides the host's, which has an item the
/// host lacks and a variant of an enum.
const WASM: &str = "wasm32-unknown-unknown";

fn cargo() -> Command {
	Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
}

fn run(cmd: &mut Command) -> Output {
	let output = cmd.output().unwrap_or_else(|err| panic!("running {cmd:?}: {err}"));
	assert!(
		output.status.success(),
		"{cmd:?} failed:\n{}{}",
		String::from_utf8_lossy(&output.stdout),
		String::from_utf8_lossy(&output.stderr)
	);
	output
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
		.arg("--config")
		.arg(fixture.join("ffi.toml"))
		.arg("--out")
		.arg(out)
		.args(targets.iter().flat_map(|target| ["--target", target]))
		.args(args)
		.arg("--")
		.args(cargo)
		.env("CARGO_TARGET_DIR", scratch().join(name).join("build"));
	command
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

fn api() -> &'static Api {
	static API: OnceLock<Api> = OnceLock::new();
	API.get_or_init(|| {
		let out = scratch().join("api").join("out");
		// A feature of the facade's that enables one in a crate it re-exports, which the C
		// library must build with.
		let features = ["--features", "doubled"];
		// One binding for the host and wasm32, which only the host builds and runs.
		let output = export("api", &out, &[host(), WASM], &features);
		assert!(
			output.status.success(),
			"exporting failed:\n{}",
			String::from_utf8_lossy(&output.stderr)
		);
		let target = scratch().join("api").join("glue");
		let build = |feature: &str| {
			run(cargo()
				.arg("build")
				.arg("--quiet")
				.arg("--locked")
				.arg("--manifest-path")
				.arg(out.join("Cargo.toml"))
				.arg("--target-dir")
				.arg(&target)
				.args(["--features", feature])
				// Glue the exporter writes builds without warnings.
				.env("RUSTFLAGS", "-D warnings --print=native-static-libs"))
		};
		// The CPython extension's symbols would leave the C programs' links undefined, so
		// it builds apart, before the library they link, and moves beside the Python.
		build("python");
		std::fs::rename(
			target.join("debug").join(PYTHON_LIB),
			out.join("python").join(PYTHON_LIB),
		)
		.unwrap();
		// The shared library Kotlin and Dart load.
		let built = build("kotlin,dart");
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

#[test]
fn api_c() {
	let api = api();
	let exe = scratch().join("api").join("api-test");
	let cc = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
	run(Command::new(cc)
		.args(["-std=c11", "-pedantic", "-Wall", "-Wextra", "-Werror", "-I"])
		.arg(api.out.join("include"))
		.arg(manifest_dir().join("tests").join("api.c"))
		.arg(api.staticlib())
		.args(&api.native)
		.arg("-o")
		.arg(&exe));
	let c = run(&mut Command::new(&exe));
	assert_eq!(
		String::from_utf8_lossy(&c.stdout),
		api.transcript,
		"the C transcript differs from the native one"
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
		("socket", "api_ports: an IPv4 socket address has IPv6 bytes or fields"),
		("family", "api_nexts: an IPv6 socket address where IPv4 is taken"),
		("array", "api_vec_u16_free: an array is not as Rust returned it"),
		("future", "api_after: a future's poll is null"),
		("started", "api_after_start: a future's poll is null"),
		("start", "api_yielded_start: a future's handle pointer is null"),
		("lent", "api_Registry_edit: a lent handle was freed"),
		(
			"lent moved",
			"api_Counter_into_label: a lent handle's value cannot move out",
		),
		("null", "api_Registry_edit: a function is null"),
		("returned", "api_made: a returned buffer is not one buf_copy made"),
		("reader", "api_tail: a reader's function is null"),
		("outlived", OUTLIVED),
		(
			"lent shared",
			"api_Meter_reset: a handle a call lent shared is borrowed exclusively",
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
		assert!(!output.status.success(), "{case} did not abort");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "{case} printed {stderr}");
	}
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

/// Build the api fixture's glue, std included, under the `sanitizer`, and run the C and
/// C++ scripts against it, built by clang under the same one.
fn sanitized(sanitizer: &str) {
	let api = api();
	let dir = scratch().join("api").join(sanitizer);
	let built = run(cargo()
		.args(["build", "--quiet", "--locked", "-Zbuild-std", "--target", host()])
		.arg("--manifest-path")
		.arg(api.out.join("Cargo.toml"))
		.arg("--target-dir")
		.arg(dir.join("glue"))
		.env(
			"RUSTFLAGS",
			format!("-D warnings -Zsanitizer={sanitizer} --print=native-static-libs"),
		));
	let native: Vec<String> = String::from_utf8_lossy(&built.stderr)
		.lines()
		.find_map(|line| line.strip_prefix("note: native-static-libs: "))
		.expect("rustc prints the static library's native libraries")
		.split_whitespace()
		.map(str::to_string)
		.collect();
	let lib = dir.join("glue").join(host()).join("debug").join("libapi.a");
	let tests = manifest_dir().join("tests");
	let sanitize = format!("-fsanitize={sanitizer}");
	// Every report fails the run, rather than only those the sanitizer deems fatal.
	let env = [
		("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=1"),
		("TSAN_OPTIONS", "halt_on_error=1"),
	];
	for (compiler, std, src, args) in [
		("clang", "-std=c11", "api.c", &[][..]),
		("clang++", "-std=c++17", "api.cpp", &["futures", "lifetimes"][..]),
	] {
		let exe = dir.join(src.replace('.', "-"));
		run(Command::new(compiler)
			.args([std, "-g", "-Wall", "-Wextra", "-Werror", &sanitize, "-I"])
			.arg(api.out.join("include"))
			.arg(tests.join(src))
			.arg(&lib)
			.args(&native)
			.arg("-o")
			.arg(&exe));
		let output = run(Command::new(&exe).envs(env));
		assert_eq!(
			String::from_utf8_lossy(&output.stdout),
			api.transcript,
			"{src} under {sanitizer} differs from the native transcript"
		);
		for arg in args {
			run(Command::new(&exe).arg(arg).envs(env));
		}
	}
}

/// The glue's `extern "C"` functions driven from Rust under Miri, by `tests/miri/api.rs`
/// mounted as a test module of a copy of the export.
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
	for file in ["rt.rs", "lib.rs"] {
		std::fs::copy(api.out.join("src").join(file), copy.join("src").join(file)).unwrap();
	}
	let mut lib = std::fs::read_to_string(copy.join("src").join("lib.rs")).unwrap();
	lib.push_str(&format!(
		"\n#[cfg(test)]\n#[path = {:?}]\nmod miri;\n",
		manifest_dir().join("tests").join("miri").join("api.rs")
	));
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
	for (standard, plain) in [(17, true), (20, false), (23, true)] {
		let exe = scratch().join("api").join(format!("api-c++{standard}{EXE_SUFFIX}"));
		run(&mut cxx(
			api,
			standard,
			plain,
			&manifest_dir().join("tests").join("api.cpp"),
			&exe,
		));
		let cpp = run(&mut Command::new(&exe));
		assert_eq!(
			String::from_utf8_lossy(&cpp.stdout),
			api.transcript,
			"the C++{standard} transcript differs from the native one"
		);
		run(Command::new(&exe).arg("futures"));
		run(Command::new(&exe).arg("lifetimes"));
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
				"api_Counter_into_label: a lent handle's value cannot move out",
			),
			("lent kept", "a lent object was moved out of"),
			("overread", "api_tail: a reader read more than it was asked"),
			("null view", "a view of null bytes has a length"),
			("outlived", OUTLIVED),
		] {
			let output = Command::new(&exe).arg(case).output().unwrap();
			assert!(!output.status.success(), "C++{standard} {case} did not abort");
			let stderr = String::from_utf8_lossy(&output.stderr);
			assert!(stderr.contains(message), "C++{standard} {case} printed {stderr}");
		}
	}

	if !cfg!(target_env = "msvc") {
		cpp_strict(api);
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
		std::fs::write(&src, format!("#include \"api/{header}\"\nint main() {{}}\n")).unwrap();
		let exe = scratch().join("api").join(format!("alone-{header}{EXE_SUFFIX}"));
		run(&mut cxx(api, 17, true, &src, &exe));
	}

	// A C++ program whose header names another contract than its library's aborts at load.
	let mismatched = scratch().join("api").join("contract.cpp");
	std::fs::write(
		&mismatched,
		"#include \"api.h\"\n#undef API_CONTRACT\n#define API_CONTRACT ((uint64_t)0)\n#include \"api.hpp\"\nint main() {}\n",
	)
	.unwrap();
	let contract = scratch().join("api").join(format!("contract{EXE_SUFFIX}"));
	run(&mut cxx(api, 17, false, &mismatched, &contract));
	let output = Command::new(&contract).output().unwrap();
	assert!(!output.status.success(), "a mismatched contract did not abort");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api/rt.hpp and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// Clang's ownership checks: the whole script builds clean under them, and each case of
/// `tests/api_strict.cpp` fails with its diagnostic.
fn cpp_strict(api: &Api) {
	let clang = |src: &Path, case: u8| {
		let mut cmd = Command::new("clang++");
		cmd.args(["-std=c++17", "-Wconsumed", "-Wdangling", "-Werror", "-c", "-I"])
			.arg(api.out.join("include"))
			.arg(format!("-DCASE={case}"))
			.arg(src)
			.arg("-o")
			.arg(scratch().join("api").join("strict.o"));
		cmd
	};
	let tests = manifest_dir().join("tests");
	run(&mut clang(&tests.join("api.cpp"), 0));
	run(&mut clang(&tests.join("api_strict.cpp"), 0));
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
	] {
		let output = clang(&tests.join("api_strict.cpp"), case).output().unwrap();
		assert!(!output.status.success(), "case {case} built");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "case {case} printed {stderr}");
	}
}

/// A build of the C++ `src` against the api library into `exe`, at C++`standard`, with
/// exceptions and RTTI off if `plain`, and warnings as errors: by `CXX` or the platform's
/// compiler, given MSVC's flags where the target is MSVC's.
fn cxx(api: &Api, standard: u8, plain: bool, src: &Path, exe: &Path) -> Command {
	let include = api.out.join("include");
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
		let mut out = OsString::from("-Fe");
		out.push(exe);
		cmd.arg("-I").arg(include).arg(src).arg(obj).arg(out);
		cmd.arg("-link").arg(api.staticlib()).args(&api.native);
		return cmd;
	}
	let mut cmd = Command::new(std::env::var_os("CXX").unwrap_or_else(|| "c++".into()));
	cmd.arg(format!("-std=c++{standard}"));
	if plain {
		cmd.args(["-fno-exceptions", "-fno-rtti"]);
	}
	// -Wshadow stands in for MSVC's warnings that a name hides another.
	cmd.args(["-pedantic", "-Wall", "-Wextra", "-Wshadow", "-Werror", "-I"])
		.arg(include)
		.arg(src)
		.arg(api.staticlib())
		.args(&api.native)
		.arg("-o")
		.arg(exe);
	cmd
}

/// The generated Kotlin, compiled with `tests/api.kt` against the library's JNI
/// functions, runs its script on the JVM, then refuses a library of another contract.
#[test]
fn api_kotlin() {
	let api = api();
	let classes = scratch().join("api").join("kotlin");
	let _ = std::fs::remove_dir_all(&classes);
	run(&mut kotlinc(&api.out, &classes));
	let mut classpath = classes.clone().into_os_string();
	classpath.push(":");
	classpath.push(coroutines());
	let java = |args: &[&str]| {
		let mut cmd = Command::new("kotlin");
		cmd.arg(format!("-Djava.library.path={}", api.lib.display()))
			.arg("-cp")
			.arg(&classpath)
			.arg("ApiKt")
			.args(args);
		cmd
	};
	kotlin(&api.transcript, &java);

	// The same classes, with another contract than the library's in place of theirs,
	// refuse to load it.
	let native_class = classes.join("api").join("Native.class");
	let bytes = std::fs::read(&native_class).unwrap();
	let contract = kotlin_contract(&api.out.join("kotlin").join("api.kt")).to_be_bytes();
	let at: Vec<usize> = (0..bytes.len() - 8).filter(|&i| bytes[i..i + 8] == contract).collect();
	assert_eq!(at.len(), 1, "Native.class holds the contract once");
	let mut mismatched = bytes.clone();
	mismatched[at[0]] ^= 0xff;
	std::fs::write(&native_class, &mismatched).unwrap();
	let output = java(&[]).output().unwrap();
	std::fs::write(&native_class, &bytes).unwrap();
	assert!(!output.status.success(), "a mismatched contract did not fail");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.kt and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// The same script on Android: the classes, shrunk by R8 into an app's dex with only the
/// generated rules keeping what JNI looks up, run on ART on the device adb reaches against
/// the library the NDK builds for its ABI, both for API 24, the oldest moq's Android library
/// supports.
#[test]
#[ignore = "needs the Android SDK and NDK, cargo-ndk, and a device: just ffi android"]
fn api_android() {
	const DEVICE: &str = "/data/local/tmp/rust2-ffi";
	let api = api();
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
		abi => panic!("the android shell's toolchain has no target for the device's ABI {abi}"),
	};
	let out = dir.join("out");
	let output = export("api", &out, &[target, WASM], &["--features", "doubled"]);
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
		.arg("--target-dir")
		.arg(scratch().join("api").join("glue"))
		.args(["--features", "kotlin"])
		.env("RUSTFLAGS", "-D warnings"));

	// The app: the script's entry point, kept as an app keeps its own, and everything it
	// reaches, which R8 shrinks and renames, keeping only what the rules keep: the
	// generated ones, and those kotlinx-coroutines' jar holds, which an app's build reads
	// from it. Kotlin's annotations are on an app's classpath, not in its dex. The
	// script's functions that hold lent bytes in their own frame are kept too, as R8 would
	// inline them into their caller, whose frame ART's interpreter keeps reaching the bytes.
	let jar = dir.join("app.jar");
	run(kotlinc(&out, &jar).arg("-include-runtime"));
	run(Command::new("jar")
		.arg("xf")
		.arg(coroutines())
		.arg("META-INF/com.android.tools/r8/coroutines.pro")
		.current_dir(&dir));
	let app = dir.join("app.pro");
	std::fs::write(
		&app,
		format!(
			"-keep class ApiKt {{ public static void main(java.lang.String[]); static *** lentAround(); static *** lentToBudget*(...); }}\n-dontwarn org.jetbrains.annotations.**\n-printusage {}\n",
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

	run(Command::new(&adb)
		.args(["shell", "rm", "-rf", DEVICE, "&&", "mkdir", "-p"])
		.arg(format!("{DEVICE}/dalvik-cache")));
	run(Command::new(&adb)
		.arg("push")
		.arg(dex.join("classes.dex"))
		.arg(dir.join("lib").join(&abi).join("libapi.so"))
		.arg(DEVICE));
	let art = |args: &[&str]| {
		let args: String = args.iter().map(|arg| format!(" '{arg}'")).collect();
		let mut cmd = Command::new(&adb);
		cmd.arg("shell").arg(format!(
			"ANDROID_DATA={DEVICE} dalvikvm -Djava.library.path={DEVICE} -cp {DEVICE}/classes.dex ApiKt{args}"
		));
		cmd
	};
	kotlin(&api.transcript, &art);
}

/// kotlinc compiling the generated Kotlin with `tests/api.kt` into `dest`.
fn kotlinc(out: &Path, dest: &Path) -> Command {
	let mut cmd = Command::new("kotlinc");
	cmd.args(["-Werror", "-cp"])
		.arg(coroutines())
		.arg(out.join("kotlin").join("api.kt"))
		.arg(manifest_dir().join("tests").join("api.kt"))
		.arg("-d")
		.arg(dest);
	cmd
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
/// transcript; then its futures, what the GC frees, and each contract it fails loud on.
fn kotlin(native: &str, vm: &dyn Fn(&[&str]) -> Command) {
	let kt = run(&mut vm(&[]));
	assert_eq!(
		String::from_utf8_lossy(&kt.stdout),
		native,
		"the Kotlin transcript differs from the native one"
	);
	run(&mut vm(&["futures"]));
	run(&mut vm(&["owned"]));
	for (case, message) in [
		("moved", "Counter is closed"),
		("boxed moved", "Node is closed"),
		("lent kept", "Counter is closed"),
		(
			"lent moved",
			"api_Counter_into_label: a lent handle's value cannot move out",
		),
		("callback threw", "api::Log::log: a call into Kotlin threw"),
		("closure threw", "api_made: a call into Kotlin threw"),
		("closure failed", "api_drive: a call into Kotlin threw"),
		("reader threw", "SeekableByteChannel.read: a call into Kotlin threw"),
		("outlived", OUTLIVED),
		("stray", "MalformedInputException"),
		("family", "an IPv6 address has no Rust SocketAddrV4"),
	] {
		let output = vm(&[case]).output().unwrap();
		assert!(!output.status.success(), "Kotlin {case} did not fail");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Kotlin {case} printed {stderr}");
	}
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
	let api = api();
	let python = |dir: &Path, args: &[&str]| {
		let mut cmd = Command::new(std::env::var_os("PYTHON").unwrap_or_else(|| "python3".into()));
		// `-P` keeps `tests/` off the path, where `api.py` is this script; dev mode and
		// warnings as errors fail on what the runtime only warns of.
		cmd.args(["-P", "-X", "dev", "-W", "error"])
			.arg(manifest_dir().join("tests").join("api.py"))
			.args(args)
			.env("PYTHONPATH", dir);
		cmd
	};
	let dir = api.out.join("python");
	let py = run(&mut python(&dir, &[]));
	assert_eq!(
		String::from_utf8_lossy(&py.stdout),
		api.transcript,
		"the Python transcript differs from the native one"
	);
	for case in ["futures", "owned", "values", "calls"] {
		run(&mut python(&dir, &[case]));
	}
	for (case, message) in [
		("moved", "ValueError: Counter is closed"),
		("boxed moved", "ValueError: Node is closed"),
		("lent kept", "ValueError: Counter is closed"),
		("lent held", "Counter: a call still holds it when its lend ends"),
		(
			"lent moved",
			"api_Counter_into_label: a lent handle's value cannot move out",
		),
		("callback threw", "api::Log::log: a call into Python raised"),
		("foreign cancelled", "a future Rust polls raised"),
		("outlived", OUTLIVED),
		("family", "ValueError: an IPv6 address has no Rust SocketAddrV4"),
		("kept threw", "api_Spawned_new: a call into Python raised"),
		("returned", "api_made: a call into Python raised"),
		("reader kept", "read: a reader kept a view of its buffer past the read"),
	] {
		let output = python(&dir, &[case]).output().unwrap();
		assert!(!output.status.success(), "Python {case} did not fail");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Python {case} printed {stderr}");
	}

	// The same Python, with another contract than the library's in place of its own,
	// refuses to load it.
	let mismatched = scratch().join("api").join("python-mismatched");
	let _ = std::fs::remove_dir_all(&mismatched);
	std::fs::create_dir_all(&mismatched).unwrap();
	std::fs::copy(dir.join(PYTHON_LIB), mismatched.join(PYTHON_LIB)).unwrap();
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
	std::fs::write(mismatched.join("api.py"), source).unwrap();
	let output = python(&mismatched, &[]).output().unwrap();
	assert!(!output.status.success(), "a mismatched contract did not fail");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.py and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// The api fixture's Python, and `tests/api_types.py` calling it as a typed caller does,
/// pass mypy and pyright, both strict, at the oldest CPython its extension loads in. Apart
/// from `api_python`, which the runners without the dev shell's checkers run alone.
#[test]
fn api_py_types() {
	let dir = scratch().join("api").join("python-types");
	let _ = std::fs::remove_dir_all(&dir);
	std::fs::create_dir_all(&dir).unwrap();
	let module = dir.join("api.py");
	let caller = dir.join("api_types.py");
	std::fs::copy(api().out.join("python").join("api.py"), &module).unwrap();
	std::fs::copy(manifest_dir().join("tests").join("api_types.py"), &caller).unwrap();
	run(Command::new("mypy")
		.args(["--strict", "--python-version", "3.11", "--cache-dir"])
		.arg(scratch().join("mypy"))
		.args([&module, &caller]));
	// Rooted in `dir`, so `import api` finds the module rather than `tests/api.py`.
	let config = dir.join("pyrightconfig.json");
	std::fs::copy(manifest_dir().join("tests").join("pyright.json"), &config).unwrap();
	run(Command::new("pyright").arg("-p").arg(&config).args([&module, &caller]));
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
	let dir = scratch().join("api").join("swift");
	let _ = std::fs::remove_dir_all(&dir);
	std::fs::create_dir_all(&dir).unwrap();
	let module = api.out.join("swift");
	let source = module.join("api.swift");
	let collections = swift_collections();
	run(swiftc(&module, 13)
		.args(["-swift-version", "5", "-typecheck"])
		.arg(&source));
	run(swiftc(&module, 13)
		.args(["-swift-version", "6", "-parse-as-library", "-module-name", "Api"])
		.args(["-emit-module", "-emit-module-path"])
		.arg(dir.join("Api.swiftmodule"))
		.args(["-emit-library", "-static", "-o"])
		.arg(dir.join("libApi.a"))
		.arg(&source));
	// The test's executor preference is macOS 15's.
	let exe = dir.join("api-test");
	let test = manifest_dir().join("tests").join("api.swift");
	run(swiftc(&module, 15)
		.args(["-parse-as-library", "-I"])
		.arg(&dir)
		.arg(&test)
		.arg(dir.join("libApi.a"))
		.args(&collections)
		.arg(api.staticlib())
		.args(&api.native)
		.arg("-o")
		.arg(&exe));
	let swift = run(&mut Command::new(&exe));
	assert_eq!(
		String::from_utf8_lossy(&swift.stdout),
		api.transcript,
		"the Swift transcript differs from the native one"
	);
	run(Command::new(&exe).arg("futures"));
	run(Command::new(&exe).arg("owned"));
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
	] {
		let output = swiftc(&module, 15)
			.args(["-parse-as-library", "-emit-sil", "-o"])
			.arg(dir.join("broken.sil"))
			.args(["-I"])
			.arg(&dir)
			.arg(format!("-D{case}"))
			.arg(&test)
			.output()
			.unwrap();
		assert!(!output.status.success(), "Swift {case} compiled");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Swift {case} printed {stderr}");
	}
	for (case, message) in [
		("family", "api_nexts: an IPv6 socket address where IPv4 is taken"),
		("outlived", OUTLIVED),
		("stray", "a path from Rust is not UTF-8"),
		("lent stray", "a path from Rust is not UTF-8"),
		("refusal moved", "Refusal is closed"),
		("target", "api_wasm_only: it is not on this library's target"),
		("variant", "api_Platform.Wasm: it is not on this library's target"),
		("plain variant", "api_Family.Wasm: it is not on this library's target"),
	] {
		let output = Command::new(&exe).arg(case).output().unwrap();
		assert!(!output.status.success(), "Swift {case} did not fail");
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
	std::fs::create_dir_all(&contract).unwrap();
	std::fs::write(contract.join("api.swift"), mismatched).unwrap();
	std::fs::write(contract.join("main.swift"), "_ = dropped()\n").unwrap();
	let exe = contract.join("contract");
	run(swiftc(&module, 13)
		.args(["-module-name", "Api"])
		.arg(contract.join("api.swift"))
		.arg(contract.join("main.swift"))
		.args(&collections)
		.arg(api.staticlib())
		.args(&api.native)
		.arg("-o")
		.arg(&exe));
	let output = Command::new(&exe).output().unwrap();
	assert!(!output.status.success(), "a mismatched contract did not fail");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.swift and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// The generated Dart, analyzed with no issue, runs `tests/api.dart`'s script, which prints
/// the native transcript, then its futures, what closing and the garbage collector free,
/// and each contract it fails loud on, and a Dart file of another contract fails to load.
#[test]
fn api_dart() {
	let api = api();
	let dir = scratch().join("api").join("dart");
	let _ = std::fs::remove_dir_all(&dir);
	std::fs::create_dir_all(&dir).unwrap();
	let generated = api.out.join("dart");
	run(Command::new("dart").args(["analyze", "--fatal-infos"]).arg(&generated));
	let lib = api.lib.join(PYTHON_LIB);
	let exe = dart_compile(&dir, &generated, &lib, &manifest_dir().join("tests").join("api.dart"));
	let dart = run(&mut dart_run(&exe, &[]));
	assert_eq!(
		String::from_utf8_lossy(&dart.stdout),
		api.transcript,
		"the Dart transcript differs from the native one"
	);
	for case in ["futures", "owned", "closed", "bytes", "bounds"] {
		run(&mut dart_run(&exe, &[case]));
	}
	for (case, message) in [
		("moved", "Bad state: Counter is closed"),
		("boxed moved", "Bad state: Node is closed"),
		("lent kept", "Bad state: Counter is closed"),
		(
			"lent moved",
			"api_Counter_into_label: a lent handle's value cannot move out",
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
		("outlived", OUTLIVED),
		("family", "an IPv6 address has no Rust SocketAddrV4"),
		("stray", "a path from Rust is not UTF-8"),
		("lent stray", "a path from Rust is not UTF-8"),
	] {
		let output = dart_run(&exe, &[case]).output().unwrap();
		assert!(!output.status.success(), "Dart {case} did not fail");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Dart {case} printed {stderr}");
	}

	// The same Dart, with another contract than the library's in place of its own, refuses
	// it at the first call.
	let source = std::fs::read_to_string(generated.join("api.dart")).unwrap();
	let at = source.find("_contract() == 0x").expect("the Dart checks its contract") + "_contract() == 0x".len();
	let mut mismatched = source.clone();
	let digit = if &source[at..at + 1] == "0" { "1" } else { "0" };
	mismatched.replace_range(at..at + 1, digit);
	let contract = dir.join("contract");
	std::fs::create_dir_all(&contract).unwrap();
	std::fs::write(contract.join("api.dart"), mismatched).unwrap();
	let main = contract.join("main.dart");
	std::fs::write(
		&main,
		"import 'package:api/api.dart' as api;\n\nvoid main() => api.dropped();\n",
	)
	.unwrap();
	let exe = dart_compile(&contract, &contract, &lib, &main);
	let output = dart_run(&exe, &[]).output().unwrap();
	assert!(!output.status.success(), "a mismatched contract did not fail");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("api.dart and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
}

/// `main` compiled into a kernel snapshot in `dir`, importing the generated Dart in
/// `package` as `package:api/api.dart`, whose native asset is the shared library `lib`: as
/// a package's build hook would bundle it, which `dart compile` has no flag for.
fn dart_compile(dir: &Path, package: &Path, lib: &Path, main: &Path) -> PathBuf {
	let sdk = std::fs::canonicalize(which("dart"))
		.unwrap()
		.parent()
		.and_then(Path::parent)
		.expect("dart is in the SDK's bin")
		.to_path_buf();
	let config = dir.join("package_config.json");
	let root = url(package);
	std::fs::write(
		&config,
		format!(
			"{{\"configVersion\": 2, \"packages\": [{{\"name\": \"api\", \"rootUri\": \"{root}/\", \"packageUri\": \"./\"}}]}}\n"
		),
	)
	.unwrap();
	let os = match std::env::consts::OS {
		"macos" => "macos",
		"windows" => "windows",
		_ => "linux",
	};
	let arch = match std::env::consts::ARCH {
		"aarch64" => "arm64",
		_ => "x64",
	};
	let assets = dir.join("native_assets.yaml");
	std::fs::write(
		&assets,
		format!(
			"format-version: [1, 0, 0]\nnative-assets:\n  {os}_{arch}:\n    \"package:api/api.dart\": [absolute, {:?}]\n",
			lib.display().to_string()
		),
	)
	.unwrap();
	let dill = dir.join(format!("{}.dill", main.file_stem().unwrap().to_string_lossy()));
	run(
		Command::new(sdk.join("bin").join(format!("dartaotruntime{EXE_SUFFIX}")))
			.arg(sdk.join("bin/snapshots/gen_kernel_aot.dart.snapshot"))
			.arg("--platform")
			.arg(sdk.join("lib/_internal/vm_platform_strong.dill"))
			.arg("--packages")
			.arg(&config)
			.arg("--native-assets")
			.arg(&assets)
			.arg("-o")
			.arg(&dill)
			.arg(main),
	);
	dill
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
fn swift_collections() -> Vec<PathBuf> {
	run(Command::new("just")
		.arg("--justfile")
		.arg(manifest_dir().join("justfile"))
		.arg("swift-collections"));
	let build = collections_dir();
	["BasicContainers", "InternalCollectionsUtilities", "SpanPreview"]
		.iter()
		.map(|module| build.join(format!("lib{module}.a")))
		.collect()
}

fn collections_dir() -> PathBuf {
	manifest_dir().join("target").join("swift-collections").join("build")
}

/// swiftc, with warnings as errors, finding the C header's module in `module` and
/// swift-collections' in [`collections_dir`], at this macOS deployment target on macOS: 13
/// is the oldest the generated `Swift.Duration` allows.
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
		.arg(collections_dir());
	cmd
}

/// The generated Go, vetted and built through cgo against the static library, runs
/// `tests/api.go`'s script, its futures, and what closing and the GC free, then fails loud
/// on each broken contract and on a library of another contract.
#[test]
fn api_go() {
	let api = api();
	// The export's Go, the module of a package per crate that it declares, beside the header
	// as the export lays them out, and the test's module, which requires it.
	let module = scratch().join("api").join("go");
	let _ = std::fs::remove_dir_all(&module);
	let export = module.join("export");
	copy_dir(&api.out.join("go"), &export);
	std::fs::create_dir_all(module.join("include")).unwrap();
	std::fs::copy(
		api.out.join("include").join("api.h"),
		module.join("include").join("api.h"),
	)
	.unwrap();
	std::fs::copy(manifest_dir().join("tests").join("api.go"), module.join("main.go")).unwrap();
	std::fs::write(
		module.join("go.mod"),
		"module apitest\n\ngo 1.24\n\nrequire api v0.0.0\n\nreplace api => ./export\n",
	)
	.unwrap();
	run(&mut go(&export, api, &["vet", "./..."]));
	let exe = module.join(format!("api-test{EXE_SUFFIX}"));
	run(&mut go(&module, api, &["vet", "./..."]));
	run(go(&module, api, &["build", "-o"]).arg(&exe));
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
	for (case, message) in [
		(
			"lent moved",
			"api_Counter_into_label: a lent handle's value cannot move out",
		),
		("nil reader", "a nil io.ReadSeeker"),
		("returned closed", "Instant is closed"),
		("family", "an address that is not IPv4 has no Rust SocketAddrV4"),
		("outlived", OUTLIVED),
		("text", "api_greet: a string is not UTF-8"),
		("callback panicked", "a log failed"),
		("duration", "a Rust Duration past time.Duration's longest"),
		("target", "api_wasm_only: it is not on this library's target"),
		("variant", "api_Platform.Wasm: it is not on this library's target"),
		("plain variant", "api_Family.Wasm: it is not on this library's target"),
	] {
		let output = Command::new(&exe).arg(case).output().unwrap();
		assert!(!output.status.success(), "Go {case} did not fail");
		let stderr = String::from_utf8_lossy(&output.stderr);
		assert!(stderr.contains(message), "Go {case} printed {stderr}");
	}

	// The same packages, with another contract than the library's in place of their own,
	// refuse it when they load.
	let runtime = export.join("rt").join("rt.go");
	let generated = std::fs::read_to_string(&runtime).unwrap();
	let at = generated
		.find("api_contract() != 0x")
		.expect("the Go checks its contract")
		+ "api_contract() != 0x".len();
	let mut mismatched = generated.clone();
	let digit = if &generated[at..at + 1] == "0" { "1" } else { "0" };
	mismatched.replace_range(at..at + 1, digit);
	std::fs::write(&runtime, mismatched).unwrap();
	std::fs::write(
		module.join("main.go"),
		"package main\n\nimport \"api/api\"\n\nfunc main() {\n\tapi.Dropped()\n}\n",
	)
	.unwrap();
	run(go(&module, api, &["build", "-o"]).arg(&exe));
	let output = Command::new(&exe).output().unwrap();
	assert!(!output.status.success(), "a mismatched contract did not fail");
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("the Go of api and the library are from different builds"),
		"a mismatched contract printed {stderr}"
	);
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
}

/// A crate named as the runtime, a Go keyword, or `main` is the Go package of that name and
/// a trailing `_`, which Go vets.
#[test]
fn go_reserved_packages() {
	for name in ["rt", "func", "main"] {
		let package = package(&format!("go_reserved_{name}"));
		let manifest = format!(
			"[package]\nname = \"go_reserved_{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"{name}\"\n\n[workspace]\n"
		);
		std::fs::write(package.join("Cargo.toml"), manifest).unwrap();
		let out = package.join("out");
		let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--config")
			.arg(package.join("ffi.toml"))
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
	let package = package("go_package_names");
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
		.arg("--config")
		.arg(package.join("ffi.toml"))
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

/// `go vet` over an export's `go/`, which the header beside it lets cgo check unlinked.
fn vet(go: &Path) {
	run(Command::new("go")
		.args(["vet", "./..."])
		.current_dir(go)
		.env("CGO_ENABLED", "1")
		.env("GOFLAGS", "-buildvcs=false")
		.env("GOTOOLCHAIN", "local"));
}

/// A copy of the directory `from` at `to`.
fn copy_dir(from: &Path, to: &Path) {
	std::fs::create_dir_all(to).unwrap();
	for entry in std::fs::read_dir(from).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		match entry.file_type().unwrap().is_dir() {
			true => copy_dir(&path, &to.join(entry.file_name())),
			false => {
				std::fs::copy(&path, to.join(entry.file_name())).unwrap();
			}
		}
	}
}

/// The go command in `module`, linking cgo against the api fixture's static library.
fn go(module: &Path, api: &Api, args: &[&str]) -> Command {
	let mut ldflags = api.staticlib().into_os_string();
	for lib in &api.native {
		ldflags.push(" ");
		ldflags.push(lib);
	}
	let mut cmd = Command::new("go");
	cmd.args(args)
		.current_dir(module)
		.env("CGO_ENABLED", "1")
		.env("CGO_LDFLAGS", ldflags)
		// The module is no checkout of its own, and must build with the Go on PATH.
		.env("GOFLAGS", "-buildvcs=false")
		.env("GOTOOLCHAIN", "local");
	cmd
}

/// A relative `CARGO_TARGET_DIR` is the caller's, wherever cargo runs.
#[test]
fn relative_target() {
	let dir = scratch().join("relative");
	std::fs::create_dir_all(&dir).unwrap();
	let census = dir.join("census");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(manifest_dir().join("fixtures/api/ffi.toml"))
		.arg("--census")
		.arg(&census)
		.current_dir(&dir)
		.env("CARGO_TARGET_DIR", "build")
		.output()
		.expect("running rust2-ffi");
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let text = std::fs::read_to_string(census.join("api-fixture.md")).unwrap();
	for line in [
		"- `Lease`: object no export returns",
		"- `Void`: object no export returns",
		"- `release`: type C never holds `Lease`",
		"- `Unnamed`: generic type no export instantiates",
	] {
		assert!(text.contains(line), "the census lists no `{line}`:\n{text}");
	}
	assert!(dir.join("build").exists());
}

#[test]
fn reject() {
	let out = scratch().join("reject").join("out");
	let output = export("reject", &out, &[], &[]);
	assert!(!output.status.success(), "exporting reject succeeded");
	let errors = errors(&output);
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

/// Each of rust2-ffi's errors the export printed, sorted: at its span, then each of its
/// notes at its own.
fn errors(output: &Output) -> Vec<String> {
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
	let names = locked("names");
	assert!(!names.status.success(), "binding renames passed the lock");
	let stderr = String::from_utf8_lossy(&names.stderr);
	for language in ["Kotlin", "Swift", "Go", "Dart", "Python"] {
		assert!(
			stderr.contains(&format!(
				"its {language} name `GraphicsEncoderFinish` is now `GraphicsEncoder_Finish`"
			)),
			"missing class rename for {language}:\n{stderr}"
		);
	}
	for (language, old, new) in [
		("Kotlin", "smoothStep", "smoothStep_"),
		("Swift", "smoothStep", "smoothStep_"),
		("Dart", "smoothStep", "smoothStep_"),
		("Python", "smooth_step", "smooth_step_"),
		("Go", "SmoothStep", "SmoothStep_"),
	] {
		assert!(
			stderr.contains(&format!("its {language} name `{old}` is now `{new}`")),
			"missing member rename for {language}:\n{stderr}"
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
		"its Dart name `StringException` is now `StringException_`",
		"its Dart name `ShapeCircle` is now `ShapeCircle_`",
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
	for language in ["Kotlin", "Python", "Dart"] {
		assert!(
			stderr.contains(&format!(
				"exception class for `string`: its {language} name `StringException` is now `StringException_`"
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

/// A one-function package named `name` in its own workspace, written into scratch.
fn package(name: &str) -> PathBuf {
	let package = scratch().join(name);
	std::fs::create_dir_all(package.join("src")).unwrap();
	let manifest = format!("[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n");
	std::fs::write(package.join("Cargo.toml"), manifest).unwrap();
	std::fs::write(package.join("src/lib.rs"), "pub fn one() -> u32 {\n\t1\n}\n").unwrap();
	std::fs::write(package.join("ffi.toml"), "").unwrap();
	package
}

/// rustc's arguments in an argfile, as cargo passes them once they overflow a command line:
/// on Windows, moq-ffi's many `-L`s do. One argument past every platform's limit forces it,
/// and starts with `@`, which inside an argfile rustc reads as itself, not another file.
#[test]
fn argfile() {
	let package = package("argfile");
	let value = format!("@{}=/", "a".repeat(1 << 21));
	write_all(
		&package,
		&[(
			".cargo/config.toml",
			&format!("build.rustflags = [\"--remap-path-prefix\", \"{value}\"]\n"),
		)],
	);
	let output = census(&package.join("ffi.toml"), &[]).output().unwrap();
	assert!(
		output.status.success(),
		"the census failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
}

/// A census of the package whose config is `config` with these cargo args, to run.
fn census(config: &Path, cargo: &[&str]) -> Command {
	let dir = config.parent().unwrap();
	let mut cmd = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"));
	cmd.arg("--config")
		.arg(config)
		.arg("--census")
		.arg(dir.join("census"))
		.arg("--")
		.args(cargo)
		.env("CARGO_TARGET_DIR", dir.join("build"));
	cmd
}

/// Finding the exports keeps to the check's `--locked`: it neither writes a lockfile the
/// check then accepts, nor passes.
#[test]
fn locked() {
	let package = package("locked");
	let _ = std::fs::remove_file(package.join("Cargo.lock"));
	let output = census(&package.join("ffi.toml"), &["--locked"]).output().unwrap();
	assert!(!output.status.success(), "a census without a lockfile passed --locked");
	assert!(!package.join("Cargo.lock").exists(), "the lockfile was written");
}

/// A package reached through a symlinked directory is the same package, though cargo
/// keeps the link in its paths. On Unix: making a symlink on Windows takes a privilege.
#[cfg(unix)]
#[test]
fn symlinked() {
	let package = package("symlinked");
	let link = scratch().join("symlinked-link");
	if !link.exists() {
		std::os::unix::fs::symlink(&package, &link).unwrap();
	}
	let output = census(&link.join("ffi.toml"), &[]).output().unwrap();
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
	let package = package("lookalike");
	let write = |path: &str, text: &str| {
		let path = package.join(path);
		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		std::fs::write(path, text).unwrap();
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
	write(
		"Cargo.toml",
		"[package]\nname = \"lookalike\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"shared\"\n\n\
		[dependencies]\nshared = { package = \"lookalike-exp\", path = \"exp\" }\n\
		bytes = { package = \"lookalike-other\", path = \"other\" }\n\n\
		[workspace]\nexclude = [\"exp\", \"other\"]\n",
	);
	write(
		"src/lib.rs",
		"pub mod exp {\n\tpub use shared::Size;\n}\n\npub mod other {\n\tpub use bytes::Size;\n}\n\n\
		pub fn total(a: shared::Size, b: bytes::Size) -> u32 {\n\ta.width + b.width\n}\n",
	);
	let out = package.join("out");
	let remap = format!("--remap-path-prefix={}=/src", package.display());
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(package.join("ffi.toml"))
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
		let package = package(&format!("standard_{name}"));
		let manifest = format!(
			"[package]\nname = \"standard_{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\nname = \"{name}\"\n\n[workspace]\n"
		);
		std::fs::write(package.join("Cargo.toml"), manifest).unwrap();
		let lib = "pub struct SocketAddrV4 {\n\tpub port: u16,\n}\n\npub fn port(addr: SocketAddrV4) -> u16 {\n\taddr.port\n}\n";
		std::fs::write(package.join("src/lib.rs"), lib).unwrap();
		let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
			.arg("--config")
			.arg(package.join("ffi.toml"))
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

/// A facade whose lib takes the name of a foreign type's crate maps that crate's type
/// through a dependency of its own, apart from the glue's dependency on the facade.
#[test]
fn foreign_lib_name() {
	let package = package("foreign_lib_name");
	write_all(
		&package,
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
	let package = package("foreign_role_names");
	write_all(
		&package,
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

/// An exclude may name what a `cfg` leaves out of this build, whether the facade's own
/// module or another crate's that the facade re-exports through a glob, or a chain of them,
/// or a field or variant of the facade's or another crate's type, under a `cfg` or one a
/// `cfg_attr` adds, an inline module's type and a const-generic one's included, but not
/// what no build has, nor what a module only some of whose items the facade re-exports
/// leaves out.
#[test]
fn configured_out() {
	let package = package("configured_out");
	let dep = manifest_dir().join("fixtures/dep");
	let manifest = format!(
		"[package]\nname = \"configured_out\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
		[dependencies]\ndep = {{ package = \"dep-fixture\", path = {:?} }}\n\n[workspace]\n",
		dep.display().to_string()
	);
	write_all(
		&package,
		&[
			("Cargo.toml", &manifest),
			(
				"src/lib.rs",
				"pub use ::dep::Size;\n\npub mod dep {\n\tpub use ::dep::*;\n}\n\n#[cfg(any())]\npub mod local {}\n\n\
				mod inner {\n\tpub fn kept() -> u32 {\n\t\t1\n\t}\n\n\t#[cfg(any())]\n\tpub mod gone {}\n}\n\n\
				mod middle {\n\tpub use crate::inner::*;\n}\n\npub mod outer {\n\tpub use crate::middle::*;\n}\n\n\
				pub struct Shape<const N: usize = { 1 }> {\n\tpub n: u32,\n\t#[cfg(any())]\n\tpub gone: u32,\n\t#[cfg_attr(all(), cfg_attr(all(), cfg(any())))]\n\tpub gated: u32,\n}\n\n\
				pub enum Kind {\n\tKept,\n\t#[cfg(any())]\n\tGone,\n}\n\n\
				pub mod nested {\n\tpub struct Inner {\n\t\tpub n: u32,\n\t\t#[cfg(any())]\n\t\tpub gone: u32,\n\t}\n}\n",
			),
			(
				"ffi.toml",
				"exclude = [\"Gone\", \"Kind::Gone\", \"dep::Area::scaled\", \"Shape::gated\", \"Shape::gone\", \"Shape::missing\", \"Size::depth\", \"nested::Inner::gone\", \"dep::gone::Gone\", \"gone::Gone\", \"local::Gone\", \"outer::gone::Gone\"]\n",
			),
		],
	);
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(package.join("ffi.toml"))
		.arg("--out")
		.arg(package.join("out"))
		.env("CARGO_TARGET_DIR", package.join("build"))
		.output()
		.unwrap();
	let stderr = String::from_utf8_lossy(&output.stderr);
	let errors: Vec<&str> = stderr.lines().filter(|l| l.starts_with("error: rust2-ffi:")).collect();
	assert_eq!(
		errors,
		[
			"error: rust2-ffi: exclude `Gone` in the config names nothing exported",
			"error: rust2-ffi: exclude `Shape::missing` in the config names nothing exported",
			"error: rust2-ffi: exclude `gone::Gone` in the config names nothing exported",
		],
		"{stderr}"
	);
}

/// The glue declares the facade's `rust-version`, raised to the 1.85 its edition needs.
#[test]
fn rust_version() {
	let package = package("rust_version");
	write_all(
		&package,
		&[(
			"Cargo.toml",
			"[package]\nname = \"rust_version\"\nversion = \"0.0.0\"\nedition = \"2021\"\nrust-version = \"1.70\"\n\n[workspace]\n",
		)],
	);
	let out = package.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(package.join("ffi.toml"))
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
	let package = package("dev_feature");
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
	let package = package("channel");
	write_all(
		&package,
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
		.arg("--config")
		.arg(package.join("ffi.toml"))
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
	let package = package("targeted");
	let shared = "pub fn halve(n: usize) -> usize {\n\tn / 2\n}\n\n\
		#[cfg(target_arch = \"wasm32\")]\npub fn wasm() -> u32 {\n\t32\n}\n\n\
		#[cfg(not(target_arch = \"wasm32\"))]\npub fn desktop() -> u32 {\n\t64\n}\n";
	// Items whose definitions differ between targets, which one binding cannot serve.
	let differing = "pub const MAX: usize = usize::MAX;\n\n\
		#[cfg(target_arch = \"wasm32\")]\npub fn width() -> u32 {\n\t32\n}\n\n\
		#[cfg(not(target_arch = \"wasm32\"))]\npub fn width() -> u64 {\n\t64\n}\n";
	write_all(&package, &[("src/lib.rs", &format!("{shared}\n{differing}"))]);
	let exporter = |args: &[&str]| {
		let mut cmd = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"));
		cmd.arg("--config")
			.arg(package.join("ffi.toml"))
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
	build_offline(&host_out, &package.join("glue"));
	let (wasm, header) = export("wasm", &["--target", WASM]);
	assert!(
		header.contains("targeted_wasm(") && !header.contains("targeted_desktop("),
		"{header}"
	);
	assert!(
		header.contains(&u32::MAX.to_string()) && !header.contains(&u64::MAX.to_string()),
		"{header}"
	);
	let glue = |out: &Path, target: &[&str]| {
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
		.arg(package.join("glue"))
		.args(target)
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
	for what in ["const `targeted::MAX`", "fn `targeted::width`"] {
		let differs = format!("rust2-ffi: {what} differs between {first} and {second}");
		assert!(stderr.contains(&differs), "{stderr}");
	}
	assert!(!merged.exists(), "a refused export wrote {}", merged.display());

	// Staged on another host, with the checkout elsewhere, an item keeps its span there.
	let staged = package.join("differing.json");
	run(&mut exporter(&["--api", staged.to_str().unwrap(), "--target", WASM]));
	let moved = scratch().join("targeted-moved");
	for file in ["Cargo.toml", "ffi.toml", "src/lib.rs"] {
		std::fs::create_dir_all(moved.join(file).parent().unwrap()).unwrap();
		std::fs::copy(package.join(file), moved.join(file)).unwrap();
	}
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(moved.join("ffi.toml"))
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

/// Writes each `(path, text)` into `dir`.
fn write_all(dir: &Path, files: &[(&str, &str)]) {
	for (path, text) in files {
		let path = dir.join(path);
		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		std::fs::write(path, text).unwrap();
	}
}

/// Export the package in `dir` with cargo `args`, then build its glue offline from
/// here, with the lockfile the check resolved.
fn export_and_build(dir: &Path, args: &[&str]) {
	let out = dir.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(dir.join("ffi.toml"))
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
	let package = package("plain");
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

/// A dependency the facade re-exports from, patched in for a registry dependency, is
/// patched into the glue's build too, though the dependency goes by its library's crate
/// name. So is a dependency of no platform's, which cargo resolves all the same.
#[test]
fn patched() {
	let package = package("patched");
	write_all(
		&package,
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

/// The URL of a new git repository in `dir`, whose branch `main` holds `files`.
fn repository(dir: &Path, files: &[(&str, &str)]) -> String {
	let _ = std::fs::remove_dir_all(dir);
	write_all(dir, files);
	for args in [
		&["init", "--quiet", "--initial-branch", "main"][..],
		&["add", "."],
		&COMMIT,
	] {
		run(Command::new("git").args(args).current_dir(dir));
	}
	format!("file://{}", dir.display())
}

/// A dependency patched in from a git repository that holds two versions of it, the patch
/// picking one by version, is carried with the version it resolved to.
#[test]
fn git_patched() {
	let package = package("git_patched");
	let manifest =
		|version| format!("[package]\nname = \"rust2-ffi-git-dep\"\nversion = \"{version}\"\nedition = \"2024\"\n");
	let stale = repository(
		&package.join("stale"),
		&[("Cargo.toml", &manifest("1.0.0")), ("src/lib.rs", "")],
	);
	let fresh = repository(
		&package.join("fresh"),
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
	write_all(&package, &[("Cargo.toml", &text)]);
	export_and_build(&package, &[]);
}

/// A dependency the facade re-exports from, which the workspace `[replace]`s by a spec
/// naming no package, is carried as
/// a `[patch]` of it, which resolves alike since a replacement keeps its version. A
/// replacement from the source it replaces is an error, since no patch can carry it.
#[test]
fn replaced() {
	let package = package("replaced");
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
	run(Command::new("git").args(COMMIT).current_dir(&git));
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
				("ffi.toml", ""),
			],
		);
	};
	files("{ path = \"dep\" }");
	export_and_build(&package, &[]);
	files(&format!("{{ git = \"{url}\", branch = \"fixed\" }}"));
	let output = census(&package.join("ffi.toml"), &[]).output().unwrap();
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
		let path = root.join(path);
		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		std::fs::write(path, text).unwrap();
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
	write("pkg/ffi.toml", "");
	let out = root.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(root.join("pkg/ffi.toml"))
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
