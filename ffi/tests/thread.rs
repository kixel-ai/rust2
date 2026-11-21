//! A model check of the threads values that are not `Send` stay on, `runtime/rt/thread.rs`,
//! under loom: every interleaving of a free on another thread against its thread's drain,
//! check, wake, and exit, and those within a preemption bound of calls lent to homes,
//! which call each other and go idle once no value lives on them. Each value records the thread
//! that drops it, which must be its own, once, unless it was freed after that thread exited.

#[path = "loom/rt.rs"]
mod rt;

use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering::SeqCst};
use loom::thread::{self, ThreadId, yield_now};
use rt::thread::{Dropped, Thread};

/// Wait out another thread's step, which loom must schedule. Past the first wait, loom
/// explores no other order of the threads, which would only repeat the wait.
fn relax(spins: &mut u32) {
	*spins += 1;
	if *spins > 1 {
		loom::skip_branch();
	}
	yield_now();
}

/// Where a value's drops are recorded, by plain counts loom does not track, so a value
/// that leaks leaks no loom object.
#[derive(Default)]
struct Drops {
	count: std::sync::atomic::AtomicUsize,
	/// Set by a drop on another thread than the value's own.
	stray: std::sync::atomic::AtomicBool,
}

impl Drops {
	fn new() -> std::sync::Arc<Self> {
		std::sync::Arc::default()
	}

	fn count(&self) -> usize {
		self.count.load(SeqCst)
	}

	fn assert_home(&self) {
		assert!(
			!self.stray.load(SeqCst),
			"a value dropped on another thread than its own"
		);
	}
}

/// A value that is not `Send`, made on `owner`'s thread, which records its drop, and drops
/// `then`, queued as a future's drop queues the value it borrowed.
struct Probe {
	owner: ThreadId,
	drops: std::sync::Arc<Drops>,
	then: Option<(Box<Probe>, Arc<Thread>)>,
}

loom::thread_local! {
	/// This thread's id, which a probe made here reads first, so a drop as the thread's
	/// locals are torn down, when loom's own `current` is gone, still finds it while it
	/// lasts.
	static ME: ThreadId = thread::current().id();
}

impl Probe {
	fn new(drops: &std::sync::Arc<Drops>) -> Self {
		Self {
			owner: ME.with(|me| *me),
			drops: drops.clone(),
			then: None,
		}
	}
}

impl Drop for Probe {
	fn drop(&mut self) {
		// Gone only as a thread that made a probe exits: its own queue's drops.
		if ME.try_with(|me| *me).is_ok_and(|me| me != self.owner) {
			self.drops.stray.store(true, SeqCst);
		}
		self.drops.count.fetch_add(1, SeqCst);
		if let Some((then, owner)) = self.then.take() {
			// SAFETY: `then` was made on `owner`'s thread, this one.
			unsafe { Dropped::new(then, owner, "a probe") }.send();
		}
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

/// A free on another thread queues the drop, which the value's thread runs at its next
/// check, whether the free lands before, during, or after the check.
#[test]
fn free_elsewhere_drops_on_check() {
	model(None, || {
		let owner = thread::spawn(|| {
			let drops = Drops::new();
			let probe = Probe::new(&drops);
			let here = Thread::here();
			let sent = Arc::new(AtomicBool::new(false));
			let freer = {
				let (here, sent) = (here.clone(), sent.clone());
				thread::spawn(move || {
					// SAFETY: the probe is `here`'s, and moves here only to be freed.
					unsafe { Dropped::new(probe, here, "a probe") }.send();
					sent.store(true, SeqCst);
				})
			};
			let mut spins = 0;
			// The free is queued before `sent`, so a check after it finds it.
			while !(sent.load(SeqCst) && drops.count() == 1) {
				here.own().unwrap();
				relax(&mut spins);
			}
			freer.join().unwrap();
			assert_eq!(drops.count(), 1);
			drops.assert_home();
		});
		owner.join().unwrap();
	});
}

/// A thread's exit runs what was queued for it, and what those drops queue in turn.
#[test]
fn exit_runs_the_queue() {
	model(None, || {
		let drops = Drops::new();
		let owner = {
			let drops = drops.clone();
			thread::spawn(move || {
				let here = Thread::here();
				let mut outer = Probe::new(&drops);
				outer.then = Some((Box::new(Probe::new(&drops)), here.clone()));
				let freer = thread::spawn(move || {
					// SAFETY: as above.
					unsafe { Dropped::new(outer, here, "a probe") }.send();
				});
				// Joined before exiting, so the free finds the thread live.
				freer.join().unwrap();
			})
		};
		owner.join().unwrap();
		assert_eq!(drops.count(), 2);
		drops.assert_home();
	});
}

/// A free racing its thread's exit drops the value there once, or leaks it if the thread
/// was gone, and never drops it on the freeing thread.
#[test]
fn free_races_exit() {
	model(None, || {
		let drops = Drops::new();
		let slot: Arc<loom::sync::Mutex<Option<Probe>>> = Arc::default();
		let record: Arc<loom::sync::Mutex<Option<Arc<Thread>>>> = Arc::default();
		let owner = {
			let (drops, slot, record) = (drops.clone(), slot.clone(), record.clone());
			thread::spawn(move || {
				*slot.lock().unwrap() = Some(Probe::new(&drops));
				*record.lock().unwrap() = Some(Thread::here());
			})
		};
		let freer = thread::spawn(move || {
			let mut spins = 0;
			let owner = loop {
				if let Some(owner) = record.lock().unwrap().take() {
					break owner;
				}
				relax(&mut spins);
			};
			let probe = slot.lock().unwrap().take().unwrap();
			// SAFETY: as above.
			unsafe { Dropped::new(probe, owner, "a probe") }.send()
		});
		owner.join().unwrap();
		let queued = freer.join().unwrap();
		// Queued, it drops as its thread exits; refused, it leaks. loom's join returns before
		// the thread's locals drop, where std's waits for them, so a queued drop is awaited.
		let mut spins = 0;
		while queued && drops.count() == 0 {
			relax(&mut spins);
		}
		assert_eq!(drops.count(), usize::from(queued));
		drops.assert_home();
	});
}

/// A free schedules a drain through the wake the thread's loop gave it, whether it lands
/// before or after the wake was given: the loop, which calls nothing else, drains it.
#[test]
fn wake_schedules_drain() {
	model(None, || {
		let owner = thread::spawn(|| {
			let drops = Drops::new();
			let probe = Probe::new(&drops);
			let here = Thread::here();
			let scheduled = Arc::new(AtomicUsize::new(0));
			let calls = Arc::new(AtomicUsize::new(0));
			let freer = thread::spawn(move || {
				// SAFETY: as above.
				unsafe { Dropped::new(probe, here, "a probe") }.send();
			});
			let wake = {
				let (scheduled, calls) = (scheduled.clone(), calls.clone());
				Arc::from_std(std::sync::Arc::new(move || {
					scheduled.fetch_add(1, SeqCst);
					calls.fetch_add(1, SeqCst);
				}) as std::sync::Arc<dyn Fn() + Send + Sync>)
			};
			Thread::wake_here(wake);
			let mut spins = 0;
			while drops.count() == 0 {
				if scheduled.swap(0, SeqCst) > 0 {
					Thread::drain_here();
				} else {
					relax(&mut spins);
				}
			}
			freer.join().unwrap();
			drops.assert_home();
			// One free, so one wake: from the push, or from `wake_here` finding it pending.
			assert_eq!(calls.load(SeqCst), 1);
		});
		owner.join().unwrap();
	});
}

/// Wait until `home` is idle, then check it is so once.
fn idles(home: *const Thread) {
	let mut spins = 0;
	while rt::idle(home) == 0 {
		relax(&mut spins);
	}
	assert_eq!(rt::idle(home), 1, "a home is idle twice");
}

/// Two threads lend calls to one home, which makes a value there: each call runs on the
/// home, the value stays on it, a free elsewhere drops it there, and the home goes idle
/// once the value and both calls are gone, whichever lets go of it last.
#[test]
fn calls_on_a_home() {
	model(Some(1), || {
		let home = Thread::home();
		let at = Arc::as_ptr(&home);
		let drops = Drops::new();
		let other = {
			let home = home.clone();
			thread::spawn(move || {
				// SAFETY: a home; the call captures nothing.
				let here = unsafe { home.run(|| Arc::as_ptr(&Thread::here())) };
				assert_eq!(here, at, "a call ran off its home");
				Thread::release(home);
			})
		};
		// SAFETY: a home; the probe and its record come back once the call returns.
		let (probe, owner) = unsafe { home.run(|| (Probe::new(&drops), Thread::here())) };
		assert_eq!(Arc::as_ptr(&owner), at);
		Thread::release(home);
		// SAFETY: the probe is the home's.
		unsafe { Dropped::new(probe, owner, "a probe") }.send();
		other.join().unwrap();
		idles(at);
		assert_eq!(drops.count(), 1);
		drops.assert_home();
		// The idle home is the one the next call takes.
		let again = Thread::home();
		assert_eq!(Arc::as_ptr(&again), at);
		Thread::release(again);
		rt::os::stop();
	});
}

/// A call on one home that calls another, which calls back the first, runs: a home waiting
/// on a call runs each call queued for it meanwhile.
#[test]
fn homes_call_each_other() {
	model(Some(1), || {
		let a = Thread::home();
		let b = Thread::home();
		let (at_a, at_b) = (Arc::as_ptr(&a), Arc::as_ptr(&b));
		assert_ne!(at_a, at_b);
		// SAFETY: homes; each call waits on the one it lends.
		let n = unsafe { a.run(|| b.run(|| a.run(|| Arc::as_ptr(&Thread::here())))) };
		assert_eq!(n, at_a);
		Thread::release(a);
		Thread::release(b);
		idles(at_a);
		idles(at_b);
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
			&["free_elsewhere_drops_on_check", "wake_schedules_drain", "calls_on_a_home", "homes_call_each_other"],
		),
		(SwapsSeeSet, &["wake_schedules_drain"]),
		(
			LoadsClear,
			&["free_elsewhere_drops_on_check", "wake_schedules_drain", "calls_on_a_home", "homes_call_each_other"],
		),
		(LoadsSet, &["wake_schedules_drain", "calls_on_a_home", "homes_call_each_other"]),
		(StoresSetSkipped, &["calls_on_a_home", "homes_call_each_other"]),
		(StoresClearSkipped, &["calls_on_a_home", "homes_call_each_other"]),
	]);
}
