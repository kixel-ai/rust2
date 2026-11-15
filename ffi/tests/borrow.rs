//! A model check of the runtime's borrow state, `runtime/rt/borrow.rs`, under loom: every
//! interleaving of shared borrows, through the table or the count, against an exclusive
//! borrow, a move, or a drop, within a preemption bound. Each borrow reads or writes a loom
//! cell, which fails the model on any access the borrow check let race, and each outcome
//! is checked against what a serial order would allow.

use loom::cell::UnsafeCell;
use loom::sync::Arc;
use loom::sync::atomic::{self, AtomicUsize};
use loom::thread::{self, yield_now};

/// The runtime's refusals, of which the borrow state returns these.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
	Moved,
	Exclusive,
	Borrowed,
	Thread,
}

#[path = "../runtime/rt/borrow.rs"]
#[allow(dead_code, reason = "the model drives part of what the runtime uses")]
mod borrow;

use borrow::{Borrow, EXCLUSIVE, TAKEN, Table};

/// A handle's borrow state and its value, beside a table of `columns` columns of `rows`.
struct Handle {
	borrow: Borrow,
	value: UnsafeCell<u64>,
	slots: Vec<AtomicUsize>,
	rows: usize,
}

impl Handle {
	fn new(columns: usize, rows: usize, biased: bool) -> Arc<Self> {
		let borrow = Borrow::new(0);
		borrow.bias.store(biased, atomic::Ordering::Relaxed);
		Arc::new(Self {
			borrow,
			value: UnsafeCell::new(0),
			slots: (0..columns * rows).map(|_| AtomicUsize::new(0)).collect(),
			rows,
		})
	}

	fn table(&self) -> Table<'_> {
		Table::new(&self.slots, self.rows)
	}

	/// Read the value under a shared borrow from `column`, if granted.
	fn read(&self, column: usize) -> Result<u64, Status> {
		let table = self.table();
		let listed = self.borrow.share(Some(&table), column)?;
		let value = self.value.with(|value| unsafe { *value });
		self.borrow.unshare(&table, column, listed);
		Ok(value)
	}

	/// Add one to the value under an exclusive borrow, if granted.
	fn write(&self) -> Result<(), Status> {
		self.borrow.take(&self.table(), EXCLUSIVE)?;
		self.value.with_mut(|value| unsafe { *value += 1 });
		self.borrow.set(0);
		Ok(())
	}

	/// Drop the value, as a binding's close does: false once already dropped.
	fn close(&self) -> Result<bool, Status> {
		match self.borrow.take(&self.table(), TAKEN) {
			Ok(()) => {
				self.value.with_mut(|value| unsafe { *value = u64::MAX });
				Ok(true)
			}
			Err(Status::Moved) => Ok(false),
			Err(status) => Err(status),
		}
	}

	/// No slot names the handle, nor does the count, once every borrow ended.
	fn settled(&self) {
		assert!(self.slots.iter().all(|slot| slot.load(atomic::Ordering::Relaxed) == 0));
		assert!(matches!(self.borrow.state.load(atomic::Ordering::Relaxed), 0 | TAKEN));
	}
}

fn model(f: impl Fn() + Sync + Send + 'static) {
	let mut builder = loom::model::Builder::new();
	builder.preemption_bound = Some(3);
	builder.check(f);
}

/// Two biased readers in their own columns and a writer: the writer's update races no read,
/// and a writer refused saw a reader in the table, so some read happened.
#[test]
fn biased_readers_and_writer() {
	model(|| {
		let handle = Handle::new(2, 1, true);
		let readers: Vec<_> = (0..2)
			.map(|column| {
				let handle = handle.clone();
				thread::spawn(move || handle.read(column))
			})
			.collect();
		let wrote = handle.write();
		let reads: Vec<_> = readers.into_iter().map(|reader| reader.join().unwrap()).collect();
		match wrote {
			Ok(()) => assert!(reads.iter().all(|read| matches!(read, Ok(0 | 1) | Err(Status::Exclusive)))),
			Err(status) => {
				assert_eq!(status, Status::Borrowed);
				assert!(reads.iter().any(|read| *read == Ok(0)));
			}
		}
		handle.settled();
	});
}

/// Readers that start on the count bias the handle once they meet, and a writer meanwhile
/// races no read, whichever path each read took.
#[test]
fn bias_taken_mid_run() {
	model(|| {
		let handle = Handle::new(2, 1, false);
		let other = handle.clone();
		let reader = thread::spawn(move || {
			let a = other.read(1);
			let b = other.read(1);
			(a, b)
		});
		let first = handle.read(0);
		let wrote = handle.write();
		let (a, b) = reader.join().unwrap();
		assert_eq!(first, Ok(0));
		if wrote.is_err() {
			assert!(a.is_ok() || b.is_ok());
		}
		handle.settled();
	});
}

/// A refused writer leaves the bias set, so a second writer still finds the reader it
/// refused: two writes in turn against one long read.
#[test]
fn refused_writer_keeps_bias() {
	model(|| {
		let handle = Handle::new(2, 1, true);
		let other = handle.clone();
		let reader = thread::spawn(move || {
			let table = other.table();
			let listed = other.borrow.share(Some(&table), 1)?;
			let first = other.value.with(|value| unsafe { *value });
			yield_now();
			let second = other.value.with(|value| unsafe { *value });
			other.borrow.unshare(&table, 1, listed);
			assert_eq!(first, second, "a write ran inside a shared borrow");
			Ok::<_, Status>(())
		});
		let _ = handle.write();
		let _ = handle.write();
		assert!(matches!(reader.join().unwrap(), Ok(()) | Err(Status::Exclusive)));
		handle.settled();
	});
}

/// Two readers in one column, which share a slot, against a close: the second falls back
/// to the count, and a close refused found one of them.
#[test]
fn shared_column_and_close() {
	model(|| {
		let handle = Handle::new(1, 1, true);
		let other = handle.clone();
		let reader = thread::spawn(move || other.read(0));
		let read = handle.read(0);
		let closed = handle.close();
		let other = reader.join().unwrap();
		assert!(matches!(read, Ok(0)));
		match closed {
			Ok(true) => assert!(matches!(other, Ok(0) | Err(Status::Moved | Status::Exclusive))),
			Ok(false) => unreachable!("closed twice"),
			Err(status) => {
				assert_eq!(status, Status::Borrowed);
				assert_eq!(other, Ok(0));
			}
		}
		assert_eq!(handle.close(), Ok(matches!(closed, Err(_))));
		handle.settled();
	});
}
