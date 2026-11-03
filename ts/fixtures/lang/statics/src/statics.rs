//! `static` atomics, each one value every use shares: of every kind, at module level and
//! in a function, as kio's `waiter` keeps its ids, starting from a value of the type's
//! extremes. Statics of any type, read from the memory rustc evaluated: a struct holding
//! an atomic, an `AtomicPtr` as parking_lot keeps its table in, std's `OnceLock`,
//! `LazyLock`, and `Once`, crossbeam-epoch's own `OnceLock` over an `UnsafeCell`, and one
//! pointing at a static of a module that reads it back; and `const`s holding cells, each
//! use of which is a value of its own, as lock_api's `RawMutex::INIT` is. Each case leaves
//! every static as it found it, or reads only what it holds whatever ran before, so a fresh
//! instance and a reused one agree.

use std::cell::{Cell, LazyCell, UnsafeCell};
use std::mem::MaybeUninit;
use std::ptr;
use std::rc::Rc;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, SeqCst};
use std::sync::atomic::{AtomicBool, AtomicI8, AtomicI64, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize};
use std::sync::{Arc, LazyLock, Once, OnceLock};

static COUNT: AtomicU64 = AtomicU64::new(u64::MAX - 2);
static SIGNED: AtomicI8 = AtomicI8::new(-128);
static FLAG: AtomicBool = AtomicBool::new(true);
static WIDE: AtomicI64 = AtomicI64::new(i64::MIN + 1);
static SIZE: AtomicUsize = AtomicUsize::new(7);
/// Read, never written: the static's memory.
static TABLE: [u8; 4] = [9, 8, 7, 6];

/// Each byte an operation on the statics; each puts what it read. First, what each holds,
/// which only the value it starts with can be.
pub fn statics(data: &[u8], out: &mut Vec<u8>) {
	out.extend(COUNT.load(Relaxed).to_le_bytes());
	out.extend(WIDE.load(Relaxed).to_le_bytes());
	out.extend([
		SIGNED.load(Relaxed) as u8,
		u8::from(FLAG.load(Relaxed)),
		SIZE.load(Relaxed) as u8,
	]);
	out.push(next(false));
	for &op in data {
		let n = op / 6;
		match op % 6 {
			// Wraps past the type's end, as an atomic's arithmetic does.
			0 => out.extend(COUNT.fetch_add(u64::from(n), AcqRel).to_le_bytes()),
			1 => out.push(SIGNED.fetch_sub(n as i8, SeqCst) as u8),
			2 => out.push(u8::from(FLAG.fetch_xor(n % 2 == 0, Relaxed))),
			3 => out.extend(WIDE.swap(-i64::from(n), Acquire).to_le_bytes()),
			4 => {
				let seen = SIZE.load(Relaxed);
				out.push(
					match SIZE.compare_exchange(seen, seen.wrapping_mul(usize::from(n)), SeqCst, Relaxed) {
						Ok(old) => old as u8,
						Err(_) => 0xfd,
					},
				);
			}
			5 if n % 2 == 0 => out.push(next(false)),
			// One table, wherever it is borrowed.
			_ => out.extend([table()[usize::from(n) % 4], u8::from(std::ptr::eq(table(), table()))]),
		}
	}
	COUNT.store(u64::MAX - 2, Relaxed);
	SIGNED.store(-128, Relaxed);
	FLAG.store(true, Relaxed);
	WIDE.store(i64::MIN + 1, Relaxed);
	SIZE.store(7, Relaxed);
	next(true);
}

fn table() -> &'static [u8; 4] {
	&TABLE
}

/// The next id from a static in a function, as kio's `waiter` takes one; starts over if `reset`.
fn next(reset: bool) -> u8 {
	static NEXT: AtomicU64 = AtomicU64::new(1);
	match reset {
		true => {
			NEXT.store(1, Relaxed);
			0
		}
		false => NEXT.fetch_add(1, Relaxed) as u8,
	}
}

/// A static's counter beside what it never changes, as foldhash's seed storage keeps its state.
struct Counter {
	name: &'static str,
	hits: AtomicU32,
	limit: u32,
	steps: [u8; 3],
	state: Option<(u8, char)>,
}

static COUNTER: Counter = Counter {
	name: "hits",
	hits: AtomicU32::new(u32::MAX - 1),
	limit: 3,
	steps: [1, 2, 4],
	state: Some((9, 'z')),
};

/// A static pointing at one of a module that reads it back, so each module imports the other.
static SHARED: &AtomicU8 = &elsewhere::LATER;

mod elsewhere {
	use std::sync::atomic::AtomicU8;
	use std::sync::atomic::Ordering::Relaxed;

	pub static LATER: AtomicU8 = AtomicU8::new(40);

	/// Through the static that points here.
	pub fn read() -> u8 {
		super::SHARED.load(Relaxed)
	}
}

/// A lock's state as lock_api's `RawMutex::INIT` holds it: every use a lock of its own.
struct Raw {
	locked: AtomicBool,
	count: AtomicU8,
}

#[allow(
	clippy::declare_interior_mutable_const,
	reason = "each use is a fresh lock, as `RawMutex::INIT` is"
)]
const INIT: Raw = Raw {
	locked: AtomicBool::new(false),
	count: AtomicU8::new(200),
};

/// A boxed node a pointer static holds.
struct Node {
	value: u8,
}

/// Null, as parking_lot's `HASHTABLE` starts.
static SLOT: AtomicPtr<Node> = AtomicPtr::new(ptr::null_mut());

/// A null pointer constant, as wasmtimer's are.
const NONE: *mut Node = ptr::null_mut();

/// Filled only by the first call that gets here, with the value every call gives.
static ONCE: OnceLock<(u8, u32)> = OnceLock::new();

/// How many lazy statics' functions ran, and so the order they finished in.
static MADE: AtomicU8 = AtomicU8::new(0);

/// Made by a function item, when first forced.
static FIRST: LazyLock<Vec<u8>> = LazyLock::new(first);

/// Made by a closure, which forces `FIRST` before it finishes.
static SECOND: LazyLock<(u8, Vec<u8>)> = LazyLock::new(|| {
	let first = FIRST.clone();
	(MADE.fetch_add(1, Relaxed), first)
});

fn first() -> Vec<u8> {
	vec![MADE.fetch_add(1, Relaxed), 7, 8]
}

/// How many times `LAZY`'s function ran in this call.
static FORCED: AtomicU8 = AtomicU8::new(0);

/// Each use a cell of its own, which runs its function once, when first forced.
#[allow(clippy::declare_interior_mutable_const, reason = "each use is a cell of its own")]
const LAZY: LazyCell<u8> = LazyCell::new(|| FORCED.fetch_add(1, Relaxed) + 10);

/// Uninitialized, as moq-net's `UNINIT` frame is.
const EMPTY: MaybeUninit<u16> = MaybeUninit::uninit();

/// Each use a cell of its own.
#[allow(clippy::declare_interior_mutable_const, reason = "each use is a cell of its own")]
const TALLY: Cell<u8> = Cell::new(250);
#[allow(clippy::declare_interior_mutable_const, reason = "each use is a cell of its own")]
const PAIR: UnsafeCell<(u8, u8)> = UnsafeCell::new((1, 2));
#[allow(clippy::declare_interior_mutable_const, reason = "each use is a cell of its own")]
const NESTED: UnsafeCell<Cell<u8>> = UnsafeCell::new(Cell::new(3));
#[allow(clippy::declare_interior_mutable_const, reason = "each use is a cell of its own")]
const CELLS: Cell<Cell<u8>> = Cell::new(Cell::new(4));

/// Each byte an operation on statics and constants read from memory; each puts what it read.
pub fn cells(data: &[u8], out: &mut Vec<u8>) {
	out.extend(COUNTER.name.as_bytes());
	out.extend(COUNTER.steps);
	match COUNTER.state {
		Some((n, c)) => out.extend([n, c as u8]),
		None => out.push(0xff),
	}
	// Two uses of one `const`, each its own lock.
	let a = INIT;
	let b = INIT;
	let c = &INIT;
	// Every lazy static, forced from either path, has run its function once.
	let (made, from) = &*SECOND;
	out.extend([*made, FIRST[0], from[0], FIRST[1]]);
	let tally = TALLY;
	tally.set(tally.get().wrapping_add(10));
	out.extend([tally.get(), TALLY.get(), PAIR.into_inner().1]);
	out.extend([NESTED.into_inner().get(), CELLS.into_inner().into_inner()]);
	for &op in data {
		let n = op / 8;
		match op % 8 {
			// Wraps past `u32::MAX`, as an atomic's arithmetic does.
			0 => out.extend(
				COUNTER
					.hits
					.fetch_add(u32::from(n) % COUNTER.limit, AcqRel)
					.to_le_bytes(),
			),
			1 => out.push(u8::from(a.locked.swap(n % 2 == 0, Acquire))),
			2 => out.extend([
				a.count.fetch_add(n, Relaxed),
				b.count.load(Relaxed),
				c.count.load(Relaxed),
			]),
			3 => out.push(u8::from(b.locked.load(Relaxed))),
			4 => {
				// Swapped between null and a box, as parking_lot's table is grown.
				let node = Box::into_raw(Box::new(Node { value: n }));
				let old = SLOT.swap(node, AcqRel);
				out.push(u8::from(old.is_null()));
				out.push(unsafe { (*SLOT.load(Acquire)).value });
				match SLOT.compare_exchange(old, NONE, AcqRel, Acquire) {
					Ok(_) => out.push(0xfd),
					Err(seen) => out.push(u8::from(seen == node)),
				}
				match SLOT.compare_exchange(node, NONE, AcqRel, Acquire) {
					Ok(seen) => out.push(unsafe { Box::from_raw(seen) }.value),
					Err(_) => out.push(0xfc),
				}
				out.push(u8::from(SLOT.load(Relaxed).is_null()));
			}
			5 => {
				let (seen, len) = ONCE.get_or_init(|| (3, 9));
				out.extend([*seen, *len as u8, u8::from(ONCE.get().is_some())]);
			}
			6 => {
				// Neither cell runs its function before it is forced, and each runs it once.
				let x = LAZY;
				let y = LAZY;
				out.push(FORCED.load(Relaxed));
				out.extend([*y, *x, *y, *LazyCell::force(&x)]);
				out.push(FORCED.swap(0, Relaxed));
			}
			7 if n % 2 == 0 => {
				// One atomic, through either static.
				out.extend([
					SHARED.fetch_add(n, Relaxed),
					elsewhere::LATER.load(Relaxed),
					elsewhere::read(),
				]);
				elsewhere::LATER.store(40, Relaxed);
			}
			_ => {
				let mut slots = [EMPTY; 3];
				let i = usize::from(n) % 3;
				slots[i].write(u16::from(n) * 300);
				out.extend(unsafe { slots[i].assume_init() }.to_le_bytes());
			}
		}
	}
	COUNTER.hits.store(u32::MAX - 1, Relaxed);
}

/// crossbeam-epoch's own `OnceLock`, which keeps its collector: a value a `Once` guards,
/// written through `UnsafeCell::get` in `call_once`'s function, read once `is_completed`
/// says it ran, and dropped in place if it did.
struct Lock<T> {
	once: Once,
	value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Sync + Send> Sync for Lock<T> {}

impl<T> Lock<T> {
	const fn new() -> Self {
		Self {
			once: Once::new(),
			value: UnsafeCell::new(MaybeUninit::uninit()),
		}
	}

	fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
		if !self.once.is_completed() {
			let slot = self.value.get();
			self.once.call_once(|| {
				let value = f();
				unsafe { slot.write(MaybeUninit::new(value)) }
			});
		}
		unsafe { &*self.value.get().cast::<T>() }
	}
}

impl<T> Drop for Lock<T> {
	fn drop(&mut self) {
		if self.once.is_completed() {
			unsafe { (*self.value.get()).assume_init_drop() };
		}
	}
}

/// What crossbeam-epoch's collector holds: a struct JS keeps as an object.
struct Collector {
	global: Arc<u32>,
}

/// Filled by the first call that gets here, with the value every call gives.
static COLLECTOR: Lock<Collector> = Lock::new();

/// Run by the first call that gets here.
static STARTED: Once = Once::new();

/// Each byte an operation on a `Once`, its own or a static's; each puts what it saw. One
/// whose function runs again from within it panics, as wasm32's does, if its byte is odd.
pub fn onces(data: &[u8], out: &mut Vec<u8>) {
	let once = Once::new();
	// Owned by each function `call_once` drops uncalled.
	let owned = Rc::new(());
	for &op in data {
		let n = op / 6;
		match op % 6 {
			0 => {
				out.push(u8::from(once.is_completed()));
				once.call_once(|| out.push(n));
				out.push(u8::from(once.is_completed()));
			}
			1 => {
				let held = Rc::clone(&owned);
				once.call_once_force(|state| {
					out.extend([u8::from(state.is_poisoned()), u8::from(once.is_completed()), n]);
					drop(held);
				});
				out.push(Rc::strong_count(&owned) as u8);
			}
			2 => {
				let fresh = Once::new();
				fresh.call_once(|| {
					out.push(u8::from(fresh.is_completed()));
					if n % 2 == 1 {
						fresh.call_once(|| out.push(0xee));
					}
				});
				out.push(u8::from(fresh.is_completed()));
			}
			3 if n % 2 == 1 => {
				let fresh = Once::new();
				fresh.call_once_force(|_| fresh.call_once_force(|_| out.push(0xee)));
			}
			3 => {
				STARTED.call_once(|| {});
				out.push(u8::from(STARTED.is_completed()));
			}
			4 => {
				let collector = COLLECTOR.get_or_init(|| Collector {
					global: Arc::new(0x0102_0304),
				});
				out.extend(collector.global.to_le_bytes());
				out.push(Arc::strong_count(&collector.global) as u8);
			}
			// The first function given runs, and the second never does; dropping the lock
			// drops the value only if it holds one. A pointer to an `Rc` is a place, and to a
			// tuple the object.
			_ if n % 3 == 0 => {
				let slot = Lock::new();
				let held = Rc::clone(&owned);
				out.push(Rc::strong_count(slot.get_or_init(|| held)) as u8);
				let unused = Rc::clone(&owned);
				out.push(Rc::strong_count(slot.get_or_init(|| unused)) as u8);
				drop(slot);
				out.push(Rc::strong_count(&owned) as u8);
			}
			_ if n % 3 == 1 => {
				let slot = Lock::new();
				let held = Rc::clone(&owned);
				out.push(slot.get_or_init(|| (n, held)).0);
				let unused = Rc::clone(&owned);
				out.push(slot.get_or_init(|| (0xee, unused)).0);
				out.push(Rc::strong_count(&owned) as u8);
				drop(slot);
				out.push(Rc::strong_count(&owned) as u8);
			}
			_ => {
				let slot: Lock<Rc<()>> = Lock::new();
				drop(slot);
				out.push(Rc::strong_count(&owned) as u8);
			}
		}
	}
}
