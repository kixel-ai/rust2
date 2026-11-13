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
	Square(u32),
}

/// Its tags are its variants' places, which `next` swaps.
#[non_exhaustive]
pub enum Stroke {
	#[cfg(feature = "next")]
	Dashed(u32),
	Solid(u32),
	#[cfg(not(feature = "next"))]
	Dashed(u32),
	#[cfg(feature = "added")]
	Dotted(u32),
}

/// A C enum, which a variant with fields in `next` makes a struct.
#[non_exhaustive]
pub enum Level {
	Low,
	#[cfg(feature = "added")]
	High,
	#[cfg(feature = "next")]
	Custom(u8),
}

/// A handle `added` makes `Clone`, so copies of it may share one value.
#[cfg_attr(feature = "added", derive(Clone))]
pub struct Token {
	id: u32,
}

impl Token {
	pub fn new(id: u32) -> Self {
		Self { id }
	}

	pub fn id(&self) -> u32 {
		self.id
	}
}

pub fn pick(mode: Mode, shape: Shape, stroke: Stroke, size: Size, level: Level) -> u32 {
	let _ = (shape, stroke, size, level);
	match mode {
		Mode::Fast => 1,
		_ => 0,
	}
}

#[cfg(feature = "added")]
pub fn extra() -> u32 {
	3
}

/// A constant `next` widens, on the host alone.
#[cfg(all(not(feature = "next"), not(target_arch = "wasm32")))]
pub const DEPTH: u32 = 8;
#[cfg(all(feature = "next", not(target_arch = "wasm32")))]
pub const DEPTH: u64 = 8;

/// A constant `next` makes signed, on wasm32 alone.
#[cfg(all(not(feature = "next"), target_arch = "wasm32"))]
pub const PAGES: u16 = 4;
#[cfg(all(feature = "next", target_arch = "wasm32"))]
pub const PAGES: i16 = 4;

/// A constant whose value `added` changes, which is no break.
#[cfg(not(feature = "added"))]
pub const LIMIT: u32 = 10;
#[cfg(feature = "added")]
pub const LIMIT: u32 = 20;

/// A tuple whose C name, `lock_tuple_tuple_u32_u64_u8`, `next` gives another tuple too.
pub fn nested(x: ((u32,), u64, u8)) -> u32 {
	x.0.0
}

#[cfg(feature = "next")]
pub fn flat(x: ((u32, u64), u8)) -> u32 {
	x.0.0
}

/// Kept class whose Pascal name collides only in the bindings.
pub mod graphics {
	pub mod encoder {
		pub struct Finish {
			pub value: u32,
		}
	}

	#[cfg(feature = "names")]
	pub struct EncoderFinish {
		pub value: u32,
	}
}

#[cfg(feature = "names")]
#[allow(non_snake_case)]
pub fn smoothStep() -> u32 {
	2
}

/// Kept function whose camel name is claimed by the added function first.
pub fn smooth_step() -> u32 {
	1
}

/// Adding a field changes a C++ method's name without changing its C symbol.
#[repr(C)]
pub struct Members {
	pub other: u32,
	#[cfg(feature = "members")]
	pub value: u32,
}

impl Members {
	pub fn value(&self) -> u32 {
		self.other
	}
}

#[cfg(feature = "added")]
pub fn failing_borrowed() -> Result<u32, &'static str> {
	Err("failure")
}

/// Error wrappers and variants occupy their own language namespaces.
#[cfg(not(feature = "closures"))]
pub fn failing() -> Result<u32, String> {
	Err("failure".to_string())
}

#[cfg(feature = "wrappers")]
pub struct StringException {
	pub value: u32,
}
#[cfg(feature = "wrappers")]
pub struct StringError {
	pub value: u32,
}
#[cfg(feature = "wrappers")]
pub struct ShapeCircle {
	pub value: u32,
}
#[cfg(feature = "wrappers")]
pub struct ModeFast {
	pub value: u32,
}

#[repr(C)]
#[cfg_attr(feature = "wrappers", derive(Debug))]
#[allow(non_snake_case)]
pub struct FieldName {
	pub GoString: u32,
}

/// Different Rust integer errors reuse Python's IntException.
pub fn failing_integer() -> Result<(), u32> {
	Err(1)
}

impl Members {
	#[cfg(feature = "added")]
	pub fn failing_integer(&self) -> Result<(), u64> {
		Err(2)
	}
	#[cfg(feature = "added")]
	pub fn failing_path(&self) -> Result<(), std::path::PathBuf> {
		Err("error".into())
	}
}

#[cfg(feature = "closures")]
pub fn attempt(check: impl FnOnce() -> Result<(), String>) {
	let _ = check();
}

impl FieldName {
	pub fn debug_description(&self, meter: Option<&Meter>) -> u64 {
		meter.map_or(0, Meter::total)
	}
}

/// Claims Go's name for `Meter`'s `Default` constructor, since types claim theirs first.
#[cfg(feature = "defaults")]
pub struct MeterDefault {
	pub value: u32,
}
