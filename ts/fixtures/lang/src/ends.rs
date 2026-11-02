//! The crate's own iterators that read from the back, know their length, and say they stay
//! ended, with their own size hints: std's adaptors and consumers call each method where
//! native's do, and `fuse` reads on past the end of one that says it stays ended.

use std::cell::RefCell;
use std::iter::FusedIterator;

/// The items `front..back`, read from either end, logging each call of its methods: `n`,
/// `b`, `h`, and `l`, each with how many items it has left.
struct Run<'a> {
	front: u8,
	back: u8,
	/// What `size_hint` adds to its upper bound, or `None` for none: past 0, it lies, and
	/// std's `len` of it panics.
	over: Option<u8>,
	log: &'a RefCell<Vec<u8>>,
}

impl Run<'_> {
	fn left(&self) -> u8 {
		self.back - self.front
	}

	fn logged(&self, what: u8) {
		self.log.borrow_mut().extend([what, self.left()]);
	}
}

impl Iterator for Run<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.logged(b'n');
		if self.front == self.back {
			return None;
		}
		self.front += 1;
		Some(self.front - 1)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.logged(b'h');
		let left = usize::from(self.left());
		(left, self.over.map(|over| left + usize::from(over)))
	}
}

impl DoubleEndedIterator for Run<'_> {
	fn next_back(&mut self) -> Option<u8> {
		self.logged(b'b');
		if self.front == self.back {
			return None;
		}
		self.back -= 1;
		Some(self.back)
	}
}

/// std's `len`, of its `size_hint`.
impl ExactSizeIterator for Run<'_> {}

/// A `Run` whose `len` is its own, which reads the run's, as hashlink's `Values` does.
struct Counted<'a>(Run<'a>);

impl Iterator for Counted<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}
}

impl DoubleEndedIterator for Counted<'_> {
	fn next_back(&mut self) -> Option<u8> {
		self.0.next_back()
	}
}

impl ExactSizeIterator for Counted<'_> {
	fn len(&self) -> usize {
		self.0.logged(b'l');
		self.0.len()
	}
}

/// Yields every other call to `next`, from either end, ending between: a `FusedIterator`
/// that isn't, which std's `fuse` reads on from as it would one that is.
struct Flicker(u8);

impl Iterator for Flicker {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.wrapping_add(1);
		(self.0 % 2 == 0).then_some(self.0)
	}
}

impl DoubleEndedIterator for Flicker {
	fn next_back(&mut self) -> Option<u8> {
		self.0 = self.0.wrapping_add(3);
		(self.0 % 2 == 0).then_some(self.0)
	}
}

impl FusedIterator for Flicker {}

/// As `Flicker`, without its `FusedIterator`, so std's `fuse` ends it at its first `None`.
struct Lapse(Flicker);

impl Iterator for Lapse {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0.next()
	}
}

/// An item, or 0xff for `None`.
fn put(out: &mut Vec<u8>, item: Option<u8>) {
	out.push(item.unwrap_or(0xff));
}

/// A hint, the upper bound 0xff for `None`.
fn put_hint(out: &mut Vec<u8>, (lower, upper): (usize, Option<usize>)) {
	out.push(lower as u8);
	out.push(upper.map_or(0xff, |upper| upper as u8));
}

/// A `Vec` collected or extended by `items`, by its length and capacity: what std reserved
/// by their hints.
fn put_vec(out: &mut Vec<u8>, v: Vec<u8>) {
	out.extend([v.len() as u8, v.capacity() as u8]);
	out.extend(v);
}

/// The run `data`'s first byte picks steps through with each of its other bytes, read
/// directly, through `&mut`, or by std's adaptors, and consumed by the third byte's pick,
/// with what each read and each call of its methods logged.
pub fn ends(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let [shape, consume, steps @ ..] = data else {
		return;
	};
	let log = RefCell::new(Vec::new());
	let over = match shape >> 4 & 3 {
		0 => None,
		over => Some(over - 1),
	};
	let mut run = Run {
		front: 0,
		back: shape % 16,
		over,
		log: &log,
	};
	for &step in steps.iter().take(8) {
		match step % 8 {
			0 => put(out, run.next()),
			1 => put(out, run.next_back()),
			2 => put(out, run.nth_back(usize::from(step >> 3) % 4)),
			3 => out.push(run.len() as u8),
			4 => put_hint(out, run.size_hint()),
			5 => put(out, (&mut run).rev().next()),
			6 => out.push((&mut run).len() as u8),
			_ => put(out, run.nth(usize::from(step >> 3) % 4)),
		}
	}
	let k = usize::from(consume >> 4);
	match kind % 4 {
		0 => consumed(run, consume % 16, k, out),
		1 => consumed(Counted(run), consume % 16, k, out),
		2 => fuses(*consume, out),
		_ => {
			// Mixed from either end, by the bits of `consume`, until they meet.
			let mut bits = *consume;
			loop {
				let item = match bits & 1 {
					0 => run.next(),
					_ => run.next_back(),
				};
				bits = bits.rotate_right(1);
				put(out, item);
				if item.is_none() {
					break;
				}
			}
			put(out, run.next_back());
		}
	}
	out.push(0xfe);
	out.extend(log.into_inner());
}

/// `it`, consumed by std's adaptor or consumer `pick`, with `k` its count where it takes one.
fn consumed<I: DoubleEndedIterator<Item = u8> + ExactSizeIterator>(mut it: I, pick: u8, k: usize, out: &mut Vec<u8>) {
	match pick {
		0 => put_vec(out, it.rev().collect()),
		1 => put_vec(out, it.collect()),
		2 => out.push(it.rev().len() as u8),
		3 => out.extend(it.enumerate().rev().flat_map(|(i, b)| [i as u8, b])),
		4 => out.extend(it.skip(k).rev()),
		5 => out.extend(it.take(k).rev()),
		6 => out.extend(it.step_by(k.max(1)).rev()),
		7 => put(out, it.rposition(|b| b % 3 == 0).map(|i| i as u8)),
		8 => put(out, it.skip(k % 4).rfind(|b| b % 3 == 0)),
		9 => put(out, it.rev().last()),
		10 => out.extend(it.map(|b| b ^ 0x55).rev().take(k)),
		11 => {
			let mut v = vec![0xaa];
			v.extend(it.by_ref().take(k));
			put_vec(out, v);
			out.push(it.len() as u8);
		}
		12 => {
			let mut zip = it.zip(0u8..5);
			while let Some((a, b)) = zip.next_back() {
				out.extend([a, b]);
			}
		}
		13 => {
			let mut fused = it.fuse();
			put(out, fused.next_back());
			out.push(fused.len() as u8);
			out.extend(fused);
		}
		14 => put(out, it.chain([7, 8]).rev().nth(k)),
		_ => {
			let mut peek = it.peekable();
			put(out, peek.peek().copied());
			out.push(peek.len() as u8);
			out.extend(peek.rev());
		}
	}
}

/// `fuse` of a `Flicker`, which reads on past its first `None`, or of a `Lapse`, which
/// doesn't, read by the bits of `bits` from either end.
fn fuses(bits: u8, out: &mut Vec<u8>) {
	let start = bits >> 4;
	let mut fused = Flicker(start).fuse();
	for k in 0..6 {
		match bits >> (k % 4) & 1 {
			0 => put(out, fused.next()),
			_ => put(out, fused.next_back()),
		}
	}
	let mut lapsed = Lapse(Flicker(start)).fuse();
	for _ in 0..4 {
		put(out, lapsed.next());
	}
	let flat: Vec<u8> = [Flicker(start), Flicker(start.wrapping_add(1))]
		.into_iter()
		.flatten()
		.take(5)
		.collect();
	out.extend(flat);
}
