//! The threads values that are not `Send` stay on. Such a value, an object or a future,
//! records the thread that made it, which alone borrows, polls, moves, or drops it: a use
//! on another thread fails its check, and a free there queues the drop to its thread,
//! which runs it at its next check, when it exits, or at once if it waits for work. One
//! freed once its thread exited leaks, saying so, as nothing may drop it.
//!
//! A home is a thread Rust owns, for a binding with no thread of its own that stays put,
//! as a goroutine or a Dart isolate moves between threads: a call it makes on a value a
//! home owns runs there while the caller waits, so the value never leaves its thread.

use std::cell::OnceCell;
use std::sync::OnceLock;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

/// A thread values that are not `Send` stay on.
pub struct Thread {
	/// Set while `jobs` holds one, so a check on the thread finds it with a load.
	pending: AtomicBool,
	jobs: Mutex<Jobs>,
	/// A home's thread, which parks for a job, unparked as one is queued.
	parked: OnceLock<std::thread::Thread>,
	/// Whether it is a home, which runs a call from any thread.
	home: bool,
}

/// What other threads left a thread to run.
struct Jobs {
	queue: Vec<Job>,
	/// Whether the thread exited, so that nothing queued runs.
	gone: bool,
}

enum Job {
	Drop(Dropped),
	/// A call another thread waits on, on a home.
	Run(Run),
}

// SAFETY: a job runs on the thread it is queued for alone, which its maker ensures.
unsafe impl Send for Job {}

/// A value that is not `Send`, freed elsewhere, to drop on its own thread: erased, and
/// with the record of its thread, which it keeps alive until it drops.
pub struct Dropped {
	data: *mut (),
	drop: unsafe fn(*mut ()),
	what: &'static str,
	owner: Arc<Thread>,
}

impl Dropped {
	/// `value`, `what` a leak names, to drop on `owner`'s thread.
	///
	/// # Safety
	/// `value` is `owner`'s: it may move to another thread only to drop there.
	pub unsafe fn new<T>(value: T, owner: Arc<Thread>, what: &'static str) -> Self {
		unsafe fn drop_box<T>(data: *mut ()) {
			// SAFETY: `new` boxed it, and it drops once.
			drop(unsafe { Box::from_raw(data.cast::<T>()) });
		}
		Self {
			data: Box::into_raw(Box::new(value)).cast(),
			drop: drop_box::<T>,
			what,
			owner,
		}
	}

	/// Queue it to its thread, or leak it, saying so, if that exited.
	pub fn send(self) {
		let owner = self.owner.clone();
		if let Err(Job::Drop(dropped)) = owner.push(Job::Drop(self)) {
			eprintln!(
				"rust2-ffi: {} leaks: it is not Send, and its thread exited before it was freed",
				dropped.what
			);
		}
	}
}

/// A call lent to a home, which another thread waits on.
struct Run {
	call: unsafe fn(*mut ()),
	data: *mut (),
	done: *const Done,
}

/// What a thread waiting on a call it lent a home waits for.
struct Done {
	ran: AtomicBool,
	waiter: std::thread::Thread,
}

impl Job {
	fn run(self) {
		match self {
			// SAFETY: queued for this thread, which owns it.
			Job::Drop(dropped) => unsafe { (dropped.drop)(dropped.data) },
			Job::Run(run) => {
				// A panic would leave the waiting thread waiting forever, so it aborts, as
				// one through the C ABI does.
				// SAFETY: the waiting thread keeps the call alive until `ran`.
				if std::panic::catch_unwind(AssertUnwindSafe(|| unsafe { (run.call)(run.data) })).is_err() {
					crate::rt::fail("a home", "a call panicked");
				}
				// SAFETY: as above; once `ran`, the waiter may return, so it is read first.
				let done = unsafe { &*run.done };
				let waiter = done.waiter.clone();
				done.ran.store(true, Ordering::Release);
				waiter.unpark();
			}
		}
	}
}

/// The record of the thread it is a field of, until the thread exits, then empty.
struct Here(OnceCell<Arc<Thread>>);

impl Drop for Here {
	fn drop(&mut self) {
		if let Some(thread) = self.0.take() {
			thread.exit();
		}
	}
}

thread_local!(static HERE: Here = const { Here(OnceCell::new()) });

/// The homes no value lives on, idle until a call takes one.
static IDLE: Mutex<Vec<Arc<Thread>>> = Mutex::new(Vec::new());

impl Thread {
	fn new(home: bool, gone: bool) -> Self {
		Self {
			pending: AtomicBool::new(false),
			jobs: Mutex::new(Jobs {
				queue: Vec::new(),
				gone,
			}),
			parked: OnceLock::new(),
			home,
		}
	}

	/// This thread's record, which a value it makes keeps. A thread past its thread locals'
	/// teardown has one that is gone, so a value it makes is used nowhere and leaks.
	pub fn here() -> Arc<Thread> {
		HERE.try_with(|here| here.0.get_or_init(|| Arc::new(Self::new(false, false))).clone())
			.unwrap_or_else(|_| Arc::new(Self::new(false, true)))
	}

	/// Whether this is its thread.
	pub fn is_here(&self) -> bool {
		HERE.try_with(|here| here.0.get().is_some_and(|thread| std::ptr::eq(&**thread, self)))
			.unwrap_or(false)
	}

	/// Whether this is its thread, so a value of it may be used, after dropping what other
	/// threads freed of it meanwhile.
	#[inline]
	pub fn own(&self) -> Result<(), crate::rt::Status> {
		if !self.is_here() {
			return Err(crate::rt::Status::NotSend);
		}
		if self.pending.load(Ordering::Acquire) {
			self.drain();
		}
		Ok(())
	}

	/// Whether it is a home on another thread, which a call on its values runs on.
	pub fn away(&self) -> bool {
		self.home && !self.is_here()
	}

	fn lock(&self) -> MutexGuard<'_, Jobs> {
		self.jobs.lock().unwrap_or_else(|e| e.into_inner())
	}

	/// Queue `job` for this thread, telling it, unless it exited.
	fn push(&self, job: Job) -> Result<(), Job> {
		let mut jobs = self.lock();
		if jobs.gone {
			return Err(job);
		}
		jobs.queue.push(job);
		self.pending.store(true, Ordering::Release);
		drop(jobs);
		if let Some(thread) = self.parked.get() {
			thread.unpark();
		}
		Ok(())
	}

	/// Run what other threads queued for this one, on it.
	#[cold]
	fn drain(&self) {
		let queue = {
			let mut jobs = self.lock();
			self.pending.store(false, Ordering::Relaxed);
			std::mem::take(&mut jobs.queue)
		};
		queue.into_iter().for_each(Job::run);
	}

	/// The thread exited: drop what was queued, and leak what is freed later.
	fn exit(&self) {
		let queue = {
			let mut jobs = self.lock();
			jobs.gone = true;
			std::mem::take(&mut jobs.queue)
		};
		queue.into_iter().for_each(Job::run);
	}

	/// A home no value lives on: an idle one, or a new thread. [`Self::release`] gives it
	/// back once the call that took it returns.
	pub fn home() -> Arc<Thread> {
		if let Some(home) = IDLE.lock().unwrap_or_else(|e| e.into_inner()).pop() {
			return home;
		}
		let home = Arc::new(Self::new(true, false));
		let serving = home.clone();
		let spawned = std::thread::Builder::new()
			.name("rust2-ffi home".to_string())
			.spawn(move || serving.serve());
		if spawned.is_err() {
			crate::rt::fail("a home", "its thread could not be spawned");
		}
		home
	}

	/// Give back a home [`Self::home`] took: idle at once if no value lives on it, else
	/// once the last that does is dropped.
	pub fn release(home: Arc<Thread>) {
		// The home's own thread holds the other count.
		if Arc::strong_count(&home) == 2 {
			IDLE.lock().unwrap_or_else(|e| e.into_inner()).push(home);
		}
	}

	/// A home's thread: run each job queued for it, forever. Idle with no value living on
	/// it, which a value's record would count.
	fn serve(self: Arc<Self>) {
		let me: *const Self = &*self;
		let _ = self.parked.set(std::thread::current());
		HERE.with(|here| {
			let _ = here.0.set(self);
		});
		// SAFETY: this thread's record holds it for as long as the thread runs, which is forever.
		let me = unsafe { &*me };
		loop {
			if !me.pending.load(Ordering::Acquire) {
				std::thread::park();
				continue;
			}
			me.drain();
			HERE.with(|here| {
				let record = here.0.get().expect("a home's record is set");
				if Arc::strong_count(record) == 1 {
					IDLE.lock().unwrap_or_else(|e| e.into_inner()).push(record.clone());
				}
			});
		}
	}

	/// Wait until `done`, running each call queued meanwhile if this thread is a home, as one
	/// a call it waits on makes may be: false at once if it is none, to wait as it would.
	/// Whatever makes `done` true then unparks this thread.
	#[cfg(feature = "dart")]
	pub fn serve_until(done: impl Fn() -> bool) -> bool {
		let home = HERE.try_with(|here| here.0.get().filter(|thread| thread.home).cloned()).ok().flatten();
		let Some(home) = home else {
			return false;
		};
		while !done() {
			match home.pending.load(Ordering::Acquire) {
				true => home.drain(),
				false => std::thread::park(),
			}
		}
		true
	}

	/// Run `f` on this home's thread, which this one waits for: at once if this is it.
	///
	/// # Safety
	/// It is a home, and what `f` captures may be used on its thread while this waits.
	pub unsafe fn run<R>(&self, f: impl FnOnce() -> R) -> R {
		if self.is_here() {
			return f();
		}
		let mut out = MaybeUninit::uninit();
		let mut call = ManuallyDrop::new(|| {
			out.write(f());
		});
		unsafe fn invoke<F: FnOnce()>(data: *mut ()) {
			// SAFETY: `run`'s caller waits until it ran, once.
			unsafe { ManuallyDrop::take(&mut *data.cast::<ManuallyDrop<F>>())() }
		}
		fn invoker<F: FnOnce()>(_: &ManuallyDrop<F>) -> unsafe fn(*mut ()) {
			invoke::<F>
		}
		let done = Done {
			ran: AtomicBool::new(false),
			waiter: std::thread::current(),
		};
		let run = Run {
			call: invoker(&call),
			data: std::ptr::from_mut(&mut call).cast(),
			done: &done,
		};
		if self.push(Job::Run(run)).is_err() {
			crate::rt::fail("a home", "its thread exited");
		}
		while !done.ran.load(Ordering::Acquire) {
			std::thread::park();
		}
		// SAFETY: it ran, writing `out`.
		unsafe { out.assume_init() }
	}
}
