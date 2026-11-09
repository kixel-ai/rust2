//! What translation refuses of wasm-bindgen, each at its span: imports of custom JS by
//! each kind of module, two that suspend on a promise, one generic per monomorphization,
//! an optional mutable slice, the intrinsics only WASM has, and an export.

// `suspending` and `jspi_block_on_promise` warn that they are experimental.
#![expect(deprecated, reason = "a refusal of each kind of import, experimental ones too")]

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/custom.js")]
extern "C" {
	fn custom() -> u32;
}

#[wasm_bindgen(raw_module = "./raw.js")]
extern "C" {
	fn raw() -> u32;
}

#[wasm_bindgen(inline_js = "export function inline() { return 1; }")]
extern "C" {
	fn inline() -> u32;
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(suspending, js_namespace = Promise, js_name = resolve)]
	fn suspended(v: &JsValue) -> JsValue;

	#[wasm_bindgen(experimental_generic_mono, js_name = String)]
	fn generic<T: js_sys::JsGeneric>(v: &T) -> JsValue;

	#[wasm_bindgen(js_name = rustFill)]
	fn fill(v: Option<&mut [u16]>);
}

/// An export, which JS calls into WASM by.
#[wasm_bindgen]
pub fn exported(x: u32) -> u32 {
	x + 1
}

pub fn run() -> u32 {
	let mut units = [0u16; 2];
	fill(Some(&mut units));
	let memory = wasm_bindgen::memory();
	let suspended = suspended(&memory);
	let blocked = js_sys::futures::jspi_block_on_promise(&js_sys::Promise::resolve(&suspended));
	let generic = generic(&suspended);
	let blocked = u32::from(blocked.is_ok());
	custom() + raw() + inline() + exported(u32::from(generic.is_null())) + u32::from(units[0]) + blocked
}
