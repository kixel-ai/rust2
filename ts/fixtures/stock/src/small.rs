//! smallvec's `SmallVec`, which translation maps to the runtime's `rs.Vec`: its length,
//! capacity, and whether it spilled after each step, and the order its items drop in.

use std::cell::RefCell;
use std::sync::{Arc, Weak};

use smallvec::SmallVec;

/// Logs its value as it drops, and its clone logs the value past 0x80.
struct Noted<'a>(u8, &'a RefCell<Vec<u8>>);

impl Drop for Noted<'_> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

impl Clone for Noted<'_> {
	fn clone(&self) -> Self {
		self.1.borrow_mut().push(0x80 | self.0);
		Noted(self.0, self.1)
	}
}

/// A length, capacity, or value, as four bytes.
fn put(out: &mut Vec<u8>, n: usize) {
	out.extend_from_slice(&(n as u32).to_le_bytes());
}

/// What a step shows of `v`: its length, capacity, and whether it spilled, its values, and
/// what dropped since.
fn show(out: &mut Vec<u8>, v: &SmallVec<[Noted<'_>; 4]>, log: &RefCell<Vec<u8>>) {
	put(out, v.len());
	put(out, v.capacity());
	out.push(u8::from(v.spilled()));
	out.extend(v.iter().map(|l| l.0));
	out.push(0xff);
	out.append(&mut log.borrow_mut());
	out.push(0xfe);
}

/// Each pair of bytes a step of a `SmallVec` of four inline, by its first, and its argument,
/// the second, from what the first byte picks: `new`, `with_capacity`, `from_vec`, or
/// `from_elem`. A step that panics ends it, as smallvec's does.
pub fn ops(data: &[u8], out: &mut Vec<u8>) {
	let Some((&first, steps)) = data.split_first() else {
		return;
	};
	let log = RefCell::new(Vec::new());
	let made = usize::from(first >> 2) % 12;
	let mut v: SmallVec<[Noted<'_>; 4]> = match first & 3 {
		0 => SmallVec::new(),
		1 => SmallVec::with_capacity(made),
		2 => SmallVec::from_vec((0..made as u8).map(|i| Noted(i, &log)).collect()),
		_ => SmallVec::from_elem(Noted(first, &log), made),
	};
	show(out, &v, &log);
	let mut next = 0x40;
	for step in steps.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		let len = v.len();
		let at = usize::from(arg) % (len + 1);
		next += 1;
		// What a `try_` reservation asks for: past what it may hold at 0xff.
		let ask = if arg == 0xff { usize::MAX } else { usize::from(arg % 20) };
		match op % 27 {
			0..=2 => v.push(Noted(next, &log)),
			3 => put(out, v.pop().map_or(0x100, |l| usize::from(l.0))),
			4 => v.insert(at, Noted(next, &log)),
			5 if len > 0 => put(out, usize::from(v.remove(at % len).0)),
			6 if len > 0 => put(out, usize::from(v.swap_remove(at % len).0)),
			7 => v.truncate(at),
			8 => v.clear(),
			9 => v.reserve(usize::from(arg % 20)),
			10 => v.reserve_exact(usize::from(arg % 20)),
			11 => v.shrink_to_fit(),
			12 => v.grow(usize::from(arg % 20).max(len)),
			13 => v.retain(|l| l.0 % 3 != arg % 3),
			14 => {
				// The range, of which the drain yields one before it drops.
				let end = at + usize::from(arg >> 4) % (len - at + 1);
				let mut d = v.drain(at..end);
				put(out, d.next().map_or(0x100, |l| usize::from(l.0)));
				put(out, d.len());
			}
			15 => v.extend((0..arg % 6).map(|i| Noted(next + i, &log))),
			16 => {
				let c = v.clone();
				show(out, &c, &log);
			}
			17 => {
				let w = std::mem::take(&mut v).into_vec();
				put(out, w.len());
				put(out, w.capacity());
				v = SmallVec::from(w);
			}
			18 => v.resize(usize::from(arg % 10), Noted(next, &log)),
			19 => {
				let mut f = 0;
				v.resize_with(usize::from(arg % 10), || {
					f += 1;
					Noted(next + f, &log)
				});
			}
			20 => {
				let mut other: SmallVec<[Noted<'_>; 2]> = (0..arg % 4).map(|i| Noted(next + i, &log)).collect();
				v.append(&mut other);
				put(out, other.len());
				put(out, other.capacity());
			}
			21 => v.retain_mut(|l| {
				l.0 = l.0.wrapping_add(1);
				l.0 % 2 == 0
			}),
			22 => {
				let mut rest = std::mem::take(&mut v).into_iter();
				put(out, rest.next().map_or(0x100, |l| usize::from(l.0)));
				put(out, rest.next_back().map_or(0x100, |l| usize::from(l.0)));
			}
			// Past the length or the capacity, which panics.
			23 => match arg % 6 {
				0 => v.insert(len + 1, Noted(next, &log)),
				1 => drop(v.remove(len)),
				2 => drop(v.swap_remove(len)),
				3 => drop(v.drain(at..len + 1)),
				4 => drop(v.drain(len..at)),
				_ => v.reserve(usize::MAX),
			},
			24 => out.push(u8::from(v.try_grow(ask.max(len)).is_ok())),
			25 | 26 => {
				let tried = if op % 27 == 25 {
					v.try_reserve(ask)
				} else {
					v.try_reserve_exact(ask)
				};
				match tried {
					Ok(()) => out.push(1),
					Err(e) => out.extend(format!("{e}").bytes()),
				}
			}
			_ => {}
		}
		show(out, &v, &log);
	}
	drop(v);
	out.append(&mut log.borrow_mut());
}

/// A `SmallVec` of three `u16`s inline, each `Copy`: made from a slice, a buffer, or an
/// element, extended and inserted into from slices, and given back as its array or not.
pub fn copies(data: &[u8], out: &mut Vec<u8>) {
	let Some((&first, rest)) = data.split_first() else {
		return;
	};
	let items: Vec<u16> = rest.iter().map(|&b| u16::from(b) * 3).collect();
	let n = items.len();
	let mut v: SmallVec<[u16; 3]> = match first % 6 {
		0 => SmallVec::from_slice(&items),
		1 => SmallVec::from_buf([1, 2, 3]),
		2 => SmallVec::from_buf_and_len([4, 5, 6], n % 4),
		3 => SmallVec::from_elem(7, n),
		4 => items.iter().copied().collect(),
		_ => SmallVec::from(&items[..]),
	};
	put(out, v.len());
	put(out, v.capacity());
	out.push(u8::from(v.spilled()));
	match (first >> 3) % 4 {
		0 => v.extend_from_slice(&items[..n / 2]),
		1 => v.insert_from_slice(v.len() / 2, &items[n / 2..]),
		2 => v.shrink_to_fit(),
		_ => v.truncate(1),
	}
	put(out, v.len());
	put(out, v.capacity());
	out.extend(format!("{v:?}").bytes());
	out.push(u8::from(v == SmallVec::<[u16; 3]>::from_slice(&items)));
	put(out, v.iter().map(|&x| usize::from(x)).sum());
	if let Some(&x) = v.as_slice().first() {
		put(out, usize::from(x));
	}
	match v.into_inner() {
		Ok(a) => out.extend(a.iter().flat_map(|x| x.to_le_bytes())),
		Err(v) => {
			put(out, v.len());
			let b = v.into_boxed_slice();
			put(out, b.len());
		}
	}
}

/// A `SmallVec` of zero-sized items, which never spills, however many it holds.
pub fn zero(data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().map_or(0, |&b| usize::from(b));
	let mut v: SmallVec<[(); 2]> = SmallVec::with_capacity(n);
	put(out, v.inline_size());
	put(out, v.capacity());
	out.push(u8::from(v.spilled()));
	for _ in 0..n {
		v.push(());
	}
	v.reserve(5);
	put(out, v.len());
	put(out, v.capacity());
	out.push(u8::from(v.spilled()));
	let w = v.into_vec();
	put(out, w.len());
	let back: SmallVec<[(); 2]> = SmallVec::from_vec(w);
	out.push(u8::from(back.spilled()));
}

/// What kio's waiter list does with its `SmallVec` of `Weak`s: registers, prunes those
/// whose owner dropped once it is full, and wakes the rest by draining it.
pub fn waiters(data: &[u8], out: &mut Vec<u8>) {
	let mut held: Vec<Arc<u8>> = Vec::new();
	let mut entries: SmallVec<[Weak<u8>; 4]> = SmallVec::new();
	for &b in data {
		match b % 4 {
			0 | 1 => {
				let a = Arc::new(b);
				if entries.len() == entries.capacity() {
					entries.retain(|e| e.strong_count() > 0);
					entries.reserve(entries.len());
				}
				entries.push(Arc::downgrade(&a));
				held.push(a);
			}
			2 if !held.is_empty() => drop(held.remove(usize::from(b) % held.len())),
			3 => {
				let woken: Vec<u8> = std::mem::take(&mut entries)
					.drain(..)
					.filter_map(|w| w.upgrade())
					.map(|a| *a)
					.collect();
				out.extend(woken);
				out.push(0xff);
			}
			_ => {}
		}
		if !entries.is_empty() {
			put(out, entries[0].strong_count());
		}
		put(out, entries.len());
		put(out, entries.capacity());
		out.push(u8::from(entries.iter().all(|e| e.strong_count() == 0)));
	}
	entries.clear();
	put(out, entries.capacity());
}
