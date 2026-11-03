//! Strings and `Cow`s, which JS keeps as primitives.

use crate::places::Noisy;

/// A closure that owns a value that drops, given as a `Pattern`, which `contains` would
/// call but never drop.
pub fn pattern_closure(s: &str) -> bool {
	let n = Noisy(1);
	s.contains(move |c: char| c as u8 == (&n).0)
}

/// A `Cow<str>` is a JS string, which keeps no variant to match on or ask about.
pub fn cow_matched(c: std::borrow::Cow<'_, str>) -> String {
	match c {
		std::borrow::Cow::Borrowed(s) => s.to_string(),
		std::borrow::Cow::Owned(s) => s,
	}
}

pub fn cow_borrowed(c: &std::borrow::Cow<'_, str>) -> bool {
	std::borrow::Cow::is_borrowed(c)
}

/// A slice's tells its variants apart, but asking is unstable.
pub fn cow_bytes(c: &std::borrow::Cow<'_, [u8]>) -> bool {
	std::borrow::Cow::is_borrowed(c)
}

/// Made where its type goes unchecked, as a local's, it is refused where it is made.
pub fn cow_made(a: &std::sync::Arc<u8>) -> usize {
	{
		let c: std::borrow::Cow<'_, std::sync::Arc<u8>> = std::borrow::Cow::Owned(a.clone());
		drop(c);
	}
	std::sync::Arc::strong_count(a)
}

/// A closure that owns a value that drops, which `String::retain` would call but never drop.
pub fn string_retained(s: &mut String) {
	let n = Noisy(1);
	s.retain(move |c| c as u8 != (&n).0);
}

/// A `str`'s UTF-8 written as bytes, or a part of it unchecked.
pub fn str_bytes_mut(s: &mut str) {
	unsafe { s.as_bytes_mut()[0] = b'a' };
	unsafe { s.get_unchecked_mut(..1) }.make_ascii_uppercase();
}

/// Bytes written through a `&mut str` to them, which no JS string is.
pub fn str_from_bytes_mut(bytes: &mut [u8]) {
	if let Ok(text) = std::str::from_utf8_mut(bytes) {
		text.make_ascii_uppercase();
	}
}
