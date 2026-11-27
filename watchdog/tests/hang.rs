//! The built watchdog over `cargo test` of a crate whose test hangs: the run ends at the hung
//! test's bound, naming it, the healthy test passes, and no test process is left.

use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

const MANIFEST: &str = r#"[package]
name = "fixture"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]
"#;

const LIB: &str = r#"#[cfg(test)]
mod tests {
    #[test]
    fn hangs() {
        let path = std::env::var("FIXTURE_PID_FILE").unwrap();
        std::fs::write(path, std::process::id().to_string()).unwrap();
        loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        }
    }

    #[test]
    fn quick() {}
}
"#;

// The default bound would end the run only after 10 minutes of silence; the override names
// the hung test by its package.
const NEXTEST: &str = r#"[profile.default]
slow-timeout = { period = "300s", terminate-after = 2 }

[[profile.default.overrides]]
filter = 'package(fixture) & test(=tests::hangs)'
slow-timeout = { period = "30s", terminate-after = 2, grace-period = "1s" }
"#;

fn write(path: &Path, text: &str) {
	std::fs::create_dir_all(path.parent().unwrap()).unwrap();
	std::fs::write(path, text).unwrap();
}

#[test]
fn a_hung_test_is_named_and_killed() {
	let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("hang");
	write(&root.join("Cargo.toml"), MANIFEST);
	write(&root.join("src/lib.rs"), LIB);
	write(&root.join(".config/nextest.toml"), NEXTEST);
	let pid_file = root.join("pid");
	let _ = std::fs::remove_file(&pid_file);

	let start = Instant::now();
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-watchdog"))
		.args(["--profile", "default", "cargo", "test"])
		.current_dir(&root)
		.env("FIXTURE_PID_FILE", &pid_file)
		// The outer test run's, which would lint the fixture or set its threads.
		.env_remove("RUSTC_WORKSPACE_WRAPPER")
		.env_remove("CARGO_BUILD_WARNINGS")
		.env_remove("RUST_TEST_THREADS")
		.output()
		.unwrap();
	let elapsed = start.elapsed();
	let stdout = String::from_utf8_lossy(&output.stdout);
	let stderr = String::from_utf8_lossy(&output.stderr);
	let log = format!("stdout:\n{stdout}\nstderr:\n{stderr}");

	assert_eq!(output.status.code(), Some(124), "{log}");
	assert!(
		stderr.contains("rust2-watchdog: test tests::hangs in fixture's unittests src/lib.rs (")
			&& stderr.contains(") ran past its 1m bound; killing the run"),
		"{log}"
	);
	assert!(stdout.contains("test tests::quick ... ok"), "{log}");
	assert!(elapsed >= Duration::from_secs(60), "{elapsed:?}\n{log}");

	let pid: libc::pid_t = std::fs::read_to_string(&pid_file).unwrap().parse().unwrap();
	// SAFETY: signal 0 only checks that the process exists.
	let alive = unsafe { libc::kill(pid, 0) } == 0;
	assert!(!alive, "the hung test's process {pid} outlived the run");
}
