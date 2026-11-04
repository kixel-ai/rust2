//! `ManuallyDrop`: a value nothing drops but its own `drop`, given back by `into_inner`
//! and `take`, cloned, and read and written through it as `Deref` and `DerefMut` lend it,
//! as crossbeam-epoch's `Local` holds its collector and futures-util a waker; and
//! `needs_drop` and `type_name`, which rustc answers for each instance.

use std::any::{type_name, type_name_of_val};
use std::cell::Cell;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit, needs_drop};
use std::rc::Rc;
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

/// Whether `T` needs dropping, as rustc says of each instance.
fn needs<T: ?Sized>() -> bool {
	needs_drop::<T>()
}

/// Whether each type needs dropping, of values JS copies or drops nothing of, such as a
/// `String`, held by hand, in an empty array, and of a closure by what it captures; what a
/// generic body that branches on it does, as std's collections do; and types' names.
pub fn needs_drops(data: &[u8], out: &mut Vec<u8>) {
	let text: String = data.iter().map(|&b| char::from(b)).collect();
	let named = move || text.len();
	let copied = move || data.len();
	let flags = [
		needs_drop::<u8>(),
		needs_drop::<u128>(),
		needs_drop::<String>(),
		needs_drop::<&String>(),
		needs_drop::<&mut Vec<u8>>(),
		needs_drop::<str>(),
		needs_drop::<[String]>(),
		needs_drop::<dyn Debug>(),
		needs_drop::<Vec<u8>>(),
		needs_drop::<Box<u8>>(),
		needs_drop::<Rc<u8>>(),
		needs_drop::<Arc<u8>>(),
		needs_drop::<Option<u8>>(),
		needs_drop::<Option<String>>(),
		needs_drop::<Result<u8, String>>(),
		needs_drop::<(u8, u16)>(),
		needs_drop::<(u8, String)>(),
		needs_drop::<[u8; 4]>(),
		needs_drop::<[String; 0]>(),
		needs_drop::<[String; 2]>(),
		needs_drop::<ManuallyDrop<String>>(),
		needs_drop::<MaybeUninit<String>>(),
		needs_drop::<PhantomData<String>>(),
		needs_drop::<Cell<String>>(),
		needs_drop::<ByHand>(),
		needs_drop::<fn(String)>(),
		needs::<String>(),
		needs::<u8>(),
		needs::<ByHand>(),
		needs::<()>(),
		needs_drop_of(&named),
		needs_drop_of(&copied),
	];
	out.extend(flags.map(u8::from));
	out.push(named() as u8 ^ copied() as u8);
	let strings: Vec<String> = data.iter().map(|b| b.to_string()).collect();
	out.extend(dropped_by_hand(strings).to_le_bytes());
	out.extend(dropped_by_hand(data.to_vec()).to_le_bytes());
	for name in [
		type_name::<ByHand>(),
		type_name::<Option<&[String]>>(),
		type_name_of_val(&named),
		type_name_of_val(data),
		name_of::<u8>(),
		name_of::<String>(),
	] {
		crate::harness::put(out, name);
	}
}

/// Whether the value's type needs dropping.
fn needs_drop_of<T>(_: &T) -> bool {
	needs_drop::<T>()
}

/// `T`'s name, as a generic body names it.
fn name_of<T: ?Sized>() -> &'static str {
	type_name::<T>()
}

/// How many items a generic body drops one by one, as std's collections do only of a type
/// that needs dropping, and forgets the rest.
fn dropped_by_hand<T>(items: Vec<T>) -> u32 {
	if !needs_drop::<T>() {
		std::mem::forget(items);
		return 0;
	}
	let mut n = 0;
	for item in items {
		drop(item);
		n += 1;
	}
	n
}
