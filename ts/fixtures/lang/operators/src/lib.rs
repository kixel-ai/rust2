//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod operators;
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
		("operators::u8s", operators::u8s),
		("operators::u16s", operators::u16s),
		("operators::u32s", operators::u32s),
		("operators::u64s", operators::u64s),
		("operators::u128s", operators::u128s),
		("operators::usizes", operators::usizes),
		("operators::i8s", operators::i8s),
		("operators::i16s", operators::i16s),
		("operators::i32s", operators::i32s),
		("operators::i64s", operators::i64s),
		("operators::i128s", operators::i128s),
		("operators::isizes", operators::isizes),
		("operators::f64s", operators::f64s),
		("operators::f32s", operators::f32s),
		("operators::bools", operators::bools),
	]
}
