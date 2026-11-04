//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
#[cfg(target_arch = "wasm32")]
#[path = "../../../wasm.rs"]
mod wasm;
mod wraps;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;

use harness::Case;

/// Each case's name and what it runs. The first two bytes, big-endian, pick one modulo
/// their count, so a case added at the end moves no other.
fn cases() -> Vec<(&'static str, Case)> {
	vec![
		("wraps::u8s", wraps::u8s),
		("wraps::u16s", wraps::u16s),
		("wraps::u32s", wraps::u32s),
		("wraps::u64s", wraps::u64s),
		("wraps::u128s", wraps::u128s),
		("wraps::usizes", wraps::usizes),
		("wraps::i8s", wraps::i8s),
		("wraps::i16s", wraps::i16s),
		("wraps::i32s", wraps::i32s),
		("wraps::i64s", wraps::i64s),
		("wraps::i128s", wraps::i128s),
		("wraps::isizes", wraps::isizes),
	]
}
