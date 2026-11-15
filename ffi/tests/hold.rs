//! A model check, under loom, of the hold count Go's `Handle_`, Kotlin's `__Handle`, and
//! Swift's class `Handle` keep beside the runtime's borrow check, each a port of `Hold`
//! below: a call holds the handle so no close frees it meanwhile, through a slot of the
//! runtime's table once calls contend, and a close that would free it claims it closed
//! first, then turns each hold it finds in the table into a count, so the last to let go
//! frees it. Every interleaving within a preemption bound: a hold reads the handle, which
//! a free writes, so loom fails the model on any free a hold could race.

use loom::cell::UnsafeCell;
use loom::sync::Arc;
use loom::sync::atomic::Ordering::SeqCst;
use loom::sync::atomic::{AtomicBool, AtomicUsize, fence};
use loom::thread;

/// Set in the hold count once closed.
const CLOSED: usize = 1;
/// One hold in the count.
const HELD: usize = 2;
/// Set in a slot whose hold a close turned into a count.
const REVOKED: usize = 1;
/// The handle's key in a slot, as the bindings use its raw pointer.
const KEY: usize = 8;

/// A handle as a binding holds it, beside a table with a slot per column for its row.
struct Hold {
	state: AtomicUsize,
	bias: AtomicBool,
	slots: Vec<AtomicUsize>,
	/// What the handle points at: Rust's memory, which the free ends.
	raw: UnsafeCell<u64>,
	freed: AtomicUsize,
}

impl Hold {
	fn new(columns: usize, biased: bool) -> Arc<Self> {
		Arc::new(Self {
			state: AtomicUsize::new(HELD),
			bias: AtomicBool::new(biased),
			slots: (0..columns).map(|_| AtomicUsize::new(0)).collect(),
			raw: UnsafeCell::new(7),
			freed: AtomicUsize::new(0),
		})
	}

	/// Hold the handle from `column`: the slot it went to, if any, or `None` once closed.
	fn hold(&self, column: usize) -> Option<Option<usize>> {
		if self.bias.load(SeqCst) && self.slots[column].compare_exchange(0, KEY, SeqCst, SeqCst).is_ok() {
			// Loom treats a sequentially consistent access as acquire and release alone,
			// so a fence stands for what Go's, Kotlin's, and Swift's atomics order.
			fence(SeqCst);
			if self.state.load(SeqCst) & CLOSED == 0 {
				return Some(Some(column));
			}
			// A close claimed it meanwhile, and may have counted this hold.
			if self.slots[column].compare_exchange(KEY, 0, SeqCst, SeqCst).is_err() {
				self.slots[column].store(0, SeqCst);
				self.release(None);
			}
			return None;
		}
		let mut n = self.state.load(SeqCst);
		let mut contended = false;
		loop {
			if n & CLOSED != 0 {
				return None;
			}
			match self.state.compare_exchange(n, n + HELD, SeqCst, SeqCst) {
				Ok(_) => break,
				Err(now) => {
					n = now;
					contended = true;
				}
			}
		}
		if contended && !self.bias.load(SeqCst) {
			self.bias.store(true, SeqCst);
		}
		Some(None)
	}

	/// Let go of a hold [`Self::hold`] took.
	fn release(&self, slot: Option<usize>) {
		if let Some(column) = slot
			&& self.slots[column].swap(0, SeqCst) == KEY
		{
			return;
		}
		if self.state.fetch_sub(HELD, SeqCst) - HELD < HELD {
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
		let mut n = self.state.load(SeqCst);
		loop {
			if n & CLOSED != 0 {
				return;
			}
			match self.state.compare_exchange(n, n | CLOSED, SeqCst, SeqCst) {
				Ok(_) => break,
				Err(now) => n = now,
			}
		}
		fence(SeqCst);
		if self.bias.load(SeqCst) {
			for slot in &self.slots {
				if slot.load(SeqCst) == KEY {
					self.state.fetch_add(HELD, SeqCst);
					if slot.compare_exchange(KEY, KEY | REVOKED, SeqCst, SeqCst).is_err() {
						self.state.fetch_sub(HELD, SeqCst);
					}
				}
			}
		}
		self.release(None);
	}

	/// Close an unchecked handle, which fails while a call holds it: whether it closed.
	fn close(&self) -> bool {
		if self.state.compare_exchange(HELD, CLOSED | HELD, SeqCst, SeqCst).is_err() {
			return false;
		}
		fence(SeqCst);
		if self.bias.load(SeqCst) && self.slots.iter().any(|slot| slot.load(SeqCst) == KEY) {
			self.state.store(HELD, SeqCst);
			return false;
		}
		self.state.store(CLOSED, SeqCst);
		self.free();
		true
	}

	/// No hold is left, and the handle was freed once, or not at all while still open.
	fn settled(&self, closed: bool) {
		assert!(self.slots.iter().all(|slot| slot.load(SeqCst) == 0));
		match closed {
			true => {
				assert_eq!(self.state.load(SeqCst), CLOSED);
				assert_eq!(self.freed.load(SeqCst), 1);
			}
			false => {
				assert_eq!(self.state.load(SeqCst), HELD);
				assert_eq!(self.freed.load(SeqCst), 0);
			}
		}
	}
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

/// Two calls in the table and a move giving up the owner's hold: the free races no read.
#[test]
fn abandon_against_biased_calls() {
	model(|| {
		let hold = Hold::new(2, true);
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
#[test]
fn abandon_while_bias_taken() {
	model(|| {
		let hold = Hold::new(2, false);
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
#[test]
fn abandon_against_shared_column() {
	model(|| {
		let hold = Hold::new(1, true);
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
#[test]
fn close_against_biased_call() {
	model(|| {
		let hold = Hold::new(2, true);
		let other = hold.clone();
		let held = thread::spawn(move || call(&other, 1));
		let closed = hold.close();
		held.join().unwrap();
		if !closed {
			assert!(hold.close());
		}
		hold.settled(true);
	});
}

/// Calls that never meet a close leave the handle open and unfreed.
#[test]
fn calls_alone() {
	model(|| {
		let hold = Hold::new(2, false);
		let other = hold.clone();
		let a = thread::spawn(move || call(&other, 1));
		assert!(call(&hold, 0));
		assert!(a.join().unwrap());
		hold.settled(false);
	});
}
