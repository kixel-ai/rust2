//! rust2-bench: counts each benchmark's instructions under stock Valgrind, one process a job,
//! in parallel, for Kixel Quest's `metrics.json` and for a local base-versus-head compare.

mod callgrind;
mod cargo;
mod compare;
mod fingerprint;
mod metrics;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Instant;

use fingerprint::Fingerprint;

/// The hidden argument that runs this executable as the fingerprint's CPU probe.
const PROBE: &str = "__cpu";

const USAGE: &str = "\
usage: rust2-bench run [options] [-- cargo bench args...]
       rust2-bench compare <base> [options] [-- cargo bench args...]
       rust2-bench fingerprint

run counts the working tree's benchmarks; compare counts <base>, a commit, in a worktree of its
own under ~/.cache/rust2-bench, then the working tree, and prints each change.

Benchmarks are the Cargo workspace's, built with CodSpeed's compatibility crates, unless
--program names them.

options:
  --benchmarks <file>  count only the benchmarks it names, one a line; any it names that no
                       benchmark has fails the run
  --out <file>         write the counts as Kixel Quest's metrics.json (run only)
  --jobs <n>           how many Valgrind processes run at once (default: every core)
  --build <command>    a shell command that builds the programs, run first (with --program)
  --program <command>  a shell command that runs a program whose benchmarks call CodSpeed's
                       instrument hooks; repeat for each program";

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.first().map(String::as_str) == Some(PROBE) {
		print!("{}", fingerprint::cpu());
		return ExitCode::SUCCESS;
	}
	if args.first().map(String::as_str) == Some(cargo::CAPTURE) {
		return match cargo::capture(&args[1..]) {
			Ok(()) => ExitCode::SUCCESS,
			Err(err) => {
				eprintln!("rust2-bench: {err}");
				ExitCode::FAILURE
			}
		};
	}
	match cli(&args) {
		Ok(()) => ExitCode::SUCCESS,
		Err(err) => {
			eprintln!("rust2-bench: {err}");
			ExitCode::FAILURE
		}
	}
}

fn cli(args: &[String]) -> Result<(), String> {
	let (command, rest) = args.split_first().ok_or(USAGE)?;
	match command.as_str() {
		"run" => {
			let options = Options::parse(rest)?;
			let counts = measure(Path::new("."), &options, true)?;
			print!("{}", metrics::table(&counts.counts));
			if let Some(out) = &options.out {
				metrics::write(out, &counts)?;
				eprintln!("rust2-bench: wrote {}", out.display());
			}
			Ok(())
		}
		"compare" => {
			let (base, rest) = rest.split_first().ok_or(USAGE)?;
			let options = Options::parse(rest)?;
			if options.out.is_some() {
				return Err("compare writes no metrics.json: --out is run's".to_owned());
			}
			compare::compare(base, &options)
		}
		"fingerprint" => {
			if !rest.is_empty() {
				return Err(USAGE.to_owned());
			}
			let fingerprint = Fingerprint::probe()?;
			println!("{}", fingerprint.label());
			eprintln!("arch: {}", fingerprint.arch);
			eprintln!("valgrind: {}", fingerprint.valgrind);
			eprintln!(
				"GLIBC_TUNABLES: {}",
				fingerprint.tunables.as_deref().unwrap_or("(unset)")
			);
			eprint!("cpu:\n{}", fingerprint.cpu);
			Ok(())
		}
		"-h" | "--help" | "help" => {
			println!("{USAGE}");
			Ok(())
		}
		_ => Err(USAGE.to_owned()),
	}
}

/// What to count, and how.
#[derive(Clone)]
pub struct Options {
	/// The only benchmarks to count, when given.
	benchmarks: Option<BTreeSet<String>>,
	out: Option<PathBuf>,
	jobs: usize,
	build: Option<String>,
	programs: Vec<String>,
	cargo: Vec<String>,
}

impl Options {
	fn parse(args: &[String]) -> Result<Self, String> {
		let mut options = Options {
			benchmarks: None,
			out: None,
			jobs: std::thread::available_parallelism().map_or(1, usize::from),
			build: None,
			programs: Vec::new(),
			cargo: Vec::new(),
		};
		let mut args = args.iter();
		while let Some(arg) = args.next() {
			let mut value = || args.next().ok_or(format!("{arg} takes a value\n\n{USAGE}"));
			match arg.as_str() {
				"--benchmarks" => {
					let path = value()?;
					let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
					options.benchmarks = Some(names(&text));
				}
				"--out" => options.out = Some(PathBuf::from(value()?)),
				"--jobs" => {
					let jobs = value()?;
					options.jobs = jobs
						.parse()
						.ok()
						.filter(|&n| n > 0)
						.ok_or(format!("--jobs {jobs} is not a count"))?;
				}
				"--build" => options.build = Some(value()?.clone()),
				"--program" => options.programs.push(value()?.clone()),
				"--" => options.cargo = args.by_ref().cloned().collect(),
				_ => return Err(format!("unknown argument {arg}\n\n{USAGE}")),
			}
		}
		if options.programs.is_empty() && options.build.is_some() {
			return Err("--build builds programs: name them with --program".to_owned());
		}
		if !options.programs.is_empty() && !options.cargo.is_empty() {
			return Err("cargo bench args are for a Cargo workspace's benchmarks, not --program's".to_owned());
		}
		Ok(options)
	}
}

/// The benchmark names a list holds, one a line, ignoring blank lines.
fn names(text: &str) -> BTreeSet<String> {
	text.lines()
		.map(str::trim)
		.filter(|line| !line.is_empty())
		.map(str::to_owned)
		.collect()
}

/// One Valgrind process: its command, and the one benchmark it must measure, when known.
pub struct Job {
	label: String,
	command: Command,
	expect: Option<String>,
}

/// Each benchmark's instruction count, under one fingerprint.
pub struct Counts {
	pub fingerprint: String,
	pub counts: BTreeMap<String, u64>,
}

/// The Valgrind every count and the fingerprint's probe run under: Callgrind, counting only
/// what the hooks bracket, following each exec, so a program a shell or `just` execs is
/// measured, and leaving the exit code the program's.
fn valgrind() -> Result<Command, String> {
	let mut command = Command::new(valgrind_path()?);
	command.args([
		"--tool=callgrind",
		"--instr-atstart=no",
		"--trace-children=yes",
		"--dump-line=no",
	]);
	Ok(command)
}

/// Valgrind on rust2-bench's own `PATH`, found once, so the fingerprint and every count run
/// the same one even when a job's environment sets another `PATH`.
pub fn valgrind_path() -> Result<&'static Path, String> {
	static PATH: std::sync::OnceLock<Result<PathBuf, String>> = std::sync::OnceLock::new();
	PATH.get_or_init(|| {
		use std::os::unix::fs::PermissionsExt;
		let path = std::env::var_os("PATH").unwrap_or_default();
		std::env::split_paths(&path)
			.map(|dir| dir.join("valgrind"))
			.find(|file| {
				file.metadata()
					.is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
			})
			.ok_or_else(|| "valgrind is not on PATH".to_owned())
			// Absolute, since jobs run in directories of their own.
			.and_then(|file| std::path::absolute(&file).map_err(|e| format!("cannot resolve {}: {e}", file.display())))
	})
	.as_deref()
	.map_err(Clone::clone)
}

/// Counts the benchmarks in `dir`: builds them, then runs each job under Valgrind. When
/// `strict`, a wanted benchmark `dir` lacks fails it; a compare checks that across both trees.
fn measure(dir: &Path, options: &Options, strict: bool) -> Result<Counts, String> {
	// Absolute, since each job runs in a directory of its own choosing.
	let dir = &dir
		.canonicalize()
		.map_err(|e| format!("cannot resolve {}: {e}", dir.display()))?;
	let fingerprint = Fingerprint::probe()?.label();
	eprintln!("rust2-bench: fingerprint {fingerprint}");
	let mut jobs = if options.programs.is_empty() {
		cargo::jobs(dir, &options.cargo, strict)?
	} else {
		if let Some(build) = &options.build {
			eprintln!("rust2-bench: {build}");
			// Its output goes to stderr, so stdout holds only the counts.
			let status = shell(build, dir)
				.stdout(std::io::stderr())
				.status()
				.map_err(|e| format!("cannot run {build}: {e}"))?;
			if !status.success() {
				return Err(format!("{build} failed ({status})"));
			}
		}
		options
			.programs
			.iter()
			.map(|program| Job {
				label: program.clone(),
				command: shell(program, dir),
				expect: None,
			})
			.collect()
	};
	if let Some(wanted) = &options.benchmarks {
		// Where every job names its benchmark, a name none has fails before any runs.
		if strict && jobs.iter().all(|job| job.expect.is_some()) {
			let known: BTreeSet<&String> = jobs.iter().filter_map(|job| job.expect.as_ref()).collect();
			missing(wanted, |name| known.contains(name))?;
		}
		// A job that runs one known benchmark runs only when wanted; a program runs
		// whichever it holds.
		jobs.retain(|job| job.expect.as_ref().is_none_or(|name| wanted.contains(name)));
	}
	let mut counts = run(jobs, &work(dir)?, options.jobs)?;
	if let Some(wanted) = &options.benchmarks {
		if strict {
			missing(wanted, |name| counts.contains_key(name))?;
		}
		counts.retain(|name, _| wanted.contains(name));
	}
	if strict && counts.is_empty() {
		return Err("no benchmark ran".to_owned());
	}
	Ok(Counts { fingerprint, counts })
}

/// Fails naming each wanted benchmark `has` says no benchmark is, so a bisect step never
/// reads a suspect it could not count as clean.
pub fn missing(wanted: &BTreeSet<String>, has: impl Fn(&String) -> bool) -> Result<(), String> {
	let missing: Vec<&str> = wanted.iter().filter(|name| !has(name)).map(String::as_str).collect();
	if missing.is_empty() {
		Ok(())
	} else {
		Err(format!("no benchmark is named {}", missing.join(", ")))
	}
}

/// `sh -c command` in `dir`.
fn shell(command: &str, dir: &Path) -> Command {
	let mut sh = Command::new("sh");
	sh.arg("-c").arg(command).current_dir(dir);
	sh
}

/// A new directory for one run's jobs in the tree at `dir`, apart from any other run's, so
/// two runs in one tree never read each other's profiles.
fn work(dir: &Path) -> Result<PathBuf, String> {
	let parent = dir.join("target/rust2-bench");
	std::fs::create_dir_all(&parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
	let pid = std::process::id();
	for n in 0.. {
		let work = parent.join(format!("jobs-{pid}-{n}"));
		match std::fs::create_dir(&work) {
			Ok(()) => return Ok(work),
			// Left by an earlier process of the same id that failed.
			Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
			Err(e) => return Err(format!("cannot create {}: {e}", work.display())),
		}
	}
	unreachable!("some name is free")
}

/// Runs each job under Valgrind, `parallel` at once, each in a directory of its own under
/// `work`, and gathers what they measured. Any failure fails the whole and keeps `work` for
/// its output; success removes it.
fn run(jobs: Vec<Job>, work: &Path, parallel: usize) -> Result<BTreeMap<String, u64>, String> {
	let total = jobs.len();
	let queue = Mutex::new(jobs.into_iter().enumerate());
	let done = AtomicUsize::new(0);
	let failed = AtomicBool::new(false);
	let results: Vec<Measured> = std::thread::scope(|scope| {
		let workers: Vec<_> = (0..parallel.min(total))
			.map(|_| {
				scope.spawn(|| {
					let mut results = Vec::new();
					// A failure stops what has not started, since the run fails anyway.
					while !failed.load(Ordering::Relaxed) {
						let Some((index, job)) = queue.lock().expect("the queue's lock is never poisoned").next()
						else {
							break;
						};
						let started = Instant::now();
						let label = job.label.clone();
						let result = one(job, &work.join(index.to_string()));
						let finished = done.fetch_add(1, Ordering::Relaxed) + 1;
						match &result {
							Ok(measured) => eprintln!(
								"rust2-bench: [{finished}/{total}] {label}: {} benchmark(s), {} instructions, {:.1}s",
								measured.len(),
								measured.iter().map(|(_, ir)| ir).sum::<u64>(),
								started.elapsed().as_secs_f64()
							),
							Err(_) => failed.store(true, Ordering::Relaxed),
						}
						results.push(result);
					}
					results
				})
			})
			.collect();
		workers
			.into_iter()
			.flat_map(|worker| worker.join().expect("a job's thread panicked"))
			.collect()
	});
	let mut counts = BTreeMap::new();
	let mut errors = Vec::new();
	for result in results {
		match result {
			Ok(measured) => {
				for (name, ir) in measured {
					if counts.insert(name.clone(), ir).is_some() {
						errors.push(format!("two benchmarks are named {name}"));
					}
				}
			}
			Err(err) => errors.push(err),
		}
	}
	if !errors.is_empty() {
		return Err(errors.join("\n"));
	}
	std::fs::remove_dir_all(work).map_err(|e| format!("cannot remove {}: {e}", work.display()))?;
	Ok(counts)
}

/// What one job measured: each benchmark's name and count, or why it failed.
type Measured = Result<Vec<(String, u64)>, String>;

/// Runs one job under Valgrind, its profiles and output in `dir`, and reads what it measured.
fn one(job: Job, dir: &Path) -> Measured {
	let Job { label, command, expect } = job;
	std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
	let file = |name: &str| {
		std::fs::File::create(dir.join(name)).map_err(|e| format!("cannot create {}: {e}", dir.join(name).display()))
	};
	// Valgrind expands every `%` in a file name, so one in the directory's stands for itself.
	let escaped = dir.to_str().ok_or("a job's directory is not UTF-8")?.replace('%', "%%");
	let mut valgrind = valgrind()?;
	valgrind
		.arg(format!("--callgrind-out-file={escaped}/callgrind.%p"))
		.arg(format!("--log-file={escaped}/valgrind.%p.log"))
		.arg(command.get_program())
		.args(command.get_args())
		.stdin(std::process::Stdio::null())
		.stdout(file("stdout")?)
		.stderr(file("stderr")?);
	if let Some(cwd) = command.get_current_dir() {
		valgrind.current_dir(cwd);
	}
	for (key, value) in command.get_envs() {
		match value {
			Some(value) => valgrind.env(key, value),
			None => valgrind.env_remove(key),
		};
	}
	let status = valgrind.status().map_err(|e| format!("cannot run valgrind: {e}"))?;
	if !status.success() {
		return Err(format!(
			"{label} failed under Valgrind ({status}); its output is in {}",
			dir.display()
		));
	}
	let mut measured = Vec::new();
	let mut entries: Vec<_> = std::fs::read_dir(dir)
		.map_err(|e| format!("cannot read {}: {e}", dir.display()))?
		.map(|entry| entry.map(|entry| entry.path()))
		.collect::<Result<_, _>>()
		.map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
	entries.sort();
	for path in entries {
		if !path
			.file_name()
			.is_some_and(|name| name.to_string_lossy().starts_with("callgrind."))
		{
			continue;
		}
		let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
		measured.extend(callgrind::parse(&text, &path.display().to_string())?);
	}
	match expect {
		Some(name) => match measured.as_slice() {
			[(only, _)] if *only == name => {}
			_ => {
				let found: Vec<&str> = measured.iter().map(|(name, _)| name.as_str()).collect();
				return Err(format!(
					"{label} should measure itself alone, but measured [{}]",
					found.join(", ")
				));
			}
		},
		None if measured.is_empty() => return Err(format!("{label} measured no benchmark")),
		None => {}
	}
	Ok(measured)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn a_list_holds_one_name_a_line() {
		let wanted = names("a::b\n\n  c[1]  \na::b\n");
		assert_eq!(wanted.into_iter().collect::<Vec<_>>(), ["a::b", "c[1]"]);
	}

	#[test]
	fn options_keep_programs_and_cargo_args_apart() {
		let args = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
		let options = Options::parse(&args(&["--jobs", "3", "--", "-p", "net", "--features", "x"])).unwrap();
		assert_eq!(options.jobs, 3);
		assert_eq!(options.cargo, ["-p", "net", "--features", "x"]);
		assert!(Options::parse(&args(&["--program", "a", "--", "-p", "net"])).is_err());
		assert!(Options::parse(&args(&["--build", "make"])).is_err());
		assert!(Options::parse(&args(&["--jobs", "0"])).is_err());
		assert!(Options::parse(&args(&["--jobs"])).is_err());
		assert!(Options::parse(&args(&["--frobnicate"])).is_err());
	}
}
