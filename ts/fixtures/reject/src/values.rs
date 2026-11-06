//! Values JS can't represent as Rust does.

use crate::places::Bare;

pub fn float(a: f16) -> f16 {
	a * 2.0
}

/// `None` and `Some(None)` would both be `rs.None`, as `Some(Some(None))` is `undefined`.
pub fn nested(x: Option<Option<Option<u8>>>) -> bool {
	x.is_some()
}

/// A generic type's `Option<T>` is an `rs.OptionOf<T>`, which decides its `None` by
/// whether TypeScript's type of `T` can be `undefined`, as a `MaybeUninit<u32>`'s `number`
/// can't, though its value can, so an `Option<MaybeUninit<u32>>`'s `None` is `rs.None`.
pub struct Maybe<T> {
	pub value: Option<T>,
}

pub fn generic_uninit(value: u32) -> bool {
	Maybe {
		value: Some(std::mem::MaybeUninit::new(value)),
	}
	.value
	.is_some()
}

/// Parsing an address is not translated yet.
pub fn addr_parsed(s: &str) -> Result<std::net::Ipv4Addr, std::net::AddrParseError> {
	s.parse()
}

/// A union whose fields overlap in values JS keeps apart.
pub union Bits {
	pub int: u32,
	pub float: f64,
}

pub fn bits(n: u32) -> u32 {
	let b = Bits { int: n };
	unsafe { b.int }
}

/// Pointers to places of no bytes, which Rust may put anywhere, `()`'s refused as made.
pub fn units_compared() -> bool {
	let (a, b, c, d) = ((), (), Bare, Bare);
	std::ptr::eq(&raw const a, std::ptr::NonNull::from(&b).as_ptr()) && std::ptr::eq(&raw const c, &raw const d)
}

pub enum Only {
	One,
}

/// A dangling pointer to an enum of no bytes, whose value is its variant's tag.
pub fn dangling_enum() -> bool {
	unsafe { std::ptr::NonNull::<Only>::dangling().as_ptr().as_ref().is_some() }
}

/// `u8`'s unstable ASCII methods are not translated.
pub fn ascii_octal(a: u8) -> bool {
	a.is_ascii_octdigit()
}

#[derive(Clone, Copy)]
pub struct Block<const N: usize>([u8; N]);

/// A struct generic over a length, which no class is, cloned out of an `Option` a pair
/// holds and written back whole in a body generic over that length.
pub fn block_cloned(a: Option<Block<2>>) -> Option<Block<2>> {
	reclone(a)
}

fn reclone<const N: usize>(a: Option<Block<N>>) -> Option<Block<N>> {
	let mut pair = (a, 0u8);
	#[allow(clippy::clone_on_copy, reason = "the clone `Option`'s `Clone` makes")]
	let copy = pair.0.clone();
	pair.0 = copy;
	pair.0
}

/// `LinkedList`'s unstable `retain`, which no mapping covers, as none does its other
/// unstable methods.
pub fn lists(l: &mut std::collections::LinkedList<u8>) {
	l.retain(|x| *x > 1);
}

/// `ManuallyDrop`'s impls beyond holding a value: compared, formatted, and a constant.
pub fn manual_impls(a: std::mem::ManuallyDrop<u32>, b: std::mem::ManuallyDrop<u32>) -> String {
	const HELD: std::mem::ManuallyDrop<u32> = std::mem::ManuallyDrop::new(3);
	format!("{a:?} {} {}", a == b, *HELD)
}
