//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod shares;
#[cfg(target_arch = "wasm32")]
#[path = "../../../wasm.rs"]
mod wasm;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;

use harness::Case;

/// Each case's name and what it runs. The first two bytes, big-endian, pick one modulo
/// their count, so a case added at the end moves no other.
fn cases() -> Vec<(&'static str, Case)> {
	vec![
		("shares::bytes", shares::bytes),
		("shares::wides", shares::wides),
		("shares::copies", shares::copies),
		("shares::drops", shares::drops),
		("shares::strs", shares::strs),
		("shares::generics", shares::generics),
		("shares::keys", shares::keys),
		("shares::addresses", shares::addresses),
		("shares::unsized_arrays", shares::unsized_arrays),
	]
}
