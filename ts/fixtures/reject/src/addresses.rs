//! Addresses read as integers where JS has no exact counterpart: of values one allocation
//! holds many of or beside another whose address is read, of places, of values of no
//! bytes, of a `Weak` whose value `Arc::new_cyclic` makes, and pointers made of addresses.

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
