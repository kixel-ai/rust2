//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

mod conversions;
#[path = "../../harness.rs"]
pub mod harness;
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
		("conversions::conversions", conversions::conversions),
		("conversions::byte_places", conversions::byte_places),
	]
}
