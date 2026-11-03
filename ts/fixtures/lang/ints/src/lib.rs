//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#[path = "../../harness.rs"]
pub mod harness;
mod ints;
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
		("ints::u8_op", ints::u8_op),
		("ints::u16_op", ints::u16_op),
		("ints::u32_op", ints::u32_op),
		("ints::u64_op", ints::u64_op),
		("ints::u128_op", ints::u128_op),
		("ints::usize_op", ints::usize_op),
		("ints::i8_op", ints::i8_op),
		("ints::i16_op", ints::i16_op),
		("ints::i32_op", ints::i32_op),
		("ints::i64_op", ints::i64_op),
		("ints::i128_op", ints::i128_op),
		("ints::isize_op", ints::isize_op),
		("ints::neg", neg),
		("ints::cast", cast),
		("ints::convert", |_, data, out| ints::convert(data, out)),
		("ints::parses", ints::parses),
		("ints::widenings", ints::widenings),
	]
}

/// Negate the signed type `kind` picks.
fn neg(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 6 {
		0 => ints::i8_neg(data, out),
		1 => ints::i16_neg(data, out),
		2 => ints::i32_neg(data, out),
		3 => ints::i64_neg(data, out),
		4 => ints::i128_neg(data, out),
		_ => ints::isize_neg(data, out),
	}
}

/// Cast from the type `kind` picks to the one the next byte does, or without one, to
/// a type `kind` also picks.
fn cast(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match data.split_first() {
		Some((&to, data)) => ints::cast(kind, to, data, out),
		None => ints::cast(kind, kind / 12, data, out),
	}
}
