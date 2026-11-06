//! Addresses read as integers where JS has no exact counterpart: of values the program
//! keeps anywhere but as an `Arc`'s value (a local it borrows or drops, a field, a `Box`,
//! a collection, a static, or where JS keeps it) or moves out of its `Arc`, of places, of
//! values of no bytes, of a `Weak` whose value `Arc::new_cyclic` makes, and pointers made
//! of addresses.

use std::sync::{Arc, Weak};

/// What a `Vec` holds many of.
pub struct Element(pub u32);

/// The addresses of a `Vec`'s elements, which Rust keeps a fixed distance apart.
pub fn elements(v: &Vec<Element>) -> Option<usize> {
	v.iter().map(|e| std::ptr::from_ref(e) as usize).max()
}

/// One of two parts of a `Pair`.
pub struct Part(pub u32);

/// What holds two `Part`s.
pub struct Pair {
	pub first: Part,
	pub second: Part,
}

/// The addresses of a struct's two parts, which Rust orders as its layout does.
pub fn parts(pair: &Pair) -> bool {
	std::ptr::from_ref(&pair.first).addr() < std::ptr::from_ref(&pair.second).addr()
}

/// The address of a place, which JS makes anew each borrow, and of a value of no bytes.
pub fn places(n: &u32, unit: &()) -> usize {
	std::ptr::from_ref(n) as usize ^ std::ptr::from_ref(unit) as usize
}

/// Raw pointers to places ordered and hashed by their addresses.
pub fn place_keys(a: &u32, b: &u32) -> bool {
	let keys = std::collections::BTreeSet::from([&raw const *a]);
	let hashed = std::collections::HashSet::from([&raw const *b]);
	keys.len() == hashed.len() && (&raw const *a) < (&raw const *b)
}

/// What `Arc::new_cyclic` makes, whose `Weak` points at no value until it returns.
pub struct Cyclic {
	pub me: Weak<Cyclic>,
	pub at: usize,
}

/// `Weak::as_ptr` before the value is made.
pub fn cyclic() -> Arc<Cyclic> {
	Arc::new_cyclic(|me| Cyclic {
		me: me.clone(),
		at: me.as_ptr() as usize,
	})
}

/// Pointers made of addresses, which JS has no memory at.
pub fn made(p: *const Element, at: usize) -> [*const Element; 3] {
	[p.with_addr(at), std::ptr::without_provenance(at), at as *const Element]
}

/// What a local holds, borrowed.
pub struct Borrowed(pub u8);

/// The address of a local, which JS has no allocation of to free.
pub fn borrowed() -> usize {
	let local = Borrowed(1);
	std::ptr::from_ref(&local) as usize
}

/// What a local holds until it drops, and an `Arc` holds elsewhere.
pub struct Dropped(pub u8);

impl Drop for Dropped {
	fn drop(&mut self) {
		self.0 = 0;
	}
}

/// The address of an `Arc`'s `Dropped`, of a type a local drops too.
pub fn dropped() -> usize {
	let shared = Arc::new(Dropped(1));
	let _local = Dropped(2);
	Arc::as_ptr(&shared) as usize
}

/// What a `Box` holds.
pub struct Boxed(pub u8);

/// The address of a box's value.
pub fn boxed() -> usize {
	let b = Box::new(Boxed(1));
	std::ptr::from_ref(&*b) as usize
}

/// What a static holds.
pub struct Held(pub u8);

static HELD: Held = Held(1);

/// The address of a static.
pub fn held() -> usize {
	std::ptr::from_ref(&HELD) as usize
}

/// What JS is given whole.
pub struct Given(pub u8);

/// A `Given` JS keeps where it likes.
pub fn given() -> Given {
	Given(1)
}

/// The address of a `Given`, which JS may keep anywhere.
pub fn given_at(given: &Given) -> usize {
	std::ptr::from_ref(given) as usize
}

/// What `Arc::into_inner` moves out.
pub struct Inner(pub u8);

/// The address of an `Arc`'s value, and the value moved out of it.
pub fn inner(shared: Arc<Inner>) -> usize {
	let at = Arc::as_ptr(&shared) as usize;
	at ^ Arc::into_inner(shared).map_or(0, |inner| usize::from(inner.0))
}

/// The values `Arc::try_unwrap` and `unwrap_or_clone` move out, which translate for no type.
pub fn unwrapped(shared: Arc<Inner>, other: Arc<Unshared>) -> u8 {
	Arc::try_unwrap(shared).map_or(0, |inner| inner.0) ^ Arc::unwrap_or_clone(other).0
}

/// What `Arc::make_mut` may move to a new `Arc`.
#[derive(Clone)]
pub struct Unshared(pub u8);

/// The address of an `Arc`'s value made its own.
pub fn unshared(shared: &mut Arc<Unshared>) -> usize {
	Arc::make_mut(shared).0 += 1;
	Arc::as_ptr(shared) as usize
}

/// What `Arc::get_mut` writes whole.
pub struct Rewritten(pub u8);

/// The address of an `Arc`'s value written whole through `get_mut`.
pub fn rewritten(shared: &mut Arc<Rewritten>) -> usize {
	if let Some(value) = Arc::get_mut(shared) {
		*value = Rewritten(2);
	}
	Arc::as_ptr(shared) as usize
}
