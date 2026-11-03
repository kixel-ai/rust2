//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#![feature(hasher_prefixfree_extras)]

#[path = "../../harness.rs"]
pub mod harness;
mod hashes;
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
		("hashes::values", hashes::values),
		("hashes::random", |_, data, out| hashes::random(data, out)),
		("hashes::bytes", |_, data, out| hashes::bytes(data, out)),
		("hashes::own", |_, data, out| hashes::own(data, out)),
		("hashes::copied", |_, data, out| hashes::copied(data, out)),
		("hashes::maps", |_, data, out| hashes::maps(data, out)),
		("hashes::equal", |_, data, out| hashes::equal(data, out)),
		("hashes::defaults", |_, data, out| hashes::defaults(data, out)),
	]
}
