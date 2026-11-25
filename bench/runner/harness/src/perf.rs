//! Linux perf counters of a case's user-space instructions (`instructions:u`), on every thread,
//! which rust2-bench reads natively instead of counting under Valgrind.

use std::cell::RefCell;
use std::io::Read;
use std::os::fd::{FromRawFd, OwnedFd};

/// Counts one case's instructions, on every thread its process runs during the count.
pub struct Counter {
	/// The PMU's perf type, or 0 on a host with one, whose CPUs rust2-bench runs the process on.
	kind: u32,
	state: RefCell<State>,
}

enum State {
	Idle,
	Counting(Counters),
	Done(Result<Count, String>),
}

/// What a case counted.
pub struct Count {
	pub instructions: u64,
	/// Whether another thread ran during the count, which makes it noisy.
	pub threaded: bool,
}

/// One case's counters, which the calling thread opened, so one `prctl` starts or stops them all.
struct Counters {
	/// The calling thread's, and each thread it spawns.
	own: OwnedFd,
	/// The calling thread's alone, which differs from `own` exactly when a spawned thread ran.
	alone: OwnedFd,
	/// Each other thread's, and each thread it spawns.
	others: Vec<OwnedFd>,
}

impl Counter {
	/// A counter of the PMU of perf type `kind`, 0 on a host with one.
	pub fn new(kind: u32) -> Self {
		Self {
			kind,
			state: RefCell::new(State::Idle),
		}
	}

	/// Opens counters on each of the process's threads, after the case's setup, and starts them,
	/// last of all, so little of its own work counts.
	pub fn start(&self) {
		let mut state = self.state.borrow_mut();
		assert!(matches!(*state, State::Idle), "a case is counted once");
		let opened = self.open();
		let start = opened.is_ok();
		*state = match opened {
			Ok(counters) => State::Counting(counters),
			Err(err) => State::Done(Err(err)),
		};
		drop(state);
		if start {
			// The thread is in the kernel from the first counter's start to the last's, so every
			// one counts its user-space instructions from the same one.
			prctl(PR_TASK_PERF_EVENTS_ENABLE);
		}
	}

	/// Stops the counters, first of all, and reads them.
	pub fn stop(&self) {
		// Stopping none, when the case could not count, changes nothing.
		prctl(PR_TASK_PERF_EVENTS_DISABLE);
		let mut state = self.state.borrow_mut();
		*state = match std::mem::replace(&mut *state, State::Idle) {
			State::Counting(counters) => State::Done(counters.read()),
			done @ State::Done(_) => done,
			State::Idle => panic!("a case stops after it starts"),
		};
	}

	/// What the case counted, once it has stopped, or why it could not count.
	pub fn count(&self) -> Result<Count, String> {
		match std::mem::replace(&mut *self.state.borrow_mut(), State::Idle) {
			State::Done(count) => count,
			_ => Err("the case never ran".to_owned()),
		}
	}

	fn open(&self) -> Result<Counters, String> {
		// SAFETY: it takes nothing and always succeeds.
		let own = unsafe { gettid() };
		let mut others = Vec::new();
		let tasks = std::fs::read_dir("/proc/self/task").map_err(|e| format!("cannot list /proc/self/task: {e}"))?;
		for task in tasks {
			let task = task.map_err(|e| format!("cannot list /proc/self/task: {e}"))?;
			let tid: i32 = task
				.file_name()
				.to_str()
				.and_then(|name| name.parse().ok())
				.ok_or("/proc/self/task lists a thread by no number")?;
			if tid != own {
				match instructions(self.kind, tid, true) {
					Ok(fd) => others.push(fd),
					// It exited since the listing, so it runs nothing in the count.
					Err(err) if err.raw_os_error() == Some(ESRCH) => {}
					Err(err) => return Err(refused(&err)),
				}
			}
		}
		Ok(Counters {
			own: instructions(self.kind, 0, true).map_err(|err| refused(&err))?,
			alone: instructions(self.kind, 0, false).map_err(|err| refused(&err))?,
			others,
		})
	}
}

impl Counters {
	fn read(self) -> Result<Count, String> {
		let alone = read(&self.alone)?;
		let mut instructions = 0;
		let mut spawned = false;
		for fd in std::iter::once(&self.own).chain(&self.others) {
			let count = read(fd)?;
			if std::ptr::eq(fd, &self.own) {
				spawned = count != alone;
			}
			instructions += count;
		}
		Ok(Count {
			instructions,
			threaded: spawned || !self.others.is_empty(),
		})
	}
}

/// Fails unless this host lets a process count its own user-space instructions on the PMU of
/// perf type `kind`, saying why not.
pub fn probe(kind: u32) -> Result<(), String> {
	instructions(kind, 0, false).map(drop).map_err(|err| refused(&err))
}

/// Why opening a counter failed, and what to change.
fn refused(err: &std::io::Error) -> String {
	let paranoid = std::fs::read_to_string("/proc/sys/kernel/perf_event_paranoid")
		.map_or_else(|_| "unreadable".to_owned(), |text| text.trim().to_owned());
	format!(
		"cannot open a perf counter of user-space instructions ({err}): this host exposes no counters to a \
		 process, as a VM without its PMU passed through, or a kernel.perf_event_paranoid above 2 (here {paranoid}), does"
	)
}

const ESRCH: i32 = 3;

/// `perf_event_attr`'s first 64 bytes, `PERF_ATTR_SIZE_VER0`, all a count needs.
#[repr(C)]
#[derive(Default)]
struct Attr {
	kind: u32,
	size: u32,
	config: u64,
	sample_period: u64,
	sample_type: u64,
	read_format: u64,
	flags: u64,
	wakeup_events: u32,
	bp_type: u32,
	config1: u64,
}

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
/// Where a hardware event's config names the PMU on a host with several.
const PERF_PMU_TYPE_SHIFT: u32 = 32;
const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 2;
const DISABLED: u64 = 1 << 0;
const INHERIT: u64 = 1 << 1;
const EXCLUDE_KERNEL: u64 = 1 << 5;
const EXCLUDE_HV: u64 = 1 << 6;
const PERF_FLAG_FD_CLOEXEC: u64 = 1 << 3;
const PR_TASK_PERF_EVENTS_DISABLE: i32 = 31;
const PR_TASK_PERF_EVENTS_ENABLE: i32 = 32;

#[cfg(target_arch = "x86_64")]
const SYS_PERF_EVENT_OPEN: i64 = 298;
#[cfg(target_arch = "aarch64")]
const SYS_PERF_EVENT_OPEN: i64 = 241;

unsafe extern "C" {
	fn syscall(number: i64, ...) -> i64;
	#[link_name = "prctl"]
	fn sys_prctl(option: i32, ...) -> i32;
	fn gettid() -> i32;
}

/// Starts or stops every perf event the calling thread opened.
fn prctl(option: i32) {
	// SAFETY: these options take no argument.
	let done = unsafe { sys_prctl(option) };
	assert_eq!(done, 0, "a thread starts and stops the perf events it opened");
}

/// A stopped counter of thread `tid`'s user-space instructions (0 for the calling thread's), and
/// when `inherit`, of each thread it spawns.
fn instructions(kind: u32, tid: i32, inherit: bool) -> std::io::Result<OwnedFd> {
	let attr = Attr {
		kind: PERF_TYPE_HARDWARE,
		size: std::mem::size_of::<Attr>() as u32,
		config: u64::from(kind) << PERF_PMU_TYPE_SHIFT | PERF_COUNT_HW_INSTRUCTIONS,
		read_format: PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING,
		flags: DISABLED | EXCLUDE_KERNEL | EXCLUDE_HV | if inherit { INHERIT } else { 0 },
		..Attr::default()
	};
	#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
	{
		// SAFETY: `attr` is a `perf_event_attr` of the size it states, read during the call.
		let fd = unsafe {
			syscall(
				SYS_PERF_EVENT_OPEN,
				&raw const attr,
				tid,
				-1i32,
				-1i32,
				PERF_FLAG_FD_CLOEXEC,
			)
		};
		if fd < 0 {
			return Err(std::io::Error::last_os_error());
		}
		// SAFETY: the kernel returned a new descriptor, which nothing else owns.
		Ok(unsafe { OwnedFd::from_raw_fd(fd as i32) })
	}
	#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
	{
		let _ = (attr, tid);
		Err(std::io::Error::new(
			std::io::ErrorKind::Unsupported,
			"rust2-bench-harness opens perf counters on x86_64 and aarch64",
		))
	}
}

/// A counter's value, refusing one that did not count the whole time it was enabled.
fn read(fd: &OwnedFd) -> Result<u64, String> {
	let mut bytes = [0u8; 24];
	let mut file = std::fs::File::from(fd.try_clone().map_err(|e| format!("cannot read a perf counter: {e}"))?);
	file.read_exact(&mut bytes)
		.map_err(|e| format!("cannot read a perf counter: {e}"))?;
	let [count, enabled, running] =
		std::array::from_fn(|i| u64::from_ne_bytes(bytes[i * 8..i * 8 + 8].try_into().expect("8 bytes")));
	// A counter the kernel shared among more events than the PMU holds counted part of the
	// time, and one on a CPU of another PMU counted none of it: neither is scaled up.
	if enabled != running {
		return Err(format!(
			"a perf counter ran {running} ns of the {enabled} ns it was enabled, so it missed instructions: another \
			 user of this host's counters multiplexed it, or a thread left the CPUs of the PMU it counts"
		));
	}
	Ok(count)
}
