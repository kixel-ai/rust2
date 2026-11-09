//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#![feature(mapped_lock_guards)]

#[path = "../../harness.rs"]
pub mod harness;
mod locks;
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
		("locks::locks", |_, data, out| locks::locks(data, out)),
		("locks::relock", |_, data, out| locks::relock(data, out)),
		("locks::rwlocks", |_, data, out| locks::rwlocks(data, out)),
		("locks::mapped", |_, data, out| locks::mapped(data, out)),
		("locks::mutex_mapped", |_, data, out| locks::mutex_mapped(data, out)),
		("locks::printed", |_, data, out| locks::printed(data, out)),
		("locks::downgraded", |_, data, out| locks::downgraded(data, out)),
		("locks::shown", |_, data, out| locks::shown(data, out)),
		("locks::guarded", |_, data, out| locks::guarded(data, out)),
	]
}
