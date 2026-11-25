//! rust2-bench: counts each benchmark's instructions under stock Valgrind, one process a job,
//! in parallel, for Kixel Quest's `metrics.json` and for a local base-versus-head compare.

mod callgrind;
mod cargo;
mod compare;
mod fingerprint;
mod metrics;
mod preload;

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{OsStr, OsString};
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

run counts the working tree's benchmarks; compare counts <base>, a commit, then the working
tree, both in one worktree of their own under ~/.cache/rust2-bench, and prints each change.

Benchmarks are the Cargo workspace's, written with rust2-bench-harness or CodSpeed's
compatibility crates, unless --program names them.

options:
  --benchmarks <file>  count only the benchmarks it names, one a line; any it names that no
                       benchmark has fails the run
  --allow-missing      with --benchmarks, report each name no benchmark has and count the rest,
                       failing only when none is there, as a bisect step needs (run only)
  --out <file>         write the counts as Kixel Quest's metrics.json (run only)
  --jobs <n>           how many Valgrind processes run at once (default: every core)
  --build <command>    a shell command that builds the programs, run first (with --program)
  --program <command>  a shell command that runs a program whose benchmarks run through
                       rust2-bench-harness or CodSpeed's instrument hooks; repeat for each";

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
			if options.allow_missing {
				return Err("compare already counts a name only one side has: --allow-missing is run's".to_owned());
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
			eprintln!("preload: {}", fingerprint.preload);
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
	/// Whether a wanted benchmark no benchmark is gets reported rather than failing the run.
	allow_missing: bool,
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
			allow_missing: false,
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
				"--allow-missing" => options.allow_missing = true,
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
		if options.allow_missing && options.benchmarks.is_none() {
			return Err("--allow-missing tolerates names --benchmarks lists: give it one".to_owned());
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

/// rust2-bench's directory in the user's cache, created when missing, as a canonical path.
pub fn cache() -> Result<PathBuf, String> {
	let cache = std::env::var_os("XDG_CACHE_HOME")
		.map(PathBuf::from)
		.or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".cache")))
		.ok_or("neither XDG_CACHE_HOME nor HOME is set, so rust2-bench's cache has nowhere to go")?
		.join("rust2-bench");
	std::fs::create_dir_all(&cache).map_err(|e| format!("cannot create {}: {e}", cache.display()))?;
	cache
		.canonicalize()
		.map_err(|e| format!("cannot resolve {}: {e}", cache.display()))
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
	let preload = preload::install()?;
	fits(&std::fs::read_to_string("/proc/self/limits").map_err(|e| format!("cannot read /proc/self/limits: {e}"))?)?;
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
			absent(wanted, |name| known.contains(name), options.allow_missing)?;
		}
		// A job that runs one known benchmark runs only when wanted; a program runs
		// whichever it holds.
		jobs.retain(|job| job.expect.as_ref().is_none_or(|name| wanted.contains(name)));
	}
	let mut counts = run(jobs, &work(dir)?, &preload, options.jobs)?;
	if let Some(wanted) = &options.benchmarks {
		if strict {
			let absent = absent(wanted, |name| counts.contains_key(name), options.allow_missing)?;
			if !absent.is_empty() {
				report(&absent)?;
			}
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
	absent(wanted, has, false).map(drop)
}

/// The wanted benchmarks `has` says no benchmark is. Any fails the run unless `allow`, when
/// only all of them does: Kixel Quest reads a benchmark `metrics.json` lacks as unmeasured,
/// never as clean, so a bisect step keeps the suspects it can count.
fn absent(wanted: &BTreeSet<String>, has: impl Fn(&String) -> bool, allow: bool) -> Result<Vec<&str>, String> {
	let absent: Vec<&str> = wanted.iter().filter(|name| !has(name)).map(String::as_str).collect();
	if absent.is_empty() || (allow && absent.len() < wanted.len()) {
		Ok(absent)
	} else {
		Err(format!("no benchmark is named {}", absent.join(", ")))
	}
}

/// Reports the wanted benchmarks a run lacks on stderr, and in the GitHub Actions job's
/// summary when it runs in one.
fn report(absent: &[&str]) -> Result<(), String> {
	use std::io::Write;
	eprintln!(
		"rust2-bench: no benchmark is named {}; counted the rest",
		absent.join(", ")
	);
	let Some(summary) = std::env::var_os("GITHUB_STEP_SUMMARY").filter(|path| !path.is_empty()) else {
		return Ok(());
	};
	let mut text = String::from("rust2-bench counted no benchmark named:\n\n");
	for name in absent {
		text.push_str(&format!("- `{name}`\n"));
	}
	text.push('\n');
	std::fs::OpenOptions::new()
		.append(true)
		.create(true)
		.open(&summary)
		.and_then(|mut file| file.write_all(text.as_bytes()))
		.map_err(|e| format!("cannot write the job summary {}: {e}", Path::new(&summary).display()))
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

/// Runs each job under Valgrind with the library at `preload`, `parallel` at once, each in a
/// directory of its own under `work`, and gathers what they measured. Any failure fails the
/// whole and keeps `work` for its output; success removes it.
fn run(jobs: Vec<Job>, work: &Path, preload: &Path, parallel: usize) -> Result<BTreeMap<String, u64>, String> {
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
						let result = one(job, &work.join(index.to_string()), preload);
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

/// The prefix of the variables [`pad`] fills.
const PAD: &str = "RUST2_BENCH_PAD_";

/// How many variables [`pad`] fills: a fixed number, so the count of pointers is fixed too, and
/// enough that none passes the 128 KiB Linux takes in one.
const PADS: usize = 16;

/// How many bytes [`pad`] brings a job's arguments and environment to: room for a dev shell's
/// several hundred KiB, and half the 2 MiB Linux takes in all under the default stack limit.
const LAUNCH: usize = 1024 * 1024;

/// How far a program's initial stack pointer sits below its strings, as Valgrind lays them out:
/// each argument and variable with its NUL, the program's path once more for `AT_EXECFN`, and a
/// pointer for each argument and variable. The rest it adds is the same for every job.
fn launch(env: &BTreeMap<OsString, OsString>, args: &[&OsStr]) -> usize {
	use std::os::unix::ffi::OsStrExt;
	let strings = args.iter().map(|arg| arg.as_bytes().len() + 1).sum::<usize>()
		+ args.first().map_or(0, |program| program.as_bytes().len() + 1)
		+ env
			.iter()
			.map(|(key, value)| key.as_bytes().len() + value.as_bytes().len() + 2)
			.sum::<usize>();
	strings + 8 * (args.len() + env.len())
}

/// Adds [`PADS`] variables to `env`, sized so the job's arguments and environment take
/// [`LAUNCH`] bytes, so its initial stack, and every address a count could depend on, sits at the
/// same place however long the checkout's path is or however large the host's environment.
fn pad(env: &mut BTreeMap<OsString, OsString>, args: &[&OsStr]) -> Result<(), String> {
	use std::os::unix::ffi::OsStrExt;
	env.retain(|key, _| !key.as_bytes().starts_with(PAD.as_bytes()));
	let name = |i: usize| format!("{PAD}{i:02}");
	// Each variable's name, `=`, NUL, and pointer.
	let used = launch(env, args) + (0..PADS).map(|i| name(i).len() + 2 + 8).sum::<usize>();
	let fill = LAUNCH.checked_sub(used).ok_or(format!(
		"its arguments and environment take {used} bytes, more than the {LAUNCH} rust2-bench pads every job's to"
	))?;
	for i in 0..PADS {
		let size = fill / PADS + usize::from(i < fill % PADS);
		env.insert(name(i).into(), "x".repeat(size).into());
	}
	Ok(())
}

/// Room beside [`LAUNCH`] for Valgrind's own path and options, which name the job's directory.
const VALGRIND_ROOM: usize = 64 * 1024;

/// Fails unless the stack limit `limits` (`/proc/self/limits`) gives `exec` room for a padded
/// launch: Linux takes arguments and environment up to a quarter of the limit, at most 6 MiB.
fn fits(limits: &str) -> Result<(), String> {
	let soft = limits
		.lines()
		.find_map(|line| line.strip_prefix("Max stack size"))
		.and_then(|rest| rest.split_whitespace().next())
		.ok_or("/proc/self/limits names no stack limit")?;
	let room = match soft {
		"unlimited" => 6 << 20,
		bytes => (bytes
			.parse::<usize>()
			.map_err(|_| format!("cannot read the stack limit {bytes}"))?
			/ 4)
		.min(6 << 20),
	};
	if room < LAUNCH + VALGRIND_ROOM {
		return Err(format!(
			"the stack limit (ulimit -s) leaves exec {} KiB for a job's arguments and environment, less than the {} KiB a \
			 padded job's launch takes: raise it to at least {} KiB",
			room / 1024,
			(LAUNCH + VALGRIND_ROOM) / 1024,
			4 * (LAUNCH + VALGRIND_ROOM) / 1024
		));
	}
	Ok(())
}

/// What one job measured: each benchmark's name and count, or why it failed.
type Measured = Result<Vec<(String, u64)>, String>;

/// Runs one job under Valgrind with the library at `preload` in every process, its profiles
/// and output in `dir`, and reads what it measured.
fn one(job: Job, dir: &Path, preload: &Path) -> Measured {
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
	// The whole environment, spelled out, so its size is known exactly.
	let mut env: BTreeMap<OsString, OsString> = std::env::vars_os().collect();
	for (key, value) in command.get_envs() {
		match value {
			Some(value) => env.insert(key.to_owned(), value.to_owned()),
			None => env.remove(key),
		};
	}
	// First, so its randomness and clock stand in front of any other preload's.
	let mut preloads = preload.as_os_str().to_owned();
	if let Some(others) = env.get(OsStr::new("LD_PRELOAD")).filter(|others| !others.is_empty()) {
		preloads.push(":");
		preloads.push(others);
	}
	env.insert("LD_PRELOAD".into(), preloads);
	let args: Vec<&OsStr> = std::iter::once(command.get_program())
		.chain(command.get_args())
		.collect();
	pad(&mut env, &args).map_err(|err| format!("{label}: {err}"))?;
	valgrind.env_clear().envs(&env);
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
		let found = callgrind::parse(&text, &path.display().to_string())?;
		if !found.is_empty() {
			// `callgrind.<pid>`, then `.<n>` for each dump after the first.
			let name = path.file_name().expect("listed").to_string_lossy();
			let pid = name["callgrind.".len()..].split('.').next().unwrap_or_default();
			preload::check(dir, pid, &label)?;
		}
		measured.extend(found);
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
	fn allowing_absent_names_fails_only_when_all_are() {
		let wanted = names("a\nb\n");
		let has = |names: &'static [&str]| move |name: &String| names.contains(&name.as_str());
		assert_eq!(absent(&wanted, has(&["a", "b"]), false), Ok(vec![]));
		assert!(absent(&wanted, has(&["a"]), false).is_err());
		assert_eq!(absent(&wanted, has(&["a"]), true), Ok(vec!["b"]));
		assert_eq!(
			absent(&wanted, has(&[]), true),
			Err("no benchmark is named a, b".to_owned())
		);
	}

	#[test]
	fn refuses_a_stack_limit_too_small_to_pad_under() {
		let limits = |soft: &str| format!("Limit Soft Limit Hard Limit Units\nMax stack size {soft} unlimited bytes\n");
		assert!(fits(&limits("8388608")).is_ok());
		assert!(fits(&limits("unlimited")).is_ok());
		assert!(fits(&limits("2097152")).unwrap_err().contains("ulimit -s"));
		assert!(fits("Max open files 1024 1024 files\n").is_err());
	}

	#[test]
	fn pads_every_launch_to_one_size() {
		let env = |pairs: &[(&str, &str)]| -> BTreeMap<OsString, OsString> {
			pairs.iter().map(|(key, value)| (key.into(), value.into())).collect()
		};
		let mut short = env(&[("CARGO_MANIFEST_DIR", "/a/rs/net"), ("RUST2_BENCH_PAD_03", "stale")]);
		let mut long = env(&[
			("CARGO_MANIFEST_DIR", "/a/much/longer/checkout/rs/net"),
			("GITHUB_REF", "refs/heads/kixel-bisect/1/abc"),
		]);
		pad(&mut short, &[OsStr::new("/a/bench"), OsStr::new("--bench")]).unwrap();
		pad(&mut long, &[OsStr::new("/a/much/longer/checkout/bench")]).unwrap();
		assert_eq!(launch(&short, &[OsStr::new("/a/bench"), OsStr::new("--bench")]), LAUNCH);
		assert_eq!(launch(&long, &[OsStr::new("/a/much/longer/checkout/bench")]), LAUNCH);
		assert_eq!(short.len(), 1 + PADS);
		let mut huge = env(&[("BIG", &"x".repeat(LAUNCH))]);
		assert!(pad(&mut huge, &[OsStr::new("/a/bench")]).is_err());
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
		assert!(Options::parse(&args(&["--allow-missing"])).is_err());
	}
}
