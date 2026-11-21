//! The runtime's `rt/thread.rs` and `rt/future.rs`, mounted as they are under loom: this
//! module stands in for `rt.rs`, giving them loom's locks, atomics, `Arc`, thread locals,
//! and threads in place of std's, and a panic where the runtime aborts, so loom reports it.
//! Its atomics are [`AtomicBool`], which loom explores every access of, and which a [`Flaw`]
//! makes wrong for `flaws_are_caught`.

use std::process::{Command, Stdio};
use std::sync::OnceLock as StdOnce;

pub use loom::sync::atomic::Ordering;
pub use loom::sync::{Arc, Mutex, MutexGuard};

/// loom's `AtomicBool`, which it also explores every thread's access around.
///
/// loom's DPOR keeps one last access per atomic, which a thread's own load overwrites, so a
/// load after another thread's never races the swap or store that follows it, and that
/// swap never runs after the other thread's load and before its next access
/// ([tokio-rs/loom#392](https://github.com/tokio-rs/loom/issues/392)). Each access here
/// first takes a relaxed read-modify-write on one gate every atomic shares, which DPOR sees
/// conflict with every other thread's, so it tries each order of the accesses within the
/// preemption bound, as `hold.rs`'s `Atom` does.
pub struct AtomicBool(loom::sync::atomic::AtomicBool);

loom::lazy_static! {
	/// The one gate, made anew each run as loom's statics are.
	static ref GATE: loom::sync::atomic::AtomicUsize = loom::sync::atomic::AtomicUsize::new(0);
}

impl AtomicBool {
	pub fn new(value: bool) -> Self {
		Self(loom::sync::atomic::AtomicBool::new(value))
	}

	fn step() {
		GATE.fetch_add(0, Ordering::Relaxed);
	}

	pub fn load(&self, order: Ordering) -> bool {
		Self::step();
		match Flaw::current() {
			Some(Flaw::LoadsClear) => false,
			Some(Flaw::LoadsSet) => true,
			_ => self.0.load(order),
		}
	}

	pub fn store(&self, value: bool, order: Ordering) {
		Self::step();
		match Flaw::current() {
			Some(Flaw::StoresSetSkipped) if value => {}
			Some(Flaw::StoresClearSkipped) if !value => {}
			_ => self.0.store(value, order),
		}
	}

	pub fn swap(&self, value: bool, order: Ordering) -> bool {
		Self::step();
		match Flaw::current() {
			// Reads the flag and leaves it as it was.
			Some(Flaw::SwapsKeepFlag) => self.0.fetch_and(true, order),
			Some(Flaw::SwapsSeeSet) => {
				self.0.swap(value, order);
				true
			}
			_ => self.0.swap(value, order),
		}
	}
}

/// A way the runtime's atomics can be made wrong, which the models must each fail on. They
/// are the runtime's own source, mounted as it is, so the atomics carry the flaws and not
/// the runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flaw {
	/// A swap reads the flag and leaves it as it was: a push never marks its thread's queue
	/// as pending, and a poll never claims its future.
	SwapsKeepFlag,
	/// A swap finds the flag already set: a push is never the first, so wakes nothing, and
	/// every poll finds another in progress.
	SwapsSeeSet,
	/// A load finds the flag clear: a check never finds a drop queued, and a wait never finds
	/// its call run.
	LoadsClear,
	/// A load finds the flag set: a check drains a queue that has nothing, and a wait ends
	/// before its call ran.
	LoadsSet,
	/// A store of `true` does nothing: a call a home ran never says so.
	StoresSetSkipped,
	/// A store of `false` does nothing: a drained queue stays pending, and a poll's claim of
	/// its future is never let go of.
	StoresClearSkipped,
}

impl Flaw {
	pub const ALL: &[Flaw] = &[
		Flaw::SwapsKeepFlag,
		Flaw::SwapsSeeSet,
		Flaw::LoadsClear,
		Flaw::LoadsSet,
		Flaw::StoresSetSkipped,
		Flaw::StoresClearSkipped,
	];

	/// The flaw this process is run with, by the `RT_FLAW` that [`flaws_are_caught`] sets
	/// for the process it runs a model in.
	fn current() -> Option<Flaw> {
		static CURRENT: StdOnce<Option<Flaw>> = StdOnce::new();
		*CURRENT.get_or_init(|| {
			let name = std::env::var("RT_FLAW").ok()?;
			Some(
				*Flaw::ALL
					.iter()
					.find(|flaw| format!("{flaw:?}") == name)
					.unwrap_or_else(|| panic!("no flaw {name}")),
			)
		})
	}
}

/// The models do not pass vacuously: each of `caught` fails on its flaw.
///
/// A model that fails with a thread of its own still parked aborts the process, which
/// loom's failure is, so each runs alone in a process of this test's binary with its flaw
/// set. A model that passes, or a name this binary has no test of, which passes by running
/// none, is missed.
pub fn flaws_are_caught(caught: &[(Flaw, &[&str])]) {
	let exe = std::env::current_exe().unwrap();
	let listed = Command::new(&exe).arg("--list").output().unwrap();
	let listed = String::from_utf8(listed.stdout).unwrap();
	for flaw in Flaw::ALL {
		let models: Vec<_> = caught.iter().filter(|(f, _)| f == flaw).collect();
		assert_eq!(models.len(), 1, "{flaw:?} has {} entries", models.len());
		assert!(!models[0].1.is_empty(), "{flaw:?} names no model");
	}
	for (flaw, names) in caught {
		for name in *names {
			assert!(
				listed.lines().any(|line| line == format!("{name}: test")),
				"{flaw:?} names no model {name}"
			);
		}
	}
	let runs: Vec<_> = caught
		.iter()
		.flat_map(|(flaw, names)| names.iter().map(move |name| (*flaw, *name)))
		.map(|(flaw, name)| {
			let run = Command::new(&exe)
				.args(["--exact", name])
				.env("RT_FLAW", format!("{flaw:?}"))
				.stdin(Stdio::null())
				.stdout(Stdio::null())
				.stderr(Stdio::null())
				.spawn()
				.unwrap();
			(flaw, name, run)
		})
		.collect();
	let missed: Vec<_> = runs
		.into_iter()
		.filter_map(|(flaw, name, mut run)| run.wait().unwrap().success().then_some((flaw, name)))
		.collect();
	assert!(missed.is_empty(), "no failure from {missed:?}");
}

/// loom's thread locals, which it drops as each of its threads ends, as std's do. loom's
/// macro takes no `const` initializer, which is only std's hint.
macro_rules! thread_local {
	(static $name:ident: $t:ty = const { $($init:tt)* }) => {
		loom::thread_local!(static $name: $t = { $($init)* });
	};
}

#[path = "../../runtime/rt/thread.rs"]
#[allow(
	dead_code,
	unexpected_cfgs,
	reason = "the models drive part of what the runtime uses, and the runtime's own features are not this crate's"
)]
pub mod thread;

#[path = "../../runtime/rt/future.rs"]
#[allow(dead_code, reason = "the models drive part of what the runtime uses")]
pub mod future;

/// The runtime's refusals, of which threads and futures return this one.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i32)]
pub enum Status {
	NotSend = 7,
}

/// Where the runtime aborts: a panic, which fails the model.
pub fn fail(what: &str, msg: &str) -> ! {
	panic!("{what}: {msg}")
}

/// Write `status` through `out`, returning a zeroed value, as the runtime's `failed` does.
///
/// # Safety
/// `out` is null or points where an `int32_t` goes, and a zeroed `R` is a valid value.
pub unsafe fn failed<R>(out: *mut i32, status: Status, what: &str) -> R {
	if out.is_null() {
		fail(what, "a status is null");
	}
	// SAFETY: the caller's contract.
	unsafe {
		out.write(status as i32);
		std::mem::zeroed()
	}
}

loom::lazy_static! {
	/// The runtime's one lock in a static, its idle homes, made anew each run as loom's
	/// statics are.
	static ref IDLE: Mutex<Vec<Arc<thread::Thread>>> = Mutex::new(Vec::new());
}

/// The idle homes' lock, for the runtime's `static`, which [`idle`] reads.
pub struct StaticMutex<T>(std::marker::PhantomData<T>);

impl<T> StaticMutex<T> {
	pub const fn new(value: T) -> Self {
		// The runtime's static starts empty, as loom's does.
		std::mem::forget(value);
		Self(std::marker::PhantomData)
	}

	pub fn lock(&self) -> std::sync::LockResult<MutexGuard<'static, Vec<Arc<thread::Thread>>>> {
		IDLE.lock()
	}
}

/// How many times the idle homes hold `home`.
pub fn idle(home: *const thread::Thread) -> usize {
	IDLE.lock()
		.unwrap()
		.iter()
		.filter(|idle| Arc::as_ptr(idle) == home)
		.count()
}

/// std's `OnceLock`, as far as the runtime uses it: set once, then read from any thread.
pub struct OnceLock<T> {
	/// 0 while empty, 1 while one sets it, 2 once set.
	state: loom::sync::atomic::AtomicUsize,
	value: loom::cell::UnsafeCell<Option<T>>,
}

// SAFETY: the value is written once, before the release that publishes it, and only read
// after.
unsafe impl<T: Send + Sync> Sync for OnceLock<T> {}

impl<T> OnceLock<T> {
	pub fn new() -> Self {
		Self {
			state: loom::sync::atomic::AtomicUsize::new(0),
			value: loom::cell::UnsafeCell::new(None),
		}
	}

	pub fn set(&self, value: T) -> Result<(), T> {
		if self
			.state
			.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
			.is_err()
		{
			return Err(value);
		}
		// SAFETY: the exchange above makes this the only writer, and no reader reads before
		// the release below.
		self.value.with_mut(|slot| unsafe { *slot = Some(value) });
		self.state.store(2, Ordering::Release);
		Ok(())
	}

	pub fn get(&self) -> Option<&T> {
		if self.state.load(Ordering::Acquire) != 2 {
			return None;
		}
		// SAFETY: set, and never written again.
		self.value.with(|slot| unsafe { (*slot).as_ref() })
	}
}

/// std's parking and threads, as far as the runtime uses them, with its homes, which serve
/// forever, made to stop once a model is done with them: loom fails a run that ends with a
/// thread still parked. A park is a token on loom's lock and condition variable, since
/// loom's own `unpark` also wakes a thread waiting on a lock, which then fails the model.
pub mod os {
	use loom::sync::atomic::{AtomicBool, Ordering};
	use loom::sync::{Arc, Condvar, Mutex};
	use std::cell::OnceCell;
	use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

	/// A thread to unpark.
	#[derive(Clone)]
	pub struct Thread(Arc<Parker>);

	struct Parker {
		token: Mutex<bool>,
		unparked: Condvar,
	}

	impl Thread {
		fn new() -> Self {
			Self(Arc::new(Parker {
				token: Mutex::new(false),
				unparked: Condvar::new(),
			}))
		}

		pub fn unpark(&self) {
			*self.0.token.lock().unwrap() = true;
			self.0.unparked.notify_one();
		}
	}

	loom::thread_local! {
		static ME: OnceCell<Thread> = OnceCell::new();
	}

	loom::lazy_static! {
		/// The homes this run spawned, which [`stop`] ends.
		static ref SPAWNED: Mutex<Vec<(Thread, loom::thread::JoinHandle<()>)>> = Mutex::new(Vec::new());
		/// Set once [`stop`] ends the homes: a park then unwinds out of the home.
		static ref STOP: AtomicBool = AtomicBool::new(false);
	}

	/// This thread, to unpark.
	pub fn current() -> Thread {
		ME.with(|me| me.get_or_init(Thread::new).clone())
	}

	/// What unwinds a stopped home out of its loop.
	struct Stopped;

	/// Wait for an unpark since the last park, or end the home if [`stop`] asked.
	pub fn park() {
		let me = current();
		let mut token = me.0.token.lock().unwrap();
		while !*token {
			token = me.0.unparked.wait(token).unwrap();
		}
		*token = false;
		drop(token);
		if STOP.load(Ordering::SeqCst) {
			resume_unwind(Box::new(Stopped));
		}
	}

	/// End every home this run spawned, which must be idle: each wakes, unwinds out of its
	/// loop, and exits, dropping its thread locals.
	pub fn stop() {
		STOP.store(true, Ordering::SeqCst);
		let spawned = std::mem::take(&mut *SPAWNED.lock().unwrap());
		for (thread, home) in spawned {
			thread.unpark();
			home.join().unwrap();
		}
	}

	/// std's `thread::Builder`, as far as the runtime uses it.
	pub struct Builder;

	impl Builder {
		pub fn new() -> Self {
			Self
		}

		pub fn name(self, _name: String) -> Self {
			self
		}

		pub fn spawn(self, f: impl FnOnce() + Send + 'static) -> std::io::Result<Spawned> {
			let thread = Thread::new();
			let me = thread.clone();
			let home = loom::thread::spawn(move || {
				ME.with(|cell| cell.set(me).ok().expect("a new thread"));
				if let Err(panic) = catch_unwind(AssertUnwindSafe(f))
					&& !panic.is::<Stopped>()
				{
					resume_unwind(panic);
				}
			});
			SPAWNED.lock().unwrap().push((thread.clone(), home));
			Ok(Spawned(thread))
		}
	}

	/// std's `JoinHandle`, as far as the runtime uses it.
	pub struct Spawned(Thread);

	impl Spawned {
		pub fn thread(&self) -> &Thread {
			&self.0
		}
	}
}
