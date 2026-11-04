//! Formatting the runtime doesn't do as std does.

/// A pointer's address, which JS has no counterpart of.
pub fn pointer(n: u8) -> String {
	format!("{:p}", &n)
}

/// `Debug` of a `ChunksMut`, which writes its slice's address.
pub fn chunks_address(v: &mut [u8]) -> String {
	format!("{:?}", v.chunks_mut(2))
}
