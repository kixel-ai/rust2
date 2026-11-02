//! `VecDeque`'s ring buffer, as std places its items: each step that moves them, then
//! where `as_slices` splits them, and `{:?}` of its `Iter`, `IterMut`, and `Drain` read in
//! part, from either end, by `nth`, and by `try_fold`. Run for items of each size std's
//! growth tells apart, bytes, which its slices view, zero-sized ones, and ones that drop.

use std::collections::VecDeque;
use std::fmt::Debug;

/// An item a byte makes, which a write through a `&mut` changes.
trait Item: Clone + Debug {
	/// Whether it is a byte, which a deque holds in a `Uint8Array`.
	const BYTE: bool = false;

	fn make(b: u8) -> Self;

	fn bump(&mut self);
}

impl Item for u8 {
	const BYTE: bool = true;

	fn make(b: u8) -> Self {
		b
	}

	fn bump(&mut self) {
		*self = self.wrapping_add(1);
	}
}

impl Item for u32 {
	fn make(b: u8) -> Self {
		u32::from(b) << 12
	}

	fn bump(&mut self) {
		*self += 1;
	}
}

impl Item for String {
	fn make(b: u8) -> Self {
		format!("s{}", b % 16)
	}

	fn bump(&mut self) {
		self.push('!');
	}
}

impl Item for () {
	fn make(_: u8) -> Self {}

	fn bump(&mut self) {}
}

/// Past 1 KiB, which std's grows from one slot rather than four.
#[derive(Clone)]
struct Big {
	id: u8,
	pad: [u8; 1100],
}

impl Debug for Big {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "B{}.{}", self.id, self.pad[1099])
	}
}

impl Item for Big {
	fn make(b: u8) -> Self {
		Big { id: b, pad: [b; 1100] }
	}

	fn bump(&mut self) {
		self.id = self.id.wrapping_add(1);
	}
}

/// Steps a deque of the item type `kind` picks through `data`.
pub fn slices(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 5 {
		0 => steps::<u8>(data, out),
		1 => steps::<u32>(data, out),
		2 => steps::<String>(data, out),
		3 => steps::<()>(data, out),
		_ => steps::<Big>(data, out),
	}
}

/// A length or capacity, as four bytes.
fn put(out: &mut Vec<u8>, n: usize) {
	out.extend_from_slice(&(n as u32).to_le_bytes());
}

/// What `{:?}` writes of `value`.
fn text(out: &mut Vec<u8>, value: &impl Debug) {
	out.extend_from_slice(format!("{value:?}").as_bytes());
}

/// A deque the first byte makes, new, with room for a few, or from a `Vec` of a few, and
/// each pair after it steps: after each, its length, capacity, and two slices.
fn steps<T: Item>(data: &[u8], out: &mut Vec<u8>) {
	let (first, rest) = data.split_first().map_or((0, &[][..]), |(&b, rest)| (b, rest));
	let n = usize::from(first >> 2) % 12;
	let mut q: VecDeque<T> = match first % 3 {
		0 => VecDeque::new(),
		1 => VecDeque::with_capacity(n),
		_ => VecDeque::from(vec![T::make(first); n]),
	};
	for step in rest.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		let at = usize::from(arg) % (q.len() + 1);
		match op % 24 {
			0..=3 => q.push_back(T::make(arg)),
			4 | 5 => q.push_front(T::make(arg)),
			6 => drop(q.pop_front()),
			7 => drop(q.pop_back()),
			8 => q.insert(at, T::make(arg)),
			9 => drop(q.remove(at)),
			10 => drop(q.swap_remove_front(at)),
			11 => drop(q.swap_remove_back(at)),
			12 => q.truncate(usize::from(arg) % 8),
			13 => {
				let mut kept = arg;
				q.retain(|_| {
					kept = kept.rotate_left(1);
					kept & 1 == 1
				});
			}
			14 => drained(&mut q, arg, out),
			15 => q.reserve(usize::from(arg) % 12),
			16 => q.reserve_exact(usize::from(arg) % 12),
			17 => q.shrink_to(usize::from(arg) % 12),
			18 => q.shrink_to_fit(),
			// Not `TrustedLen`, so it reserves once full, and a `TrustedLen` one.
			19 => q.extend((0..arg % 8).filter(|i| i % 3 != 0).map(T::make)),
			20 => q.extend((0..arg % 8).map(T::make)),
			21 => q.clear(),
			22 => q = remade(q, arg),
			_ => mutated(&mut q, arg, out),
		}
		put(out, q.len());
		put(out, q.capacity());
		text(out, &q.as_slices());
		iterated(&q, arg, out);
	}
}

/// A deque made anew from `q`: a clone, or one with room of its own cloned from it,
/// collected from its own `into_iter` or a `Vec`'s read in part, whose buffers std's takes
/// as they are, or from a `Vec` it collects.
fn remade<T: Item>(q: VecDeque<T>, arg: u8) -> VecDeque<T> {
	let skip = usize::from(arg / 5) % 4;
	match arg % 5 {
		0 if arg & 0x20 == 0 => q.clone(),
		0 => {
			let mut own = VecDeque::with_capacity(skip * 3);
			own.push_front(T::make(arg));
			own.clone_from(&q);
			own
		}
		1 => {
			let mut it = q.into_iter();
			for _ in 0..skip {
				drop(it.next());
			}
			drop(it.next_back());
			it.collect()
		}
		// By `advance_by`, which drains the front or clears it all.
		2 => {
			let mut it = q.into_iter();
			drop(it.nth(skip));
			it.collect()
		}
		3 => VecDeque::from(q.into_iter().collect::<Vec<T>>()),
		// A `Vec<u8>`'s iterator reads a view of its bytes, which leaves its capacity behind.
		_ if T::BYTE => q,
		_ => {
			let mut v: Vec<T> = q.into_iter().collect();
			v.reserve(skip);
			let mut it = v.into_iter();
			for _ in 0..skip {
				drop(it.next());
			}
			it.collect()
		}
	}
}

/// A drain of a range `arg` picks, written as it is read from either end and then dropped.
fn drained<T: Item>(q: &mut VecDeque<T>, arg: u8, out: &mut Vec<u8>) {
	let len = q.len();
	let start = usize::from(arg & 7) % (len + 1);
	let end = start + usize::from(arg >> 3 & 7) % (len - start + 1);
	let mut d = q.drain(start..end);
	text(out, &d);
	if arg & 0x40 != 0 {
		drop(d.next());
	}
	if arg & 0x80 != 0 {
		drop(d.next_back());
	}
	text(out, &d);
}

/// Items bumped through `as_mut_slices`, then through `iter_mut` read in part, written.
fn mutated<T: Item>(q: &mut VecDeque<T>, arg: u8, out: &mut Vec<u8>) {
	let (a, b) = q.as_mut_slices();
	if let Some(x) = a.first_mut() {
		x.bump();
	}
	if let Some(x) = b.last_mut() {
		x.bump();
	}
	let mut it = q.iter_mut();
	for _ in 0..arg % 4 {
		if let Some(x) = it.next() {
			x.bump();
		}
	}
	if let Some(x) = it.next_back() {
		x.bump();
	}
	text(out, &it);
}

/// `q`'s `Iter`, written before and after each of three reads `arg` picks.
fn iterated<T: Item>(q: &VecDeque<T>, arg: u8, out: &mut Vec<u8>) {
	let mut it = q.iter();
	text(out, &it);
	for k in 0..3 {
		let pick = arg.rotate_left(3 * k);
		let mut count = pick >> 3 & 3;
		let read = match pick % 7 {
			0 => it.next(),
			1 => it.next_back(),
			2 => it.nth(usize::from(count)),
			3 => it.nth_back(usize::from(count)),
			// By `try_fold`, which reads the first slice and then the second, swapping neither.
			4 => it.find(|_| {
				count = count.saturating_sub(1);
				count == 0
			}),
			5 => it.rfind(|_| {
				count = count.saturating_sub(1);
				count == 0
			}),
			_ => it.clone().next(),
		};
		text(out, &read);
		text(out, &it);
	}
}
