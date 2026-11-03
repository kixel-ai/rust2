//! `clone_from` and `Clone` of the crate's own, which std calls where the translation would not.

use crate::places::Noisy;

pub fn refilled(v: &mut [Reused], r: Reused) {
	v.fill(r);
}

/// `clone_from_slice` calls `Reused`'s own `clone_from` for each element.
pub fn recloned(v: &mut [Reused], w: &[Reused]) {
	v.clone_from_slice(w);
}

/// A `clone_from` of its own, which `fill` calls for each slot but the last.
pub struct Reused(Vec<u8>);

impl Clone for Reused {
	fn clone(&self) -> Self {
		Reused(self.0.clone())
	}

	fn clone_from(&mut self, source: &Self) {
		self.0.clone_from(&source.0);
	}
}

/// `clone_from` of a `Vec` whose elements' is their own.
pub fn vec_clone_from(v: &mut Vec<Reused>, from: &Vec<Reused>) {
	v.clone_from(from);
}

/// The tags of the first of two slots, each written whole as `fill` beside a pair of
/// tags, then changed through a pointer to them.
macro_rules! tagged_slots {
	($slots:ident) => {{
		let tags = unsafe { &raw mut $slots[0].assume_init_mut().1 };
		unsafe {
			(*tags).0 ^= 1;
			(*tags).0
		}
	}};
}

#[derive(Clone, Copy)]
pub struct Cloned(pub u8);

/// Slots of a type parameter's values written whole through an iterator zipped with
/// clones of them: std's code calls the crate's `Clone`, so the writes fill nothing fresh.
pub fn cloned_slots(fill: [Cloned; 2]) -> u8 {
	kept_slots(fill)
}

fn kept_slots<T: Clone>(fill: [T; 2]) -> u8 {
	let mut slots: [std::mem::MaybeUninit<([T; 2], (u8, u8))>; 2] = [const { std::mem::MaybeUninit::uninit() }; 2];
	for (slot, value) in slots.iter_mut().zip(fill.iter().cloned()) {
		slot.write(([value.clone(), value], (0, 1)));
	}
	tagged_slots!(slots)
}

pub struct Dropped(pub Noisy);

/// [`cloned_slots`] zipped with a vector's values, which std's iterator drops through
/// the crate's `Drop` of a field.
pub fn dropped_slots(fill: Vec<Dropped>) -> u8 {
	moved_slots(fill)
}

fn moved_slots<T>(fill: Vec<T>) -> u8 {
	let mut slots: [std::mem::MaybeUninit<(T, (u8, u8))>; 2] = [const { std::mem::MaybeUninit::uninit() }; 2];
	for (slot, value) in slots.iter_mut().zip(fill) {
		slot.write((value, (0, 1)));
	}
	tagged_slots!(slots)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Greatest(pub u8);

/// A slot written whole through the greatest of a type parameter's values, which std
/// finds through the crate's `Ord`.
pub fn ranked_slots(fill: [Greatest; 2]) -> u8 {
	greatest_slot(fill)
}

fn greatest_slot<T: Ord + Copy>(fill: [T; 2]) -> u8 {
	let mut slots: [([T; 2], (u8, u8)); 2] = [(fill, (0, 0)); 2];
	let Some(top) = slots.iter_mut().max() else {
		return 0;
	};
	*top = (fill, (1, 2));
	let tags = &raw mut top.1;
	unsafe {
		(*tags).0 ^= 1;
		(*tags).0
	}
}

/// What a type parameter's value does to a slot of it.
pub trait Touch: Sized {
	fn touch(&self, slot: &mut std::mem::MaybeUninit<([Self; 2], (u8, u8))>);
}

#[derive(Clone, Copy)]
pub struct Touched(pub u8);

impl Touch for Touched {
	fn touch(&self, _: &mut std::mem::MaybeUninit<([Self; 2], (u8, u8))>) {}
}

/// [`cloned_slots`] where each slot is first given to the crate's own trait, through
/// the type parameter's bound.
pub fn touched_slots(fill: [Touched; 2]) -> u8 {
	given_slots(fill)
}

fn given_slots<T: Touch + Copy>(fill: [T; 2]) -> u8 {
	let mut slots: [std::mem::MaybeUninit<([T; 2], (u8, u8))>; 2] = [const { std::mem::MaybeUninit::uninit() }; 2];
	for (slot, at) in slots.iter_mut().zip(0u8..) {
		fill[0].touch(slot);
		slot.write((fill, (at, !at)));
	}
	tagged_slots!(slots)
}
