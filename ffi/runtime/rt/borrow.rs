//! A handle's run-time borrow check: a count of shared borrows, or a mark for an exclusive
//! borrow or a move, biased toward readers in the style of BRAVO (Dice and Kogan, USENIX
//! ATC 2019). Once threads contend for one handle's count, a shared borrow stops touching
//! it: it publishes the handle in its own thread's slot of a [`Table`] instead, so threads
//! sharing a handle write nothing in common. Whatever would conflict with a shared borrow
//! (an exclusive borrow, a reservation to move out, a move, a drop) takes the count first,
//! then revokes the bias and fails if a slot still names the handle. A slot whose reader is
//! still checking the bias is waited out, which takes it a few instructions, so the check
//! never refuses a borrow the count alone would have let in.
//!
//! The runtime compiles this beside `rt.rs`, and `ffi/tests/borrow.rs` under loom, each with
//! its own `atomic`, `yield_now`, and `Status`.

use super::Status;
use super::atomic::Ordering::{Acquire, Relaxed, Release, SeqCst};
use super::atomic::{AtomicBool, AtomicUsize, fence};
use super::yield_now;

/// The value is borrowed exclusively.
pub const EXCLUSIVE: usize = usize::MAX;
/// The value moved out, or a binding's close dropped it.
pub const TAKEN: usize = usize::MAX - 1;
/// A call holds the value to move out once its checks pass, or a move or drop holds it
/// while it revokes the bias.
pub const RESERVED: usize = usize::MAX - 2;

/// Marks a slot whose reader has yet to see whether the bias still stands.
const PENDING: usize = 1;

/// The slots biased shared borrows publish their handles in: a column per thread and a
/// row per handle, so slots of two threads lie a column apart and never share a cache line
/// once a column spans one. Two borrows wanting one slot (two handles in one row on one
/// thread, or two threads in one column) fall back to the count.
pub struct Table<'a> {
	slots: &'a [AtomicUsize],
	rows: usize,
}

impl<'a> Table<'a> {
	/// `slots`, `rows` to a column: a power of two dividing its length.
	pub const fn new(slots: &'a [AtomicUsize], rows: usize) -> Self {
		assert!(rows.is_power_of_two() && slots.len().is_multiple_of(rows));
		Self { slots, rows }
	}

	fn row(&self, key: usize) -> usize {
		// Fibonacci hashing of the address, past the bits its alignment zeroes.
		let hash = ((key >> 3) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
		(hash >> 32) as usize & (self.rows - 1)
	}

	fn slot(&self, column: usize, key: usize) -> &'a AtomicUsize {
		let columns = self.slots.len() / self.rows;
		&self.slots[column % columns * self.rows + self.row(key)]
	}

	/// The slots that may name `key`, one in each column.
	fn named(&self, key: usize) -> impl Iterator<Item = &'a AtomicUsize> {
		self.slots.iter().skip(self.row(key)).step_by(self.rows)
	}
}

/// The borrow state of one handle's value.
pub struct Borrow {
	/// The count of shared borrows outside the table, or [`EXCLUSIVE`], [`RESERVED`], or
	/// [`TAKEN`]. A value that is not `Sync` holds the id of the thread borrowing it instead.
	pub state: AtomicUsize,
	/// Set while shared borrows may sit in the table: only by one holding the count, and
	/// cleared only by one holding the value exclusively, once no slot names it.
	pub(super) bias: AtomicBool,
}

impl Borrow {
	pub fn new(state: usize) -> Self {
		Self {
			state: AtomicUsize::new(state),
			bias: AtomicBool::new(false),
		}
	}

	/// What a slot holds for this borrow: its address, which an `AtomicUsize`'s alignment
	/// leaves the low bit of clear for [`PENDING`].
	fn key(&self) -> usize {
		std::ptr::from_ref(self) as usize
	}

	/// Borrow shared, from the thread in `column` of `table`, or by the count alone without
	/// one: whether the borrow went to the table, as [`Self::unshare`] takes it.
	pub fn share(&self, table: Option<&Table<'_>>, column: usize) -> Result<bool, Status> {
		if let Some(table) = table
			&& self.bias.load(Relaxed)
		{
			let key = self.key();
			let slot = table.slot(column, key);
			if slot.compare_exchange(0, key | PENDING, Relaxed, Relaxed).is_ok() {
				// The slot is published before the bias is read, as the revocation clears
				// the bias before it reads the slots, so it either stops this borrow here
				// or finds it in its scan.
				fence(SeqCst);
				if self.bias.load(Acquire) {
					slot.store(key, Release);
					return Ok(true);
				}
				slot.store(0, Release);
			}
		}
		let mut state = self.state.load(Relaxed);
		let mut contended = false;
		loop {
			match state {
				TAKEN => return Err(Status::Moved),
				EXCLUSIVE | RESERVED => return Err(Status::Exclusive),
				_ => {}
			}
			match self.state.compare_exchange(state, state + 1, Acquire, Relaxed) {
				Ok(_) => break,
				Err(now) => {
					state = now;
					contended = true;
				}
			}
		}
		// Another borrow changed the count meanwhile: later ones go to the table. Held
		// count in hand, no exclusive borrow is running to miss this.
		if contended && !self.bias.load(Relaxed) {
			self.bias.store(true, Release);
		}
		Ok(false)
	}

	/// End a shared borrow [`Self::share`] took, in `column` and `listed` as it said.
	pub fn unshare(&self, table: &Table<'_>, column: usize, listed: bool) {
		match listed {
			true => table.slot(column, self.key()).store(0, Release),
			false => {
				self.state.fetch_sub(1, Release);
			}
		}
	}

	/// Take the value from no borrow to `to`: [`EXCLUSIVE`], [`RESERVED`], or [`TAKEN`].
	pub fn take(&self, table: &Table<'_>, to: usize) -> Result<(), Status> {
		// Moved out only once no slot names it, so a borrow refused meanwhile never reports
		// a move that then fails.
		let hold = match to {
			TAKEN => RESERVED,
			to => to,
		};
		match self.state.compare_exchange(0, hold, Acquire, Relaxed) {
			Ok(_) => {}
			Err(TAKEN) => return Err(Status::Moved),
			Err(_) => return Err(Status::Borrowed),
		}
		if !self.revoke(table) {
			self.state.store(0, Release);
			return Err(Status::Borrowed);
		}
		if hold != to {
			self.state.store(to, Relaxed);
		}
		Ok(())
	}

	/// Borrow a value that is not `Sync` on the thread whose id is `me`: whether this is
	/// its first borrow there. No shared borrow of one goes to the table, as only a `Sync`
	/// value's does.
	pub fn local(&self, me: usize) -> Result<bool, Status> {
		match self.state.compare_exchange(0, me, Acquire, Relaxed) {
			Ok(_) => Ok(true),
			Err(owner) if owner == me => Ok(false),
			Err(TAKEN) => Err(Status::Moved),
			Err(EXCLUSIVE | RESERVED) => Err(Status::Exclusive),
			Err(_) => Err(Status::Thread),
		}
	}

	/// Let go of the value: end an exclusive borrow, a reservation, or a thread's last local
	/// borrow, or mark a reservation moved out with [`TAKEN`].
	pub fn set(&self, state: usize) {
		self.state.store(state, Release);
	}

	/// Whether any borrow holds the value, as a move would find it.
	pub fn borrowed(&self, table: &Table<'_>) -> bool {
		!matches!(self.state.load(Acquire), 0 | TAKEN) || self.listed(table)
	}

	/// Whether the state is `state` and no slot names this, as a lent handle's must be
	/// when its lend ends.
	pub fn is(&self, table: &Table<'_>, state: usize) -> bool {
		self.state.load(Acquire) == state && !self.listed(table)
	}

	/// Holding the count against any new shared borrow, clear the bias unless a slot names
	/// this: whether none did.
	fn revoke(&self, table: &Table<'_>) -> bool {
		if !self.bias.load(Acquire) {
			return true;
		}
		self.bias.store(false, Relaxed);
		fence(SeqCst);
		if self.scan(table) {
			// Biased again, so the next to take the count scans again.
			self.bias.store(true, Release);
			return false;
		}
		true
	}

	/// Whether a slot may name this, and does.
	fn listed(&self, table: &Table<'_>) -> bool {
		self.bias.load(Acquire) && self.scan(table)
	}

	/// Whether a slot names this, once each reader still checking the bias is done.
	fn scan(&self, table: &Table<'_>) -> bool {
		let key = self.key();
		for slot in table.named(key) {
			loop {
				match slot.load(Acquire) {
					now if now == key | PENDING => yield_now(),
					now if now == key => return true,
					_ => break,
				}
			}
		}
		false
	}
}
