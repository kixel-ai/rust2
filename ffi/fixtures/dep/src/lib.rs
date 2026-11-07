//! Another export, exported into the api fixture's library, which names its types.

/// A size: a record of this export, which another's signatures name.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
	pub width: u32,
	pub height: u32,
}

/// An area: an object of this export, which another's signatures take and return.
#[derive(Clone, Debug)]
pub struct Area {
	size: Size,
}

impl Area {
	/// An area of `size`, doubled on each side with the `doubled` feature.
	pub fn new(size: Size) -> Self {
		let scale = if cfg!(feature = "doubled") { 2 } else { 1 };
		let size = Size {
			width: size.width * scale,
			height: size.height * scale,
		};
		Self { size }
	}

	pub fn size(&self) -> Size {
		self.size
	}

	/// Its width times its height.
	pub fn pixels(&self) -> u64 {
		u64::from(self.size.width) * u64::from(self.size.height)
	}
}

/// Another crate's item that is no export, which each export re-exporting it refuses or
/// excludes on its own.
pub use std::collections::HashSet;

/// What no build compiles, which a config still names, as it does what one platform builds.
#[cfg(any())]
pub mod gone {}
