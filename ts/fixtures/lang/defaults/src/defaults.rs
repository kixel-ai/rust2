//! Values `Default` makes of std types from their parts' defaults: `()`, tuples, arrays,
//! empty ones of any element, boxes, `Arc`s, and `Weak`s, which never upgrade, as
//! `Weak::new`'s don't; called, filled in place, and passed as functions.

use std::cell::OnceCell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

/// A type of the crate's own, whose `Default` impl is no zero value.
#[derive(Debug, Clone, PartialEq)]
struct Made {
	n: u64,
	tag: String,
}

impl Default for Made {
	fn default() -> Self {
		Self {
			n: 3,
			tag: "made".into(),
		}
	}
}

/// A type without a `Default`, whose empty array still has one.
#[derive(Debug)]
struct Bare;

/// What a generic caller makes, as kio's `Producer::default` makes its `T`.
fn made<T: Default>() -> T {
	T::default()
}

/// Each byte an operation, its value the operand; each puts what it read.
pub fn defaults(data: &[u8], out: &mut Vec<u8>) {
	for &op in data {
		let s = match op % 9 {
			0 => {
				let unit: () = made();
				let pair: (Option<u64>, Made) = Default::default();
				let nested: ((), (u8, bool), [Made; 0], [Bare; 0]) = made();
				format!("{unit:?} {pair:?} {nested:?}")
			}
			1 => {
				let made: [Made; 2] = made();
				let grid: [[u64; 2]; 3] = Default::default();
				let bytes: [u8; 4] = made_bytes(op);
				format!("{made:?} {grid:?} {bytes:?}")
			}
			// Each atomic is its own, which a store to one doesn't change in another.
			2 => {
				let counts: [AtomicU64; 4] = Default::default();
				counts[usize::from(op % 4)].store(u64::from(op), Ordering::Relaxed);
				counts[0].fetch_add(1, Ordering::Relaxed);
				let loads: Vec<u64> = counts.iter().map(|c| c.load(Ordering::Relaxed)).collect();
				format!("{loads:?}")
			}
			3 => {
				let boxed: Box<Made> = Box::default();
				let shared: Arc<Made> = made();
				let again = shared.clone();
				format!("{boxed:?} {:?} {}", *shared, Arc::strong_count(&again))
			}
			4 => {
				let none: Option<(Option<u64>, Made)> = (op % 3 == 0).then(|| (Some(u64::from(op)), Made::default()));
				format!("{:?}", none.unwrap_or_default())
			}
			// A dangling `Weak`, made either way, beside one of a live `Arc`.
			5 => {
				let empty: Weak<Made> = Weak::new();
				let default: Weak<Made> = made();
				let arc = Arc::new(Made::default());
				let live = Arc::downgrade(&arc);
				let copy = empty.clone();
				format!(
					"{} {} {} {} {} {} {}",
					empty.upgrade().is_none(),
					Weak::strong_count(&default),
					Weak::ptr_eq(&empty, &default),
					Weak::ptr_eq(&empty, &copy),
					Weak::ptr_eq(&empty, &live),
					copy.upgrade().is_none(),
					live.upgrade().is_some()
				)
			}
			// A struct holding one until an `Arc` replaces it.
			6 => {
				let mut slot = Slot::default();
				let before = slot.get();
				let arc = Arc::new(Made {
					n: u64::from(op),
					..Made::default()
				});
				slot.weak = Arc::downgrade(&arc);
				format!("{before:?} {:?}", slot.get())
			}
			// Filled in place, and passed as a function, as moq-net's are.
			7 => {
				let mut boxed: Option<Box<Made>> = (op % 3 == 0).then(Box::default);
				boxed.get_or_insert_default().n += u64::from(op);
				let mut slots: Option<[Option<u8>; 3]> = None;
				slots.get_or_insert_default()[usize::from(op % 3)] = Some(op);
				let cell: OnceCell<Arc<Made>> = OnceCell::new();
				let shared = cell.get_or_init(Arc::default).clone();
				format!("{boxed:?} {slots:?} {} {}", shared.n, Arc::strong_count(&shared))
			}
			_ => {
				let made: Made = made();
				format!("{} {}", made.n + u64::from(op), made.tag)
			}
		};
		out.extend_from_slice(s.as_bytes());
		out.push(b'\n');
	}
}

/// A placeholder that upgrades once it points at something.
#[derive(Default)]
struct Slot {
	weak: Weak<Made>,
}

impl Slot {
	fn get(&self) -> Option<u64> {
		self.weak.upgrade().map(|made| made.n)
	}
}

/// `[u8; 4]`'s default, written past its first byte.
fn made_bytes(op: u8) -> [u8; 4] {
	let mut bytes: [u8; 4] = Default::default();
	bytes[usize::from(op % 4)] = op;
	bytes
}
