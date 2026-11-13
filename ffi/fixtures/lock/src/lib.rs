//! An API exported for the host and wasm32, whose `next` feature breaks it in pairs and
//! whose `added` feature only adds to it.

/// Its C name, `lock_audio_encoder_finish`, is taken by `audio::encoder_finish` in `next`,
/// which comes first by path.
pub mod audio_encoder {
	pub fn finish(level: u32) -> u32 {
		level
	}
}

#[cfg(feature = "next")]
pub mod audio {
	pub fn encoder_finish(level: u32) -> u32 {
		level + 1
	}
}

/// `LOCK_LIMITS_MAX_RATE`, which `limits::MAX_RATE` takes in `next`.
pub mod limits_max {
	pub const RATE: u32 = 60;
}

#[cfg(feature = "next")]
pub mod limits {
	pub const MAX_RATE: u32 = 30;
}

#[cfg(not(feature = "next"))]
pub fn removed() -> u32 {
	1
}

/// Not on wasm32 in `next`.
#[cfg(any(not(feature = "next"), not(target_arch = "wasm32")))]
pub fn portable() -> u32 {
	2
}

#[cfg(not(feature = "next"))]
pub fn scale(x: u32) -> u32 {
	x * 2
}

#[cfg(feature = "next")]
pub fn scale(x: u64) -> u64 {
	x * 2
}

#[cfg_attr(feature = "added", derive(Clone))]
pub struct Meter {
	total: u64,
}

impl Meter {
	pub fn new() -> Self {
		Self { total: 0 }
	}

	#[cfg(not(feature = "next"))]
	pub fn add(&mut self, n: u32) {
		self.total += u64::from(n);
	}

	#[cfg(feature = "next")]
	pub fn add(&mut self, n: u16) {
		self.total += u64::from(n);
	}

	#[cfg(feature = "added")]
	pub fn reset(&mut self) {
		self.total = 0;
	}

	pub fn total(&self) -> u64 {
		self.total
	}
}

impl Default for Meter {
	fn default() -> Self {
		Self::new()
	}
}

pub struct Size {
	pub width: u32,
	#[cfg(feature = "next")]
	pub depth: u32,
}

/// Only wasm32's API has it, so only the staged target's API breaks.
#[cfg(target_arch = "wasm32")]
pub struct Canvas {
	#[cfg(not(feature = "next"))]
	pub width: u32,
	#[cfg(feature = "next")]
	pub width: u64,
}

#[cfg(target_arch = "wasm32")]
pub fn canvas(width: u32) -> Canvas {
	Canvas { width: width.into() }
}

pub enum Mode {
	Fast,
	Slow,
	#[cfg(feature = "next")]
	Idle,
}

pub enum Shape {
	#[cfg(not(feature = "next"))]
	Circle(u32),
	#[cfg(feature = "next")]
	Circle(f64),
}

#[non_exhaustive]
pub enum Level {
	Low,
	#[cfg(feature = "added")]
	High,
}

pub fn pick(mode: Mode, shape: Shape, size: Size, level: Level) -> u32 {
	let _ = (shape, size, level);
	match mode {
		Mode::Fast => 1,
		_ => 0,
	}
}

#[cfg(feature = "added")]
pub fn extra() -> u32 {
	3
}
