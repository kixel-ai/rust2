//! Iterators and collections of the crate's own that override what std's adaptors call.

/// An iterator whose own `fold` holds its accumulator in an `Option`, of an `rt.None` or not.
pub struct Folds;

impl Iterator for Folds {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, _f: F) -> B {
		Some(init).unwrap()
	}
}

pub fn folds() -> usize {
	Folds.map(|b| b + 1).count()
}

/// An iterator whose `rfind` std's adaptors would call in place of its `next_back`.
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

	fn rfind<P: FnMut(&u8) -> bool>(&mut self, _p: P) -> Option<u8> {
		Some(0)
	}
}

pub fn backs() -> Option<u8> {
	Backs.rev().find(|&b| b == 0)
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

/// An iterator whose own `fold` tells its accumulator apart by a helper generic over it,
/// in a closure.
pub struct Keeps;

impl Iterator for Keeps {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, _f: F) -> B {
		let kept = |x| keep(x);
		kept(init)
	}
}

#[inline(never)]
fn keep<T>(x: T) -> T {
	Some(x).unwrap()
}

pub fn keeps() -> usize {
	Keeps.map(|b| b + 1).count()
}

/// An iterator whose own `fold` tells its accumulator apart by its size.
pub struct Sizes;

impl Iterator for Sizes {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, _f: F) -> B {
		assert!(const { size_of::<B>() } > 0);
		init
	}
}

pub fn sizes() -> usize {
	Sizes.map(|b| b + 1).count()
}

/// An iterator whose own `fold` drops its accumulator, as the one it is translated for drops.
pub struct Discards;

impl Iterator for Discards {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	#[allow(clippy::empty_loop)]
	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, _f: F) -> B {
		{
			let _held = init;
		}
		loop {}
	}
}

pub fn discards() -> usize {
	Discards.map(|b| b + 1).count()
}

/// An iterator whose own `fold` drops its accumulator by `drop`, which only its closure can be.
pub struct Disposes;

impl Iterator for Disposes {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	#[allow(clippy::empty_loop)]
	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, f: F) -> B {
		drop(f);
		drop(init);
		loop {}
	}
}

pub fn disposes() -> usize {
	Disposes.map(|b| b + 1).count()
}

/// An iterator whose own `fold` tells its accumulator apart by an iterator of the crate over it.
pub struct Probes;

impl Iterator for Probes {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, _f: F) -> B {
		assert!(Probe(&init).count() > 0);
		init
	}
}

/// Counts its type parameter's size.
pub struct Probe<'a, T>(&'a T);

impl<T> Iterator for Probe<'_, T> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn count(self) -> usize {
		size_of::<T>()
	}
}

pub fn probes() -> usize {
	Probes.map(|b| b + 1).count()
}

/// An iterator whose own `fold` tells its closure apart by the size of a closure holding it.
pub struct Measures;

impl Iterator for Measures {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn fold<B, F: FnMut(B, u8) -> B>(self, init: B, f: F) -> B {
		let held = move || {
			let _held = &f;
		};
		assert!(measure(held) > 0);
		init
	}
}

#[inline(never)]
fn measure<T>(_x: T) -> usize {
	size_of::<T>()
}

pub fn measures() -> usize {
	Measures.map(|b| b + 1).count()
}
