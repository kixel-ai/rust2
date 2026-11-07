//! Imports of the fixture's own, of each kind and shape wasm-bindgen declares, against
//! JS's globals and functions the harness defines on `globalThis` first.

use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	// A JS number into each Rust number type, which the WebAssembly ABI coerces: an `f64`
	// past `u32` wraps, and NaN is zero.
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_u8(x: f64) -> u8;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_i8(x: f64) -> i8;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_u16(x: f64) -> u16;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_i16(x: f64) -> i16;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_u32(x: f64) -> u32;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_i32(x: f64) -> i32;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_usize(x: f64) -> usize;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_f32(x: f64) -> f32;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_bool(x: f64) -> bool;
	#[wasm_bindgen(js_namespace = Math, js_name = round)]
	pub fn round_option(x: Option<f64>) -> Option<u32>;

	// 64- and 128-bit integers, as BigInts both ways.
	#[wasm_bindgen(js_namespace = BigInt, js_name = asIntN)]
	pub fn as_int_n(bits: u32, v: i64) -> i64;
	#[wasm_bindgen(js_namespace = BigInt, js_name = asIntN)]
	pub fn as_int_n_signed(bits: i32, v: i64) -> i64;
	#[wasm_bindgen(js_namespace = BigInt, js_name = asUintN)]
	pub fn as_uint_n(bits: u32, v: u64) -> u64;
	#[wasm_bindgen(js_namespace = BigInt, js_name = asIntN)]
	pub fn as_int_n128(bits: u32, v: i128) -> i128;
	#[wasm_bindgen(js_namespace = BigInt, js_name = asUintN)]
	pub fn as_uint_n128(bits: u32, v: u128) -> u128;

	// Text: a `char`, a `&str`, and a `String` to JS and back.
	#[wasm_bindgen(js_name = String)]
	pub fn char_string(c: char) -> String;
	#[wasm_bindgen(js_name = String)]
	pub fn first_char(s: &str) -> char;
	#[wasm_bindgen(js_name = String)]
	pub fn string_option(s: Option<String>) -> Option<String>;
	#[wasm_bindgen(js_name = String)]
	pub fn js_string(v: &JsValue) -> JsString;

	// Slices and vectors: bytes, numbers, and JS values.
	#[wasm_bindgen(js_namespace = Array, js_name = from)]
	pub fn bytes_array(bytes: &[u8]) -> Array;
	#[wasm_bindgen(js_namespace = Array, js_name = from)]
	pub fn floats_array(floats: &[f64]) -> Vec<f64>;
	#[wasm_bindgen(js_namespace = Array, js_name = from)]
	pub fn wides_array(wides: &[u64]) -> Vec<JsValue>;
	#[wasm_bindgen(js_namespace = Array, js_name = from)]
	pub fn values_array(values: Vec<JsValue>) -> Vec<JsValue>;
	#[wasm_bindgen(js_namespace = Array, js_name = from)]
	pub fn strings_array(values: Vec<String>) -> Vec<String>;
	#[wasm_bindgen(js_namespace = Uint8Array, js_name = from)]
	pub fn bytes_copy(bytes: Vec<u8>) -> Vec<u8>;
	#[wasm_bindgen(js_namespace = Uint8Array, js_name = from)]
	pub fn bytes_boxed(bytes: &[u8]) -> Box<[u8]>;
	#[wasm_bindgen(js_namespace = Object, js_name = keys)]
	pub fn keys(o: &JsValue) -> Vec<String>;

	// A variadic call, whose last argument JS spreads.
	#[wasm_bindgen(variadic, js_namespace = Math, js_name = max)]
	pub fn max_of(first: f64, rest: &[f64]) -> f64;
	#[wasm_bindgen(variadic, js_namespace = Array, js_name = of)]
	pub fn array_of(values: &Array) -> Array;

	// `catch`, into a `Result` of what JS threw.
	#[wasm_bindgen(catch, js_namespace = JSON, js_name = parse)]
	pub fn parse(text: &str) -> Result<JsValue, JsValue>;
	#[wasm_bindgen(catch, js_namespace = BigInt, js_name = asIntN)]
	pub fn as_int_n_number(bits: u32, v: f64) -> Result<i64, JsValue>;
	#[wasm_bindgen(catch, js_name = rustCheck)]
	pub fn check(v: &JsValue) -> Result<(), JsValue>;

	// Statics, of a namespace's value, an optional one, and a string literal.
	#[wasm_bindgen(thread_local_v2, js_namespace = Math, js_name = PI)]
	pub static PI: f64;
	#[wasm_bindgen(thread_local_v2, js_name = rustMissing)]
	pub static MISSING: Option<JsValue>;
	#[wasm_bindgen(thread_local_v2, js_name = globalThis)]
	pub static GLOBAL: Object;

	// A class of its own: a constructor, methods, getters and setters, structural and on
	// its prototype, indexing, and `instanceof`.
	#[wasm_bindgen(extends = Object, js_name = Map)]
	#[derive(Clone, Debug)]
	pub type Table;
	#[wasm_bindgen(constructor, js_class = Map)]
	pub fn new() -> Table;
	#[wasm_bindgen(method, js_class = Map)]
	pub fn set(this: &Table, key: &JsValue, value: &JsValue) -> Table;
	#[wasm_bindgen(method, js_class = Map, js_name = get)]
	pub fn get(this: &Table, key: &JsValue) -> Option<f64>;
	#[wasm_bindgen(method, final, js_class = Map, js_name = has)]
	pub fn has_final(this: &Table, key: &JsValue) -> bool;
	#[wasm_bindgen(method, getter, js_class = Map)]
	pub fn size(this: &Table) -> u32;
	#[wasm_bindgen(method, getter, final, js_class = Map, js_name = size)]
	pub fn size_final(this: &Table) -> u32;
	#[wasm_bindgen(method, structural, getter = size)]
	pub fn size_structural(this: &Table) -> u32;

	#[wasm_bindgen(extends = Object)]
	#[derive(Clone, Debug)]
	pub type Point;
	#[wasm_bindgen(method, structural, getter)]
	pub fn x(this: &Point) -> f64;
	#[wasm_bindgen(method, structural, setter)]
	pub fn set_x(this: &Point, x: f64);
	#[wasm_bindgen(method, structural, indexing_getter)]
	pub fn at(this: &Point, key: &str) -> JsValue;
	#[wasm_bindgen(method, structural, indexing_setter)]
	pub fn put(this: &Point, key: &str, value: &JsValue);
	#[wasm_bindgen(method, structural, indexing_deleter)]
	pub fn remove(this: &Point, key: &str);
	#[wasm_bindgen(method, structural, js_name = toString)]
	pub fn describe(this: &Point) -> String;

	// A class no global names, which no value is an instance of.
	#[wasm_bindgen(extends = Object, js_name = RustNoSuchClass)]
	pub type Missing;

	// A class by its vendor-prefixed name, where the unprefixed one is not defined.
	#[wasm_bindgen(extends = Object, vendor_prefix = webkit, js_name = RustVendored)]
	pub type Vendored;
	#[wasm_bindgen(constructor, js_class = RustVendored)]
	pub fn new() -> Vendored;
	#[wasm_bindgen(method, js_class = RustVendored)]
	pub fn name(this: &Vendored) -> String;

	// A static method of a class, and one named by a symbol.
	#[wasm_bindgen(static_method_of = Table, js_class = Map, js_name = groupBy)]
	pub fn group_by(items: &Array, key: &js_sys::Function) -> Table;
	#[wasm_bindgen(method, js_class = Map, js_name = "[Symbol.iterator]")]
	pub fn entries(this: &Table) -> js_sys::Iterator;

	// A string enum, both ways.
	#[wasm_bindgen(js_name = rustTypeof)]
	pub fn type_of(v: &JsValue) -> Kind;
	#[wasm_bindgen(js_name = String)]
	pub fn kind_string(k: Kind) -> String;
}

/// What `typeof` says, of the kinds the harness reads.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
	Number = "number",
	String = "string",
	Object = "object",
	Undefined = "undefined",
}

// A string literal as a JS string, which rustfmt would drop, as plain Rust gives no
// `extern` static a value.
#[rustfmt::skip]
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(thread_local_v2, static_string)]
	pub static GREETING: JsString = "hé\u{1f600}";
}

#[wasm_bindgen]
extern "C" {
	// A namespace's variadic function, called on its namespace, and one spreading what has
	// no iterator.
	#[wasm_bindgen(variadic, js_namespace = Uint8Array, js_name = of)]
	pub fn bytes_of(bytes: &[u8]) -> Vec<u8>;
	#[wasm_bindgen(catch, variadic, js_namespace = Array, js_name = of)]
	pub fn array_of_caught(values: &Array) -> Result<Array, JsValue>;
	// A namespace a getter makes anew each time it is read, read once a call.
	#[wasm_bindgen(variadic, js_namespace = rustFresh, js_name = of)]
	pub fn fresh_of(values: &[u32]) -> u32;

	// What converting a `catch` import's result throws: caught where the JS glue converts,
	// as for a `char`, and thrown where the WebAssembly ABI coerces, as for a `u32`.
	#[wasm_bindgen(catch, js_name = String)]
	pub fn char_caught(v: &JsValue) -> Result<char, JsValue>;
	#[wasm_bindgen(catch, js_name = rustIdentity)]
	pub fn u32_caught(v: &JsValue) -> Result<u32, JsValue>;
	#[wasm_bindgen(js_name = rustIdentity)]
	pub fn u32_of(v: &JsValue) -> u32;
	// An optional small integer, which crosses the ABI's `i32` as it is, as a `u32` does.
	#[wasm_bindgen(js_name = rustIdentity)]
	pub fn u8_option(v: &JsValue) -> Option<u8>;
	#[wasm_bindgen(catch, js_name = rustIdentity)]
	pub fn i16_option_caught(v: &JsValue) -> Result<Option<i16>, JsValue>;

	// An optional `bool`, which the JS glue reads by its truthiness.
	#[wasm_bindgen(js_namespace = Math, js_name = abs)]
	pub fn abs_flag(x: Option<f64>) -> Option<bool>;

	// Globals no JS defines: each throws a `ReferenceError`, as naming it does.
	#[wasm_bindgen(catch, js_name = rustNoSuchFunction)]
	pub fn missing_function() -> Result<(), JsValue>;
	#[wasm_bindgen(catch, constructor, js_class = RustNoSuchClass)]
	pub fn new() -> Result<Missing, JsValue>;
}
