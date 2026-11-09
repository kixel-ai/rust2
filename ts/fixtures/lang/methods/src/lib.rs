//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#![feature(nonzero_ops, wrapping_int_impl, wrapping_next_power_of_two)]

#[path = "../../harness.rs"]
pub mod harness;
mod methods;
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
		("methods::u8s", methods::u8s),
		("methods::u16s", methods::u16s),
		("methods::u32s", methods::u32s),
		("methods::u64s", methods::u64s),
		("methods::u128s", methods::u128s),
		("methods::usizes", methods::usizes),
		("methods::i8s", methods::i8s),
		("methods::i16s", methods::i16s),
		("methods::i32s", methods::i32s),
		("methods::i64s", methods::i64s),
		("methods::i128s", methods::i128s),
		("methods::isizes", methods::isizes),
	]
}
