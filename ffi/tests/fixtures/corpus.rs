//! A pinned corpus of popular crates.io crates, `corpus/crates.txt`, each exported through
//! a facade of its own, `pub use <crate>::*;`, as a consumer would export it. Each is
//! censused, then, if the census refuses nothing, since a refusal fails the export, exported,
//! and its glue and every language's binding of it built. A panic, an error with or without a
//! span, or a binding that does not build fails the run; a refusal does not, since the census
//! lists it.
//!
//! The facades are one workspace, resolved by `corpus/Cargo.lock`, so a run builds what the
//! last update resolved. `just ffi corpus-update` takes the top crates by all-time
//! downloads afresh. Building them runs their build scripts: run it where that is safe, as
//! the nightly's runner is.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use super::{cargo, manifest_dir, run, scratch, step};

/// The crates a census, an export, or a binding of fails today, each with the quest that
/// fixes it, or else why none can. They are not run; the quest's PR deletes its entry.
const KNOWN: &[(&str, &str)] = &[
	("anyhow", "/quest/m1/ffi-apart.md"),
	("axum", "/quest/m1/ffi-corpus-bugs.md"),
	("bytes", "/quest/m1/ffi-corpus-bugs.md"),
	(
		"core-foundation",
		"macOS's CoreFoundation: calls a platform library this host lacks, so no binding loads",
	),
	("crossbeam-utils", "/quest/m1/ffi-apart.md"),
	("darling", "/quest/m1/ffi-apart.md"),
	("der", "/quest/m1/ffi-apart.md"),
	("env_logger", "/quest/m1/ffi-apart.md"),
	("errno", "/quest/m1/ffi-corpus-bugs.md"),
	("hermit-abi", "/quest/m1/ffi-apart.md"),
	("hyper", "/quest/m1/ffi-corpus-bugs.md"),
	("icu_locale_core", "/quest/m1/ffi-apart.md"),
	("itertools", "/quest/m1/ffi-apart.md"),
	("js-sys", "/quest/m1/ffi-inference.md"),
	("libc", "/quest/m1/ffi-apart.md"),
	("log", "/quest/m1/ffi-apart.md"),
	("nu-ansi-term", "/quest/m1/ffi-apart.md"),
	("object", "/quest/m1/ffi-apart.md"),
	("parking_lot", "/quest/m1/ffi-apart.md"),
	("petgraph", "/quest/m1/ffi-apart.md"),
	("prettyplease", "/quest/m1/ffi-apart.md"),
	("redox_syscall", "/quest/m1/ffi-apart.md"),
	("untrusted", "/quest/m1/ffi-apart.md"),
	("wasm-bindgen", "/quest/m1/ffi-apart.md"),
	(
		"windows-core",
		"Windows's API: calls a platform library this host lacks, so no binding loads",
	),
];

/// How many crates the update keeps, by all-time downloads, before dropping those that are
/// no library or do not build as they are.
const TOP: usize = 300;

/// A crate of the corpus.
struct Krate {
	name: String,
	version: String,
}

/// `corpus/crates.txt`: a crate and its version on each line, after `#` comments.
fn list() -> Vec<Krate> {
	let text = std::fs::read_to_string(corpus_dir().join("crates.txt")).unwrap();
	(text.lines())
		.filter(|line| !line.is_empty() && !line.starts_with('#'))
		.map(|line| {
			let (name, version) = line.split_once(' ').expect("a line is a crate and its version");
			Krate {
				name: name.to_string(),
				version: version.to_string(),
			}
		})
		.collect()
}

fn corpus_dir() -> PathBuf {
	manifest_dir().join("corpus")
}

/// Where a run builds: a shard's own, so shards run side by side.
fn base() -> PathBuf {
	match std::env::var("CORPUS_SHARD") {
		Ok(shard) => scratch().join(format!("corpus-{}", shard.replace('/', "-of-"))),
		Err(_) => scratch().join("corpus"),
	}
}

/// The workspace of facades.
fn workspace() -> PathBuf {
	base().join("facades")
}

/// Write the workspace of a facade for each of `crates`, each `lib.rs` re-exporting the
/// library `libs` names, or nothing until it is known.
fn write_workspace(crates: &[Krate], libs: &BTreeMap<String, String>) {
	let ws = workspace();
	let members: Vec<String> = crates.iter().map(|k| format!("\t{:?},", k.name)).collect();
	step::write(
		&ws.join("Cargo.toml"),
		format!(
			"[workspace]\nresolver = \"3\"\nmembers = [\n{}\n]\n",
			members.join("\n")
		),
	);
	for krate in crates {
		let dir = ws.join(&krate.name);
		step::write(
			&dir.join("Cargo.toml"),
			format!(
				"[package]\nname = \"corpus-{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\n{name} = \"={version}\"\n",
				name = krate.name,
				version = krate.version
			),
		);
		let lib = match libs.get(&krate.name) {
			Some(lib) => format!(
				"#[allow(unused_imports, reason = \"a library may be empty on this host\")]\npub use {lib}::*;\n"
			),
			None => String::new(),
		};
		step::write(&dir.join("src").join("lib.rs"), lib);
	}
}

/// What `cargo metadata` says of the workspace's dependencies: each registry package's
/// name and version, with its library's name if it has one that is no proc macro.
fn libraries() -> BTreeMap<(String, String), Option<String>> {
	let output = run(cargo()
		.args(["metadata", "--format-version", "1", "--locked", "--offline"])
		.current_dir(workspace()));
	let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
	let mut libs = BTreeMap::new();
	for package in metadata["packages"].as_array().unwrap() {
		if package["source"].is_null() {
			continue;
		}
		let lib = (package["targets"].as_array().unwrap().iter())
			.find(|target| target["kind"].as_array().unwrap().iter().any(|k| k == "lib"))
			.map(|target| target["name"].as_str().unwrap().replace('-', "_"));
		// Keyed without build metadata, as the list writes a version.
		let version = package["version"].as_str().unwrap();
		let key = (
			package["name"].as_str().unwrap().to_string(),
			version.split('+').next().unwrap().to_string(),
		);
		libs.insert(key, lib);
	}
	libs
}

/// Each crate's library name, from the workspace its lockfile resolves, fetched first.
fn resolve(crates: &[Krate]) -> BTreeMap<String, String> {
	run(cargo().args(["fetch", "--locked"]).current_dir(workspace()));
	let libraries = libraries();
	(crates.iter())
		.map(|k| {
			let lib = libraries.get(&(k.name.clone(), k.version.clone()));
			let lib = lib
				.cloned()
				.flatten()
				.unwrap_or_else(|| panic!("{} {} has no library", k.name, k.version));
			(k.name.clone(), lib)
		})
		.collect()
}

/// Where one stage of a crate failed, and what it printed.
struct Failure {
	krate: String,
	stage: String,
	output: String,
}

/// What a crate's run reports when it passes: its census's counts.
struct Passed {
	krate: String,
	refused: usize,
}

/// Census and export every crate of the corpus, or of `CORPUS_SHARD`'s `i/n`, the `i`th of
/// `n` slices, or those `CORPUS_CRATES` names, and build each binding, writing the report
/// to `target/tmp/corpus/report.md`, or a shard's to `target/tmp/corpus-<i>-of-<n>/report.md`.
#[test]
#[ignore = "the nightly's: every crate of the corpus, just ffi corpus"]
fn nightly() {
	let crates = list();
	write_workspace(&crates, &BTreeMap::new());
	step::copy(&corpus_dir().join("Cargo.lock"), &workspace().join("Cargo.lock"));
	let libs = resolve(&crates);
	write_workspace(&crates, &libs);
	let mut chosen: Vec<&Krate> = crates.iter().collect();
	if let Ok(shard) = std::env::var("CORPUS_SHARD") {
		let (i, n) = shard.split_once('/').expect("CORPUS_SHARD is i/n");
		let (i, n): (usize, usize) = (i.parse().unwrap(), n.parse().unwrap());
		assert!(n > 0 && (1..=n).contains(&i), "CORPUS_SHARD is i/n with 1 <= i <= n");
		chosen = (chosen.into_iter().enumerate())
			.filter(|(k, _)| k % n == i - 1)
			.map(|(_, krate)| krate)
			.collect();
	}
	if let Ok(names) = std::env::var("CORPUS_CRATES") {
		let names: Vec<&str> = names.split(',').collect();
		chosen.retain(|k| names.contains(&k.name.as_str()));
	}
	for (name, quest) in KNOWN {
		assert!(
			crates.iter().any(|k| k.name == *name),
			"KNOWN's {name} ({quest}) is no crate of the corpus: remove it"
		);
	}
	let mut failures = Vec::new();
	let mut passed = Vec::new();
	let mut skipped = Vec::new();
	for krate in chosen {
		if let Some((_, quest)) = KNOWN.iter().find(|(name, _)| *name == krate.name) {
			skipped.push(format!("{} ({quest})", krate.name));
			continue;
		}
		eprintln!("corpus: {} {}", krate.name, krate.version);
		match one(krate) {
			Ok(pass) => passed.push(pass),
			Err(failure) => failures.push(failure),
		}
	}
	let report = report(&passed, &failures, &skipped);
	step::write(&base().join("report.md"), &report);
	assert!(failures.is_empty(), "{report}");
}

/// Census, export, and build one crate's facade.
fn one(krate: &Krate) -> Result<Passed, Failure> {
	let name = &krate.name;
	let fail = |stage: String, output: &Output| Failure {
		krate: name.clone(),
		stage,
		output: trimmed(output),
	};
	let facade = workspace().join(name);
	let dir = base().join("crates").join(name);
	let build = base().join("build");
	let exporter = |mode: &str, out: &Path| {
		let mut cmd = Command::new(env!("CARGO_BIN_EXE_rust2-ffi"));
		cmd.arg("--manifest-path")
			.arg(facade.join("Cargo.toml"))
			.arg(mode)
			.arg(out)
			.args(["--", "--locked"])
			.env("CARGO_TARGET_DIR", &build);
		cmd
	};
	let census = dir.join("census");
	let output = super::output(&mut exporter("--census", &census));
	if !output.status.success() {
		return Err(fail(stage(&output, "census"), &output));
	}
	let text = std::fs::read_to_string(census.join(format!("corpus-{name}.md"))).unwrap();
	let refused = refusals(&text);
	if refused > 0 {
		return Ok(Passed {
			krate: name.clone(),
			refused,
		});
	}
	let out = dir.join("out");
	let output = super::output(&mut exporter("--out", &out));
	if !output.status.success() {
		return Err(fail(stage(&output, "export"), &output));
	}
	let lib = format!("corpus_{}", name.replace('-', "_"));
	let built = bind(&lib, &out).map_err(|(stage, output)| fail(stage.to_string(), &output))?;
	languages(&lib, &out, &dir, &built).map_err(|(stage, output)| fail(stage.to_string(), &output))?;
	Ok(Passed {
		krate: name.clone(),
		refused: 0,
	})
}

/// The failed stage's name, saying how it failed where that is the exporter's own bug: a
/// panic, or an error of its own with no span.
fn stage(output: &Output, stage: &str) -> String {
	let stderr = String::from_utf8_lossy(&output.stderr);
	let lines: Vec<&str> = stderr.lines().collect();
	let spanless = (lines.iter().enumerate()).any(|(i, line)| {
		line.starts_with("error: rust2-ffi")
			&& !lines
				.get(i + 1)
				.is_some_and(|next| next.trim_start().starts_with("-->"))
	});
	match (stderr.contains("panicked at"), spanless) {
		(true, _) => format!("{stage} panicked"),
		(false, true) => format!("{stage} failed with no span"),
		(false, false) => format!("{stage} failed"),
	}
}

/// How many items the census refuses: its table's refused column, Swift's included.
fn refusals(census: &str) -> usize {
	(census.lines())
		.take_while(|line| !line.starts_with("Exported types:"))
		.filter_map(|line| line.strip_prefix("| ")?.split(" | ").nth(2)?.parse::<usize>().ok())
		.sum()
}

/// The first lines of what a command printed, where its first error is, enough to say why
/// it failed.
fn trimmed(output: &Output) -> String {
	let text = format!(
		"{}{}",
		String::from_utf8_lossy(&output.stderr),
		String::from_utf8_lossy(&output.stdout)
	);
	let lines: Vec<&str> = text
		.lines()
		.filter(|line| !line.trim_start().starts_with("Checking"))
		.collect();
	match lines.len() > 40 {
		true => format!("{}\n... {} more lines", lines[..40].join("\n"), lines.len() - 40),
		false => lines.join("\n"),
	}
}

/// The glue's libraries, and what a program linking the static one links besides.
struct Built {
	lib: PathBuf,
	native: Vec<String>,
}

/// Build the glue the export at `out` writes, into target directories every crate shares:
/// the CPython extension apart, as the api fixture's, copied beside the Python.
fn bind(lib: &str, out: &Path) -> Result<Built, (&'static str, Output)> {
	let shared = base();
	let build = |target: &Path, feature: &str| {
		super::output(
			cargo()
				.args(["build", "--quiet", "--locked", "--manifest-path"])
				.arg(out.join("Cargo.toml"))
				.arg("--target-dir")
				.arg(target)
				.args(["--features", feature])
				.env("RUSTFLAGS", "-D warnings --print=native-static-libs"),
		)
	};
	let python = shared.join("glue-python");
	let output = build(&python, "python");
	if !output.status.success() {
		return Err(("glue (python)", output));
	}
	let shared_lib = super::shared(lib);
	step::copy(
		&python.join("debug").join(&shared_lib),
		&out.join("python").join(&shared_lib),
	);
	let glue = shared.join("glue");
	let output = build(&glue, "kotlin,dart");
	if !output.status.success() {
		return Err(("glue", output));
	}
	let native = String::from_utf8_lossy(&output.stderr)
		.lines()
		.find_map(|line| line.strip_prefix("note: native-static-libs: "))
		.expect("rustc prints the static library's native libraries")
		.split_whitespace()
		.map(str::to_string)
		.collect();
	Ok(Built {
		lib: glue.join("debug"),
		native,
	})
}

/// Build each language's binding of the export at `out`: C and C++ programs including its
/// headers, compiled but not linked, since a library may call what this host lacks; the Kotlin compiled; the Python imported;
/// the Swift type-checked; the Go vetted; and the Dart compiled to a kernel.
fn languages(lib: &str, out: &Path, dir: &Path, built: &Built) -> Result<(), (&'static str, Output)> {
	let checked = |stage: &'static str, cmd: &mut Command| {
		let output = super::output(cmd);
		match output.status.success() {
			true => Ok(()),
			false => Err((stage, output)),
		}
	};
	let staticlib = built.lib.join(format!("lib{lib}.a"));
	let include = out.join("include");
	let c = dir.join("main.c");
	step::write(
		&c,
		format!("#include <{lib}.h>\n\nint main(void) {{\n\treturn 0;\n}}\n"),
	);
	let cc = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
	checked(
		"C",
		Command::new(cc)
			.args(["-std=c11", "-pedantic", "-Wall", "-Wextra", "-Werror", "-I"])
			.arg(&include)
			.arg(&c)
			.arg("-c")
			.arg("-o")
			.arg(dir.join("c.o")),
	)?;
	let cpp = dir.join("main.cpp");
	step::write(
		&cpp,
		format!("#include <{lib}.hpp>\n\nint main() {{\n\treturn 0;\n}}\n"),
	);
	let link = super::Link {
		include: include.clone(),
		staticlib: staticlib.clone(),
		native: &built.native,
		generated: None,
	};
	for (standard, plain) in [(17, true), (20, false)] {
		let exe = dir.join(format!("c++{standard}.o"));
		checked(
			"C++",
			&mut super::cxx_command(&link, standard, plain, &cpp, &exe, false),
		)?;
	}
	checked(
		"Kotlin",
		Command::new("kotlinc")
			.args(["-J-Xmx4g", "-Werror", "-cp"])
			.arg(super::coroutines())
			.arg(out.join("kotlin").join(format!("{lib}.kt")))
			.arg("-d")
			.arg(dir.join("kotlin")),
	)?;
	checked(
		"Python",
		Command::new(std::env::var_os("PYTHON").unwrap_or_else(|| "python3".into()))
			.args(["-P", "-X", "dev", "-W", "error", "-c"])
			.arg(format!("import {lib}"))
			.env("PYTHONPATH", out.join("python")),
	)?;
	super::swift_collections();
	checked(
		"Swift",
		super::swiftc(&out.join("swift"), 15)
			.args(["-swift-version", "6", "-typecheck", "-module-name", "Corpus"])
			.arg(out.join("swift").join(format!("{lib}.swift"))),
	)?;
	let go = out.join("go");
	if go.join("go.mod").exists() {
		checked(
			"Go",
			Command::new("go")
				.args(["vet", "./..."])
				.current_dir(&go)
				.env("CGO_ENABLED", "1")
				.env("GOFLAGS", "-buildvcs=false")
				.env("GOTOOLCHAIN", "local"),
		)?;
	}
	let dart = dir.join("dart");
	let main = dart.join("main.dart");
	step::write(
		&main,
		format!("import 'package:{lib}/{lib}.dart';\n\nvoid main() {{}}\n"),
	);
	let shared = built.lib.join(super::shared(lib));
	let (mut kernel, _) = super::dart_kernel(lib, &dart, &out.join("dart"), &shared, &main, &[]);
	checked("Dart", &mut kernel)?;
	Ok(())
}

/// The run's report: what passed with its refusal count, what failed and where, and what
/// [`KNOWN`] skipped.
fn report(passed: &[Passed], failures: &[Failure], skipped: &[String]) -> String {
	let mut out = String::new();
	let _ = writeln!(
		out,
		"# Corpus\n\n{} crates passed, {} failed, {} skipped as known.\n",
		passed.len(),
		failures.len(),
		skipped.len()
	);
	if !failures.is_empty() {
		let _ = writeln!(out, "## Failed\n");
		for failure in failures {
			let _ = writeln!(
				out,
				"### {}: {}\n\n```\n{}\n```\n",
				failure.krate, failure.stage, failure.output
			);
		}
	}
	let _ = writeln!(out, "## Passed\n\n| Crate | Refused |\n| --- | ---: |");
	for pass in passed {
		let _ = writeln!(out, "| {} | {} |", pass.krate, pass.refused);
	}
	if !skipped.is_empty() {
		let _ = writeln!(out, "\n## Skipped\n");
		for skip in skipped {
			let _ = writeln!(out, "- {skip}");
		}
	}
	out
}

/// Take the top [`TOP`] crates by all-time downloads from crates.io afresh, at their newest
/// stable versions, and keep each that is a library, not a proc macro, and builds as it is
/// on this host, writing `corpus/crates.txt` and the lockfile resolving them.
#[test]
#[ignore = "fetches from crates.io: just ffi corpus-update"]
fn update() {
	let mut crates = Vec::new();
	for page in 1..=TOP.div_ceil(100) {
		let url = format!("https://crates.io/api/v1/crates?page={page}&per_page=100&sort=downloads");
		let output = run(Command::new("curl").args([
			"--fail",
			"--silent",
			"--show-error",
			"--user-agent",
			"rust2-ffi corpus (https://github.com/kixel-ai/rust2)",
			&url,
		]));
		let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
		for krate in body["crates"].as_array().unwrap() {
			// Build metadata is no part of a requirement.
			let version = krate["max_stable_version"].as_str().unwrap();
			crates.push(Krate {
				name: krate["name"].as_str().unwrap().to_string(),
				version: version.split('+').next().unwrap().to_string(),
			});
		}
	}
	crates.truncate(TOP);
	write_workspace(&crates, &BTreeMap::new());
	let lock = workspace().join("Cargo.lock");
	let _ = std::fs::remove_file(&lock);
	run(cargo().arg("generate-lockfile").current_dir(workspace()));
	run(cargo().arg("fetch").current_dir(workspace()));
	let libraries = libraries();
	let mut dropped = Vec::new();
	crates.retain(|k| {
		let lib = libraries.get(&(k.name.clone(), k.version.clone())).cloned().flatten();
		if lib.is_none() {
			dropped.push(format!("{}: no library but a proc macro", k.name));
		}
		lib.is_some()
	});
	// What does not build as it is, needing a feature or a system library, is no test
	// of the exporter.
	prune(&crates);
	write_workspace(&crates, &resolve(&crates));
	crates.retain(|k| {
		let output = super::output(
			cargo()
				.args(["check", "--quiet", "--locked", "--offline", "--package"])
				.arg(format!("corpus-{}", k.name))
				.current_dir(workspace()),
		);
		if !output.status.success() {
			dropped.push(format!("{}: does not build as it is", k.name));
		}
		output.status.success()
	});
	let date = String::from_utf8_lossy(&run(Command::new("date").args(["-u", "+%Y-%m-%d"])).stdout)
		.trim()
		.to_string();
	let mut text = format!(
		"# The top {TOP} crates on crates.io by all-time downloads on {date}, at their newest stable\n# versions, written by `just ffi corpus-update`. Left out:\n"
	);
	for drop in &dropped {
		let _ = writeln!(text, "# - {drop}");
	}
	for krate in &crates {
		let _ = writeln!(text, "{} {}", krate.name, krate.version);
	}
	std::fs::write(corpus_dir().join("crates.txt"), text).unwrap();
	prune(&crates);
	std::fs::copy(&lock, corpus_dir().join("Cargo.lock")).unwrap();
}

/// Write the workspace of `crates` and drop from its lockfile what only a crate no longer
/// among them locked, which a `--locked` command refuses; Cargo moves nothing else.
fn prune(crates: &[Krate]) {
	write_workspace(crates, &BTreeMap::new());
	run(cargo()
		.args(["metadata", "--format-version", "1", "--offline"])
		.current_dir(workspace()));
}
