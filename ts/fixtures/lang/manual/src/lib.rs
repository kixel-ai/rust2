//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod manual;
mod uninit;
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
		("manual::arcs", |_, data, out| manual::arcs(data, out)),
		("manual::needs_drops", |_, data, out| manual::needs_drops(data, out)),
		("uninit::uninits", |_, data, out| uninit::uninits(data, out)),
		("uninit::boxed_uninits", |_, data, out| uninit::boxed_uninits(data, out)),
		("uninit::boxed_one", |_, data, out| uninit::boxed_one(data, out)),
		("uninit::boxed_zeroed", |_, data, out| uninit::boxed_zeroed(data, out)),
	]
}
