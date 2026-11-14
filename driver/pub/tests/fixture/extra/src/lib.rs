//! A library app uses, which facade exposes whole with `pub extern crate`.

/// Kept: app calls it.
pub fn used() -> u32 {
	7
}

/// External: facade exposes this crate, so crates outside the audit may call it.
pub fn exposed() -> u32 {
	8
}
