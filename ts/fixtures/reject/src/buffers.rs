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

/// `slice_ref` of bytes the body doesn't show borrowed from that `Bytes`: a `&'static str`'s,
/// which Rust shares and JS encodes anew, of another `Bytes` of them, and of a clone, which
/// shares the storage in both.
pub fn sliced_elsewhere(n: usize) -> usize {
	let text = "shared";
	let a = bytes::Bytes::from_static(text.as_bytes());
	let b = bytes::Bytes::from_static(text.as_bytes());
	let c = a.clone();
	a.slice_ref(&text.as_bytes()[..n]).len() + a.slice_ref(&b[..n]).len() + a.slice_ref(c.as_ref()).len()
}
