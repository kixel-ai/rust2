//! The one entry point both sides of the differential test run: native Rust built for
//! wasm32 and run through wasm-bindgen's CLI, and its TypeScript translation. The first
//! input byte picks what to exercise and the rest is its input; the output records each
//! result as a line of text, so the two sides agree only if each step does.

use std::fmt::Write;
use std::sync::atomic::{AtomicU32, Ordering};

use js_sys::{
	Array, BigInt64Array, Float64Array, Function, Int16Array, JSON, JsString, Math, Object, Reflect, Uint8Array,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{TextDecoder, TextDecoderOptions, TextEncoder, console};

use crate::imports::{self, Point, Table};

/// What a case runs on the rest of the input, writing each result to `out`.
type Case = fn(&mut Input, &mut String);

/// Each case's name and what it runs. The first byte picks one modulo their count, so a
/// case added at the end moves no other.
pub fn cases() -> Vec<(&'static str, Case)> {
	vec![
		("math", math),
		("numbers", numbers),
		("bigints", bigints),
		("text", text),
		("encoding", encoding),
		("json", json),
		("arrays", arrays),
		("typed", typed),
		("casts", casts),
		("ops", ops),
		("classes", classes),
		("statics", statics),
		("console", logs),
		("caught", caught),
		("throws", throws),
		("edges", edges),
	]
}

/// Run one case.
pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = String::new();
	let Some((&op, data)) = input.split_first() else {
		return Vec::new();
	};
	define();
	let cases = cases();
	let (_, case) = cases[usize::from(op) % cases.len()];
	case(&mut Input(data), &mut out);
	out.into_bytes()
}

/// The functions on `globalThis` that the fixture's own imports call.
fn define() {
	let global = imports::GLOBAL.with(Object::clone);
	let typeof_ = Function::new_with_args("v", "return typeof v");
	let identity = Function::new_with_args("v", "return v");
	let check = Function::new_with_args(
		"v",
		"if (typeof v !== 'number') throw new TypeError('not a number: ' + String(v))",
	);
	Reflect::set(&global, &"rustTypeof".into(), &typeof_).unwrap();
	Reflect::set(&global, &"rustCheck".into(), &check).unwrap();
	Reflect::set(&global, &"rustIdentity".into(), &identity).unwrap();
}

/// The rest of a case's input.
pub struct Input<'a>(&'a [u8]);

/// Doubles at each edge an import's coercion has.
const EDGES: [f64; 18] = [
	0.0,
	-0.0,
	1.5,
	-2.5,
	f64::NAN,
	f64::INFINITY,
	f64::NEG_INFINITY,
	4_294_967_301.0,
	2_147_483_648.0,
	-2_147_483_649.0,
	9_007_199_254_740_994.0,
	1e300,
	5e-324,
	255.5,
	-1.0,
	65_535.5,
	-128.5,
	18_446_744_073_709_551_616.0,
];

impl Input<'_> {
	fn u8(&mut self) -> u8 {
		match self.0.split_first() {
			Some((&b, rest)) => {
				self.0 = rest;
				b
			}
			None => 0,
		}
	}

	/// Whether the next byte's low bit is clear.
	fn flag(&mut self) -> bool {
		self.u8() & 1 == 0
	}

	fn bytes<const N: usize>(&mut self) -> [u8; N] {
		let mut buf = [0u8; N];
		for b in &mut buf {
			*b = self.u8();
		}
		buf
	}

	fn u32(&mut self) -> u32 {
		u32::from_le_bytes(self.bytes())
	}

	fn u64(&mut self) -> u64 {
		u64::from_le_bytes(self.bytes())
	}

	fn u128(&mut self) -> u128 {
		u128::from_le_bytes(self.bytes())
	}

	/// A double: an edge, or one of any bits.
	fn f64(&mut self) -> f64 {
		let pick = self.u8();
		match EDGES.get(usize::from(pick % 32)) {
			Some(&edge) => edge,
			None => f64::from_bits(self.u64()),
		}
	}

	/// Up to 15 bytes.
	fn slice(&mut self) -> &[u8] {
		let len = usize::from(self.u8() % 16).min(self.0.len());
		let (taken, rest) = self.0.split_at(len);
		self.0 = rest;
		taken
	}

	/// Text from up to 15 bytes, each malformed sequence as U+FFFD.
	fn text(&mut self) -> String {
		String::from_utf8_lossy(self.slice()).into_owned()
	}

	/// A JS value of each kind the cases read.
	fn value(&mut self) -> JsValue {
		match self.u8() % 12 {
			0 => JsValue::UNDEFINED,
			1 => JsValue::NULL,
			2 => JsValue::TRUE,
			3 => JsValue::FALSE,
			4 => JsValue::from_f64(self.f64()),
			5 => JsValue::from_str(&self.text()),
			6 => JsValue::from(self.u64() as i64),
			7 => imports::bytes_array(self.slice()).into(),
			8 => Uint8Array::from(self.slice()).into(),
			9 => Object::new().into(),
			10 => JsValue::from(self.u32() % 1000),
			_ => JsString::from_char_code(&self.units()).into(),
		}
	}

	/// UTF-16 code units, lone surrogates among them.
	fn units(&mut self) -> Vec<u16> {
		let len = self.u8() % 6;
		(0..len)
			.map(|_| match self.u8() % 4 {
				0 => 0xd800 | u16::from(self.u8()),
				1 => 0xdc00 | u16::from(self.u8()),
				_ => u16::from(self.u8()) | 0x40,
			})
			.collect()
	}
}

/// A JS value's text, as `String()` gives it, in a line.
fn show(out: &mut String, what: &str, v: &JsValue) {
	let text = String::from(imports::js_string(v));
	writeln!(out, "{what}: {text:?}").unwrap();
}

/// A `Math` result's bits, any NaN as `f64::NAN`'s: ECMAScript leaves which NaN a
/// function returns to the engine, and V8's varies by CPU and call path, both sides'
/// calls being the same.
fn op_bits(x: f64) -> u64 {
	if x.is_nan() { f64::NAN } else { x }.to_bits()
}

fn math(input: &mut Input, out: &mut String) {
	let (x, y) = (input.f64(), input.f64());
	writeln!(out, "floor {:?}", op_bits(Math::floor(x))).unwrap();
	writeln!(out, "trunc {:?}", op_bits(Math::trunc(x))).unwrap();
	writeln!(out, "sign {:?}", op_bits(Math::sign(x))).unwrap();
	let fround = Math::fround(x);
	let fround = if fround.is_nan() { f32::NAN } else { fround };
	writeln!(out, "fround {:?}", fround.to_bits()).unwrap();
	writeln!(out, "hypot {:?}", op_bits(Math::hypot(x, y))).unwrap();
	writeln!(out, "atan2 {:?}", op_bits(Math::atan2(x, y))).unwrap();
	writeln!(out, "pow {:?}", op_bits(Math::pow(x, y))).unwrap();
	writeln!(out, "max {:?}", op_bits(Math::max(x, y))).unwrap();
	writeln!(out, "clz32 {}", Math::clz32(input.u32() as i32)).unwrap();
	let rest = [y, input.f64(), input.f64()];
	writeln!(
		out,
		"max_of {:?}",
		op_bits(imports::max_of(x, &rest[..usize::from(input.u8() % 4)]))
	)
	.unwrap();
}

fn numbers(input: &mut Input, out: &mut String) {
	let x = input.f64();
	writeln!(out, "u8 {}", imports::round_u8(x)).unwrap();
	writeln!(out, "i8 {}", imports::round_i8(x)).unwrap();
	writeln!(out, "u16 {}", imports::round_u16(x)).unwrap();
	writeln!(out, "i16 {}", imports::round_i16(x)).unwrap();
	writeln!(out, "u32 {}", imports::round_u32(x)).unwrap();
	writeln!(out, "i32 {}", imports::round_i32(x)).unwrap();
	writeln!(out, "usize {}", imports::round_usize(x)).unwrap();
	writeln!(out, "f32 {}", imports::round_f32(x).to_bits()).unwrap();
	writeln!(out, "bool {}", imports::round_bool(x)).unwrap();
	let maybe = (input.flag()).then_some(x);
	writeln!(out, "option {:?}", imports::round_option(maybe)).unwrap();
	let n = JsValue::from_f64(x);
	writeln!(
		out,
		"integer {} safe {}",
		js_sys::Number::is_integer(&n),
		js_sys::Number::is_safe_integer(&n)
	)
	.unwrap();
	match js_sys::Number::from(x).to_string_with_radix(input.u8() % 40) {
		Ok(s) => writeln!(out, "radix {:?}", String::from(s)).unwrap(),
		Err(e) => show(out, "radix threw", &e),
	}
	show(out, "string", &JsValue::from_f64(x));
}

fn bigints(input: &mut Input, out: &mut String) {
	let bits = u32::from(input.u8() % 140);
	let (a, b) = (input.u64(), input.u128());
	writeln!(out, "as_int_n {}", imports::as_int_n(bits, a as i64)).unwrap();
	writeln!(out, "as_uint_n {}", imports::as_uint_n(bits, a)).unwrap();
	writeln!(out, "as_int_n128 {}", imports::as_int_n128(bits, b as i128)).unwrap();
	writeln!(out, "as_uint_n128 {}", imports::as_uint_n128(bits, b)).unwrap();
	match imports::as_int_n_number(bits, input.f64()) {
		Ok(v) => writeln!(out, "from number {v}").unwrap(),
		Err(e) => show(out, "from number threw", &e),
	}
	show(out, "i64", &JsValue::from(a as i64));
	show(out, "u128", &JsValue::from(b));
	show(out, "i128", &JsValue::from(b as i128));
}

fn text(input: &mut Input, out: &mut String) {
	let c = char::from_u32(input.u32() % 0x11_0000).unwrap_or('\u{fffd}');
	writeln!(out, "char_string {:?}", imports::char_string(c)).unwrap();
	let s = input.text();
	writeln!(out, "first_char {:?}", imports::first_char(&s)).unwrap();
	let maybe = (input.flag()).then(|| s.clone());
	writeln!(out, "string_option {:?}", imports::string_option(maybe)).unwrap();
	let units = input.units();
	let js = JsString::from_char_code(&units);
	writeln!(out, "units {units:?} length {}", js.length()).unwrap();
	writeln!(out, "well formed {}", js.is_valid_utf16()).unwrap();
	writeln!(out, "lossy {:?}", String::from(&js)).unwrap();
	writeln!(out, "as_string {:?}", JsValue::from(&js).as_string()).unwrap();
	writeln!(out, "upper {:?}", String::from(js.to_upper_case())).unwrap();
	writeln!(out, "char_code_at {:?}", js.char_code_at(0).to_bits()).unwrap();
	writeln!(out, "code_point_at {:?}", js.code_point_at(0).as_f64()).unwrap();
	match JsString::from_code_point1(input.u32() % 0x12_0000) {
		Ok(s) => writeln!(out, "from_code_point {:?}", String::from(s)).unwrap(),
		Err(e) => show(out, "from_code_point threw", &e),
	}
	writeln!(out, "greeting {:?}", imports::GREETING.with(|s| String::from(s))).unwrap();
	writeln!(out, "includes {}", js.includes(&s, 0)).unwrap();
}

fn encoding(input: &mut Input, out: &mut String) {
	let encoder = TextEncoder::new().unwrap();
	let s = input.text();
	writeln!(out, "encoding {}", encoder.encoding()).unwrap();
	writeln!(out, "encode {:?}", encoder.encode_with_input(&s)).unwrap();
	let labels = ["utf-8", "UTF8", "latin1", "utf-16le", "no such label"];
	let label = labels[usize::from(input.u8()) % labels.len()];
	let options = TextDecoderOptions::new();
	options.set_fatal(input.flag());
	writeln!(out, "fatal {:?}", options.get_fatal()).unwrap();
	match TextDecoder::new_with_label_and_options(label, &options) {
		Ok(decoder) => {
			writeln!(out, "decoder {} fatal {}", decoder.encoding(), decoder.fatal()).unwrap();
			match decoder.decode_with_u8_array(input.slice()) {
				Ok(s) => writeln!(out, "decode {s:?}").unwrap(),
				Err(e) => show(out, "decode threw", &e),
			}
		}
		Err(e) => show(out, "decoder threw", &e),
	}
}

fn json(input: &mut Input, out: &mut String) {
	let texts = [
		"{\"a\":1,\"b\":[true,null]}",
		"[1,2",
		"\"\\ud800\"",
		"1e400",
		"{\"x\":{\"y\":\"z\"}}",
	];
	let text = match input.u8() % 6 {
		5 => input.text(),
		i => texts[usize::from(i)].to_string(),
	};
	match imports::parse(&text) {
		Ok(v) => {
			match JSON::stringify(&v) {
				// `undefined` where JSON has no text for it, which no `String` is.
				Ok(s) => writeln!(out, "stringify {:?}", s.as_string()).unwrap(),
				Err(e) => show(out, "stringify threw", &e),
			}
			if v.is_object() {
				writeln!(out, "keys {:?}", imports::keys(&v)).unwrap();
				let keys = Reflect::own_keys(&v).unwrap();
				writeln!(out, "own_keys {}", keys.length()).unwrap();
				show(out, "get a", &Reflect::get(&v, &"a".into()).unwrap());
				writeln!(out, "has x {:?}", Reflect::has(&v, &"x".into())).unwrap();
			}
		}
		Err(e) => show(out, "parse threw", &e),
	}
	match JSON::stringify(&input.value()) {
		Ok(s) => writeln!(out, "value {:?}", s.as_string()).unwrap(),
		Err(e) => show(out, "value threw", &e),
	}
}

fn arrays(input: &mut Input, out: &mut String) {
	let array = Array::new();
	for _ in 0..input.u8() % 5 {
		array.push(&input.value());
	}
	writeln!(out, "length {}", array.length()).unwrap();
	writeln!(out, "join {:?}", String::from(array.join(","))).unwrap();
	let probe = input.value();
	writeln!(
		out,
		"index_of {} includes {}",
		array.index_of(&probe, 0),
		array.includes(&probe, 0)
	)
	.unwrap();
	writeln!(out, "is_array {} {}", Array::is_array(&array), Array::is_array(&probe)).unwrap();
	show(out, "pop", &array.pop());
	show(out, "at", &array.at(-1));
	show(out, "get", &array.get(input.u32() % 8));
	array.set(input.u32() % 6, probe.clone());
	show(out, "reversed", &array.reverse());
	show(out, "sorted", &array.sort());
	show(out, "of2", &Array::of2(&probe, &array));
	show(out, "array_of", &imports::array_of(&array));
	let values: Vec<JsValue> = array.iter().collect();
	writeln!(out, "values {}", imports::values_array(values).len()).unwrap();
	let strings = vec![input.text(), input.text()];
	writeln!(out, "strings {:?}", imports::strings_array(strings)).unwrap();
	let floats = [input.f64(), input.f64()];
	let back: Vec<u64> = imports::floats_array(&floats).iter().map(|f| f.to_bits()).collect();
	writeln!(out, "floats {back:?}").unwrap();
	let wides = [input.u64(), u64::MAX];
	for v in imports::wides_array(&wides) {
		show(out, "wide", &v);
	}
	show(out, "bytes", &imports::bytes_array(input.slice()).into());
}

fn typed(input: &mut Input, out: &mut String) {
	let bytes = input.slice().to_vec();
	let array = Uint8Array::from(&bytes[..]);
	writeln!(out, "length {}", array.length()).unwrap();
	let i = input.u32() % 20;
	writeln!(out, "get_index {i} {}", array.get_index(i)).unwrap();
	array.set_index(input.u32() % 20, input.u8());
	let sub = array.subarray(input.u32() % 8, input.u32() % 16);
	let mut copy = vec![0; sub.length() as usize];
	sub.copy_to(&mut copy);
	writeln!(out, "subarray {copy:?}").unwrap();
	array.fill(input.u8(), 1, 3);
	let mut whole = vec![0; array.length() as usize];
	array.copy_to(&mut whole);
	writeln!(out, "filled {whole:?}").unwrap();
	writeln!(out, "copy {:?}", imports::bytes_copy(bytes.clone())).unwrap();
	writeln!(out, "boxed {:?}", imports::bytes_boxed(&bytes)).unwrap();
	let floats = Float64Array::new_from_slice(&[input.f64(), input.f64()]);
	writeln!(
		out,
		"floats {:?} {:?}",
		floats.get_index(0).to_bits(),
		floats.get_index(5).to_bits()
	)
	.unwrap();
	let shorts = Int16Array::new(&floats);
	writeln!(out, "shorts {} {}", shorts.get_index(0), shorts.get_index(1)).unwrap();
	let wides = BigInt64Array::new_from_slice(&[input.u64() as i64, i64::MIN]);
	writeln!(out, "wides {} {}", wides.get_index(0), wides.get_index(1)).unwrap();
	show(out, "at", &array.at(-1).map_or(JsValue::NULL, JsValue::from));
}

fn casts(input: &mut Input, out: &mut String) {
	let v = input.value();
	writeln!(out, "debug {v:?}").unwrap();
	writeln!(
		out,
		"undefined {} null {} none {} object {} function {} string {} bigint {} symbol {} falsy {} truthy {}",
		v.is_undefined(),
		v.is_null(),
		v.is_null_or_undefined(),
		v.is_object(),
		v.is_function(),
		v.is_string(),
		v.is_bigint(),
		v.is_symbol(),
		v.is_falsy(),
		v.is_truthy()
	)
	.unwrap();
	writeln!(
		out,
		"as_f64 {:?} as_bool {:?} as_string {:?}",
		v.as_f64().map(f64::to_bits),
		v.as_bool(),
		v.as_string()
	)
	.unwrap();
	show(out, "typeof", &v.js_typeof());
	let kind = imports::type_of(&v);
	writeln!(out, "kind {kind:?} {}", imports::kind_string(kind)).unwrap();
	writeln!(
		out,
		"array {}",
		v.dyn_ref::<Array>().map(Array::length).unwrap_or(u32::MAX)
	)
	.unwrap();
	writeln!(out, "bytes {:?}", v.dyn_ref::<Uint8Array>().map(Uint8Array::length)).unwrap();
	writeln!(out, "table {}", v.is_instance_of::<Table>()).unwrap();
	writeln!(out, "missing class {}", v.is_instance_of::<imports::Missing>()).unwrap();
	match v.clone().dyn_into::<JsString>() {
		Ok(s) => writeln!(out, "js string {}", s.length()).unwrap(),
		Err(back) => show(out, "not a js string", &back),
	}
	let obj: &Object = v.unchecked_ref();
	writeln!(out, "unchecked {}", obj.is_undefined()).unwrap();
	let other = input.value();
	writeln!(out, "loose_eq {} eq {}", v.loose_eq(&other), v == other).unwrap();
	writeln!(
		out,
		"constants {:?} {:?} {:?} {:?}",
		JsValue::NULL,
		JsValue::UNDEFINED,
		JsValue::TRUE,
		JsValue::FALSE
	)
	.unwrap();
	writeln!(out, "from bool {:?}", JsValue::from_bool(input.flag())).unwrap();
}

fn ops(input: &mut Input, out: &mut String) {
	let pick = |input: &mut Input| match input.u8() % 3 {
		0 => JsValue::from_f64(input.f64()),
		1 => JsValue::from(input.u64() as i64 % 1000),
		_ => JsValue::from(input.u32() % 50),
	};
	let (a, b) = (pick(input), pick(input));
	// Each operator throws mixing a BigInt with a number, which `checked_div` alone catches.
	if a.is_bigint() != b.is_bigint() {
		show(out, "checked_div", &a.checked_div(&b));
		writeln!(out, "mixed lt {} eq {}", a.lt(&b), a.loose_eq(&b)).unwrap();
		return;
	}
	show(out, "add", &(&a + &b));
	show(out, "sub", &(&a - &b));
	show(out, "mul", &(&a * &b));
	show(out, "checked_div", &a.checked_div(&b));
	show(out, "neg", &(-&a));
	show(out, "and", &(&a & &b));
	show(out, "or", &(&a | &b));
	show(out, "xor", &(&a ^ &b));
	show(out, "not", &JsValue::from_bool(!&a));
	show(out, "shl", &(&a << &b));
	show(out, "shr", &(&a >> &b));
	if !a.is_bigint() {
		show(out, "unsigned_shr", &JsValue::from(a.unsigned_shr(&b)));
		show(out, "pow", &a.pow(&b));
	}
	writeln!(out, "lt {} le {} gt {} ge {}", a.lt(&b), a.le(&b), a.gt(&b), a.ge(&b)).unwrap();
}

fn classes(input: &mut Input, out: &mut String) {
	let table = Table::new();
	for _ in 0..input.u8() % 4 {
		table.set(&input.value(), &JsValue::from_f64(input.f64()));
	}
	let key = input.value();
	table.set(&key, &JsValue::from_f64(input.f64()));
	writeln!(out, "get {:?}", table.get(&key).map(f64::to_bits)).unwrap();
	writeln!(out, "get other {:?}", table.get(&input.value()).map(f64::to_bits)).unwrap();
	writeln!(out, "has {}", table.has_final(&key)).unwrap();
	writeln!(
		out,
		"size {} {} {}",
		table.size(),
		table.size_final(),
		table.size_structural()
	)
	.unwrap();
	let iter = table.entries();
	while let Ok(next) = iter.next()
		&& !next.done()
	{
		show(out, "entry", &next.value());
	}
	let items = imports::bytes_array(input.slice());
	let parity = Function::new_with_args("x", "return x % 2");
	writeln!(out, "groups {}", Table::group_by(&items, &parity).size()).unwrap();
	let point: Point = imports::parse("{\"x\":1.5,\"y\":2}").unwrap().unchecked_into();
	writeln!(out, "x {:?}", point.x().to_bits()).unwrap();
	point.set_x(input.f64());
	writeln!(out, "x {:?}", point.x().to_bits()).unwrap();
	let key = input.text();
	point.put(&key, &input.value());
	show(out, "at", &point.at(&key));
	point.remove("y");
	show(out, "y", &point.at("y"));
	writeln!(out, "describe {:?}", point.describe()).unwrap();
	writeln!(out, "keys {:?}", imports::keys(&point)).unwrap();
}

fn statics(_: &mut Input, out: &mut String) {
	writeln!(out, "pi {:?}", imports::PI.with(|pi| pi.to_bits())).unwrap();
	writeln!(out, "missing {:?}", imports::MISSING.with(Option::is_some)).unwrap();
	writeln!(out, "greeting {:?}", imports::GREETING.with(|s| s.length())).unwrap();
	let global = imports::GLOBAL.with(Object::clone);
	writeln!(out, "global {}", Reflect::has(&global, &"Math".into()).unwrap()).unwrap();
	// A class only its vendor-prefixed name defines, which the caller defines.
	let vendored = imports::Vendored::new();
	writeln!(
		out,
		"vendored {:?} {}",
		vendored.name(),
		JsValue::from(&vendored).is_instance_of::<imports::Vendored>()
	)
	.unwrap();
}

fn logs(input: &mut Input, out: &mut String) {
	// What `console.log` logs, captured into an array in its place.
	let console = Reflect::get(&imports::GLOBAL.with(Object::clone), &"console".into()).unwrap();
	let original = Reflect::get(&console, &"log".into()).unwrap();
	let logged = Array::new();
	let capture = Function::new_with_args("...args", "this.push(args.map(String).join(' '))").bind0(&logged);
	Reflect::set(&console, &"log".into(), &capture).unwrap();
	console::log_0();
	console::log_1(&input.value());
	console::log_2(&input.value(), &JsValue::from_str(&input.text()));
	let many = Array::of3(&input.value(), &input.value(), &JsValue::from(input.u32()));
	console::log(&many);
	Reflect::set(&console, &"log".into(), &original).unwrap();
	for line in logged.iter() {
		show(out, "logged", &line);
	}
}

fn caught(input: &mut Input, out: &mut String) {
	let v = input.value();
	match imports::check(&v) {
		Ok(()) => writeln!(out, "a number").unwrap(),
		Err(e) => {
			let error = e.dyn_ref::<js_sys::Error>().map(|e| String::from(e.message()));
			writeln!(
				out,
				"threw {error:?} type error {}",
				e.is_instance_of::<js_sys::TypeError>()
			)
			.unwrap();
		}
	}
	match Reflect::get(&v, &"length".into()) {
		Ok(len) => show(out, "length", &len),
		Err(e) => show(out, "length threw", &e),
	}
	match Reflect::own_keys(&v) {
		Ok(keys) => writeln!(out, "own keys {}", keys.length()).unwrap(),
		Err(e) => show(out, "own keys threw", &e),
	}
}

/// An import that throws without `catch`: the throw unwinds Rust and leaves the call.
fn throws(input: &mut Input, out: &mut String) {
	let bits = i32::from(input.u8()) - 128;
	let v = imports::as_int_n_signed(bits, 1);
	writeln!(out, "as_int_n {v}").unwrap();
}

/// A struct of the crate's own over a JS value, laid out as an imported type is, which
/// stays a struct: its `Drop` runs.
#[repr(transparent)]
struct Owned {
	obj: JsValue,
}

/// How many `Owned`s have dropped.
static DROPS: AtomicU32 = AtomicU32::new(0);

impl Drop for Owned {
	fn drop(&mut self) {
		DROPS.fetch_add(1, Ordering::Relaxed);
	}
}

/// Whether `e` is an `Error` of the class `class` names, as each side's engine words its
/// message.
fn error_class(out: &mut String, what: &str, e: &JsValue) {
	let name = Reflect::get(e, &"name".into()).unwrap_or_default();
	show(out, what, &name);
}

/// Each conversion's edge: namespaces' variadic calls, `catch` around the glue's
/// conversions and not the ABI's, optional `bool`s, globals no JS defines, and a struct of
/// the crate's own over a JS value.
fn edges(input: &mut Input, out: &mut String) {
	writeln!(out, "bytes of {:?}", imports::bytes_of(input.slice())).unwrap();
	let spread: Array = match input.flag() {
		true => imports::bytes_array(input.slice()),
		false => {
			// An array-like without an iterator.
			let like = Object::new();
			Reflect::set(&like, &"length".into(), &JsValue::from(1)).unwrap();
			Reflect::set(&like, &"0".into(), &input.value()).unwrap();
			like.unchecked_into()
		}
	};
	match imports::array_of_caught(&spread) {
		Ok(array) => writeln!(out, "spread {}", array.length()).unwrap(),
		Err(e) => error_class(out, "spread threw", &e),
	}
	let counts = [input.u32(), input.u32()];
	let fresh = imports::fresh_of(&counts[..usize::from(input.u8() % 3)]);
	writeln!(out, "fresh of {fresh}").unwrap();
	// The `None` an `Option<u8>` takes `0xFFFFFF` for, beside an input's value.
	let sentinel = imports::u8_option(&JsValue::from(0x00ff_ffff));
	let small = imports::u8_option(&JsValue::from_f64(input.f64()));
	writeln!(out, "u8 option {sentinel:?} {small:?}").unwrap();
	let v = input.value();
	match imports::char_caught(&v) {
		Ok(c) => writeln!(out, "char {c:?}").unwrap(),
		Err(e) => error_class(out, "char threw", &e),
	}
	let x = match input.flag() {
		true => Some(input.f64()),
		false => None,
	};
	writeln!(out, "abs flag {:?}", imports::abs_flag(x)).unwrap();
	match imports::missing_function() {
		Ok(()) => writeln!(out, "missing function called").unwrap(),
		Err(e) => error_class(out, "missing function threw", &e),
	}
	match imports::Missing::new() {
		Ok(_) => writeln!(out, "missing class made").unwrap(),
		Err(e) => error_class(out, "missing class threw", &e),
	}
	let before = DROPS.load(Ordering::Relaxed);
	{
		let owned = Owned { obj: input.value() };
		show(out, "owned", &owned.obj);
	}
	writeln!(out, "drops {}", DROPS.load(Ordering::Relaxed) - before).unwrap();
	// Last, as each may throw past the case: `catch` imports whose numbers the ABI coerces,
	// and a plain one.
	let v = input.value();
	match imports::u32_caught(&v) {
		Ok(n) => writeln!(out, "u32 {n}").unwrap(),
		Err(e) => error_class(out, "u32 threw", &e),
	}
	match imports::i16_option_caught(&input.value()) {
		Ok(n) => writeln!(out, "i16 option {n:?}").unwrap(),
		Err(e) => error_class(out, "i16 option threw", &e),
	}
	writeln!(out, "u32 of {}", imports::u32_of(&input.value())).unwrap();
}
