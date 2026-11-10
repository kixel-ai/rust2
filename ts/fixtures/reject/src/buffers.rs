//! `bytes`' buffers, and buffers of the crate's own, where JS's storage would tell apart what Rust's doesn't.

/// A buffer of the crate's own that JS copies, so `&mut` to it is an `rs.Place`, which the
/// provided `copy_to_bytes` doesn't read through.
pub enum Blank {
	Empty,
}

impl bytes::Buf for Blank {
	fn remaining(&self) -> usize {
		0
	}

	fn chunk(&self) -> &[u8] {
		&[]
	}

	fn advance(&mut self, cnt: usize) {
		assert_eq!(cnt, 0);
	}
}

pub fn blank_bytes(len: usize) -> usize {
	bytes::Buf::copy_to_bytes(&mut Blank::Empty, len).len()
}

/// `slice_ref` of bytes the body doesn't show of that `Bytes`'s storage: a `&'static str`'s,
/// which Rust shares and JS encodes anew, and of another `Bytes` of them.
pub fn sliced_elsewhere(n: usize) -> usize {
	let text = "shared";
	let a = bytes::Bytes::from_static(text.as_bytes());
	let b = bytes::Bytes::from_static(text.as_bytes());
	a.slice_ref(&text.as_bytes()[..n]).len() + a.slice_ref(&b[..n]).len()
}

/// `slice_ref` of a clone of a `Bytes` the body then points at other storage.
pub fn sliced_reassigned(n: usize) -> usize {
	let mut a = bytes::Bytes::from_static(b"one");
	let c = a.clone();
	a = bytes::Bytes::from_static(b"two");
	a.slice_ref(&c[..n]).len()
}

/// `slice_ref` of a clone of a `Bytes` that `mem::swap` or `mem::replace` points at other
/// storage through a `&mut`.
pub fn sliced_swapped(n: usize) -> usize {
	let mut a = bytes::Bytes::from_static(b"one");
	let mut b = bytes::Bytes::from_static(b"two");
	let c = a.clone();
	std::mem::swap(&mut a, &mut b);
	let mut d = bytes::Bytes::from_static(b"six");
	let e = d.clone();
	drop(std::mem::replace(&mut d, b));
	a.slice_ref(&c[..n]).len() + d.slice_ref(&e[..n]).len()
}

/// `slice_ref` of a clone of a `Bytes` that `mem::swap` points at other storage through a
/// `&mut` cast to a raw pointer and back.
pub fn sliced_cast_swapped(n: usize) -> usize {
	let mut a = bytes::Bytes::from_static(b"one");
	let mut b = bytes::Bytes::from_static(b"two");
	let c = a.clone();
	let p = &raw mut a;
	let q = p as *const bytes::Bytes;
	std::mem::swap(unsafe { &mut *(q as *mut bytes::Bytes) }, &mut b);
	a.slice_ref(&c[..n]).len()
}
