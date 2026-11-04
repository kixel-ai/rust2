//! Thread-locals on one thread, each one value every use shares: of `Cell`s, `RefCell`s,
//! and a struct of both, as kio's `Polling` is, initialized by a `const` block or lazily
//! by an expression; read and written through `with`, given a closure or a function,
//! `try_with`, and the `Cell` and `RefCell` helpers, as kio's `coop` does; nested; and
//! declared in a function, as kio's `serial` is; and a copy of a key written whole through
//! a `&mut`, which then names the other's storage. Each case leaves
//! every thread-local as it found it, so a fresh instance and a reused one agree.

use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::mem;
use std::sync::{Arc, Mutex};
use std::thread::LocalKey;

/// Logs its id when it drops.
struct Noisy {
	id: u8,
	log: Arc<Mutex<Vec<u8>>>,
}

impl Drop for Noisy {
	fn drop(&mut self) {
		self.log.lock().unwrap().push(self.id);
	}
}

/// Only `PAIRS`'s `const` writes one in place.
#[derive(Clone, Copy)]
struct Twin(u16, u16);

/// What a batch holds, as kio's `Polling` does.
struct Polling {
	current: Cell<Option<(u64, u64)>>,
	held: RefCell<Vec<u8>>,
}

thread_local! {
	static COUNT: Cell<u32> = const { Cell::new(7) };
	static LOG: RefCell<Vec<u8>> = RefCell::new(vec![1, 2, 3]);
	static POLLING: Polling = const {
		Polling {
			current: Cell::new(None),
			held: RefCell::new(Vec::new()),
		}
	};
	static HELD: RefCell<Option<Noisy>> = const { RefCell::new(None) };
	static TALLY: RefCell<u32> = RefCell::new(0);
	// A copy the `const` makes, then changes the original of, of a type nothing else writes.
	static PAIRS: Cell<(Twin, Twin)> = const {
		let mut x = Twin(1, 2);
		let y = x;
		x.0 = 3;
		Cell::new((x, y))
	};
	/// The keys `keys` writes over each other's copies, initialized by a `const` block or lazily.
	static LEFT: Cell<u8> = const { Cell::new(1) };
	static RIGHT: Cell<u8> = Cell::new(2);
}

/// Each byte an operation on the thread-locals; each puts what it read and what dropped.
/// First, what each holds, which only the value it starts with can be.
pub fn locals(data: &[u8], out: &mut Vec<u8>) {
	out.push(COUNT.with(Cell::get) as u8);
	out.extend(LOG.with_borrow(|log| log.clone()));
	out.push(POLLING.with(|p| p.current.get().map_or(0xfd, |(a, _)| a as u8)));
	out.push(POLLING.with(|p| p.held.borrow().len() as u8));
	out.push(HELD.with_borrow(|held| held.as_ref().map_or(0xfd, |n| n.id)));
	out.push(TALLY.with_borrow(|tally| *tally as u8));
	let (Twin(a, b), Twin(c, d)) = PAIRS.get();
	out.extend([a as u8, b as u8, c as u8, d as u8]);
	out.push(serial(false));
	let log = Arc::new(Mutex::new(Vec::new()));
	for &op in data {
		match op % 9 {
			0 => {
				COUNT.with(|c| c.set(c.get().wrapping_add(u32::from(op))));
				out.push(COUNT.with(Cell::get) as u8);
			}
			1 => LOG.with(|log| log.borrow_mut().push(op)),
			2 => out.push(LOG.try_with(|log| log.borrow().len()).map_or(0xfd, |n| n as u8)),
			// One read within another.
			3 => out.push(COUNT.with(|c| LOG.with(|log| c.get() as u8 ^ log.borrow()[0]))),
			4 => {
				let outer = POLLING.with(|p| p.current.replace(Some((u64::from(op), 1))));
				let len = POLLING.with(|p| {
					p.held.borrow_mut().push(op);
					p.held.borrow().len()
				});
				out.push(outer.map_or(0xfd, |(a, _)| a as u8));
				out.push(len as u8);
			}
			// Replaces what it held, which drops.
			5 => HELD.with(|held| {
				*held.borrow_mut() = Some(Noisy {
					id: op,
					log: log.clone(),
				})
			}),
			6 => {
				COUNT.set(COUNT.get().wrapping_mul(3));
				out.push(COUNT.replace(u32::from(op)) as u8);
				out.push(COUNT.take() as u8);
				COUNT.set(u32::from(op) + 1);
				let (Twin(a, b), Twin(c, d)) = PAIRS.get();
				out.extend([a as u8, b as u8, c as u8, d as u8]);
				// `PhantomData`, which a lazy key's function takes, is nothing, as `()` is.
				let marker = (op % 2 == 0).then_some(PhantomData::<u8>);
				out.push(marker.map_or(0xfd, |PhantomData| 1));
			}
			// The old value each helper replaces drops, or is put.
			7 => {
				LOG.with_borrow_mut(|log| log.push(op));
				out.push(LOG.with_borrow(|log| log.len() as u8));
				out.extend(LOG.replace(vec![op]));
				LOG.set(vec![op, op]);
				out.extend(LOG.take());
				LOG.set(vec![1, 2, 3]);
				HELD.set(Some(Noisy {
					id: op,
					log: log.clone(),
				}));
				// Drops what it held.
				HELD.with_borrow_mut(|held| *held = None);
				let old = HELD.replace(Some(Noisy {
					id: op.wrapping_add(1),
					log: log.clone(),
				}));
				out.push(old.as_ref().map_or(0xfd, |n| n.id));
				drop(old);
				out.push(HELD.take().map_or(0xfd, |n| n.id));
				// A value JS copies, written through its place.
				TALLY.with_borrow_mut(|tally| *tally = tally.wrapping_mul(3).wrapping_add(u32::from(op)));
				out.push(TALLY.with_borrow(|tally| *tally as u8));
				TALLY.set(TALLY.with_borrow(|tally| tally / 2));
			}
			_ => out.push(serial(false)),
		}
		out.extend(log.lock().unwrap().iter());
		log.lock().unwrap().clear();
	}
	COUNT.with(|c| c.set(7));
	TALLY.with(|tally| *tally.borrow_mut() = 0);
	LOG.with(|log| log.borrow_mut().truncate(3));
	POLLING.with(|p| {
		p.current.set(None);
		p.held.borrow_mut().clear();
	});
	HELD.with(|held| held.borrow_mut().take());
	serial(true);
	out.extend(log.lock().unwrap().iter());
}

/// A serial from a thread-local in a function, as kio's `serial` is; starts over if `reset`.
fn serial(reset: bool) -> u8 {
	thread_local! {
		static NEXT: Cell<(u8, u8)> = const { Cell::new((0, 4)) };
	}
	NEXT.with(|next| {
		let (n, end) = match reset {
			true => (0, 4),
			false => next.get(),
		};
		next.set((n.wrapping_add(u8::from(!reset)), end));
		n
	})
}

/// `*r = v`, through the reference.
fn assign<T>(r: &mut T, v: T) {
	*r = v;
}

/// Each byte a copy of `LEFT` written whole with `RIGHT` by each way std moves a value, or
/// not at all, after which adding the byte through the copy changes only the key it names.
/// Each puts what the replaced copy names, then what each key holds. A key is used only
/// through a `&'static`, so each copy is leaked.
pub fn keys(data: &[u8], out: &mut Vec<u8>) {
	for &op in data {
		let key: &'static mut LocalKey<Cell<u8>> = Box::leak(Box::new(LEFT));
		match op % 5 {
			0 => assign(key, RIGHT),
			1 => out.push(Box::leak(Box::new(mem::replace(key, RIGHT))).get()),
			2 => {
				let other = Box::leak(Box::new(RIGHT));
				mem::swap(key, other);
				out.push(other.get());
			}
			3 => Cell::from_mut(&mut *key).set(RIGHT),
			_ => {}
		}
		key.set(key.get().wrapping_add(op));
		out.extend([LEFT.get(), RIGHT.get()]);
	}
	LEFT.set(1);
	RIGHT.set(2);
}
