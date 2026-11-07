//! A module the items fixture re-exports, so its `pub` items are that crate's too.

pub mod shapes {
	/// `ShapesSides` in the items fixture's Go package.
	pub const SIDES: u8 = 4;
	/// `ShapesUnit`, another crate's static.
	pub static UNIT: u8 = 1;

	/// `ShapesArea` in the items fixture's Go package.
	pub fn area(w: u8, h: u8) -> u8 {
		w.wrapping_mul(h)
	}
}
