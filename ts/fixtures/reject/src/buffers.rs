//! `bytes`' buffers, and buffers of the crate's own, where JS's storage would tell apart what Rust's doesn't.

/// An `Arc`'s bytes would need a `ByteSlice` made inside it.
pub fn shared(b: std::sync::Arc<[u8; 2]>) -> std::sync::Arc<[u8]> {
	b
}

/// An owner that drops, which would drop with the last clone of the `Bytes`.
pub fn owned_bytes(owner: std::sync::Arc<[u8]>) -> usize {
	bytes::Bytes::from_owner(owner).len()
}

/// The room past a `BytesMut`'s bytes, as a slice JS would hold apart from its storage.
pub fn bytes_mut_spare(buf: &mut bytes::BytesMut) -> usize {
	buf.spare_capacity_mut().len()
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
