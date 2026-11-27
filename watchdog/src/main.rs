//! Run `cargo test`, or a tool that takes its arguments such as `cross test`, and kill it when
//! one test runs past its bound in the workspace's `.config/nextest.toml`, naming that test.
//!
//! Each test binary runs all of its tests in one process, as `cargo test` does, so a target
//! where a process launch is dear, such as the iOS simulator, Intel SDE, or an Android
//! device, pays one per binary rather than one per test as under nextest.

#[cfg(not(unix))]
compile_error!("rust2-watchdog kills process groups, which only Unix has");

mod cargo;
mod config;
mod filter;
mod watch;

use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::process::{Child, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

use crate::cargo::{Artifacts, Test};
use crate::config::Bounds;
use crate::watch::{Kill, Watch};

const USAGE: &str = "usage: rust2-watchdog [--profile NAME] COMMAND [ARG...]

Runs COMMAND, a `test` command such as `cargo test` or `cross test`, and kills it when one
test runs past its bound, naming that test. The bounds are nextest's: profile NAME's
slow-timeout in the workspace's .config/nextest.toml, period times terminate-after, and its
overrides. NAME defaults to $NEXTEST_PROFILE, then `default`.

Exits with COMMAND's status, or 124 when a bound ran out.";

// As `timeout` exits when its command runs out of time.
const TIMED_OUT: u8 = 124;

fn main() -> ExitCode {
	match run() {
		Ok(code) => code,
		Err(error) => {
			eprintln!("rust2-watchdog: {error:#}");
			ExitCode::from(2)
		}
	}
}

fn run() -> Result<ExitCode> {
	let mut args = std::env::args_os().skip(1).peekable();
	let mut profile = None;
	while let Some(arg) = args.next_if(|arg| arg.to_string_lossy().starts_with('-')) {
		match arg.to_str() {
			Some("-h" | "--help") => {
				println!("{USAGE}");
				return Ok(ExitCode::SUCCESS);
			}
			Some("-P" | "--profile") => {
				let name = args.next().context("--profile takes a name")?;
				profile = Some(name.into_string().ok().context("--profile's name is not UTF-8")?);
			}
			Some("--") => break,
			_ => bail!("unknown option {}\n\n{USAGE}", arg.display()),
		}
	}
	let profile = match profile {
		Some(profile) => profile,
		None => std::env::var("NEXTEST_PROFILE").unwrap_or_else(|_| "default".to_owned()),
	};
	let test = Test::new(args.collect::<Vec<OsString>>())?;
	if std::env::var_os("RUST_TEST_NOCAPTURE").is_some_and(|value| value != "0") {
		bail!("RUST_TEST_NOCAPTURE lets a test's output pose as libtest's; unset it");
	}

	let path = test.nextest_config()?;
	let text = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
	let bounds = Bounds::parse(&text, &profile).with_context(|| path.display().to_string())?;

	// Build first, unbounded, so the run's only silences are the tests'.
	let build = test
		.build()
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit())
		.output()
		.context("running the build pass")?;
	let stdout = String::from_utf8_lossy(&build.stdout);
	let artifacts = Artifacts::parse(&stdout, |line| eprintln!("{line}"))?;
	if !build.status.success() {
		return Ok(exit_code(build.status));
	}

	supervise(&test, &bounds, &artifacts)
}

enum Event {
	Output(Vec<u8>),
	Closed,
	Signal(i32),
}

fn supervise(test: &Test, bounds: &Bounds, artifacts: &Artifacts) -> Result<ExitCode> {
	// One pipe for stdout and stderr, so cargo's `Running` lines and libtest's arrive in the
	// order they were written.
	let (mut reader, writer) = io::pipe()?;
	let mut command = test.run();
	command.stdout(writer.try_clone()?).stderr(writer).process_group(0);
	let mut child = command.spawn().context("running the command")?;
	// Drop our ends of the pipe's write side, so it closes when the command's do.
	drop(command);
	let group = libc::pid_t::try_from(child.id()).context("the command's process ID")?;

	let (events, received) = mpsc::channel();
	let output = events.clone();
	thread::spawn(move || {
		let mut buffer = vec![0; 64 * 1024];
		loop {
			match reader.read(&mut buffer) {
				Ok(0) | Err(_) => break,
				Ok(n) => {
					if output.send(Event::Output(buffer[..n].to_vec())).is_err() {
						return;
					}
				}
			}
		}
		let _ = output.send(Event::Closed);
	});
	// A cancel, such as a CI step's or Ctrl-C, reaches the whole group.
	let mut signals = Signals::new([SIGHUP, SIGINT, SIGTERM])?;
	thread::spawn(move || {
		for signal in signals.forever() {
			if events.send(Event::Signal(signal)).is_err() {
				return;
			}
		}
	});

	let start = Instant::now();
	let mut watch = Watch::new(bounds, artifacts);
	let mut stdout = io::stdout().lock();
	loop {
		let now = start.elapsed();
		if let Some(kill) = watch.expired(now) {
			stdout.flush()?;
			stop(&mut child, group, &kill)?;
			return Ok(ExitCode::from(TIMED_OUT));
		}
		let event = match watch.deadline() {
			Some(deadline) => match received.recv_timeout(deadline.saturating_sub(now)) {
				Ok(event) => event,
				Err(mpsc::RecvTimeoutError::Timeout) => continue,
				Err(mpsc::RecvTimeoutError::Disconnected) => Event::Closed,
			},
			None => received.recv().unwrap_or(Event::Closed),
		};
		match event {
			Event::Output(bytes) => {
				// With nowhere to write, such as after `| head` exits, stop rather than leave the
				// command running unwatched.
				let written = stdout.write_all(&bytes).and_then(|()| stdout.flush());
				let timed = written
					.map_err(anyhow::Error::from)
					.and_then(|()| watch.output(&bytes, start.elapsed()));
				if let Err(error) = timed {
					let kill = Kill {
						reason: format!("{error:#}"),
						grace: bounds.default.grace,
					};
					stop(&mut child, group, &kill)?;
					return Ok(ExitCode::from(2));
				}
			}
			Event::Closed => break,
			Event::Signal(signal) => signal_group(group, signal)?,
		}
	}
	Ok(exit_code(child.wait()?))
}

// SIGTERM, then SIGKILL once the grace runs out: `simctl spawn` passes a SIGTERM on to its
// test in the simulator, but dies to a SIGKILL and leaves the test running there.
fn stop(child: &mut Child, group: libc::pid_t, kill: &Kill) -> Result<()> {
	// Best effort: stderr may be a closed pipe too, and the group must still be stopped.
	let _ = writeln!(io::stderr(), "\nrust2-watchdog: {}; killing the run", kill.reason);
	signal_group(group, SIGTERM)?;
	let deadline = Instant::now() + kill.grace;
	loop {
		// Reap the command first: a zombie still counts as the group's.
		if child.try_wait()?.is_some() && !group_alive(group) {
			return Ok(());
		}
		if Instant::now() >= deadline {
			break;
		}
		thread::sleep(Duration::from_millis(50));
	}
	let _ = writeln!(
		io::stderr(),
		"rust2-watchdog: the run outlived its {} grace; sending SIGKILL",
		humantime::format_duration(kill.grace)
	);
	signal_group(group, libc::SIGKILL)?;
	child.wait()?;
	Ok(())
}

fn signal_group(group: libc::pid_t, signal: i32) -> Result<()> {
	// SAFETY: killpg reads only its arguments.
	if unsafe { libc::killpg(group, signal) } == 0 {
		return Ok(());
	}
	let error = io::Error::last_os_error();
	// The group may have ended since.
	if error.raw_os_error() == Some(libc::ESRCH) {
		return Ok(());
	}
	Err(error).context("signalling the command's process group")
}

fn group_alive(group: libc::pid_t) -> bool {
	// SAFETY: killpg reads only its arguments, and signal 0 only checks the group exists.
	let alive = unsafe { libc::killpg(group, 0) } == 0;
	alive || io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

fn exit_code(status: std::process::ExitStatus) -> ExitCode {
	match (status.code(), status.signal()) {
		(Some(code), _) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
		// As a shell reports a command a signal ended.
		(None, Some(signal)) => ExitCode::from(u8::try_from(128 + signal).unwrap_or(1)),
		(None, None) => ExitCode::FAILURE,
	}
}
