//! A model check, under loom, of the hold count Go's `Handle_`, Kotlin's `__Handle`, and
//! Swift's class `Handle` keep beside the runtime's borrow check, each a port of `Hold`
//! below: a call holds the handle so no close frees it meanwhile, through a slot of the
//! runtime's table once calls contend, and a close that would free it claims it closed
//! first, then turns each hold it finds in the table into a count, so the last to let go
//! frees it. A checked close meeting another's one-crossing drop waits until Rust commits or
//! refuses that drop, which the commit signals before any `Drop` runs, so a `Drop` that
//! closes the value again never waits on itself.
//! Every interleaving within a preemption bound, [`Atom`] making it so: a hold
//! reads the handle, which a free writes, so loom fails the model on any free a hold could
//! race. A [`Flaw`] makes the port wrong in one place, and `flaws_are_caught` has each fail.

use loom::cell::UnsafeCell;
use loom::sync::Arc;
use loom::sync::atomic::Ordering::{Relaxed, SeqCst};
use loom::sync::atomic::{AtomicUsize, fence};
use loom::thread;

/// Set in the hold count once closed.
const CLOSED: usize = 1;
/// Set in the hold count once calls contended for it, after which they go to the table.
const BIAS: usize = 2;
/// Set beside [`CLOSED`] while a close decides, which may yet leave it open.
const CLOSING: usize = 4;
/// One hold in the count, past the bits above, so the count is below it only once no hold
/// is left.
const HELD: usize = 8;
/// Set in a slot whose hold a close turned into a count.
const REVOKED: usize = 1;
/// The handle's key in a slot: an address or id unique while the handle lives, its low bit
/// clear for [`REVOKED`].
const KEY: usize = 8;

/// A sequentially consistent atomic that loom also explores every thread's access around.
///
/// loom's DPOR keeps one last access per atomic, which a thread's own load overwrites, so
/// a load after another thread's never races the store or exchange that follows it, and
/// that exchange never runs after the other thread's load and before its next access: a
/// close never lands between a call's `load`, `compare_exchange`, and `fetch_sub` on the
/// count. Each access here first takes a relaxed read-modify-write on one `gate` all the
/// atomics share, which DPOR sees conflict with every other thread's, so it tries each
/// order of the accesses within the preemption bound.
struct Atom {
	value: AtomicUsize,
	gate: Arc<AtomicUsize>,
}

impl Atom {
	fn new(value: usize, gate: &Arc<AtomicUsize>) -> Self {
		Self {
			value: AtomicUsize::new(value),
			gate: gate.clone(),
		}
	}

	fn step(&self) {
		self.gate.fetch_add(0, Relaxed);
	}

	fn load(&self) -> usize {
		self.step();
		self.value.load(SeqCst)
	}

	fn store(&self, value: usize) {
		self.step();
		self.value.store(value, SeqCst);
	}

	fn swap(&self, value: usize) -> usize {
		self.step();
		self.value.swap(value, SeqCst)
	}

	fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize> {
		self.step();
		self.value.compare_exchange(current, new, SeqCst, SeqCst)
	}

	fn fetch_add(&self, value: usize) -> usize {
		self.step();
		self.value.fetch_add(value, SeqCst)
	}

	fn fetch_sub(&self, value: usize) -> usize {
		self.step();
		self.value.fetch_sub(value, SeqCst)
	}

	fn fetch_or(&self, value: usize) -> usize {
		self.step();
		self.value.fetch_or(value, SeqCst)
	}
}

/// A way the port can be made wrong, which its models must each fail on.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Flaw {
	/// A call on the count does not look for a close.
	HoldIgnoresClosed,
	/// A call in the table does not look again for a close that claimed it meanwhile.
	HoldSkipsRecheck,
	/// A call in the table publishes its slot without the fence ordering it before that look.
	HoldSkipsFence,
	/// A close frees a handle a call on the count holds.
	CloseIgnoresCount,
	/// A close frees a handle a call in the table holds.
	CloseIgnoresSlots,
	/// A close claims the handle without the fence ordering it before its look at the table.
	CloseSkipsFence,
	/// A move claims the handle without the fence ordering it before its look at the table.
	AbandonSkipsFence,
	/// A move does not turn the holds in the table into counts.
	AbandonSkipsSlots,
	/// A checked close's one-crossing drop claims a handle calls hold.
	DropFreeSkipsClaim,
	/// The last hold is let go of while the owner's remains.
	ReleaseFreesEarly,
	/// A checked close meeting another's one-crossing drop returns at once, though Rust may
	/// yet refuse that drop.
	CloseSkipsCommitWait,
	/// Rust's drop runs the value's `Drop` without storing the handle closed first.
	DropSkipsCommit,
}

/// A handle as a binding holds it, beside a table with a slot per column for its row.
struct Hold {
	state: Atom,
	slots: Vec<Atom>,
	/// What the handle points at: Rust's memory, which the free ends.
	raw: UnsafeCell<u64>,
	freed: AtomicUsize,
	/// Whether Rust took the value to drop, as its borrow state's `TAKEN`.
	taken: Atom,
	/// How many times the value's `Drop` ran.
	dropped: AtomicUsize,
	flaw: Option<Flaw>,
}

impl Hold {
	fn new(columns: usize, biased: bool, flaw: Option<Flaw>) -> Arc<Self> {
		let gate = Arc::new(AtomicUsize::new(0));
		Arc::new(Self {
			state: Atom::new(HELD | if biased { BIAS } else { 0 }, &gate),
			slots: (0..columns).map(|_| Atom::new(0, &gate)).collect(),
			raw: UnsafeCell::new(7),
			freed: AtomicUsize::new(0),
			taken: Atom::new(0, &gate),
			dropped: AtomicUsize::new(0),
			flaw,
		})
	}

	/// Rust's drop of the value, as `rt::Object::close` makes it: done once the value is
	/// gone, refused while `refused` has a borrow stand, and otherwise the value taken, then
	/// its `Drop`, `then`. For a close in one crossing, `commit`, the state stored closed, as
	/// the glue's commit does, once it can no longer fail, before any `Drop`. Whether it
	/// passed.
	fn rust_drop(&self, refused: bool, commit: bool, then: &dyn Fn()) -> bool {
		let commit = || {
			if commit && !self.flawed(Flaw::DropSkipsCommit) {
				self.state.store(CLOSED);
			}
		};
		if self.taken.load() != 0 {
			commit();
			return true;
		}
		if refused {
			return false;
		}
		if self.taken.swap(1) != 0 {
			commit();
			return true;
		}
		commit();
		self.dropped.fetch_add(1, SeqCst);
		then();
		true
	}

	/// Whether the port is made wrong this way.
	fn flawed(&self, flaw: Flaw) -> bool {
		self.flaw == Some(flaw)
	}

	/// Order the access before it ahead of the one after: loom treats a sequentially
	/// consistent access as acquire and release alone, so a fence stands for what Go's,
	/// Kotlin's, and Swift's atomics order.
	fn fence(&self, skipped: Flaw) {
		if !self.flawed(skipped) {
			fence(SeqCst);
		}
	}

	/// Hold the handle from `column`: the slot it went to, if any, or `None` once closed.
	fn hold(&self, column: usize) -> Option<Option<usize>> {
		// The fast path each binding inlines: the count, while calls neither contend for it
		// nor find it closed.
		let n = self.state.load();
		let closed = if self.flawed(Flaw::HoldIgnoresClosed) {
			0
		} else {
			CLOSED
		};
		if n & (closed | BIAS) == 0 && self.state.compare_exchange(n, n + HELD).is_ok() {
			return Some(None);
		}
		if self.state.load() & BIAS != 0 && self.slots[column].compare_exchange(0, KEY).is_ok() {
			self.fence(Flaw::HoldSkipsFence);
			if self.flawed(Flaw::HoldSkipsRecheck) || self.state.load() & CLOSED == 0 {
				return Some(Some(column));
			}
			// A close claimed it meanwhile, and may have counted this hold.
			if self.slots[column].compare_exchange(KEY, 0).is_err() {
				self.slots[column].store(0);
				self.release(None);
			}
			return None;
		}
		let mut n = self.state.load();
		loop {
			if n & CLOSED != 0 {
				return None;
			}
			match self.state.compare_exchange(n, n + HELD) {
				Ok(_) => break,
				Err(now) => n = now,
			}
		}
		// The fast path failed on an open handle with no bias only as another call changed
		// the count meanwhile: later ones go to the table.
		self.state.fetch_or(BIAS);
		Some(None)
	}

	/// Let go of a hold [`Self::hold`] took.
	fn release(&self, slot: Option<usize>) {
		if let Some(column) = slot
			&& self.slots[column].swap(0) == KEY
		{
			return;
		}
		let left = if self.flawed(Flaw::ReleaseFreesEarly) {
			2 * HELD
		} else {
			HELD
		};
		if self.state.fetch_sub(HELD) - HELD < left {
			self.free();
		}
	}

	fn free(&self) {
		self.raw.with_mut(|raw| unsafe { *raw = 0 });
		self.freed.fetch_add(1, SeqCst);
	}

	/// A call's use of a held handle.
	fn read(&self) -> u64 {
		self.raw.with(|raw| unsafe { *raw })
	}

	/// Give up the owner's hold once a move or a checked close took the value, freeing the
	/// handle once no hold is left.
	fn abandon(&self) {
		let n = loop {
			let n = self.state.load();
			if n & CLOSING != 0 {
				thread::yield_now();
				continue;
			}
			if n & CLOSED != 0 {
				return;
			}
			if self.state.compare_exchange(n, n | CLOSED).is_ok() {
				break n;
			}
		};
		self.fence(Flaw::AbandonSkipsFence);
		if n & BIAS != 0 && !self.flawed(Flaw::AbandonSkipsSlots) {
			for slot in &self.slots {
				if slot.load() == KEY {
					self.state.fetch_add(HELD);
					if slot.compare_exchange(KEY, KEY | REVOKED).is_err() {
						self.state.fetch_sub(HELD);
					}
				}
			}
		}
		self.release(None);
	}

	/// Close an unchecked handle, which fails while a call holds it, and does nothing once
	/// closed, waiting out another close that is still deciding.
	fn close(&self) -> Closed {
		let n = loop {
			let n = self.state.load();
			if n & CLOSING != 0 {
				thread::yield_now();
				continue;
			}
			if n & CLOSED != 0 {
				return Closed::Already;
			}
			if n & !BIAS != HELD && !self.flawed(Flaw::CloseIgnoresCount) {
				return Closed::Borrowed;
			}
			let claim = n | CLOSED | CLOSING;
			if self.state.compare_exchange(n, claim).is_ok() {
				break n;
			}
		};
		self.fence(Flaw::CloseSkipsFence);
		if n & BIAS != 0 && !self.flawed(Flaw::CloseIgnoresSlots) && self.slots.iter().any(|slot| slot.load() == KEY) {
			self.state.fetch_sub(CLOSED | CLOSING);
			return Closed::Borrowed;
		}
		self.state.store(CLOSED);
		self.free();
		Closed::Now
	}

	/// Close a checked handle as Go's and Kotlin's do, with [`Self::rust_drop`]'s `refused`
	/// and `then`. No call holding it, in one crossing: claimed closed so no call starts,
	/// then the value dropped, which may reopen it, and the handle freed. Else, unless
	/// closed, held for the drop alone, then giving up the owner's hold. Meeting another
	/// close's crossing, it waits until Rust commits that drop or refuses it, then decides
	/// again. Whether it closed it.
	fn checked_close(&self, column: usize, refused: bool, then: &dyn Fn()) -> bool {
		loop {
			let held = match self.flawed(Flaw::DropFreeSkipsClaim) {
				true => self.state.load() & !CLOSED,
				false => HELD,
			};
			if self.state.compare_exchange(held, CLOSED | CLOSING).is_ok() {
				if !self.rust_drop(refused, true, then) {
					self.state.store(HELD);
					return false;
				}
				self.free();
				return true;
			}
			let n = self.state.load();
			if n & CLOSING != 0 && !self.flawed(Flaw::CloseSkipsCommitWait) {
				thread::yield_now();
				continue;
			}
			if n & CLOSED != 0 {
				return true;
			}
			let Some(slot) = self.hold(column) else {
				continue;
			};
			let passed = self.rust_drop(refused, false, then);
			self.release(slot);
			if passed {
				self.abandon();
			}
			return passed;
		}
	}

	/// No hold is left, and the handle was freed once, or not at all while still open, and
	/// the value dropped at most once.
	fn settled(&self, closed: bool) {
		assert!(self.slots.iter().all(|slot| slot.load() == 0));
		assert!(self.dropped.load(SeqCst) <= 1);
		match closed {
			true => {
				assert_eq!(self.state.load() & !BIAS, CLOSED);
				assert_eq!(self.freed.load(SeqCst), 1);
			}
			false => {
				assert_eq!(self.state.load() & !BIAS, HELD);
				assert_eq!(self.freed.load(SeqCst), 0);
			}
		}
	}
}

/// What an unchecked close did.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Closed {
	/// Closed and freed it.
	Now,
	/// Found it closed.
	Already,
	/// Refused, as a call held it.
	Borrowed,
}

fn model(f: impl Fn() + Sync + Send + 'static) {
	let mut builder = loom::model::Builder::new();
	builder.preemption_bound = Some(3);
	builder.check(f);
}

/// A call holding and reading the handle from `column`: whether it held it.
fn call(hold: &Hold, column: usize) -> bool {
	match hold.hold(column) {
		Some(slot) => {
			assert_eq!(hold.read(), 7);
			hold.release(slot);
			true
		}
		None => false,
	}
}

/// Each model, as a test of the port as it is and as a function of the flaw to make it fail.
macro_rules! models {
	($($(#[$doc:meta])* fn $name:ident($flaw:ident) $body:block)*) => {
		mod models {
			use super::*;
			$(pub fn $name($flaw: Option<Flaw>) $body)*

			/// The model of that name.
			pub fn run(name: &str, flaw: Option<Flaw>) {
				match name {
					$(stringify!($name) => $name(flaw),)*
					_ => panic!("no model {name}"),
				}
			}
		}
		$($(#[$doc])* #[test] fn $name() { models::$name(None) })*
	};
}

models! {
	/// Two calls in the table and a move giving up the owner's hold: the free races no read.
	fn abandon_against_biased_calls(flaw) {
		model(move || {
			let hold = Hold::new(2, true, flaw);
			let calls: Vec<_> = (0..2)
				.map(|column| {
					let hold = hold.clone();
					thread::spawn(move || call(&hold, column))
				})
				.collect();
			hold.abandon();
			for c in calls {
				c.join().unwrap();
			}
			hold.settled(true);
		});
	}

	/// Calls on the count bias the handle once they meet, while a move frees it.
	fn abandon_while_bias_taken(flaw) {
		model(move || {
			let hold = Hold::new(2, false, flaw);
			let other = hold.clone();
			let calls = thread::spawn(move || {
				call(&other, 1);
				call(&other, 1)
			});
			call(&hold, 0);
			hold.abandon();
			calls.join().unwrap();
			hold.settled(true);
		});
	}

	/// Two calls sharing a column, one in the slot and one on the count, against a move.
	fn abandon_against_shared_column(flaw) {
		model(move || {
			let hold = Hold::new(1, true, flaw);
			let other = hold.clone();
			let first = thread::spawn(move || call(&other, 0));
			let second = call(&hold, 0);
			hold.abandon();
			first.join().unwrap();
			let _ = second;
			hold.settled(true);
		});
	}

	/// An unchecked close refused while a call holds the handle, in the table or the count,
	/// and freeing it otherwise, which no read races. A call that published its slot as the
	/// close claimed the handle is refused too, as the close is, which tells it from one that
	/// holds only by waiting out the call.
	fn close_against_biased_call(flaw) {
		model(move || {
			let hold = Hold::new(2, true, flaw);
			let other = hold.clone();
			let held = thread::spawn(move || call(&other, 1));
			let closed = hold.close();
			held.join().unwrap();
			if closed != Closed::Now {
				assert_eq!(hold.close(), Closed::Now);
			}
			hold.settled(true);
		});
	}

	/// Two unchecked closes against a call in the table: one that finds the handle closed
	/// returns only once the other's close stands, never while it may yet be refused.
	fn closes_against_biased_call(flaw) {
		model(move || {
			let hold = Hold::new(2, true, flaw);
			let (other, call_hold) = (hold.clone(), hold.clone());
			let held = thread::spawn(move || call(&call_hold, 1));
			let first = thread::spawn(move || other.close());
			let second = hold.close();
			let first = first.join().unwrap();
			held.join().unwrap();
			let closes = [first, second];
			if closes.contains(&Closed::Already) {
				assert!(closes.contains(&Closed::Now));
			}
			if !closes.contains(&Closed::Now) {
				assert_eq!(hold.close(), Closed::Now);
			}
			hold.settled(true);
		});
	}

	/// A checked close against calls on the count, which bias it as they meet, and a GC giving
	/// up the owner's hold: a call starting during the one-crossing drop finds it closed, and
	/// the handle is freed once, which no read races.
	fn checked_close_against_calls(flaw) {
		model(move || {
			let hold = Hold::new(2, false, flaw);
			let (other, collector) = (hold.clone(), hold.clone());
			let calls = thread::spawn(move || {
				call(&other, 1);
				call(&other, 1)
			});
			let collected = thread::spawn(move || collector.abandon());
			assert!(hold.checked_close(0, false, &|| {}));
			calls.join().unwrap();
			collected.join().unwrap();
			hold.settled(true);
		});
	}

	/// A checked close Rust refuses, against a call: it leaves the handle open, unfreed, and
	/// usable, whichever path it took.
	fn checked_close_refused(flaw) {
		model(move || {
			let hold = Hold::new(2, false, flaw);
			let other = hold.clone();
			let held = thread::spawn(move || call(&other, 1));
			assert!(!hold.checked_close(0, true, &|| {}));
			held.join().unwrap();
			assert!(call(&hold, 0));
			hold.settled(false);
		});
	}

	/// Calls that never meet a close leave the handle open and unfreed.
	fn calls_alone(flaw) {
		model(move || {
			let hold = Hold::new(2, false, flaw);
			let other = hold.clone();
			let a = thread::spawn(move || call(&other, 1));
			assert!(call(&hold, 0));
			assert!(a.join().unwrap());
			hold.settled(false);
		});
	}

	/// An unchecked close against a call on the count: refused while it holds the handle, and
	/// freeing it otherwise, which no read races.
	fn close_against_call(flaw) {
		model(move || {
			let hold = Hold::new(1, false, flaw);
			let other = hold.clone();
			let held = thread::spawn(move || call(&other, 0));
			let closed = hold.close();
			held.join().unwrap();
			if closed != Closed::Now {
				assert_eq!(hold.close(), Closed::Now);
			}
			hold.settled(true);
		});
	}

	/// Two checked closes Rust refuses: one meeting the other's crossing waits out its
	/// refusal and is refused too, never reporting closed a handle left open.
	fn checked_closes_refused(flaw) {
		model(move || {
			let hold = Hold::new(2, false, flaw);
			let other = hold.clone();
			let first = thread::spawn(move || other.checked_close(1, true, &|| {}));
			assert!(!hold.checked_close(0, true, &|| {}));
			assert!(!first.join().unwrap());
			hold.settled(false);
		});
	}

	/// Two checked closes Rust lets drop: each reports closed only once Rust took the value,
	/// which drops once.
	fn checked_closes(flaw) {
		model(move || {
			let hold = Hold::new(2, false, flaw);
			let other = hold.clone();
			let first = thread::spawn(move || {
				let closed = other.checked_close(1, false, &|| {});
				assert_eq!(other.taken.load(), 1);
				closed
			});
			assert!(hold.checked_close(0, false, &|| {}));
			assert_eq!(hold.taken.load(), 1);
			assert!(first.join().unwrap());
			hold.settled(true);
			assert_eq!(hold.dropped.load(SeqCst), 1);
		});
	}

	/// A `Drop` that hands the close of its own value to another thread and joins it,
	/// against a second close: neither waits on the crossing that runs the `Drop`, which loom
	/// would report as a deadlock.
	fn checked_close_from_drop(flaw) {
		model(move || {
			let hold = Hold::new(3, false, flaw);
			let (other, inner) = (hold.clone(), hold.clone());
			let second = thread::spawn(move || other.checked_close(1, false, &|| {}));
			let then = move || {
				let inner = inner.clone();
				let joined = thread::spawn(move || inner.checked_close(2, false, &|| {}));
				assert!(joined.join().unwrap());
			};
			assert!(hold.checked_close(0, false, &then));
			assert!(second.join().unwrap());
			hold.settled(true);
			assert_eq!(hold.dropped.load(SeqCst), 1);
		});
	}
}

/// Each flaw with the models that must fail on it.
const CAUGHT: &[(Flaw, &[&str])] = &[
	(Flaw::HoldIgnoresClosed, &["abandon_while_bias_taken"]),
	(
		Flaw::HoldSkipsRecheck,
		&["abandon_against_biased_calls", "close_against_biased_call"],
	),
	(
		Flaw::HoldSkipsFence,
		&["abandon_against_biased_calls", "close_against_biased_call"],
	),
	(Flaw::CloseIgnoresCount, &["close_against_call"]),
	(Flaw::CloseIgnoresSlots, &["close_against_biased_call"]),
	(Flaw::CloseSkipsFence, &["close_against_biased_call"]),
	(Flaw::AbandonSkipsFence, &["abandon_against_biased_calls"]),
	(Flaw::AbandonSkipsSlots, &["abandon_against_biased_calls"]),
	(Flaw::DropFreeSkipsClaim, &["checked_close_against_calls"]),
	(Flaw::ReleaseFreesEarly, &["calls_alone"]),
	(Flaw::CloseSkipsCommitWait, &["checked_closes_refused", "checked_closes"]),
	(Flaw::DropSkipsCommit, &["checked_close_from_drop"]),
];

/// The models do not pass vacuously: each fails on the flaws it is there to catch.
#[test]
fn flaws_are_caught() {
	let runs: Vec<_> = CAUGHT
		.iter()
		.flat_map(|&(flaw, names)| names.iter().map(move |&name| (flaw, name)))
		.map(|(flaw, name)| {
			let run = std::thread::spawn(move || models::run(name, Some(flaw)));
			(flaw, name, run)
		})
		.collect();
	let missed: Vec<_> = runs
		.into_iter()
		.filter_map(|(flaw, name, run)| run.join().is_ok().then_some((flaw, name)))
		.collect();
	assert!(missed.is_empty(), "no failure from {missed:?}");
}
