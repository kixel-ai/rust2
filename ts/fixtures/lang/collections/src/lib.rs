//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

mod collections;
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
		("collections::hash_maps", collections::hash_maps),
		("collections::sets", collections::sets),
		("collections::btree_maps", collections::btree_maps),
		("collections::deques", |_, data, out| collections::deques(data, out)),
		("collections::vecs", |_, data, out| collections::vecs(data, out)),
		("collections::stores", |_, data, out| collections::stores(data, out)),
		("collections::optionals", collections::optionals),
		("collections::set_ops", collections::set_ops),
		("collections::unlike_eqs", collections::unlike_eqs),
		("collections::own_nes", collections::own_nes),
		("collections::cell_eqs", collections::cell_eqs),
		("collections::unlike_compares", collections::unlike_compares),
		("collections::ref_orders", collections::ref_orders),
		("collections::deque_ranges", collections::deque_ranges),
		("collections::extracts", collections::extracts),
		("collections::btree_extracts", collections::btree_extracts),
		("collections::btree_searches", collections::btree_searches),
		("collections::btree_borrows", collections::btree_borrows),
		("collections::btree_drops", collections::btree_drops),
	]
}
