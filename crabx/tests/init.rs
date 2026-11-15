use std::{fs, path::Path, process::Command};

fn git(dir: &Path, args: &[&str]) {
	let status = Command::new("git").current_dir(dir).args(args).status().unwrap();
	assert!(status.success(), "git {args:?}");
}

fn init(dir: &Path) -> std::process::Output {
	Command::new(env!("CARGO_BIN_EXE_crabx"))
		.arg("init")
		.current_dir(dir)
		.output()
		.unwrap()
}

#[test]
fn writes_the_caller_for_origins_default_branch() {
	let temp = tempfile::tempdir().unwrap();
	let origin = temp.path().join("origin");
	let repo = temp.path().join("repo");
	fs::create_dir_all(origin.join("cli")).unwrap();
	git(&origin, &["init", "-q", "-b", "trunk"]);
	fs::write(origin.join("cli/Cargo.toml"), "").unwrap();
	git(&origin, &["add", "."]);
	git(
		&origin,
		&["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "x"],
	);

	// Outside a clone with origin/HEAD, init refuses.
	let output = init(&origin);
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr).contains("set-head"));

	git(temp.path(), &["clone", "-q", origin.to_str().unwrap(), "repo"]);
	let output = init(&repo.join("cli"));
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let written = fs::read_to_string(repo.join(".github/workflows/binaries.yml")).unwrap();
	assert!(written.contains("    branches: ['trunk']\n"));
	assert!(written.contains("crabx.yml@87549d30ad59b6d38eed731b484c4c6b6a089162 # pipeline-v1.0.0\n"));
	// Only the caller changed.
	let status = Command::new("git")
		.current_dir(&repo)
		.args(["status", "--porcelain", "-uall"])
		.output()
		.unwrap();
	assert_eq!(
		String::from_utf8_lossy(&status.stdout),
		"?? .github/workflows/binaries.yml\n"
	);

	// Rerunning is a no-op; a different file is never overwritten.
	assert!(init(&repo).status.success());
	fs::write(repo.join(".github/workflows/binaries.yml"), "mine").unwrap();
	assert!(!init(&repo).status.success());
	assert_eq!(
		fs::read_to_string(repo.join(".github/workflows/binaries.yml")).unwrap(),
		"mine"
	);

	// Options and arguments are refused before anything is read.
	let output = Command::new(env!("CARGO_BIN_EXE_crabx"))
		.args(["init", "x"])
		.current_dir(&repo)
		.output()
		.unwrap();
	assert!(!output.status.success());
}

#[test]
fn refuses_a_repository_without_a_crate() {
	let temp = tempfile::tempdir().unwrap();
	git(temp.path(), &["init", "-q"]);
	let output = init(temp.path());
	assert!(String::from_utf8_lossy(&output.stderr).contains("tracks no Cargo.toml"));
	assert!(!temp.path().join(".github").exists());
}

#[cfg(unix)]
#[test]
fn refuses_a_symlink_out_of_the_repository() {
	let temp = tempfile::tempdir().unwrap();
	let origin = temp.path().join("origin");
	let outside = temp.path().join("outside");
	fs::create_dir_all(&origin).unwrap();
	fs::create_dir_all(&outside).unwrap();
	git(&origin, &["init", "-q", "-b", "main"]);
	fs::write(origin.join("Cargo.toml"), "").unwrap();
	std::os::unix::fs::symlink("../outside", origin.join(".github")).unwrap();
	git(&origin, &["add", "."]);
	git(
		&origin,
		&["-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "x"],
	);
	git(temp.path(), &["clone", "-q", origin.to_str().unwrap(), "repo"]);
	let output = init(&temp.path().join("repo"));
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr).contains("not a directory inside the repository"));
	assert_eq!(fs::read_dir(&outside).unwrap().count(), 0);
}
