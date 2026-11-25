//! The library rust2-bench preloads into every process a counted job runs, so a benchmark's
//! instruction count repeats: randomness comes from a fixed-seed stream, and time from a clock
//! that starts at the same instant in every process and advances only as the process reads it,
//! 1 µs a read, and as its timed waits expire, by their timeout.
//!
//! A real wait still takes real time: a deadline the process computed on this clock is moved
//! onto the host's before the kernel sees it, so a timed wait blocks as long as it asked to.
//! It interposes glibc's entry points, so a process that reaches the kernel another way, such as
//! Go's runtime or a raw `syscall` instruction, bypasses it; a static executable never loads it.
//!
//! Built by build.rs as a `no_std` cdylib for x86_64 Linux, the only target rust2-bench
//! fingerprints, whose calling convention passes `syscall`'s variadic arguments as it passes
//! six fixed ones.

#![no_std]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering::Relaxed};

const NS: u64 = 1_000_000_000;

/// How far each read moves the clock: forward, so a loop waiting for time to pass ends.
const TICK: u64 = 1_000;

/// Where the monotonic clocks start: a day, as a host's uptime, so code subtracting a span from
/// the first instant it reads does not underflow.
const MONOTONIC: u64 = 86_400 * NS;

/// Where the wall clock starts: 2026-01-01T00:00:00Z.
const REALTIME: u64 = 1_767_225_600 * NS;

/// TAI ran 37 seconds ahead of UTC on that date.
const TAI: u64 = REALTIME + 37 * NS;

/// The stream's seed; any constant does, but it must not be a constant stream, which hangs
/// code that draws until two values differ, such as QUIC's connection IDs.
const SEED: u64 = 0x7275_7374_3262_656e;

/// What a process logs to Valgrind once this library loads, which rust2-bench requires of every
/// process that measured a benchmark.
const MARKER: &[u8] = b"rust2-bench preload: deterministic randomness and clock\n\0";

const CLOCK_REALTIME: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const CLOCK_PROCESS_CPUTIME_ID: c_int = 2;
const CLOCK_THREAD_CPUTIME_ID: c_int = 3;
const CLOCK_MONOTONIC_RAW: c_int = 4;
const CLOCK_REALTIME_COARSE: c_int = 5;
const CLOCK_MONOTONIC_COARSE: c_int = 6;
const CLOCK_BOOTTIME: c_int = 7;
const CLOCK_REALTIME_ALARM: c_int = 8;
const CLOCK_BOOTTIME_ALARM: c_int = 9;
const CLOCK_TAI: c_int = 11;
const TIMER_ABSTIME: c_int = 1;

const SYS_CLOCK_GETTIME: c_long = 228;
const SYS_FUTEX: c_long = 202;
const SYS_GETRANDOM: c_long = 318;
const FUTEX_WAIT: c_long = 0;
const FUTEX_WAIT_BITSET: c_long = 9;
const FUTEX_PRIVATE_FLAG: c_long = 128;
const FUTEX_CLOCK_REALTIME: c_long = 256;

const GRND_NONBLOCK: c_uint = 1;
const GRND_RANDOM: c_uint = 2;
const GRND_INSECURE: c_uint = 4;

const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EIO: c_int = 5;
const ETIMEDOUT: c_int = 110;

/// `dlsym`'s handle for the next definition of a symbol after this library's.
const RTLD_NEXT: *mut c_void = -1isize as *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timespec {
	sec: i64,
	nsec: i64,
}

#[repr(C)]
pub struct Timeval {
	sec: i64,
	usec: i64,
}

impl Timespec {
	fn from_ns(ns: u64) -> Self {
		Self {
			sec: (ns / NS) as i64,
			nsec: (ns % NS) as i64,
		}
	}

	/// Nanoseconds, or `None` for a time before the epoch or out of range, which the real
	/// call then refuses as the kernel would.
	fn ns(&self) -> Option<u64> {
		if self.sec < 0 || !(0..NS as i64).contains(&self.nsec) {
			return None;
		}
		(self.sec as u64).checked_mul(NS)?.checked_add(self.nsec as u64)
	}
}

// glibc's, which the process has loaded before this library's symbols bind: the library links
// nothing, so it carries no path of the host that built it.
unsafe extern "C" {
	fn abort() -> !;
	fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
	fn dlvsym(handle: *mut c_void, symbol: *const c_char, version: *const c_char) -> *mut c_void;
	fn __errno_location() -> *mut c_int;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
	// SAFETY: abort takes nothing and never returns.
	unsafe { abort() }
}

/// The next definition of a function after this library's, found once.
macro_rules! real {
	($name:literal $(@ $version:literal)?: $ty:ty) => {{
		static FOUND: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
		let mut found = FOUND.load(Relaxed);
		if found.is_null() {
			let name = concat!($name, "\0").as_ptr().cast();
			// SAFETY: the name is NUL-terminated, and RTLD_NEXT is a handle dlsym takes.
			found = unsafe { real!(@find name $(, $version)?) };
			if found.is_null() {
				// SAFETY: abort takes nothing and never returns.
				unsafe { abort() }
			}
			FOUND.store(found, Relaxed);
		}
		// SAFETY: glibc's function of this name has this type.
		unsafe { core::mem::transmute::<*mut c_void, $ty>(found) }
	}};
	(@find $name:ident) => { dlsym(RTLD_NEXT, $name) };
	// dlsym alone finds a symbol's oldest version, which for pthread_cond_* is the pre-NPTL one.
	(@find $name:ident, $version:literal) => {
		dlvsym(RTLD_NEXT, $name, concat!($version, "\0").as_ptr().cast())
	};
}

fn set_errno(err: c_int) {
	// SAFETY: glibc's errno location is this thread's, always valid.
	unsafe { *__errno_location() = err }
}

fn errno() -> c_int {
	// SAFETY: as in set_errno.
	unsafe { *__errno_location() }
}

/// Logs [`MARKER`] to Valgrind's log through a client request, a no-op on the CPU.
extern "C" fn loaded() {
	// VG_USERREQ__PRINTF_VALIST_BY_REF: the format holds no conversion, so the va_list it is
	// handed is never read.
	let va_list = [0u64; 3];
	let args: [u64; 6] = [0x1403, MARKER.as_ptr() as u64, va_list.as_ptr() as u64, 0, 0, 0];
	// SAFETY: the rotations sum to 128, leaving rdi as it was, and `xchg rbx, rbx` changes
	// nothing; under Valgrind, the request reads `args` and writes rdx.
	unsafe {
		core::arch::asm!(
			"rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51", "xchg rbx, rbx",
			in("rax") args.as_ptr(),
			inout("rdx") 0u64 => _,
			options(nostack),
		);
	}
}

#[used]
#[unsafe(link_section = ".init_array")]
static LOADED: extern "C" fn() = loaded;

// Randomness: word n of the stream is SplitMix64's nth output from SEED, so threads drawing at
// once each take words of their own.

static DRAWN: AtomicU64 = AtomicU64::new(0);

/// Writes the stream's next `len` bytes to `buf`.
///
/// # Safety
///
/// `buf` holds `len` writable bytes.
unsafe fn fill(buf: *mut u8, len: usize) {
	let words = len.div_ceil(8) as u64;
	let first = DRAWN.fetch_add(words, Relaxed);
	for i in 0..words {
		let mut z = SEED.wrapping_add((first + i + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15));
		z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
		let bytes = (z ^ (z >> 31)).to_le_bytes();
		let at = i as usize * 8;
		let n = (len - at).min(8);
		// SAFETY: the caller hands `len` writable bytes at `buf`, and `at + n <= len`.
		unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.add(at), n) };
	}
}

/// glibc's `getrandom`, which std's `HashMap` seeds and the `getrandom` crate read.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn getrandom(buf: *mut c_void, len: usize, flags: c_uint) -> isize {
	if flags & !(GRND_NONBLOCK | GRND_RANDOM | GRND_INSECURE) != 0
		|| flags & (GRND_RANDOM | GRND_INSECURE) == GRND_RANDOM | GRND_INSECURE
	{
		set_errno(EINVAL);
		return -1;
	}
	if buf.is_null() && len > 0 {
		set_errno(EFAULT);
		return -1;
	}
	// SAFETY: the caller hands `len` writable bytes.
	unsafe { fill(buf.cast(), len) };
	len as isize
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getentropy(buf: *mut c_void, len: usize) -> c_int {
	if len > 256 {
		set_errno(EIO);
		return -1;
	}
	// SAFETY: the caller hands `len` writable bytes.
	unsafe { fill(buf.cast(), len) };
	0
}

// Time: one count of nanoseconds since the process started, shared by every clock.

static ELAPSED: AtomicU64 = AtomicU64::new(0);

/// Where `clock` starts, or `None` for one this library leaves the host's, such as another
/// process's CPU clock.
fn base(clock: c_int) -> Option<u64> {
	match clock {
		CLOCK_REALTIME | CLOCK_REALTIME_COARSE | CLOCK_REALTIME_ALARM => Some(REALTIME),
		CLOCK_TAI => Some(TAI),
		CLOCK_MONOTONIC | CLOCK_MONOTONIC_RAW | CLOCK_MONOTONIC_COARSE | CLOCK_BOOTTIME | CLOCK_BOOTTIME_ALARM => {
			Some(MONOTONIC)
		}
		CLOCK_PROCESS_CPUTIME_ID | CLOCK_THREAD_CPUTIME_ID => Some(0),
		_ => None,
	}
}

/// Reads the clock that starts at `base`, moving it forward a tick.
fn read(base: u64) -> u64 {
	base + ELAPSED.fetch_add(TICK, Relaxed) + TICK
}

/// How far ahead of this clock, starting at `base`, `deadline` is, without moving it.
fn ahead(base: u64, deadline: &Timespec) -> Option<u64> {
	Some(deadline.ns()?.saturating_sub(base + ELAPSED.load(Relaxed)))
}

/// Moves the clock to at least `deadline`, on the clock starting at `base`, once a wait for it
/// expired.
fn reach(base: u64, deadline: &Timespec) {
	if let Some(ns) = deadline.ns() {
		ELAPSED.fetch_max(ns.saturating_sub(base), Relaxed);
	}
}

/// Moves the clock forward `ns`, once a wait that long expired.
fn pass(ns: u64) {
	ELAPSED.fetch_add(ns, Relaxed);
}

/// The host's reading of `clock` `ahead` from now, for a deadline the kernel compares with it.
fn host_deadline(clock: c_int, ahead: u64) -> Timespec {
	let get = real!("clock_gettime": unsafe extern "C" fn(c_int, *mut Timespec) -> c_int);
	let mut now = Timespec { sec: 0, nsec: 0 };
	// SAFETY: `now` is a timespec to write.
	if unsafe { get(clock, &mut now) } != 0 {
		// SAFETY: abort takes nothing and never returns.
		unsafe { abort() }
	}
	Timespec::from_ns(now.ns().unwrap_or(0).saturating_add(ahead))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clock_gettime(clock: c_int, out: *mut Timespec) -> c_int {
	match base(clock) {
		Some(base) if !out.is_null() => {
			// SAFETY: the caller hands a timespec to write.
			unsafe { *out = Timespec::from_ns(read(base)) };
			0
		}
		Some(_) => {
			set_errno(EFAULT);
			-1
		}
		None => {
			let real = real!("clock_gettime": unsafe extern "C" fn(c_int, *mut Timespec) -> c_int);
			// SAFETY: the caller's arguments, as it passed them.
			unsafe { real(clock, out) }
		}
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gettimeofday(out: *mut Timeval, _zone: *mut c_void) -> c_int {
	if !out.is_null() {
		let ns = read(REALTIME);
		// SAFETY: the caller hands a timeval to write.
		unsafe {
			*out = Timeval {
				sec: (ns / NS) as i64,
				usec: (ns % NS / 1_000) as i64,
			}
		};
	}
	0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn time(out: *mut i64) -> i64 {
	let sec = (read(REALTIME) / NS) as i64;
	if !out.is_null() {
		// SAFETY: the caller hands a time_t to write.
		unsafe { *out = sec };
	}
	sec
}

// Waits: each one that expires moves the clock by what it waited for.

/// Moves the clock by a wait of `timeout` milliseconds that returned `ready`, when it expired.
fn waited_ms(timeout: c_int, ready: c_int) {
	if timeout > 0 && ready == 0 {
		pass(timeout as u64 * 1_000_000);
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn epoll_wait(epfd: c_int, events: *mut c_void, max: c_int, timeout: c_int) -> c_int {
	let real = real!("epoll_wait": unsafe extern "C" fn(c_int, *mut c_void, c_int, c_int) -> c_int);
	// SAFETY: the caller's arguments, as it passed them.
	let ready = unsafe { real(epfd, events, max, timeout) };
	waited_ms(timeout, ready);
	ready
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn epoll_pwait(
	epfd: c_int,
	events: *mut c_void,
	max: c_int,
	timeout: c_int,
	mask: *const c_void,
) -> c_int {
	let real = real!("epoll_pwait": unsafe extern "C" fn(c_int, *mut c_void, c_int, c_int, *const c_void) -> c_int);
	// SAFETY: the caller's arguments, as it passed them.
	let ready = unsafe { real(epfd, events, max, timeout, mask) };
	waited_ms(timeout, ready);
	ready
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn poll(fds: *mut c_void, n: u64, timeout: c_int) -> c_int {
	let real = real!("poll": unsafe extern "C" fn(*mut c_void, u64, c_int) -> c_int);
	// SAFETY: the caller's arguments, as it passed them.
	let ready = unsafe { real(fds, n, timeout) };
	waited_ms(timeout, ready);
	ready
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nanosleep(span: *const Timespec, rest: *mut Timespec) -> c_int {
	let real = real!("nanosleep": unsafe extern "C" fn(*const Timespec, *mut Timespec) -> c_int);
	// SAFETY: the caller's arguments, as it passed them.
	let slept = unsafe { real(span, rest) };
	if slept == 0 {
		// SAFETY: the call succeeded, so `span` is a timespec to read.
		pass(unsafe { (*span).ns() }.unwrap_or(0));
	}
	slept
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn clock_nanosleep(
	clock: c_int,
	flags: c_int,
	at: *const Timespec,
	rest: *mut Timespec,
) -> c_int {
	let real = real!("clock_nanosleep": unsafe extern "C" fn(c_int, c_int, *const Timespec, *mut Timespec) -> c_int);
	let Some(base) = base(clock).filter(|_| !at.is_null()) else {
		// SAFETY: the caller's arguments, as it passed them.
		return unsafe { real(clock, flags, at, rest) };
	};
	// SAFETY: the caller hands a timespec to read.
	let at = unsafe { *at };
	if flags & TIMER_ABSTIME == 0 {
		// SAFETY: the caller's arguments, as it passed them.
		let slept = unsafe { real(clock, flags, &at, rest) };
		if slept == 0 {
			pass(at.ns().unwrap_or(0));
		}
		return slept;
	}
	let Some(ahead) = ahead(base, &at) else {
		// SAFETY: the caller's arguments, which the kernel refuses.
		return unsafe { real(clock, flags, &at, rest) };
	};
	// A span to sleep on the host's monotonic clock, which any clock's sleep can stand on.
	let span = Timespec::from_ns(ahead);
	// SAFETY: a timespec to read; an absolute sleep writes no remainder.
	let slept = unsafe { real(CLOCK_MONOTONIC, 0, &span, core::ptr::null_mut()) };
	if slept == 0 {
		reach(base, &at);
	}
	slept
}

/// `pthread_cond_timedwait`'s and `pthread_cond_clockwait`'s deadline moved onto the host's
/// `clock`, or `None` when the deadline is malformed and the real call should refuse it.
fn cond_deadline(clock: c_int, at: *const Timespec) -> Option<(u64, Timespec)> {
	let base = base(clock)?;
	if at.is_null() {
		return None;
	}
	// SAFETY: the caller hands a timespec to read.
	let ahead = ahead(base, unsafe { &*at })?;
	Some((base, host_deadline(clock, ahead)))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_cond_timedwait(cond: *mut c_void, mutex: *mut c_void, at: *const Timespec) -> c_int {
	let real = real!("pthread_cond_timedwait" @ "GLIBC_2.3.2": unsafe extern "C" fn(*mut c_void, *mut c_void, *const Timespec) -> c_int);
	// A condition variable's clock is its attribute's, which the call does not say: tell them
	// apart by where this library starts each, decades apart.
	// SAFETY: when not null, the caller hands a timespec to read.
	let clock = match unsafe { at.as_ref() }.and_then(Timespec::ns) {
		Some(ns) if ns >= REALTIME / 2 => CLOCK_REALTIME,
		_ => CLOCK_MONOTONIC,
	};
	let Some((base, host)) = cond_deadline(clock, at) else {
		// SAFETY: the caller's arguments, as it passed them.
		return unsafe { real(cond, mutex, at) };
	};
	// SAFETY: the caller's condition and mutex, with a deadline on the same clock.
	let woke = unsafe { real(cond, mutex, &host) };
	if woke == ETIMEDOUT {
		// SAFETY: `at` was read above.
		reach(base, unsafe { &*at });
	}
	woke
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pthread_cond_clockwait(
	cond: *mut c_void,
	mutex: *mut c_void,
	clock: c_int,
	at: *const Timespec,
) -> c_int {
	let real = real!("pthread_cond_clockwait": unsafe extern "C" fn(*mut c_void, *mut c_void, c_int, *const Timespec) -> c_int);
	let Some((base, host)) = cond_deadline(clock, at) else {
		// SAFETY: the caller's arguments, as it passed them.
		return unsafe { real(cond, mutex, clock, at) };
	};
	// SAFETY: the caller's condition and mutex, with a deadline on the same clock.
	let woke = unsafe { real(cond, mutex, clock, &host) };
	if woke == ETIMEDOUT {
		// SAFETY: `at` was read above.
		reach(base, unsafe { &*at });
	}
	woke
}

type Syscall = unsafe extern "C" fn(c_long, ...) -> c_long;

/// glibc's `syscall`, through which the `getrandom` crate's 0.2 releases read randomness and
/// std waits on a futex. On x86_64 a variadic call passes its first six arguments as a call with
/// six fixed ones does, so this reads them all, as glibc's own does.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall(
	n: c_long,
	a: c_long,
	b: c_long,
	c: c_long,
	d: c_long,
	e: c_long,
	f: c_long,
) -> c_long {
	let real = real!("syscall": Syscall);
	match n {
		// SAFETY: the caller's arguments, as getrandom takes them.
		SYS_GETRANDOM => (unsafe { getrandom(a as *mut c_void, b as usize, c as c_uint) }) as c_long,
		// SAFETY: the caller's arguments, as clock_gettime takes them.
		SYS_CLOCK_GETTIME => (unsafe { clock_gettime(a as c_int, b as *mut Timespec) }) as c_long,
		SYS_FUTEX if d != 0 => {
			// SAFETY: the caller's arguments, as the futex call takes them.
			unsafe { futex(real, a, b, c, d as *const Timespec, e, f) }
		}
		// SAFETY: the caller's arguments, as it passed them.
		_ => unsafe { real(n, a, b, c, d, e, f) },
	}
}

/// A futex call with a timeout: a wait's deadline moves onto the host's clock, and a wait that
/// expires moves this one.
unsafe fn futex(
	real: Syscall,
	addr: c_long,
	op: c_long,
	val: c_long,
	at: *const Timespec,
	e: c_long,
	f: c_long,
) -> c_long {
	let cmd = op & !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);
	// SAFETY: the caller's arguments, as it passed them.
	let pass_through = || unsafe { real(SYS_FUTEX, addr, op, val, at as c_long, e, f) };
	match cmd {
		FUTEX_WAIT => {
			let woke = pass_through();
			if woke == -1 && errno() == ETIMEDOUT {
				// SAFETY: the kernel read the timeout, so it is a timespec.
				pass(unsafe { (*at).ns() }.unwrap_or(0));
			}
			woke
		}
		FUTEX_WAIT_BITSET => {
			let (clock, base) = if op & FUTEX_CLOCK_REALTIME != 0 {
				(CLOCK_REALTIME, REALTIME)
			} else {
				(CLOCK_MONOTONIC, MONOTONIC)
			};
			// SAFETY: a wait's timeout is a timespec to read.
			let at = unsafe { &*at };
			let Some(ahead) = ahead(base, at) else {
				return pass_through();
			};
			let host = host_deadline(clock, ahead);
			// SAFETY: the caller's arguments, with the deadline on the host's clock.
			let woke = unsafe { real(SYS_FUTEX, addr, op, val, &host as *const Timespec as c_long, e, f) };
			if woke == -1 && errno() == ETIMEDOUT {
				reach(base, at);
			}
			woke
		}
		_ => pass_through(),
	}
}
