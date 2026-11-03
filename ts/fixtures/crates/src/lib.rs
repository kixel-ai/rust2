//! A package that translates with its dependency, `crates-dep`, as one program.
//!
//! `harness::run` is the entry point both sides of the differential test run.

pub mod harness;
#[cfg(feature = "shared")]
mod crates_dep {}
#[cfg(feature = "alike")]
pub mod fmt {
	/// Spelled `fmt_Error` in a suffix, as std's `fmt::Error` is.
	pub struct Error;
}

/// An instance over the crate's `fmt::Error` and one over std's, which a suffix spells
/// alike: translation refuses to name them, rather than number one.
#[cfg(feature = "alike")]
pub fn alike() -> u8 {
	fn one<T>(_: T) -> u8 {
		1
	}
	one(fmt::Error) + one(std::fmt::Error)
}
/// Whether the dependency's `Bytes` of a `String`'s bytes is the one handle of its storage,
/// which the `String`'s capacity decides: translation refuses it, as the bytes cross crates.
pub fn unique(text: String) -> bool {
	::crates_dep::string_bytes(text).is_unique()
}

#[cfg(target_arch = "wasm32")]
#[path = "../../wasm.rs"]
mod wasm;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;
