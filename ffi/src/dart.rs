//! Dart over the C ABI through `dart:ffi`: `dart/<prefix>.dart`, which binds each C function
//! with `@Native`, so it finds the library as the package's native asset, and the Rust the
//! crate's `dart` feature builds for what `dart:ffi` lacks.
//!
//! Each name is its C name without the prefix, as Dart spells it: `moq_track_producer` is
//! `TrackProducer`, and its method `moq_track_producer_append_group` is
//! `TrackProducer.appendGroup`. A reserved word takes a trailing `_`. A type's function
//! returning it is a constructor, which may share its name with a field: an object's `new`
//! its unnamed one (`Counter('c', 1)`), and any other a named one (`Position.group`). Its
//! other functions are static members. An object is a final class owning its
//! handle, freed by `close` or the garbage collector; a record a final class; an enum a Dart
//! enum, or with data a sealed class of a final class per variant; a tuple a record; a
//! trait an abstract class, whose `close` Rust calls once it drops an implementation; a
//! closure a function type; a reader a `SeekableReader`; a future a `Cancelable`, a
//! `Future` that cancels; an error a thrown exception. The runtime they share, [`runtime/rt.dart`](../runtime/rt.dart), holds
//! the handles, the stack a call lowers its values into, and the port the isolate hears
//! Rust through.
//!
//! Dart runs code only on its isolate's thread, so Rust reaches Dart from any other through
//! [`runtime/dart.rs`](../runtime/dart.rs): a waker posts to the isolate's port, and a call
//! into a closure, a reader, or a trait's implementation waits for the isolate to run it.
//! Each call into Rust runs on a thread of its own while Rust holds an implementation, a
//! closure, or a reader, while the isolate's runs those calls.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;

use crate::abi::{self, Abi, Sig, Ty, pascal};
use crate::doc;
use crate::glue::rs;
use crate::ir::{Access, Closure, Family, Function, Int, Literal, On, Type};
use crate::kotlin::strip_refs;

const RUNTIME: &str = include_str!("../runtime/rt.dart");

/// The Rust runtime the crate's `dart` feature builds.
pub(crate) const GLUE_RUNTIME: &str = include_str!("../runtime/dart.rs");

/// Bytes Rust returns up to this long are copied into Dart's memory, and longer ones lent
/// to it: a lent list's finalizer, and the external size it charges the garbage collector,
/// cost more than a copy up to about this long.
const LEND: usize = 64 * 1024;

/// What the generator writes: the Dart, and the Rust glue the crate's `dart` feature builds.
pub(crate) struct Output {
	pub(crate) names: BTreeMap<String, String>,
	pub(crate) dart: String,
	pub(crate) glue: String,
}

/// The Dart for `abi`, whose header's contract is `contract`, or the names two items
/// claim alike.
pub(crate) fn generate(abi: &Abi<'_>, packages: &str, contract: u64) -> Result<Output, String> {
	// What the runtime and the ranges' and arrays' classes, which the ABI names, take.
	let ranges: BTreeSet<String> = (abi.ranges.iter())
		.flat_map(|(c, ty)| {
			let name = pascal(&c[abi.prefix.len() + 1..]);
			let cases: &[&str] = match ty {
				Type::Bound(_) => &["", "Unbounded", "Included", "Excluded"],
				_ => &[""],
			};
			cases
				.iter()
				.map(move |case| format!("{name}{case}"))
				.collect::<Vec<_>>()
		})
		.chain(abi.arrays.values().map(|n| Abi::array_class(*n)))
		.collect();
	let taken = |name: &str| TAKEN.contains(&name) || ranges.contains(name);
	let mut g = Gen {
		locked: BTreeMap::new(),
		abi,
		classes: classes(abi, &taken),
		variants: HashMap::new(),
		structs: BTreeMap::new(),
		layouts: HashMap::new(),
		twins: BTreeSet::new(),
		natives: BTreeMap::new(),
		helpers: BTreeMap::new(),
		writers: HashMap::new(),
		readers: HashMap::new(),
		types: String::new(),
		top: String::new(),
		glue: String::new(),
		names: BTreeMap::new(),
		errors: Vec::new(),
		thrown: BTreeMap::new(),
		conforms: BTreeSet::new(),
		comparables: BTreeSet::new(),
		relays: Vec::new(),
		trampolines: Vec::new(),
		constructors: BTreeMap::new(),
		tabled: BTreeSet::new(),
		arrays: BTreeMap::new(),
		written: BTreeSet::new(),
		fresh: 0,
		on: On::ALL,
		space: abi.space("dart"),
		handed: BTreeMap::new(),
		streams: Vec::new(),
	};
	g.locked = crate::lock::types(abi, &g.classes);
	g.variants = g.variants(&taken);
	let mut locked = std::mem::take(&mut g.locked);
	crate::lock::variants(abi, &mut locked, |c, v| {
		g.variants.get(&(g.class(c), v.to_string())).cloned()
	});
	g.locked = locked;
	let types: Vec<String> = (g.classes.keys())
		.map(|c| g.class(c))
		.chain(g.variants.values().cloned())
		.collect();
	for name in types {
		g.names.insert(name, "a type".to_string());
	}
	for name in &ranges {
		g.names.insert(name.clone(), "the runtime".to_string());
	}
	g.items();
	g.unconstructed();
	if !g.errors.is_empty() {
		return Err(g.errors.join("\n"));
	}
	let p = &abi.prefix;
	let (contract_fn, target_fn) = (private(&format!("{p}_contract")), private(&format!("{p}_target")));
	// A library whose every item is on every target never checks one.
	let checks = {
		let code: String = [&g.types, &g.top]
			.into_iter()
			.chain(g.helpers.values())
			.chain(g.structs.values())
			.chain(g.natives.values())
			.map(String::as_str)
			.collect();
		calls(&code, "_on(") > 0
	};
	// A library with no leaf never asks whether it has proven them.
	let leaves = {
		let code: String = [&g.types, &g.top]
			.into_iter()
			.chain(g.helpers.values())
			.chain(g.structs.values())
			.map(String::as_str)
			.collect();
		code.contains("if (_proven) {")
	};
	let proven = match leaves {
		true => {
			"/// Whether the library runs the bodies its leaves were proven against, which its leaf\n/// calls need: else each is served as any other call.\nfinal bool _proven = _dartProven();\n\n"
		}
		false => "",
	};
	let on_fn = match checks {
		true => format!(
			"/// The index of the library's target among those this file serves.\n\
			final int _target = {target_fn}();\n\n\
			/// Throws unless the library's target is among those of [on], as the item [what] is.\n\
			void _on(int on, String what) {{\n\tif ((on >>> _target) & 1 == 0) {{\n\t\tthrow UnsupportedError(\"$what: it is not on this library's target\");\n\t}}\n}}\n\n"
		),
		false => String::new(),
	};
	let mut dart = format!(
		"// Generated by rust2-ffi: Dart over the C ABI of the pub API of {packages}. Do not edit.\n\
		\n\
		import 'dart:async' as dasync;\n\
		import 'dart:convert' as convert;\n\
		import 'dart:ffi' as ffi;\n\
		import 'dart:io' as io;\n\
		import 'dart:isolate' as isolate;\n\
		import 'dart:typed_data' as typed;\n\n\
		const _file = '{p}.dart';\n\n\
		/// Bytes Rust returns past this long are lent to Dart rather than copied.\n\
		const _lend = {LEND};\n\n\
		bool _contract() => {contract_fn}() == 0x{contract:016x};\n\n\
		{on_fn}\
		{proven}\
		/// Throws unless [status], which the call [what] wrote, is zero: a run-time check failed.\n\
		// ignore: unused_element\n\
		void _status(int status, String what) {{\n\tif (status == 0) return;\n\t_n.status.value = 0;\n\tfinal m = _statusMessage(status);\n\tvar n = 0;\n\twhile (m[n] != 0) {{\n\t\tn++;\n\t}}\n\tthrow StateError('$what: ${{const convert.Utf8Decoder().convert(m.asTypedList(n))}}');\n}}\n\n"
	);
	for (name, params, ret, dart_params, dart_ret) in [
		("contract", "", "ffi.Uint64", "", "int"),
		("target", "", "ffi.Uint32", "", "int"),
		("dart_proven", "", "ffi.Bool", "", "bool"),
		("buf_free", "_Buf", "ffi.Void", "_Buf p0", "void"),
		("buf_copy", "_Buf", "_Buf", "_Buf p0", "_Buf"),
		("buf_clone", "_Buf", "_Buf", "_Buf p0", "_Buf"),
		("dart_buf", "ffi.Size", "_Buf", "int p0", "_Buf"),
		(
			"dart_init",
			"ffi.Pointer<ffi.Void>",
			"ffi.Bool",
			"ffi.Pointer<ffi.Void> p0",
			"bool",
		),
		(
			"dart_home",
			"ffi.Int64, ffi.Pointer<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>",
			"ffi.Pointer<ffi.Void>",
			"int p0, ffi.Pointer<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>> p1",
			"ffi.Pointer<ffi.Void>",
		),
		(
			"dart_waker",
			"ffi.Pointer<ffi.Void>, ffi.Int64",
			"_Waker",
			"ffi.Pointer<ffi.Void> p0, int p1",
			"_Waker",
		),
		(
			"dart_waker_call",
			"ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>",
			"ffi.Void",
			"ffi.Pointer<ffi.Void> p0, ffi.Pointer<ffi.Void> p1, ffi.Pointer<ffi.Void> p2",
			"void",
		),
		(
			"dart_future",
			"ffi.Pointer<ffi.Void>, ffi.Int64",
			"_ForeignFuture",
			"ffi.Pointer<ffi.Void> p0, int p1",
			"_ForeignFuture",
		),
		(
			"dart_future_done",
			"ffi.Pointer<ffi.Void>, ffi.Bool",
			"ffi.Bool",
			"ffi.Pointer<ffi.Void> p0, bool p1",
			"bool",
		),
		(
			"dart_relay",
			"ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>, ffi.Int64",
			"ffi.Pointer<ffi.Void>",
			"ffi.Pointer<ffi.Void> p0, ffi.Pointer<ffi.Void> p1, int p2",
			"ffi.Pointer<ffi.Void>",
		),
		(
			"dart_relay_free",
			"ffi.Pointer<ffi.Void>",
			"ffi.Void",
			"ffi.Pointer<ffi.Void> p0",
			"void",
		),
		(
			"dart_relay_drop",
			"ffi.Pointer<ffi.Void>",
			"ffi.Void",
			"ffi.Pointer<ffi.Void> p0",
			"void",
		),
		(
			"dart_run",
			"ffi.Pointer<ffi.Void>",
			"ffi.Void",
			"ffi.Pointer<ffi.Void> p0",
			"void",
		),
		(
			"dart_owner_free",
			"ffi.Pointer<ffi.Void>",
			"ffi.Void",
			"ffi.Pointer<ffi.Void> p0",
			"void",
		),
		(
			"dart_alloc",
			"ffi.Size",
			"ffi.Pointer<ffi.Void>",
			"int p0",
			"ffi.Pointer<ffi.Void>",
		),
		(
			"dart_free",
			"ffi.Pointer<ffi.Void>",
			"ffi.Void",
			"ffi.Pointer<ffi.Void> p0",
			"void",
		),
		(
			"dart_nursery",
			"ffi.Handle, ffi.Size",
			"ffi.Pointer<ffi.Pointer<ffi.Void>>",
			"Object p0, int p1",
			"ffi.Pointer<ffi.Pointer<ffi.Void>>",
		),
		("dart_abort", "_Buf", "ffi.Void", "_Buf p0", "void"),
		(
			"status_message",
			"ffi.Int32",
			"ffi.Pointer<ffi.Uint8>",
			"int p0",
			"ffi.Pointer<ffi.Uint8>",
		),
		(
			"dart_reader_read",
			"ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Uint8>, ffi.Size, ffi.Pointer<ffi.Size>, ffi.Pointer<_Buf>",
			"ffi.Bool",
			"ffi.Pointer<ffi.Void> p0, ffi.Pointer<ffi.Uint8> p1, int p2, ffi.Pointer<ffi.Size> p3, ffi.Pointer<_Buf> p4",
			"bool",
		),
		(
			"dart_reader_seek",
			"ffi.Pointer<ffi.Void>, ffi.Int, ffi.Int64, ffi.Pointer<ffi.Uint64>, ffi.Pointer<_Buf>",
			"ffi.Bool",
			"ffi.Pointer<ffi.Void> p0, int p1, int p2, ffi.Pointer<ffi.Uint64> p3, ffi.Pointer<_Buf> p4",
			"bool",
		),
	] {
		if (name == "target" && !checks) || (name == "dart_proven" && !leaves) {
			continue;
		}
		let local = match name {
			"contract" | "target" => private(&format!("{p}_{name}")),
			"buf_free" => "_bufFree".to_string(),
			"buf_copy" => "_bufCopy".to_string(),
			"status_message" => "_statusMessage".to_string(),
			"buf_clone" => "_bufClone".to_string(),
			other => format!("_{}", camel(other)),
		};
		let symbol = match name.strip_prefix("dart") {
			Some(rest) => format!("{}{rest}", g.space),
			None => format!("{p}_{name}"),
		};
		// What returns a constant runs no other code: a leaf call.
		let leaf = match name {
			"contract" | "target" | "dart_proven" | "status_message" => ", isLeaf: true",
			_ => "",
		};
		let _ = write!(
			dart,
			"@ffi.Native<{ret} Function({params})>(symbol: '{symbol}'{leaf})\nexternal {dart_ret} {local}({dart_params});\n\n"
		);
	}
	dart.push_str(RUNTIME);
	dart.push('\n');
	// Each link once every name is final; dartdoc's show only the item's own name.
	let render = |link: &doc::Link<'_>| match link.text {
		Some(_) => None,
		None => g.link(link.target).map(|target| format!("[{target}]")),
	};
	dart.push_str(&doc::links(&g.types, abi.api, render));
	dart.push_str(&doc::links(&g.top, abi.api, render));
	for code in g.helpers.values() {
		dart.push_str(code);
	}
	for code in g.structs.values() {
		dart.push_str(code);
	}
	for code in g.natives.values() {
		dart.push_str(code);
	}
	while dart.ends_with("\n\n") {
		dart.pop();
	}
	let space = &g.space;
	let glue = format!(
		"//! Generated by rust2-ffi: what dart/{p}.dart calls that `dart:ffi` lacks. Do not edit.\n\n\
		mod runtime;\n\n\
		pub use runtime::acting;\n\n\
		#[allow(unused_imports, reason = \"the C ABI's structs\")]\n\
		use crate::*;\n\n\
		use std::ffi::c_void;\n\n\
		use crate::rt;\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_init(data: *const c_void) -> bool {{\n\tunsafe {{ runtime::init(data) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_home(port: i64, dropped: unsafe extern \"C\" fn(i64)) -> *const c_void {{\n\tstd::ptr::from_ref(runtime::home(port, dropped)).cast()\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_waker(home: *const c_void, token: i64) -> rt::Waker {{\n\truntime::waker(unsafe {{ &*home.cast() }}, token)\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_waker_call(home: *const c_void, f: unsafe extern \"C\" fn(*mut c_void), data: *mut c_void) {{\n\tlet home = unsafe {{ &*home.cast::<runtime::Home>() }};\n\tunsafe {{ home.serve(move || f(data)) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_future(home: *const c_void, token: i64) -> rt::ForeignFuture {{\n\truntime::future(unsafe {{ &*home.cast() }}, token)\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_future_done(data: *const c_void, failed: bool) -> bool {{\n\tunsafe {{ runtime::future_done(data, failed) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_relay(home: *const c_void, f: *const c_void, token: i64) -> *mut c_void {{\n\truntime::relay(unsafe {{ &*home.cast() }}, f, token).cast()\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_relay_free(relay: *mut c_void) {{\n\tunsafe {{ runtime::relay_free(relay.cast()) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_relay_drop(relay: *mut c_void) {{\n\tunsafe {{ runtime::relay_drop(relay) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_run(home: *const c_void) {{\n\tunsafe {{ runtime::run(&*home.cast()) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_owner_free(owner: *mut c_void) {{\n\tunsafe {{ rt::Buf {{ ptr: std::ptr::null(), len: 0, owner }}.free() }}\n}}\n\n\
		unsafe extern \"C\" {{\n\tfn malloc(len: usize) -> *mut c_void;\n\tfn free(ptr: *mut c_void);\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_alloc(len: usize) -> *mut c_void {{\n\tlet ptr = unsafe {{ malloc(len) }};\n\tif ptr.is_null() {{\n\t\tstd::alloc::handle_alloc_error(std::alloc::Layout::array::<u8>(len).expect(\"a length\"));\n\t}}\n\tptr\n}}\n\n\
		#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_buf(len: usize) -> rt::Buf {{\n\trt::Buf::vec(vec![0; len])\n}}\n\n\
		#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_proven() -> bool {{\n\tcrate::PROVEN\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_free(ptr: *mut c_void) {{\n\tunsafe {{ free(ptr) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_nursery(object: *mut c_void, len: usize) -> *mut *mut c_void {{\n\tunsafe {{ runtime::nursery(object, len) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_abort(text: rt::Buf) {{\n\trt::fail(\"dart\", unsafe {{ text.str(\"{space}_abort\") }})\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_reader_read(data: *mut c_void, buf: *mut u8, len: usize, n: *mut usize, err: *mut rt::Buf) -> bool {{\n\tunsafe {{ runtime::reader_read(data, buf, len, n, err) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_reader_seek(data: *mut c_void, whence: std::ffi::c_int, offset: i64, position: *mut u64, err: *mut rt::Buf) -> bool {{\n\tunsafe {{ runtime::reader_seek(data, whence, offset, position, err) }}\n}}\n\n{}",
		g.glue
	);
	Ok(Output {
		names: g.locked,
		dart: dart + "\n",
		glue,
	})
}

/// What the runtime and the generated code name unqualified, which an exported item of
/// the same name would shadow, and the prefixes they import Dart's libraries under.
const TAKEN: &[&str] = &[
	"ArgumentError",
	"BigInt",
	"Cancelable",
	"Cancelled",
	"Comparable",
	"Deprecated",
	"Duration",
	"Exception",
	"Expando",
	"FormatException",
	"Function",
	"Future",
	"IpAddr",
	"Ipv4Addr",
	"Ipv6Addr",
	"List",
	"Map",
	"Mut",
	"Never",
	"Object",
	"PathBuf",
	"RangeError",
	"RustDuration",
	"SeekFrom",
	"SeekableReader",
	"SocketAddr",
	"SocketAddrV4",
	"SocketAddrV6",
	"StackTrace",
	"StateError",
	"String",
	"U64",
	"UnsupportedError",
	"Waker",
	"bool",
	"convert",
	"dasync",
	"double",
	"ffi",
	"identical",
	"int",
	"io",
	"isolate",
	"override",
	"typed",
];

/// Dart's reserved words, which no identifier may be, and its built-in identifiers, which
/// no type may be.
const RESERVED: &[&str] = &[
	"assert", "await", "break", "case", "catch", "class", "const", "continue", "default", "do", "else", "enum",
	"extends", "false", "final", "finally", "for", "if", "in", "is", "new", "null", "rethrow", "return", "super",
	"switch", "this", "throw", "true", "try", "var", "void", "while", "with", "yield",
	// The core types the binding names unqualified, which a member of the class would hide.
	"bool", "double", "dynamic", "int", "num",
];

const BUILT_IN: &[&str] = &[
	"abstract",
	"as",
	"covariant",
	"deferred",
	"dynamic",
	"export",
	"extension",
	"external",
	"factory",
	"Function",
	"get",
	"implements",
	"import",
	"interface",
	"late",
	"library",
	"mixin",
	"operator",
	"part",
	"required",
	"set",
	"static",
	"typedef",
];

/// The members Dart gives every object.
const OBJECT: &[&str] = &["hashCode", "noSuchMethod", "runtimeType", "toString"];

struct Gen<'a> {
	locked: BTreeMap<String, String>,
	abi: &'a Abi<'a>,
	/// Each exported type's name, by its C name.
	classes: HashMap<String, String>,
	/// Each data enum's variant's class, by the enum's Dart name and the variant's.
	variants: HashMap<(String, String), String>,
	/// Each C struct's `dart:ffi` class, by C name.
	structs: BTreeMap<String, String>,
	/// What each C struct holds by value, by C name: what decides whether it holds an `f32`,
	/// and its twin's fields if so.
	layouts: HashMap<String, Layout>,
	/// Each C struct holding an `f32` whose twin the glue defines.
	twins: BTreeSet<String>,
	/// Each C function's `@Native` declaration, by C name.
	natives: BTreeMap<String, String>,
	/// The conversions, polls, and trampolines, by name.
	helpers: BTreeMap<String, String>,
	/// Each writer's name, by the type it writes: types of one C struct, such as a `Vec` of
	/// `SocketAddrV4` and one of `SocketAddr`, check what they write apart.
	writers: HashMap<Type, String>,
	/// Each reader's name, by the type it reads: types of one C struct, such as
	/// `Vec<Vec<u32>>` and `Vec<BTreeSet<u32>>`, which Dart holds differently, read apart.
	readers: HashMap<Type, String>,
	/// The types, then the top-level functions and constants.
	types: String,
	top: String,
	/// The Rust glue's relays and served calls.
	glue: String,
	/// Each top-level Dart name, with what claimed it.
	names: BTreeMap<String, String>,
	errors: Vec<String>,
	/// The exception each error type that is no exported type throws as, by name.
	thrown: BTreeMap<String, Type>,
	/// The exported types thrown as errors, which implement `Exception`.
	conforms: BTreeSet<String>,
	/// The exported types ordered by `Ord`, which implement `Comparable`.
	comparables: BTreeSet<String>,
	/// Each C signature of a lent function, whose relay is `<space>_fn_<n>`.
	relays: Vec<Ty>,
	/// Each closure type, whose trampoline is `_t<n>`.
	trampolines: Vec<Type>,
	/// Each object class's `_` constructor, with the native that frees what it builds, and its
	/// `_lent` one, which go if nothing calls them.
	constructors: BTreeMap<String, (String, String, String)>,
	/// The callbacks whose tables are written, which only one given to Rust has.
	tabled: BTreeSet<String>,
	/// Each array class's `_of` and `_write`, which go if nothing reads or writes the class.
	arrays: BTreeMap<String, (String, String)>,
	/// The array classes written into Rust.
	written: BTreeSet<String>,
	fresh: usize,
	/// The targets of the item being written, whose calls check theirs.
	on: On,
	/// The namespace of the glue's C names, apart from the export's.
	space: String,
	/// A value's `&mut self` methods by their C names, which a call makes in place on a
	/// copy it hands back.
	handed: BTreeMap<String, abi::Handed>,
	/// The streams among the methods just written, each by its `next`'s member, item type,
	/// name-table key, and deprecation, whose `asStream` its type adds after every method.
	streams: Vec<(String, String, String, Option<String>)>,
}

impl<'a> Gen<'a> {
	fn fresh(&mut self, base: &str) -> String {
		self.fresh += 1;
		format!("{base}{}_", self.fresh)
	}

	/// A C name without the prefix.
	fn bare<'n>(&self, c: &'n str) -> &'n str {
		&c[self.abi.prefix.len() + 1..]
	}

	/// The Dart name of the C type `c`.
	fn class(&self, c: &str) -> String {
		type_name(&self.classes.get(c).cloned().unwrap_or_else(|| pascal(self.bare(c))))
	}

	/// The top-level Dart name `what` takes, `name` unless that is taken.
	fn claim(&mut self, name: &str, what: String) -> String {
		let free = alternatives(name)
			.find(|alt| !(TAKEN.contains(&alt.as_str()) || self.names.contains_key(alt)))
			.expect("alternatives never end");
		self.names.insert(free.clone(), what);
		free
	}

	/// The class of the variant `variant` of the data enum Dart names `name`.
	fn variant_class(&self, name: &str, variant: &str) -> String {
		self.variants[&(name.to_string(), variant.to_string())].clone()
	}

	/// The class of each variant of a data enum, by the enum's Dart name and the variant's:
	/// `{enum}{variant}`, [`unique`] among them and apart from every other top-level
	/// name the runtime or a type takes.
	fn variants(&self, taken: &dyn Fn(&str) -> bool) -> HashMap<(String, String), String> {
		let api = self.abi.api;
		let mut keys = Vec::new();
		for (i, e) in api.enums.iter().enumerate() {
			if e.plain() {
				continue;
			}
			let name = self.class(&self.abi.enumeration(i));
			keys.extend(e.variants.iter().map(|v| (name.clone(), v.name.clone())));
		}
		keys.sort();
		let types: BTreeSet<String> = (self.classes.keys()).map(|c| self.class(c)).collect();
		let names = keys
			.iter()
			.map(|(name, variant)| format!("{name}{}", variant.replace('_', "")))
			.collect();
		let names = unique(names, |name| taken(name) || types.contains(name));
		keys.into_iter().zip(names).collect()
	}

	/// Asserts that no two members of the type `rust` share a Dart name, as [`collides`]
	/// reads them and [`abi::assert_apart`] checks.
	fn distinct(&self, rust: &str, members: &[String]) {
		abi::assert_apart(rust, "Dart", members, collides);
	}

	/// Drops the constructors of the object classes that nothing calls: a type only ever lent
	/// to a callback has no `_`, and one only ever returned has no `_lent`, which Dart's
	/// analyzer would report as unused.
	fn unconstructed(&mut self) {
		let code: String = [&self.types, &self.top]
			.into_iter()
			.chain(self.helpers.values())
			.chain(self.structs.values())
			.map(String::as_str)
			.collect();
		let calls = |name: &str, ctor: &str| calls(&code, &format!("{name}.{ctor}("));
		for (name, (of, write)) in std::mem::take(&mut self.arrays) {
			if calls(&name, "_of") == 0 {
				self.types = self.types.replace(&of, "");
			}
			if !self.written.contains(&name) {
				self.types = self.types.replace(&write, "");
			}
		}
		for (name, (owned, lent, freeing)) in std::mem::take(&mut self.constructors) {
			// Each declares itself once.
			let owned_unused = calls(&name, "_") == 1;
			let lent_unused = calls(&name, "_lent") == 1;
			if owned_unused {
				self.types = self.types.replace(&owned, "");
				self.natives.retain(|_, decl| !decl.contains(&format!(" {freeing}(")));
			}
			// A class nothing makes, an object only ever passed to Rust, keeps one constructor, as
			// Dart's implicit one would call `_Object()`, which takes arguments.
			if lent_unused {
				let kept = match owned_unused {
					true => format!(
						"\t// The class's only constructor, which no Dart code calls.\n\t// ignore: unused_element\n{lent}"
					),
					false => String::new(),
				};
				self.types = self.types.replace(&lent, &kept);
			}
		}
	}

	/// The Dart type of `ty`, as a value, a parameter, or a result.
	fn dt(&mut self, ty: &Type) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Array(n) => Abi::array_class(*n),
			Type::Unit => "void".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) | Type::NonZero(int) => int_dt(*int).to_string(),
			Type::F32 | Type::F64 => "double".to_string(),
			Type::Char | Type::Str | Type::String => "String".to_string(),
			Type::Path | Type::PathBuf => "PathBuf".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "typed.Uint8List".to_string(),
			Type::U128 => "BigInt".to_string(),
			Type::Duration => "RustDuration".to_string(),
			Type::SocketAddr(family) => addr_class(true, *family).to_string(),
			Type::IpAddr(family) => addr_class(false, *family).to_string(),
			// `u64`'s block is a `Uint64List` read as `U64`, which an `int` stands for.
			Type::List(..) if ty.block() == Some(&Type::Int(Int::U64)) => "List<U64>".to_string(),
			Type::List(..) if let Some(elem) = ty.block() => format!("typed.{}", typed_list(elem)),
			Type::List(_, inner) => format!("List<{}>", self.dt(inner)),
			Type::Waker => "void Function()".to_string(),
			Type::Reader => "SeekableReader".to_string(),
			Type::Future => "Future<void>".to_string(),
			Type::Option(inner) => {
				if matches!(inner.base(), Type::Option(_)) {
					let inner = self.dt(inner.base());
					self.errors.push(format!(
						"`Option<Option<{inner}>>` has no Dart form, which has one null"
					));
				}
				format!("{}?", self.dt(inner))
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.dt(inner),
			Type::Record(i) => self.class(&abi.record(*i)),
			Type::Enum(i) => self.class(&abi.enumeration(*i)),
			Type::Object(i) => self.class(&abi.object(*i)),
			Type::Callback(i, _) => self.class(&abi.callback(*i)),
			Type::Closure(closure_) => {
				let (params, ret) = (&closure_.params, &closure_.ok);
				let params: Vec<String> = params.iter().map(|p| self.dt(p)).collect();
				format!("{} Function({})", self.dt(ret), params.join(", "))
			}
			Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => self.class(&abi.range(ty)),
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.dt(e)).collect();
				match elements.len() {
					1 => format!("({},)", elements[0]),
					_ => format!("({})", elements.join(", ")),
				}
			}
		}
	}

	/// The `dart:ffi` native type of a C type.
	fn nt(&mut self, ty: &Ty) -> String {
		match ty {
			Ty::Void => "ffi.Void".to_string(),
			Ty::Bool => "ffi.Bool".to_string(),
			Ty::Int(int) => native_int(*int).to_string(),
			// Its bits, which no float `dart:ffi` converts keeps for a signaling NaN.
			Ty::F32 | Ty::Char => "ffi.Uint32".to_string(),
			Ty::F64 => "ffi.Double".to_string(),
			Ty::Enum(_) | Ty::Poll => "ffi.Int".to_string(),
			Ty::Status => "ffi.Int32".to_string(),
			Ty::Handle { .. } | Ty::Future(_) | Ty::Data => "ffi.Pointer<ffi.Void>".to_string(),
			Ty::Out(inner) | Ty::In(inner) => format!("ffi.Pointer<{}>", self.nt(inner)),
			Ty::Fn(params, ret) => {
				let params: Vec<String> = std::iter::once("ffi.Pointer<ffi.Void>".to_string())
					.chain(params.iter().map(|p| self.nt(p)))
					.collect();
				format!(
					"ffi.Pointer<ffi.NativeFunction<{} Function({})>>",
					self.nt(ret),
					params.join(", ")
				)
			}
			ty => self.struct_class(ty),
		}
	}

	/// The Dart type `dart:ffi` gives a C type in a `@Native` signature.
	fn ct(&mut self, ty: &Ty) -> String {
		match ty {
			Ty::Void => "void".to_string(),
			Ty::Bool => "bool".to_string(),
			Ty::Int(_) | Ty::F32 | Ty::Char | Ty::Enum(_) | Ty::Poll | Ty::Status => "int".to_string(),
			Ty::F64 => "double".to_string(),
			ty => self.nt(ty),
		}
	}

	/// Whether `dart:ffi` passes a C type as a Dart scalar, not a struct.
	fn scalar(ty: &Ty) -> bool {
		!matches!(
			ty,
			Ty::Buf
				| Ty::U128
				| Ty::Duration
				| Ty::SocketAddr(_)
				| Ty::IpAddr(_)
				| Ty::Struct(_)
				| Ty::Waker
				| Ty::ForeignFuture
				| Ty::ForeignReader
		)
	}

	/// The class of a C struct, defined once.
	fn struct_class(&mut self, ty: &Ty) -> String {
		let abi = self.abi;
		let name = match ty {
			Ty::Buf => return "_Buf".to_string(),
			Ty::U128 => return "_U128".to_string(),
			Ty::Duration => return "_Duration".to_string(),
			Ty::SocketAddr(family) => return format!("_{}", addr_class(true, *family)),
			Ty::IpAddr(family) => return format!("_{}", addr_class(false, *family)),
			Ty::Waker => return "_Waker".to_string(),
			Ty::ForeignFuture => return "_ForeignFuture".to_string(),
			Ty::ForeignReader => return "_ForeignReader".to_string(),
			Ty::Struct(name) => name.clone(),
			other => unreachable!("{other:?} is no struct"),
		};
		if (0..abi.api.callbacks.len()).any(|i| abi.callback(i) == name) {
			return "_Callback".to_string();
		}
		let class = private_type(&name);
		if self.structs.contains_key(&name) {
			return class;
		}
		self.structs.insert(name.clone(), String::new());
		self.layouts.insert(name.clone(), Layout::Opaque);
		if let Some(n) = abi.arrays.get(&name) {
			let code = format!(
				"final class {class} extends ffi.Struct {{\n\t@ffi.Array({n})\n\texternal ffi.Array<ffi.Uint8> items;\n}}\n\n"
			);
			self.structs.insert(name, code);
			return class;
		}
		let mut fields: Vec<(String, Ty)> = Vec::new();
		if let Some(ty) = abi.ranges.get(&name) {
			match ty {
				Type::Range(int) => {
					fields.push(("start".into(), Ty::Int(*int)));
					fields.push(("end".into(), Ty::Int(*int)));
				}
				Type::Bound(int) => {
					fields.push(("kind".into(), Ty::Enum(abi.rt("bound"))));
					fields.push(("value".into(), Ty::Int(*int)));
				}
				Type::Bounds(int) => {
					let bound = Ty::Struct(abi.range(&Type::Bound(*int)));
					fields.push(("start".into(), bound.clone()));
					fields.push(("end".into(), bound));
				}
				other => unreachable!("{other:?} is no range"),
			}
		} else if let Some(elem) = abi.lists.get(&name) {
			let elem = abi.ty(elem);
			fields.push(("ptr".into(), Ty::In(Box::new(elem))));
			fields.push(("len".into(), Ty::Int(Int::Usize)));
			fields.push(("owner".into(), Ty::Data));
		} else if let Some(inner) = abi.options.get(&name) {
			fields.push(("some".into(), Ty::Bool));
			fields.push(("value".into(), abi.ty(inner)));
		} else if let Some(elements) = abi.tuples.get(&name) {
			for (n, element) in elements.iter().enumerate() {
				fields.push((format!("${n}"), abi.ty(element)));
			}
		} else if let Some(i) = (0..abi.api.records.len()).find(|&i| abi.record(i) == name) {
			if let Some(opaque) = &abi.api.records[i].opaque {
				let code = format!(
					"final class {class} extends ffi.Struct {{\n\t@ffi.Array({})\n\texternal ffi.Array<ffi.Uint{}> $opaque;\n}}\n\n",
					opaque.units(),
					opaque.unit() * 8
				);
				self.structs.insert(name, code);
				return class;
			}
			for field in &abi.api.records[i].fields {
				fields.push((abi::escape(&field.name), abi.ty(&field.ty)));
			}
		} else if let Some(i) = (0..abi.api.enums.len()).find(|&i| abi.enumeration(i) == name) {
			let e = &abi.api.enums[i];
			let mut extra = String::new();
			let union = private_type(&format!("{name}_u"));
			let mut members = Vec::new();
			let mut payloads = Vec::new();
			for variant in e.variants.iter().filter(|v| !v.fields.is_empty()) {
				let payload = abi::payload(&name, &variant.name);
				let payload_class = private_type(&payload);
				let mut decls = Vec::new();
				let mut tys = Vec::new();
				for field in &variant.fields {
					let ty = abi.ty(&field.ty);
					decls.push(self.field(&ty, &abi::escape(&field.name)));
					tys.push(ty);
				}
				self.layouts.insert(payload.clone(), Layout::Fields(tys));
				payloads.push(payload.clone());
				let _ = write!(
					extra,
					"final class {payload_class} extends ffi.Struct {{\n{}}}\n\n",
					decls.concat()
				);
				members.push(format!(
					"\texternal {payload_class} {};\n",
					slot(&abi::escape(&abi::snake(&variant.name)))
				));
			}
			let _ = write!(
				extra,
				"final class {union} extends ffi.Union {{\n{}}}\n\n",
				members.concat()
			);
			let decls = format!(
				"{}\texternal {union} $u;\n",
				self.field(&Ty::Enum(format!("{name}_tag")), "tag")
			);
			self.layouts.insert(name.clone(), Layout::Enum(payloads));
			self.structs.insert(
				name,
				format!("final class {class} extends ffi.Struct {{\n{decls}}}\n\n{extra}"),
			);
			return class;
		} else {
			unreachable!("`{name}` is no struct of the API");
		}
		let decls: String = fields.iter().map(|(n, ty)| self.field(ty, n)).collect();
		self.layouts.insert(
			name.clone(),
			Layout::Fields(fields.into_iter().map(|(_, ty)| ty).collect()),
		);
		self.structs.insert(
			name,
			format!("final class {class} extends ffi.Struct {{\n{decls}}}\n\n"),
		);
		class
	}

	/// Whether `ty` holds an `f32` by value, which Dart passes as its bits: in a struct, which
	/// `dart:ffi` then passes by an ABI of its own, as one of integers where C's has a float.
	fn holds_f32(&mut self, ty: &Ty) -> bool {
		match ty {
			Ty::F32 => true,
			Ty::Struct(name) => {
				// A data enum's payload is laid out with its enum, which has no class of its own.
				if !self.layouts.contains_key(name) {
					self.struct_class(ty);
				}
				match self.layouts.get(name).cloned() {
					Some(Layout::Fields(tys)) => tys.iter().any(|ty| self.holds_f32(ty)),
					Some(Layout::Enum(payloads)) => payloads.iter().any(|p| self.holds_f32(&Ty::Struct(p.clone()))),
					Some(Layout::Opaque) | None => false,
				}
			}
			_ => false,
		}
	}

	/// The Rust type a C type crosses between Dart and the glue as: an `f32` as its bits, and
	/// a struct holding one as its twin, which Dart's class mirrors.
	fn crossing(&mut self, ty: &Ty) -> String {
		match ty {
			Ty::F32 => "u32".to_string(),
			Ty::Struct(name) if self.holds_f32(ty) => self.twin(name),
			ty => rs(ty),
		}
	}

	/// The twin of the C struct `name`, which holds an `f32`: of its layout, each `f32` in it
	/// as its bits, so it crosses by the ABI `dart:ffi` gives its class. Named in the glue's
	/// namespace, which no C name takes.
	fn twin(&mut self, name: &str) -> String {
		let twin = format!("{}_twin_{}", self.space, self.bare(name));
		let union = format!("{}_union_{}", self.space, self.bare(name));
		if !self.twins.insert(name.to_string()) {
			return twin;
		}
		let code = match self.layouts.get(name).cloned() {
			Some(Layout::Fields(tys)) => {
				let fields: Vec<String> = tys.iter().map(|ty| format!("pub {}", self.crossing(ty))).collect();
				format!(
					"#[repr(C)]\n#[derive(Clone, Copy)]\npub struct {twin}({});\n\n",
					fields.join(", ")
				)
			}
			Some(Layout::Enum(payloads)) => {
				let members: Vec<String> = (payloads.iter().enumerate())
					.map(|(n, p)| format!("\tpub v{n}: {},\n", self.crossing(&Ty::Struct(p.clone()))))
					.collect();
				format!(
					"#[repr(C)]\n#[derive(Clone, Copy)]\npub struct {twin}(pub i32, pub {union});\n\n#[repr(C)]\n#[derive(Clone, Copy)]\npub union {union} {{\n{}}}\n\n",
					members.concat()
				)
			}
			Some(Layout::Opaque) | None => unreachable!("`{name}` holds no f32"),
		};
		self.glue.push_str(&code);
		twin
	}

	/// The C value of `e`, which crossed from Dart as [`Self::crossing`] gives `ty`.
	fn dart_in(&mut self, ty: &Ty, e: &str) -> String {
		match ty {
			Ty::F32 => format!("f32::from_bits({e})"),
			Ty::Struct(_) if self.holds_f32(ty) => {
				let twin = self.crossing(ty);
				format!("unsafe {{ std::mem::transmute::<{twin}, {}>({e}) }}", rs(ty))
			}
			_ => e.to_string(),
		}
	}

	/// The C value `e` of type `ty` as it crosses to Dart, as [`Self::crossing`] gives it.
	fn dart_out(&mut self, ty: &Ty, e: &str) -> String {
		match ty {
			Ty::F32 => format!("{e}.to_bits()"),
			Ty::Struct(_) if self.holds_f32(ty) => {
				let twin = self.crossing(ty);
				format!("unsafe {{ std::mem::transmute::<{}, {twin}>({e}) }}", rs(ty))
			}
			_ => e.to_string(),
		}
	}

	/// The Rust type of a Dart trampoline taking its data, then `params`, and returning `ret`,
	/// each as it crosses.
	fn crossing_fn(&mut self, params: &[Ty], ret: &Ty) -> String {
		let params: String = params.iter().map(|ty| format!(", {}", self.crossing(ty))).collect();
		format!("unsafe extern \"C\" fn(*mut c_void{params}) -> {}", self.crossing(ret))
	}

	/// Whether a C function of `params` and `ret` passes an `f32` by value, as its bits to
	/// Dart, so Dart calls it, or it calls Dart, through the glue.
	fn passes_f32(&mut self, params: &[Ty], ret: &Ty) -> bool {
		self.holds_f32(ret) || params.iter().any(|ty| self.holds_f32(ty))
	}

	/// A struct's field of C type `ty`, named `$name`.
	fn field(&mut self, ty: &Ty, name: &str) -> String {
		let annotation = match ty {
			Ty::Bool => Some("ffi.Bool"),
			Ty::Int(int) => Some(native_int(*int)),
			Ty::F32 | Ty::Char => Some("ffi.Uint32"),
			Ty::F64 => Some("ffi.Double"),
			Ty::Enum(_) | Ty::Poll => Some("ffi.Int"),
			_ => None,
		};
		match annotation {
			Some(a) => format!("\t@{a}()\n\texternal {} {};\n", self.ct(ty), slot(name)),
			None => format!("\texternal {} {};\n", self.nt(ty), slot(name)),
		}
	}

	/// The `@Native` binding of a C function, declared once, as Dart calls it.
	fn native(&mut self, sig: &Sig) -> String {
		self.declare(&sig.name, sig, false)
	}

	/// The `@Native` binding of `sig`'s C function as the Dart function `_<name>`, declared
	/// once, as a leaf call if `leaf`.
	fn declare(&mut self, name: &str, sig: &Sig, leaf: bool) -> String {
		let local = private(name);
		if !self.natives.contains_key(name) {
			let natives: Vec<String> = sig.params.iter().map(|(_, ty)| self.nt(ty)).collect();
			let darts: Vec<String> = (sig.params.iter().enumerate())
				.map(|(k, (_, ty))| format!("{} p{k}", self.ct(ty)))
				.collect();
			let decl = format!(
				"@ffi.Native<{} Function({})>(symbol: '{}'{})\nexternal {} {local}({});\n\n",
				self.nt(&sig.ret),
				natives.join(", "),
				sig.name,
				if leaf { ", isLeaf: true" } else { "" },
				self.ct(&sig.ret),
				darts.join(", ")
			);
			self.natives.insert(name.to_string(), decl);
		}
		local
	}

	/// The Dart function calling `call` through a thunk that serves the isolate while it
	/// waits, as [`runtime/dart.rs`](../runtime/dart.rs)'s `Home::serve` does, which takes
	/// the isolate's home first.
	fn served(&mut self, call: &Sig) -> String {
		let name = format!("{}_serve_{}", self.space, self.bare(&call.name));
		if self.natives.contains_key(&name) {
			return private(&name);
		}
		let mut params = vec![Ty::Data];
		params.extend(call.params.iter().map(|(_, ty)| ty.clone()));
		let decls: Vec<String> = (call.params.iter().enumerate())
			.map(|(n, (_, ty))| format!(", p{n}: {}", self.crossing(ty)))
			.collect();
		let mut args: Vec<String> = (call.params.iter().enumerate())
			.map(|(n, (_, ty))| self.dart_in(ty, &format!("p{n}")))
			.collect();
		// A status the call writes reaches the isolate's one cell only once `serve` stopped
		// serving: a call it served meanwhile, on the isolate's thread, read its own first. A
		// null one, which aborts on a failed check, passes through.
		let status = call.params.last().is_some_and(|(_, ty)| abi::is_status(ty));
		let cell = match status {
			true => {
				let cell = args.pop().expect("a status is a parameter");
				args.push("s_".to_string());
				Some(cell)
			}
			false => None,
		};
		let call_ = match self.handed.get(&call.name) {
			Some(h) => h.call(&args),
			None => format!("crate::{}({})", call.name, args.join(", ")),
		};
		let body = match cell {
			Some(cell) => format!(
				"let mut status = 0;\n\tlet s_ = match {cell}.is_null() {{\n\t\ttrue => {cell},\n\t\tfalse => &raw mut status,\n\t}};\n\tlet r = unsafe {{ home.serve(move || {call_}) }};\n\tif !{cell}.is_null() {{\n\t\tunsafe {{ *{cell} = status }};\n\t}}\n\t{}",
				self.dart_out(&call.ret, "r")
			),
			None => self.dart_out(&call.ret, &format!("unsafe {{ home.serve(move || {call_}) }}")),
		};
		let ret = self.crossing(&call.ret);
		let _ = write!(
			self.glue,
			"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {name}(home: *const c_void{}) -> {ret} {{\n\tlet home = unsafe {{ &*home.cast::<runtime::Home>() }};\n\t{body}\n}}\n\n",
			decls.concat(),
		);
		let native = self.native_fn(&name, params, call.ret.clone());
		self.checked_on(&name, native, call.params.len() + 1, &call.name)
	}

	/// The Dart function making `call`, a leaf as [`Abi::leaf`] proves, as a leaf call: to
	/// the C function itself, on the isolate's thread, which no call it makes back waits on.
	fn leaf(&mut self, call: &Sig) -> String {
		let name = format!("{}_leaf_{}", self.space, self.bare(&call.name));
		let params: Vec<Ty> = call.params.iter().map(|(_, ty)| ty.clone()).collect();
		// One passing an `f32` calls the glue's twin of it, which converts its bits.
		let native = match self.passes_f32(&params, &call.ret) {
			true => {
				let decls: Vec<String> = (params.iter().enumerate())
					.map(|(n, ty)| format!("p{n}: {}", self.crossing(ty)))
					.collect();
				let args: Vec<String> = (params.iter().enumerate())
					.map(|(n, ty)| self.dart_in(ty, &format!("p{n}")))
					.collect();
				let body = self.dart_out(
					&call.ret,
					&format!("unsafe {{ crate::{}({}) }}", call.name, args.join(", ")),
				);
				let ret = self.crossing(&call.ret);
				let _ = write!(
					self.glue,
					"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {name}({}) -> {ret} {{\n\t{body}\n}}\n\n",
					decls.join(", "),
				);
				let twin = Sig {
					name: name.clone(),
					..call.clone()
				};
				self.declare(&name, &twin, true)
			}
			false => self.declare(&name, call, true),
		};
		self.checked_on(&name, native, call.params.len(), &call.name)
	}

	/// `native`, declared under `name` with `params` parameters, or on a target its item,
	/// the C function `what`, is not on, a function throwing before it calls in.
	fn checked_on(&mut self, name: &str, native: String, params: usize, what: &str) -> String {
		let api = self.abi.api;
		if api.everywhere(self.on) {
			return native;
		}
		// On a target its item is not on, the call throws before it calls in.
		let checked = format!("{native}$on");
		if !self.natives.contains_key(&checked) {
			let decl = self.natives[name].replacen(&format!("{native}("), &format!("{checked}("), 1);
			let params: Vec<String> = (0..params).map(|n| format!("p{n}")).collect();
			let declared = &decl[decl.find("external ").expect("a native") + "external ".len()..];
			let ret = &declared[..declared.find(' ').expect("a native's return")];
			let typed =
				&declared[declared.find('(').expect("a native's parameters")..declared.rfind(");").expect("a native")];
			let wrapper = format!(
				"{ret} {native}{typed}) {{\n\t_on(0x{:x}, '{}');\n\treturn {checked}({});\n}}\n\n",
				self.on.0 & api.all().0,
				what,
				params.join(", ")
			);
			self.natives.insert(name.to_string(), decl);
			self.natives.insert(checked, wrapper);
		}
		native
	}

	/// The Dart functions owning a handle of the object `c`, whose C type is `handle`: one
	/// giving the garbage collector a Dart object owning it to free it with, the C function
	/// freeing it, which the nursery keeps for the isolate's exit, the one freeing it, and of
	/// a checked value, the one dropping it then freeing it if asked; the last two serve the
	/// isolate meanwhile and take back the garbage collector's hold if it took one.
	fn owned(&mut self, c: &str, handle: &Ty, checked: bool) -> Owned {
		let bare = self.bare(c);
		let space = &self.space;
		let own = format!("{space}_own_{bare}");
		let own_ = private(&own);
		let raw = rs(handle);
		let _ = write!(
			self.glue,
			"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {own}(object: *mut c_void, raw: *mut c_void) -> *mut c_void {{\n\t// The garbage collector's free runs with no isolate to serve.\n\tunsafe extern \"C\" fn finalize(_: *mut c_void, raw: *mut c_void) {{\n\t\tunsafe {{ crate::{c}_free(raw.cast()) }}\n\t}}\n\tunsafe {{ runtime::own(object, raw, finalize) }}\n}}\n\n"
		);
		self.natives.insert(
			own.clone(),
			format!(
				"@ffi.Native<ffi.Pointer<ffi.Void> Function(ffi.Handle, ffi.Pointer<ffi.Void>)>(symbol: '{own}')\nexternal ffi.Pointer<ffi.Void> {own_}(Object p0, ffi.Pointer<ffi.Void> p1);\n\n"
			),
		);
		let freeing = format!("{space}_freeing_{bare}");
		let freeing_ = private(&freeing);
		self.natives.insert(
			freeing.clone(),
			format!(
				"@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(symbol: '{c}_free')\nexternal void {freeing_}(ffi.Pointer<ffi.Void> p0);\n\n"
			),
		);
		let free = format!("{space}_free_{bare}");
		let free_ = private(&free);
		let _ = write!(
			self.glue,
			"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {free}(home: *const c_void, own: *mut c_void, object: *mut c_void, raw: {raw}) {{\n\tif !own.is_null() {{\n\t\tunsafe {{ runtime::disown(own, object) }};\n\t}}\n\tlet home = unsafe {{ &*home.cast::<runtime::Home>() }};\n\tunsafe {{ home.serve(move || crate::{c}_free(raw)) }}\n}}\n\n"
		);
		self.natives.insert(
			free.clone(),
			format!(
				"@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>, ffi.Handle, ffi.Pointer<ffi.Void>)>(symbol: '{free}')\nexternal void {free_}(ffi.Pointer<ffi.Void> p0, ffi.Pointer<ffi.Void> p1, Object p2, ffi.Pointer<ffi.Void> p3);\n\n"
			),
		);
		let close = checked.then(|| {
			let close = format!("{space}_close_{bare}");
			let close_ = private(&close);
			// The status stays the call's own while the isolate serves others meanwhile, as a
			// served call's does.
			let _ = write!(
				self.glue,
				"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {close}(home: *const c_void, own: *mut c_void, object: *mut c_void, raw: {raw}, free: bool, out: *mut i32) {{\n\tlet home = unsafe {{ &*home.cast::<runtime::Home>() }};\n\tlet status = unsafe {{\n\t\thome.serve(move || {{\n\t\t\tlet mut status = 0;\n\t\t\tcrate::{c}_drop(raw, &raw mut status);\n\t\t\tif status == 0 && free {{\n\t\t\t\tcrate::{c}_free(raw);\n\t\t\t}}\n\t\t\tstatus\n\t\t}})\n\t}};\n\tif status != 0 {{\n\t\tunsafe {{ *out = status }};\n\t}} else if free && !own.is_null() {{\n\t\tunsafe {{ runtime::disown(own, object) }};\n\t}}\n}}\n\n"
			);
			self.natives.insert(
				close.clone(),
				format!(
					"@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>, ffi.Pointer<ffi.Void>, ffi.Handle, ffi.Pointer<ffi.Void>, ffi.Bool, ffi.Pointer<ffi.Int32>)>(symbol: '{close}')\nexternal void {close_}(ffi.Pointer<ffi.Void> p0, ffi.Pointer<ffi.Void> p1, Object p2, ffi.Pointer<ffi.Void> p3, bool p4, ffi.Pointer<ffi.Int32> p5);\n\n"
				),
			);
			close_
		});
		Owned {
			own: own_,
			freeing: freeing_,
			free: free_,
			close,
		}
	}

	/// [`Self::served`] of a C function by name, parameters, and return.
	fn served_fn(&mut self, name: &str, params: Vec<Ty>, ret: Ty) -> String {
		let sig = Sig {
			name: name.to_string(),
			params: params.into_iter().map(|ty| (String::new(), ty)).collect(),
			ret,
		};
		self.served(&sig)
	}

	/// The `@Native` binding of a C function by name, parameters, and return.
	fn native_fn(&mut self, name: &str, params: Vec<Ty>, ret: Ty) -> String {
		let sig = Sig {
			name: name.to_string(),
			params: params.into_iter().map(|ty| (String::new(), ty)).collect(),
			ret,
		};
		self.native(&sig)
	}

	/// The C value of the Dart value `e` of type `ty`, whose C type is a scalar, nested in
	/// a call's arguments: an object held by the arena `a_`.
	fn lower(&mut self, ty: &Type, e: &str) -> String {
		match ty {
			Type::Bool | Type::F64 => e.to_string(),
			Type::F32 => format!("_n.f32Bits({e})"),
			Type::NonZero(int) => format!("_n.nonZero({})", self.lower(&Type::Int(*int), e)),
			Type::Int(int) => match int {
				Int::U8 => format!("_n.u8({e})"),
				Int::U16 => format!("_n.u16({e})"),
				Int::U32 => format!("_n.u32({e})"),
				Int::I8 => format!("_n.i8({e})"),
				Int::I16 => format!("_n.i16({e})"),
				Int::I32 => format!("_n.i32({e})"),
				Int::U64 => format!("{e}.bits"),
				Int::Usize => format!("_n.usize({e})"),
				Int::Isize => format!("_n.isize({e})"),
				Int::I64 => e.to_string(),
			},
			Type::Char => format!("_n.char({e})"),
			Type::Enum(_) => format!("{e}._c"),
			Type::Object(_) => format!("a_.hold({e})"),
			Type::Option(inner) if matches!(inner.base(), Type::Object(_)) => format!("a_.holdOrNull({e})"),
			Type::Box(inner) | Type::Ref(_, inner) => self.lower(inner, e),
			other => unreachable!("{other:?} is no scalar"),
		}
	}

	/// Statements writing the Dart value `e` of type `ty` into the C place `dst`, a
	/// struct's field or element: handles held by the arena `a_`, and bytes given to Rust
	/// where `take`, the Dart `bool` whether Rust takes the value, else viewed.
	fn write(&mut self, ty: &Type, e: &str, dst: &str, take: &str) -> String {
		let abi = self.abi;
		let c = abi.ty(ty);
		if Self::scalar(&c) {
			return format!("{dst} = {};", self.lower(ty, e));
		}
		match ty.base() {
			Type::Str | Type::String => format!("a_.string({dst}, {e}, {take});"),
			Type::Path | Type::PathBuf => format!("a_.bytes({dst}, {e}._bytes, {take});"),
			Type::Slice | Type::Vec | Type::Bytes => format!("a_.bytes({dst}, {e}, {take});"),
			Type::U128 => format!("_n.u128({dst}, {e});"),
			Type::Duration => format!("{e}._put({dst});"),
			Type::Array(n) => {
				self.written.insert(Abi::array_class(*n));
				format!("{e}._write({dst});")
			}
			// The sealed class of either family writes which it is, and each family itself.
			Type::SocketAddr(Family::Any) => format!("SocketAddr._put({dst}, {e});"),
			Type::IpAddr(Family::Any) => format!("IpAddr._put({dst}, {e});"),
			Type::SocketAddr(_) | Type::IpAddr(_) => format!("{e}._put({dst});"),
			Type::Option(inner) => match inner.base() {
				Type::Str | Type::String => format!("a_.string({dst}, {e}, {take});"),
				Type::Path | Type::PathBuf => format!("a_.bytes({dst}, {e}?._bytes, {take});"),
				Type::Slice | Type::Vec | Type::Bytes => format!("a_.bytes({dst}, {e}, {take});"),
				Type::List(..) => {
					let c = self.c_name(inner);
					let w = self.writer(&c, inner.base());
					format!("{w}({e}, {dst}, a_, {take});")
				}
				_ => {
					let c = abi.option(inner);
					let w = self.writer(&c, ty.base());
					format!("{w}({e}, {dst}, a_, {take});")
				}
			},
			Type::Record(_)
			| Type::Enum(_)
			| Type::List(..)
			| Type::Tuple(_)
			| Type::Range(_)
			| Type::Bound(_)
			| Type::Bounds(_) => {
				let c = self.c_name(ty.base());
				let w = self.writer(&c, ty.base());
				format!("{w}({e}, {dst}, a_, {take});")
			}
			other => unreachable!("{other:?} is never nested"),
		}
	}

	/// The C struct name of a type that crosses as one.
	fn c_name(&self, ty: &Type) -> String {
		match self.abi.ty(ty) {
			Ty::Struct(name) => name,
			other => unreachable!("{other:?} is no struct"),
		}
	}

	/// The function writing a Dart value of type `ty` into the C struct `c`.
	fn writer(&mut self, c: &str, ty: &Type) -> String {
		if let Some(name) = self.writers.get(ty) {
			return name.clone();
		}
		let mut name = format!("_w${}", mangle(c));
		for k in 2.. {
			if !self.helpers.contains_key(&name) {
				break;
			}
			name = format!("_w${}${k}", mangle(c));
		}
		self.writers.insert(ty.clone(), name.clone());
		self.helpers.insert(name.clone(), String::new());
		let class = self.struct_class(&Ty::Struct(c.to_string()));
		let dt = self.dt(ty);
		let abi = self.abi;
		let body = match ty {
			Type::Record(i) if abi.api.records[*i].opaque.is_some() => {
				let units = abi.api.records[*i].opaque.as_ref().map_or(0, |o| o.units());
				let lines: Vec<String> = (0..units).map(|k| format!("c.$opaque[{k}] = v._u{k};")).collect();
				lines.join("\n\t")
			}
			Type::Record(i) => {
				let record = &abi.api.records[*i];
				let mut lines = Vec::new();
				for (field, name) in record.fields.iter().zip(field_names(&record.fields)) {
					let e = format!("v.{name}");
					let dst = format!("c.{}", slot(&abi::escape(&field.name)));
					lines.push(self.write(&field.ty, &e, &dst, "t"));
				}
				lines.join("\n\t")
			}
			Type::Enum(i) => {
				let e = &abi.api.enums[*i];
				let class_name = self.class(c);
				let mut lines = Vec::new();
				for (n, variant) in e.variants.iter().enumerate() {
					let case = self.variant_class(&class_name, &variant.name);
					let mut body = vec![format!("c.$tag = {n};")];
					// A value of a variant the library's target lacks is refused as it crosses.
					if !abi.api.everywhere(variant.on) {
						let on = variant.on.0 & abi.api.all().0;
						body.insert(0, format!("_on(0x{on:x}, '{c}.{}');", variant.name));
					}
					let member = abi::escape(&abi::snake(&variant.name));
					for (field, name) in variant.fields.iter().zip(field_names(&variant.fields)) {
						let e = format!("v.{name}");
						let dst = format!("c.$u.{}.{}", slot(&member), slot(&abi::escape(&field.name)));
						body.push(self.write(&field.ty, &e, &dst, "t"));
					}
					lines.push(format!(
						"if (v is {case}) {{\n\t\t{}\n\t\treturn;\n\t}}",
						body.join("\n\t\t")
					));
				}
				lines.join("\n\t")
			}
			Type::Option(inner) => {
				let some = self.write(inner, "v", "c.$value", "t");
				format!("if (v == null) {{\n\t\tc.$some = false;\n\t}} else {{\n\t\tc.$some = true;\n\t\t{some}\n\t}}")
			}
			// A block is copied whole into the arena: `U64`s as the `int`s they are, and `f32`s
			// as their bits, which the C pointer holds.
			Type::List(_, elem) if ty.block().is_some() => {
				let nt = self.nt(&abi.ty(elem));
				let v = match **elem {
					Type::Int(Int::U64) => "v as List<int>",
					Type::F32 => "v.buffer.asUint32List(v.offsetInBytes, v.length)",
					_ => "v",
				};
				format!(
					"if (v == null) {{\n\t\tc.$ptr = ffi.nullptr;\n\t\tc.$len = 0;\n\t\tc.$owner = ffi.nullptr;\n\t\treturn;\n\t}}\n\tfinal p = a_<{nt}>(v.isEmpty ? 1 : v.length);\n\tp.asTypedList(v.length).setAll(0, {v});\n\tc.$ptr = p;\n\tc.$len = v.length;\n\tc.$owner = ffi.nullptr;"
				)
			}
			Type::List(_, elem) => {
				let elem_c = abi.ty(elem);
				let nt = self.nt(&elem_c);
				let item = match Self::scalar(&elem_c) {
					true => format!("p[i] = {};", self.lower(elem, "v[i]")),
					// An array's items are behind its pointer, which Rust only reads.
					false => self.write(elem, "v[i]", "p[i]", "false"),
				};
				format!(
					"if (v == null) {{\n\t\tc.$ptr = ffi.nullptr;\n\t\tc.$len = 0;\n\t\tc.$owner = ffi.nullptr;\n\t\treturn;\n\t}}\n\tfinal p = a_<{nt}>(v.isEmpty ? 1 : v.length);\n\tfor (var i = 0; i < v.length; i++) {{\n\t\t{item}\n\t}}\n\tc.$ptr = p;\n\tc.$len = v.length;\n\tc.$owner = ffi.nullptr;"
				)
			}
			Type::Tuple(elements) => {
				let mut lines = Vec::new();
				for (n, element) in elements.iter().enumerate() {
					lines.push(self.write(element, &format!("v.${}", n + 1), &format!("c.$${n}"), "t"));
				}
				lines.join("\n\t")
			}
			Type::Range(int) => format!(
				"c.$start = {};\n\tc.$end = {};",
				self.lower(&Type::Int(*int), "v.start"),
				self.lower(&Type::Int(*int), "v.end")
			),
			Type::Bound(int) => {
				let class_name = self.class(c);
				format!(
					"switch (v) {{\n\t\tcase {class_name}Unbounded():\n\t\t\tc.$kind = 0;\n\t\t\tc.$value = 0;\n\t\tcase {class_name}Included(:final value):\n\t\t\tc.$kind = 1;\n\t\t\tc.$value = {};\n\t\tcase {class_name}Excluded(:final value):\n\t\t\tc.$kind = 2;\n\t\t\tc.$value = {};\n\t}}",
					self.lower(&Type::Int(*int), "value"),
					self.lower(&Type::Int(*int), "value")
				)
			}
			Type::Bounds(int) => {
				let bound = Type::Bound(*int);
				format!(
					"{}\n\t{}",
					self.write(&bound, "v.start", "c.$start", "t"),
					self.write(&bound, "v.end", "c.$end", "t")
				)
			}
			other => unreachable!("{other:?} has no writer"),
		};
		let nullable = match ty {
			Type::List(..) => "?",
			_ => "",
		};
		self.helpers.insert(
			name.clone(),
			format!("void {name}({dt}{nullable} v, {class} c, _Arena a_, bool t) {{\n\t{body}\n}}\n\n"),
		);
		name
	}

	/// The function reading a Dart value of type `ty` out of the C struct `c`, owning what
	/// it holds.
	fn reader(&mut self, c: &str, ty: &Type) -> String {
		if let Some(name) = self.readers.get(ty) {
			return name.clone();
		}
		let mut name = format!("_r${}", mangle(c));
		for k in 2.. {
			if !self.helpers.contains_key(&name) {
				break;
			}
			name = format!("_r${}${k}", mangle(c));
		}
		self.readers.insert(ty.clone(), name.clone());
		self.helpers.insert(name.clone(), String::new());
		let abi = self.abi;
		let dt = self.dt(ty);
		let (param, body) = match ty {
			Type::Record(i) if abi.api.records[*i].opaque.is_some() => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let units = abi.api.records[*i].opaque.as_ref().map_or(0, |o| o.units());
				let words: Vec<String> = (0..units).map(|k| format!("c.$opaque[{k}]")).collect();
				(class, format!("{dt}._({})", words.join(", ")))
			}
			Type::Record(i) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let record = &abi.api.records[*i];
				let mut args = Vec::new();
				for (field, name) in record.fields.iter().zip(field_names(&record.fields)) {
					let value = self.lift(&field.ty, &format!("c.{}", slot(&abi::escape(&field.name))));
					args.push(match record.tuple {
						true => value,
						false => format!("{name}: {value}"),
					});
				}
				(class, format!("{dt}({})", args.join(", ")))
			}
			Type::Enum(i) if abi.api.enums[*i].plain() => {
				let e = &abi.api.enums[*i];
				let cases: Vec<String> = (e.variants.iter().zip(enum_values(e)))
					.map(|(v, value)| format!("\t\t{} => {dt}.{value},", v.discriminant))
					.collect();

				(
					"int".to_string(),
					format!(
						"switch (c) {{\n{}\n\t\t_ => throw StateError('{c}: an invalid value'),\n\t}}",
						cases.join("\n")
					),
				)
			}
			Type::Enum(i) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let e = &abi.api.enums[*i];
				let mut cases = Vec::new();
				for (n, variant) in e.variants.iter().enumerate() {
					let case = self.variant_class(&dt, &variant.name);
					let member = abi::escape(&abi::snake(&variant.name));
					let mut args = Vec::new();
					for (field, name) in variant.fields.iter().zip(field_names(&variant.fields)) {
						let value = self.lift(
							&field.ty,
							&format!("c.$u.{}.{}", slot(&member), slot(&abi::escape(&field.name))),
						);
						args.push(match variant.tuple {
							true => value,
							false => format!("{name}: {value}"),
						});
					}
					cases.push(format!("\t\t{n} => {case}({}),", args.join(", ")));
				}
				(
					class,
					format!(
						"switch (c.$tag) {{\n{}\n\t\t_ => throw StateError('{c}: an invalid tag'),\n\t}}",
						cases.join("\n")
					),
				)
			}
			Type::Option(inner) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let value = self.lift(inner, "c.$value");
				(class, format!("c.$some ? {value} : null"))
			}
			// A block is copied whole out of Rust's array.
			Type::List(..) if let Some(block) = ty.block() => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let list = typed_list(block);
				let free = self.native_fn(&format!("{c}_free"), vec![Ty::Struct(c.to_string())], Ty::Void);
				let cast = match block {
					Type::Int(Int::U64) => " as List<U64>",
					_ => "",
				};
				// A copy of typed data, which keeps each float's bits.
				let ptr = match block {
					Type::F32 => "c.$ptr.cast<ffi.Float>()",
					_ => "c.$ptr",
				};
				return self.finish_reader(
					name,
					format!(
						"{dt} {{NAME}}({class} c) {{\n\tfinal out = typed.{list}.fromList({ptr}.asTypedList(c.$len));\n\t{free}(c);\n\treturn out{cast};\n}}\n\n"
					),
				);
			}
			Type::List(_, elem) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let item = self.lift(elem, "p[i]");
				let free = self.native_fn(&format!("{c}_free"), vec![Ty::Struct(c.to_string())], Ty::Void);
				let elem_dt = self.dt(elem);
				return self.finish_reader(
					name,
					format!(
						"{dt} {{NAME}}({class} c) {{\n\tfinal p = c.$ptr;\n\tfinal n = c.$len;\n\tfinal out = <{elem_dt}>[for (var i = 0; i < n; i++) {item}];\n\tc.$len = 0;\n\t{free}(c);\n\treturn out;\n}}\n\n"
					),
				);
			}
			Type::Tuple(elements) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let lifted: Vec<String> = (elements.iter().enumerate())
					.map(|(n, e)| self.lift(e, &format!("c.$${n}")))
					.collect();
				let body = match lifted.len() {
					1 => format!("({},)", lifted[0]),
					_ => format!("({})", lifted.join(", ")),
				};
				(class, body)
			}
			Type::Range(int) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let int = Type::Int(*int);
				let (start, end) = (self.lift(&int, "c.$start"), self.lift(&int, "c.$end"));
				(class, format!("{dt}({start}, {end})"))
			}
			Type::Bound(int) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let value = self.lift(&Type::Int(*int), "c.$value");
				(
					class,
					format!(
						"switch (c.$kind) {{\n\t\t0 => const {dt}Unbounded(),\n\t\t1 => {dt}Included({value}),\n\t\t2 => {dt}Excluded({value}),\n\t\t_ => throw StateError('{c}: an invalid kind'),\n\t}}"
					),
				)
			}
			Type::Bounds(int) => {
				let class = self.struct_class(&Ty::Struct(c.to_string()));
				let bound = Type::Bound(*int);
				let start = self.lift(&bound, "c.$start");
				let end = self.lift(&bound, "c.$end");
				(class, format!("{dt}({start}, {end})"))
			}
			other => unreachable!("{other:?} has no reader"),
		};
		self.finish_reader(name, format!("{dt} {{NAME}}({param} c) => {body};\n\n"))
	}

	fn finish_reader(&mut self, name: String, code: String) -> String {
		self.helpers.insert(name.clone(), code.replace("{NAME}", &name));
		name
	}

	/// The Dart value of the C value `e` of type `ty`, which it takes ownership of: a
	/// scalar, or a struct's view. `e` may be read more than once.
	fn lift(&mut self, ty: &Type, e: &str) -> String {
		let abi = self.abi;
		match ty {
			Type::Str | Type::String => format!("_n.string({e})"),
			Type::Path | Type::PathBuf => format!("PathBuf._(_n.bytes({e}))"),
			Type::Slice | Type::Vec | Type::Bytes => format!("_n.bytes({e})"),
			Type::Int(Int::U64 | Int::Usize) | Type::NonZero(Int::U64 | Int::Usize) => format!("U64.fromBits({e})"),
			Type::F32 => format!("_n.f32Of({e})"),
			Type::Bool | Type::F64 | Type::Int(_) | Type::NonZero(_) => e.to_string(),
			Type::Char => format!("_n.scalar({e})"),
			Type::U128 => format!("_n.u128Of({e})"),
			Type::Duration => format!("RustDuration._of({e})"),
			Type::Array(n) => format!("{}._of({e})", Abi::array_class(*n)),
			Type::SocketAddr(family) => format!("{}._of({e})", addr_class(true, *family)),
			Type::IpAddr(family) => format!("{}._of({e})", addr_class(false, *family)),
			Type::Option(inner) => match inner.base() {
				Type::Object(_) => {
					let dt = self.dt(inner);
					format!("({e} == ffi.nullptr ? null : {dt}._({e}))")
				}
				Type::Str | Type::String => format!("_n.stringOrNull({e})"),
				Type::Path | Type::PathBuf => format!("({e}.ptr == ffi.nullptr ? null : PathBuf._(_n.bytes({e})))"),
				Type::Slice | Type::Vec | Type::Bytes => format!("_n.bytesOrNull({e})"),
				Type::List(..) => {
					let lifted = self.lift(inner, e);
					format!("({e}.$ptr == ffi.nullptr ? null : {lifted})")
				}
				_ => {
					let c = abi.option(inner);
					let r = self.reader(&c, &Type::Option(Box::new(inner.base().clone())));
					format!("{r}({e})")
				}
			},
			Type::Box(inner) | Type::Ref(_, inner) => self.lift(inner, e),
			Type::Object(_) => format!("{}._({e})", self.dt(ty)),
			Type::Waker => format!("Waker._({e})"),
			Type::Record(_)
			| Type::Enum(_)
			| Type::List(..)
			| Type::Tuple(_)
			| Type::Range(_)
			| Type::Bound(_)
			| Type::Bounds(_) => {
				let c = match abi.ty(ty) {
					Ty::Struct(name) | Ty::Enum(name) => name,
					other => unreachable!("{other:?} is no value"),
				};
				let r = self.reader(&c, ty);
				format!("{r}({e})")
			}
			other => unreachable!("{other:?} is never returned"),
		}
	}

	/// The Dart value of the C value `e` of type `ty`, which C lends a callback or closure
	/// for its call: bytes and text copied, and a scalar as it is.
	fn lift_lent(&mut self, ty: &Type, e: &str) -> String {
		match ty {
			Type::Str | Type::String => format!("_n.stringLent({e})"),
			Type::Path | Type::PathBuf => format!("PathBuf._(_n.bytesLent({e}))"),
			Type::Slice | Type::Vec | Type::Bytes => format!("_n.bytesLent({e})"),
			Type::Option(inner) => match inner.base() {
				Type::Str | Type::String => format!("_n.stringLentOrNull({e})"),
				Type::Path | Type::PathBuf => {
					format!("({e}.ptr == ffi.nullptr ? null : PathBuf._(_n.bytesLent({e})))")
				}
				Type::Slice | Type::Vec | Type::Bytes => format!("_n.bytesLentOrNull({e})"),
				_ => self.lift(ty, e),
			},
			Type::Ref(_, inner) => self.lift_lent(inner, e),
			ty => self.lift(ty, e),
		}
	}

	fn items(&mut self) {
		let abi = self.abi;
		let api = abi.api;
		for (c, ty) in &abi.ranges.clone() {
			self.range(c, ty);
		}
		for (c, n) in &abi.arrays {
			let name = Abi::array_class(*n);
			let s = self.struct_class(&Ty::Struct(c.clone()));
			let of = format!(
				"{name}._(this._bytes);\n\n\tstatic {name} _of({s} c) => {name}._(typed.Uint8List.fromList([for (var i = 0; i < {n}; i++) c.items[i]]));\n\n\t"
			);
			let write = format!(
				"void _write({s} c) {{\n\t\tfor (var i = 0; i < {n}; i++) {{\n\t\t\tc.items[i] = _bytes[i];\n\t\t}}\n\t}}\n\n\t"
			);
			self.arrays.insert(name.clone(), (of.clone(), write.clone()));
			let _ = write!(
				self.types,
				"/// A Rust `[u8; {n}]`: exactly {n} bytes, copied in and out, so it never changes.\nfinal class {name} {{\n\tfinal typed.Uint8List _bytes;\n\n\t/// A copy of [bytes], which must hold exactly {n}, each from 0 through 255.\n\t{name}(List<int> bytes) : _bytes = typed.Uint8List.fromList(bytes) {{\n\t\tif (bytes.length != {n} || bytes.any((b) => b < 0 || b > 255)) {{\n\t\t\tthrow ArgumentError.value(bytes, 'bytes', 'is no [u8; {n}]');\n\t\t}}\n\t}}\n\n\tint operator [](int index) => _bytes[index];\n\n\ttyped.Uint8List toBytes() => typed.Uint8List.fromList(_bytes);\n\n\t{of}{write}@override\n\tbool operator ==(Object other) => other is {name} && _equal(other._bytes);\n\n\tbool _equal(typed.Uint8List other) {{\n\t\tfor (var i = 0; i < {n}; i++) {{\n\t\t\tif (other[i] != _bytes[i]) return false;\n\t\t}}\n\t\treturn true;\n\t}}\n\n\t@override\n\tint get hashCode => Object.hashAll(_bytes);\n\n\t@override\n\tString toString() => _bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join();\n}}\n\n"
			);
		}
		for constant in &api.constants {
			let name = member(&abi::snake(self.bare(&abi.name(&constant.path))), &[]);
			let name = self.claim(&name, format!("const `{}`", constant.path.join("::")));
			self.locked
				.insert(format!("const {}", constant.path.join("::")), name.clone());
			let value = match constant.value {
				Literal::Bool(b) => b.to_string(),
				Literal::Int(Int::U64 | Int::Usize, v) => format!("U64.fromBits({})", int_literal(v)),
				Literal::Int(_, v) => int_literal(v),
			};
			let _ = write!(
				self.top,
				"{}const {name} = {value};\n\n",
				docs(&constant.docs, constant.deprecated.as_deref(), "")
			);
		}
		for i in 0..api.records.len() {
			self.record(i);
		}
		for i in 0..api.enums.len() {
			self.enumeration(i);
		}
		for i in 0..api.objects.len() {
			self.object(i);
		}
		for i in 0..api.callbacks.len() {
			self.callback(i);
		}
		let functions: Vec<&Function> = api.functions.iter().collect();
		let named = self.names(&functions, (None, None), |_, name| {
			TAKEN.contains(&name) || self.names.contains_key(name)
		});
		for (function, name) in api.functions.iter().zip(&named) {
			if let Some((code, _)) = self.function(function, (None, None), name, &[]) {
				self.top.push_str(&code);
				self.top.push('\n');
			}
		}
		for (name, ty) in &self.thrown.clone() {
			let dt = self.dt(ty);
			let _ = write!(
				self.types,
				"/// A `{dt}` Rust returned as an error.\nfinal class {name} implements Exception {{\n\tfinal {dt} error;\n\n\tconst {name}(this.error);\n\n\t@override\n\tString toString() => '$error';\n}}\n\n"
			);
		}
		// Each exported type thrown as an error implements `Exception`, and each ordered by
		// `Ord` `Comparable`, which its declaration names once all are known.
		let names: BTreeSet<String> = self.conforms.union(&self.comparables).cloned().collect();
		for name in &names {
			let interfaces: Vec<String> = [
				self.conforms.contains(name).then(|| "Exception".to_string()),
				self.comparables.contains(name).then(|| format!("Comparable<{name}>")),
			]
			.into_iter()
			.flatten()
			.collect();
			for marker in [format!("class {name} "), format!("enum {name} ")] {
				let at = self.types.find(&format!("{marker}{{"));
				let at = at.or_else(|| self.types.find(&format!("{marker}extends _Object {{")));
				if let Some(at) = at {
					let open = at + self.types[at..].find('{').expect("a body");
					self.types
						.insert_str(open, &format!("implements {} ", interfaces.join(", ")));
					break;
				}
			}
		}
	}

	/// A class for a range or pair of bounds, or a sealed class for a bound.
	fn range(&mut self, c: &str, ty: &Type) {
		let name = self.class(c);
		let code = match ty {
			Type::Range(int) => {
				let dt = int_dt(*int);
				// A `U64` prints unsigned only through its own method.
				let (start, end) = match dt {
					"U64" => ("${start.toRadixString(10)}", "${end.toRadixString(10)}"),
					_ => ("$start", "$end"),
				};
				format!(
					"/// Rust's `Range`: from `start`, up to but not including `end`.\nfinal class {name} {{\n\tfinal {dt} start;\n\tfinal {dt} end;\n\n\tconst {name}(this.start, this.end);\n\n\t@override\n\tbool operator ==(Object other) => other is {name} && other.start == start && other.end == end;\n\n\t@override\n\tint get hashCode => Object.hash(start, end);\n\n\t@override\n\tString toString() => '{start}..{end}';\n}}\n\n"
				)
			}
			Type::Bound(int) => {
				let dt = int_dt(*int);
				format!(
					"/// Rust's `Bound`: what a range's start or end bound does.\nsealed class {name} {{\n\tconst {name}();\n}}\n\n\
					final class {name}Unbounded extends {name} {{\n\tconst {name}Unbounded();\n\n\t@override\n\tbool operator ==(Object other) => other is {name}Unbounded;\n\n\t@override\n\tint get hashCode => 0;\n}}\n\n\
					final class {name}Included extends {name} {{\n\tfinal {dt} value;\n\n\tconst {name}Included(this.value);\n\n\t@override\n\tbool operator ==(Object other) => other is {name}Included && other.value == value;\n\n\t@override\n\tint get hashCode => Object.hash(1, value);\n}}\n\n\
					final class {name}Excluded extends {name} {{\n\tfinal {dt} value;\n\n\tconst {name}Excluded(this.value);\n\n\t@override\n\tbool operator ==(Object other) => other is {name}Excluded && other.value == value;\n\n\t@override\n\tint get hashCode => Object.hash(2, value);\n}}\n\n"
				)
			}
			Type::Bounds(int) => {
				let bound = self.class(&self.abi.range(&Type::Bound(*int)));
				format!(
					"/// A range's start and end bounds, as Rust's `RangeBounds` gives them.\nfinal class {name} {{\n\tfinal {bound} start;\n\tfinal {bound} end;\n\n\tconst {name}(this.start, this.end);\n\n\t@override\n\tbool operator ==(Object other) => other is {name} && other.start == start && other.end == end;\n\n\t@override\n\tint get hashCode => Object.hash(start, end);\n}}\n\n"
				)
			}
			other => unreachable!("{other:?} is no range"),
		};
		self.types.push_str(&code);
	}

	/// The members a type's std traits export: `Display` is `toString()`, `Debug`
	/// `debug()`, `PartialEq` `==` and with `Hash` `hashCode`, `Default` a constructor
	/// `default_()`, and `Clone` an object's `clone()`.
	fn traits(&mut self, ty: &Type, c: &str, t: crate::ir::Traits, body: &mut Vec<String>, members: &mut Vec<String>) {
		let name = self.dt(ty);
		let object = matches!(ty, Type::Object(_));
		let c_ty = self.abi.ty(ty);
		// The receiver, lowered into the arena or held by it.
		let this = |g: &mut Self, e: &str| -> (String, String) {
			match object {
				true => (String::new(), format!("a_.hold({e})")),
				false => g.value_ptr(ty, &c_ty, e),
			}
		};
		let recv = match object {
			true => Ty::Handle {
				name: c.to_string(),
				constant: true,
			},
			false => Ty::In(Box::new(c_ty.clone())),
		};
		// An object whose run-time checks may fail passes each its status, which throws.
		let status = self.abi.traits_status_of(ty);
		let params = |args: Vec<Ty>| -> Vec<Ty> {
			let mut args = args;
			if status {
				args.push(Ty::Out(Box::new(Ty::Status)));
			}
			args
		};
		// The statements calling `f` with `args`, returning what `lift` makes of its result.
		let call = |f: &str, args: &str, what: &str, lift: &dyn Fn(&str) -> String| -> String {
			match status {
				true => format!(
					"final s_ = _n.status;\n\t\t\tfinal r_ = {f}({args}, s_);\n\t\t\t_status(s_.value, '{what}');\n\t\t\treturn {};",
					lift("r_")
				),
				false => format!("return {};", lift(&format!("{f}({args})"))),
			}
		};
		// A call in the arena `a_`, which a leaf call of an object's needs none of.
		let arena = |leaf: bool, code: String| match (leaf && object, held_only(&code)) {
			(true, _) => code.replace("\n\t\t\t", "\n\t\t"),
			(false, Some(code)) => {
				format!("final h_ = _hold();\n\t\ttry {{\n\t\t\t{code}\n\t\t}} finally {{\n\t\t\t_release();\n\t\t}}")
			}
			(false, None) => {
				format!("final a_ = _Arena();\n\t\ttry {{\n\t\t\t{code}\n\t\t}} finally {{\n\t\t\ta_.end();\n\t\t}}")
			}
		};
		// `code(leaf)`'s call as a leaf call where the library reports `_proven`, else served.
		let proven = |g: &mut Self, leaf: bool, code: &dyn Fn(&mut Self, bool) -> String| match leaf {
			true => {
				let (proven, served) = (code(g, true), code(g, false));
				let nested = |code: String| code.replace("\n\t\t", "\n\t\t\t");
				format!(
					"if (_proven) {{\n\t\t\t{}\n\t\t}} else {{\n\t\t\t{}\n\t\t}}",
					nested(proven),
					nested(served)
				)
			}
			false => code(g, false),
		};
		let shown = self.abi.api.shown(ty);
		let member = |g: &mut Self, t: &str, name: &str| crate::lock::trait_member(&mut g.locked, shown, t, name);
		if t.display {
			member(self, "Display", "toString");
			members.push("toString".to_string());
			let what = format!("{c}_to_string");
			let f = self.served_fn(&what, params(vec![recv.clone()]), Ty::Buf);
			let (pre, arg) = this(self, "this");
			let call = arena(
				false,
				format!(
					"{pre}{}",
					call(&f, &format!("_n.home, {arg}"), &what, &|r| format!("_n.string({r})"))
				),
			);
			body.push(format!("\t@override\n\tString toString() {{\n\t\t{call}\n\t}}\n"));
		}
		if t.debug {
			member(self, "Debug", "debug");
			members.push("debug".to_string());
			let what = format!("{c}_debug");
			let f = self.served_fn(&what, params(vec![recv.clone()]), Ty::Buf);
			let (pre, arg) = this(self, "this");
			let call = arena(
				false,
				format!(
					"{pre}{}",
					call(&f, &format!("_n.home, {arg}"), &what, &|r| format!("_n.string({r})"))
				),
			);
			body.push(format!(
				"\t/// Rust's `Debug` form.\n\tString debug() {{\n\t\t{call}\n\t}}\n"
			));
		}
		if object && t.clone {
			member(self, "Clone", "clone");
			members.push("clone".to_string());
			let what = format!("{c}_clone");
			let f = self.served_fn(&what, params(vec![recv.clone()]), recv.clone());
			let call = arena(
				false,
				call(&f, "_n.home, a_.hold(this)", &what, &|r| format!("{name}._({r})")),
			);
			body.push(format!(
				"\t/// A new handle to a clone.\n\t{name} clone() {{\n\t\t{call}\n\t}}\n"
			));
		}
		// A Dart enum compares by identity, as a plain Rust enum does, and may not override `==`.
		let plain = matches!(ty, Type::Enum(i) if self.abi.api.enums[*i].plain());
		if t.eq && !plain {
			members.push("hashCode".to_string());
			let what = format!("{c}_eq");
			let sig = Sig {
				name: what.clone(),
				params: params(vec![recv.clone(), recv.clone()])
					.into_iter()
					.map(|ty| (String::new(), ty))
					.collect(),
				ret: Ty::Bool,
			};
			// A leaf call, as `Abi::leaf_trait` proves, takes no home and holds no handle. It
			// stalls the isolate group's GC until it returns, so it walks no length the caller
			// picks.
			let leaf = self.abi.leaf_trait(ty, false) && t.leaf_sized;
			let eq = proven(self, leaf, &|g, leaf| {
				let (f, (pa, a), (pb, b)) = match (leaf, object) {
					// `other` bound to its own type, since Dart keeps a value alive to the end
					// of its scope only for a variable declared `Finalizable`, which a
					// promoted `Object` is not: the `_on` check before the native may run a GC.
					(true, true) => (
						g.leaf(&sig),
						(String::new(), "_live".to_string()),
						(format!("final {name} o_ = other;\n\t\t"), "o_._live".to_string()),
					),
					(true, false) => (g.leaf(&sig), this(g, "this"), this(g, "other")),
					(false, _) => (g.served(&sig), this(g, "this"), this(g, "other")),
				};
				let home = if leaf { "" } else { "_n.home, " };
				arena(
					leaf,
					format!(
						"{pa}{pb}{}",
						call(&f, &format!("{home}{a}, {b}"), &what, &|r| r.to_string())
					),
				)
			});
			body.push(format!(
				"\t/// Rust's `PartialEq`.\n\t@override\n\tbool operator ==(Object other) {{\n\t\tif (other is! {name}) return false;\n\t\t{eq}\n\t}}\n"
			));
			// Equal values hash alike only by Rust's own `Hash`, which a type without one lacks.
			if t.hash {
				member(self, "Hash", "hashCode");
				let what = format!("{c}_hash");
				let sig = Sig {
					name: what.clone(),
					params: params(vec![recv.clone()])
						.into_iter()
						.map(|ty| (String::new(), ty))
						.collect(),
					ret: Ty::Int(Int::U64),
				};
				let leaf = self.abi.leaf_trait(ty, true) && t.leaf_sized;
				let hash = proven(self, leaf, &|g, leaf| {
					let (f, (pre, arg)) = match (leaf, object) {
						(true, true) => (g.leaf(&sig), (String::new(), "_live".to_string())),
						(true, false) => (g.leaf(&sig), this(g, "this")),
						(false, _) => (g.served(&sig), this(g, "this")),
					};
					let home = if leaf { "" } else { "_n.home, " };
					arena(
						leaf,
						format!("{pre}{}", call(&f, &format!("{home}{arg}"), &what, &|r| r.to_string())),
					)
				});
				body.push(format!(
					"\t/// Rust's `Hash`.\n\t@override\n\tint get hashCode {{\n\t\t{hash}\n\t}}\n"
				));
			} else {
				body.push(format!(
					"\t/// Rust has no `Hash` for it, so it keys no `Set` or `Map`.\n\t@override\n\tint get hashCode => throw UnsupportedError('{name} has no Rust Hash');\n"
				));
			}
		}
		// Rust's `partial_cmp`, -1, 0, 1, or 2 when unordered, behind `partialCompareTo` and
		// `<` and its kin, as Rust's are; with `Ord`, its `cmp` as `Comparable`'s `compareTo`.
		for &(_, suffix) in crate::c::comparisons(t.ord) {
			let what = format!("{c}_{suffix}");
			let f = self.served_fn(&what, params(vec![recv.clone(), recv.clone()]), Ty::Int(Int::I8));
			let (pa, a) = this(self, "this");
			let (pb, b) = this(self, "other");
			let compare = arena(
				false,
				format!(
					"{pa}{pb}{}",
					call(&f, &format!("_n.home, {a}, {b}"), &what, &|r| r.to_string())
				),
			);
			match suffix {
				"cmp" => {
					member(self, "Ord", "compareTo");
					members.push("compareTo".to_string());
					self.comparables.insert(name.clone());
					body.push(format!(
						"\t/// Rust's `Ord`: -1, 0, or 1 as this is less than, equal to, or greater than [other].\n\t@override\n\tint compareTo({name} other) {{\n\t\t{compare}\n\t}}\n"
					));
				}
				_ => {
					member(self, "PartialOrd", "partialCompareTo");
					members.push("partialCompareTo".to_string());
					members.push("_partialCmp".to_string());
					body.push(format!(
						"\t/// Rust's `partial_cmp`: -1, 0, 1, or 2 when unordered.\n\tint _partialCmp({name} other) {{\n\t\t{compare}\n\t}}\n\n\t/// Rust's `PartialOrd`: -1, 0, or 1 as this is less than, equal to, or greater than [other], or null when the two are unordered.\n\tint? partialCompareTo({name} other) {{\n\t\tfinal c_ = _partialCmp(other);\n\t\treturn c_ == 2 ? null : c_;\n\t}}\n\n\tbool operator <({name} other) => _partialCmp(other) == -1;\n\n\tbool operator <=({name} other) {{\n\t\tfinal c_ = _partialCmp(other);\n\t\treturn c_ == -1 || c_ == 0;\n\t}}\n\n\tbool operator >({name} other) => _partialCmp(other) == 1;\n\n\tbool operator >=({name} other) {{\n\t\tfinal c_ = _partialCmp(other);\n\t\treturn c_ == 0 || c_ == 1;\n\t}}\n"
					));
				}
			}
		}
		if t.default {
			member(self, "Default", "default_");
			members.push("new default_".to_string());
			let ret = match object {
				true => Ty::Handle {
					name: c.to_string(),
					constant: false,
				},
				false => c_ty.clone(),
			};
			// A value that is not `Send` is made on a home, which its calls then run on.
			let homed = match ty {
				Type::Object(i) if !self.abi.api.objects[*i].send => "_homed",
				_ => "",
			};
			let f = self.served_fn(&format!("{c}_default{homed}"), vec![], ret);
			let value = match object {
				true => format!("{name}._({f}(_n.home))"),
				false => self.lift(ty, &format!("{f}(_n.home)")),
			};
			body.push(format!(
				"\t/// Rust's `Default`.\n\tfactory {name}.default_() {{\n\t\t_n.check();\n\t\treturn {value};\n\t}}\n"
			));
		}
	}

	/// A value's receiver lowered into the arena: the statements, and the pointer C reads.
	fn value_ptr(&mut self, ty: &Type, c: &Ty, e: &str) -> (String, String) {
		let s = self.fresh("s");
		let nt = self.nt(c);
		let write = match Self::scalar(c) {
			true => format!("{s}.value = {};", self.lower(ty, e)),
			// Read through a pointer, so Rust takes nothing of it.
			false => self.write(ty, e, &format!("{s}.ref"), "false"),
		};
		(format!("final {s} = a_<{nt}>();\n\t\t\t{write}\n\t\t\t"), s)
	}

	/// Each method of a type, as members of its Dart type.
	fn methods(
		&mut self,
		methods: &[Function],
		owner: &Ty,
		ty: &Type,
		body: &mut Vec<String>,
		members: &mut Vec<String>,
	) {
		let functions: Vec<&Function> = methods.iter().collect();
		let owned = (Some(owner), Some(ty));
		let declared: Vec<Option<&str>> = (methods.iter()).map(|method| self.declared(method, owned).2).collect();
		// Each name a member its type declares already takes, as [`collides`] reads it.
		let named = self.names(&functions, owned, |k, name| match declared[k] {
			Some(kind) => members.iter().any(|other| collides(other, &format!("{kind}{name}"))),
			None => false,
		});
		for (method, name) in methods.iter().zip(&named) {
			if let Some((code, member)) = self.function(method, owned, name, members) {
				members.push(member);
				body.push(indent(&code));
			}
		}
		// After every method, so a Rust method of its name keeps it.
		for (next, item, key, deprecated) in std::mem::take(&mut self.streams) {
			let mut name = "asStream".to_string();
			while members.iter().any(|other| collides(other, &name)) {
				name.push('_');
			}
			self.locked.insert(format!("{key} / stream"), name.clone());
			members.push(name.clone());
			let text = format!(
				"Each item [{next}] yields, as a stream, which calls [{next}] only while its listener listens and is not paused, one call at a time.\n\nIt reads nothing ahead, so a loop left early leaves the next item to the next call. A pause or cancel drops the pending call's future in Rust, as dropping it there would, so it loses an item only where [{next}] does when dropped. `None` ends it, and an error is its last event before it ends."
			);
			body.push(indent(&format!(
				"{}dasync.Stream<{item}> {name}() => _stream(_{next}Pull);\n",
				docs(&text, deprecated.as_deref(), "")
			)));
		}
	}

	fn record(&mut self, i: usize) {
		let abi = self.abi;
		let record = &abi.api.records[i];
		self.on = record.on;
		let c = abi.record(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&record.path, &record.args);
		let ty = Type::Record(i);
		if let Some(opaque) = &record.opaque {
			// Its bytes, in the units of its C struct, each an `int` of the same bits.
			let units: Vec<String> = (0..opaque.units()).map(|k| format!("_u{k}")).collect();
			let mut members = Vec::new();
			let mut body = vec![format!(
				"\tconst {name}._({});\n\n{}",
				units.iter().map(|u| format!("this.{u}")).collect::<Vec<_>>().join(", "),
				units.iter().map(|u| format!("\tfinal int {u};\n")).collect::<String>()
			)];
			self.traits(&ty, &c, record.traits, &mut body, &mut members);
			self.methods(&record.methods, &Ty::Struct(c.clone()), &ty, &mut body, &mut members);
			self.check_members(&rust, &mut members);
			let text = format!(
				"{rust}\n\nIts bytes, which only Rust makes or reads: copy it, but never make one.\n\n{}",
				record.docs
			);
			let _ = write!(
				self.types,
				"{}final class {name} {{\n{}}}\n\n",
				docs(text.trim(), record.deprecated.as_deref(), ""),
				body.join("\n")
			);
			return;
		}
		let fields: Vec<(String, String)> = (record.fields.iter().zip(field_names(&record.fields)))
			.map(|(f, name)| {
				crate::lock::field(&mut self.locked, &record.shown, &f.name, &name);
				(name, self.dt(&f.ty))
			})
			.collect();
		let mut members: Vec<String> = fields.iter().map(|(f, _)| f.clone()).collect();
		let mut body: Vec<String> = (record.fields.iter().zip(&fields))
			.map(|(f, (name, dt))| format!("{}\tfinal {dt} {name};\n", docs(&f.docs, f.deprecated.as_deref(), "\t")))
			.collect();
		let params: Vec<String> = match record.tuple {
			true => fields.iter().map(|(f, _)| format!("this.{f}")).collect(),
			false => fields.iter().map(|(f, _)| format!("required this.{f}")).collect(),
		};
		let params = match (record.tuple, params.is_empty()) {
			(_, true) => String::new(),
			(true, false) => params.join(", "),
			(false, false) => format!("{{{}}}", params.join(", ")),
		};
		body.push(format!("\tconst {name}({params});\n"));
		self.traits(&ty, &c, record.traits, &mut body, &mut members);
		self.methods(&record.methods, &Ty::Struct(c.clone()), &ty, &mut body, &mut members);
		self.check_members(&rust, &mut members);
		let text = format!("`{rust}`\n\n{}", record.docs);
		let _ = write!(
			self.types,
			"{}final class {name} {{\n{}}}\n\n",
			docs(text.trim(), record.deprecated.as_deref(), ""),
			body.join("\n")
		);
	}

	/// Check a type's members: none is one Dart gives every object unless it overrides it,
	/// and no two share a name.
	fn check_members(&mut self, rust: &str, members: &mut [String]) {
		self.distinct(rust, members);
	}

	fn enumeration(&mut self, i: usize) {
		let abi = self.abi;
		let e = &abi.api.enums[i];
		self.on = e.on;
		let c = abi.enumeration(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&e.path, &e.args);
		let ty = Type::Enum(i);
		let mut members = Vec::new();
		let mut body = Vec::new();
		let text = format!("`{rust}`\n\n{}", e.docs);
		if e.plain() {
			// A value of a variant the library's target lacks is refused as it crosses, when
			// its C value is read.
			let limited = e.variants.iter().any(|v| !abi.api.everywhere(v.on));
			let values: Vec<String> = (e.variants.iter().zip(enum_values(e)))
				.map(|(v, value)| {
					crate::lock::variant(&mut self.locked, &e.shown, &v.name, &value);
					members.push(format!("static {value}"));
					match limited {
						true => format!(
							"\t{value}({}, 0x{:x}, '{c}.{}')",
							v.discriminant,
							v.on.0 & abi.api.all().0,
							v.name
						),
						false => format!("\t{value}({})", v.discriminant),
					}
				})
				.collect();
			body.push(match limited {
				true => format!(
					"{};\n\n\tconst {name}(this._value, this._targets, this._what);\n\n\tfinal int _value;\n\tfinal int _targets;\n\tfinal String _what;\n\n\t/// Its C value.\n\tint get _c {{\n\t\t_on(_targets, _what);\n\t\treturn _value;\n\t}}\n",
					values.join(",\n")
				),
				false => format!(
					"{};\n\n\tconst {name}(this._c);\n\n\t/// Its C value.\n\tfinal int _c;\n",
					values.join(",\n")
				),
			});
			self.traits(&ty, &c, e.traits, &mut body, &mut members);
			// What every Dart enum has, which a method takes a trailing `_` beside.
			members.extend(["index".to_string(), "static values".to_string()]);
			let owner = abi.ty(&ty);
			self.methods(&e.methods, &owner, &ty, &mut body, &mut members);
			self.distinct(&rust, &members);
			let _ = write!(
				self.types,
				"{}enum {name} {{\n{}}}\n\n",
				docs(text.trim(), e.deprecated.as_deref(), ""),
				body.join("\n")
			);
			return;
		}
		body.push(format!("\tconst {name}();\n"));
		self.traits(&ty, &c, e.traits, &mut body, &mut members);
		// Each variant's fields, which its class declares beside the methods it inherits, so a
		// method takes a trailing `_` beside one, as a record's does beside its own.
		let fields: BTreeSet<String> = (e.variants.iter()).flat_map(|v| field_names(&v.fields)).collect();
		members.extend(fields);
		let owner = abi.ty(&ty);
		self.methods(&e.methods, &owner, &ty, &mut body, &mut members);
		self.distinct(&rust, &members);
		let _ = write!(
			self.types,
			"{}sealed class {name} {{\n{}}}\n\n",
			docs(text.trim(), e.deprecated.as_deref(), ""),
			body.join("\n")
		);
		for (k, variant) in e.variants.iter().enumerate() {
			let case = self.variant_class(&name, &variant.name);
			let fields: Vec<(String, String)> = (variant.fields.iter().zip(field_names(&variant.fields)))
				.map(|(f, name)| {
					crate::lock::variant_field(&mut self.locked, &e.shown, &variant.name, &f.name, &name);
					(name, self.dt(&f.ty))
				})
				.collect();
			let decls: String = fields.iter().map(|(f, dt)| format!("\tfinal {dt} {f};\n")).collect();
			let params: Vec<String> = match variant.tuple {
				true => fields.iter().map(|(f, _)| format!("this.{f}")).collect(),
				false => fields.iter().map(|(f, _)| format!("required this.{f}")).collect(),
			};
			let params = match (variant.tuple, params.is_empty()) {
				(_, true) => String::new(),
				(true, false) => params.join(", "),
				(false, false) => format!("{{{}}}", params.join(", ")),
			};
			let sep = if decls.is_empty() { "" } else { "\n" };
			// Without Rust's `PartialEq`, a variant compares by its fields, as a value does.
			let eq = match e.traits.eq {
				true => String::new(),
				false if fields.is_empty() => format!(
					"\n\t@override\n\tbool operator ==(Object other) => other is {case};\n\n\t@override\n\tint get hashCode => {k};\n"
				),
				false => String::new(),
			};
			let _ = write!(
				self.types,
				"/// `{rust}::{}`.\nfinal class {case} extends {name} {{\n{decls}{sep}\tconst {case}({params});\n{eq}}}\n\n",
				variant.name
			);
		}
	}

	fn object(&mut self, i: usize) {
		let abi = self.abi;
		let object = &abi.api.objects[i];
		self.on = object.on;
		let c = abi.object(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&object.path, &object.args);
		let ty = Type::Object(i);
		let owner = Ty::Handle {
			name: c.clone(),
			constant: false,
		};
		let mut members = vec!["close".to_string()];
		crate::lock::trait_member(&mut self.locked, &object.shown, "Drop", "close");
		let checked = abi.checked[i];
		let Owned {
			own,
			freeing,
			free,
			close,
		} = self.owned(&c, &owner, checked);
		let close = match close {
			Some(close) => format!(
				"\n\t@override\n\tvoid _close(bool free) {{\n\t\tfinal s_ = _n.status;\n\t\t{close}(_n.home, _own, this, _raw, free, s_);\n\t\t_status(s_.value, '{c}_drop');\n\t}}\n"
			),
			None => String::new(),
		};
		let close = format!("\t@override\n\tvoid _free() => {free}(_n.home, _own, this, _raw);\n{close}");
		let owned_ctor = format!(
			"\t{name}._(ffi.Pointer<ffi.Void> raw) : super(raw, true, {checked}) {{\n\t\t_n.nursery.push(this, ffi.Native.addressOf<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>({freeing}).cast());\n\t}}\n\n"
		);
		let lent_ctor = format!("\t{name}._lent(ffi.Pointer<ffi.Void> raw) : super(raw, false, {checked});\n\n");
		let body = format!(
			"{owned_ctor}{lent_ctor}\t@override\n\tString get _type => '{name}';\n\n\t@override\n\tvoid _register() => _own = {own}(this, _raw);\n\n{close}"
		);
		self.constructors
			.insert(name.clone(), (owned_ctor, lent_ctor, freeing.clone()));
		let mut body = vec![body];
		self.traits(&ty, &c, object.traits, &mut body, &mut members);
		self.methods(&object.methods, &owner, &ty, &mut body, &mut members);
		self.distinct(&rust, &members);
		let text = format!("`{rust}`\n\n{}", object.docs);
		let _ = write!(
			self.types,
			"{}final class {name} extends _Object {{\n{}}}\n\n",
			docs(text.trim(), object.deprecated.as_deref(), ""),
			body.join("\n")
		);
	}

	fn callback(&mut self, i: usize) {
		let abi = self.abi;
		let callback = &abi.api.callbacks[i];
		self.on = callback.on;
		let c = abi.callback(i);
		let name = self.class(&c);
		let rust = callback.path.join("::");
		let mut body = Vec::new();
		let mut members = Vec::new();
		for (method, member) in callback.methods.iter().zip(callback_members(&callback.methods)) {
			crate::lock::function(&mut self.locked, method, &member);
			members.push(member.clone());
			let names = abi::distinct(&method.params, param_name, |_| false);
			let params: Vec<String> = (method.params.iter().zip(names))
				.map(|(p, name)| format!("{} {name}", self.dt(&p.ty)))
				.collect();
			body.push(format!(
				"{}\t{} {member}({});",
				docs(
					&self.function_docs(method, doc::sections(&method.docs).body),
					method.deprecated.as_deref(),
					"\t"
				),
				self.dt(&method.ok),
				params.join(", ")
			));
		}
		self.distinct(&rust, &members);
		body.push(
			"\t/// Called once Rust drops this implementation, to free what it holds: at once on the\n\t/// isolate's thread, or else from its event loop.\n\tvoid close() {}".to_string(),
		);
		let text = format!(
			"{rust}: implemented in Dart. Rust calls its methods on the isolate's thread: from\n\
			another, the call waits for the isolate's event loop to run it.\n\n{}",
			callback.docs
		);
		let _ = write!(
			self.types,
			"{}abstract class {name} {{\n{}\n}}\n\n",
			docs(text.trim(), callback.deprecated.as_deref(), ""),
			body.join("\n\n")
		);
	}

	/// What Rust's calls into a callback `c` run: a trampoline per method on the isolate's
	/// thread, the relay in the glue that reaches it, and the tables that hold both, which only
	/// a callback given to Rust needs.
	fn callback_tables(&mut self, c: &str) {
		let abi = self.abi;
		let i = (0..abi.api.callbacks.len())
			.find(|&i| abi.callback(i) == c)
			.expect("a callback of that name");
		let callback = &abi.api.callbacks[i];
		let on = std::mem::replace(&mut self.on, callback.on);
		let name = self.class(c);
		let rust = callback.path.join("::");
		let mut relays = Vec::new();
		let mut trampolines = Vec::new();
		for (method, member) in callback.methods.iter().zip(callback_members(&callback.methods)) {
			// The trampoline Rust's relay calls on the isolate's thread.
			let c_params: Vec<Ty> = method.params.iter().map(|p| abi.ty(&p.ty)).collect();
			let ret = abi.ty(&method.ok);
			let lifted: Vec<String> = (method.params.iter().enumerate())
				.map(|(k, p)| self.lift_lent(&p.ty, &format!("p{k}")))
				.collect();
			let call = format!("(_n.lent[data.address] as {name}).{member}({})", lifted.join(", "));
			let what = format!("{rust}::{}", method.name());
			let t = format!("_cb${}${}", mangle(self.bare(c)), mangle(method.name()));
			let body = match &method.ok {
				Type::Unit => format!("{call};"),
				ok => format!("return {};", self.lower(ok, &call)),
			};
			let tramp = self.trampoline_fn(&t, &c_params, &ret, &body, &what, &[]);
			trampolines.push(tramp);
			// Its relay in the glue.
			let relay = format!("{}_cb_{}_{}", self.space, self.bare(c), method.name());
			let mut fn_params = vec![Ty::Data];
			fn_params.extend(c_params.iter().cloned());
			let native = self.native_fn(&relay, fn_params.clone(), ret.clone());
			let nt = self.nt(&Ty::Fn(c_params.clone(), Box::new(ret.clone())));
			let inner = nt["ffi.Pointer<".len()..nt.len() - 1].to_string();
			relays.push(format!("ffi.Native.addressOf<{inner}>({native}).cast<ffi.Void>()"));
			let args: Vec<String> = (c_params.iter().enumerate())
				.map(|(k, ty)| self.dart_out(ty, &format!("p{k}")))
				.collect();
			let decls: Vec<String> = (c_params.iter().enumerate())
				.map(|(k, ty)| format!(", p{k}: {}", rs(ty)))
				.collect();
			let mut call_args = vec!["relay.token".to_string()];
			call_args.extend(args);
			// The trampoline takes and returns an `f32` as its bits, whatever C's table says.
			let cast = match self.passes_f32(&c_params, &ret) {
				true => format!(
					"\n\tlet f = unsafe {{ std::mem::transmute::<_, {}>(f) }};",
					self.crossing_fn(&c_params, &ret)
				),
				false => String::new(),
			};
			let called = self.dart_in(
				&ret,
				&format!(
					"unsafe {{ relay.home.call(\"{what}\", move || f({})) }}",
					call_args.join(", ")
				),
			);
			let _ = write!(
				self.glue,
				"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {relay}(data: *mut c_void{}) -> {} {{\n\tlet relay = unsafe {{ &*data.cast::<runtime::Relay>() }};\n\tlet table = unsafe {{ &*relay.f.cast::<crate::{c}_vtable>() }};\n\tlet f = table.{m}.unwrap_or_else(|| rt::fail(\"{c}_vtable.{m}\", \"the function is null\"));{cast}\n\t{called}\n}}\n\n",
				decls.concat(),
				rs(&ret),
				m = abi::member(method),
			);
		}
		let tables = format!("_tables${}", mangle(self.bare(c)));
		self.helpers.insert(
			tables.clone(),
			format!(
				"final {tables} = _n.tables('{rust}', [\n\t{}\n], [\n\t{}\n]);\n\n",
				relays.join(",\n\t"),
				trampolines.join(",\n\t")
			),
		);
		self.on = on;
	}

	/// A static function `name` C calls with `data` and `c_params`, running `body`, which
	/// returns the C result, with each object in `lent` closed after, and aborting on what
	/// it throws. Returns the pointer to it as an expression.
	fn trampoline_fn(
		&mut self,
		name: &str,
		c_params: &[Ty],
		ret: &Ty,
		body: &str,
		what: &str,
		lent: &[String],
	) -> String {
		let params: Vec<String> = std::iter::once("ffi.Pointer<ffi.Void> data".to_string())
			.chain((c_params.iter().enumerate()).map(|(k, ty)| format!("{} p{k}", self.ct(ty))))
			.collect();
		let ret_dt = self.ct(ret);
		let body = match lent.is_empty() {
			true => body.to_string(),
			false => format!(
				"{}\n\t\ttry {{\n\t\t\t{body}\n\t\t}} finally {{\n\t\t\t{}\n\t\t}}",
				lent.iter()
					.enumerate()
					.map(|(k, l)| format!("final l{k} = {l};"))
					.collect::<Vec<_>>()
					.join("\n\t\t"),
				(0..lent.len())
					.map(|k| format!("l{k}.close();"))
					.collect::<Vec<_>>()
					.join(" ")
			),
		};
		// What a throw returns, which never happens, as each aborts; a pointer or a struct
		// takes none.
		let exceptional = match ret {
			Ty::Bool => ", false".to_string(),
			Ty::F64 => ", 0.0".to_string(),
			Ty::Int(_) | Ty::F32 | Ty::Char | Ty::Enum(_) => ", 0".to_string(),
			_ => String::new(),
		};
		let mut native_params = vec!["ffi.Pointer<ffi.Void>".to_string()];
		native_params.extend(c_params.iter().map(|ty| self.nt(ty)));
		let sig = format!("{} Function({})", self.nt(ret), native_params.join(", "));
		self.helpers.insert(
			name.to_string(),
			format!(
				"{ret_dt} {name}({}) {{\n\ttry {{\n\t\t{body}\n\t}} catch (e, s) {{\n\t\t_n.abort('{what}', e, s);\n\t}}\n}}\n\n",
				params.join(", ")
			),
		);
		format!("ffi.Pointer.fromFunction<{sig}>({name}{exceptional}).cast<ffi.Void>()")
	}

	/// The relay and trampoline of the closure `closure`, whose type is `ty`: the relay's
	/// address, and the trampoline's pointer, made once.
	fn closure(&mut self, ty: &Type, closure: &Closure) -> (String, String) {
		let abi = self.abi;
		let space = self.space.clone();
		let sig = abi.closure(closure);
		let Ty::Fn(c_params, c_ret) = &sig else {
			unreachable!("a closure is a function")
		};
		let (c_params, c_ret) = (c_params.clone(), (**c_ret).clone());
		let k = match self.relays.iter().position(|r| *r == sig) {
			Some(k) => k,
			None => {
				self.relays.push(sig.clone());
				let k = self.relays.len() - 1;
				let decls: Vec<String> = (c_params.iter().enumerate())
					.map(|(n, ty)| format!(", p{n}: {}", rs(ty)))
					.collect();
				let args: String = (c_params.iter().enumerate())
					.map(|(n, ty)| format!(", {}", self.dart_out(ty, &format!("p{n}"))))
					.collect();
				// The trampoline takes and returns an `f32` as its bits.
				let fn_ty = self.crossing_fn(&c_params, &c_ret);
				let called = self.dart_in(
					&c_ret,
					&format!("unsafe {{ relay.home.call(\"a closure\", move || f(relay.token{args})) }}"),
				);
				let _ = write!(
					self.glue,
					"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_fn_{k}(data: *mut c_void{}) -> {} {{\n\tlet relay = unsafe {{ &*data.cast::<runtime::Relay>() }};\n\tlet f = unsafe {{ std::mem::transmute::<*const c_void, {fn_ty}>(relay.f) }};\n\t{called}\n}}\n\n",
					decls.concat(),
					rs(&c_ret),
				);
				k
			}
		};
		let mut fn_params = vec![Ty::Data];
		fn_params.extend(c_params.iter().cloned());
		let relay = self.native_fn(&format!("{}_fn_{k}", self.space), fn_params, c_ret.clone());
		let nt = self.nt(&sig);
		let inner = nt["ffi.Pointer<".len()..nt.len() - 1].to_string();
		let relay = format!("ffi.Native.addressOf<{inner}>({relay})");
		// The trampoline, per closure type, since two of one C signature may lift apart.
		let t = match self.trampolines.iter().position(|t| t == ty) {
			Some(t) => t,
			None => {
				self.trampolines.push(ty.clone());
				let t = self.trampolines.len() - 1;
				let dt = self.dt(ty);
				let mut args = Vec::new();
				let mut lends = Vec::new();
				for (n, ty) in closure.params.iter().enumerate() {
					match ty {
						// An object, by reference, lent for the call.
						Type::Ref(_, object) if matches!(**object, Type::Object(_)) => {
							lends.push(format!("{}._lent(p{n})", self.dt(object)));
							args.push(format!("l{}", lends.len() - 1));
						}
						ty => args.push(self.lift_lent(ty, &format!("p{n}"))),
					}
				}
				let call = format!("(_n.lent[data.address] as {dt})({})", args.join(", "));
				let body = match &closure.err {
					None => match &closure.ok {
						Type::Unit => format!("{call};"),
						ok => format!("return {};", self.returned(ok, &call)),
					},
					// A failure is the exception the binding throws for Rust's error, whose
					// error goes to the last out pointer, and a value to the one before it.
					Some(err) => {
						let Thrown::Wrapped(thrown) = self.thrown(err) else {
							unreachable!("a closure fails with a String")
						};
						let at = closure.params.len();
						let ok = match &closure.ok {
							Type::Unit => format!("{call};"),
							ok => {
								let value = self.returned(ok, "r_");
								let write = match abi.ty(ok) {
									c if Self::scalar(&c) => format!("p{at}.value = {value};"),
									Ty::Buf => format!("_n.setBuf(p{at}, {value});"),
									_ => format!("p{at}.ref = {value};"),
								};
								format!("final r_ = {call};\n\t{write}")
							}
						};
						let err_at = c_params.len() - 1;
						format!(
							"try {{\n\t{ok}\n\treturn true;\n}} on {thrown} catch (e) {{\n\t_n.setBuf(p{err_at}, _n.copied(e.error));\n\treturn false;\n}}"
						)
					}
				};
				let pointer = self.trampoline_fn(&format!("_t{t}"), &c_params, &c_ret, &body, "a closure", &lends);
				self.helpers
					.insert(format!("_t{t}_"), format!("final _t{t}_ = {pointer};\n\n"));
				t
			}
		};
		(relay, format!("_t{t}_"))
	}

	/// The C value of the Dart value `e` of type `ty` a closure returns, which Rust then
	/// owns: text as a buffer `buf_copy` made, and a value as a struct it writes.
	fn returned(&mut self, ty: &Type, e: &str) -> String {
		match ty {
			Type::String => format!("_n.copied({e})"),
			Type::Record(_) | Type::Option(_) => {
				let c = self.c_name(ty);
				let name = format!("_ret${}", mangle(&c));
				if !self.helpers.contains_key(&name) {
					let class = self.struct_class(&Ty::Struct(c.clone()));
					let w = self.writer(&c, ty);
					let dt = self.dt(ty);
					self.helpers.insert(
						name.clone(),
						format!(
							"{class} {name}({dt} v) {{\n\tfinal a_ = _Arena();\n\ttry {{\n\t\tfinal c = ffi.Struct.create<{class}>();\n\t\t{w}(v, c, a_, true);\n\t\treturn c;\n\t}} finally {{\n\t\ta_.end();\n\t}}\n}}\n\n"
						),
					);
				}
				format!("{name}({e})")
			}
			ty => self.lower(ty, e),
		}
	}

	/// The name each of `functions` takes as a member of `owner` (whose IR type is
	/// `owner_ty`) if it has one, or at the top level, apart from each other and from what
	/// `taken` holds for each, by its index, as [`abi::named`] keeps a scope's. Two clash
	/// only as [`collides`] reads them, so a named constructor keeps the name of an instance
	/// method, as Dart allows.
	fn names(
		&self,
		functions: &[&Function],
		owner: (Option<&Ty>, Option<&Type>),
		taken: impl Fn(usize, &str) -> bool,
	) -> Vec<String> {
		let mut rust = Vec::new();
		let mut spelled = Vec::new();
		let mut kinds = Vec::new();
		for function in functions {
			let (name, member, kind) = self.declared(function, owner);
			rust.push(name);
			spelled.push(member);
			kinds.push(kind);
		}
		let rust: Vec<&str> = rust.iter().map(String::as_str).collect();
		// An unnamed constructor clashes with none.
		let clash = |k: usize, a: &str, j: usize, b: &str| match (kinds[k], kinds[j]) {
			(Some(kk), Some(kj)) => collides(&format!("{kk}{a}"), &format!("{kj}{b}")),
			_ => false,
		};
		abi::named_apart(&rust, spelled, own, taken, alternative, clash)
	}

	/// The declaration `function` makes as a member of `owner` (whose IR type is `owner_ty`)
	/// if it has one, before it keeps apart from another: the Rust name it spells, its Dart
	/// name, and what [`collides`] reads before that name, `static ` or `new ` or none, or
	/// `None` for an unnamed constructor, which no name clashes with.
	fn declared(
		&self,
		function: &Function,
		(owner, owner_ty): (Option<&Ty>, Option<&Type>),
	) -> (String, String, Option<&'static str>) {
		let abi = self.abi;
		let (back, export) = match (owner.zip(owner_ty)).and_then(|(owner, ty)| abi.handed_back(function, owner, ty)) {
			Some((back, export, _)) => (Some(back), export),
			None => (None, abi.export(function, owner)),
		};
		let function = back.as_ref().unwrap_or(function);
		let own = match owner {
			Some(Ty::Struct(o) | Ty::Enum(o) | Ty::Handle { name: o, .. }) => {
				export.named.strip_prefix(o.as_str()).and_then(|r| r.strip_prefix('_'))
			}
			_ => None,
		};
		let mut reserved: Vec<&str> = OBJECT.to_vec();
		if matches!(owner_ty, Some(Type::Object(_))) {
			reserved.push("close");
		}
		// Named as Rust's path is, whatever C's clashes rename.
		let rust = own.unwrap_or_else(|| self.bare(&export.named)).to_string();
		let member = member(&rust, &reserved);
		let receiver = function.receiver.filter(|_| owner.is_some());
		let is_static = receiver.is_none() && owner.is_some();
		let stat = if is_static { "static " } else { "" };
		let plain = function.params.is_empty() && !function.future && function.err.is_none();
		let kind = match (function.access, receiver) {
			(Access::Field { set: false, .. }, Some(_)) if plain => Some(""),
			(Access::Const, None) if plain => Some(stat),
			// A function of a type returning it is a constructor: an object's or opaque
			// value's `new` is its unnamed one.
			_ if is_static && !function.future && Some(&strip_refs(&function.ok)) == owner_ty => {
				let unnamed = match owner_ty {
					Some(Type::Object(_)) => true,
					Some(Type::Record(i)) => abi.api.records[*i].opaque.is_some(),
					_ => false,
				};
				match rust == "new" && unnamed {
					true => None,
					false => Some("new "),
				}
			}
			_ => Some(stat),
		};
		(rust, member, kind)
	}

	/// `function`'s Dart form `member`, of `owner` (whose IR type is `owner_ty`) if it has
	/// one, apart from the members `taken` names, and its entry as [`collides`] reads it.
	fn function(
		&mut self,
		function: &Function,
		owner: (Option<&Ty>, Option<&Type>),
		member: &str,
		taken: &[String],
	) -> Option<(String, String)> {
		let on = std::mem::replace(&mut self.on, function.on);
		let code = self.function_on(function, owner, member, taken);
		self.on = on;
		code
	}

	fn function_on(
		&mut self,
		function: &Function,
		(owner, owner_ty): (Option<&Ty>, Option<&Type>),
		member: &str,
		taken: &[String],
	) -> Option<(String, String)> {
		let abi = self.abi;
		let _p = abi.prefix.clone();
		// A value is immutable in Dart, so its `&mut self` method returns it as the call left
		// it, calling the C function in place on a copy it hands back.
		let handed = (owner.zip(owner_ty)).and_then(|(owner, ty)| abi.handed_back(function, owner, ty));
		let (handed, export) = match handed {
			Some((back, export, handed)) => {
				self.handed.insert(export.call.name.clone(), handed);
				(Some(back), export)
			}
			// Called by a twin that runs it on a home, if it makes a value that is not `Send`
			// from none, as an isolate moves between threads.
			None => (None, abi.export(function, owner).on_home()),
		};
		let function = handed.as_ref().unwrap_or(function);
		let c_name = export.call.name.clone();
		let owner_c = match owner {
			Some(Ty::Struct(n) | Ty::Enum(n) | Ty::Handle { name: n, .. }) => Some(n.clone()),
			_ => None,
		};
		let own = match &owner_c {
			Some(o) => export.named.strip_prefix(o.as_str()).and_then(|r| r.strip_prefix('_')),
			None => None,
		};
		// Named as Rust's path is, whatever C's clashes rename.
		let named = self.bare(&export.named).to_string();
		let snake = own.unwrap_or(&named);

		let receiver = function.receiver.filter(|_| owner.is_some());
		// A leaf call runs on the isolate's thread, with no thread hop, as nothing in it
		// calls back: see `Abi::leaf`. Nor does Dart run meanwhile, so it holds no handle
		// against a close, and takes only scalars, so it needs no arena. It is one only where
		// the library reports `_proven`, else served as any other call.
		let leaf = abi.leaf(function, owner_ty);
		let mut pre = Vec::new();
		let mut args: Vec<String> = Vec::new();
		let mut c_params = export.call.params.iter();
		if receiver.is_some() {
			let (_, c) = c_params.next().expect("a method has its receiver");
			let ty = owner_ty.expect("a method has its type").clone();
			match c {
				Ty::Handle { .. } if leaf => args.push("_live".to_string()),
				Ty::Handle { .. } => {
					args.push("a_.hold(this)".to_string());
				}
				Ty::In(inner) => {
					let (stmts, ptr) = self.value_ptr(&ty, inner, "this");
					pre.push(stmts.trim_end().replace("\n\t\t\t", "\n").to_string());
					args.push(ptr);
				}
				other => unreachable!("a receiver is {other:?}"),
			}
		}
		let mut params = Vec::new();
		// Each implementation and waker is given to Rust as the call evaluates its arguments,
		// so every other one that may throw is lowered before, and a call that throws gives
		// Rust nothing.
		let mut gave = false;
		// Whether the call is given a value Rust takes, whose buffers the arena frees
		// unless the call returns.
		let mut gives = false;
		let names = abi::distinct(&function.params, param_name, |_| false);
		let mut writes = Vec::new();
		for (k, (param, name)) in function.params.iter().zip(names.iter().cloned()).enumerate() {
			let (_, c) = c_params.next().expect("each parameter has its C one");
			// A value passed for `&mut`, in a holder that takes what Rust writes back.
			if param.ty.written() {
				let inner = param.ty.base();
				params.push(format!("Mut<{}> {name}", self.dt(inner)));
				let value = format!("{name}.value");
				match Self::scalar(c) {
					true => args.push(self.lower(inner, &value)),
					false => {
						let v = format!("c{k}_");
						let nt = self.nt(c);
						gives = true;
						let write = self.write(inner, &value, &format!("{v}.ref"), "true");
						pre.push(format!("final {v} = a_<{nt}>();\n{write}"));
						args.push(format!("{v}.ref"));
					}
				}
				let out = Abi::written(function, &abi::escape(&param.name));
				let lifted = self.lift(inner, &self.out_value(&out, Some(&Ty::Out(Box::new(c.clone())))));
				writes.push(format!("{value} = {lifted};"));
				continue;
			}
			params.push(format!("{} {name}", self.dt(&param.ty)));
			match (&param.ty, c) {
				(Type::Closure(closure_), Ty::Fn(..)) => {
					c_params.next().expect("a closure has its data");
					let (relay, trampoline) = self.closure(&param.ty, closure_);
					args.push(relay);
					if closure_.kept {
						// Given to Rust, which frees it through the relay's drop.
						c_params.next().expect("a kept closure has its free");
						args.push(format!("_n.keep({name}, {trampoline})"));
						args.push("_n.relayDrop".to_string());
						gave = true;
					} else {
						args.push(format!("a_.lend({name}, {trampoline})"));
					}
				}
				(_, Ty::ForeignReader) => args.push(format!("_n.reader(a_, {name})")),
				(Type::Callback(i, _), _) => {
					let cb = abi.callback(*i);
					if self.tabled.insert(cb.clone()) {
						self.callback_tables(&cb);
					}
					args.push(format!(
						"_n.give(_tables${}, {name}, {name}.close)",
						mangle(self.bare(&cb))
					));
					gave = true;
				}
				(Type::Waker, _) => {
					args.push(format!("_n.wakerOf({name})"));
					gave = true;
				}
				(Type::Future, _) => {
					args.push(format!("_n.future({name})"));
					gave = true;
				}
				(ty, c) if Self::scalar(c) => {
					let lowered = self.lower(ty, &name);
					if gave {
						let v = format!("c{k}_");
						pre.push(format!("final {v} = {lowered};"));
						args.push(v);
					} else {
						args.push(lowered);
					}
				}
				(ty, c) => {
					let v = format!("c{k}_");
					let nt = self.nt(c);
					// Rust takes what a call is passed by value, and only reads a borrow.
					let take = match ty {
						Type::Ref(..) => "false",
						Type::Option(inner) if matches!(**inner, Type::Ref(..)) => "false",
						_ => "true",
					};
					gives |= take == "true";
					let write = self.write(ty, &name, &format!("{v}.ref"), take);
					pre.push(format!("final {v} = a_<{nt}>();\n{write}"));
					args.push(format!("{v}.ref"));
				}
			}
		}
		let outs: Vec<(String, Ty)> = c_params.filter(|(_, ty)| !abi::is_status(ty)).cloned().collect();
		// A call that fails a run-time check writes its status, which throws once it returns.
		let status = export.call.status().is_some();
		let check = format!("_status(s_.value, '{c_name}');");
		if status {
			pre.push("final s_ = _n.status;".to_string());
		}
		// A call this target lacks throws before any argument is lowered or given to Rust.
		if !abi.api.everywhere(self.on) {
			pre.insert(0, format!("_on(0x{:x}, '{c_name}');", self.on.0 & abi.api.all().0));
		}

		// A tuple of one returned whole is its element, written through its one out pointer.
		let ok = strip_refs(&function.ok);
		let err = function.err.as_ref().map(strip_refs);
		let thrown = err.as_ref().map(|e| self.thrown(e));
		// A waker Dart gets is one it can close, unlike one it gives.
		let ok_dt = match ok {
			Type::Waker => "Waker".to_string(),
			_ => self.dt(&ok),
		};

		// Each call, and each poll and free of its future, serves the isolate while it waits.
		if !leaf {
			args.insert(0, "_n.home".to_string());
		}

		let mut body = Vec::new();
		let ret_type;
		let mut pull = None;
		if let Some(abi::Future { start, poll, free, .. }) = &export.future {
			ret_type = format!("Cancelable<{ok_dt}>");
			// The poll, which completes the future.
			let mut poll_args = vec!["_n.home".to_string(), "f".to_string(), "w".to_string()];
			let poll_fn = self.served(poll);
			let free_fn = self.served(free);
			let poll_outs: Vec<(String, Ty)> = poll.params[2..]
				.iter()
				.filter(|(_, ty)| !abi::is_status(ty))
				.cloned()
				.collect();
			let mut stmts: Vec<String> = Vec::new();
			for (n, ty) in &poll_outs {
				stmts.push(self.out_decl(n, ty));
				poll_args.push(out_var(n));
			}
			// A future that is not `Send` lives on a home, which polls it, so its check fails
			// only for one a callback made elsewhere.
			let polled = match poll.status() {
				Some(_) => {
					stmts.push("final s_ = _n.status;".to_string());
					poll_args.push("s_".to_string());
					format!(
						"final p_ = {poll_fn}({});\n\t\t_status(s_.value, '{}');\n\t\tswitch (p_) {{",
						poll_args.join(", "),
						poll.name
					)
				}
				None => format!("switch ({poll_fn}({})) {{", poll_args.join(", ")),
			};
			let ready = match ok {
				Type::Unit => "null".to_string(),
				_ => self.lift_outs(&function.ok, &ok),
			};
			let mut cases = vec![format!("\t\tcase 1:\n\t\t\tc._ready({ready});\n\t\t\treturn true;")];
			// A start that finished returns a future of its own, complete already if ready:
			// only one that pended makes a call to poll it again.
			let mut started = vec![format!(
				"case 1:\n\treturn Cancelable._(Future<{ok_dt}>.syncValue({ready}));"
			)];
			if let (Some(err), Some(thrown)) = (&err, &thrown) {
				let lifted = self.lift(err, &self.out_value("err", poll_outs.last().map(|(_, t)| t)));
				let error = thrown_value(thrown, &lifted);
				cases.push(format!("\t\tcase 2:\n\t\t\tc._error({error});\n\t\t\treturn true;"));
				// Never complete already: a future that fails with no listener reports its
				// error as uncaught.
				started.push(format!(
					"case 2:\n\treturn Cancelable._(Future<{ok_dt}>.error({error}));"
				));
			}
			started.push("default:\n\tf_ = o$f_.value;".to_string());
			let poll_name = format!("_p${}", mangle(&c_name));
			self.helpers.insert(
				poll_name.clone(),
				format!(
					"bool {poll_name}(ffi.Pointer<ffi.Void> f, _Waker w, _Call<{ok_dt}> c) {{\n\tfinal a_ = _n.stack;\n\tfinal m_ = a_.mark();\n\ttry {{\n\t\t{}\n\t\t{polled}\n{}\n\t\t\tdefault:\n\t\t\t\treturn false;\n\t\t}}\n\t}} finally {{\n\t\ta_.reset(m_);\n\t}}\n}}\n\n",
					stmts.join("\n\t\t"),
					cases.iter().map(|c| c.replace("\n\t\t", "\n\t\t\t").replacen("\t\tcase", "\t\t\tcase", 1)).collect::<Vec<_>>().join("\n")
				),
			);
			// The call and its first poll in one, which completes the call if ready, or else
			// leaves it the future to poll.
			let start_fn = self.served(start);
			let mut start_args = args.clone();
			let mut inner = pre.clone();
			for (n, ty) in &poll_outs {
				inner.push(self.out_decl(n, ty));
				start_args.push(out_var(n));
			}
			inner.push("final o$f_ = a_<ffi.Pointer<ffi.Void>>();".to_string());
			start_args.push("o$f_".to_string());
			let started = indent(&started.join("\n"));
			if status {
				start_args.push("s_".to_string());
			}
			inner.push(format!("final started_ = {start_fn}({});", start_args.join(", ")));
			// Rust took what it was given, even for a call its check refuses.
			if gives {
				inner.push("a_.took();".to_string());
			}
			if status {
				inner.push(check.clone());
			}
			// A stream's `next` has a twin its `asStream` calls, which hands what a start
			// that finished returns to the stream in place, and returns its call that pended.
			if function.stream().is_some() && receiver.is_some() && matches!(owner_ty, Some(Type::Object(_))) {
				let mut cases = vec![format!("case 1:\n\tp_._ready({ready});\n\treturn null;")];
				if let (Some(err), Some(thrown)) = (&err, &thrown) {
					let lifted = self.lift(err, &self.out_value("err", poll_outs.last().map(|(_, t)| t)));
					let error = thrown_value(thrown, &lifted);
					cases.push(format!("case 2:\n\tp_._error({error});\n\treturn null;"));
				}
				cases.push("default:\n\tf_ = o$f_.value;".to_string());
				let mut twin = inner.clone();
				twin.push(format!("switch (started_) {{\n{}\n}}", indent(&cases.join("\n"))));
				let item = ok_dt.strip_suffix('?').expect("a stream's `next` returns an `Option`");
				pull = Some((
					item.to_string(),
					format!(
						"final ffi.Pointer<ffi.Void> f_;\nfinal a_ = _Arena();\ntry {{\n{}\n}} finally {{\n\ta_.end();\n}}\nreturn _Call<{ok_dt}>._({free_fn}, {poll_name}, f_);",
						indent(&twin.join("\n"))
					),
				));
			}
			inner.push(format!("switch (started_) {{\n{started}\n}}"));
			body.push("final ffi.Pointer<ffi.Void> f_;".to_string());
			body.push(format!(
				"final a_ = _Arena();\ntry {{\n{}\n}} finally {{\n\ta_.end();\n}}",
				indent(&inner.join("\n"))
			));
			body.push(format!("return _call(_Call<{ok_dt}>._({free_fn}, {poll_name}, f_));"));
		} else {
			ret_type = ok_dt.clone();
			// The statements after `pre` making the call through `call_fn` with `args`.
			let made = |g: &mut Self, call_fn: String, args: &[String], writes: Vec<String>| {
				let mut inner = Vec::new();
				for (n, ty) in &outs {
					inner.push(g.out_decl(n, ty));
				}
				let mut call_args = args.to_vec();
				call_args.extend(outs.iter().map(|(n, _)| out_var(n)));
				if status {
					call_args.push("s_".to_string());
				}
				let call = format!("{call_fn}({})", call_args.join(", "));
				// Rust took what it was given, even for a call its check refuses.
				let took = match gives {
					true => "\na_.took();",
					false => "",
				};
				let checked = |inner: &mut Vec<String>| {
					if status {
						inner.push(check.clone());
					}
				};
				match (&err, &ok) {
					(Some(err), ok) => {
						inner.push(format!("final ok_ = {call};{took}"));
						checked(&mut inner);
						inner.extend(writes);
						let ready = match ok {
							Type::Unit => "return;".to_string(),
							_ => format!("return {};", g.lift_outs(&function.ok, ok)),
						};
						inner.push(format!("if (ok_) {{\n\t{ready}\n}}"));
						let err_ty = outs.last().map(|(_, t)| t);
						let lifted = g.lift(err, &g.out_value("err", err_ty));
						inner.push(format!(
							"throw {};",
							thrown_value(thrown.as_ref().expect("an error throws"), &lifted)
						));
					}
					(None, Type::Unit) => {
						inner.push(format!("{call};{took}"));
						checked(&mut inner);
						inner.extend(writes);
					}
					// A value C returns beside what Rust wrote back is its return, as with none.
					(None, ok) if outs.len() > writes.len() => {
						inner.push(format!("{call};{took}"));
						checked(&mut inner);
						inner.extend(writes);
						inner.push(format!("return {};", g.lift_outs(&function.ok, ok)));
					}
					(None, ok) => {
						inner.push(format!("final r_ = {call};{took}"));
						checked(&mut inner);
						inner.extend(writes);
						inner.push(format!("return {};", g.lift(ok, "r_")));
					}
				}
				inner
			};
			// A served call, in the arena `a_`, or holding its one handle alone.
			let served = |inner: Vec<String>| {
				let code = inner.join("\n");
				match held_only(&code) {
					Some(code) => format!(
						"final h_ = _hold();\ntry {{\n{}\n}} finally {{\n\t_release();\n}}",
						indent(&code)
					),
					None => format!(
						"final a_ = _Arena();\ntry {{\n{}\n}} finally {{\n\ta_.end();\n}}",
						indent(&code)
					),
				}
			};
			match leaf {
				// Served instead, holding its receiver and serving the isolate, where the build
				// runs other bodies than the proof saw. Its `pre` needs no arena, as a leaf
				// takes only scalars.
				true => {
					let call_fn = self.leaf(&export.call);
					let proven = made(self, call_fn, &args, writes.clone());
					let mut unproven = args.clone();
					unproven[0] = "a_.hold(this)".to_string();
					unproven.insert(0, "_n.home".to_string());
					let call_fn = self.served(&export.call);
					let unproven = made(self, call_fn, &unproven, writes);
					body.extend(pre.clone());
					body.push(format!(
						"if (_proven) {{\n{}\n}} else {{\n{}\n}}",
						indent(&proven.join("\n")),
						indent(&served(unproven))
					));
				}
				false => {
					let call_fn = self.served(&export.call);
					let mut inner = pre.clone();
					inner.extend(made(self, call_fn, &args, writes));
					body.push(served(inner));
				}
			}
		}

		let mut text = doc::sections(&function.docs).body;
		if handed.is_some() {
			text.push_str(
				"\n\nRust changes a copy of this value, which it returns first, as the call left it: this one never changes.",
			);
		}
		for (param, name) in function.params.iter().zip(&names) {
			match &param.ty {
				Type::Closure(closure_) => {
					text.push_str(&match closure_.kept {
						true => format!(
							"\n\nRust keeps `{name}` until it drops it, calling it on the isolate's thread: from another, the call waits for the isolate's event loop to run it."
						),
						false => format!(
							"\n\nRust calls `{name}` only during the call, on the isolate's thread, while the call runs on another."
						),
					});
					if closure_.params.iter().any(|p| matches!(p.base(), Type::Object(_))) {
						text.push_str(&format!(
							"\nAn object it lends lives only for that call of `{name}`: keep none past it."
						));
					}
					if let Some(err) = &closure_.err
						&& let Thrown::Wrapped(thrown) = self.thrown(err)
					{
						text.push_str(&format!(
							"\nIt fails by throwing [{thrown}], whose error Rust takes; anything else it throws aborts."
						));
					}
				}
				Type::Ref(_, inner) if **inner == Type::Reader => text.push_str(&format!(
					"\n\nRust reads and seeks `{name}` only during the call, on the isolate's thread, while the call runs on another."
				)),
				_ => {}
			}
		}
		if function.future {
			text.push_str("\n\nCancelling it cancels it in Rust, losing nothing.");
		}
		let is_static = receiver.is_none() && owner.is_some();
		let stat = if is_static { "static " } else { "" };
		let code_body = indent(&body.join("\n"));
		let mut member = match owner {
			None => self.claim(member, format!("fn `{}`", function.path.join("::"))),
			Some(_) => member.to_string(),
		};
		// A member its type declares already, as a field of its name, takes the next of its
		// alternatives.
		let mut alternatives = alternatives(&member);
		alternatives.next();
		let (head, member) = loop {
			let (head, entry) = match (function.access, receiver) {
				(Access::Field { set: false, .. }, Some(_))
					if function.params.is_empty() && !function.future && err.is_none() =>
				{
					(format!("{ret_type} get {member}"), member.clone())
				}
				(Access::Const, None) if function.params.is_empty() && !function.future && err.is_none() => {
					(format!("{stat}{ret_type} get {member}"), format!("{stat}{member}"))
				}
				// A function of a type returning it is a constructor, which, unlike a static
				// member, may share its name with an instance member, as a Rust function may
				// with a field. An object's or opaque value's `new` is its unnamed one, which a
				// record's fields take.
				_ if is_static && !function.future && Some(&ok) == owner_ty => {
					let unnamed = match owner_ty {
						Some(Type::Object(_)) => true,
						Some(Type::Record(i)) => self.abi.api.records[*i].opaque.is_some(),
						_ => false,
					};
					if snake == "new" && unnamed {
						(format!("factory {ret_type}({})", params.join(", ")), "new ".to_string())
					} else {
						(
							format!("factory {ret_type}.{member}({})", params.join(", ")),
							format!("new {member}"),
						)
					}
				}
				_ => (
					format!("{stat}{ret_type} {member}({})", params.join(", ")),
					format!("{stat}{member}"),
				),
			};
			if entry == "new " || !taken.iter().any(|other| collides(other, &entry)) {
				break (head, entry);
			}
			member = alternatives.next().expect("alternatives never end");
		};
		crate::lock::function(&mut self.locked, function, &member);

		let text = self.function_docs(function, text);
		let mut code = format!(
			"{}{head} {{\n{code_body}\n}}\n",
			docs(&text, function.deprecated.as_deref(), "")
		);
		if let Some((item, twin)) = pull {
			let ok = ret_type.strip_prefix("Cancelable<").and_then(|r| r.strip_suffix('>'));
			let _ = write!(
				code,
				"\n/// [{member}]'s start, for its stream: `p_` takes what finishes at once, and a call that pends is returned.\n_Call<{}>? _{member}Pull(_Pull<{item}> p_) {{\n{}\n}}\n",
				ok.expect("a stream's `next` is a future"),
				indent(&twin),
			);
			let key = crate::lock::function_key(function);
			self.streams
				.push((member.clone(), item, key, function.deprecated.clone()));
		}
		Some((code, member))
	}

	/// The declaration of the local the call writes the out value `name` of C type `ty` to.
	fn out_decl(&mut self, name: &str, ty: &Ty) -> String {
		let Ty::Out(inner) = ty else {
			unreachable!("an out parameter is a pointer")
		};
		let nt = self.nt(inner);
		format!("final {} = a_<{nt}>();", out_var(name))
	}

	/// The C value an out pointer `o_<name>` holds once written: a scalar's value, or a
	/// struct's view.
	fn out_value(&self, name: &str, ty: Option<&Ty>) -> String {
		match ty {
			Some(Ty::Out(inner)) if !Self::scalar(inner) => format!("{}.ref", out_var(name)),
			_ => format!("{}.value", out_var(name)),
		}
	}

	/// The Dart value of a call's outs, which return `ok`, or each element of a tuple.
	fn lift_outs(&mut self, declared: &Type, ok: &Type) -> String {
		let abi = self.abi;
		match declared {
			Type::Tuple(elements) => {
				let lifted: Vec<String> = (elements.iter().enumerate())
					.map(|(n, e)| {
						let e = strip_refs(e);
						let c = abi.ty(&e);
						let v = match Self::scalar(&c) {
							true => format!("o$out{n}.value"),
							false => format!("o$out{n}.ref"),
						};
						self.lift(&e, &v)
					})
					.collect();
				match lifted.len() {
					1 => format!("({},)", lifted[0]),
					_ => format!("({})", lifted.join(", ")),
				}
			}
			_ => {
				let c = abi.ty(ok);
				let v = match Self::scalar(&c) {
					true => "o$out.value",
					false => "o$out.ref",
				};
				self.lift(ok, v)
			}
		}
	}

	/// `text`, a function's docs without its sections, then its `# Errors` as what it
	/// throws, and its `# Panics` as when it aborts.
	fn function_docs(&mut self, function: &Function, text: String) -> String {
		let sections = doc::sections(&function.docs);
		let mut text = text.trim().to_string();
		match (&sections.errors, &function.err) {
			(Some(errors), Some(err)) => {
				let thrown = match self.thrown(err) {
					Thrown::Itself => self.dt(err),
					Thrown::Wrapped(name) => name,
				};
				let _ = write!(text, "\n\nThrows [{thrown}]: {errors}");
			}
			(Some(errors), None) => {
				let _ = write!(text, "\n\n# Errors\n\n{errors}");
			}
			(None, _) => {}
		}
		if let Some(panics) = &sections.panics {
			let _ = write!(text, "\n\n# Aborts\n\n{panics}");
		}
		text.trim().to_string()
	}

	/// What a dartdoc link names for `target`, if Dart has it.
	fn link(&self, target: doc::Target<'_>) -> Option<String> {
		let abi = self.abi;
		let owner = |owner: doc::Owner| match owner {
			doc::Owner::Record(i) => self.class(&abi.record(i)),
			doc::Owner::Enum(i) => self.class(&abi.enumeration(i)),
			doc::Owner::Object(i) => self.class(&abi.object(i)),
			doc::Owner::Callback(i) => self.class(&abi.callback(i)),
		};
		Some(match target {
			doc::Target::Record(i) => owner(doc::Owner::Record(i)),
			doc::Target::Enum(i) => owner(doc::Owner::Enum(i)),
			doc::Target::Object(i) => owner(doc::Owner::Object(i)),
			doc::Target::Callback(i) => owner(doc::Owner::Callback(i)),
			doc::Target::Variant(i, v) => {
				let e = &abi.api.enums[i];
				let name = owner(doc::Owner::Enum(i));
				match e.plain() {
					true => format!("{name}.{}", enum_values(e).swap_remove(v)),
					false => self.variant_class(&name, &e.variants[v].name),
				}
			}
			doc::Target::Field(i, n) => format!(
				"{}.{}",
				owner(doc::Owner::Record(i)),
				field_names(&abi.api.records[i].fields).swap_remove(n)
			),
			doc::Target::Constant(c) => self.locked.get(&format!("const {}", c.path.join("::")))?.clone(),
			doc::Target::Function(of, f) => {
				// The lock holds a member as its type declares it: `static <name>`, or `new
				// <name>` for a constructor, unnamed where the name is empty, which dartdoc
				// links as `new`.
				let member = self.locked.get(&crate::lock::function_key(f))?;
				let member = member.strip_prefix("static ").unwrap_or(member);
				let member = match member.strip_prefix("new ") {
					Some("") => "new",
					Some(named) => named,
					None => member,
				};
				match of {
					Some(of) => format!("{}.{member}", owner(of)),
					None => member.to_string(),
				}
			}
		})
	}

	/// How an error of type `err` is thrown: as itself if an exported type, which then
	/// implements `Exception`, or else in an exception of its own.
	fn thrown(&mut self, err: &Type) -> Thrown {
		match err.base() {
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
				let dt = self.dt(err);
				self.conforms.insert(dt);
				Thrown::Itself
			}
			_ => {
				// One exception for each C type and Dart type: a `Vec<NonZero<u16>>` shares
				// `Vec<u16>`'s C type, but Dart holds it as a `List<int>`, not a `Uint16List`.
				let key = self.abi.key(err);
				let dt = self.dt(err);
				let thrown: Vec<(String, Type)> = (self.thrown.iter())
					.filter(|(_, seen)| self.abi.key(seen) == key)
					.map(|(name, seen)| (name.clone(), seen.clone()))
					.collect();
				let seen = thrown.into_iter().find(|(_, seen)| self.dt(seen) == dt);
				let name = match seen {
					Some((name, _)) => name,
					None => {
						let name = self.claim(&format!("{}Exception", pascal(&key)), format!("the error type `{dt}`"));
						self.thrown.insert(name.clone(), err.clone());
						name
					}
				};
				crate::lock::exception(&mut self.locked, self.abi, err, &dt, &name);
				Thrown::Wrapped(name)
			}
		}
	}
}

/// The Dart names of what owns an object's handle, as [`Gen::owned`] makes them.
struct Owned {
	/// Gives the garbage collector a hold of it.
	own: String,
	/// The C function freeing it.
	freeing: String,
	/// Frees it.
	free: String,
	/// Of a checked value, drops it, then frees it if asked.
	close: Option<String>,
}

/// What a C struct holds by value, as Dart declares it.
#[derive(Clone)]
enum Layout {
	/// Fields of these types.
	Fields(Vec<Ty>),
	/// A data enum's tag, then a union of these payload structs.
	Enum(Vec<String>),
	/// Only bytes or integers: an array, or an opaque record.
	Opaque,
}

/// How a function throws its error.
enum Thrown {
	Itself,
	Wrapped(String),
}

/// The error value thrown for the lifted error `lifted`.
fn thrown_value(thrown: &Thrown, lifted: &str) -> String {
	match thrown {
		Thrown::Itself => lifted.to_string(),
		Thrown::Wrapped(name) => format!("{name}({lifted})"),
	}
}

/// The `dart:typed_data` list holding a block's element exactly, bit for bit: a
/// `Uint64List` reads a `u64` past `i64::MAX` as the negative `int` of its bits, which a
/// `List<U64>` it stands for reads unsigned.
fn typed_list(elem: &Type) -> &'static str {
	match elem {
		Type::Int(Int::U8) => "Uint8List",
		Type::Int(Int::U16) => "Uint16List",
		Type::Int(Int::U32) => "Uint32List",
		Type::Int(Int::U64) => "Uint64List",
		Type::Int(Int::I8) => "Int8List",
		Type::Int(Int::I16) => "Int16List",
		Type::Int(Int::I32) => "Int32List",
		Type::Int(Int::I64) => "Int64List",
		Type::F32 => "Float32List",
		Type::F64 => "Float64List",
		other => unreachable!("{other:?} is no block's element"),
	}
}

/// The Dart type of a Rust integer: `int` but for `u64` and `usize`, whose values past
/// `i64::MAX` it holds as negative, which `U64` reads unsigned.
fn int_dt(int: Int) -> &'static str {
	match int {
		Int::U64 | Int::Usize => "U64",
		_ => "int",
	}
}

/// The runtime's class of a socket address, or else an IP address, of `family`.
fn addr_class(socket: bool, family: Family) -> &'static str {
	match (socket, family) {
		(true, Family::Any) => "SocketAddr",
		(true, Family::V4) => "SocketAddrV4",
		(true, Family::V6) => "SocketAddrV6",
		(false, Family::Any) => "IpAddr",
		(false, Family::V4) => "Ipv4Addr",
		(false, Family::V6) => "Ipv6Addr",
	}
}

/// The `dart:ffi` type of a C integer.
fn native_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "ffi.Uint8",
		Int::U16 => "ffi.Uint16",
		Int::U32 => "ffi.Uint32",
		Int::U64 => "ffi.Uint64",
		Int::Usize => "ffi.Size",
		Int::I8 => "ffi.Int8",
		Int::I16 => "ffi.Int16",
		Int::I32 => "ffi.Int32",
		Int::I64 => "ffi.Int64",
		Int::Isize => "ffi.IntPtr",
	}
}

/// A Dart integer literal of `v`, its bits for a `u64` past `i64`.
fn int_literal(v: i128) -> String {
	match v > i64::MAX as i128 {
		true => format!("0x{:x}", v as u64),
		false => v.to_string(),
	}
}

/// A C name as a private Dart name, its `_`s as `$`s: Dart's lints read an underscore inside
/// an identifier as a break in its words, and no Rust or C name has a `$` to clash with.
fn private(c: &str) -> String {
	format!("_{}", mangle(c))
}

/// [`private`], for a type, which starts in upper case.
fn private_type(c: &str) -> String {
	let mut chars = mangle(c).chars().collect::<Vec<_>>();
	chars[0] = chars[0].to_ascii_uppercase();
	format!("_{}", chars.into_iter().collect::<String>())
}

/// How many times `code` has `call`, which is no end of a longer name.
fn calls(code: &str, call: &str) -> usize {
	(code.match_indices(call))
		.filter(|(at, _)| !code[..*at].ends_with(|c: char| c.is_alphanumeric() || c == '_' || c == '$'))
		.count()
}

/// The local a call's out pointer `name` lands in.
fn out_var(name: &str) -> String {
	format!("o${}", mangle(name))
}

/// `name` with its `_`s as `$`s.
fn mangle(name: &str) -> String {
	name.replace('_', "$")
}

/// The name of a field of a C struct: `$` and its name, as Dart's own members never are.
fn slot(name: &str) -> String {
	format!("${}", mangle(name))
}

/// A snake-case name in camel case: `into_label` is `intoLabel`, and `0_set` is `$0Set`.
fn camel(name: &str) -> String {
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("${}", pascal(name));
	}
	let pascal = pascal(name);
	let mut chars = pascal.chars();
	match chars.next() {
		Some(first) => first.to_lowercase().chain(chars).collect(),
		None => name.to_string(),
	}
}

/// A type's Dart name: a built-in identifier takes a `2`, since Dart's lints allow no `_` in
/// a type's name.
fn type_name(name: &str) -> String {
	match BUILT_IN.contains(&name) {
		true => format!("{name}2"),
		false => name.to_string(),
	}
}

/// `name`, then the names to try in its place where it is taken: a type takes the next number
/// (`Cancelled2`), as Dart's lints allow no `_` in one, and any other name a trailing `_`,
/// then a number before it (`default_`, `default2_`), which no name Rust gives it ends in.
fn alternatives(name: &str) -> impl Iterator<Item = String> + use<> {
	let base = name.trim_end_matches('_').to_string();
	let type_ = name.starts_with(|c: char| c.is_ascii_uppercase());
	let name = name.to_string();
	let first = name.clone();
	std::iter::once(first).chain(
		(1..)
			.filter_map(move |n| match (type_, n) {
				(true, 1) => None,
				(true, n) => Some(format!("{base}{n}")),
				(false, 1) => Some(format!("{base}_")),
				(false, n) => Some(format!("{base}{n}_")),
			})
			.filter(move |alt| *alt != name),
	)
}

/// `names`, each kept apart from what `taken` names and from the others, as [`abi::unique`]
/// does, but by [`alternatives`]: one taken, or an earlier one's, takes the first free
/// alternative, never another's own.
fn unique(names: Vec<String>, taken: impl Fn(&str) -> bool) -> Vec<String> {
	let mut used = BTreeSet::new();
	(names.iter())
		.map(|name| {
			let free = alternatives(name)
				.find(|alt| !(taken(alt) || used.contains(alt) || (alt != name && names.contains(alt))))
				.expect("alternatives never end");
			used.insert(free.clone());
			free
		})
		.collect()
}

/// The classes `abi` names, as [`Abi::classes`] does, but with no `_` in a name, which
/// Dart's lints allow in no type's: one it joined from a module and a type, `Polar_Turn`,
/// drops it, and one it made distinct with a trailing `_` takes a number instead, each
/// after every name that has none.
fn classes(abi: &Abi<'_>, taken: &dyn Fn(&str) -> bool) -> HashMap<String, String> {
	let mut classes = abi.classes(|_| (String::new(), 0), taken);
	let mut apart: Vec<(String, String)> = (classes.iter())
		.filter(|(_, name)| name.contains('_'))
		.map(|(c, name)| (name.clone(), c.clone()))
		.collect();
	apart.sort();
	let mut used: BTreeSet<String> = (classes.values()).filter(|name| !name.contains('_')).cloned().collect();
	for (name, c) in apart {
		let base = name.trim_end_matches('_').replace('_', "");
		let free = alternatives(&base)
			.find(|alt| !(taken(alt) || used.contains(alt)))
			.expect("alternatives never end");
		used.insert(free.clone());
		classes.insert(c, free);
	}
	classes
}

/// A member's Dart name: in camel case, with `_` appended if a reserved word or one of
/// `reserved`.
fn member(snake: &str, reserved: &[&str]) -> String {
	let name = camel(snake);
	match RESERVED.contains(&name.as_str()) || reserved.contains(&name.as_str()) {
		true => format!("{name}_"),
		false => name,
	}
}

/// A field's Dart name, clear of the members every object has and of the methods any trait
/// may give a type, so a type that gains `Debug`, `PartialOrd`, or `Ord` renames no field.
fn field_name(name: &str) -> String {
	member(
		name,
		&[
			"hashCode",
			"noSuchMethod",
			"runtimeType",
			"toString",
			"debug",
			"compareTo",
			"partialCompareTo",
		],
	)
}

/// A parameter's Dart name. The call's own locals end in `_`, which no camel-case name does.
fn param_name(name: &str) -> String {
	member(name, &[])
}

/// Whether the Dart name `name` reads back as the Rust one `rust`, as it spells it or in
/// camel case, for [`abi::named`].
fn own(rust: &str, name: &str) -> bool {
	name == rust || abi::snake(name) == rust
}

/// The `n`th of the [`alternatives`] to `name`, for [`abi::named`].
fn alternative(name: &str, n: usize) -> String {
	alternatives(name).nth(n).expect("alternatives never end")
}

/// The names of a callback's `methods`, one scope's, apart as [`abi::named`] keeps them, and
/// from what every object has and the `close` hook Rust calls once it drops an implementation.
fn callback_members(methods: &[Function]) -> Vec<String> {
	let reserved: Vec<&str> = OBJECT.iter().copied().chain(["close"]).collect();
	let rust: Vec<&str> = methods.iter().map(|method| method.name()).collect();
	let spelled = rust.iter().map(|name| member(&abi::snake(name), &reserved)).collect();
	abi::named(&rust, spelled, own, |_, _| false, alternative)
}

/// The Dart names of `fields`, one scope's, each [`field_name`], kept apart as [`abi::named`]
/// keeps them.
fn field_names(fields: &[crate::ir::Field]) -> Vec<String> {
	let rust: Vec<&str> = fields.iter().map(|f| f.name.as_str()).collect();
	let spelled = rust.iter().map(|name| field_name(name)).collect();
	abi::named(&rust, spelled, own, |_, _| false, alternative)
}

/// A plain enum's values: each variant in camel case, kept apart from the others, and from a
/// member every Dart enum has or one its traits give it, since a static member clashes with
/// either kind, as [`abi::named`] keeps them.
fn enum_values(e: &crate::ir::Enum) -> Vec<String> {
	const ENUM: &[&str] = &[
		"values",
		"index",
		"name",
		"hashCode",
		"noSuchMethod",
		"runtimeType",
		"toString",
		"compareTo",
		"partialCompareTo",
	];
	let t = e.traits;
	let made = [(t.debug, "debug"), (t.default, "default_")];
	let taken = |_, name: &str| ENUM.contains(&name) || made.iter().any(|(has, made)| *has && *made == name);
	let rust: Vec<&str> = e.variants.iter().map(|v| v.name.as_str()).collect();
	let spelled = rust.iter().map(|name| member(&abi::snake(name), &[])).collect();
	abi::named(
		&rust,
		spelled,
		|rust, name| pascal(name) == rust,
		taken,
		abi::underscored,
	)
}

/// Whether the members `a` and `b` of one type clash, each `static x`, `new x` for a
/// constructor, or else an instance member: Dart's instance and static members share one
/// namespace, and its constructors and static members another.
fn collides(a: &str, b: &str) -> bool {
	let spaces = |member: &str| -> (Vec<usize>, String) {
		match member.split_once(' ') {
			Some(("static", name)) => (vec![0, 1], name.to_string()),
			Some(("new", name)) => (vec![1], name.to_string()),
			_ => (vec![0], member.to_string()),
		}
	};
	let ((a_spaces, a), (b_spaces, b)) = (spaces(a), spaces(b));
	a == b && a_spaces.iter().any(|space| b_spaces.contains(space))
}

/// Dart doc comments holding `docs`, each line indented by `indent`, or nothing for none,
/// and `@Deprecated` with its note where `deprecated` has one.
fn docs(docs: &str, deprecated: Option<&str>, indent: &str) -> String {
	let mut out: String = docs
		.lines()
		.map(|line| match line.is_empty() {
			true => format!("{indent}///\n"),
			false => format!("{indent}/// {line}\n"),
		})
		.collect();
	// Never `@deprecated`, which says "next release" and a member named `deprecated` hides.
	if let Some(note) = deprecated {
		let _ = writeln!(out, "{indent}@Deprecated({})", dart_string(note));
	}
	out
}

/// A Dart string literal of `text`.
fn dart_string(text: &str) -> String {
	let mut out = String::from("'");
	for c in text.chars() {
		match c {
			'\'' | '\\' | '$' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\r' => out.push_str("\\r"),
			'\t' => out.push_str("\\t"),
			c if c.is_control() => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
			c => out.push(c),
		}
	}
	out.push('\'');
	out
}

/// Code indented by one tab more, but for blank lines.
/// `code`, a call's statements in the arena `a_`, without it if all it asks of the arena is
/// to hold `this` for the call: then holding `this` itself, as `h_`, costs less.
fn held_only(code: &str) -> Option<String> {
	let uses = code
		.match_indices("a_")
		.filter(|(at, _)| !code[..*at].ends_with(|c: char| c.is_alphanumeric() || c == '_'))
		.count();
	(uses == 1 && code.contains("a_.hold(this)")).then(|| code.replace("a_.hold(this)", "h_"))
}

fn indent(code: &str) -> String {
	let mut out: String = code
		.lines()
		.map(|l| match l.is_empty() {
			true => "\n".to_string(),
			false => format!("\t{l}\n"),
		})
		.collect();
	if !code.ends_with('\n') {
		out.pop();
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Every type the runtime names unqualified is its own or [`TAKEN`], so no exported
	/// item shadows it.
	#[test]
	fn runtime_names_taken() {
		let mut code = String::new();
		for line in RUNTIME.lines() {
			let line = line.split_once("//").map_or(line, |(code, _)| code);
			// A string literal names nothing.
			let mut quote = None;
			for c in line.chars() {
				match quote {
					Some(q) if c == q => quote = None,
					Some(_) => {}
					None if c == '\'' || c == '"' => quote = Some(c),
					None => code.push(c),
				}
			}
			code.push('\n');
		}
		// A capitalized word no `.` qualifies, nor part of a longer one.
		let words: Vec<&str> = (code.char_indices())
			.filter(|&(i, c)| {
				c.is_ascii_uppercase() && !code[..i].ends_with(|p: char| p == '.' || p == '_' || p.is_alphanumeric())
			})
			.map(|(i, _)| {
				let rest = &code[i..];
				&rest[..rest
					.find(|c: char| !(c == '_' || c.is_alphanumeric()))
					.unwrap_or(rest.len())]
			})
			.collect();
		let declared: BTreeSet<&str> = ["class ", "enum ", "type ", "typedef "]
			.iter()
			.flat_map(|kind| code.split(kind).skip(1))
			.filter_map(|rest| rest.split(|c: char| !(c == '_' || c.is_alphanumeric())).next())
			.collect();
		for word in words {
			// A one-letter name is a type parameter.
			if word.len() > 1 && !declared.contains(word) {
				assert!(TAKEN.contains(&word), "the runtime's `{word}` is not in TAKEN");
			}
		}
	}
}
