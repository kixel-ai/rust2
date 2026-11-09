//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

mod elements;
#[path = "../../harness.rs"]
pub mod harness;
mod spare;
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
		("elements::vecs", |_, data, out| elements::vecs(data, out)),
		("elements::arrays", |_, data, out| elements::arrays(data, out)),
		("elements::segments", |_, data, out| elements::segments(data, out)),
		("elements::inline", |_, data, out| elements::inline(data, out)),
		("elements::edges", |_, data, out| elements::edges(data, out)),
		("elements::raw_slices", |_, data, out| elements::raw_slices(data, out)),
		("elements::empty_parts", |_, data, out| elements::empty_parts(data, out)),
		("spare::spares", spare::spares),
	]
}
