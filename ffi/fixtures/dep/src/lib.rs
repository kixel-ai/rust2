//! Another crate, whose types the api fixture re-exports and names.

/// A size: a record of this crate, which the api fixture's signatures name.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
	pub width: u32,
	pub height: u32,
	/// What no build has, which the `configured_out` test excludes.
	#[cfg(any())]
	pub depth: u32,
}

/// An area: an object of this crate, which the api fixture's signatures take and return.
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

/// What no build compiles, which a config still names, as it does what one platform builds.
#[cfg(any())]
pub mod gone {}
