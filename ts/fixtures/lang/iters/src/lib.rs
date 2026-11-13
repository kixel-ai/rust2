//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#![feature(iter_order_by)]

#[path = "../../harness.rs"]
pub mod harness;
mod iters;
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
		("iters::adaptors", iters::adaptors),
		("iters::consumers", iters::consumers),
		("iters::ranges", |_, data, out| iters::ranges(data, out)),
		("iters::options", |_, data, out| iters::options(data, out)),
		("iters::own", |_, data, out| iters::own(data, out)),
		("iters::values", |_, data, out| iters::values(data, out)),
		("iters::skips", iters::skips),
		("iters::ranked", iters::ranked),
		("iters::returned", |_, data, out| iters::returned(data, out)),
		("iters::reducers", |_, data, out| iters::reducers(data, out)),
		("iters::peeked_others", |_, data, out| iters::peeked_others(data, out)),
		("iters::zips", iters::zips),
		("iters::zips_back", iters::zips_back),
		("iters::range_froms", iters::range_froms),
		("iters::range_methods", iters::range_methods),
		("iters::compares", iters::compares),
		("iters::option_items", |_, data, out| iters::option_items(data, out)),
		("iters::lent", |_, data, out| iters::lent(data, out)),
		("iters::by_value", iters::by_value),
		("iters::held_fields", iters::held_fields),
		("iters::unfused", iters::unfused),
	]
}
