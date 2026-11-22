//! A model check of the futures C polls and the wakers it polls them with,
//! `runtime/rt/future.rs`, under loom: every interleaving of a wake from another thread
//! against a poll, a change of waker, a start, and a free, and of a future that is not
//! `Send` polled and freed from another thread, and those within a preemption bound of one
//! made on a home, and of two polls of one future, one of which must be refused. Each C waker
//! must be dropped once, never woken after, and woken whenever a poll pended before the
//! value it waited for arrived; each future must drop once, on its own thread. Every access
//! of the runtime's atomics is explored, `rt::AtomicBool` making it so, and
//! `flaws_are_caught` has each model fail on the atomics made wrong.

#[path = "loom/rt.rs"]
mod rt;

use loom::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use loom::sync::{Arc, Mutex};
use loom::thread::{self, ThreadId, yield_now};
use rt::future::{Future, PENDING, READY, Waker};
use rt::thread::Thread;
use std::ffi::c_void;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Wait out another thread's step, which loom must schedule. Past the first wait, loom
/// explores no other order of the threads, which would only repeat the wait.
fn relax(spins: &mut u32) {
	*spins += 1;
	if *spins > 1 {
		loom::skip_branch();
	}
	yield_now();
}

/// A C waker's counts: wakes, and drops, after which a wake fails the model.
#[derive(Default)]
struct Counts {
	wakes: AtomicUsize,
	drops: AtomicUsize,
}

unsafe extern "C" fn wake(data: *mut c_void) {
	// SAFETY: `waker`'s data, which outlives the model's futures.
	let counts = unsafe { &*data.cast::<Counts>() };
	assert_eq!(counts.drops.load(SeqCst), 0, "a waker woken after its drop");
	counts.wakes.fetch_add(1, SeqCst);
}

unsafe extern "C" fn drop_waker(data: *mut c_void) {
	// SAFETY: as above.
	let counts = unsafe { &*data.cast::<Counts>() };
	assert_eq!(counts.drops.fetch_add(1, SeqCst), 0, "a waker dropped twice");
}

/// The C waker of `counts`, as C passes it to each poll.
fn waker(counts: &Arc<Counts>) -> Waker {
	Waker {
		wake: Some(wake),
		drop: Some(drop_waker),
		data: Arc::as_ptr(counts).cast_mut().cast(),
	}
}

/// A one-slot channel, as moq's tracks are: a receive registers its waker until a send.
#[derive(Default)]
struct Slot {
	value: Option<u32>,
	waker: Option<std::task::Waker>,
}

type Channel = Arc<Mutex<Slot>>;

fn send(channel: &Channel, value: u32) {
	let waker = {
		let mut slot = channel.lock().unwrap();
		slot.value = Some(value);
		slot.waker.take()
	};
	if let Some(waker) = waker {
		waker.wake();
	}
}

/// A receive, which counts its drop, and for one that is not `Send`, checks it is on the
/// thread that made it.
struct Recv {
	channel: Channel,
	owner: Option<ThreadId>,
	dropped: Arc<AtomicUsize>,
}

impl Recv {
	fn new(channel: &Channel, dropped: &Arc<AtomicUsize>) -> Self {
		Self {
			channel: channel.clone(),
			owner: None,
			dropped: dropped.clone(),
		}
	}

	/// One that is not `Send`, staying on this thread.
	fn local(channel: &Channel, dropped: &Arc<AtomicUsize>) -> Self {
		let mut recv = Self::new(channel, dropped);
		recv.owner = Some(thread::current().id());
		recv
	}
}

impl std::future::Future for Recv {
	type Output = u32;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
		let mut slot = self.channel.lock().unwrap();
		match slot.value.take() {
			Some(value) => Poll::Ready(value),
			None => {
				slot.waker = Some(cx.waker().clone());
				Poll::Pending
			}
		}
	}
}

impl Drop for Recv {
	fn drop(&mut self) {
		if let Some(owner) = self.owner {
			assert_eq!(thread::current().id(), owner, "a future dropped off its thread");
		}
		self.dropped.fetch_add(1, SeqCst);
	}
}

/// The future handle of a receive, whose result C reads through its one out pointer.
fn recv(channel: &Channel, dropped: &Arc<AtomicUsize>) -> *mut Future {
	Future::new(Recv::new(channel, dropped), lower)
}

fn lower(value: u32, outs: &[*mut c_void], _err: *mut c_void) -> i32 {
	// SAFETY: the poll's out pointer, a `u32`.
	unsafe { outs[0].cast::<u32>().write(value) };
	READY
}

/// Poll `future` with `waker`, reading its value once ready.
fn poll(future: *mut Future, waker: Waker) -> Option<u32> {
	let mut out = 0u32;
	let outs = [(&raw mut out).cast::<c_void>()];
	// SAFETY: a live future handle, polled one call at a time, and its result a `u32`.
	match unsafe { Future::poll(future, waker, &outs, std::ptr::null_mut(), std::ptr::null_mut(), "poll") } {
		PENDING => None,
		READY => Some(out),
		other => panic!("poll returned {other}"),
	}
}

/// Every interleaving of `f`, or those with at most `preemptions` switches away from a
/// thread that could have run on, where a home's threads make every interleaving too many
/// for the gate.
fn model(preemptions: Option<usize>, f: impl Fn() + Sync + Send + 'static) {
	let mut builder = loom::model::Builder::new();
	builder.preemption_bound = preemptions;
	builder.check(f);
}

/// A send on another thread races the poll: a poll that pends is woken, the next is ready,
/// and the waker C gave once is dropped once, by whichever thread lets go of it last.
#[test]
fn wake_races_poll() {
	model(None, || {
		let channel = Channel::default();
		let counts = Arc::new(Counts::default());
		let dropped = Arc::new(AtomicUsize::new(0));
		let future = recv(&channel, &dropped);
		let sender = {
			let channel = channel.clone();
			thread::spawn(move || send(&channel, 7))
		};
		let first = poll(future, waker(&counts));
		sender.join().unwrap();
		let value = match first {
			Some(value) => value,
			None => {
				assert_eq!(counts.wakes.load(SeqCst), 1, "a pending poll was not woken");
				poll(future, waker(&counts)).expect("ready once woken")
			}
		};
		assert_eq!(value, 7);
		// SAFETY: live, and no longer polled.
		unsafe { Future::free(future) };
		assert_eq!(dropped.load(SeqCst), 1);
		assert_eq!(counts.drops.load(SeqCst), 1);
	});
}

/// A poll with a new waker replaces the last, which the sender may wake meanwhile: each is
/// dropped once, and one of them is woken if the second poll pended too.
#[test]
fn waker_replaced_while_woken() {
	model(None, || {
		let channel = Channel::default();
		let (first, second) = (Arc::new(Counts::default()), Arc::new(Counts::default()));
		let dropped = Arc::new(AtomicUsize::new(0));
		let future = recv(&channel, &dropped);
		let sender = {
			let channel = channel.clone();
			thread::spawn(move || send(&channel, 7))
		};
		let mut value = poll(future, waker(&first));
		let replaced = value.is_none();
		if replaced {
			value = poll(future, waker(&second));
		}
		sender.join().unwrap();
		if value.is_none() {
			assert_eq!(second.wakes.load(SeqCst), 1, "the latest waker was not woken");
			value = poll(future, waker(&second));
		}
		assert_eq!(value, Some(7));
		// SAFETY: as above.
		unsafe { Future::free(future) };
		assert_eq!(first.drops.load(SeqCst), 1);
		assert_eq!(second.drops.load(SeqCst), usize::from(replaced));
		assert_eq!(dropped.load(SeqCst), 1);
	});
}

/// A receive that never finishes, which fails the model if a poll of it starts before
/// another's ends.
struct Overlap {
	channel: Channel,
	inside: Arc<AtomicUsize>,
}

impl std::future::Future for Overlap {
	type Output = u32;

	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<u32> {
		assert_eq!(self.inside.fetch_add(1, SeqCst), 0, "two polls at once");
		// A lock, where loom may run the other poll.
		drop(self.channel.lock().unwrap());
		self.inside.fetch_sub(1, SeqCst);
		Poll::Pending
	}
}

/// Poll `future`, whether the runtime let the poll in: not if another was in the future.
fn poll_admitted(future: *mut Future, waker: Waker) -> bool {
	match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| poll(future, waker))) {
		Ok(polled) => {
			assert_eq!(polled, None, "a future that never finishes finished");
			true
		}
		Err(panic) => {
			let msg = panic.downcast_ref::<String>().map(String::as_str);
			assert!(
				msg.is_some_and(|msg| msg.ends_with("a future is polled concurrently")),
				"a poll failed otherwise: {msg:?}"
			);
			false
		}
	}
}

/// Two threads poll one future at once: the runtime lets one through and refuses the other,
/// or runs them one after the other, and never has both in the future's poll.
#[test]
fn polls_race() {
	model(None, || {
		let channel = Channel::default();
		let inside = Arc::new(AtomicUsize::new(0));
		let future = Future::new(Overlap { channel, inside }, lower);
		let (first, second) = (Arc::new(Counts::default()), Arc::new(Counts::default()));
		let other = {
			let handle = future as usize;
			let second = second.clone();
			// SAFETY: live until the free below, which follows the join.
			thread::spawn(move || poll_admitted(handle as *mut Future, waker(&second)))
		};
		let mine = poll_admitted(future, waker(&first));
		let theirs = other.join().unwrap();
		assert!(mine || theirs, "both polls were refused");
		// SAFETY: live, and no longer polled.
		unsafe { Future::free(future) };
	});
}

/// A free while a send on another thread wakes the pending poll's waker: the future drops
/// once, and its waker once, never woken after.
#[test]
fn free_races_wake() {
	model(None, || {
		let channel = Channel::default();
		let counts = Arc::new(Counts::default());
		let dropped = Arc::new(AtomicUsize::new(0));
		let future = recv(&channel, &dropped);
		let sender = {
			let channel = channel.clone();
			thread::spawn(move || send(&channel, 7))
		};
		let _ = poll(future, waker(&counts));
		// SAFETY: as above.
		unsafe { Future::free(future) };
		sender.join().unwrap();
		assert_eq!(dropped.load(SeqCst), 1);
		assert_eq!(counts.drops.load(SeqCst), 1);
	});
}

/// A start polls once with no waker, so a send landing meanwhile wakes nothing, and C's
/// first poll with its waker then finds the value or registers to be woken.
#[test]
fn start_then_poll() {
	model(None, || {
		let channel = Channel::default();
		let counts = Arc::new(Counts::default());
		let dropped = Arc::new(AtomicUsize::new(0));
		let future = recv(&channel, &dropped);
		let sender = {
			let channel = channel.clone();
			thread::spawn(move || send(&channel, 7))
		};
		let mut out = 0u32;
		let outs = [(&raw mut out).cast::<c_void>()];
		let mut handle = std::ptr::null_mut();
		// SAFETY: just made, and its result a `u32`.
		let started = unsafe { Future::start(future, &outs, std::ptr::null_mut(), &mut handle, "start") };
		let value = match started {
			READY => {
				assert!(handle.is_null(), "a ready start left a handle");
				sender.join().unwrap();
				out
			}
			PENDING => {
				assert_eq!(handle, future);
				let first = poll(handle, waker(&counts));
				sender.join().unwrap();
				let value = match first {
					Some(value) => value,
					None => {
						assert_eq!(counts.wakes.load(SeqCst), 1, "a pending poll was not woken");
						poll(handle, waker(&counts)).expect("ready once woken")
					}
				};
				// SAFETY: as above.
				unsafe { Future::free(handle) };
				assert_eq!(counts.drops.load(SeqCst), 1);
				value
			}
			other => panic!("start returned {other}"),
		};
		assert_eq!(value, 7);
		assert_eq!(dropped.load(SeqCst), 1);
	});
}

/// A receive that is not `Send`, made and polled on one thread, polled and freed on
/// another: the poll there fails, dropping only the waker it was given, and the free there
/// queues the drop to the future's thread, which runs it at its next check.
#[test]
fn unsent_polled_and_freed_elsewhere() {
	model(None, || {
		let owner = thread::spawn(|| {
			let channel = Channel::default();
			let (mine, theirs) = (Arc::new(Counts::default()), Arc::new(Counts::default()));
			let dropped = Arc::new(AtomicUsize::new(0));
			let future = Future::unsent(Recv::local(&channel, &dropped), lower);
			assert_eq!(poll(future, waker(&mine)), None);
			let handle = future as usize;
			let other = {
				let theirs = theirs.clone();
				thread::spawn(move || {
					let future = handle as *mut Future;
					let mut status = 0;
					let mut out = 0u32;
					let outs = [(&raw mut out).cast::<c_void>()];
					// SAFETY: live, and polled by no other thread meanwhile.
					let polled = unsafe {
						Future::poll(future, waker(&theirs), &outs, std::ptr::null_mut(), &mut status, "poll")
					};
					assert_eq!((polled, status), (PENDING, rt::Status::NotSend as i32));
					// SAFETY: live, and no longer polled.
					unsafe { Future::free(future) };
				})
			};
			let mut spins = 0;
			while dropped.load(SeqCst) == 0 {
				Thread::drain_here();
				relax(&mut spins);
			}
			other.join().unwrap();
			assert_eq!(theirs.drops.load(SeqCst), 1, "a refused poll kept its waker");
			// The channel holds the pending poll's waker until it drops.
			drop(channel);
			assert_eq!(mine.drops.load(SeqCst), 1);
			assert_eq!(theirs.wakes.load(SeqCst) + mine.wakes.load(SeqCst), 0);
		});
		owner.join().unwrap();
	});
}

/// A future that is not `Send`, made on a home, polls there from any thread, and its free
/// elsewhere drops it there; the home then goes idle.
#[test]
fn unsent_on_a_home() {
	model(Some(2), || {
		let home = Thread::home();
		let at = Arc::as_ptr(&home);
		let channel = Channel::default();
		let counts = Arc::new(Counts::default());
		let dropped = Arc::new(AtomicUsize::new(0));
		// SAFETY: a home; the handle is the caller's once made.
		let future = unsafe { home.run(|| Future::unsent(Recv::local(&channel, &dropped), lower)) };
		assert_eq!(poll(future, waker(&counts)), None);
		let sender = {
			let channel = channel.clone();
			thread::spawn(move || send(&channel, 7))
		};
		sender.join().unwrap();
		assert_eq!(counts.wakes.load(SeqCst), 1);
		assert_eq!(poll(future, waker(&counts)), Some(7));
		// SAFETY: live, and no longer polled.
		unsafe { Future::free(future) };
		Thread::release(home);
		let mut spins = 0;
		while rt::idle(at) == 0 {
			relax(&mut spins);
		}
		assert_eq!(dropped.load(SeqCst), 1);
		assert_eq!(counts.drops.load(SeqCst), 1);
		rt::os::stop();
	});
}

/// The models each fail on the atomics made wrong, so they do not pass vacuously.
#[test]
fn flaws_are_caught() {
	use rt::Flaw::*;
	rt::flaws_are_caught(&[
		(
			SwapsKeepFlag,
			&["polls_race", "unsent_polled_and_freed_elsewhere", "unsent_on_a_home"],
		),
		(
			SwapsSeeSet,
			&[
				"polls_race",
				"wake_races_poll",
				"waker_replaced_while_woken",
				"free_races_wake",
				"start_then_poll",
			],
		),
		(LoadsClear, &["unsent_polled_and_freed_elsewhere", "unsent_on_a_home"]),
		(LoadsSet, &["unsent_on_a_home"]),
		(StoresSetSkipped, &["unsent_on_a_home"]),
		(
			StoresClearSkipped,
			&["wake_races_poll", "waker_replaced_while_woken", "start_then_poll"],
		),
	]);
}
