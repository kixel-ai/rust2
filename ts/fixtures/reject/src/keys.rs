//! Keys and orders of the crate's own, which the runtime's collections and comparisons don't call.

/// JS compares objects by identity, not by `PartialOrd`.
#[derive(PartialEq, PartialOrd)]
pub struct Key(pub u8);

pub fn keyed(range: std::ops::Range<Key>) -> bool {
	range.contains(&Key(2))
}

/// A key the runtime has no `Hash` and `Eq` of.
pub fn set_keys() -> usize {
	let map: std::collections::HashMap<std::collections::BTreeSet<u8>, u8> = std::collections::HashMap::new();
	map.len()
}

/// A key whose `Hash` and `Eq` agree with a `u8`'s, but which is a JS object.
#[derive(PartialEq, Eq, Hash)]
pub struct Id {
	pub n: u8,
}

impl std::borrow::Borrow<u8> for Id {
	fn borrow(&self) -> &u8 {
		&self.n
	}
}

/// Looked up by a `u8`, which the map would look for among its primitive keys.
pub fn borrowed(map: &std::collections::HashMap<Id, u8>) -> bool {
	map.contains_key(&1)
}

/// `max` drops the operand it doesn't return, which the runtime's leaves undropped.
pub fn widest(a: std::sync::Arc<str>, b: std::sync::Arc<str>) -> std::sync::Arc<str> {
	a.max(b)
}

/// std hands its closure a `&mut` to a value JS copies, which its mapping has as the value.
pub fn passed(map: &mut std::collections::HashMap<u8, u8>) -> u8 {
	map.get_mut(&1).map_or(0, |v| {
		*v += 1;
		*v
	})
}

/// A `hash_slice` of its own, which a `Vec`'s `Hash` calls where the translation hashes each
/// element.
#[derive(PartialEq, Eq)]
pub struct Batched(u8);

impl std::hash::Hash for Batched {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}

	fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H) {
		state.write_usize(data.len());
	}
}

pub fn batched(v: Vec<Batched>) -> usize {
	let mut set = std::collections::HashSet::new();
	set.insert(v);
	set.len()
}

/// The places `iter_mut` lends, which the runtime would compare as the objects they are.
pub fn places_sorted(v: &mut [u8]) -> bool {
	v.iter_mut().is_sorted()
}

impl PartialEq<Key> for u8 {
	fn eq(&self, other: &Key) -> bool {
		*self == other.0
	}
}

impl PartialEq<u8> for Key {
	fn eq(&self, other: &u8) -> bool {
		self.0 == *other
	}
}

impl PartialOrd<Key> for u8 {
	fn partial_cmp(&self, other: &Key) -> Option<std::cmp::Ordering> {
		self.partial_cmp(&other.0)
	}
}

impl PartialOrd<u8> for Key {
	fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(other)
	}
}

/// Integer bounds against the crate's item, which compares by its own `PartialOrd`.
pub fn keyed_across(n: u8) -> bool {
	(..10u8).contains(&Key(n)) || std::ops::RangeBounds::contains(&(1u8..), &Key(n))
}

/// Bounds behind `&mut`s, which JS may keep as places rather than the integers.
pub fn place_bounds(a: &mut u8, b: &mut u8) -> bool {
	(a..b).is_empty()
}
