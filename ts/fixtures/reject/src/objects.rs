//! Trait objects the translation doesn't make.

/// An object of std's trait of a std type whose impl no mapping makes.
pub fn dynamic(v: &mut Vec<u8>) -> usize {
	let w: &mut dyn std::io::Write = v;
	w.write(&[1]).unwrap_or(0)
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

impl Scaled<u8> for u8 {
	fn scale(&self, by: u8) -> u8 {
		self.wrapping_mul(by)
	}
}

impl Scaled<u16> for u8 {
	fn scale(&self, by: u16) -> u16 {
		u16::from(*self).wrapping_mul(by)
	}
}

/// One type as objects of two arguments of one generic trait, which the class of the
/// value's own type, shared by its objects, can't both have.
pub fn scaled_twice(x: &u8) -> u16 {
	let a: &dyn Scaled<u8> = x;
	let b: &dyn Scaled<u16> = x;
	u16::from(a.scale(2)).wrapping_add(b.scale(3))
}

pub trait Taken {
	type Item;
	fn take(&mut self) -> Option<Self::Item>;
}

impl Taken for u32 {
	type Item = std::mem::MaybeUninit<u32>;
	fn take(&mut self) -> Option<std::mem::MaybeUninit<u32>> {
		Some(std::mem::MaybeUninit::new(*self))
	}
}

/// An object whose method returns an `Option` of its associated type, which its type has
/// as an `rs.OptionOf<Item>`, bound to a `MaybeUninit<u32>`, whose `None` is `rs.None`
/// though TypeScript's type of it, a `number`, can't be `undefined`.
pub fn uninit_item(x: &mut u32) -> bool {
	let s: &mut dyn Taken<Item = std::mem::MaybeUninit<u32>> = x;
	s.take().is_some()
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
/// an `rs.ByteSlice`.
pub fn first_of(n: u8) -> u8 {
	let b: Box<dyn First> = Box::new([n, 8, 9, 10]);
	b.first()
}

pub mod first {
	pub trait Named {
		fn name(&self) -> u8;
	}

	impl Named for u8 {
		fn name(&self) -> u8 {
			*self
		}
	}
}

pub mod second {
	pub trait Named {
		fn name(&self) -> u8;
	}

	impl Named for u8 {
		fn name(&self) -> u8 {
			self.wrapping_add(1)
		}
	}
}

/// Objects of two traits of one name with a method of one name, which the class of the
/// value's own type, shared by its objects, can't both have.
pub fn same_named(x: &u8) -> u8 {
	let a: &dyn first::Named = x;
	let b: &dyn second::Named = x;
	a.name().wrapping_add(b.name())
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

/// A raw pointer to an iterator as one to a trait object, whose object would hold the
/// `Range` there as it is made, not as it is read.
pub fn raw_dyn_iter(n: u8) -> Option<u8> {
	let mut range = 0..n;
	let p: *mut dyn Iterator<Item = u8> = &raw mut range;
	unsafe { (&mut *p).next() }
}

/// Raw pointers to trait objects compared, each an object made where it unsized.
pub fn dyn_compared(n: &mut u8) -> bool {
	let p: *mut dyn Reading = n;
	let q: *mut dyn Reading = n;
	std::ptr::eq(p, q)
}

impl Reading for [u16; 2] {
	fn read(&self) -> u8 {
		self[0] as u8
	}
}

/// An array as an object, whose class's methods would take the array a box holds and the
/// slice of it a reference is.
pub fn array_object(a: std::sync::Arc<[u16; 2]>) -> std::sync::Arc<dyn Reading> {
	a
}
