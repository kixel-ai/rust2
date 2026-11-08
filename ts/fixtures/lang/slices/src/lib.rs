//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod slices;
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
		("slices::parts", slices::parts),
		("slices::drains", slices::drains),
		("slices::chunked", slices::chunked),
		("slices::subslices", slices::subslices),
		("slices::ranges", slices::ranges),
		("slices::edits", slices::edits),
		("slices::arrays", slices::arrays),
		("slices::arrays_mut", slices::arrays_mut),
		("slices::array_forms", slices::array_forms),
		("slices::array_identity", slices::array_identity),
		("slices::array_sources", slices::array_sources),
		("slices::item_writes", slices::item_writes),
		("slices::array_parts", slices::array_parts),
	]
}
