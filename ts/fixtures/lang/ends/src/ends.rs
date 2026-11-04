//! The crate's own iterators that read from the back, know their length, and say they stay
//! ended, with their own size hints, `nth`s, folds, `count`, `last`, finds, `for_each`,
//! `reduce`, `all`, `is_sorted_by`, `collect`, comparisons, and `by_ref`: std's adaptors,
//! consumers, and collections call each method where native's do, and `fuse` reads on past
//! the end of one that says it stays ended.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList};
use std::iter::FusedIterator;
use std::rc::Rc;
use std::sync::Arc;

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

/// A `Run` with its own `nth`, `nth_back`, `fold`, `rfold`, `count`, `last`, `find`, `rfind`,
/// `find_map`, `for_each`, `reduce`, `all`, and `is_sorted_by`, each logged as `N`, `B`, `f`,
/// `r`, `c`, `L`, `F`, `R`, `M`, `E`, `D`, `A`, and `S`, which std's adaptors and consumers
/// call in place of `next` and `next_back`, its own `collect`, `eq`, `partial_cmp`, and
/// `by_ref`, logged as `C`, `Q`, `O`, and `Y`, which std's collections and comparisons call
/// of it itself, and its own `position` and `max`, logged as `P` and `X`, which only the
/// crate's own calls reach. Its `count` reads nothing, and its `last`, `for_each`, `reduce`,
/// `is_sorted_by`, `max`, `collect`, and comparisons read from the back.
struct Overrides<'a>(Run<'a>);

impl Iterator for Overrides<'_> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0.next()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.0.size_hint()
	}

	fn nth(&mut self, n: usize) -> Option<u8> {
		self.0.logged(b'N');
		for _ in 0..n {
			self.0.next()?;
		}
		self.0.next()
	}

	fn fold<B, F: FnMut(B, u8) -> B>(mut self, init: B, mut f: F) -> B {
		self.0.logged(b'f');
		let mut acc = init;
		while let Some(b) = self.0.next() {
			acc = f(acc, b);
		}
		acc
	}

	fn count(self) -> usize {
		self.0.logged(b'c');
		usize::from(self.0.left())
	}

	fn last(mut self) -> Option<u8> {
		self.0.logged(b'L');
		self.0.next_back()
	}

	fn find<P: FnMut(&u8) -> bool>(&mut self, mut p: P) -> Option<u8> {
		self.0.logged(b'F');
		while let Some(b) = self.0.next() {
			if p(&b) {
				return Some(b);
			}
		}
		None
	}

	fn find_map<B, F: FnMut(u8) -> Option<B>>(&mut self, f: F) -> Option<B> {
		self.0.logged(b'M');
		// Through std's, which only moves what it finds.
		(&mut self.0).find_map(f)
	}

	fn for_each<F: FnMut(u8)>(mut self, mut f: F) {
		self.0.logged(b'E');
		while let Some(b) = self.0.next_back() {
			f(b);
		}
	}

	fn reduce<F: FnMut(u8, u8) -> u8>(mut self, f: F) -> Option<u8> {
		self.0.logged(b'D');
		let first = self.0.next_back()?;
		// Through std's fold, which drops `f` as it ends.
		Some(std::iter::from_fn(|| self.0.next_back()).fold(first, f))
	}

	fn all<F: FnMut(u8) -> bool>(&mut self, mut f: F) -> bool {
		self.0.logged(b'A');
		while let Some(b) = self.0.next() {
			if !f(b) {
				return false;
			}
		}
		true
	}

	fn position<P: FnMut(u8) -> bool>(&mut self, mut p: P) -> Option<usize> {
		self.0.logged(b'P');
		let mut i = 0;
		while let Some(b) = self.0.next() {
			if p(b) {
				return Some(i);
			}
			i += 1;
		}
		None
	}

	fn max(mut self) -> Option<u8> {
		self.0.logged(b'X');
		let mut best = None;
		while let Some(b) = self.0.next_back() {
			if best.is_none_or(|m| b > m) {
				best = Some(b);
			}
		}
		best
	}

	fn is_sorted_by<F: FnMut(&u8, &u8) -> bool>(mut self, mut compare: F) -> bool {
		self.0.logged(b'S');
		let Some(mut last) = self.0.next_back() else {
			return true;
		};
		while let Some(b) = self.0.next_back() {
			if !compare(&b, &last) {
				return false;
			}
			last = b;
		}
		true
	}

	fn collect<B: FromIterator<u8>>(self) -> B {
		self.0.logged(b'C');
		self.rev().collect()
	}

	fn eq<I>(self, other: I) -> bool
	where
		I: IntoIterator,
		Self::Item: PartialEq<I::Item>,
	{
		self.0.logged(b'Q');
		self.rev().eq(other)
	}

	fn partial_cmp<I>(self, other: I) -> Option<Ordering>
	where
		I: IntoIterator,
		Self::Item: PartialOrd<I::Item>,
	{
		self.0.logged(b'O');
		self.rev().partial_cmp(other)
	}

	fn by_ref(&mut self) -> &mut Self {
		self.0.logged(b'Y');
		self
	}
}

impl DoubleEndedIterator for Overrides<'_> {
	fn next_back(&mut self) -> Option<u8> {
		self.0.next_back()
	}

	fn nth_back(&mut self, n: usize) -> Option<u8> {
		self.0.logged(b'B');
		for _ in 0..n {
			self.0.next_back()?;
		}
		self.0.next_back()
	}

	fn rfold<B, F: FnMut(B, u8) -> B>(mut self, init: B, mut f: F) -> B {
		self.0.logged(b'r');
		// Through std's fold, by a closure holding `f`.
		std::iter::from_fn(|| self.0.next_back()).fold(init, move |acc, b| f(acc, b))
	}

	fn rfind<P: FnMut(&u8) -> bool>(&mut self, mut p: P) -> Option<u8> {
		self.0.logged(b'R');
		// Through std's find, by a closure holding `p`.
		std::iter::from_fn(|| self.0.next_back()).find(move |b| p(b))
	}
}

impl ExactSizeIterator for Overrides<'_> {}

/// The pairs `(b / 2, b)` of the bytes `I` yields, with its own `collect`, logged as `K`,
/// which collects them from the back.
struct Pairs<'a, I>(I, &'a RefCell<Vec<u8>>);

impl<I: DoubleEndedIterator<Item = u8>> Iterator for Pairs<'_, I> {
	type Item = (u8, u8);

	fn next(&mut self) -> Option<(u8, u8)> {
		self.0.next().map(|b| (b / 2, b))
	}

	fn collect<B: FromIterator<(u8, u8)>>(self) -> B {
		self.1.borrow_mut().push(b'K');
		self.0.rev().map(|b| (b / 2, b)).collect()
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
/// by `consumed`, or by `folded` past the fifth `kind`, and boxed, or mapped and boxed, by
/// either past the tenth, or by `found` past the fourteenth or `collected` past the
/// eighteenth, by value, borrowed, or boxed, with what each read and each call of its
/// methods logged.
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
			5 if step & 8 == 0 => put(out, (&mut run).rev().next()),
			5 => put(out, (&mut run).next_back()),
			6 => out.push((&mut run).len() as u8),
			_ => put(out, run.nth(usize::from(step >> 3) % 4)),
		}
	}
	let k = usize::from(consume >> 4);
	let pick = consume % 16;
	match kind % 22 {
		0 => consumed(run, pick, k, out),
		1 => consumed(Counted(run), pick, k, out),
		4 => consumed(Overrides(run), pick, k, out),
		5 => folded(run, pick, k, &log, out),
		6 => folded(Counted(run), pick, k, &log, out),
		9 => folded(Overrides(run), pick, k, &log, out),
		// std's `Iterator` for a box forwards `last` but none of the other by-value overrides,
		// including those of what an adaptor in it holds.
		10 => consumed(Box::new(Overrides(run)), pick, k, out),
		11 => folded(Box::new(Overrides(run)), pick, k, &log, out),
		12 => consumed(Box::new(Overrides(run).map(|b| b)), pick, k, out),
		13 => folded(Box::new(Overrides(run).map(|b| b)), pick, k, &log, out),
		// std's `Iterator` for `&mut I` and for a box forward none of `find`, `rfind`,
		// `find_map`, `for_each`, `reduce`, `all`, and `is_sorted_by`.
		14 => found(Overrides(run), pick, k, &log, out),
		15 => found(&mut Overrides(run), pick, k, &log, out),
		16 => found(Box::new(Overrides(run)), pick, k, &log, out),
		17 => found(run, pick, k, &log, out),
		// Nor `collect`, `eq`, `partial_cmp`, and `by_ref`.
		18 => collected(Overrides(run), pick, k, &log, out),
		19 => collected(&mut Overrides(run), pick, k, &log, out),
		20 => collected(Box::new(Overrides(run)), pick, k, &log, out),
		21 => collected(run, pick, k, &log, out),
		2 | 7 => fuses(*consume, out),
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
		// `rfind` folds from the back, through each adaptor that reads the length as it starts.
		8 => put(
			out,
			match k % 4 {
				0 => it.skip(k / 4).rfind(|b| b % 3 == 0),
				1 => it.take(k / 4).rfind(|b| b % 3 == 0),
				2 => it
					.enumerate()
					.rfind(|&(i, b)| (i as u8 + b) % 3 == 0)
					.map(|(i, b)| i as u8 ^ b),
				_ => it.step_by(k / 4 + 1).rfind(|b| b % 3 == 0),
			},
		),
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

/// `it`, consumed by std's fold, or a consumer or adaptor that folds it or reads it by its
/// `nth`, `count`, or `last`, picked by `pick`, with `k` its count where it takes one. One
/// fold panics past `k` items, with what was logged so far.
fn folded<I: DoubleEndedIterator<Item = u8> + ExactSizeIterator>(
	mut it: I,
	pick: u8,
	k: usize,
	log: &RefCell<Vec<u8>>,
	out: &mut Vec<u8>,
) {
	let k8 = k as u8;
	match pick {
		0 => out.push(it.fold(1u8, |a, b| a.wrapping_mul(3) ^ b)),
		1 => out.push(it.rfold(1u8, |a, b| {
			assert!(k < 15 || b != 2, "rfold at {b} after {:?}", log.borrow().as_slice());
			a.wrapping_mul(3) ^ b
		})),
		// Each `try_rfold`, `try_fold`, or `try_for_each` breaks at an item, then the rest are
		// read on by `next`.
		2 if k % 4 == 0 => {
			put(out, it.try_rfold(0u8, |a, b| (b != k8).then_some(a ^ b)));
			out.extend(it);
		}
		2 if k % 4 == 2 => {
			put(out, it.try_fold(0u8, |a, b| (b != k8 / 4).then_some(a ^ b)));
			out.extend(it);
		}
		2 if k % 4 == 3 => {
			let r = it.try_for_each(|b| (b != k8 / 4).then(|| out.push(b)));
			out.push(r.is_some() as u8);
			out.extend(it);
		}
		// Of an `Option` accumulator, whose `Some(None)` goes on and whose `None` breaks.
		2 => {
			let r = it.try_rfold(None, |a: Option<u8>, b| match b % 3 {
				0 => None,
				1 => Some(None),
				_ => Some(a.or(Some(b))),
			});
			out.extend(match r {
				Some(a) => [0, a.unwrap_or(0xff)],
				None => [1, 0],
			});
			out.extend(it);
		}
		3 => {
			let r: Result<u8, u8> = match k % 2 {
				0 => it.try_rfold(0u8, |a, b| if b % 4 == k8 % 4 { Err(b) } else { Ok(a ^ b) }),
				_ => it.try_fold(0u8, |a, b| if b % 4 == k8 % 4 { Err(b) } else { Ok(a ^ b) }),
			};
			out.extend(match r {
				Ok(a) => [0, a],
				Err(b) => [1, b],
			});
			put(out, it.next());
		}
		4 if k % 2 == 1 => {
			use std::ops::ControlFlow;
			let r = it.by_ref().rev().try_for_each(|b| match b > k8 / 2 {
				true => ControlFlow::Break(b),
				false => ControlFlow::Continue(out.push(b)),
			});
			put(
				out,
				match r {
					ControlFlow::Break(b) => Some(b),
					ControlFlow::Continue(()) => None,
				},
			);
			out.push(it.len() as u8);
		}
		4 => {
			use std::ops::ControlFlow;
			let r = it.by_ref().rev().try_rfold(0u8, |a, b| match b > k8 {
				true => ControlFlow::Break(b),
				false => ControlFlow::Continue(a ^ b),
			});
			out.extend(match r {
				ControlFlow::Continue(a) => [0, a],
				ControlFlow::Break(b) => [1, b],
			});
			out.push(it.len() as u8);
		}
		5 if k % 2 == 0 => out.push(it.count() as u8),
		5 => out.push(it.map(|b| b ^ 1).count() as u8),
		6 if k % 2 == 0 => put(out, it.last()),
		6 => put(out, it.rev().last()),
		7 if k % 4 == 0 => out.push(it.map(u32::from).sum::<u32>() as u8),
		7 if k % 4 == 1 => it.for_each(|b| out.push(b)),
		// `partition`, which folds it into two `Vec`s.
		7 => {
			let (a, b): (Vec<u8>, Vec<u8>) = it.partition(|b| b % 3 == k8 % 3);
			put_vec(out, a);
			put_vec(out, b);
		}
		8 if k % 2 == 0 => put(out, it.max()),
		8 => put(out, it.min_by_key(|b| b % 3)),
		9 if k % 2 == 0 => out.push(it.chain([7, 8]).count() as u8),
		9 => put(out, it.chain([7, 8]).last()),
		10 if k % 2 == 0 => out.push(it.skip(k / 2).count() as u8),
		10 => put(out, it.skip(k / 2).last()),
		11 => match k % 4 {
			0 => out.push(it.rev().fold(1u8, |a, b| a.wrapping_mul(5) ^ b)),
			1 => put(out, it.rev().nth(k / 4)),
			2 => put(out, it.rev().last()),
			_ => out.push(it.rev().rfold(1u8, |a, b| a.wrapping_mul(5) ^ b)),
		},
		12 => {
			let mut skip = it.skip(k / 2);
			put(out, skip.next());
			put(out, skip.next_back());
		}
		13 if k % 2 == 0 => put_vec(out, it.step_by(k / 4 + 1).collect()),
		// `scan`, whose `None` ends the collect, and which `next` reads on past.
		13 => {
			let mut scan = it.scan(0u8, |s, b| {
				*s ^= b;
				(*s != k8 / 2).then_some(*s)
			});
			put_vec(out, scan.by_ref().collect());
			put(out, scan.next());
			out.push(scan.fold(0, |a, b| a ^ b));
		}
		14 if k % 2 == 0 => put(out, it.fuse().last()),
		14 => {
			let mut peek = it.peekable();
			put(out, peek.peek().copied());
			out.push(peek.count() as u8);
		}
		// Through `&mut`, which reads by `try_fold` rather than its by-value methods.
		_ => {
			out.push((&mut it).take(k / 2).fold(0u8, |a, _| a + 1));
			put(out, (&mut it).last());
			out.push(it.len() as u8);
		}
	}
}

/// Logs `G` as it drops.
struct Guard<'a>(&'a RefCell<Vec<u8>>);

impl Drop for Guard<'_> {
	fn drop(&mut self) {
		self.0.borrow_mut().push(b'G');
	}
}

/// `it`, read by std's adaptors and consumers that call its `find`, `rfind`, `find_map`,
/// `for_each`, `reduce`, `all`, or `is_sorted_by`, or that read it by `try_fold` beside
/// them, picked by `pick`, with `k` what each tests against.
fn found<I: DoubleEndedIterator<Item = u8> + ExactSizeIterator>(
	mut it: I,
	pick: u8,
	k: usize,
	log: &RefCell<Vec<u8>>,
	out: &mut Vec<u8>,
) {
	let k8 = k as u8;
	match pick {
		0 => out.extend(it.filter(|b| b % 3 != k8 % 3)),
		1 => out.extend(it.filter(|b| b % 2 == k8 % 2).rev()),
		2 => out.extend(it.filter_map(|b| (b % 3 != k8 % 3).then_some(b ^ 0x0f))),
		3 if k % 2 == 0 => out.extend(it.filter_map(|b| (b % 3 != 1).then_some(b)).rev()),
		3 => out.extend(it.skip_while(|&b| b < k8 / 2)),
		4 if k % 2 == 0 => out.extend(it.chain([7, 8]).filter(|b| b % 2 == 1)),
		4 => out.extend(it.chain([7, 8]).filter(|b| b % 2 == 1).rev()),
		5 if k % 2 == 0 => out.extend(it.fuse().filter(|b| b % 4 != k8 % 4)),
		5 => {
			let mut fused = it.fuse().filter(|b| b % 4 == k8 % 4);
			put(out, fused.next_back());
			put(out, fused.next_back());
			out.extend(fused);
		}
		6 if k % 2 == 0 => {
			let mut rev = it.rev();
			put(out, rev.find(|&b| b < k8));
			put(out, rev.rfind(|&b| b < k8));
			out.extend(rev);
		}
		6 => {
			put(out, it.find(|&b| b > k8 / 2));
			put(out, it.rfind(|&b| b > k8 / 2));
			out.extend(it);
		}
		7 if k % 2 == 0 => {
			put(out, it.find_map(|b| (b > k8 / 2).then_some(b ^ 1)));
			out.extend(it);
		}
		// Of a found value whose `None` is `rs.None`.
		7 => {
			let found = it.find_map(|b| (b >= k8 / 2).then_some((b % 2 == 0).then_some(b)));
			out.extend(match found {
				Some(Some(b)) => [0, b],
				Some(None) => [1, 0],
				None => [2, 0],
			});
			out.extend(it);
		}
		8 if k % 2 == 0 => out.extend(BTreeSet::from_iter(it)),
		8 => {
			let mut list = LinkedList::from([0xaa]);
			list.extend(it);
			out.extend(list);
		}
		9 => put(
			out,
			match k % 4 {
				0 => it.max(),
				1 => it.min(),
				2 => it.max_by(|a, b| (a % 3).cmp(&(b % 3))),
				_ => {
					let guard = Guard(log);
					it.min_by(move |a, b| {
						let _ = &guard;
						(a % 3).cmp(&(b % 3))
					})
				}
			},
		),
		10 if k % 2 == 0 => out.push(it.is_sorted() as u8),
		10 => {
			let guard = Guard(log);
			out.push(it.is_sorted_by(move |a, b| {
				let _ = &guard;
				a / 2 <= b / 2
			}) as u8);
		}
		11 => {
			put(out, it.position(|b| b == k8 / 2).map(|i| i as u8));
			put(out, it.reduce(|a, b| a.wrapping_mul(3) ^ b));
		}
		12 if k % 2 == 0 => {
			out.push(it.all(|b| b != k8 / 2) as u8);
			out.extend(it);
		}
		12 => it.for_each(|b| out.push(b)),
		// Each reduces by `next`, then by its fold, as std's adaptors do.
		13 if k % 2 == 0 => put(out, it.filter(|b| b % 3 != 1).max()),
		13 => put(out, it.filter_map(|b| (b % 3 != 1).then_some(b)).min()),
		// Each reads by `try_fold`, which reads on by `next` and never by `find`.
		14 => {
			let mut filter = it.filter(|b| b % 2 == 0);
			out.push(filter.any(|b| b > k8 / 2) as u8);
			out.extend(filter);
		}
		_ => match k % 4 {
			0 => {
				let mut skip = it.skip_while(|&b| b < 2);
				put(out, skip.position(|b| b > k8 / 2).map(|i| i as u8));
				out.extend(skip);
			}
			1 => {
				let mut filter_map = it.filter_map(|b| (b % 3 != 0).then_some(b));
				out.push(filter_map.all(|b| b != k8 / 2) as u8);
				out.extend(filter_map);
			}
			2 => {
				let mut chain = it.chain([7, 8]).filter(|b| b % 2 == 0);
				put(out, chain.find(|&b| b > k8 / 2));
				out.extend(chain);
			}
			_ => {
				let mut fused = it.by_ref().fuse().filter(|b| b % 2 == 1);
				put(out, fused.next());
				out.extend(it.rev());
			}
		},
	}
}

/// `it`, collected by std's `FromIterator` of the collections that collect a `Vec` by its
/// `collect` first, as a `BTreeMap` collects `Pairs` of it, compared to `0..k` by std's
/// comparisons that call its `eq` or `partial_cmp`, or read through its `by_ref`, picked by
/// `pick`.
fn collected<I: DoubleEndedIterator<Item = u8> + ExactSizeIterator>(
	mut it: I,
	pick: u8,
	k: usize,
	log: &RefCell<Vec<u8>>,
	out: &mut Vec<u8>,
) {
	let other = 0..k as u8;
	let order = |o: Option<Ordering>| o.map_or(0xff, |o| o as i8 as u8);
	let pairs = |map: BTreeMap<u8, u8>| map.into_iter().flat_map(|(k, v)| [k, v]);
	match pick {
		0 => out.extend(it.collect::<Box<[u8]>>()),
		1 => out.extend(Box::<[u8]>::from_iter(it)),
		2 => out.extend(Rc::<[u8]>::from_iter(it).iter()),
		3 => out.extend(Arc::<[u8]>::from_iter(it).iter()),
		4 => out.extend(BTreeSet::from_iter(it)),
		5 => out.extend(BinaryHeap::from_iter(it).into_vec()),
		6 => out.extend(pairs(BTreeMap::from_iter(Pairs(it, log)))),
		7 => out.extend(pairs(BTreeMap::from_iter(&mut Pairs(it, log)))),
		8 => out.extend(pairs(BTreeMap::from_iter(Box::new(Pairs(it, log))))),
		9 => out.push(u8::from(it.eq(other))),
		10 => out.push(u8::from(it.ne(other))),
		11 if k % 2 == 0 => out.push(u8::from(it.lt(other))),
		11 => out.push(u8::from(it.gt(other))),
		12 if k % 2 == 0 => out.push(u8::from(it.le(other))),
		12 => out.push(u8::from(it.ge(other))),
		13 => out.push(order(it.partial_cmp(other))),
		14 => {
			out.push(it.by_ref().take(k).count() as u8);
			for b in it.by_ref() {
				out.push(b);
			}
		}
		_ => {
			put(out, it.by_ref().next());
			put_vec(out, it.collect());
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
