//! web-codecs' `VideoEncoder` over web-sys, with the closures it hands JS and the promises
//! it awaits, which translation makes direct JS calls: the JS value is the value itself.
//!
//! `harness::run` is the entry point both sides of the differential test run in one
//! browser: natively through wasm-bindgen's CLI, and translated.

mod encoder;
pub mod harness;

/// The native side's export, which translation leaves out: it exports the crate's `pub`
/// API itself.
#[cfg(not(rust2_ts))]
#[wasm_bindgen::prelude::wasm_bindgen(js_name = run)]
pub fn exported() -> js_sys::Promise {
	harness::run()
}
