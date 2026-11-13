#![cfg(unix)]

use std::{
	fs,
	io::{BufRead, BufReader},
	os::unix::{ffi::OsStringExt, process::ExitStatusExt},
	path::Path,
	process::{Command, Stdio},
};

fn cache(temp: &Path) -> std::path::PathBuf {
	if cfg!(target_os = "macos") {
		temp.join("Library/Caches")
	} else {
		temp.join("cache")
	}
}

fn crabx(temp: &Path) -> Command {
	let mut command = Command::new(env!("CARGO_BIN_EXE_crabx"));
	command.env("HOME", temp).env("XDG_CACHE_HOME", cache(temp));
	command
}

fn setup(temp: &Path, name: &str) -> std::path::PathBuf {
	let helper = temp.join("helper");
	let source = temp.join("helper.rs");
	fs::write(
		&source,
		r##"
        use std::{io::{self, Write}, os::unix::ffi::OsStrExt};
        fn main() {
            let args: Vec<_> = std::env::args_os().skip(1).collect();
            if args.first().is_some_and(|arg| arg == "install") {
                let expected = std::env::var_os("CRABX_TEST_CRATE").unwrap_or_else(|| "fixture".into());
                assert_eq!(args[1], expected);
                assert!(args.iter().any(|arg| arg == "--locked"));
                let target = args.iter().position(|arg| arg == "--target").unwrap();
                assert_eq!(args[target + 1], std::env::var_os("CRABX_TEST_TARGET").unwrap());
                if let Some(version) = args.iter().position(|arg| arg == "--version") {
                    assert_eq!(args[version + 1], "=1.2.3");
                }
                let root = args.iter().position(|arg| arg == "--root").unwrap();
                let root = std::path::Path::new(&args[root + 1]);
                std::fs::copy(std::env::current_exe().unwrap(), root.join("bin/tool")).unwrap();
                std::fs::write(root.join(".crates2.json"),
                    r#"{"installs":{"fixture 1.2.3 (registry+https://github.com/rust-lang/crates.io-index)":{}}}"#).unwrap();
                return;
            }
            if args.first().is_some_and(|arg| arg == "signal") {
                println!("ready");
                io::stdout().flush().unwrap();
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                return;
            }
            println!("pid={}", std::process::id());
            for arg in args { println!("{:?}", arg.as_bytes()); }
            eprintln!("stderr survives");
            std::process::exit(37);
        }
    "##,
	)
	.unwrap();
	assert!(
		Command::new("rustc")
			.arg(&source)
			.arg("-o")
			.arg(&helper)
			.status()
			.unwrap()
			.success()
	);
	let root = cache(temp)
		.join("crabx/v1")
		.join(binstalk::TARGET)
		.join("author")
		.join(name);
	let entry = root.join("1.2.3");
	fs::create_dir_all(entry.join("bin")).unwrap();
	fs::copy(&helper, entry.join("bin/tool")).unwrap();
	fs::write(
		entry.join("record.json"),
		r#"{"version":"1.2.3","bins":["tool"],"source":"fixture"}"#,
	)
	.unwrap();
	root
}

#[test]
fn build_is_explicit_host_targeted_and_separate_from_author_cache() {
	let temp = tempfile::tempdir().unwrap();
	setup(temp.path(), "fixture");
	let output = crabx(temp.path())
		.args(["--build", "FiXtUrE@1.2.3", "built argument"])
		.env("CARGO", temp.path().join("helper"))
		.env("CRABX_TEST_CRATE", "FiXtUrE")
		.env("CRABX_TEST_TARGET", binstalk::TARGET)
		.env("PATH", "")
		.output()
		.unwrap();
	assert_eq!(
		output.status.code(),
		Some(37),
		"{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let built = cache(temp.path())
		.join("crabx/v1")
		.join(binstalk::TARGET)
		.join("build/fixture/1.2.3");
	assert!(built.join(".crates2.json").is_file());
	assert!(built.join("record.json").is_file());
	let again = crabx(temp.path())
		.args(["--build", "fixture@1.2.3"])
		.env("CARGO", "missing-cargo")
		.env("PATH", "")
		.output()
		.unwrap();
	assert_eq!(again.status.code(), Some(37));
}

#[test]
fn pinned_cache_matches_direct_execution_without_cargo_or_network() {
	let temp = tempfile::tempdir().unwrap();
	let root = setup(temp.path(), "fixture");
	// A pin never consults unrelated mutable update metadata.
	fs::write(root.join("latest"), "corrupt mutable metadata").unwrap();
	let args = [
		"hello world".into(),
		"--build".into(),
		"--".into(),
		std::ffi::OsString::from_vec(vec![0xff, b'x']),
	];
	let direct = Command::new(temp.path().join("helper")).args(&args).output().unwrap();
	let child = crabx(temp.path())
		.arg("FiXtUrE@1.2.3")
		.args(&args)
		.env("PATH", "")
		.env("HTTPS_PROXY", "http://127.0.0.1:1")
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.unwrap();
	let pid = child.id();
	let output = child.wait_with_output().unwrap();
	assert_eq!(output.status.code(), Some(37));
	assert_eq!(output.stderr, direct.stderr);
	let stdout = String::from_utf8(output.stdout).unwrap();
	assert_eq!(stdout.lines().next().unwrap(), format!("pid={pid}"));
	assert_eq!(
		stdout.lines().skip(1).collect::<Vec<_>>(),
		String::from_utf8(direct.stdout)
			.unwrap()
			.lines()
			.skip(1)
			.collect::<Vec<_>>()
	);
}

#[test]
fn exec_preserves_signals_and_incomplete_cache_is_an_error() {
	let temp = tempfile::tempdir().unwrap();
	let root = setup(temp.path(), "fixture");
	let mut child = crabx(temp.path())
		.args(["fixture@1.2.3", "signal"])
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.unwrap();
	let mut ready = String::new();
	BufReader::new(child.stdout.take().unwrap())
		.read_line(&mut ready)
		.unwrap();
	assert_eq!(ready, "ready\n");
	// Readiness is synchronized over a pipe, never a timing assumption.
	assert_eq!(unsafe { libc::kill(child.id() as i32, libc::SIGTERM) }, 0);
	assert_eq!(child.wait().unwrap().signal(), Some(libc::SIGTERM));
	fs::remove_file(root.join("1.2.3/bin/tool")).unwrap();
	let output = crabx(temp.path())
		.arg("fixture@1.2.3")
		.env("PATH", "")
		.output()
		.unwrap();
	assert!(!output.status.success());
	assert!(
		String::from_utf8(output.stderr)
			.unwrap()
			.contains("missing or invalid cached binary")
	);
}

#[test]
fn unpinned_build_reuses_pinned_entry_and_reports_refresh_errors() {
	let temp = tempfile::tempdir().unwrap();
	setup(temp.path(), "fixture");
	let root = cache(temp.path())
		.join("crabx/v1")
		.join(binstalk::TARGET)
		.join("build/fixture");
	fs::create_dir_all(&root).unwrap();
	// Hold the refresh lock to synchronize detached updaters without sleeps or network.
	let lock = fs::File::create(root.join("refresh.lock")).unwrap();
	lock.lock().unwrap();
	for krate in ["fixture@1.2.3", "fixture"] {
		let output = crabx(temp.path())
			.args(["--build", krate])
			.env("CARGO", temp.path().join("helper"))
			.env("CRABX_TEST_TARGET", binstalk::TARGET)
			.output()
			.unwrap();
		assert_eq!(
			output.status.code(),
			Some(37),
			"{}",
			String::from_utf8_lossy(&output.stderr)
		);
	}
	assert_eq!(fs::read_to_string(root.join("latest")).unwrap(), "1.2.3");
	let retry_at = std::time::SystemTime::now() + std::time::Duration::from_secs(1800);
	fs::write(
		root.join("refresh.json"),
		serde_json::to_vec(&serde_json::json!({
			"etag": null,
			"version": null,
			"failure": {
				"message": "fixture update failure",
				"version": null,
				"reported": false,
				"delay": std::time::Duration::from_secs(1800),
				"retry_at": retry_at,
			}
		}))
		.unwrap(),
	)
	.unwrap();
	drop(lock);
	let output = crabx(temp.path())
		.args(["--build", "fixture"])
		.env("CARGO", "must-not-run")
		.output()
		.unwrap();
	assert_eq!(output.status.code(), Some(37));
	assert!(String::from_utf8_lossy(&output.stderr).contains("fixture update failure"));
	let state: serde_json::Value = serde_json::from_slice(&fs::read(root.join("refresh.json")).unwrap()).unwrap();
	assert_eq!(state["failure"]["reported"], true);
	let output = crabx(temp.path()).args(["--build", "fixture"]).output().unwrap();
	assert_eq!(output.status.code(), Some(37));
	assert!(!String::from_utf8_lossy(&output.stderr).contains("fixture update failure"));
	fs::write(root.join("refresh.json"), "malformed").unwrap();
	let output = crabx(temp.path()).args(["--build", "fixture"]).output().unwrap();
	assert_eq!(output.status.code(), Some(1));
	let diagnostic = String::from_utf8_lossy(&output.stderr);
	assert!(diagnostic.contains("corrupt refresh state"));
	assert!(diagnostic.contains(&root.join("refresh.json").display().to_string()));
	let output = crabx(temp.path()).args(["--build", "fixture@1.2.3"]).output().unwrap();
	assert_eq!(output.status.code(), Some(37));
}
