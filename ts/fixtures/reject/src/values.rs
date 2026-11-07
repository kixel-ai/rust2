//! Values JS can't represent as Rust does.

use crate::places::Bare;

pub fn float(a: f16) -> f16 {
	a * 2.0
}

/// `None` and `Some(None)` would both be `rs.None`, as `Some(Some(None))` is `undefined`.
pub fn nested(x: Option<Option<Option<u8>>>) -> bool {
	x.is_some()
}

/// A generic type's `Option<T>` is `T | undefined`, which can't hold `rs.None`, the `None`
/// of an `Option<()>`.
pub struct Maybe<T> {
	pub value: Option<T>,
}

pub fn generic_unit(value: Option<()>) -> bool {
	Maybe { value }.value.is_some()
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
