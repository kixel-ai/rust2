//! `ManuallyDrop`: a value nothing drops but its own `drop`, given back by `into_inner`
//! and `take`, cloned, and read and written through it as `Deref` and `DerefMut` lend it,
//! as crossbeam-epoch's `Local` holds its collector and futures-util a waker.

use std::mem::{ManuallyDrop, MaybeUninit};
use std::sync::Arc;

/// The crate's struct holding values by hand, which its `Drop` drops one of.
struct ByHand {
	shared: ManuallyDrop<Arc<u8>>,
	count: ManuallyDrop<u32>,
	pair: ManuallyDrop<(u8, u16)>,
}

impl Drop for ByHand {
	fn drop(&mut self) {
		unsafe { ManuallyDrop::drop(&mut self.shared) };
	}
}

/// A value's `ManuallyDrop`, as a generic one makes it, which only an instance lays out.
fn held<T: Clone>(value: &T) -> ManuallyDrop<T> {
	ManuallyDrop::new(value.clone())
}

/// An `Arc` held by hand each way, its strong count after each: forgotten, given back,
/// cloned through, dropped, taken, in a struct whose `Drop` drops it, and in an `Arc` that
/// drops, beside values JS copies written through it and copied while it may hold none.
pub fn arcs(data: &[u8], out: &mut Vec<u8>) {
	let shared = Arc::new(7u8);
	let mut kept = Vec::new();
	for &b in data {
		match b % 9 {
			0 => {
				let _forgotten = ManuallyDrop::new(shared.clone());
			}
			1 => drop(ManuallyDrop::into_inner(held(&shared))),
			2 => {
				let m = held(&shared);
				kept.push(ManuallyDrop::clone(&m));
				out.push(**m);
				kept.push(m);
			}
			3 => {
				let mut m = held(&shared);
				unsafe { ManuallyDrop::drop(&mut m) };
			}
			4 => {
				let mut m = held(&shared);
				let taken = unsafe { ManuallyDrop::take(&mut m) };
				out.push(*taken + Arc::strong_count(&taken) as u8);
			}
			5 => {
				let mut h = ByHand {
					shared: held(&shared),
					count: ManuallyDrop::new(u32::from(b)),
					pair: ManuallyDrop::new((b, u16::from(b) << 4)),
				};
				*h.count += 3;
				h.pair.0 ^= 0x5a;
				let pair: &(u8, u16) = &h.pair;
				out.extend((*h.count).to_le_bytes());
				out.extend([pair.0, pair.1 as u8, **h.shared]);
			}
			6 => drop(Arc::new(held(&shared))),
			7 => {
				let mut slot = MaybeUninit::new(ManuallyDrop::new((b, u16::from(b) << 3)));
				let copy = slot;
				unsafe { slot.assume_init_mut().0 ^= 0x33 };
				let (a, c) = (unsafe { copy.assume_init() }, unsafe { slot.assume_init() });
				out.extend([a.0, a.1 as u8, c.0, c.1 as u8]);
			}
			_ => {
				let mut m = ManuallyDrop::new([b, b ^ 1]);
				m[1] = m[0].wrapping_add(9);
				let copy = *m;
				m[0] = 0;
				out.extend(copy);
				out.extend(ManuallyDrop::into_inner(m));
			}
		}
		out.push(Arc::strong_count(&shared) as u8);
	}
	// What the `Vec` holds by hand it never drops.
	drop(kept);
	out.push(Arc::strong_count(&shared) as u8);
}
