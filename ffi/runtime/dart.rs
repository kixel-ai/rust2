//! The runtime the `dart` feature builds for `dart/<prefix>.dart`: what an isolate needs
//! of Rust that `dart:ffi` lacks.
//!
//! A wake and a dropped waker each reach the isolate as an integer posted to its port,
//! which never blocks and which a port closed with its isolate drops. Dart runs code only
//! on the thread in its isolate, so a call into Dart from any other thread, and the drop of
//! an implementation there, waits in the isolate's queue for it to run: from its event
//! loop, which the port tells of each, or, while it waits in a call from Dart, from that
//! wait. A call waits so only while Rust holds an implementation or a lent closure of the
//! isolate, any of which another thread may call: then it runs on a thread of its own.

use std::cell::Cell;
use std::collections::VecDeque;
use std::ffi::{CStr, c_char, c_void};
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, OnceLock};

use crate::rt;

/// What an integer posted to an isolate's port asks, in its low two bits.
const WAKE: i64 = 0;
const DROP: i64 = 1;
const RUN: i64 = 2;

/// What Rust calls of Dart's API.
struct Api {
	post: unsafe extern "C" fn(i64, i64) -> bool,
	isolate: unsafe extern "C" fn() -> *mut c_void,
}

static API: OnceLock<Api> = OnceLock::new();

fn api() -> &'static Api {
	API.get().unwrap_or_else(|| rt::fail("dart", "Dart's API is not initialized"))
}

/// `NativeApi.initializeApiDLData`'s table, as `dart_api_dl.c` reads it.
#[repr(C)]
struct Table {
	major: i32,
	minor: i32,
	functions: *const Entry,
}

#[repr(C)]
struct Entry {
	name: *const c_char,
	function: *const c_void,
}

/// Find what Rust calls of Dart's API in `NativeApi.initializeApiDLData`, returning
/// whether its major version is the 2 this was written against.
///
/// # Safety
/// `data` is `NativeApi.initializeApiDLData`.
pub unsafe fn init(data: *const c_void) -> bool {
	// SAFETY: the caller's contract.
	let table = unsafe { &*data.cast::<Table>() };
	if table.major != 2 {
		return false;
	}
	let find = |name: &CStr| {
		let mut entry = table.functions;
		loop {
			// SAFETY: the table ends at an entry with a null name.
			let e = unsafe { &*entry };
			if e.name.is_null() {
				return None;
			}
			// SAFETY: as above, each name is a C string.
			if unsafe { CStr::from_ptr(e.name) } == name {
				return Some(e.function);
			}
			// SAFETY: as above.
			entry = unsafe { entry.add(1) };
		}
	};
	let (Some(post), Some(isolate)) = (find(c"Dart_PostInteger"), find(c"Dart_CurrentIsolate")) else {
		return false;
	};
	// SAFETY: Dart's API declares them so.
	let _ = API.set(unsafe {
		Api {
			post: std::mem::transmute::<*const c_void, unsafe extern "C" fn(i64, i64) -> bool>(post),
			isolate: std::mem::transmute::<*const c_void, unsafe extern "C" fn() -> *mut c_void>(isolate),
		}
	});
	true
}

/// An isolate: its port, and the calls into it other threads wait on. One lives for each
/// isolate that loads the library, and is never freed, since nothing tells Rust an
/// isolate exited.
pub struct Home {
	port: i64,
	isolate: usize,
	/// What tells Dart, on its thread, that Rust dropped the implementation of a token.
	dropped: unsafe extern "C" fn(i64),
	/// How many relays Rust holds: the implementations and lent closures of the isolate it
	/// may call from another thread.
	relays: AtomicUsize,
	tasks: Mutex<VecDeque<Task>>,
	ready: Condvar,
}

/// The isolate the calling thread is in, whose port is `port` and which `dropped` tells
/// of each implementation Rust drops on its thread.
pub fn home(port: i64, dropped: unsafe extern "C" fn(i64)) -> &'static Home {
	// SAFETY: a call from Dart.
	let isolate = unsafe { (api().isolate)() } as usize;
	Box::leak(Box::new(Home {
		port,
		isolate,
		dropped,
		relays: AtomicUsize::new(0),
		tasks: Mutex::default(),
		ready: Condvar::new(),
	}))
}

/// What the isolate runs on its thread for another.
enum Task {
	/// A call into Dart another thread waits on.
	Call(Arc<Job>),
	/// An implementation of this token Rust dropped, which nothing waits on, since a
	/// `NativeFinalizer` may drop it on a thread the garbage collector holds.
	Drop(i64),
}

impl Home {
	fn post(&self, message: i64) -> bool {
		// SAFETY: Dart may be posted to from any thread.
		unsafe { (api().post)(self.port, message) }
	}

	/// Whether the calling thread is in this isolate, so it may call into Dart.
	fn here(&self) -> bool {
		// SAFETY: Dart's API may be called from any thread.
		unsafe { (api().isolate)() as usize == self.isolate }
	}

	fn lock(&self) -> MutexGuard<'_, VecDeque<Task>> {
		self.tasks.lock().unwrap_or_else(|e| e.into_inner())
	}

	/// Queue `task` for a wait in a call from Dart to run, and tell the event loop to run
	/// it if none does first, returning whether the isolate is alive to.
	fn push(&self, task: Task) -> bool {
		self.lock().push_back(task);
		self.ready.notify_all();
		self.post(RUN)
	}

	/// Run `task` on the isolate's thread.
	///
	/// # Safety
	/// Called from Dart in this isolate.
	unsafe fn run(&self, task: Task) {
		match task {
			Task::Call(job) => job.run(),
			// SAFETY: the caller's contract.
			Task::Drop(token) => unsafe { (self.dropped)(token) },
		}
	}

	/// Run `f`, which calls into Dart, on this isolate's thread: at once if called there,
	/// or else as a job the isolate runs while this thread waits.
	///
	/// # Safety
	/// `f` only calls into Dart.
	pub unsafe fn call<R>(&self, what: &str, f: impl FnOnce() -> R) -> R {
		if self.here() {
			return f();
		}
		let mut f = Some(f);
		let mut out = None;
		let mut run = || out = Some((f.take().expect("a job runs once"))());
		let run: &mut (dyn FnMut() + '_) = &mut run;
		// SAFETY: this thread waits below until the job ran, so `run` outlives its use.
		let run: *mut (dyn FnMut() + 'static) = unsafe { std::mem::transmute(run) };
		let job = Arc::new(Job {
			run: Mutex::new(Some(Run(run))),
			thread: rt::thread(),
			done: Mutex::new(false),
			ran: Condvar::new(),
		});
		if !self.push(Task::Call(job.clone())) {
			rt::fail(what, "the Dart isolate it calls into has exited");
		}
		let mut done = job.done.lock().unwrap_or_else(|e| e.into_inner());
		while !*done {
			done = job.ran.wait(done).unwrap_or_else(|e| e.into_inner());
		}
		drop(done);
		out.expect("the job ran")
	}

	/// Run `f`, a call from Dart, at once if Rust holds no relay of this isolate, since
	/// then nothing it waits on can call into Dart. Else run it on a thread of its own while
	/// this one, the isolate's, runs each task queued until `f` returns. `f`'s thread acts
	/// as this one does for the handles it borrows, as does this one for each job's.
	///
	/// # Safety
	/// Called from Dart in this isolate, and `f` is a call through the C ABI.
	pub unsafe fn serve<R>(&self, f: impl FnOnce() -> R) -> R {
		// Only this thread makes a relay, so none appears before `f` returns.
		if self.relays.load(Ordering::Acquire) == 0 {
			return f();
		}
		let caller = rt::thread();
		let done = AtomicBool::new(false);
		let mut f = Some(f);
		let mut out = None;
		let mut run = || {
			ACTING.set(caller);
			let f = f.take().expect("a call runs once");
			out = Some(std::panic::catch_unwind(AssertUnwindSafe(f)));
			let _tasks = self.lock();
			done.store(true, Ordering::Release);
			self.ready.notify_all();
		};
		let run: &mut (dyn FnMut() + '_) = &mut run;
		// SAFETY: this thread waits below until the call ran, so `run` outlives its use.
		Worker::lend(Run(unsafe { std::mem::transmute(run) }));
		let mut tasks = self.lock();
		loop {
			if let Some(task) = tasks.pop_front() {
				drop(tasks);
				// SAFETY: the caller's contract.
				unsafe { self.run(task) };
				tasks = self.lock();
			} else if done.load(Ordering::Acquire) {
				break;
			} else {
				tasks = self.ready.wait(tasks).unwrap_or_else(|e| e.into_inner());
			}
		}
		drop(tasks);
		match out.expect("the call ran") {
			Ok(result) => result,
			Err(panic) => std::panic::resume_unwind(panic),
		}
	}
}

/// The workers parked for a call to run, shared by every isolate: a spawn per call costs
/// about three times a handoff to one.
static IDLE: Mutex<Vec<Arc<Worker>>> = Mutex::new(Vec::new());

/// A thread that runs a call from Dart while the isolate's thread serves.
struct Worker {
	call: Mutex<Option<Run>>,
	ready: Condvar,
}

impl Worker {
	/// Run `run` on an idle worker, or a new one if none is.
	fn lend(run: Run) {
		let idle = IDLE.lock().unwrap_or_else(|e| e.into_inner()).pop();
		let worker = idle.unwrap_or_else(|| {
			let worker = Arc::new(Worker {
				call: Mutex::new(None),
				ready: Condvar::new(),
			});
			let parked = worker.clone();
			std::thread::spawn(move || parked.park());
			worker
		});
		*worker.call.lock().unwrap_or_else(|e| e.into_inner()) = Some(run);
		worker.ready.notify_one();
	}

	/// Run each call lent to this worker, idle between them.
	fn park(self: Arc<Self>) {
		loop {
			let mut call = self.call.lock().unwrap_or_else(|e| e.into_inner());
			let Run(run) = loop {
				match call.take() {
					Some(run) => break run,
					None => call = self.ready.wait(call).unwrap_or_else(|e| e.into_inner()),
				}
			};
			drop(call);
			// SAFETY: the lending thread waits until it ran.
			unsafe { (*run)() };
			IDLE.lock().unwrap_or_else(|e| e.into_inner()).push(self.clone());
		}
	}
}

thread_local!(static ACTING: Cell<usize> = const { Cell::new(0) });

/// The thread this one acts as for the handles it borrows, if another: one waiting on it.
pub fn acting() -> Option<usize> {
	Some(ACTING.get()).filter(|&id| id != 0)
}

struct Run(*mut (dyn FnMut() + 'static));

// SAFETY: the thread that made it waits until it ran.
unsafe impl Send for Run {}

/// A call into Dart another thread waits on.
pub struct Job {
	run: Mutex<Option<Run>>,
	/// The waiting thread, which the isolate's acts as while it runs the call.
	thread: usize,
	done: Mutex<bool>,
	ran: Condvar,
}

impl Job {
	fn run(&self) {
		let run = self.run.lock().unwrap_or_else(|e| e.into_inner()).take();
		if let Some(Run(run)) = run {
			let was = ACTING.replace(self.thread);
			// SAFETY: the waiting thread keeps it alive until `done`.
			unsafe { (*run)() };
			ACTING.set(was);
		}
		*self.done.lock().unwrap_or_else(|e| e.into_inner()) = true;
		self.ran.notify_all();
	}
}

/// Run the task the isolate's port was told of, unless a wait in a call ran it already.
///
/// # Safety
/// Called from Dart in `home`'s isolate.
pub unsafe fn run(home: &Home) {
	let task = home.lock().pop_front();
	if let Some(task) = task {
		// SAFETY: the caller's contract.
		unsafe { home.run(task) };
	}
}

/// A waker posting its token to the isolate's port: to wake, and once dropped.
struct Wake {
	home: &'static Home,
	token: i64,
}

unsafe extern "C" fn wake(data: *mut c_void) {
	// SAFETY: `waker` made `data`, which `drop_wake` has not freed.
	let wake = unsafe { &*data.cast::<Wake>() };
	wake.home.post((wake.token << 2) | WAKE);
}

unsafe extern "C" fn drop_wake(data: *mut c_void) {
	// SAFETY: `waker` made `data`, freed once.
	let wake = unsafe { Box::from_raw(data.cast::<Wake>()) };
	wake.home.post((wake.token << 2) | DROP);
}

/// A waker for the isolate's `token`.
pub fn waker(home: &'static Home, token: i64) -> rt::Waker {
	rt::Waker {
		wake: Some(wake),
		drop: Some(drop_wake),
		data: Box::into_raw(Box::new(Wake { home, token })).cast(),
	}
}

/// The data Rust passes a closure or an implementation's method from Dart: the function,
/// or the table of functions, Dart made for it, and the token Dart passes it, which Rust
/// calls through on the isolate's thread.
#[repr(C)]
pub struct Relay {
	pub f: *const c_void,
	pub token: *mut c_void,
	pub home: &'static Home,
}

/// A relay for the isolate's `token`, which Dart frees after the call it lends it for, or
/// Rust once it drops the implementation it is the data of.
pub fn relay(home: &'static Home, f: *const c_void, token: i64) -> *mut Relay {
	home.relays.fetch_add(1, Ordering::Relaxed);
	Box::into_raw(Box::new(Relay {
		f,
		token: token as *mut c_void,
		home,
	}))
}

/// Free a relay lent for a call.
///
/// # Safety
/// `relay` came from [`relay`], freed once.
pub unsafe fn relay_free(relay: *mut Relay) {
	// SAFETY: the caller's contract.
	let relay = unsafe { Box::from_raw(relay) };
	relay.home.relays.fetch_sub(1, Ordering::Release);
}

/// An implementation's `free`: tell the isolate Rust dropped it, so its `close` runs before
/// the call that dropped it returns, on the isolate's thread or one the call serves, or
/// else from its event loop. A `NativeFinalizer` runs with no current isolate, so a drop it
/// makes never runs Dart at once.
///
/// # Safety
/// `relay` came from [`relay`], freed once.
pub unsafe extern "C" fn relay_drop(relay: *mut c_void) {
	// SAFETY: the caller's contract.
	let relay = unsafe { Box::from_raw(relay.cast::<Relay>()) };
	let token = relay.token as i64;
	let home = relay.home;
	home.relays.fetch_sub(1, Ordering::Release);
	if home.here() {
		// SAFETY: on the isolate's thread, in a call from Dart.
		return unsafe { (home.dropped)(token) };
	}
	// An isolate that exited has nothing left to close.
	home.push(Task::Drop(token));
}
