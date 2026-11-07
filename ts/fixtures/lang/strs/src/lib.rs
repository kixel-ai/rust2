//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod strs;
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
		("strs::strs", strs::strs),
		("strs::slices", strs::slices),
		("strs::edits", strs::edits),
		("strs::parses", strs::parses),
		("strs::iters", strs::iters),
		("strs::drains", strs::drains),
		("strs::hinted", strs::hinted),
		("strs::gets", strs::gets),
		("strs::unicode", strs::unicode),
		("strs::utf16", strs::utf16),
		("strs::pointers", strs::pointers),
	]
}
