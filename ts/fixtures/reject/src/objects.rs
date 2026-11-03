//! Trait objects the translation doesn't make.

pub fn dynamic(h: &mut dyn std::hash::Hasher) -> u64 {
	h.finish()
}

pub trait Reading {
	fn read(&self) -> u8;
}

impl Reading for u8 {
	fn read(&self) -> u8 {
		*self
	}
}

pub trait Scaled<T> {
	fn scale(&self, by: T) -> T;
}

/// A trait object of a generic trait.
pub fn generic_object(s: &dyn Scaled<u8>) -> u8 {
	s.scale(2)
}

pub trait First {
	fn first(&self) -> u8;
}

impl First for [u8; 4] {
	fn first(&self) -> u8 {
		self[0]
	}
}

/// A trait object of a byte array, which a box holds as a `Uint8Array` and a reference as
/// an `rt.ByteSlice`.
pub fn first_of(n: u8) -> u8 {
	let b: Box<dyn First> = Box::new([n, 8, 9, 10]);
	b.first()
}

pub trait Keyed: std::any::Any {
	fn key(&self) -> u8;
}

impl Keyed for u8 {
	fn key(&self) -> u8 {
		*self
	}
}

/// An upcast to `dyn Any`, whose downcast checks for the class of the value's own type,
/// not the object of another trait.
pub fn any_upcast(k: &dyn Keyed) -> bool {
	let any: &dyn std::any::Any = k;
	any.is::<u8>()
}

/// The same upcast of a `Weak`.
pub fn weak_any_upcast(k: std::rc::Weak<dyn Keyed>) -> bool {
	let any: std::rc::Weak<dyn std::any::Any> = k;
	any.upgrade().is_some_and(|a| a.is::<u8>())
}

/// A `TypeId`, which no value stands for yet.
pub fn any_type_id(a: &dyn std::any::Any) -> bool {
	a.type_id() == std::any::TypeId::of::<u8>()
}

pub trait Source {
	type Item: Reading;
	fn item(&self) -> &Self::Item;
}

/// A trait object of a type only an instance decides, which a census lowering this over
/// its type parameter can't make.
pub fn source_reading<S: Source>(s: &S) -> u8 {
	let r: &dyn Reading = s.item();
	r.read()
}

/// `size_of_val` of a trait object, which its vtable gives.
pub fn dyn_size(d: &dyn std::fmt::Debug) -> usize {
	size_of_val(d)
}

/// A raw pointer to an iterator as one to a trait object, whose `rt.Place` no `rt.Iter` is.
pub fn raw_dyn_iter(n: u8) -> Option<u8> {
	let mut range = 0..n;
	let p: *mut dyn Iterator<Item = u8> = &raw mut range;
	unsafe { (&mut *p).next() }
}
