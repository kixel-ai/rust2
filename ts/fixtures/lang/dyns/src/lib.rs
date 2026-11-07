//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

mod dyns;
#[path = "../../harness.rs"]
pub mod harness;
mod ids;
mod stds;
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
		("dyns::dyns", |_, data, out| dyns::dyns(data, out)),
		("dyns::shared", |_, data, out| dyns::shared(data, out)),
		("dyns::anys", |_, data, out| dyns::anys(data, out)),
		("dyns::fns", |_, data, out| dyns::fns(data, out)),
		("dyns::pointers", |_, data, out| dyns::pointers(data, out)),
		("dyns::iters", |_, data, out| dyns::iters(data, out)),
		("dyns::boxes", |_, data, out| dyns::boxes(data, out)),
		("dyns::tails", |_, data, out| dyns::tails(data, out)),
		("dyns::rests", |_, data, out| dyns::rests(data, out)),
		("dyns::generics", |_, data, out| dyns::generics(data, out)),
		("stds::stds", |_, data, out| stds::stds(data, out)),
		("dyns::options", |_, data, out| dyns::options(data, out)),
		("ids::ids", |_, data, out| ids::ids(data, out)),
	]
}
