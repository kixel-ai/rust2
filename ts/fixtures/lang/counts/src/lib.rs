//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

mod counts;
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
		("counts::arcs", |_, data, out| counts::arcs(data, out)),
		("counts::rcs", |_, data, out| counts::rcs(data, out)),
		("counts::cyclic_arcs", |_, data, out| counts::cyclic_arcs(data, out)),
		("counts::cyclic_rcs", |_, data, out| counts::cyclic_rcs(data, out)),
		("counts::unique_arcs", |_, data, out| counts::unique_arcs(data, out)),
		("counts::unique_rcs", |_, data, out| counts::unique_rcs(data, out)),
		("counts::cloned_arcs", |_, data, out| counts::cloned_arcs(data, out)),
		("counts::cloned_rcs", |_, data, out| counts::cloned_rcs(data, out)),
		("counts::copies", |_, data, out| counts::copies(data, out)),
		("counts::addressed_arcs", |_, data, out| {
			counts::addressed_arcs(data, out)
		}),
		("counts::addressed_rcs", |_, data, out| counts::addressed_rcs(data, out)),
		("counts::pointed_arcs", |_, data, out| counts::pointed_arcs(data, out)),
		("counts::pointed_rcs", |_, data, out| counts::pointed_rcs(data, out)),
	]
}
