//! `Iterator::cycle`: its iterator read on, and then clones of it as it was cycled, each
//! made as the last ends, read by `next`, `nth`, and folds that stop early, with what
//! each clone owns cloned and dropped as std's are.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Logs its id as it drops, and as it is cloned, the id with the high bit set: its clone's
/// id is one more.
struct Logged {
	id: u8,
	log: Rc<RefCell<Vec<u8>>>,
}

impl Logged {
	fn id(&self) -> u8 {
		self.id
	}

	/// Logs `x`, as a closure that holds it is called.
	fn called(&self, x: u8) -> u8 {
		self.log.borrow_mut().push(0x40 | x & 0x3f);
		x ^ self.id
	}
}

impl Clone for Logged {
	fn clone(&self) -> Self {
		self.log.borrow_mut().push(0x80 | self.id);
		Self {
			id: self.id.wrapping_add(1),
			log: self.log.clone(),
		}
	}
}

impl Drop for Logged {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.id);
	}
}

/// Puts what was logged since the last flush, after a marker.
fn flush(log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	out.push(0xfd);
	out.extend(log.borrow_mut().drain(..));
}

/// The crate's own iterator, which `cycle` clones by its `Clone`.
#[derive(Clone)]
struct Countdown(u8);

impl Iterator for Countdown {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

/// `it` cycled, its size hint, and then read as each of `steps` picks: by `next`, `nth`,
/// or `find` or `position` of the item a count picks, each item as `put` makes it, or
/// 0xee for none.
fn read<I: Iterator + Clone>(it: I, steps: &[u8], put: impl Fn(I::Item) -> u8, out: &mut Vec<u8>) {
	let mut it = it.cycle();
	let (lower, upper) = it.size_hint();
	out.extend([lower as u8, (lower >> 24) as u8, upper.map_or(0xfe, |n| n as u8)]);
	for &step in steps.iter().take(12) {
		let n = usize::from(step >> 2) % 9;
		let mut left = n;
		let mut nth = |_: &I::Item| {
			let hit = left == 0;
			left = left.wrapping_sub(1);
			hit
		};
		let item = match step & 3 {
			0 => it.next(),
			1 => it.nth(n),
			2 => it.find(|item| nth(item)),
			_ => {
				out.push(it.position(|item| nth(&item)).map_or(0xee, |i| i as u8));
				continue;
			}
		};
		out.push(item.map_or(0xee, &put));
	}
}

/// `cycle` of std's iterators and adaptors and of the crate's own, each partly read,
/// empty, or both, the first byte of `data` a count and the rest the steps `read` takes.
pub fn cycles(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = usize::from(data.first().map_or(0, |&b| b % 5));
	let steps = data.get(1..).unwrap_or(&[]);
	let id = |x: &u8| *x;
	match kind % 6 {
		// Partly read before it is cycled, read through, and empty.
		0 => {
			let mut it = data.iter();
			it.nth(n);
			read(it, steps, id, out);
			let mut it = data.iter();
			it.by_ref().for_each(drop);
			read(it, steps, id, out);
			read(data[..0].iter(), steps, id, out);
			read(data.iter().take(n), steps, id, out);
			read(Some(n as u8).into_iter(), steps, |x| x, out);
			read(std::iter::empty::<u8>(), steps, |x| x, out);
			read(Countdown(n as u8 * 2), steps, |x| x, out);
		}
		// Ranges, an inclusive one folded its own way, and adaptors, some empty only once read.
		1 => {
			let lo = data.first().copied().unwrap_or(0) % 8;
			let hi = data.get(1).copied().unwrap_or(9) % 12;
			read(lo..hi, steps, |x| x, out);
			let mut range = lo..=hi;
			range.next();
			read(range.clone(), steps, |x| x, out);
			range.by_ref().for_each(drop);
			read(range, steps, |x| x, out);
			read(data.iter().filter(|&&x| x % 3 != 0), steps, id, out);
			read(data.iter().chain(data.iter().rev().take(2)), steps, id, out);
			read(data.iter().step_by(n + 1), steps, id, out);
			read(data.iter().skip(n), steps, id, out);
			read(data.iter().enumerate(), steps, |(i, x)| i as u8 ^ x, out);
			let mut peeked = data.iter().peekable();
			peeked.peek();
			read(peeked, steps, id, out);
			read(data.chunks(n + 1).flatten(), steps, id, out);
		}
		// Taken across several wraps, and consumed by value.
		2 => {
			let m = usize::from(data.last().copied().unwrap_or(0) % 40);
			out.extend(data.iter().cycle().take(m));
			out.push(data.iter().cycle().take(m).count() as u8);
			out.push(data.iter().cycle().take(m).fold(0u8, |a, &x| a.wrapping_add(x)));
			out.extend(data.iter().zip(b"key".iter().cycle()).map(|(a, b)| a ^ b));
			out.extend((1..=3u8).cycle().skip(n).step_by(2).take(m));
			let v: Vec<u8> = data.iter().copied().cycle().take(m).collect();
			out.extend(v);
			out.push(data.iter().cycle().take(m).any(|&x| x == 7) as u8);
			#[allow(clippy::infinite_iter, reason = "the cycle of nothing ends")]
			{
				out.push(data[..0].iter().cycle().count() as u8);
				out.extend(data[..0].iter().cycle().last());
			}
			out.extend(data[..0].iter().cycle().take(m));
			// A source that pauses, whose clones share where it is, so a later clone can be
			// empty: `next` returns `None` there and reads on after, `count` of a `take`
			// stops there, and a fold reads past it.
			let at = Rc::new(Cell::new(0u8));
			let pausing = std::iter::from_fn(move || {
				let a = at.get();
				at.set(a.wrapping_add(1));
				[true, false, true, false, false][usize::from(a % 5)].then_some(a)
			});
			out.push(pausing.clone().cycle().take(m).count() as u8);
			let mut it = pausing.clone().cycle();
			for _ in 0..m % 9 {
				out.push(it.next().map_or(0xee, |x| x));
			}
			out.push(it.by_ref().take(m).count() as u8);
			out.push(pausing.cycle().take(m).fold(0u8, |a, x| a.wrapping_add(x)));
		}
		3 => closures(data, steps, out),
		4 => owned(data, n, steps, out),
		// Clones of a cycle, read in turns with it.
		_ => {
			let mut it = data.iter().cycle();
			it.nth(n);
			let mut clone = it.clone();
			for &step in steps.iter().take(16) {
				let item = match step & 3 {
					0 => it.next(),
					1 => clone.next(),
					2 => it.nth(usize::from(step >> 2) % 9),
					_ => clone.nth(usize::from(step >> 2) % 9),
				};
				out.push(item.map_or(0xee, id));
			}
			let clone = data[..0].iter().cycle().clone();
			out.push(clone.size_hint().0 as u8);
			// A cycle of a cycle, which clones it only if it is empty.
			read(it.clone(), steps, id, out);
			read(data[..0].iter().cycle(), steps, id, out);
		}
	}
}

/// Adaptors whose closures log as they are called and hold a value that logs as it is
/// cloned and dropped: when each clone of them is made, and when each drops.
fn closures(data: &[u8], steps: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };

	let tag = logged(0x10);
	let mut it = data.iter().take(3).map(move |&x| tag.called(x));
	it.next();
	let mut it = it.cycle();
	flush(&log, out);
	for _ in 0..7 {
		out.extend(it.next());
	}
	flush(&log, out);
	let mut left = data.len() % 5;
	out.extend(it.find(|_| {
		left = left.wrapping_sub(1);
		left == usize::MAX
	}));
	flush(&log, out);
	let mut left = data.len() % 7;
	out.extend(
		it.position(|_| {
			left = left.wrapping_sub(1);
			left == usize::MAX
		})
		.map(|i| i as u8),
	);
	flush(&log, out);
	out.extend(it.nth(5));
	flush(&log, out);
	drop(it);
	flush(&log, out);

	// A closure that counts its calls, which each clone counts on from where it was cycled,
	// and a filter of what it returns.
	let (tag, mut count) = (logged(0x20), 0u8);
	let it = data.iter().take(4).map(move |&x| {
		count = count.wrapping_add(1);
		tag.called(x) ^ count
	});
	let gate = logged(0x30);
	let it = it.filter(move |&x| gate.called(x) & 3 != 0);
	read(it, steps, |x| x, out);
	flush(&log, out);

	// A source whose clones share what they read, which only the iterator cycled yields:
	// each clone after is empty, which ends both `next` and a fold.
	let left = Rc::new(Cell::new(data.len() % 4));
	let tag = logged(0x38);
	let it = std::iter::from_fn(move || {
		let l = left.get().checked_sub(1)?;
		left.set(l);
		Some(tag.called(l as u8))
	});
	let mut it = it.cycle();
	flush(&log, out);
	out.extend(it.next());
	out.push(it.position(|_| false).is_none() as u8);
	flush(&log, out);
	out.push(it.next().is_none() as u8);
	out.push(it.nth(2).is_none() as u8);
	flush(&log, out);
	drop(it);
	flush(&log, out);
}

/// Iterators that own their items, each cloned as the iterator is at each wrap, and
/// dropped as `nth` skips them and as the cycle drops what it holds and its clone.
fn owned(data: &[u8], n: usize, steps: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };

	let items: Vec<Logged> = data.iter().take(4).map(|&b| logged(b & 0x70)).collect();
	let mut it = items.into_iter();
	it.next();
	flush(&log, out);
	let mut it = it.cycle();
	flush(&log, out);
	out.extend(it.nth(n).map(|x| x.id()));
	flush(&log, out);
	out.extend(it.nth(n + 3).map(|x| x.id()));
	flush(&log, out);
	// Past as many as leaves it mid-way through a clone, for most counts.
	let mut left = n * 2;
	out.extend(
		it.find(|_| {
			left = left.wrapping_sub(1);
			left == usize::MAX
		})
		.map(|x| x.id()),
	);
	flush(&log, out);
	let clone = it.clone();
	flush(&log, out);
	drop(it);
	flush(&log, out);
	read(clone, steps, |x| x.id(), out);
	flush(&log, out);

	let empty: Vec<Logged> = Vec::new();
	let mut it = empty.into_iter().cycle();
	out.push(it.nth(3).is_none() as u8);
	out.push(it.find(|_| true).is_none() as u8);
	let (lower, upper) = it.size_hint();
	out.extend([lower as u8, upper.map_or(0xfe, |n| n as u8)]);
}
