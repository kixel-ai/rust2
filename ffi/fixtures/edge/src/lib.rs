//! A small API of shapes whose Dart leaves part of the generator's output unused, each of which
//! Dart's analyzer would report once nothing refers to it.

/// An array taken and never returned, so Dart never reads one.
pub fn sum(bytes: [u8; 32]) -> u32 {
	bytes.iter().map(|b| u32::from(*b)).sum()
}

/// An array returned and never taken, so Dart never writes one.
pub fn zeros() -> [u8; 4] {
	[0; 4]
}

/// A record only on one target, whose only items are the checks of its derived `Debug`: the
/// target check is no function's.
#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
pub struct Wasm {
	pub n: u32,
}
