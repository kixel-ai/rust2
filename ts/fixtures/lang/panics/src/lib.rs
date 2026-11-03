//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod located;
mod panics;
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
		("panics::wrapped", panics::wrapped),
		("panics::called", panics::called),
		("panics::awaited", panics::awaited),
		("panics::raised", panics::raised),
		("located::called", located::called),
		("located::read", located::read),
		("located::panicked", located::panicked),
	]
}
