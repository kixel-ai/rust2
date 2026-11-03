//! Iterators and collections of the crate's own that override what std's adaptors call.

/// `scan` of items that can be `undefined`: its closure's `None` and `Some(None)` would both be.
pub fn scanned(v: &[u8]) -> usize {
	v.iter()
		.scan(Vec::<u8>::new(), |_, &b| Some((b > 1).then_some(b)))
		.count()
}

/// An iterator whose `fold` std's adaptors would call in place of its `next`.
pub struct Folds;

impl Iterator for Folds {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, _f: F) -> B {
		init
	}
}

pub fn folds() -> usize {
	Folds.map(|b| b + 1).count()
}

/// An iterator whose `nth_back` std's adaptors would call in place of its `next_back`.
pub struct Backs;

impl Iterator for Backs {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}
}

impl DoubleEndedIterator for Backs {
	fn next_back(&mut self) -> Option<u8> {
		None
	}

	fn nth_back(&mut self, _n: usize) -> Option<u8> {
		Some(0)
	}
}

pub fn backs() -> usize {
	Backs.rev().skip(1).count()
}

/// `scan` of `()`s: its closure's `None` and `Some(())` would both be `undefined`.
pub fn scanned_unit(v: &[u8]) -> usize {
	v.iter().scan(Vec::<u8>::new(), |_, _| Some(())).count()
}

pub struct Part(pub u64);

impl std::iter::Sum<Part> for std::time::Duration {
	fn sum<I: Iterator<Item = Part>>(parts: I) -> Self {
		std::time::Duration::from_secs(parts.map(|p| p.0 * 2).sum())
	}
}

impl std::iter::Sum<Part> for u64 {
	fn sum<I: Iterator<Item = Part>>(parts: I) -> Self {
		parts.map(|p| p.0 * 2).sum()
	}
}

/// A `Sum` of the crate's into a std type, which std's `Sum` mapping would bypass.
pub fn parts_summed(a: Vec<Part>, b: Vec<Part>) -> (std::time::Duration, u64) {
	(a.into_iter().sum(), b.into_iter().sum())
}

/// A collection of the crate's own that overrides `extend_one`, which a tuple's `Extend`
/// calls, as only a nightly crate can.
#[derive(Default)]
pub struct Tens(pub u8);

impl Extend<u8> for Tens {
	fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
		for b in iter {
			self.0 = self.0.wrapping_add(b);
		}
	}

	fn extend_one(&mut self, item: u8) {
		self.0 = self.0.wrapping_add(item).wrapping_mul(10);
	}
}

/// A tuple of collections, one of them the crate's own that overrides `extend_one`.
pub fn tens(data: &[u8]) -> (Vec<u8>, Tens) {
	data.iter().map(|&b| (b, b)).collect()
}
