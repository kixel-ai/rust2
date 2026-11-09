//! A part of the language the driver translates, exercised against native Rust: what
//! each of its modules says.

#![cfg_attr(feature = "hook", feature(nonzero_ops))]

#[path = "../../harness.rs"]
pub mod harness;
#[cfg(feature = "hook")]
mod hooked;
mod located;
mod panics;
#[cfg(target_arch = "wasm32")]
#[path = "../../../wasm.rs"]
mod wasm;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;

use std::cell::RefCell;

use harness::Case;

/// Each case's name and what it runs. The first two bytes, big-endian, pick one modulo
/// their count, so a case added at the end moves no other.
fn cases() -> Vec<(&'static str, Case)> {
	let cases: Vec<(&'static str, Case)> = vec![
		("panics::wrapped", panics::wrapped),
		("panics::called", panics::called),
		("panics::awaited", panics::awaited),
		("panics::raised", panics::raised),
		("located::called", located::called),
		("located::read", located::read),
		("located::panicked", located::panicked),
		("edition2018::panicked", edition2018::panicked),
	];
	#[cfg(feature = "hook")]
	let cases = (cases.into_iter())
		.chain([
			("hooked::panicked", hooked::panicked as Case),
			("hooked::aborted", hooked::aborted),
			("hooked::sited", hooked::sited),
			("hooked::stock", hooked::stock),
			("hooked::edition2018_panicked", hooked::edition2018_panicked),
			("hooked::tokio_stock", hooked::tokio_stock),
		])
		.collect();
	cases
}

thread_local! {
	/// What `hooked`'s hooks wrote, which the host reads after each case.
	static NOTE: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Take what the hooks wrote since the last call.
pub fn note() -> String {
	NOTE.with_borrow_mut(std::mem::take)
}

#[cfg(target_arch = "wasm32")]
static NOTED: std::sync::Mutex<String> = std::sync::Mutex::new(String::new());

/// Where what the hooks wrote is, as UTF-8, taken from them; its length is `note_len`,
/// which reads it after this.
#[cfg(target_arch = "wasm32")]
#[unsafe(export_name = "note")]
pub extern "C" fn noted() -> *const u8 {
	let mut noted = NOTED.lock().unwrap();
	*noted = note();
	noted.as_ptr()
}

/// The length of what `note` took.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn note_len() -> usize {
	NOTED.lock().unwrap().len()
}
