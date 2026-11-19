//! An executable model of the ownership protocol the C ABI's README section states: what
//! each step of a random sequence of borrows, exclusive lends, moves, drops, frees, polls,
//! and cancels must return, on the thread that made a value or another. A seed makes a
//! script of steps over the api fixture's types, each with the outcome the model expects,
//! which every driver runs through its own surface and prints back line for line.
//!
//! The model is the spec: it reads the README's rules, not the runtime's code, so a driver
//! printing another outcome is a bug in one of them. Where several checks would fail, it
//! follows the order the glue checks them in: the receiver, then each parameter in order;
//! and for one handle, whether its thread may use it, then whether a call lent it, then
//! its borrow state.
//!
//! Mounted by `tests/fixtures/protocol.rs`, which writes the scripts, and by
//! `tests/miri/protocol.rs`, which runs them in the glue under Miri.

use std::fmt::Write as _;

/// The threads a script runs on: the first, which runs the cleanup, and a worker.
pub const THREADS: usize = 2;

/// What a driver's surface can run, which shapes the scripts written for it.
#[derive(Clone, Copy, Debug)]
pub struct Profile {
	/// Steps on the worker thread too.
	pub threads: bool,
	/// Locals made on a home now and then, through `Local::new_homed`.
	pub homes: bool,
	/// Every value that is not `Send` made on a home, through each `_homed` twin, as a
	/// binding whose threads move makes them.
	pub homed: bool,
	/// Frees amid the steps, which a binding whose collector frees in its own time cannot
	/// make: it closes instead, and its cleanup drops what is left.
	pub frees: bool,
	/// Polls a step at a time with the script's waker, whose counts end the transcript;
	/// else each future is awaited to its end in one step.
	pub polls: bool,
	/// Steps whose check fails, which a surface that aborts on one never runs.
	pub failures: bool,
	/// `<prefix>_thread_drain`, which a binding that drains only at a thread's next check
	/// or its exit never calls.
	pub drain: bool,
	/// The most futures pending at once. A binding whose await drives every pending future,
	/// as Dart's event loop does, keeps one, so no other ends early; one whose call runs a
	/// future that wakes itself to its end in place, as a Kotlin `suspend fun` does, keeps
	/// none: each future is awaited in the step after the one that made it.
	pub pending: usize,
	/// A future that is not `Send` made from no such value on a home, as a binding whose
	/// coroutines resume on any thread makes one.
	pub homed_futures: bool,
	/// A binding's own check that every handle a call names is open, before Rust's: a
	/// closed one fails as moved whatever Rust would refuse first, and closing it again does
	/// nothing.
	pub closed_first: bool,
	/// Whether a call moving a value out closes its handle, as Kotlin's does; Dart's stays
	/// open, so Rust's own check refuses its next use.
	pub moves_close: bool,
}

impl Profile {
	/// The C ABI's: everything.
	pub const C: Self = Self {
		threads: true,
		homes: true,
		homed: false,
		frees: true,
		polls: true,
		failures: true,
		drain: true,
		pending: usize::MAX,
		homed_futures: false,
		closed_first: false,
		moves_close: false,
	};

	/// The C ABI's under Miri, which counts a home's thread outliving the script as a leak.
	pub const MIRI: Self = Self {
		homes: false,
		..Self::C
	};

	/// Kotlin's on the JVM: a second thread, no homes but a future's made from no value that
	/// is not `Send`, and each future awaited in the step after its call; no frees amid the steps, as the GC frees in its own time, and no
	/// drain, as a value freed elsewhere drops at its thread's next check.
	pub const KOTLIN: Self = Self {
		threads: true,
		homes: false,
		homed: false,
		frees: false,
		polls: false,
		failures: true,
		drain: false,
		pending: 0,
		homed_futures: true,
		closed_first: true,
		moves_close: true,
	};

	/// Dart's: one isolate, whose thread may change, so every value that is not `Send` is
	/// on a home and a second thread has no meaning; the event loop polls every pending
	/// future as one is awaited, so one pends at a time.
	pub const DART: Self = Self {
		threads: false,
		homes: true,
		homed: true,
		frees: false,
		polls: false,
		failures: true,
		drain: false,
		pending: 1,
		homed_futures: true,
		closed_first: true,
		moves_close: false,
	};
}

/// [`crate`]'s `rt::Status`, as each driver prints it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
	Moved,
	Exclusive,
	Borrowed,
	Lent,
	Outlived,
	NotSend,
}

impl Status {
	/// Its number in the C ABI, `<PREFIX>_STATUS_*`.
	pub fn code(self) -> i32 {
		match self {
			Status::Moved => 1,
			Status::Exclusive => 2,
			Status::Borrowed => 3,
			Status::Lent => 5,
			Status::Outlived => 6,
			Status::NotSend => 7,
		}
	}

	/// The word a driver prints for it, the C name in lower case.
	pub fn name(self) -> &'static str {
		match self {
			Status::Moved => "moved",
			Status::Exclusive => "exclusive",
			Status::Borrowed => "borrowed",
			Status::Lent => "lent",
			Status::Outlived => "outlived",
			Status::NotSend => "not_send",
		}
	}
}

/// What a handle's slot holds: an object of an api type, or a future.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
	Counter,
	Ledger,
	/// `Ledger::edit`'s exclusive lend: not `Send`, publishing on drop once written.
	Draft,
	/// `Ledger::receipt`'s exclusive lend, whose clones share it.
	Receipt,
	/// `Send` but not `Sync`: a future borrowing it holds it exclusively.
	Tally,
	/// Not `Send`: it stays on the thread that made it, or on a home.
	Local,
	Meters,
	/// Owned by `Meter::default`, or lent by `Meters::first`.
	Meter,
	/// `Counter::plus_later`: `Send`, holding both counters shared.
	PlusLater,
	/// `Tally::bump_later`: not `Send`, holding the tally exclusively.
	BumpLater,
	/// `Local::later`: not `Send`, holding the local exclusively.
	LocalLater,
}

impl Kind {
	/// Whether it is a future, which a poll drives and a cancel frees.
	pub fn future(self) -> bool {
		matches!(self, Kind::PlusLater | Kind::BumpLater | Kind::LocalLater)
	}

	/// Its name in a script.
	pub fn name(self) -> &'static str {
		match self {
			Kind::Counter => "Counter",
			Kind::Ledger => "Ledger",
			Kind::Draft => "Draft",
			Kind::Receipt => "Receipt",
			Kind::Tally => "Tally",
			Kind::Local => "Local",
			Kind::Meters => "Meters",
			Kind::Meter => "Meter",
			Kind::PlusLater => "Counter_plus_later",
			Kind::BumpLater => "Tally_bump_later",
			Kind::LocalLater => "Local_later",
		}
	}
}

/// One call. A slot is a handle's index in the script; a step that makes a handle names the
/// slot it fills, the next unused one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
	/// `Counter::new` into the slot, at the start.
	CounterNew(usize, u64),
	CounterGet(usize),
	/// `Counter::add`: exclusive, failing past the fixture's `LIMIT`.
	CounterAdd(usize, u64),
	/// `Counter::into_sum`: moves the value out.
	CounterIntoSum(usize, u64),
	/// `Counter::plus`, both shared.
	CounterPlus(usize, Option<usize>),
	/// `Counter::add_to`: the receiver shared, the other exclusive.
	CounterAddTo(usize, Option<usize>),
	/// `Counter::clone` into the first slot.
	CounterClone(usize, usize),
	/// `Counter::plus_later` into the first slot.
	PlusLater(usize, usize, Option<usize>),
	LedgerNew(usize),
	/// `Ledger::edit` into the first slot.
	LedgerEdit(usize, usize),
	LedgerPublished(usize),
	LedgerSeal(usize),
	/// `Ledger::receipt` into the first slot.
	LedgerReceipt(usize, usize),
	DraftWrite(usize),
	DraftLines(usize),
	/// `Draft::commit`: moves the draft out, ending its lend.
	DraftCommit(usize),
	ReceiptCount(usize),
	/// `Receipt::clone` into the first slot, sharing the lend.
	ReceiptClone(usize, usize),
	/// `tallied`: moves the receipt into a call that ends its lend as it returns.
	Tallied(usize),
	/// `stored`: a call that could keep the receipt past it, which its lend refuses.
	Stored(usize),
	TallyNew(usize),
	TallyBump(usize),
	/// `Tally::bump_later` into the first slot.
	BumpLater(usize, usize),
	/// `Local::new`, or `Local::new_homed` onto a home.
	LocalNew(usize, bool),
	LocalBump(usize),
	/// `Local::share` into the first slot: a local of the same count and thread.
	LocalShare(usize, usize),
	/// `Local::later` into the first slot.
	LocalLater(usize, usize),
	MetersNew(usize),
	/// `Meters::first` into the first slot: a handle lent from the meters.
	MetersFirst(usize, usize),
	/// `Meters::clear`: exclusive, ending what `first` lent.
	MetersClear(usize),
	/// `Meter::default`: an owned meter.
	MeterNew(usize),
	MeterAdd(usize, u64),
	/// `Meter::reset`: exclusive, which a lent handle refuses.
	MeterReset(usize),
	/// `_drop`: drop the value now, leaving the handle.
	Drop(usize),
	/// `_free`, or a future's `_future_free`.
	Free(usize),
	/// A future's poll with the script's counting waker.
	Poll(usize),
	/// A future awaited to its end, as a binding awaits one.
	Await(usize),
	/// `<prefix>_thread_drain`: run what other threads queued for this one.
	Drain,
}

impl Op {
	/// The future slot a call fills, if it makes one.
	fn future(self) -> Option<usize> {
		match self {
			Op::PlusLater(f, ..) | Op::BumpLater(f, _) | Op::LocalLater(f, _) => Some(f),
			_ => None,
		}
	}

	/// The objects a call names, not the slot it fills: what a binding checks are open.
	fn handles(self) -> Vec<usize> {
		match self {
			Op::CounterGet(h)
			| Op::CounterAdd(h, _)
			| Op::CounterIntoSum(h, _)
			| Op::CounterClone(_, h)
			| Op::LedgerEdit(_, h)
			| Op::LedgerPublished(h)
			| Op::LedgerSeal(h)
			| Op::LedgerReceipt(_, h)
			| Op::DraftWrite(h)
			| Op::DraftLines(h)
			| Op::DraftCommit(h)
			| Op::Tallied(h)
			| Op::Stored(h)
			| Op::ReceiptCount(h)
			| Op::ReceiptClone(_, h)
			| Op::TallyBump(h)
			| Op::BumpLater(_, h)
			| Op::LocalBump(h)
			| Op::LocalShare(_, h)
			| Op::LocalLater(_, h)
			| Op::MetersFirst(_, h)
			| Op::MetersClear(h)
			| Op::MeterAdd(h, _)
			| Op::MeterReset(h) => vec![h],
			Op::CounterPlus(h, o) | Op::CounterAddTo(h, o) | Op::PlusLater(_, h, o) => {
				[Some(h), o].into_iter().flatten().collect()
			}
			// A close of a closed handle does nothing, and a future is never closed.
			Op::CounterNew(..)
			| Op::LedgerNew(_)
			| Op::TallyNew(_)
			| Op::LocalNew(..)
			| Op::MetersNew(_)
			| Op::MeterNew(_)
			| Op::Drop(_)
			| Op::Free(_)
			| Op::Poll(_)
			| Op::Await(_)
			| Op::Drain => Vec::new(),
		}
	}
}

/// One step: a call on a thread, and what it must return.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step {
	pub thread: usize,
	pub op: Op,
	pub expect: Outcome,
}

/// What a step returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
	/// It ran, returning nothing to compare, or a handle into its slot.
	Ok,
	/// It ran, returning this number: a count, a sum, or a `bool` as 0 or 1.
	Value(u64),
	/// It ran, returning Rust's `Err`.
	Error,
	/// A check failed before any Rust code ran.
	Failed(Status),
	/// A poll pended.
	Pending,
}

impl Outcome {
	/// The line a driver prints for it.
	pub fn line(self) -> String {
		match self {
			Outcome::Ok => "ok".to_string(),
			Outcome::Value(n) => n.to_string(),
			Outcome::Error => "error".to_string(),
			Outcome::Failed(status) => status.name().to_string(),
			Outcome::Pending => "pending".to_string(),
		}
	}
}

impl Step {
	/// The step's line in a script: its thread, the op's name, then its slots and numbers,
	/// `-` for an absent handle.
	pub fn line(&self) -> String {
		let opt = |h: Option<usize>| h.map_or("-".to_string(), |h| h.to_string());
		let op = match self.op {
			Op::CounterNew(h, n) => format!("counter_new {h} {n}"),
			Op::CounterGet(h) => format!("counter_get {h}"),
			Op::CounterAdd(h, n) => format!("counter_add {h} {n}"),
			Op::CounterIntoSum(h, n) => format!("counter_into_sum {h} {n}"),
			Op::CounterPlus(h, o) => format!("counter_plus {h} {}", opt(o)),
			Op::CounterAddTo(h, o) => format!("counter_add_to {h} {}", opt(o)),
			Op::CounterClone(to, h) => format!("counter_clone {to} {h}"),
			Op::PlusLater(to, h, o) => format!("plus_later {to} {h} {}", opt(o)),
			Op::LedgerNew(h) => format!("ledger_new {h}"),
			Op::LedgerEdit(to, h) => format!("ledger_edit {to} {h}"),
			Op::LedgerPublished(h) => format!("ledger_published {h}"),
			Op::LedgerSeal(h) => format!("ledger_seal {h}"),
			Op::LedgerReceipt(to, h) => format!("ledger_receipt {to} {h}"),
			Op::DraftWrite(h) => format!("draft_write {h}"),
			Op::DraftLines(h) => format!("draft_lines {h}"),
			Op::DraftCommit(h) => format!("draft_commit {h}"),
			Op::ReceiptCount(h) => format!("receipt_count {h}"),
			Op::ReceiptClone(to, h) => format!("receipt_clone {to} {h}"),
			Op::Tallied(h) => format!("tallied {h}"),
			Op::Stored(h) => format!("stored {h}"),
			Op::TallyNew(h) => format!("tally_new {h}"),
			Op::TallyBump(h) => format!("tally_bump {h}"),
			Op::BumpLater(to, h) => format!("bump_later {to} {h}"),
			Op::LocalNew(h, homed) => format!("local_new {h} {}", u8::from(homed)),
			Op::LocalBump(h) => format!("local_bump {h}"),
			Op::LocalShare(to, h) => format!("local_share {to} {h}"),
			Op::LocalLater(to, h) => format!("local_later {to} {h}"),
			Op::MetersNew(h) => format!("meters_new {h}"),
			Op::MetersFirst(to, h) => format!("meters_first {to} {h}"),
			Op::MetersClear(h) => format!("meters_clear {h}"),
			Op::MeterNew(h) => format!("meter_new {h}"),
			Op::MeterAdd(h, n) => format!("meter_add {h} {n}"),
			Op::MeterReset(h) => format!("meter_reset {h}"),
			Op::Drop(h) => format!("drop {h}"),
			Op::Free(h) => format!("free {h}"),
			Op::Poll(h) => format!("poll {h}"),
			Op::Await(h) => format!("await {h}"),
			Op::Drain => "drain".to_string(),
		};
		format!("{} {op}", self.thread)
	}
}

/// A script: its steps, the kind of each slot, and what the counting waker must have seen
/// once every step ran and each thread exited.
pub struct Script {
	pub seed: u64,
	pub profile: Profile,
	pub steps: Vec<Step>,
	pub kinds: Vec<Kind>,
	pub wakes: u64,
	pub drops: u64,
}

impl Script {
	/// The script as a driver reads it: the slots' kinds on the first line, then a step a
	/// line.
	pub fn text(&self) -> String {
		let mut text = String::new();
		let kinds: Vec<_> = self.kinds.iter().map(|kind| kind.name()).collect();
		writeln!(text, "{}", kinds.join(" ")).unwrap();
		for step in &self.steps {
			writeln!(text, "{}", step.line()).unwrap();
		}
		text
	}

	/// What a driver prints: each step's outcome a line, then the waker's counts if it polls
	/// with the script's waker.
	pub fn transcript(&self) -> String {
		let mut text = String::new();
		for step in &self.steps {
			writeln!(text, "{}", step.expect.line()).unwrap();
		}
		if self.profile.polls {
			writeln!(text, "wakes {} drops {}", self.wakes, self.drops).unwrap();
		}
		text
	}
}

/// The api fixture's `LIMIT`, past which `Counter::add` fails.
const LIMIT: u64 = 1000;

/// Polls a fixture future pends before its third finishes it.
const POLLS: u32 = 3;

/// Where a value that is not `Send` stays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Owner {
	/// Any thread may use it.
	Any,
	/// The script's thread that made it.
	Thread(usize),
	/// A home: a call from any thread runs there.
	Home,
}

/// Something a value's drop gives back: what is observable of it.
#[derive(Clone, Copy, Debug)]
enum Release {
	/// A draft's drop: publishes if written, then ends the ledger's lend.
	Draft { ledger: usize, edited: bool },
	/// A receipt's drop: ends its share of the ledger's lend.
	Receipt { ledger: usize },
	/// A future's drop: gives back what it borrows, and drops the waker it adopted.
	Future { slot: usize },
	/// Nothing the script observes.
	Nothing,
}

/// A slot's handle and the value it holds.
#[derive(Clone, Debug)]
struct Slot {
	kind: Kind,
	/// The handle was freed: no later step names it.
	freed: bool,
	/// The value was moved out or dropped.
	gone: bool,
	/// A binding closed its handle: by a close, or by a move where its moves close it.
	closed: bool,
	owner: Owner,
	/// Futures borrowing it shared.
	shared: u32,
	/// A future or lend borrowing it exclusively: its count of holders.
	exclusive: u32,
	/// For a handle lent from another: that slot, and its epoch then.
	lender: Option<(usize, u64)>,
	/// Bumped by each successful exclusive borrow, move, drop, and free, ending what it lent.
	epoch: u64,
	/// A counter's or meter's value, a ledger's published count, a tally's count, or a
	/// local's count group.
	n: u64,
	/// A ledger's lines and seal, or a draft's or receipt's ledger.
	ledger: Option<usize>,
	lines: u64,
	sealed: bool,
	/// A draft written through, which its drop or commit publishes.
	edited: bool,
	/// A future's polls so far, whether it finished, and whether it adopted the waker.
	polls: u32,
	done: bool,
	adopted: bool,
	/// A future's borrows: shared slots and an exclusive one.
	holds_shared: Vec<usize>,
	holds_exclusive: Option<usize>,
}

impl Slot {
	fn new(kind: Kind, owner: Owner) -> Self {
		Self {
			kind,
			freed: false,
			gone: false,
			closed: false,
			owner,
			shared: 0,
			exclusive: 0,
			lender: None,
			epoch: 0,
			n: 0,
			ledger: None,
			lines: 0,
			sealed: false,
			edited: false,
			polls: 0,
			done: false,
			adopted: false,
			holds_shared: Vec::new(),
			holds_exclusive: None,
		}
	}
}

/// The model's state as a script runs.
#[derive(Clone)]
struct Model {
	/// What the surface runs, which decides where values stay and what a binding checks.
	profile: Profile,
	slots: Vec<Slot>,
	/// Each thread's drops other threads queued, run at its next check of one of its values.
	queued: [Vec<Release>; THREADS],
	/// Each local count group's count.
	groups: Vec<u64>,
	wakes: u64,
	drops: u64,
}

impl Model {
	fn new(profile: Profile) -> Self {
		Self {
			profile,
			slots: Vec::new(),
			queued: Default::default(),
			groups: Vec::new(),
			wakes: 0,
			drops: 0,
		}
	}

	fn add(&mut self, slot: Slot) -> usize {
		self.slots.push(slot);
		self.slots.len() - 1
	}

	/// Run what other threads queued for `thread`.
	fn drain(&mut self, thread: usize) {
		for release in std::mem::take(&mut self.queued[thread]) {
			self.release(release);
		}
	}

	/// Whether `thread` may use `h`'s value, draining its queue if the value is its own,
	/// as the runtime's check does.
	fn own(&mut self, h: usize, thread: usize) -> Result<(), Status> {
		match self.slots[h].owner {
			Owner::Any | Owner::Home => Ok(()),
			Owner::Thread(t) if t == thread => {
				self.drain(thread);
				Ok(())
			}
			Owner::Thread(_) => Err(Status::NotSend),
		}
	}

	/// For a lent handle, whether its lender still lends it.
	fn lender(&self, h: usize) -> Result<(), Status> {
		match self.slots[h].lender {
			Some((lender, epoch)) if self.slots[lender].epoch != epoch => Err(Status::Outlived),
			_ => Ok(()),
		}
	}

	/// A shared borrow of `h` for a call or a future.
	fn shared(&mut self, h: usize, thread: usize) -> Result<(), Status> {
		self.own(h, thread)?;
		let slot = &self.slots[h];
		if slot.gone {
			return Err(Status::Moved);
		}
		if slot.exclusive > 0 {
			return Err(Status::Exclusive);
		}
		self.lender(h)
	}

	/// An exclusive borrow of `h`, or a move out of it, for a call or future.
	fn exclusive(&mut self, h: usize, thread: usize) -> Result<(), Status> {
		self.own(h, thread)?;
		let slot = &self.slots[h];
		// A lent handle is never borrowed exclusively nor moved out. A lend's holder moves
		// out only into the call that ends it, which [`Self::ending`] checks.
		if slot.lender.is_some() {
			return Err(Status::Lent);
		}
		if slot.gone {
			return Err(Status::Moved);
		}
		if slot.shared > 0 || slot.exclusive > 0 {
			return Err(Status::Borrowed);
		}
		Ok(())
	}

	/// A move of a lend's holder into the call that ends its lend: the one move it makes.
	fn ending(&mut self, h: usize, thread: usize) -> Result<(), Status> {
		self.own(h, thread)?;
		let slot = &self.slots[h];
		if slot.gone {
			return Err(Status::Moved);
		}
		if slot.shared > 0 || slot.exclusive > 0 {
			return Err(Status::Borrowed);
		}
		self.changed(h);
		self.slots[h].gone = true;
		Ok(())
	}

	/// An exclusive borrow that ran: it ends what the value lent.
	fn changed(&mut self, h: usize) {
		self.slots[h].epoch += 1;
	}

	/// The value of `h` dropped on `thread`: here, or queued to its own thread.
	fn drop_value(&mut self, h: usize, thread: usize) {
		let release = self.releases(h);
		match self.slots[h].owner {
			Owner::Thread(t) if t != thread => self.queued[t].push(release),
			_ => self.release(release),
		}
	}

	/// What dropping `h`'s value gives back.
	fn releases(&self, h: usize) -> Release {
		let slot = &self.slots[h];
		match slot.kind {
			Kind::Draft => Release::Draft {
				ledger: slot.ledger.expect("a draft's ledger"),
				edited: slot.edited,
			},
			Kind::Receipt => Release::Receipt {
				ledger: slot.ledger.expect("a receipt's ledger"),
			},
			kind if kind.future() => Release::Future { slot: h },
			_ => Release::Nothing,
		}
	}

	fn release(&mut self, release: Release) {
		match release {
			Release::Draft { ledger, edited } => {
				if edited {
					self.slots[ledger].n += 1;
				}
				self.slots[ledger].exclusive -= 1;
			}
			Release::Receipt { ledger } => self.slots[ledger].exclusive -= 1,
			Release::Future { slot } => self.end_future(slot),
			Release::Nothing => {}
		}
	}

	/// A future finished or dropped: it gives back its borrows and its waker.
	fn end_future(&mut self, f: usize) {
		let held = std::mem::take(&mut self.slots[f].holds_shared);
		for h in held {
			self.slots[h].shared -= 1;
		}
		if let Some(h) = self.slots[f].holds_exclusive.take() {
			self.slots[h].exclusive -= 1;
		}
		if std::mem::take(&mut self.slots[f].adopted) {
			self.drops += 1;
		}
		self.slots[f].done = true;
	}

	/// Run `op` on `thread`: what it returns.
	fn step(&mut self, thread: usize, op: Op) -> Outcome {
		if self.profile.closed_first {
			if op.handles().iter().any(|&h| self.slots[h].closed) {
				return Outcome::Failed(Status::Moved);
			}
			// A second close never reaches Rust, so it drains nothing either.
			if let Op::Drop(h) = op
				&& self.slots[h].closed
			{
				return Outcome::Ok;
			}
		}
		let outcome = match self.run(thread, op) {
			Ok(outcome) => outcome,
			Err(status) => Outcome::Failed(status),
		};
		if !matches!(outcome, Outcome::Failed(_)) {
			match op {
				Op::Drop(h) => self.slots[h].closed = true,
				Op::CounterIntoSum(h, _) | Op::DraftCommit(h) | Op::Tallied(h) if self.profile.moves_close => {
					self.slots[h].closed = true;
				}
				_ => {}
			}
		}
		outcome
	}

	fn run(&mut self, thread: usize, op: Op) -> Result<Outcome, Status> {
		// Where a value that is not `Send` a call makes stays.
		let here = match self.profile.homed {
			true => Owner::Home,
			false => Owner::Thread(thread),
		};
		Ok(match op {
			Op::CounterNew(h, n) => {
				let mut slot = Slot::new(Kind::Counter, Owner::Any);
				slot.n = n;
				assert_eq!(self.add(slot), h);
				Outcome::Ok
			}
			Op::CounterGet(h) => {
				self.shared(h, thread)?;
				Outcome::Value(self.slots[h].n)
			}
			Op::CounterAdd(h, n) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				let v = self.slots[h].n + n;
				if v > LIMIT {
					return Ok(Outcome::Error);
				}
				self.slots[h].n = v;
				Outcome::Value(v)
			}
			Op::CounterIntoSum(h, n) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				self.slots[h].gone = true;
				Outcome::Value(self.slots[h].n + n)
			}
			Op::CounterPlus(h, o) => {
				self.shared(h, thread)?;
				if let Some(o) = o {
					self.shared(o, thread)?;
				}
				Outcome::Value(self.slots[h].n + o.map_or(0, |o| self.slots[o].n))
			}
			Op::CounterAddTo(h, o) => {
				self.shared(h, thread)?;
				// The receiver's borrow stands while the other is checked.
				self.slots[h].shared += 1;
				let other = o.map(|o| self.exclusive(o, thread)).transpose();
				self.slots[h].shared -= 1;
				other?;
				match o {
					Some(o) => {
						self.changed(o);
						self.slots[o].n += self.slots[h].n;
						Outcome::Value(1)
					}
					None => Outcome::Value(0),
				}
			}
			Op::CounterClone(to, h) => {
				self.shared(h, thread)?;
				let mut slot = Slot::new(Kind::Counter, Owner::Any);
				slot.n = self.slots[h].n;
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::PlusLater(to, h, o) => {
				self.shared(h, thread)?;
				self.slots[h].shared += 1;
				if let Some(o) = o
					&& let Err(status) = self.shared(o, thread)
				{
					self.slots[h].shared -= 1;
					return Err(status);
				}
				let mut slot = Slot::new(Kind::PlusLater, Owner::Any);
				slot.holds_shared.push(h);
				if let Some(o) = o {
					self.slots[o].shared += 1;
					slot.holds_shared.push(o);
				}
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::LedgerNew(h) => {
				assert_eq!(self.add(Slot::new(Kind::Ledger, Owner::Any)), h);
				Outcome::Ok
			}
			Op::LedgerEdit(to, h) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				if self.slots[h].sealed {
					return Ok(Outcome::Error);
				}
				self.slots[h].exclusive += 1;
				let mut slot = Slot::new(Kind::Draft, here);
				slot.ledger = Some(h);
				assert_eq!(self.add(slot), to);
				Outcome::Value(1)
			}
			Op::LedgerPublished(h) => {
				self.shared(h, thread)?;
				Outcome::Value(self.slots[h].n)
			}
			Op::LedgerSeal(h) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				self.slots[h].sealed = true;
				Outcome::Ok
			}
			Op::LedgerReceipt(to, h) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				self.slots[h].exclusive += 1;
				let mut slot = Slot::new(Kind::Receipt, Owner::Any);
				slot.ledger = Some(h);
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::DraftWrite(h) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				self.slots[h].edited = true;
				let ledger = self.slots[h].ledger.expect("a draft's ledger");
				self.slots[ledger].lines += 1;
				Outcome::Ok
			}
			Op::DraftLines(h) => {
				self.shared(h, thread)?;
				let ledger = self.slots[h].ledger.expect("a draft's ledger");
				Outcome::Value(self.slots[ledger].lines)
			}
			Op::DraftCommit(h) => {
				self.ending(h, thread)?;
				let ledger = self.slots[h].ledger.expect("a draft's ledger");
				self.release(Release::Draft {
					ledger,
					edited: self.slots[h].edited,
				});
				Outcome::Value(self.slots[ledger].n)
			}
			Op::Tallied(h) => {
				self.ending(h, thread)?;
				let ledger = self.slots[h].ledger.expect("a receipt's ledger");
				self.release(Release::Receipt { ledger });
				Outcome::Value(self.slots[ledger].n)
			}
			// Refused by its signature alone, whatever the receipt's state.
			Op::Stored(_) => return Err(Status::Lent),
			Op::ReceiptCount(h) => {
				self.shared(h, thread)?;
				let ledger = self.slots[h].ledger.expect("a receipt's ledger");
				Outcome::Value(self.slots[ledger].n)
			}
			Op::ReceiptClone(to, h) => {
				self.shared(h, thread)?;
				let ledger = self.slots[h].ledger.expect("a receipt's ledger");
				self.slots[ledger].exclusive += 1;
				let mut slot = Slot::new(Kind::Receipt, Owner::Any);
				slot.ledger = Some(ledger);
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::TallyNew(h) => {
				assert_eq!(self.add(Slot::new(Kind::Tally, Owner::Any)), h);
				Outcome::Ok
			}
			Op::TallyBump(h) => {
				self.shared(h, thread)?;
				self.slots[h].n += 1;
				Outcome::Value(self.slots[h].n)
			}
			Op::BumpLater(to, h) => {
				self.exclusive(h, thread)?;
				self.slots[h].exclusive += 1;
				// Made from no value that is not `Send`, it may live on a home.
				let owner = if self.profile.homed_futures { Owner::Home } else { here };
				let mut slot = Slot::new(Kind::BumpLater, owner);
				slot.holds_exclusive = Some(h);
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::LocalNew(h, homed) => {
				let mut slot = Slot::new(Kind::Local, if homed { Owner::Home } else { here });
				assert!(
					homed || !self.profile.homed,
					"a homed profile makes every local on a home"
				);
				self.groups.push(0);
				slot.n = (self.groups.len() - 1) as u64;
				assert_eq!(self.add(slot), h);
				Outcome::Ok
			}
			Op::LocalBump(h) => {
				self.shared(h, thread)?;
				let group = self.slots[h].n as usize;
				self.groups[group] += 1;
				Outcome::Value(self.groups[group])
			}
			Op::LocalShare(to, h) => {
				self.shared(h, thread)?;
				let mut slot = Slot::new(Kind::Local, self.slots[h].owner);
				slot.n = self.slots[h].n;
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::LocalLater(to, h) => {
				self.exclusive(h, thread)?;
				self.slots[h].exclusive += 1;
				let mut slot = Slot::new(Kind::LocalLater, self.slots[h].owner);
				slot.holds_exclusive = Some(h);
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::MetersNew(h) => {
				assert_eq!(self.add(Slot::new(Kind::Meters, Owner::Any)), h);
				Outcome::Ok
			}
			Op::MetersFirst(to, h) => {
				self.shared(h, thread)?;
				let mut slot = Slot::new(Kind::Meter, Owner::Any);
				slot.lender = Some((h, self.slots[h].epoch));
				assert_eq!(self.add(slot), to);
				Outcome::Ok
			}
			Op::MetersClear(h) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				// The first meter is new, so a handle lent from it next counts from zero.
				self.slots[h].n = 0;
				Outcome::Ok
			}
			Op::MeterNew(h) => {
				assert_eq!(self.add(Slot::new(Kind::Meter, Owner::Any)), h);
				Outcome::Ok
			}
			Op::MeterAdd(h, n) => {
				self.shared(h, thread)?;
				// A lent meter counts in its lender's first meter.
				let at = self.slots[h].lender.map_or(h, |(lender, _)| lender);
				self.slots[at].n += n;
				Outcome::Value(self.slots[at].n)
			}
			Op::MeterReset(h) => {
				self.exclusive(h, thread)?;
				self.changed(h);
				self.slots[h].n = 0;
				Outcome::Ok
			}
			Op::Drop(h) => {
				// Elsewhere than its own thread, it drops nothing there: it queues the drop.
				if !matches!(self.slots[h].owner, Owner::Thread(t) if t != thread) {
					self.own(h, thread)?;
				}
				if self.slots[h].gone {
					return Ok(Outcome::Ok);
				}
				if self.slots[h].shared > 0 || self.slots[h].exclusive > 0 {
					return Err(Status::Borrowed);
				}
				self.changed(h);
				self.slots[h].gone = true;
				// A lent handle only stops lending; its value is its lender's.
				if self.slots[h].lender.is_none() {
					self.drop_value(h, thread);
				}
				Outcome::Ok
			}
			Op::Free(h) => {
				let slot = &mut self.slots[h];
				slot.freed = true;
				if slot.kind.future() {
					if !slot.done {
						self.drop_value(h, thread);
					}
					return Ok(Outcome::Ok);
				}
				slot.epoch += 1;
				// A future borrowing the value keeps it; none here observes its drop then.
				if !slot.gone && slot.lender.is_none() {
					self.drop_value(h, thread);
				}
				Outcome::Ok
			}
			Op::Await(f) => {
				let mut outcome = self.run(thread, Op::Poll(f))?;
				while outcome == Outcome::Pending {
					outcome = self.run(thread, Op::Poll(f))?;
				}
				outcome
			}
			Op::Poll(f) => {
				if let Owner::Thread(t) = self.slots[f].owner
					&& t != thread
				{
					// A waker it never adopted is Rust's to drop at once.
					if !self.slots[f].adopted {
						self.drops += 1;
					}
					return Err(Status::NotSend);
				}
				self.own(f, thread)?;
				self.slots[f].adopted = true;
				self.slots[f].polls += 1;
				if self.slots[f].polls < POLLS {
					// The fixture's future wakes its waker as it pends.
					self.wakes += 1;
					return Ok(Outcome::Pending);
				}
				let value = match self.slots[f].kind {
					Kind::PlusLater => self.slots[f].holds_shared.iter().map(|&h| self.slots[h].n).sum(),
					Kind::BumpLater => {
						let h = self.slots[f].holds_exclusive.expect("a tally");
						self.slots[h].n += 1;
						self.slots[h].n
					}
					Kind::LocalLater => {
						let h = self.slots[f].holds_exclusive.expect("a local");
						let group = self.slots[h].n as usize;
						self.groups[group] += 1;
						self.groups[group]
					}
					kind => unreachable!("{kind:?} is no future"),
				};
				self.end_future(f);
				Outcome::Value(value)
			}
			Op::Drain => {
				self.drain(thread);
				Outcome::Ok
			}
		})
	}
}

/// splitmix64: a seed makes the same script on every host.
struct Rng(u64);

impl Rng {
	fn next(&mut self) -> u64 {
		self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
		let mut z = self.0;
		z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
		z ^ (z >> 31)
	}

	fn below(&mut self, n: usize) -> usize {
		(self.next() % n as u64) as usize
	}

	fn chance(&mut self, percent: u64) -> bool {
		self.next() % 100 < percent
	}

	fn pick<T: Copy>(&mut self, items: &[T]) -> Option<T> {
		(!items.is_empty()).then(|| items[self.below(items.len())])
	}
}

/// The most handles a script keeps live at once, so steps meet the ones they conflict with.
const LIVE: usize = 8;

/// A script of `len` random steps from `seed` for a surface of `profile`, then the steps
/// that end every handle left: each future freed, or awaited where the profile frees
/// nothing amid the steps, then each object freed or dropped, each on its own thread, and
/// a drain of each thread, so nothing is left for a thread's exit to drop.
pub fn script(seed: u64, len: usize, profile: Profile) -> Script {
	let mut rng = Rng(seed);
	let mut model = Model::new(profile);
	let mut steps = Vec::new();
	let push = |model: &mut Model, steps: &mut Vec<Step>, thread: usize, op: Op| {
		let expect = model.step(thread, op);
		steps.push(Step { thread, op, expect });
	};
	while steps.len() < len {
		// Most steps run on the first thread, so its values meet one another.
		let thread = usize::from(profile.threads && rng.chance(25));
		let Some(op) = pick(&mut rng, &model, profile) else {
			continue;
		};
		if !profile.failures && matches!(model.clone().step(thread, op), Outcome::Failed(_)) {
			continue;
		}
		push(&mut model, &mut steps, thread, op);
		// A surface that runs a future to its end in the call that makes it awaits it next.
		if profile.pending == 0
			&& let Some(f) = op.future()
			&& steps
				.last()
				.is_some_and(|step| !matches!(step.expect, Outcome::Failed(_)))
		{
			push(&mut model, &mut steps, thread, Op::Await(f));
		}
	}
	let live = |model: &Model, kinds: &[Kind]| -> Vec<usize> {
		(0..model.slots.len())
			.filter(|&h| {
				let slot = &model.slots[h];
				!slot.freed && kinds.contains(&slot.kind) && !(slot.kind.future() && slot.done && !profile.frees)
			})
			.collect()
	};
	let home = |model: &Model, h: usize| match model.slots[h].owner {
		Owner::Thread(t) => t,
		_ => 0,
	};
	// Futures first, which may hold the objects, then each lend's holder, then the rest.
	let futures = [Kind::PlusLater, Kind::BumpLater, Kind::LocalLater];
	let lends = [Kind::Draft, Kind::Receipt];
	let rest = [
		Kind::Counter,
		Kind::Ledger,
		Kind::Tally,
		Kind::Local,
		Kind::Meters,
		Kind::Meter,
	];
	// Without frees, a future runs to its end: polled until it is, where the profile polls.
	if !profile.frees && profile.polls {
		for f in live(&model, &futures) {
			let thread = home(&model, f);
			while !model.slots[f].done {
				push(&mut model, &mut steps, thread, Op::Poll(f));
			}
			model.slots[f].freed = true;
		}
	}
	for (kinds, end) in [
		(&futures[..], if profile.frees { Op::Free } else { Op::Await }),
		(&lends[..], if profile.frees { Op::Free } else { Op::Drop }),
		(&rest[..], if profile.frees { Op::Free } else { Op::Drop }),
	] {
		for h in live(&model, kinds) {
			let thread = home(&model, h);
			push(&mut model, &mut steps, thread, end(h));
			// Ended, whether freed or awaited or dropped.
			model.slots[h].freed = true;
		}
	}
	// Without a drain, what is left queued drops as each thread exits, which no step sees.
	if profile.drain {
		for thread in 0..if profile.threads { THREADS } else { 1 } {
			push(&mut model, &mut steps, thread, Op::Drain);
		}
		assert!(model.queued.iter().all(Vec::is_empty), "a drop is left queued");
	}
	Script {
		seed,
		profile,
		steps,
		kinds: model.slots.iter().map(|slot| slot.kind).collect(),
		wakes: model.wakes,
		drops: model.drops,
	}
}

/// A random op valid to run on any thread: one naming no freed handle, polling no finished
/// future, breaking no contract that aborts rather than fails a check, and freeing no value
/// on a home whose drop the script would observe, since the home drops it in its own time.
fn pick(rng: &mut Rng, model: &Model, profile: Profile) -> Option<Op> {
	let next = model.slots.len();
	let live: Vec<usize> = (0..next).filter(|&h| !model.slots[h].freed).collect();
	let of = |kinds: &[Kind]| -> Vec<usize> {
		live.iter()
			.copied()
			.filter(|&h| kinds.contains(&model.slots[h].kind))
			.collect()
	};
	let counters = of(&[Kind::Counter]);
	let some = |rng: &mut Rng, items: &[usize]| if rng.chance(80) { rng.pick(items) } else { None };
	let n = |rng: &mut Rng| rng.next() % 300;
	let objects: Vec<usize> = live
		.iter()
		.copied()
		.filter(|&h| !model.slots[h].kind.future())
		.collect();
	let futures: Vec<usize> = live
		.iter()
		.copied()
		.filter(|&h| model.slots[h].kind.future() && !model.slots[h].done)
		.collect();
	let making = live.len() < LIVE;
	// A future is made only while fewer than the profile allows pend, one where it awaits
	// each at once.
	let future = making && futures.len() < profile.pending.max(1);
	Some(match rng.below(14) {
		0 if making => match rng.below(6) {
			0 => Op::CounterNew(next, rng.next() % 100),
			1 => Op::LedgerNew(next),
			2 => Op::TallyNew(next),
			3 => Op::LocalNew(next, profile.homed || (profile.homes && rng.chance(25))),
			4 => Op::MetersNew(next),
			_ => Op::MeterNew(next),
		},
		1 => {
			let h = rng.pick(&counters)?;
			match rng.below(7) {
				0 => Op::CounterGet(h),
				1 => Op::CounterAdd(h, n(rng)),
				2 => Op::CounterIntoSum(h, n(rng)),
				3 => Op::CounterPlus(h, some(rng, &counters)),
				4 => Op::CounterAddTo(h, some(rng, &counters)),
				5 if making => Op::CounterClone(next, h),
				_ if future => Op::PlusLater(next, h, some(rng, &counters)),
				_ => Op::CounterGet(h),
			}
		}
		2 => {
			let h = rng.pick(&of(&[Kind::Ledger]))?;
			match rng.below(4) {
				0 if making => Op::LedgerEdit(next, h),
				1 if making => Op::LedgerReceipt(next, h),
				2 => Op::LedgerSeal(h),
				_ => Op::LedgerPublished(h),
			}
		}
		3 => {
			let h = rng.pick(&of(&[Kind::Draft]))?;
			match rng.below(3) {
				0 => Op::DraftWrite(h),
				1 => Op::DraftLines(h),
				_ => Op::DraftCommit(h),
			}
		}
		4 => {
			let h = rng.pick(&of(&[Kind::Receipt]))?;
			match rng.below(5) {
				0 if making => Op::ReceiptClone(next, h),
				1 => Op::Tallied(h),
				2 => Op::Stored(h),
				_ => Op::ReceiptCount(h),
			}
		}
		5 => {
			let h = rng.pick(&of(&[Kind::Tally]))?;
			match rng.below(2) {
				0 if future => Op::BumpLater(next, h),
				_ => Op::TallyBump(h),
			}
		}
		6 => {
			let h = rng.pick(&of(&[Kind::Local]))?;
			match rng.below(3) {
				0 if making => Op::LocalShare(next, h),
				// A home's future would drop on the home in its own time, after its waker's
				// last count is read: only a thread's locals make one for the script's waker.
				1 if future && (model.slots[h].owner != Owner::Home || !profile.polls) => Op::LocalLater(next, h),
				_ => Op::LocalBump(h),
			}
		}
		7 => {
			let h = rng.pick(&of(&[Kind::Meters]))?;
			match rng.below(2) {
				0 if making => Op::MetersFirst(next, h),
				_ => Op::MetersClear(h),
			}
		}
		8 => {
			let h = rng.pick(&of(&[Kind::Meter]))?;
			match rng.below(2) {
				0 => Op::MeterAdd(h, n(rng)),
				_ => Op::MeterReset(h),
			}
		}
		9 | 10 => {
			let f = rng.pick(&futures)?;
			if profile.polls { Op::Poll(f) } else { Op::Await(f) }
		}
		11 => Op::Drop(rng.pick(&objects)?),
		12 if profile.frees => {
			let h = rng.pick(&live)?;
			let slot = &model.slots[h];
			let racy =
				slot.owner == Owner::Home && !slot.gone && !slot.done && !matches!(model.releases(h), Release::Nothing);
			if racy {
				return None;
			}
			Op::Free(h)
		}
		_ if profile.drain => Op::Drain,
		_ => return None,
	})
}
