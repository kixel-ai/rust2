//! `bytes`' buffers, and buffers of the crate's own, where JS's storage would tell apart what Rust's doesn't.

/// An `Arc`'s bytes would need a `ByteSlice` made inside it.
pub fn shared(b: std::sync::Arc<[u8; 2]>) -> std::sync::Arc<[u8]> {
	b
}

/// An owner that drops, which would drop with the last clone of the `Bytes`.
pub fn owned_bytes(owner: std::sync::Arc<[u8]>) -> usize {
	bytes::Bytes::from_owner(owner).len()
}

/// A `BytesMut`'s capacity, which `bytes` reclaims only once the `Bytes` it froze drops.
pub fn bytes_mut_capacity(n: usize) -> usize {
	let mut buf = bytes::BytesMut::with_capacity(n);
	buf.extend_from_slice(b"frame");
	drop(buf.split_to(2).freeze());
	buf.reserve(n);
	buf.capacity()
}

/// `bytes` reads a `Buf` of the crate's own into a `BytesMut` by its `copy_to_bytes` only
/// at zero capacity.
pub fn bytes_mut_put(buf: &mut bytes::BytesMut, src: Counted) {
	bytes::BufMut::put(buf, src);
}

/// Through a `&mut`, `bytes` advances it by the room left in the `BytesMut`'s capacity.
pub fn bytes_mut_put_through(mut buf: &mut bytes::BytesMut, src: Counted) {
	bytes::BufMut::put(&mut buf, src);
}

/// A `Buf` whose `copy_to_bytes` differs from its `chunk` and `advance`.
pub struct Counted(pub bytes::Bytes);

impl bytes::Buf for Counted {
	fn remaining(&self) -> usize {
		self.0.remaining()
	}

	fn chunk(&self) -> &[u8] {
		self.0.chunk()
	}

	fn advance(&mut self, cnt: usize) {
		self.0.advance(cnt);
	}

	fn copy_to_bytes(&mut self, len: usize) -> bytes::Bytes {
		bytes::Bytes::from(vec![0; len])
	}
}

/// A buffer of the crate's own that JS copies, so `&mut` to it is an `rt.Place`, which the
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

/// The room past a vector's length, whose size is its capacity.
pub fn spare(v: &mut Vec<u16>) -> usize {
	v.spare_capacity_mut().len()
}
