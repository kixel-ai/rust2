//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

mod cows;
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
		("cows::cows", cows::cows),
		("cows::slices", cows::slices),
		("cows::bytes", cows::bytes),
		("cows::clone_froms", cows::clone_froms),
		("cows::variants", cows::variants),
	]
}
