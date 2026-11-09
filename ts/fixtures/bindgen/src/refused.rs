//! What translation refuses of wasm-bindgen, each at its span: imports of custom JS by
//! each kind of module, one that suspends on a promise, one generic per monomorphization,
//! a closure, an optional mutable slice, the intrinsics only WASM has, and an export.

// `suspending` warns that it is experimental.
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

	#[wasm_bindgen(js_name = setTimeout)]
	fn set_timeout(f: &Closure<dyn FnMut()>, ms: u32);
}

/// An export, which JS calls into WASM by.
#[wasm_bindgen]
pub fn exported(x: u32) -> u32 {
	x + 1
}

pub fn run() -> u32 {
	let mut units = [0u16; 2];
	fill(Some(&mut units));
	let closure = Closure::new(|| {});
	set_timeout(&closure, 0);
	let memory = wasm_bindgen::memory();
	let suspended = suspended(&memory);
	let generic = generic(&suspended);
	custom() + raw() + inline() + exported(u32::from(generic.is_null())) + u32::from(units[0])
}
