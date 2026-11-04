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

/// A `String`'s bytes, in storage of its capacity, which JS keeps none of, so this crate's
/// reads of a `Bytes`' capacity are refused.
pub fn string_bytes(text: String) -> bytes::Bytes {
	bytes::Bytes::from(text)
}

/// What reads a `Bytes`' capacity: whether it is the one handle of its storage, and
/// whether truncating gives the storage up.
pub fn capacity_reads(mut b: bytes::Bytes, mut buf: bytes::BytesMut) -> usize {
	use bytes::{Buf, BufMut};
	let unique = b.is_unique();
	let vec = Vec::from(b.clone());
	let taken = bytes::BytesMut::from(b.clone());
	buf.put(b.clone());
	buf.put(b.clone().chain(&b"chained"[..]));
	b.truncate(1);
	b.clear();
	let into = b.try_into_mut().map_or(0, |m| m.capacity());
	usize::from(unique) + vec.capacity() + taken.capacity() + into
}

/// The room past a vector's length, whose size is its capacity.
pub fn spare(v: &mut Vec<u16>) -> usize {
	v.spare_capacity_mut().len()
}
