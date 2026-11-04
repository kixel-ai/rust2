//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod saturates;
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
		("saturates::u8s", saturates::u8s),
		("saturates::u16s", saturates::u16s),
		("saturates::u32s", saturates::u32s),
		("saturates::u64s", saturates::u64s),
		("saturates::u128s", saturates::u128s),
		("saturates::usizes", saturates::usizes),
		("saturates::i8s", saturates::i8s),
		("saturates::i16s", saturates::i16s),
		("saturates::i32s", saturates::i32s),
		("saturates::i64s", saturates::i64s),
		("saturates::i128s", saturates::i128s),
		("saturates::isizes", saturates::isizes),
	]
}
