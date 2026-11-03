//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#![feature(cell_leak, refcell_try_map)]

mod borrows;
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
		("borrows::borrows", |_, data, out| borrows::borrows(data, out)),
		("borrows::guards", |_, data, out| borrows::guards(data, out)),
		("borrows::maps", |_, data, out| borrows::maps(data, out)),
		("borrows::tries", |_, data, out| borrows::tries(data, out)),
		("borrows::leaks", |_, data, out| borrows::leaks(data, out)),
	]
}
