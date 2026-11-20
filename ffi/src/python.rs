//! Python over the C ABI, through a CPython extension: `python/<prefix>.py`, and the
//! extension functions it calls, which the crate builds with its `python` feature on
//! CPython's stable ABI, so one build loads in every CPython from 3.11.
//!
//! Each name is its C name without the prefix, as Python spells it: `moq_track_producer`
//! is `moq.TrackProducer`, and its method `moq_track_producer_append_group` is
//! `TrackProducer.append_group`. A keyword, or a member Python gives every such class,
//! takes a trailing `_`. An object is a class owning its handle, freed once its last
//! reference goes; a record a class written as a dataclass would make it; an enum an
//! `Enum`, or for one with data a class with such a record per variant; a trait an abstract
//! class; a closure any callable; a reader a `ForeignReader`; a future an awaitable; an
//! error an exception, with a subclass per variant of an enum. The runtime they
//! share, [`runtime/rt.py`](../runtime/rt.py), holds the awaitable and the queue an event
//! loop reads its wakes from; the glue's, [`runtime/python.rs`](../runtime/python.rs),
//! the conversions and the types the extension defines.
//!
//! The glue converts each Python value to its C value and back itself, so a call is one
//! crossing: a record crosses as its fields, read and set by name, and bytes returned as a
//! `memoryview` lending Rust's buffer. Each function but an `async def` is the
//! extension's own, which `_register` sets on its class or the module binds, so Python
//! calls it with no frame of its own; the Python only declares it, for type checkers.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;
use std::hash::Hasher;

use rustc_data_structures::stable_hash::StableHasher;

use crate::abi::{self, Abi, Ty, pascal};
use crate::doc;
use crate::ir::{Access, Family, Function, Int, Literal, On, Order, Param, Receiver, Traits, Type};
use crate::kotlin::{indent_by, rust_c, strip_refs};

const RUNTIME: &str = include_str!("../runtime/rt.py");
const NATIVE: &str = include_str!("../runtime/native.py");
pub(crate) const GLUE_RUNTIME: &str = include_str!("../runtime/python.rs");

/// What the generator writes: the Python module, and the Rust module of the extension it
/// loads.
pub(crate) struct Output {
	pub(crate) names: BTreeMap<String, String>,
	pub(crate) python: String,
	pub(crate) glue: String,
}

/// The runtime's names the generated Python uses unqualified, which no exported item may
/// take. Its `Waker`, `Mut`, and `Bound` need no entry: the C ABI claims `<prefix>_waker`,
/// `<prefix>_mut`, and `<prefix>_bound` already.
const TAKEN: &[&str] = &[
	"_n",
	"_load",
	"_T",
	"_Future",
	"_Pollable",
	"_Wakeup",
	"_wakeups",
	"_wakeup",
	"_resume",
	"_drains",
	"_Const",
	"_PENDING",
	"_consts",
	"_abc",
	"_array",
	"_asyncio",
	"_dataclasses",
	"_Record",
	"_Dataclass",
	"_NotImplemented",
	"_RuntimeError",
	"_StopAsyncIteration",
	"_getattr",
	"_same",
	"_all",
	"_any",
	"_dict",
	"_float",
	"_int",
	"_memoryview",
	"_TypeError",
	"_list",
	"_tuple",
	"_type",
	"_zip",
	"_datetime",
	"_ipaddress",
	"_enum",
	"_socket",
	"_sys",
	"_typing",
	"_weakref",
	"_get_running_loop",
	"_property",
	"_staticmethod",
	"_isinstance",
	"_str",
	"_bytes",
	"_len",
	"_Exception",
	"_ValueError",
	"_AttributeError",
	"_BaseException",
	"_ImportError",
	"_NotImplementedError",
	"_bool",
	"_bytearray",
	"_object",
	"_range",
	"_deprecated",
	"_T",
	"ForeignReader",
	"Duration",
];

/// The module's `@_deprecated`, which `TAKEN` keeps an export from shadowing.
const DEPRECATOR: &str = "_T = _typing.TypeVar(\"_T\")\n\nif _typing.TYPE_CHECKING:\n\tfrom typing_extensions import deprecated as _deprecated\nelse:\n\n\tdef _deprecated(message: _str, /) -> _typing.Callable[[_T], _T]:\n\t\t\"\"\"Marks a deprecated item for type checkers.\"\"\"\n\t\treturn lambda item: item\n";

/// Any buffer, as Python before 3.12's `collections.abc.Buffer` spells it.
const BUFFER: &str = "_bytes | _bytearray | _memoryview";

/// A socket address as Python's `socket` module holds it: IPv4's, and IPv6's.
const V4: &str = "_tuple[_str, _int]";
const V6: &str = "_tuple[_str, _int, _int, _int]";

/// The registry's first entries, the runtime's: the glue's `T_FUTURE`, `T_DURATION`,
/// `T_RESUME`, `T_WAKER`, `T_POLLABLE`, `T_IPV4`, `T_IPV6`, and `T_DRAINS`.
const RUNTIME_TYPES: &[&str] = &[
	"_Future",
	"Duration",
	"_resume",
	"Waker",
	"_Pollable",
	"_ipaddress.IPv4Address",
	"_ipaddress.IPv6Address",
	"_drains",
];

/// The Python for `abi`, whose header's contract is `contract`, or the names two items
/// claim alike. The Python's own contract hashes the header's with everything it and its
/// glue add, as their conversions change with no C declaration changing.
pub(crate) fn generate(abi: &Abi<'_>, packages: &str, contract: u64) -> Result<Output, String> {
	let mut g = Gen {
		locked: BTreeMap::new(),
		abi,
		classes: abi.classes(|_| (String::new(), 0), |name| TAKEN.contains(&name)),
		py: String::new(),
		top: String::new(),
		consts: Vec::new(),
		natives: Vec::new(),
		glue: String::new(),
		table: Vec::new(),
		members: Vec::new(),
		types: RUNTIME_TYPES.iter().map(|t| t.to_string()).collect(),
		index: BTreeMap::new(),
		keys: BTreeSet::new(),
		exceptions: BTreeMap::new(),
		thrown: HashMap::new(),
		names: BTreeMap::new(),
		converters: HashMap::new(),
		errors: Vec::new(),
		on: On::ALL,
	};
	for name in TAKEN {
		g.names.insert(name.to_string(), "the runtime".to_string());
	}
	for name in g.classes.values() {
		g.names.insert(name.clone(), "a type".to_string());
	}
	g.locked = crate::lock::types(abi, &g.classes);
	g.write();
	if !g.errors.is_empty() {
		return Err(g.errors.join("\n"));
	}
	let p = &abi.prefix;
	let registry: String = g.types.iter().map(|t| format!("\t\t{t},\n")).collect();
	let consts: String = g
		.consts
		.iter()
		.map(|c| format!("\t\t\"{}\": _n.{},\n", c.name, c.bare))
		.collect();
	let declared: String = match g.consts.is_empty() {
		true => "\tpass\n".to_string(),
		false => g
			.consts
			.iter()
			.map(|c| format!("\t{}: {}\n{}", c.name, c.ty, docstring(&c.docs, "\t")))
			.collect(),
	};
	// What only runs: the registry, and the module's constants, declared above for type
	// checkers and made by `__getattr__` on each read.
	let mut body = format!(
		"{RUNTIME}\n{}{}if _typing.TYPE_CHECKING:\n{declared}else:\n\t_n._register(\n{registry}\t)\n\n\t_consts = {{\n{consts}\t}}\n\n\tdef __getattr__(name: _str) -> _object:\n\t\t\"\"\"A constant, made anew each time it is read, as Rust makes it.\"\"\"\n\t\tget = _consts.get(name)\n\t\tif get is None:\n\t\t\traise _AttributeError(f\"module {{__name__!r}} has no attribute {{name!r}}\")\n\t\treturn get()\n",
		g.py, g.top
	);
	let mut hasher = StableHasher::new();
	hasher.write_u64(contract);
	for part in [&body, &g.glue, GLUE_RUNTIME] {
		hasher.write(part.as_bytes());
	}
	let contract = Hasher::finish(&hasher);
	let native = indent_by(NATIVE, 1);
	let natives: String = g.natives.iter().map(|n| format!("\n{}", indent_by(n, 2))).collect();
	let head = format!(
		"# Generated by rust2-ffi: Python over the C ABI of the pub API of {packages}. Do not edit.\n\
		\"\"\"The pub API of {packages}, over the CPython extension built into the library beside this file.\"\"\"\n\n\
		from __future__ import annotations\n\n\
		import abc as _abc\n\
		import array as _array\n\
		import asyncio as _asyncio\n\
		import datetime as _datetime\n\
		import ipaddress as _ipaddress\n\
		import enum as _enum\n\
		import inspect as _inspect\n\
		import socket as _socket\n\
		import sys as _sys\n\
		import typing as _typing\n\
		import weakref as _weakref\n\
		from builtins import AttributeError as _AttributeError\n\
		from builtins import BaseException as _BaseException\n\
		from builtins import Exception as _Exception\n\
		from builtins import ImportError as _ImportError\n\
		from builtins import NotImplemented as _NotImplemented\n\
		from builtins import NotImplementedError as _NotImplementedError\n\
		from builtins import RuntimeError as _RuntimeError\n\
		from builtins import StopAsyncIteration as _StopAsyncIteration\n\
		from builtins import TypeError as _TypeError\n\
		from builtins import ValueError as _ValueError\n\
		from builtins import all as _all\n\
		from builtins import any as _any\n\
		from builtins import bool as _bool\n\
		from builtins import bytearray as _bytearray\n\
		from builtins import bytes as _bytes\n\
		from builtins import dict as _dict\n\
		from builtins import float as _float\n\
		from builtins import getattr as _getattr\n\
		from builtins import int as _int\n\
		from builtins import isinstance as _isinstance\n\
		from builtins import len as _len\n\
		from builtins import list as _list\n\
		from builtins import memoryview as _memoryview\n\
		from builtins import object as _object\n\
		from builtins import property as _property\n\
		from builtins import range as _range\n\
		from builtins import staticmethod as _staticmethod\n\
		from builtins import str as _str\n\
		from builtins import tuple as _tuple\n\
		from builtins import type as _type\n\
		from builtins import zip as _zip\n\n\
		if _typing.TYPE_CHECKING:\n\
		\timport dataclasses as _dataclasses\n\n\
		_T = _typing.TypeVar(\"_T\")\n\n\n\
		def _get_running_loop() -> _asyncio.AbstractEventLoop:\n\
		\t\"\"\"The running asyncio event loop, which every future and `Waker` of this module runs on.\"\"\"\n\
		\ttry:\n\
		\t\treturn _asyncio.get_running_loop()\n\
		\texcept _RuntimeError:\n\
		\t\traise _RuntimeError(\"{p}'s futures and wakers run on a running asyncio event loop, and none is running: trio and other event loops are unsupported\") from None\n\n\n\
		def _same(a: _object, b: _object, /) -> _bool:\n\
		\t\"\"\"Whether a and b are equal as Rust's `==` compares them: never by identity, so a NaN\n\
		\tequals nothing, while a list's, tuple's, or dict's own `==` takes an item as equal to itself.\"\"\"\n\
		\tif _isinstance(a, (_list, _tuple)):\n\
		\t\txs = _typing.cast(\"_typing.Sequence[_object]\", a)\n\
		\t\tys = _typing.cast(\"_typing.Sequence[_object]\", b)\n\
		\t\treturn _type(xs) is _type(ys) and _len(xs) == _len(ys) and _all(_same(x, y) for x, y in _zip(xs, ys))\n\
		\tif _isinstance(a, _dict):\n\
		\t\tif not _isinstance(b, _dict):\n\
		\t\t\treturn False\n\
		\t\tm = _typing.cast(\"_typing.Mapping[_object, _object]\", a)\n\
		\t\tn = _typing.cast(\"_typing.Mapping[_object, _object]\", b)\n\
		\t\treturn m.keys() == n.keys() and _all(_same(v, n[k]) for k, v in m.items())\n\
		\tif a is None or b is None or _isinstance(a, (_int, _float, _str)):\n\
		\t\treturn a == b\n\
		\ttry:\n\
		\t\t# Any buffer, through a memoryview, which compares numbers as Rust does, where\n\
		\t\t# numpy's own `==` compares item by item into another array.\n\
		\t\treturn _memoryview(_typing.cast(\"_bytes\", a)) == _memoryview(_typing.cast(\"_bytes\", b))\n\
		\texcept _TypeError:\n\
		\t\treturn a == b\n\n\n\
		def _load() -> _object:\n\
		\timport importlib.machinery\n\
		\timport importlib.util\n\
		\timport os\n\
		\timport re\n\n\
		\t# Loading by path skips the filename tag that keeps a GIL build's library out of a\n\
		\t# free-threaded interpreter, which the stable ABI does not cover and which would crash.\n\
		\t# Its extension suffixes carry the tag (`314t`, or `314td` on a POSIX debug build, after `_d` on a Windows one),\n\
		\t# where sysconfig's `Py_GIL_DISABLED` follows an overridden sysconfig data file, as a\n\
		\t# Nix shell sets.\n\
		\tif _any(re.match(r\"(?:_d)?\\.(?:cpython-|cp)\\d+td?[-.]\", s) for s in importlib.machinery.EXTENSION_SUFFIXES):\n\
		\t\traise _ImportError(\n\
		\t\t\t\"{p}.py loads a library for CPython's GIL build, which a free-threaded CPython cannot use: \"\n\
		\t\t\tf\"{{_sys.implementation.name}} {{_sys.version.partition(' ')[0]}}, free-threaded\"\n\
		\t\t)\n\n\
		\tname = {{\"win32\": \"{p}.dll\", \"darwin\": \"lib{p}.dylib\"}}.get(_sys.platform, \"lib{p}.so\")\n\
		\tpath = os.path.join(os.path.dirname(os.path.abspath(__file__)), name)\n\
		\tloader = importlib.machinery.ExtensionFileLoader(\"_{p}\", path)\n\
		\tmodule = importlib.util.module_from_spec(importlib.machinery.ModuleSpec(\"_{p}\", loader, origin=path))\n\
		\tloader.exec_module(module)\n\
		\treturn module\n\n\n\
		if _typing.TYPE_CHECKING:\n\
		{native}{natives}\
		else:\n\
		\t_n = _load()\n\
		\t# Checked before any other call: a Python file and a library from different builds\n\
		\t# must not mix.\n\
		\tif _n.contract() != 0x{contract:016x}:\n\
		\t\traise _ImportError(\"{p}.py and the library are from different builds\")\n\
		_PENDING = _n.PENDING\n\n\n"
	);
	body.insert_str(0, &head);
	if body.contains("@_deprecated(") {
		// Type checkers flag each use of a deprecated item, which they read from
		// `typing_extensions`, whose stubs they bundle, as no CPython before 3.13 has
		// `warnings.deprecated`; a call warns from the library itself. The module's own uses are
		// no consumer's.
		body = body.replacen(
			"\n",
			"\n# pyright: reportDeprecated=false\n# mypy: disable-error-code=\"deprecated\"\n",
			1,
		);
		body = body.replacen("_T = _typing.TypeVar(\"_T\")\n", DEPRECATOR, 1);
	}
	// Each link once every name is final.
	let render = |link: &doc::Link<'_>| {
		let (role, target) = g.link(link.target)?;
		Some(match link.text {
			Some(text) => format!(":{role}:`{} <{target}>`", text.replace('`', "")),
			None => format!(":{role}:`{target}`"),
		})
	};
	let body = doc::links(&body, abi.api, render);
	let glue = format!(
		"//! Generated by rust2-ffi: the CPython extension `python/{p}.py` loads, over the C ABI of {packages}.\n\n\
		#![allow(non_snake_case, non_upper_case_globals, unused_mut, unused_unsafe, unused_parens, unused_variables, clippy::all)]\n\n\
		mod rt;\n\n\
		use rt::*;\n\n\
		unsafe extern \"C\" fn f_contract(_: *mut PyObject, _: *mut *mut PyObject, _: Py_ssize_t) -> *mut PyObject {{\n\
		\trt::wrap(|| rt::uint_out(0x{contract:016x}))\n\
		}}\n\n\
		{}\
		static METHODS: rt::Methods<{}> = rt::Methods(std::cell::UnsafeCell::new([\n\
		\trt::method(c\"contract\", f_contract),\n\
		{}\
		\trt::END,\n\
		]));\n\n\
		static MEMBERS: rt::Gil<[rt::Member; {}]> = rt::Gil::new([\n\
		{}\
		]);\n\n\
		static MODULE: rt::Gil<PyModuleDef> = rt::Gil::new(PyModuleDef {{\n\
		\tm_base: PyModuleDef_HEAD_INIT,\n\
		\tm_name: c\"_{p}\".as_ptr(),\n\
		\tm_doc: std::ptr::null(),\n\
		\tm_size: -1,\n\
		\tm_methods: std::ptr::null_mut(),\n\
		\tm_slots: std::ptr::null_mut(),\n\
		\tm_traverse: None,\n\
		\tm_clear: None,\n\
		\tm_free: None,\n\
		}});\n\n\
		#[unsafe(no_mangle)]\n\
		pub unsafe extern \"C\" fn PyInit__{p}() -> *mut PyObject {{\n\
		\tunsafe {{\n\
		\t\tlet def = MODULE.get();\n\
		\t\tdef.m_methods = METHODS.ptr();\n\
		\t\trt::module(def, MEMBERS.get())\n\
		\t}}\n\
		}}\n",
		{
			let keys: String = g
				.keys
				.iter()
				.map(|k| format!("static K_{k}: rt::Key = rt::Key::new(c\"{k}\");\n"))
				.collect();
			format!("{keys}\n{}{}", g.index_consts(), g.glue)
		},
		g.table.len() + 2,
		g.table.iter().map(|t| format!("\t{t},\n")).collect::<String>(),
		g.members.len(),
		g.members.iter().map(|m| format!("\t{m},\n")).collect::<String>(),
	);
	let glue = doc::links(&glue, abi.api, render);
	Ok(Output {
		names: g.locked,
		python: body,
		glue,
	})
}

/// How a function's Python form is held.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
	/// A method of a class, taking `self`.
	Member,
	/// A static method of a class.
	Static,
	/// A module-level function.
	Top,
}

/// What a native function Python calls directly is.
struct Direct<'s> {
	/// Its Python name.
	member: &'s str,
	place: Place,
	/// Whether it is a read-only property.
	property: bool,
	/// The C name of the class it is a member of, which `_register` sets it on.
	owner: Option<&'s str>,
	/// Its parameters' Python names, after its `self`.
	params: &'s [String],
	docs: &'s str,
}

/// A module-level constant that is no literal, made by its native function on each read.
struct Const {
	name: String,
	bare: String,
	/// Its Python annotation.
	ty: String,
	docs: String,
}

/// A function's Python form.
enum Form {
	/// Python's own definition.
	Python(String),
	/// The glue's, declared for type checkers, and the statement binding it at run time
	/// unless `_register` sets it on its class.
	Native(String, Option<String>),
}

/// A class's members: those Python defines, and those the glue does, which the class
/// declares only for type checkers and binds in `runtime` unless `_register` does.
#[derive(Default)]
struct Body {
	python: Vec<String>,
	native: Vec<String>,
	runtime: Vec<String>,
}

impl Body {
	fn codes(&self) -> impl Iterator<Item = &String> {
		self.python.iter().chain(&self.native)
	}

	/// Each member's code, the glue's in one block.
	fn members(&self) -> Vec<String> {
		let mut members = self.python.clone();
		if !self.native.is_empty() {
			let native: Vec<String> = self.native.iter().map(|m| indent_by(m, 1)).collect();
			members.push(format!("\tif _typing.TYPE_CHECKING:\n{}", native.join("\n")));
		}
		if !self.runtime.is_empty() {
			let runtime: Vec<String> = self.runtime.iter().map(|m| indent_by(m, 1)).collect();
			members.push(format!("\telse:\n{}", runtime.join("")));
		}
		members
	}
}

struct Gen<'a> {
	locked: BTreeMap<String, String>,
	abi: &'a Abi<'a>,
	/// Each exported type's name, by its C name.
	classes: HashMap<String, String>,
	/// The classes, then the module-level functions.
	py: String,
	top: String,
	/// The module-level constants that are no literal.
	consts: Vec<Const>,
	/// The glue's functions the Python calls by name, declared for type checkers.
	natives: Vec<String>,
	glue: String,
	/// The extension's method table entries.
	table: Vec<String>,
	/// The members `_register` sets on their classes.
	members: Vec<String>,
	/// The classes the glue makes values of, in the order the Python registers them.
	types: Vec<String>,
	/// Each registered class's index, by the C name or key it is registered under.
	index: BTreeMap<String, usize>,
	/// The attribute names the glue reads and sets.
	keys: BTreeSet<String>,
	/// The exception class of each error type, by name.
	exceptions: BTreeMap<String, Type>,
	/// Each exception class's name, by the name it would take but for a clash.
	thrown: HashMap<String, String>,
	/// Each module-level Python name, with what claimed it.
	names: BTreeMap<String, String>,
	/// The suffix of the glue's `r_` and `w_` of each option, tuple, and list type by
	/// its C struct and what the struct holds: types of one struct, such as a `Vec` of
	/// `SocketAddrV4` and one of `SocketAddr`, convert apart.
	converters: HashMap<(String, Vec<Type>), String>,
	errors: Vec<String>,
	/// The targets of the item being written, whose native functions check theirs.
	on: On,
}

impl<'a> Gen<'a> {
	/// A C name without the prefix.
	fn bare<'n>(&self, c: &'n str) -> &'n str {
		&c[self.abi.prefix.len() + 1..]
	}

	/// The Python class name of the C type `c`.
	fn class(&self, c: &str) -> String {
		self.classes.get(c).cloned().unwrap_or_else(|| pascal(self.bare(c)))
	}

	/// The module-level Python name `what` takes, `name` unless that is taken.
	fn claim(&mut self, name: &str, what: String) -> String {
		abi::claim(&mut self.names, |_| false, name, what)
	}

	/// Register the class the Python expression `py` names under `key`, returning the
	/// glue's constant for its index.
	fn register(&mut self, key: &str, py: String) -> String {
		if !self.index.contains_key(key) {
			self.index.insert(key.to_string(), self.types.len());
			self.types.push(py);
		}
		format!("T_{key}")
	}

	/// The glue's constant for the index of the class registered under `key`.
	fn t(&self, key: &str) -> String {
		format!("T_{key}")
	}

	fn index_consts(&self) -> String {
		let mut out = String::new();
		for (key, n) in &self.index {
			let _ = writeln!(out, "const T_{key}: usize = {n};");
		}
		out.push('\n');
		out
	}

	/// The glue's interned attribute name `name`.
	fn key(&mut self, name: &str) -> String {
		self.keys.insert(name.to_string());
		format!("&K_{name}")
	}

	/// The Python annotation of `ty` as a value, owning what it holds.
	fn py_ty(&self, ty: &Type) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Array(n) => Abi::array_class(*n),
			Type::Unit => "None".to_string(),
			Type::Bool => "_bool".to_string(),
			Type::Int(_) | Type::NonZero(_) | Type::U128 => "_int".to_string(),
			Type::F32 | Type::F64 => "_float".to_string(),
			Type::Char | Type::Str | Type::String | Type::Path | Type::PathBuf => "_str".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "_memoryview".to_string(),
			Type::Duration => "Duration".to_string(),
			Type::SocketAddr(family) => match family {
				Family::V4 => V4.to_string(),
				Family::V6 => V6.to_string(),
				Family::Any => format!("{V4} | {V6}"),
			},
			Type::IpAddr(Family::V4) => "_ipaddress.IPv4Address".to_string(),
			Type::IpAddr(Family::V6) => "_ipaddress.IPv6Address".to_string(),
			Type::IpAddr(Family::Any) => "_ipaddress.IPv4Address | _ipaddress.IPv6Address".to_string(),
			Type::List(_, inner) if ty.block().is_some() => format!("_array.array[{}]", self.py_ty(inner)),
			Type::List(_, inner) => format!("_list[{}]", self.py_ty(inner)),
			Type::Waker => "Waker".to_string(),
			Type::Reader => "ForeignReader".to_string(),
			Type::Future => "_typing.Awaitable[None]".to_string(),
			Type::Option(inner) => format!("{} | None", self.py_ty(inner)),
			Type::Box(inner) | Type::Ref(_, inner) => self.py_ty(inner),
			Type::Record(i) => self.class(&abi.record(*i)),
			Type::Enum(i) => self.class(&abi.enumeration(*i)),
			Type::Object(i) => self.class(&abi.object(*i)),
			Type::Callback(i, _) => self.class(&abi.callback(*i)),
			Type::Closure(closure_) => {
				let (params, ret) = (&closure_.params, &closure_.ok);
				let params: Vec<String> = params.iter().map(|p| self.py_lent(p)).collect();
				format!("_typing.Callable[[{}], {}]", params.join(", "), self.py_param(ret))
			}
			Type::Range(_) => "_range".to_string(),
			Type::Bound(_) | Type::Bounds(_) => self.class(&abi.range(ty)),
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.py_ty(e)).collect();
				format!("_tuple[{}]", elements.join(", "))
			}
		}
	}

	/// The Python annotation of a parameter of type `ty`, or of what Python returns to Rust:
	/// its bytes are any buffer, its path any path, and its list any iterable, which
	/// `rt::list_in` reads once.
	fn py_param(&self, ty: &Type) -> String {
		self.py_in(ty, "_typing.Iterable")
	}

	/// The Python annotation of a record's field of type `ty`: as a parameter's, but its list
	/// is a sequence, since the record keeps it and Rust may read it more than once.
	fn py_field(&self, ty: &Type) -> String {
		self.py_in(ty, "_typing.Sequence")
	}

	/// The Python annotation of a value of type `ty` that Python passes in, each list as the
	/// covariant container `list` names, so a caller's `list` of one alternative of a widened
	/// element type checks.
	fn py_in(&self, ty: &Type, list: &str) -> String {
		match ty {
			Type::Slice | Type::Vec | Type::Bytes => BUFFER.to_string(),
			Type::Path | Type::PathBuf => "_str".to_string(),
			// Python's own IPv6 address may leave out its flowinfo and scope_id.
			Type::SocketAddr(Family::V6) => format!("{V4} | {V6}"),
			// Any buffer of its numbers, as numpy's arrays are too.
			Type::List(..) if ty.block().is_some() => format!("{} | _memoryview", self.py_ty(ty)),
			Type::List(_, inner) => format!("{list}[{}]", self.py_in(inner, list)),
			Type::Option(inner) => format!("{} | None", self.py_in(inner, list)),
			Type::Box(inner) | Type::Ref(_, inner) => self.py_in(inner, list),
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.py_in(e, list)).collect();
				format!("_tuple[{}]", elements.join(", "))
			}
			_ => self.py_ty(ty),
		}
	}

	/// The glue's expression, an `R<C>`, reading the Python object `o` as the C value of
	/// `ty`, with what it borrows held in the scope `s`; `t` is the glue's `bool`, whether
	/// Rust takes what the position holds, as it takes a value passed by value but not one
	/// read through a pointer or an array's item, so a buffer crosses there as its own.
	fn lift(&mut self, ty: &Type, o: &str, t: &str) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Array(n) => format!(
				"rt::array_in::<{n}>({o}).map(|items| crate::{} {{ items }})",
				abi.array(*n)
			),
			Type::Bool => format!("rt::bool_in({o})"),
			Type::Int(int) => format!("rt::int_in::<{}>({o})", int.rust()),
			Type::NonZero(int) => format!("rt::nonzero_in::<{}>({o})", int.rust()),
			Type::F32 => format!("rt::f32_in({o})"),
			Type::F64 => format!("rt::float_in({o})"),
			Type::Char => format!("rt::char_in({o})"),
			Type::Str | Type::String => format!("rt::str_in({o})"),
			Type::Slice | Type::Vec | Type::Bytes => format!("rt::bytes_in({o}, s, {t})"),
			Type::Path | Type::PathBuf => format!("rt::path_in({o}, s)"),
			Type::U128 => format!("rt::u128_in({o})"),
			Type::Duration => format!("rt::duration_in({o})"),
			Type::SocketAddr(family) | Type::IpAddr(family) => format!("rt::{}_in({o})", addr_fn(ty, *family)),
			Type::List(_, inner) if ty.block().is_some() => format!("rt::block_in::<{}>({o}, s)", number(inner)),
			Type::List(..) => format!("r_{}({o}, s, {t})", self.converter(ty)),
			Type::Option(inner) => match inner.base() {
				Type::Object(i) => format!("rt::hold_opt({o}, {}, s)", self.t(&abi.object(*i))),
				base if Abi::is_buf(base) => {
					let some = self.lift(inner, o, t);
					format!("(if rt::is_none({o}) {{ Ok(crate::rt::Buf::NONE) }} else {{ {some} }})")
				}
				Type::List(..) => {
					let some = self.lift(inner, o, t);
					format!("(if rt::is_none({o}) {{ Ok(crate::rt::List::NONE) }} else {{ {some} }})")
				}
				_ => format!("r_{}({o}, s, {t})", self.converter(ty)),
			},
			Type::Box(inner) => self.lift(inner, o, t),
			Type::Ref(_, inner) => self.lift(inner, o, t),
			Type::Record(_) => format!("r_{}({o}, s, {t})", self.c_struct(ty)),
			Type::Enum(i) if abi.api.enums[*i].variants.iter().any(|v| !abi.api.everywhere(v.on)) => {
				// Checks the member a value is against the library's target.
				let c = abi.enumeration(*i);
				match abi.api.enums[*i].plain() {
					true => format!("rt::member_in({o}, {}).and_then(on_{c})", self.t(&c)),
					false => format!("r_{c}({o}, s, {t})"),
				}
			}
			Type::Enum(i) => match abi.api.enums[*i].plain() {
				true => format!("rt::member_in({o}, {})", self.t(&abi.enumeration(*i))),
				false => format!("r_{}({o}, s, {t})", abi.enumeration(*i)),
			},
			Type::Object(i) => format!("rt::hold({o}, {}, s)", self.t(&abi.object(*i))),
			Type::Callback(i, _) => {
				let c = abi.callback(*i);
				format!(
					"Ok(crate::{c} {{ vtable: &VTABLE_{c}, data: rt::callback({o}, {}, s)? }})",
					self.t(&c)
				)
			}
			Type::Waker => format!("rt::waker_in({o}, s)"),
			Type::Reader => format!("rt::reader_in({o})"),
			Type::Future => format!("rt::future_in({o}, s)"),
			Type::Range(int) => format!("rt::range_in::<{}>({o})", int.rust()),
			Type::Bound(_) | Type::Bounds(_) => format!("r_{}({o}, s)", abi.range(ty)),
			Type::Tuple(_) => format!("r_{}({o}, s, {t})", self.converter(ty)),
			Type::Unit | Type::Closure(..) => unreachable!("{ty:?} is no value Python passes"),
		}
	}

	/// The glue's expression, an `R<Obj>`, of the C value `v` of `ty`, which Rust handed
	/// over: what it owns is freed or moves into the Python value.
	fn lower(&mut self, ty: &Type, v: &str) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Array(n) => format!("rt::array_out(&{v}.items, {})", self.t(&abi.array(*n))),
			Type::Unit => "rt::none()".to_string(),
			Type::Bool => format!("rt::bool_out({v})"),
			Type::Int(int) | Type::NonZero(int) if signed(*int) => format!("rt::int_out({v} as i64)"),
			Type::Int(_) | Type::NonZero(_) => format!("rt::uint_out({v} as u64)"),
			Type::F32 => format!("rt::f32_out({v})"),
			Type::F64 => format!("rt::float_out({v})"),
			Type::Char => format!("rt::char_out({v})"),
			Type::Str | Type::String => format!("rt::str_out({v})"),
			Type::Slice | Type::Vec | Type::Bytes => format!("rt::bytes_out({v})"),
			Type::Path | Type::PathBuf => format!("rt::path_out({v})"),
			Type::U128 => format!("rt::u128_out({v})"),
			Type::Duration => format!("rt::duration_out({v})"),
			Type::SocketAddr(family) | Type::IpAddr(family) => format!("rt::{}_out({v})", addr_fn(ty, *family)),
			Type::List(_, inner) if ty.block().is_some() => format!("rt::block_out::<{}>({v})", number(inner)),
			Type::List(..) => format!("w_{}({v})", self.converter(ty)),
			Type::Option(inner) => match inner.base() {
				Type::Object(i) => format!(
					"rt::object_opt({v}, {}, crate::{}_free)",
					self.t(&abi.object(*i)),
					abi.object(*i)
				),
				base if Abi::is_buf(base) || matches!(base, Type::List(..)) => {
					let some = self.lower(inner, "v");
					format!("{{ let v = {v}; if v.is_none() {{ rt::none() }} else {{ {some} }} }}")
				}
				_ => format!("w_{}({v})", self.converter(ty)),
			},
			Type::Box(inner) | Type::Ref(_, inner) => self.lower(inner, v),
			Type::Record(_) => format!("w_{}({v})", self.c_struct(ty)),
			Type::Enum(i) => match abi.api.enums[*i].plain() {
				true => format!("rt::member({}, {v} as i64)", self.t(&abi.enumeration(*i))),
				false => format!("w_{}({v})", abi.enumeration(*i)),
			},
			Type::Object(i) => format!(
				"rt::object({v}, {}, crate::{}_free)",
				self.t(&abi.object(*i)),
				abi.object(*i)
			),
			Type::Range(int) => {
				let start = self.lower(&Type::Int(*int), "r.start");
				let end = self.lower(&Type::Int(*int), "r.end");
				format!("{{ let r = {v}; rt::range_out({start}, {end}) }}")
			}
			Type::Bound(_) | Type::Bounds(_) => format!("w_{}({v})", abi.range(ty)),
			Type::Tuple(_) => format!("w_{}({v})", self.converter(ty)),
			Type::Waker => format!("rt::waker_out({v})"),
			Type::Callback(..) | Type::Closure(..) | Type::Future | Type::Reader => {
				unreachable!("{ty:?} is no value Rust returns")
			}
		}
	}

	/// The C name of the struct `ty` crosses as.
	fn c_struct(&self, ty: &Type) -> String {
		match self.abi.ty(&strip_refs(ty)) {
			Ty::Struct(name) => name,
			other => unreachable!("{ty:?} crosses as {other:?}, no struct"),
		}
	}

	/// The suffix of the glue's `r_` and `w_` converting the option, tuple, or list `ty`,
	/// written here the first time, so a block's struct gets no item-by-item glue: its C
	/// struct's name for the type the struct was made for, else a name of its own.
	fn converter(&mut self, ty: &Type) -> String {
		let abi = self.abi;
		let c = self.c_struct(ty);
		let (held, made): (Vec<Type>, Vec<Type>) = match strip_refs(ty) {
			Type::Option(inner) => (vec![*inner], vec![abi.options[&c].clone()]),
			Type::List(_, elem) => (vec![*elem], vec![abi.lists[&c].clone()]),
			Type::Tuple(elements) => (elements, abi.tuples[&c].clone()),
			other => unreachable!("{other:?} has no converter"),
		};
		let held: Vec<Type> = held.iter().map(strip_refs).collect();
		let key = (c.clone(), held.clone());
		if let Some(name) = self.converters.get(&key) {
			return name.clone();
		}
		// A numbered name must miss every converter's: each type's own, and the others numbered.
		let taken = |name: &str| {
			abi.options.contains_key(name)
				|| abi.lists.contains_key(name)
				|| abi.tuples.contains_key(name)
				|| abi.ranges.contains_key(name)
				|| abi.arrays.contains_key(name)
				|| (0..abi.api.records.len()).any(|i| abi.record(i) == name)
				|| (0..abi.api.enums.len()).any(|i| abi.enumeration(i) == name)
				|| (0..abi.api.objects.len()).any(|i| abi.object(i) == name)
				|| self.converters.values().any(|n| n == name)
		};
		let made: Vec<Type> = made.iter().map(strip_refs).collect();
		let name = match held == made {
			true => c.clone(),
			false => (self.converters.len() + 2..)
				.map(|k| format!("{c}_{k}"))
				.find(|name| !taken(name))
				.expect("some number is free"),
		};
		self.converters.insert(key, name.clone());
		match strip_refs(ty) {
			Type::Option(_) => self.option(&name, &c, &held[0]),
			Type::List(..) => self.list(&name, &c, &held[0]),
			_ => self.tuple(&name, &c, &held),
		}
		name
	}

	fn write(&mut self) {
		let abi = self.abi;
		let api = abi.api;

		if abi.ranges.values().any(|ty| matches!(ty, Type::Bound(_))) {
			let bound = abi.rt("bound");
			self.register(&bound, "Bound".to_string());
			self.py.push_str(
				"class Bound(_enum.Enum):\n\t\"\"\"What a range's start or end bound does: nothing, include its value, or exclude it.\"\"\"\n\n\tUNBOUNDED = 0\n\tINCLUDED = 1\n\tEXCLUDED = 2\n\n\n",
			);
		}
		for (c, ty) in &abi.ranges.clone() {
			self.range(c, ty);
		}
		for (c, n) in &abi.arrays {
			let name = Abi::array_class(*n);
			self.register(c, name.clone());
			let _ = write!(
				self.py,
				"class {name}(_bytes):\n\t\"\"\"A Rust `[u8; {n}]`: exactly {n} bytes.\"\"\"\n\n\t__slots__ = ()\n\n\tdef __new__(cls, value: {BUFFER}) -> _typing.Self:\n\t\tself = _bytes.__new__(cls, value)\n\t\tif _len(self) != {n}:\n\t\t\traise _ValueError(f\"{{_len(self)}} bytes, not {n}\")\n\t\treturn self\n\n\n"
			);
		}
		for constant in &api.constants {
			let name = self.bare(&abi.name(&constant.path)).to_uppercase();
			let name = self.claim(&name, format!("const `{}`", constant.path.join("::")));
			self.locked
				.insert(format!("const {}", constant.path.join("::")), name.clone());
			let value = match constant.value {
				Literal::Bool(true) => "True".to_string(),
				Literal::Bool(false) => "False".to_string(),
				Literal::Int(_, v) => v.to_string(),
			};
			let _ = write!(
				self.top,
				"{name} = {value}\n{}\n",
				docstring(&deprecation(&constant.docs, constant.deprecated.as_deref()), "")
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
		// The glue's functions, declared for type checkers and bound to its own.
		let (mut declared, mut bound) = (Vec::new(), Vec::new());
		for function in &api.functions {
			match self.function(function, None, None, Place::Top) {
				Some(Form::Native(code, binding)) => {
					bound.push(indent_by(&binding.expect("a module's function is bound"), 1));
					declared.push(indent_by(&code, 1));
				}
				Some(Form::Python(code)) => {
					self.top.push_str(&code);
					self.top.push_str("\n\n");
				}
				None => {}
			}
		}
		if !declared.is_empty() {
			let _ = write!(
				self.top,
				"if _typing.TYPE_CHECKING:\n{}\nelse:\n{}\n\n",
				declared.join("\n\n"),
				bound.concat()
			);
		}
		for (name, ty) in &self.exceptions.clone() {
			self.exception_class(name, ty);
		}
	}

	/// The exception class `name` of an error of type `ty`: for an enum's, or an object's
	/// that is one, a subclass per variant, such as `ErrorException.Timeout`, which the
	/// class makes of a value of that variant.
	fn exception_class(&mut self, name: &str, ty: &Type) {
		let py = self.py_ty(ty);
		let abi = self.abi;
		let key = crate::lock::error_key(abi, ty);
		let ty = strip_box(&strip_refs(ty));
		// The Rust type, its variants' names, how `__new__` picks the subclass of `error`, and
		// what the class declares for type checkers besides each subclass.
		let (rust, variants, pick, declared) = match ty {
			Type::Enum(i) if !abi.api.enums[i].variants.is_empty() => {
				let e = &abi.api.enums[i];
				// By what tells a variant apart: a plain enum's member, or the class of a
				// variant of one with data.
				let key = match e.plain() {
					true => "error",
					false => "_type(error)",
				};
				let names: Vec<&str> = e.variants.iter().map(|v| v.name.as_str()).collect();
				let pick = format!("_{name}_variants.get({key}, cls)");
				(abi.api.rust(&e.path, &e.args), names, pick, String::new())
			}
			Type::Object(i) if !abi.api.objects[i].variants.is_empty() => {
				let object = &abi.api.objects[i];
				let declared = self.variant(i, name, &py);
				let names: Vec<&str> = object.variants.iter().map(|v| v.name.as_str()).collect();
				let pick = format!("_{name}_variants[{name}._variant(error)]");
				(abi.api.rust(&object.path, &object.args), names, pick, declared)
			}
			_ => {
				let _ = write!(
					self.py,
					"class {name}(_Exception):\n\t\"\"\"A `{py}` Rust returned as an error, in `error`, whose `str` is the exception's.\"\"\"\n\n\tdef __init__(self, error: {py}) -> None:\n\t\t_Exception.__init__(self, error)\n\t\tself.error = error\n\n\tdef __str__(self) -> _str:\n\t\treturn _str(self.error)\n\n\n"
				);
				return;
			}
		};
		let vnames = variant_classes(variants.iter().copied());
		// Kept apart by order, a subclass's name is locked, so a variant added before one
		// whose name it takes breaks the baseline.
		for (variant, vname) in variants.iter().zip(&vnames) {
			self.locked
				.insert(format!("{key} / variant {variant}"), format!("{name}.{vname}"));
		}
		let aliases: String = (vnames.iter())
			.map(|v| format!("\t\t{v}: _typing.TypeAlias = \"_{name}_{v}\"\n"))
			.collect();
		let _ = write!(
			self.py,
			"class {name}(_Exception):\n\t\"\"\"A `{py}` Rust returned as an error, in `error`, whose `str` is the exception's.\n\n\tEach variant raises its own subclass, such as `{name}.{}`, which `{name}(error)`\n\tmakes too.\"\"\"\n\n\tdef __new__(cls, error: {py}) -> _typing.Self:\n\t\tvariant = {pick} if cls is {name} else cls\n\t\treturn _typing.cast(_typing.Self, _Exception.__new__(variant, error))\n\n\tdef __init__(self, error: {py}) -> None:\n\t\t_Exception.__init__(self, error)\n\t\tself.error = error\n\n\tdef __str__(self) -> _str:\n\t\treturn _str(self.error)\n\n\tif _typing.TYPE_CHECKING:\n{declared}{aliases}\n\n",
			vnames[0]
		);
		for (vname, variant) in vnames.iter().zip(&variants) {
			let class = format!("_{name}_{vname}");
			let _ = write!(
				self.py,
				"class {class}({name}):\n\t\"\"\"`{rust}::{variant}` Rust returned as an error.\"\"\"\n\n\n{class}.__name__ = \"{vname}\"\n{class}.__qualname__ = \"{name}.{vname}\"\nif not _typing.TYPE_CHECKING:\n\t{name}.{vname} = {class}\n\n\n"
			);
		}
		let classes: Vec<String> = vnames.iter().map(|v| format!("\t_{name}_{v},\n")).collect();
		let _ = match ty {
			// An object's variant is its index plus one, or `0` for one the export knows no
			// name of, which raises the class itself.
			Type::Object(_) => write!(
				self.py,
				"_{name}_variants: _tuple[_type[{name}], ...] = (\n\t{name},\n{})\n\n\n",
				classes.concat()
			),
			_ => {
				let e = match ty {
					Type::Enum(i) => &abi.api.enums[i],
					_ => unreachable!("an enum"),
				};
				let entries: Vec<String> = (e.variants.iter().zip(&vnames))
					.map(|(v, vname)| match e.plain() {
						true => format!("\t{py}.{}: _{name}_{vname},\n", abi::snake(&v.name).to_uppercase()),
						false => format!("\t_{py}_{vname}: _{name}_{vname},\n"),
					})
					.collect();
				let by = match e.plain() {
					true => py.clone(),
					false => format!("_type[{py}]"),
				};
				write!(
					self.py,
					"_{name}_variants: _dict[{by}, _type[{name}]] = {{\n{}}}\n\n\n",
					entries.concat()
				)
			}
		};
	}

	/// The static method `_variant` of the exception class `name` of object `i`'s errors,
	/// of Python type `py`, telling which variant one holds by its index plus one, or `0`
	/// for one the export knows no name of; returning its declaration, for type checkers.
	/// No Rust name reaches the exception class, so none clashes with it.
	fn variant(&mut self, i: usize, name: &str, py: &str) -> String {
		let ty = Type::Object(i);
		let c = self.abi.object(i);
		let bare = format!("{}_variant", self.bare(&c));
		let on = std::mem::replace(&mut self.on, self.abi.api.objects[i].on);
		let this = self.lift(
			&Type::Ref(crate::ir::Mutability::Not, Box::new(ty.clone())),
			"a[0]",
			"false",
		);
		let (status, arg, check) = match self.abi.traits_status_of(&ty) {
			true => (
				"let mut status = 0;\n\t\t",
				", &mut status",
				format!("\n\t\ts.status(status, \"{c}_variant\")?;"),
			),
			false => ("", "", String::new()),
		};
		let body = self.checked(
			&bare,
			&format!(
				"let this = {this}?;\n\t\t{status}let ret = rt::call(|| crate::variant::{c}(this{arg}));{check}\n\t\ts.done();\n\t\trt::uint_out(ret as u64)"
			),
		);
		let _ = write!(
			self.glue,
			"unsafe extern \"C\" fn variant_{c}(_: *mut PyObject, a: *const *mut PyObject, n: Py_ssize_t, kw: *mut PyObject) -> *mut PyObject {{\n\trt::wrap(|| unsafe {{\n\t\tlet a = rt::params::<1>(None, a, n, kw, &[\"error\"], \"{name}._variant\")?;\n\t\tlet s = &mut rt::Scope::default();\n\t\t{body}\n\t}})\n}}\n\n"
		);
		let docs = "Which variant `error` holds, by its index plus one, or `0` for one no name is known of.";
		let doc = c_str(&format!("_variant(error)\n--\n\n{docs}"));
		let t = self.t(&format!("exc_{name}"));
		self.members.push(format!(
			"rt::Member::Static({t}, rt::function(c\"_variant\", {doc}, variant_{c}))"
		));
		self.on = on;
		format!(
			"\t\t@_staticmethod\n\t\tdef _variant(error: {py}, /) -> _int:\n\t\t\t\"\"\"{docs}\"\"\"\n\t\t\t...\n\n"
		)
	}

	/// A range's bound or pair of bounds as a record, and its glue; a `Range` is
	/// Python's `range`.
	fn range(&mut self, c: &str, ty: &Type) {
		let name = self.class(c);
		let (fields, read, write) = match ty {
			Type::Range(_) => return,
			Type::Bound(int) => {
				let bound = self.abi.rt("bound");
				let (kind, value) = (self.key("kind"), self.key("value"));
				let t_bound = self.t(&bound);
				(
					vec![("kind", "Bound".to_string()), ("value", "_int".to_string())],
					format!(
						"let k = rt::field(o, {t}, {kind}, s)?;\n\tlet v = rt::attr(o, {value}, s)?;\n\tOk(crate::{c} {{ kind: rt::member_in(k, {t_bound})?, value: rt::int_in::<{r}>(v)? }})",
						t = self.t(c),
						r = int.rust()
					),
					format!(
						"rt::record({t}, vec![({kind}, rt::member({t_bound}, v.kind as i64)), ({value}, {})])",
						self.lower(&Type::Int(*int), "v.value"),
						t = self.t(c)
					),
				)
			}
			Type::Bounds(int) => {
				let bound = self.abi.range(&Type::Bound(*int));
				let (start, end) = (self.key("start"), self.key("end"));
				let class = self.class(&bound);
				(
					vec![("start", class.clone()), ("end", class)],
					format!(
						"let a = rt::field(o, {t}, {start}, s)?;\n\tlet b = rt::attr(o, {end}, s)?;\n\tOk(crate::{c} {{ start: r_{bound}(a, s)?, end: r_{bound}(b, s)? }})",
						t = self.t(c)
					),
					format!(
						"rt::record({t}, vec![({start}, w_{bound}(v.start)), ({end}, w_{bound}(v.end))])",
						t = self.t(c)
					),
				)
			}
			other => unreachable!("{other:?} is no range"),
		};
		self.register(c, name.clone());
		let fields: Vec<Slot> = (fields.into_iter())
			.map(|(name, ty)| Slot {
				name: name.to_string(),
				ty,
				docs: String::new(),
				same: false,
			})
			.collect();
		let _ = write!(self.py, "{}\n\n", record_class(&name, "_Record", "", &fields, &[]));
		let _ = write!(
			self.glue,
			"fn r_{c}(o: *mut PyObject, s: &mut rt::Scope) -> R<crate::{c}> {{\n\t{read}\n}}\n\nfn w_{c}(v: crate::{c}) -> R<Obj> {{\n\t{write}\n}}\n\n"
		);
	}

	/// The glue of an option struct `c` holding `inner`, as `r_<name>` and `w_<name>`.
	fn option(&mut self, name: &str, c: &str, inner: &Type) {
		let read = self.lift(inner, "o", "take");
		let write = self.lower(inner, "v.value");
		let _ = write!(
			self.glue,
			"fn r_{name}(o: *mut PyObject, s: &mut rt::Scope, take: bool) -> R<crate::{c}> {{\n\tmatch rt::is_none(o) {{\n\t\ttrue => Ok(crate::{c} {{ some: false, value: unsafe {{ std::mem::zeroed() }} }}),\n\t\tfalse => Ok(crate::{c} {{ some: true, value: {}? }}),\n\t}}\n}}\n\nfn w_{name}(v: crate::{c}) -> R<Obj> {{\n\tmatch v.some {{\n\t\ttrue => {write},\n\t\tfalse => rt::none(),\n\t}}\n}}\n\n",
			read
		);
	}

	/// The glue of a tuple struct `c`, element by element, as `r_<name>` and `w_<name>`.
	fn tuple(&mut self, name: &str, c: &str, elements: &[Type]) {
		let n = elements.len();
		let mut reads = Vec::new();
		let mut fields = Vec::new();
		let mut writes = Vec::new();
		for (k, ty) in elements.iter().enumerate() {
			let lift = self.lift(ty, &format!("x{k}"), "take");
			reads.push(format!("let x{k} = rt::tuple_item(o, {k}, {n})?;"));
			fields.push(format!("_{k}: {lift}?"));
			writes.push(self.lower(ty, &format!("v._{k}")));
		}
		let _ = write!(
			self.glue,
			"fn r_{name}(o: *mut PyObject, s: &mut rt::Scope, take: bool) -> R<crate::{c}> {{\n\t{}\n\tOk(crate::{c} {{ {} }})\n}}\n\nfn w_{name}(v: crate::{c}) -> R<Obj> {{\n\trt::tuple(vec![{}])\n}}\n\n",
			reads.join("\n\t"),
			fields.join(", "),
			writes.join(", ")
		);
	}

	/// The glue of a `Vec`'s array `c`, as `r_<name>` and `w_<name>`.
	fn list(&mut self, name: &str, c: &str, elem: &Type) {
		// An array's items are behind its pointer, which Rust only reads.
		let read = self.lift(elem, "o", "false");
		let write = self.lower(elem, "e");
		let _ = write!(
			self.glue,
			"fn r_{name}(o: *mut PyObject, s: &mut rt::Scope, take: bool) -> R<crate::{c}> {{\n\trt::list_in(o, s, |o, s| {read})\n}}\n\nfn w_{name}(v: crate::{c}) -> R<Obj> {{\n\trt::list_out(v, |e| {write})\n}}\n\n"
		);
	}

	/// The Python members a type's std traits export beyond Python's own, with their glue:
	/// `__str__` from `Display`, `__repr__` from `Debug`, a static `default` from
	/// `Default`, and `clone`, `__eq__`, and `__hash__` from `Hash` for an object.
	fn traits(&mut self, ty: &Type, c: &str, t: Traits, body: &mut Body) {
		let bare = self.bare(c).to_string();
		let class = self.class(c);
		let object = matches!(ty, Type::Object(_));
		let opaque = matches!(ty, Type::Record(i) if self.abi.api.records[*i].opaque.is_some());
		// The receiver as the glue reads it: a handle, held for the call, or a value.
		let this = match object {
			true => self.lift(
				&Type::Ref(crate::ir::Mutability::Not, Box::new(ty.clone())),
				"a[0]",
				"false",
			),
			false => self.lift(ty, "a[0]", "false"),
		};
		let pass = match object {
			true => "this",
			false => "&this",
		};
		// An object whose run-time checks may fail passes each its status, which raises.
		let checked = self.abi.traits_status_of(ty);
		let (status, arg) = match checked {
			true => ("let mut status = 0;\n\t\t", ", &mut status"),
			false => ("", ""),
		};
		let check = |suffix: &str| match checked {
			true => format!("\n\t\ts.status(status, \"{c}_{suffix}\")?;"),
			false => String::new(),
		};
		// A leaf keeps the GIL, which no part of it can wait on: see `Abi::leaf`.
		let called = |leaf: bool, call: String| match leaf {
			true => call,
			false => format!("rt::call(|| {call})"),
		};
		let unary = |g: &mut Self, suffix: &str, member: &str, docs: &str, lower: String| {
			let call = called(
				suffix == "hash" && g.abi.leaf_trait(ty, true),
				format!("crate::{c}_{suffix}({pass}{arg})"),
			);
			let d = Direct {
				member,
				place: Place::Member,
				property: false,
				owner: Some(c),
				params: &[],
				docs,
			};
			g.direct(
				&format!("{bare}_{suffix}"),
				d,
				&format!(
					"let this = {this}?;\n\t\t{status}let ret = {call};{}\n\t\ts.done();\n\t\t{lower}",
					check(suffix)
				),
			);
		};
		if t.display {
			unary(self, "to_string", "__str__", "", "rt::str_out(ret)".to_string());
			body.native.push("\tdef __str__(self) -> _str: ...\n".to_string());
		}
		if t.debug {
			unary(self, "debug", "__repr__", "", "rt::str_out(ret)".to_string());
			body.native.push("\tdef __repr__(self) -> _str: ...\n".to_string());
		}
		if object && t.clone {
			let lower = self.lower(ty, "ret");
			let docs = "A new handle to a clone.";
			unary(self, "clone", "clone", docs, lower);
			body.native.push(format!(
				"\tdef clone(self) -> {class}:\n\t\t\"\"\"{docs}\"\"\"\n\t\t...\n"
			));
		}
		// An opaque value compares through Rust, as an object does; a record compares its
		// fields, as `record_class` writes.
		if (object || opaque) && t.eq {
			let other = match object {
				true => self.lift(
					&Type::Ref(crate::ir::Mutability::Not, Box::new(ty.clone())),
					"a[1]",
					"false",
				),
				false => self.lift(ty, "a[1]", "false"),
			};
			let (a, b) = match object {
				true => ("this", "other"),
				false => ("&this", "&other"),
			};
			let call = called(self.abi.leaf_trait(ty, false), format!("crate::{c}_eq({a}, {b}{arg})"));
			self.native(
				&format!("{bare}_eq"),
				2,
				&format!(
					"let this = {this}?;\n\t\tlet other = {other}?;\n\t\t{status}let ret = {call};{}\n\t\ts.done();\n\t\trt::bool_out(ret)",
					check("eq")
				),
			);
			// Python makes a class that defines `__eq__` alone unhashable, as `Eq` without `Hash` is.
			body.python.push(format!(
				"\tdef __eq__(self, other: _object) -> _bool:\n\t\treturn _isinstance(other, {class}) and _n.{bare}_eq(self, other)\n"
			));
			self.natives.push(format!(
				"@_staticmethod\ndef {bare}_eq(this: {class}, other: {class}, /) -> _bool: ...\n"
			));
		}
		// Every type orders through Rust's `partial_cmp`, as Rust's own `<` and its kin do:
		// -1, 0, 1, or 2 when unordered, which none of them is.
		if t.ord != Order::None {
			let other = match object {
				true => self.lift(
					&Type::Ref(crate::ir::Mutability::Not, Box::new(ty.clone())),
					"a[1]",
					"false",
				),
				false => self.lift(ty, "a[1]", "false"),
			};
			let (a, b) = match object {
				true => ("this", "other"),
				false => ("&this", "&other"),
			};
			let call = called(false, format!("crate::{c}_partial_cmp({a}, {b}{arg})"));
			self.native(
				&format!("{bare}_partial_cmp"),
				2,
				&format!(
					"let this = {this}?;\n\t\tlet other = {other}?;\n\t\t{status}let ret = {call};{}\n\t\ts.done();\n\t\trt::int_out(ret.into())",
					check("partial_cmp")
				),
			);
			for (dunder, test) in [
				("__lt__", "== -1"),
				("__le__", "in (-1, 0)"),
				("__gt__", "== 1"),
				("__ge__", "in (0, 1)"),
			] {
				body.python.push(format!(
					"\tdef {dunder}(self, other: _object) -> _bool:\n\t\tif not _isinstance(other, {class}):\n\t\t\treturn _NotImplemented\n\t\treturn _n.{bare}_partial_cmp(self, other) {test}\n"
				));
			}
			self.natives.push(format!(
				"@_staticmethod\ndef {bare}_partial_cmp(this: {class}, other: {class}, /) -> _int: ...\n"
			));
		}
		// Rust's `Hash` makes it hashable again, agreeing with its `__eq__`.
		if (object || opaque) && t.hash {
			unary(self, "hash", "__hash__", "", "rt::uint_out(ret)".to_string());
			body.native.push("\tdef __hash__(self) -> _int: ...\n".to_string());
		}
		if t.default {
			let lower = self.lower(ty, "ret");
			let docs = "Rust's `Default`.";
			let d = Direct {
				member: "default",
				place: Place::Static,
				property: false,
				owner: Some(c),
				params: &[],
				docs,
			};
			self.direct(
				&format!("{bare}_default"),
				d,
				&format!("let ret = rt::call(|| crate::{c}_default());\n\t\ts.done();\n\t\t{lower}"),
			);
			body.native.push(format!(
				"\t@_staticmethod\n\tdef default() -> {class}:\n\t\t\"\"\"{docs}\"\"\"\n\t\t...\n"
			));
		}
	}

	/// The methods of a type, each a member or a static method by its receiver, after its
	/// members `body`, checked to differ from them and its data's names `data`, which a
	/// method's name takes a trailing `_` to keep apart from, as from those of `reserved`.
	fn methods(
		&mut self,
		methods: &[Function],
		(ty, owner, rust): (&Type, &Ty, &str),
		data: &[String],
		reserved: &[&str],
		body: &mut Body,
	) {
		// A stream's async iteration takes its protocol's names, whichever method comes first.
		let streams = matches!(ty, Type::Object(_)) && methods.iter().any(|m| m.stream().is_some());
		let protocol: &[&str] = match streams {
			true => &["__aiter__", "__anext__"],
			false => &[],
		};
		for method in methods {
			let place = match method.receiver {
				Some(_) => Place::Member,
				None => Place::Static,
			};
			// A name its data or an earlier member takes, its own takes a trailing `_` against.
			let earlier: Vec<String> = body.codes().filter_map(|code| member_name(code)).collect();
			let reserved: Vec<&str> = (reserved.iter().copied())
				.chain(protocol.iter().copied())
				.chain(data.iter().map(String::as_str))
				.chain(earlier.iter().map(String::as_str))
				.collect();
			let code = match self.function(method, Some(owner), Some((ty, &reserved)), place) {
				Some(Form::Native(code, binding)) => {
					body.native.push(indent_by(&code, 1));
					body.runtime.extend(binding.map(|b| indent_by(&b, 1)));
					code
				}
				Some(Form::Python(code)) => {
					body.python.push(indent_by(&code, 1));
					code
				}
				None => continue,
			};
			if let (Type::Object(_), Some(item)) = (ty, method.stream()) {
				let next = member_name(&code).expect("a method has a name");
				let stream = self.stream(&next, item, method.deprecated.as_deref());
				body.python.extend(stream.iter().map(|code| indent_by(code, 1)));
			}
		}
		self.distinct(rust, data, body.codes());
	}

	/// An object's async iteration over its stream's method `next`, yielding `item`s.
	/// Deprecated as `next` is, by `deprecated`'s note.
	fn stream(&self, next: &str, item: &Type, deprecated: Option<&str>) -> [String; 2] {
		let item = self.py_ty(item);
		let aiter = "def __aiter__(self) -> _typing.Self:\n\t\"\"\"This stream, which `async for` iterates through `__anext__`.\"\"\"\n\treturn self";
		let anext = format!(
			"async def __anext__(self) -> {item}:\n\t\"\"\"The next item, awaiting `{next}` once: `StopAsyncIteration` at `None`, and its error raised.\n\n\t\
			It reads nothing ahead, so a loop left early leaves the next item to the next call, and\n\t\
			cancelling the awaiting task drops `{next}`'s future, as dropping it in Rust would: it loses\n\t\
			an item only where that future does when dropped, as one that is not cancel-safe does.\"\"\"\n\t\
			item = await self.{next}()\n\tif item is None:\n\t\traise _StopAsyncIteration\n\treturn item"
		);
		[aiter.to_string(), format!("{}{anext}", deprecator(deprecated))]
	}

	/// Check that the members `codes` of the type `rust` and its data's names `data` differ.
	fn distinct<'c>(&mut self, rust: &str, data: &[String], codes: impl Iterator<Item = &'c String>) {
		let mut seen = BTreeSet::new();
		for name in data.iter().cloned().chain(codes.filter_map(|code| member_name(code))) {
			if !seen.insert(name.clone()) {
				self.errors.push(format!(
					"two members of `{rust}` are both `{name}` in Python, which rust2-ffi should keep apart"
				));
			}
		}
	}

	fn record(&mut self, i: usize) {
		let abi = self.abi;
		let record = &abi.api.records[i];
		self.on = record.on;
		let c = abi.record(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&record.path, &record.args);
		let t = self.register(&c, name.clone());
		let ty = Type::Record(i);
		let mut body = Body::default();
		if record.opaque.is_some() {
			body.python.push("\t__slots__ = ()\n".to_string());
		}
		self.traits(&ty, &c, record.traits, &mut body);
		let owner = Ty::Struct(c.clone());
		let data: Vec<String> = record.fields.iter().map(|f| field_name(&f.name)).collect();
		self.methods(&record.methods, (&ty, &owner, &rust), &data, &[], &mut body);
		if record.opaque.is_some() {
			let text = format!(
				"{rust}\n\n{}\n\nIts bytes, which only Rust makes or reads: copy it, but never make one.",
				record.docs
			);
			let _ = write!(
				self.py,
				"{}class {name}(_n.Value):\n{}{}\n\n",
				deprecator(record.deprecated.as_deref()),
				docstring(&deprecation(text.trim(), record.deprecated.as_deref()), "\t"),
				body.members().join("\n")
			);
			let _ = write!(
				self.glue,
				"fn r_{c}(o: *mut PyObject, _s: &mut rt::Scope, _take: bool) -> R<crate::{c}> {{\n\trt::value_in(o, {t})\n}}\n\nfn w_{c}(v: crate::{c}) -> R<Obj> {{\n\trt::value_out({t}, v)\n}}\n\n"
			);
			return;
		}
		let fields: Vec<Slot> = (record.fields.iter())
			.map(|f| Slot {
				name: field_name(&f.name),
				ty: self.py_field(&f.ty),
				docs: deprecation(&f.docs, f.deprecated.as_deref()),
				same: own_eq(abi, &f.ty),
			})
			.collect();
		let docs = deprecation(&format!("{rust}\n\n{}", record.docs), record.deprecated.as_deref());
		let _ = write!(
			self.py,
			"{}{}\n\n",
			deprecator(record.deprecated.as_deref()),
			record_class(&name, "_Record", &docs, &fields, &body.members())
		);
		// The glue.
		let mut reads = Vec::new();
		let mut fields = Vec::new();
		let mut writes = Vec::new();
		for (n, field) in record.fields.iter().enumerate() {
			let key = self.key(&field_name(&field.name));
			let read = match n {
				0 => format!("rt::field(o, {t}, {key}, s)?"),
				_ => format!("rt::attr(o, {key}, s)?"),
			};
			reads.push(format!("let x{n} = {read};"));
			fields.push(format!("f{n}: {}?", self.lift(&field.ty, &format!("x{n}"), "take")));
			writes.push(format!("({key}, {})", self.lower(&field.ty, &format!("v.f{n}"))));
		}
		if record.fields.is_empty() {
			reads.push(format!("rt::check(o, {t})?;"));
		}
		let _ = write!(
			self.glue,
			"fn r_{c}(o: *mut PyObject, s: &mut rt::Scope, take: bool) -> R<crate::{c}> {{\n\t{}\n\tOk(crate::{c} {{ {} }})\n}}\n\nfn w_{c}(v: crate::{c}) -> R<Obj> {{\n\trt::record({t}, vec![{}])\n}}\n\n",
			reads.join("\n\t"),
			fields.join(", "),
			writes.join(", ")
		);
	}

	fn enumeration(&mut self, i: usize) {
		let abi = self.abi;
		let e = &abi.api.enums[i];
		self.on = e.on;
		let c = abi.enumeration(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&e.path, &e.args);
		let t = self.register(&c, name.clone());
		let ty = Type::Enum(i);
		let mut body = Body::default();
		self.traits(&ty, &c, e.traits, &mut body);
		let owner = abi.ty(&ty);
		if e.plain() {
			let missing: String = (e.variants.iter())
				.filter_map(|v| {
					let cfg = abi.cfg(v.on)?;
					let d = v.discriminant;
					Some(format!(
						"\t\t#[cfg(not({cfg}))]\n\t\t{d} => rt::missing(\"{c}.{}\"),\n",
						v.name
					))
				})
				.collect();
			if !missing.is_empty() {
				let _ = write!(
					self.glue,
					"fn on_{c}(v: i32) -> R<i32> {{\n\tmatch v {{\n{missing}\t\t_ => Ok(v),\n\t}}\n}}\n\n"
				);
			}
			let data: Vec<String> = e.variants.iter().map(|v| abi::snake(&v.name).to_uppercase()).collect();
			// An `Enum`'s members have these.
			self.methods(&e.methods, (&ty, &owner, &rust), &data, &["name", "value"], &mut body);
			let entries: String = e
				.variants
				.iter()
				.map(|v| format!("\t{} = {}\n", abi::snake(&v.name).to_uppercase(), v.discriminant))
				.collect();
			let _ = write!(
				self.py,
				"{}class {name}(_enum.Enum):\n{}{entries}{}\n\n",
				deprecator(e.deprecated.as_deref()),
				docstring(
					&deprecation(&format!("{rust}\n\n{}", e.docs), e.deprecated.as_deref()),
					"\t"
				),
				body.members().iter().map(|m| format!("\n{m}")).collect::<String>()
			);
			return;
		}
		// Each variant's class, and its fields, which on an instance hide the enum's members.
		let vnames = variant_classes(e.variants.iter().map(|v| v.name.as_str()));
		for (variant, vname) in e.variants.iter().zip(&vnames) {
			let key = format!("{} / variant {}", crate::lock::type_key(&e.shown), variant.name);
			self.locked.insert(key, format!("{name}.{vname}"));
		}
		let mut data = vnames.clone();
		let fields: BTreeSet<String> = e
			.variants
			.iter()
			.flat_map(|v| &v.fields)
			.map(|f| field_name(&f.name))
			.collect();
		data.extend(fields);
		self.methods(&e.methods, (&ty, &owner, &rust), &data, &[], &mut body);
		// Each variant's class is bound on the enum's once made, after the enum's own.
		for vname in &vnames {
			body.native
				.push(format!("\t{vname}: _typing.TypeAlias = \"_{name}_{vname}\"\n"));
		}
		let _ = write!(
			self.py,
			"{}class {name}:\n{}\t__slots__ = ()\n{}\n\n",
			deprecator(e.deprecated.as_deref()),
			docstring(
				&deprecation(&format!("{rust}\n\n{}", e.docs), e.deprecated.as_deref()),
				"\t"
			),
			body.members().iter().map(|m| format!("\n{m}")).collect::<String>()
		);
		let mut reads = Vec::new();
		let mut writes = Vec::new();
		for (n, (variant, vname)) in e.variants.iter().zip(&vnames).enumerate() {
			let class = format!("_{name}_{vname}");
			let key = abi::payload(&c, &variant.name);
			let tv = self.register(&key, class.clone());
			let fields: Vec<Slot> = (variant.fields.iter())
				.map(|f| Slot {
					name: field_name(&f.name),
					ty: self.py_field(&f.ty),
					docs: f.docs.clone(),
					same: own_eq(abi, &f.ty),
				})
				.collect();
			let _ = write!(
				self.py,
				"{}\n\n{class}.__name__ = \"{vname}\"\n{class}.__qualname__ = \"{name}.{vname}\"\nif not _typing.TYPE_CHECKING:\n\t{name}.{vname} = {class}\n\n\n",
				record_class(&class, &format!("{name}, _Record"), "", &fields, &[])
			);
			// A value of a variant the library's target lacks is refused as it crosses.
			let missing = match abi.cfg(variant.on) {
				Some(cfg) => format!(
					"if !cfg!({cfg}) {{\n\t\t\treturn rt::missing(\"{c}.{}\");\n\t\t}}\n\t\t",
					variant.name
				),
				None => String::new(),
			};
			match variant.fields.is_empty() {
				true => {
					reads.push(format!(
						"if rt::is(o, {tv}) {{\n\t\t{missing}return Ok(crate::{c} {{ tag: {n}, u: unsafe {{ std::mem::zeroed() }} }});\n\t}}"
					));
					writes.push(format!("\t\t{n} => rt::record({tv}, vec![]),"));
				}
				false => {
					let payload = abi::payload(&c, &variant.name);
					let mut binds = Vec::new();
					let mut fields = Vec::new();
					let mut lowers = Vec::new();
					for (k, f) in variant.fields.iter().enumerate() {
						let key = self.key(&field_name(&f.name));
						binds.push(format!("let x{k} = rt::attr(o, {key}, s)?;"));
						fields.push(format!("f{k}: {}?", self.lift(&f.ty, &format!("x{k}"), "take")));
						lowers.push(format!("({key}, {})", self.lower(&f.ty, &format!("p.f{k}"))));
					}
					reads.push(format!(
						"if rt::is(o, {tv}) {{\n\t\t{missing}{}\n\t\treturn Ok(crate::{c} {{ tag: {n}, u: crate::{c}_union {{ v{n}: crate::{payload} {{ {} }} }} }});\n\t}}",
						binds.join("\n\t\t"),
						fields.join(", ")
					));
					writes.push(format!(
						"\t\t{n} => {{\n\t\t\tlet p = unsafe {{ v.u.v{n} }};\n\t\t\trt::record({tv}, vec![{}])\n\t\t}}",
						lowers.join(", ")
					));
				}
			}
		}
		let _ = write!(
			self.glue,
			"fn r_{c}(o: *mut PyObject, s: &mut rt::Scope, take: bool) -> R<crate::{c}> {{\n\t{}\n\trt::check(o, {t})?;\n\tcrate::rt::fail(\"python\", \"a variant is of no class of its enum\")\n}}\n\nfn w_{c}(v: crate::{c}) -> R<Obj> {{\n\tmatch v.tag {{\n{}\n\t\t_ => crate::rt::fail(\"python\", \"an enum's tag is invalid\"),\n\t}}\n}}\n\n",
			reads.join("\n\t"),
			writes.join("\n")
		);
	}

	fn object(&mut self, i: usize) {
		let abi = self.abi;
		let object = &abi.api.objects[i];
		self.on = object.on;
		let c = abi.object(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&object.path, &object.args);
		self.register(&c, name.clone());
		let ty = Type::Object(i);
		let mut body = Body {
			python: vec!["\t__slots__ = ()\n".to_string()],
			..Body::default()
		};
		self.traits(&ty, &c, object.traits, &mut body);
		let owner = Ty::Handle {
			name: c.clone(),
			constant: false,
		};
		self.methods(&object.methods, (&ty, &owner, &rust), &[], &[], &mut body);
		let text = format!(
			"{rust}\n\n{}\n\nIts handle is freed once its last reference goes. A call after one that moved its value out raises.",
			object.docs
		);
		let _ = write!(
			self.py,
			"{}class {name}(_n.Object):\n{}{}\n\n",
			deprecator(object.deprecated.as_deref()),
			docstring(&deprecation(text.trim(), object.deprecated.as_deref()), "\t"),
			body.members().join("\n")
		);
	}

	fn callback(&mut self, i: usize) {
		let abi = self.abi;
		let callback = &abi.api.callbacks[i];
		self.on = callback.on;
		let c = abi.callback(i);
		let name = self.class(&c);
		let rust = callback.path.join("::");
		self.register(&c, name.clone());
		let mut body = Vec::new();
		let mut table = vec!["free: Some(rt::free_callback)".to_string()];
		let mut names: Vec<String> = Vec::new();
		for (n, method) in callback.methods.iter().enumerate() {
			// A name an earlier method took, its own takes a trailing `_` against.
			let reserved: Vec<&str> = names.iter().map(String::as_str).collect();
			let member = member(&abi::snake(method.name()), &reserved);
			names.push(member.clone());
			crate::lock::function(&mut self.locked, method, &member);
			let params: Vec<String> = std::iter::once("self".to_string())
				.chain(
					param_names(&method.params)
						.into_iter()
						.zip(&method.params)
						.map(|(name, p)| format!("{name}: {}", self.py_lent(&p.ty))),
				)
				.collect();
			// Type checkers flag a call of a deprecated method of an implementation too.
			let deprecated = match deprecator(method.deprecated.as_deref()) {
				d if d.is_empty() => d,
				d => format!("\t{d}"),
			};
			body.push(format!(
				"{deprecated}\t@_abc.abstractmethod\n\tdef {member}({}) -> {}:\n{}\t\t...\n",
				params.join(", "),
				self.py_param(&method.ok),
				docstring(
					&deprecation(
						&self.function_docs(method, doc::sections(&method.docs).body),
						method.deprecated.as_deref(),
					),
					"\t\t"
				)
			));
			let c_params: Vec<Ty> = method.params.iter().map(|p| abi.ty(&p.ty)).collect();
			let c_ret = abi.ty(&method.ok);
			let lent: Vec<Type> = method.params.iter().map(|p| p.ty.clone()).collect();
			let trampoline = format!("py_{c}_{n}");
			let key = self.key(&member);
			self.trampoline(
				&trampoline,
				&format!("{rust}::{}", method.name()),
				(&lent, &method.ok),
				(&c_params, &c_ret),
				Some(&key),
				None,
			);
			table.push(format!("{}: Some({trampoline})", method.name()));
		}
		self.distinct(&rust, &[], body.iter());
		let text = format!(
			"{rust}: implemented in Python. Rust calls its methods from any thread, and once it\ndrops the implementation, calls its `close()` if it has one.\n\n{}",
			callback.docs
		);
		let _ = write!(
			self.py,
			"{}class {name}(_abc.ABC):\n{}\t__slots__ = ()\n\n{}\n\n",
			deprecator(callback.deprecated.as_deref()),
			docstring(&deprecation(text.trim(), callback.deprecated.as_deref()), "\t"),
			body.join("\n")
		);
		let _ = write!(
			self.glue,
			"static VTABLE_{c}: crate::{c}_vtable = crate::{c}_vtable {{ {} }};\n\n",
			table.join(", ")
		);
	}

	/// The Python annotation of a value Rust lends a callback or closure: its bytes copied.
	fn py_lent(&self, ty: &Type) -> String {
		match ty.base() {
			Type::Slice | Type::Vec | Type::Bytes => "_bytes".to_string(),
			Type::Option(inner) if matches!(inner.base(), Type::Slice | Type::Vec | Type::Bytes) => {
				"_bytes | None".to_string()
			}
			_ => self.py_ty(ty),
		}
	}

	/// The Rust function C calls for a callback's method or a closure, named `trampoline`,
	/// which calls the Python value Rust holds: for a callback, its method `key`, and for a
	/// closure, the callable itself. A closure returning a `Result` writes its value
	/// through an out, and fails with the `error` of the exception class `exception`
	/// registers that it raises.
	fn trampoline(
		&mut self,
		trampoline: &str,
		what: &str,
		(lent, ret): (&[Type], &Type),
		(c_params, c_ret): (&[Ty], &Ty),
		key: Option<&str>,
		exception: Option<&str>,
	) {
		let mut rust_params = vec!["data: *mut std::ffi::c_void".to_string()];
		let mut args = Vec::new();
		for (k, (ty, c)) in lent.iter().zip(c_params).enumerate() {
			rust_params.push(format!("p{k}: {}", rust_c(self.abi, c)));
			let p = format!("p{k}");
			args.push(match (c, strip_refs(&strip_box(ty))) {
				(Ty::Handle { .. }, Type::Object(i)) => format!("rt::lent({p}, {}, s)", self.t(&self.abi.object(i))),
				(Ty::Buf, _) => self.lent_buf(ty, &p),
				_ => self.lower(ty, &p),
			});
		}
		let call = match key {
			Some(key) => format!("rt::call_method(data.cast(), {key}, vec![{}])", args.join(", ")),
			None => format!("rt::call_py(data.cast(), vec![{}])", args.join(", ")),
		};
		let outs = &c_params[lent.len()..];
		let (call, back) = match exception {
			Some(exception) => {
				let err = outs.len() - 1;
				for (k, c) in outs.iter().enumerate() {
					let Ty::Out(inner) = c else {
						unreachable!("a closure's {c:?} after what it takes is an out")
					};
					rust_params.push(format!("out{k}: *mut {}", rust_c(self.abi, inner)));
				}
				let call = format!(
					"match {call} {{\n\t\t\tOk(r) => r,\n\t\t\tErr(rt::Raised) => {{\n\t\t\t\tout{err}.write(rt::closure_error({exception})?);\n\t\t\t\treturn Ok(false);\n\t\t\t}}\n\t\t}}"
				);
				let back = match err {
					0 => "Ok(true)".to_string(),
					_ => format!("out0.write({}?);\n\t\tOk(true)", self.given(ret, "r.ptr()")),
				};
				(call, back)
			}
			None => {
				let back = match c_ret {
					Ty::Void => "Ok(())".to_string(),
					_ => self.given(ret, "r.ptr()"),
				};
				(format!("{call}?"), back)
			}
		};
		let rust_ret = rust_c(self.abi, c_ret);
		let _ = write!(
			self.glue,
			"unsafe extern \"C\" fn {trampoline}({}) -> {rust_ret} {{\n\trt::upcall(\"{what}\", || unsafe {{\n\t\tlet s = &mut rt::Scope::default();\n\t\tlet r = {call};\n\t\t{back}\n\t}})\n}}\n\n",
			rust_params.join(", "),
		);
	}

	/// The glue's expression, an `R<C>`, of the Python object `o` a closure returns as the
	/// C value of `ty` that Rust then owns: a string copied, as `<prefix>_buf_copy` makes
	/// one.
	fn given(&mut self, ty: &Type, o: &str) -> String {
		match ty {
			Type::String => format!("rt::str_given({o})"),
			_ => self.lift(ty, o, "false"),
		}
	}

	/// `text`, a function's docs without its sections, then its `# Errors` as what it
	/// raises, and its `# Panics` as when it aborts.
	fn function_docs(&mut self, function: &Function, text: String) -> String {
		let sections = doc::sections(&function.docs);
		let mut text = text.trim().to_string();
		let section = |title: &str, body: &str| {
			let body: String = body
				.lines()
				.map(|l| format!("\n    {l}").trim_end().to_string())
				.collect();
			format!("\n\n{title}:{body}")
		};
		match (&sections.errors, &function.err) {
			(Some(errors), Some(err)) => {
				let thrown = self.thrown(err);
				text.push_str(&section("Raises", &format!("{thrown}: {errors}")));
			}
			(Some(errors), None) => text.push_str(&section("Errors", errors)),
			(None, _) => {}
		}
		if let Some(panics) = &sections.panics {
			text.push_str(&section("Aborts", panics));
		}
		text.trim().to_string()
	}

	/// The Sphinx role and target a link names for `target`, if Python has it.
	fn link(&self, target: doc::Target<'_>) -> Option<(&'static str, String)> {
		let abi = self.abi;
		let owner = |owner: doc::Owner| match owner {
			doc::Owner::Record(i) => self.class(&abi.record(i)),
			doc::Owner::Enum(i) => self.class(&abi.enumeration(i)),
			doc::Owner::Object(i) => self.class(&abi.object(i)),
			doc::Owner::Callback(i) => self.class(&abi.callback(i)),
		};
		Some(match target {
			doc::Target::Record(i) => ("class", owner(doc::Owner::Record(i))),
			doc::Target::Enum(i) => ("class", owner(doc::Owner::Enum(i))),
			doc::Target::Object(i) => ("class", owner(doc::Owner::Object(i))),
			doc::Target::Callback(i) => ("class", owner(doc::Owner::Callback(i))),
			doc::Target::Variant(i, v) => {
				let e = &abi.api.enums[i];
				let name = &e.variants[v].name;
				match e.plain() {
					true => (
						"attr",
						format!("{}.{}", owner(doc::Owner::Enum(i)), abi::snake(name).to_uppercase()),
					),
					// As its class is named, apart from its siblings'.
					false => (
						"class",
						format!(
							"{}.{}",
							owner(doc::Owner::Enum(i)),
							variant_classes(e.variants.iter().map(|v| v.name.as_str())).swap_remove(v)
						),
					),
				}
			}
			doc::Target::Field(i, n) => (
				"attr",
				format!(
					"{}.{}",
					owner(doc::Owner::Record(i)),
					field_name(&abi.api.records[i].fields[n].name)
				),
			),
			doc::Target::Constant(c) => (
				"data",
				self.locked.get(&format!("const {}", c.path.join("::")))?.clone(),
			),
			doc::Target::Function(of, f) => {
				let member = self.locked.get(&crate::lock::function_key(f))?;
				let role = match f.access {
					Access::Field { .. } | Access::Const => "attr",
					_ => "meth",
				};
				match of {
					Some(of) => (role, format!("{}.{member}", owner(of))),
					None => ("func", member.clone()),
				}
			}
		})
	}

	/// The registry's index of the exception class an error of type `e` raises.
	fn exception(&mut self, e: &Type) -> String {
		let name = self.thrown(e);
		self.t(&format!("exc_{name}"))
	}

	/// The name of the exception class an error of type `e` raises, made once per type.
	fn thrown(&mut self, e: &Type) -> String {
		let py = self.py_ty(e);
		// Named for Rust's type, as Python's `str` reads as no type of its own.
		let name = match e {
			Type::Str | Type::String => "StringException".to_string(),
			_ => format!("{}Exception", pascal_type(&py)),
		};
		let thrown = match self.thrown.get(&name) {
			Some(thrown) => thrown.clone(),
			None => {
				let thrown = self.claim(&name, format!("the exception of `{py}`"));
				self.thrown.insert(name, thrown.clone());
				self.exceptions.insert(thrown.clone(), e.clone());
				self.register(&format!("exc_{thrown}"), thrown.clone());
				thrown
			}
		};
		crate::lock::exception(&mut self.locked, self.abi, e, &py, &thrown);
		thrown
	}

	/// The glue's expression, an `R<Obj>`, of a buffer `p` Rust lends for a call: text
	/// is copied as when returned, and bytes into `bytes`, since a `memoryview` could
	/// outlive the call.
	fn lent_buf(&mut self, ty: &Type, p: &str) -> String {
		match strip_refs(ty) {
			Type::Slice | Type::Vec | Type::Bytes => format!("rt::lent_bytes({p})"),
			Type::Option(inner) if matches!(*inner, Type::Slice | Type::Vec | Type::Bytes) => {
				format!("(if {p}.is_none() {{ rt::none() }} else {{ rt::lent_bytes({p}) }})")
			}
			_ => self.lower(ty, p),
		}
	}

	/// A native function `bare`, which the generated Python calls by position with `n`
	/// arguments as `a`, whose glue runs `body` with a scope `s`, returning an `R<Obj>`.
	fn native(&mut self, bare: &str, n: usize, body: &str) {
		let body = self.checked(bare, body);
		let _ = write!(
			self.glue,
			"unsafe extern \"C\" fn f_{bare}(_: *mut PyObject, a: *mut *mut PyObject, n: Py_ssize_t) -> *mut PyObject {{\n\trt::wrap(|| unsafe {{\n\t\tlet a = rt::args(a, n, {n}, \"{bare}\")?;\n\t\tlet s = &mut rt::Scope::default();\n\t\t{body}\n\t}})\n}}\n\n"
		);
		self.table.push(format!("rt::method(c\"{bare}\", f_{bare})"));
	}

	/// `body`, raising first on a target the item being written is not on.
	fn checked(&self, bare: &str, body: &str) -> String {
		match self.abi.cfg(self.on) {
			Some(cfg) => format!(
				"if !cfg!({cfg}) {{\n\t\t\treturn rt::missing(\"{}_{bare}\");\n\t\t}}\n\t\t{body}",
				self.abi.prefix
			),
			None => body.to_string(),
		}
	}

	/// A native function `bare` that Python calls as `d` describes, with no Python
	/// frame between. Its glue runs `body` like a [`Gen::native`]'s, with `a` holding
	/// `self` first if it has one.
	fn direct(&mut self, bare: &str, d: Direct<'_>, body: &str) {
		let body = &self.checked(bare, body);
		let member = d.member;
		let class = d.owner.map(|c| (self.t(c), self.class(c)));
		if d.property {
			let (t, _) = class.expect("a property has its class");
			let _ = write!(
				self.glue,
				"unsafe extern \"C\" fn f_{bare}(this: *mut PyObject, _: *mut std::ffi::c_void) -> *mut PyObject {{\n\trt::wrap(|| unsafe {{\n\t\tlet a = [this];\n\t\tlet s = &mut rt::Scope::default();\n\t\t{body}\n\t}})\n}}\n\n"
			);
			let doc = match d.docs.trim() {
				"" => "None".to_string(),
				docs => format!("Some({})", c_str(docs)),
			};
			self.members.push(format!(
				"rt::Member::Get({t}, rt::property(c\"{member}\", {doc}, f_{bare}))"
			));
			return;
		}
		let method = d.place == Place::Member;
		let qual = match &class {
			Some((_, name)) => format!("{name}.{member}"),
			None => member.to_string(),
		};
		let names: Vec<String> = d.params.iter().map(|p| format!("\"{p}\"")).collect();
		let _ = write!(
			self.glue,
			"unsafe extern \"C\" fn f_{bare}(this: *mut PyObject, a: *const *mut PyObject, n: Py_ssize_t, kw: *mut PyObject) -> *mut PyObject {{\n\trt::wrap(|| unsafe {{\n\t\tlet a = rt::params::<{}>({}, a, n, kw, &[{}], \"{qual}\")?;\n\t\tlet s = &mut rt::Scope::default();\n\t\t{body}\n\t}})\n}}\n\n",
			d.params.len() + method as usize,
			if method { "Some(this)" } else { "None" },
			names.join(", "),
		);
		// The signature `inspect.signature` reads, before the docstring.
		let params: Vec<&str> = method
			.then_some("$self")
			.into_iter()
			.chain(d.params.iter().map(String::as_str))
			.collect();
		let doc = c_str(&format!("{member}({})\n--\n\n{}", params.join(", "), d.docs.trim()));
		let def = format!("rt::function(c\"{member}\", {doc}, f_{bare})");
		match (class, d.place) {
			(Some((t, _)), Place::Member) => self.members.push(format!("rt::Member::Method({t}, {def})")),
			(Some((t, _)), Place::Static) => self.members.push(format!("rt::Member::Static({t}, {def})")),
			_ => self.table.push(def),
		}
	}

	/// `function`'s Python form, a member of `owner` if it has one (whose IR type and
	/// reserved member names are `owner_ty`), with its native function and glue.
	fn function(
		&mut self,
		function: &Function,
		owner: Option<&Ty>,
		owner_ty: Option<(&Type, &[&str])>,
		place: Place,
	) -> Option<Form> {
		let on = std::mem::replace(&mut self.on, function.on);
		let form = self.function_on(function, owner, owner_ty, place);
		self.on = on;
		form
	}

	fn function_on(
		&mut self,
		function: &Function,
		owner: Option<&Ty>,
		owner_ty: Option<(&Type, &[&str])>,
		place: Place,
	) -> Option<Form> {
		let abi = self.abi;
		// A value is immutable in Python, so its `&mut self` method returns it as the call
		// left it, calling the C function in place on a copy it hands back.
		let handed = (owner.zip(owner_ty)).and_then(|(owner, (ty, _))| abi.handed_back(function, owner, ty));
		let (back, export, handed) = match handed {
			Some((back, export, handed)) => (Some(back), export, Some(handed)),
			None => (None, abi.export(function, owner), None),
		};
		let function = back.as_ref().unwrap_or(function);
		let c_name = export.call.name.clone();
		let bare = self.bare(&c_name).to_string();
		let owner_c = match owner {
			Some(Ty::Struct(n) | Ty::Enum(n) | Ty::Handle { name: n, .. }) => Some(n.clone()),
			_ => None,
		};
		let own = match &owner_c {
			Some(o) => export.named.strip_prefix(o.as_str()).and_then(|r| r.strip_prefix('_')),
			None => None,
		};
		let reserved = owner_ty.map_or(&[][..], |(_, r)| r);
		// Named as Rust's path is, whatever C's clashes rename.
		let named = self.bare(&export.named).to_string();
		let snake = own.unwrap_or(&named);
		let member = member(&abi::snake(snake), reserved);
		let constant = function.access == Access::Const && function.params.is_empty() && place != Place::Member;
		let member = match (place, constant) {
			(Place::Top, true) => self.claim(&member.to_uppercase(), format!("const `{}`", function.path.join("::"))),
			(Place::Top, false) => self.claim(&member, format!("fn `{}`", function.path.join("::"))),
			// Upper case can join two Rust names, as icu_properties' `SentenceBreak::Sp`
			// and its alias `SP`, so the later is kept apart again.
			(Place::Static, true) => self::member(&member.to_uppercase(), reserved),
			_ => member,
		};

		crate::lock::function(&mut self.locked, function, &member);

		// The glue: each argument read, the call, and its value lowered.
		let mut stmts = Vec::new();
		let mut c_args = Vec::new();
		let mut py_params = Vec::new();
		let mut py_args = Vec::new();
		// Its native function's parameters, declared for type checkers by position, so no
		// Rust name clashes with the receiver's.
		let mut native_params = Vec::new();
		let mut n = 0;
		let mut c_params = export.call.params.iter();
		let receiver = function.receiver.filter(|_| owner.is_some());
		if let Some(receiver) = receiver {
			let (_, c) = c_params.next().expect("a method has its receiver");
			let ty = owner_ty.expect("a method has its type").0.clone();
			py_params.push("self".to_string());
			py_args.push("self".to_string());
			native_params.push(format!("p0: {}", self.py_ty(&ty)));
			match c {
				Ty::Handle { .. } => {
					let ty = match receiver {
						Receiver::Owned => ty,
						Receiver::Ref => Type::Ref(crate::ir::Mutability::Not, Box::new(ty)),
						Receiver::Mut => Type::Ref(crate::ir::Mutability::Mut, Box::new(ty)),
					};
					stmts.push(format!("let p{n} = {}?;", self.lift(&ty, &format!("a[{n}]"), "true")));
					c_args.push(format!("p{n}"));
				}
				_ => {
					// Read through a pointer, so Rust takes nothing of it.
					stmts.push(format!("let p{n} = {}?;", self.lift(&ty, &format!("a[{n}]"), "false")));
					c_args.push(format!("&p{n}"));
				}
			}
			n += 1;
		}
		let mut closures = Vec::new();
		// Each parameter Rust writes back, by its argument's index, and what it holds.
		let mut written: Vec<(usize, Type)> = Vec::new();
		for (param, name) in function.params.iter().zip(param_names(&function.params)) {
			let (_, c) = c_params.next().expect("each parameter has its C one");
			let annotation = match &param.ty {
				Type::Ref(_, inner) if param.ty.written() => format!("Mut[{}]", self.py_ty(inner)),
				ty => self.py_param(ty),
			};
			py_params.push(format!("{name}: {annotation}"));
			native_params.push(format!("p{}: {annotation}", native_params.len()));
			py_args.push(name.clone());
			match (&param.ty, c) {
				(Type::Ref(_, inner), _) if param.ty.written() => {
					stmts.push(format!(
						"let h{n} = rt::mut_value(a[{n}], s)?;\n\t\tlet p{n} = {}?;",
						self.lift(inner, &format!("h{n}"), "true")
					));
					c_args.push(format!("p{n}"));
					written.push((n, (**inner).clone()));
				}
				(Type::Closure(closure_), Ty::Fn(c_lent, c_ret)) => {
					c_params.next().expect("a closure has its data");
					let trampoline = format!("py_{c_name}_{}", abi::escape(&param.name));
					closures.push((
						(**closure_).clone(),
						c_lent.clone(),
						(**c_ret).clone(),
						trampoline.clone(),
					));
					c_args.push(format!("Some({trampoline})"));
					c_args.push(format!("p{n}"));
					match closure_.kept {
						true => {
							c_params.next().expect("a kept closure has its free");
							stmts.push(format!("let p{n} = rt::kept(a[{n}], s)?;"));
							c_args.push("Some(rt::free_closure)".to_string());
						}
						false => stmts.push(format!("let p{n} = rt::closure(a[{n}])?;")),
					}
				}
				_ => {
					stmts.push(format!(
						"let p{n} = {}?;",
						self.lift(&param.ty, &format!("a[{n}]"), "true")
					));
					c_args.push(format!("p{n}"));
				}
			}
			n += 1;
		}
		for (closure_, c_lent, c_ret, trampoline) in closures {
			let exception = closure_.err.as_ref().map(|e| self.exception(e));
			self.trampoline(
				&trampoline,
				&c_name,
				(&closure_.params, &closure_.ok),
				(&c_lent, &c_ret),
				None,
				exception.as_deref(),
			);
		}
		let outs: Vec<(String, Ty)> = c_params.filter(|(_, ty)| !abi::is_status(ty)).cloned().collect();
		// A call that fails a run-time check writes its status, which raises once it returns.
		let (status, check) = match export.call.status() {
			Some(_) => (
				"let mut status = 0;\n\t\t".to_string(),
				format!("\n\t\ts.status(status, \"{c_name}\")?;"),
			),
			None => (String::new(), String::new()),
		};
		let status_arg = export.call.status().map(|_| "&mut status".to_string());

		// The outs follow the unstripped return: a borrowed tuple is one struct, not its elements.
		let ok = strip_refs(&function.ok);
		let err = function.err.as_ref().map(strip_refs);
		let exception = err.as_ref().map(|e| self.exception(e));
		if let Some(abi::Future { poll, free, .. }) = &export.future {
			let poll_outs: Vec<(String, Ty)> = poll.params[2..]
				.iter()
				.filter(|(_, ty)| !abi::is_status(ty))
				.cloned()
				.collect();
			// A poll of a future that is not `Send` on another thread raises.
			let (ps, ps_check) = match poll.status() {
				Some(_) => (", &mut ps", format!("\n\t\trt::check_status(ps, \"{}\")?;", poll.name)),
				None => ("", String::new()),
			};
			let (decl, ptrs, value) = self.outs(
				&poll_outs,
				&[],
				&function.ok,
				err.as_ref(),
				exception.as_deref(),
				"r == 1",
			);
			let _ = write!(
				self.glue,
				"unsafe fn poll_{c_name}(f: *mut std::ffi::c_void, w: crate::rt::Waker) -> R<Option<Obj>> {{\n\tunsafe {{\n\t\tlet mut ps = 0;\n\t\tlet _ = &mut ps;\n\t\t{decl}\n\t\tlet r = rt::call(|| crate::{}(f as _, w{}{ps}));{ps_check}\n\t\tif r == 0 {{\n\t\t\treturn Ok(None);\n\t\t}}\n\t\t{value}.map(Some)\n\t}}\n}}\n\nunsafe fn free_{c_name}(f: *mut std::ffi::c_void) {{\n\tunsafe {{ crate::{}(f as _) }}\n}}\n\n",
				poll.name,
				ptrs.iter().map(|p| format!(", {p}")).collect::<String>(),
				free.name,
			);
			stmts.push(format!(
				"{status}let f = rt::call(|| crate::{c_name}({}));{check}\n\t\ts.done();\n\t\trt::start(f.cast(), poll_{c_name}, free_{c_name}, s, {})",
				c_args.iter().chain(&status_arg).cloned().collect::<Vec<_>>().join(", "),
				function.eager
			));
		} else {
			let (decl, ptrs, value) = match outs.is_empty() {
				true => {
					let value = self.lower(&ok, "ret");
					(String::new(), Vec::new(), value)
				}
				false => self.outs(&outs, &written, &function.ok, err.as_ref(), exception.as_deref(), "ret"),
			};
			let args: Vec<String> = c_args.iter().cloned().chain(ptrs).chain(status_arg).collect();
			let call = match &handed {
				Some(h) => h.call(&args),
				None => format!("crate::{c_name}({})", args.join(", ")),
			};
			// A leaf keeps the GIL, which no part of it can wait on: see `Abi::leaf`.
			let call = match abi.leaf(function, owner_ty.map(|(ty, _)| ty)) {
				true => call,
				false => format!("rt::call(|| {call})"),
			};
			stmts.push(format!(
				"{decl}\n\t\t{status}let ret = {call};{check}\n\t\ts.done();\n\t\t{value}"
			));
		}
		// The Python.
		let mut text = doc::sections(&function.docs).body;
		if handed.is_some() {
			text.push_str(
				"\n\nRust changes a copy of this value, which it returns first, as the call left it: this one never changes.",
			);
		}
		for (param, name) in function.params.iter().zip(param_names(&function.params)) {
			if let Type::Closure(closure_) = &param.ty {
				let lent = &closure_.params;
				match closure_.kept {
					true => text.push_str(&format!(
						"\n\nRust keeps `{name}` past the call, and calls it from any thread, one call at a time, until it lets go of it."
					)),
					false => text.push_str(&format!(
						"\n\nRust calls `{name}` only during the call, from any thread."
					)),
				}
				if lent.iter().any(|p| matches!(p.base(), Type::Object(_))) {
					text.push_str(&format!(
						"\nAn object it lends lives only for that call of `{name}`: keep none past it."
					));
				}
				if let Some(err) = &closure_.err {
					let name = self.thrown(err);
					text.push_str(&format!(
						"\nTo fail, it raises `{name}`, whose error Rust takes; any other exception aborts."
					));
				}
			}
			if param.ty == Type::Ref(crate::ir::Mutability::Mut, Box::new(Type::Reader)) {
				text.push_str(&format!(
					"\n\nRust reads and seeks `{name}` only during the call, from any thread, one call at a time: an `OSError` either raises fails Rust's read or seek with its message."
				));
			}
		}
		let text = deprecation(&self.function_docs(function, text), function.deprecated.as_deref());
		// An `async def` stays Python's, as callers may need a coroutine.
		let direct = !constant && (export.future.is_none() || function.eager);
		// A call of a deprecated function warns, as Python's own deprecations do.
		if let Some(note) = &function.deprecated {
			let what = match &owner_c {
				Some(c) => format!("{}.{member}() is deprecated", self.class(c)),
				None => format!("{member}() is deprecated"),
			};
			let warning = match note.is_empty() {
				true => what,
				false => format!("{what}: {note}"),
			};
			// The caller's frame is the first, or the second past the Python that calls the
			// glue: an `async def`'s, or a constant's getter.
			let level = match direct {
				true => 1,
				false => 2,
			};
			stmts.insert(0, format!("rt::deprecated({}, {level})?;", c_str(&warning)));
		}
		let body = stmts.join("\n\t\t");
		let property = matches!(function.access, Access::Field { set: false, .. }) && place == Place::Member;
		match direct {
			true => {
				let names = param_names(&function.params);
				let d = Direct {
					member: &member,
					place,
					property,
					owner: owner_c.as_deref(),
					params: &names,
					docs: &text,
				};
				self.direct(&bare, d, &body);
			}
			false => self.native(&bare, n, &body),
		}
		let native = format!("_n.{bare}({})", py_args.join(", "));
		let ret = self.py_ty(&ok);
		let docs = docstring(text.trim(), "\t");
		let params = py_params.join(", ");
		// The glue's own is only declared, for type checkers.
		let code = match (export.future.is_some(), function.eager) {
			(true, false) => {
				if !native_params.is_empty() {
					native_params.push("/".to_string());
				}
				self.natives.push(format!(
					"@_staticmethod\ndef {bare}({}) -> {ret} | _Future[{ret}]: ...\n",
					native_params.join(", ")
				));
				format!(
					"async def {member}({params}) -> {ret}:\n{docs}\tr = {native}\n\treturn (await r) if _isinstance(r, _Future) else r"
				)
			}
			(true, true) => format!("def {member}({params}) -> _typing.Awaitable[{ret}]:\n{docs}\t..."),
			(false, _) => format!("def {member}({params}) -> {ret}:\n{docs}\t..."),
		};
		let code = format!("{}{code}", deprecator(function.deprecated.as_deref()));
		let code = match (function.access, place) {
			(Access::Const, Place::Top) if constant => {
				let name = member.clone();
				self.consts.push(Const {
					name,
					bare,
					ty: ret,
					docs: text,
				});
				return None;
			}
			(Access::Const, Place::Static) if constant => {
				let name = member.clone();
				self.natives
					.push(format!("@_staticmethod\ndef {bare}() -> {ret}: ...\n"));
				// An `Enum` makes a member of any other value, to a type checker as at run time.
				let value = match owner_ty {
					Some((Type::Enum(i), _)) if abi.api.enums[*i].plain() => format!("_enum.nonmember(_n.{bare}())"),
					_ => format!("_n.{bare}()"),
				};
				return Some(Form::Native(
					format!("{name} = {value}\n{}", docstring(text.trim(), "")),
					Some(format!("{name} = _Const(_n.{bare})\n")),
				));
			}
			_ if property => format!("@_property\n{code}"),
			(_, Place::Static) => format!("@_staticmethod\n{code}"),
			_ => code,
		};
		Some(match (direct, place) {
			(true, Place::Top) => Form::Native(code, Some(format!("{member} = _n.{member}\n"))),
			(true, _) => Form::Native(code, None),
			(false, _) => Form::Python(code),
		})
	}

	/// The out values of `outs`, for a function returning `ok` or raising `err` as
	/// `exception`: their declarations, the pointers the C call takes after its arguments,
	/// and the glue's expression of the value, an `R<Obj>`, where `ok` says whether it
	/// returned ok. What Rust wrote back into each of `written`, by its argument's index,
	/// is set in its `Mut` first, whether or not the call failed.
	fn outs(
		&mut self,
		outs: &[(String, Ty)],
		written: &[(usize, Type)],
		ok: &Type,
		err: Option<&Type>,
		exception: Option<&str>,
		is_ok: &str,
	) -> (String, Vec<String>, String) {
		let mut decl = Vec::new();
		let mut ptrs = Vec::new();
		let types: Vec<&Type> = match ok {
			Type::Unit => Vec::new(),
			Type::Tuple(elements) => elements.iter().collect(),
			ok => vec![ok],
		};
		let mut values = Vec::new();
		let mut error = String::new();
		let mut holds = Vec::new();
		let back = written.iter().map(|(_, ty)| ty);
		for (k, ((name, _), ty)) in outs.iter().zip(back.chain(types).chain(err)).enumerate() {
			decl.push(format!("let mut o_{name} = std::mem::MaybeUninit::zeroed();"));
			ptrs.push(format!("o_{name}.as_mut_ptr()"));
			let value = self.lower(ty, &format!("o_{name}.assume_init()"));
			match written.get(k) {
				Some((n, _)) => holds.push(format!("rt::set_mut(a[{n}], {value})?; ")),
				None if name == "err" => error = value,
				None => values.push(value),
			}
		}
		let value = match ok {
			Type::Unit => "rt::none()".to_string(),
			Type::Tuple(_) => format!("rt::tuple(vec![{}])", values.join(", ")),
			// A value C returns, beside what Rust wrote back.
			ok if err.is_none() && !written.is_empty() => self.lower(&strip_refs(ok), "ret"),
			_ => values.pop().expect("an out value"),
		};
		let value = match exception {
			Some(exception) => format!("(if {is_ok} {{ {value} }} else {{ rt::raise_error({exception}, {error}) }})"),
			None => value,
		};
		let value = match holds.is_empty() {
			true => value,
			false => format!("{{ {}{value} }}", holds.concat()),
		};
		(decl.join("\n\t\t"), ptrs, value)
	}
}

/// The glue's name for a socket address family.
/// The runtime's name for the converters of a socket or IP address of `family`, before
/// `_in` or `_out`.
fn addr_fn(ty: &Type, family: Family) -> &'static str {
	match (ty, family) {
		(Type::SocketAddr(_), Family::Any) => "addr",
		(Type::SocketAddr(_), Family::V4) => "addr_v4",
		(Type::SocketAddr(_), Family::V6) => "addr_v6",
		(_, Family::Any) => "ip",
		(_, Family::V4) => "ipv4",
		(_, Family::V6) => "ipv6",
	}
}

fn signed(int: Int) -> bool {
	matches!(int, Int::I8 | Int::I16 | Int::I32 | Int::I64 | Int::Isize)
}

/// `ty` without its `Box`es.
fn strip_box(ty: &Type) -> Type {
	match ty {
		Type::Box(inner) => strip_box(inner),
		ty => ty.clone(),
	}
}

/// A Python annotation as the start of a class name: `list[str] | None` is `ListStrNone`.
fn pascal_type(py: &str) -> String {
	py.split(|c: char| !c.is_ascii_alphanumeric())
		.map(|word| {
			let mut chars = word.chars();
			match chars.next() {
				Some(first) => first.to_uppercase().chain(chars).collect(),
				None => String::new(),
			}
		})
		.collect()
}

/// Python's hard keywords, and the builtin constants.
const KEYWORDS: &[&str] = &[
	"False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del",
	"elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal",
	"not", "or", "pass", "raise", "return", "try", "while", "with", "yield",
];

/// A name with `_` appended if Python reads it as a keyword.
fn kw(name: &str) -> String {
	match KEYWORDS.contains(&name) {
		true => format!("{name}_"),
		false => name.to_string(),
	}
}

/// A member's Python name: its snake-case name, with `_` appended if a keyword, and until
/// it is none of `reserved`, such as one Python gives every such class, nor of [`TAKEN`]:
/// a member named as a runtime alias, such as `_int`, would be what a later annotation
/// `-> _int` names in the class body.
fn member(snake: &str, reserved: &[&str]) -> String {
	// A positional field's accessor, `0_get`, is `_0_get`, as its field is `_0`.
	let digit = snake
		.starts_with(|c: char| c.is_ascii_digit())
		.then(|| format!("_{snake}"));
	// A keyword's `_` first, so a reservation is checked against the name the class emits.
	let snake = kw(digit.as_deref().unwrap_or(snake));
	let taken = |name: &str| reserved.contains(&name) || TAKEN.contains(&name);
	if !taken(&snake) {
		return snake;
	}
	let mut name = format!("{snake}_");
	while taken(&name) {
		name.push('_');
	}
	name
}

/// The class names of variants `names`, as Python spells them: `FooBar` for Rust's
/// `FooBar` and `Foo_Bar` alike, so a later one an earlier one took takes a trailing `_`.
fn variant_classes<'n>(names: impl Iterator<Item = &'n str>) -> Vec<String> {
	let mut classes: Vec<String> = Vec::new();
	for name in names {
		let mut class = kw(&pascal(name));
		while classes.contains(&class) {
			class.push('_');
		}
		classes.push(class);
	}
	classes
}

/// A field's Python name: a positional one's is `_0`, `_1`, ...
fn field_name(name: &str) -> String {
	match name.starts_with(|c: char| c.is_ascii_digit()) {
		true => format!("_{name}"),
		false => kw(&abi::snake(name)),
	}
}

/// A parameter's Python name, apart from the runtime's names a body reads: not `_any`, which
/// only `_load` reads, and it has no parameter.
fn param_name(name: &str) -> String {
	let name = kw(&abi::snake(name));
	match (TAKEN.contains(&name.as_str()) && name != "_any") || name == "self" {
		true => format!("{name}_"),
		false => name,
	}
}

/// The Python names of `params`, each apart from the others: one that an earlier parameter's
/// rename took, such as `_all_` after `_all`, gets another `_`.
fn param_names<'a>(params: impl IntoIterator<Item = &'a Param>) -> Vec<String> {
	let mut out: Vec<String> = Vec::new();
	for param in params {
		let mut name = param_name(&param.name);
		while out.contains(&name) {
			name.push('_');
		}
		out.push(name);
	}
	out
}

/// The name a member's code declares, for [`Gen::distinct`].
fn member_name(code: &str) -> Option<String> {
	let line = code.lines().find(|l| {
		let l = l.trim_start();
		!l.starts_with('@') && !l.is_empty()
	})?;
	let line = line.trim_start();
	let line = line.strip_prefix("async ").unwrap_or(line);
	if let Some(rest) = line.strip_prefix("def ") {
		return Some(rest.chars().take_while(|c| *c != '(').collect());
	}
	let name: String = line.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
	(!name.is_empty()).then_some(name)
}

/// A C string literal of `text`.
fn c_str(text: &str) -> String {
	format!("c{text:?}")
}

/// `@_deprecated` with `deprecated`'s note, which type checkers read, where it has one.
fn deprecator(deprecated: Option<&str>) -> String {
	match deprecated {
		Some(note) => format!("@_deprecated({})\n", py_str(note)),
		None => String::new(),
	}
}

/// `docs`, then the note of `deprecated`, where it has one.
fn deprecation(docs: &str, deprecated: Option<&str>) -> String {
	match deprecated {
		Some("") => format!("{docs}\n\nDeprecated."),
		Some(note) => format!("{docs}\n\nDeprecated: {note}"),
		None => docs.to_string(),
	}
	.trim()
	.to_string()
}

/// A Python string literal of `text`.
fn py_str(text: &str) -> String {
	let mut out = String::from("\"");
	for c in text.chars() {
		match c {
			'"' | '\\' => {
				out.push('\\');
				out.push(c);
			}
			c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
			c => out.push(c),
		}
	}
	out.push('"');
	out
}

/// A docstring holding `docs`, each line indented by `indent`, or nothing for none.
fn docstring(docs: &str, indent: &str) -> String {
	let docs = docs.trim();
	if docs.is_empty() {
		return String::new();
	}
	let escaped = docs.replace('\\', "\\\\").replace("\"\"\"", "\\\"\\\"\\\"");
	let mut out = format!("{indent}\"\"\"");
	for (n, line) in escaped.lines().enumerate() {
		if n > 0 {
			out.push('\n');
			if !line.is_empty() {
				out.push_str(indent);
			}
		}
		out.push_str(line);
	}
	if escaped.ends_with('"') {
		out.push(' ');
	}
	out.push_str("\"\"\"\n");
	out
}

/// The Rust type of a block's number.
fn number(ty: &Type) -> &'static str {
	match ty {
		Type::Int(int) => int.rust(),
		Type::F32 => "f32",
		Type::F64 => "f64",
		other => unreachable!("{other:?} is no block's number"),
	}
}

/// Whether a record or variant compares its field of `ty` through `_same`: `ty` holds a
/// float, or a block, which may be a buffer passed in whose own `==` compares item by item.
fn own_eq(abi: &Abi, ty: &Type) -> bool {
	abi.api.holds_float(ty) || ty.holds_block()
}

/// A field of a record's class.
struct Slot {
	/// Its Python name.
	name: String,
	/// Its Python annotation.
	ty: String,
	docs: String,
	/// Whether it compares through `_same`, as [`own_eq`] says.
	same: bool,
}

/// The class `name` of a record, subclassing `bases`, as `@dataclass(slots=True,
/// weakref_slot=True)` would make it but written out, so no `dataclass` runs at import: its
/// `fields` slotted beside a weak reference and in `__match_args__`, an `__init__` taking
/// each, and an `__eq__` comparing each as Rust's `==` does; then its `members`.
/// `_Record`, among `bases`, adds a dataclass's `repr`, `__dataclass_fields__`, and
/// `__replace__`.
fn record_class(name: &str, bases: &str, docs: &str, fields: &[Slot], members: &[String]) -> String {
	let quoted: Vec<String> = fields.iter().map(|f| format!("\"{}\"", f.name)).collect();
	let tuple = |items: &[String]| match items {
		[one] => format!("({one},)"),
		items => format!("({})", items.join(", ")),
	};
	let slots: Vec<String> = quoted.iter().cloned().chain(["\"__weakref__\"".to_string()]).collect();
	let mut out = format!(
		"class {name}({bases}):\n{}\t__slots__ = {}\n\t__match_args__ = {}\n",
		docstring(docs, "\t"),
		tuple(&slots),
		tuple(&quoted),
	);
	for f in fields {
		let _ = write!(out, "\t{}: {}\n{}", f.name, f.ty, docstring(&f.docs, "\t"));
	}
	if !fields.is_empty() {
		let params: Vec<String> = fields.iter().map(|f| format!(", {}: {}", f.name, f.ty)).collect();
		// pyright takes a name with no lower case letter, such as a tuple's `_0`, for a
		// constant, which `__init__` would redefine.
		let sets: Vec<String> = (fields.iter())
			.map(|f| match f.name.chars().any(char::is_lowercase) {
				true => format!("\t\tself.{0} = {0}\n", f.name),
				false => format!(
					"\t\tself.{0} = {0}  # pyright: ignore[reportConstantRedefinition]\n",
					f.name
				),
			})
			.collect();
		let _ = write!(
			out,
			"\n\tdef __init__(self{}) -> None:\n{}",
			params.concat(),
			sets.concat()
		);
	}
	// As a dataclass's, `__eq__` takes only its own class, not a subclass; the `isinstance` is for
	// the type checkers. A dataclass's own `__eq__` compares tuples of its fields before Python 3.13, which take
	// a value as equal to itself, so a NaN would equal itself and a numpy array would raise.
	let eq: Vec<String> = fields
		.iter()
		.map(|f| match f.same {
			true => format!("_same(self.{0}, other.{0})", f.name),
			false => format!("self.{0} == other.{0}", f.name),
		})
		.collect();
	let eq = match eq.is_empty() {
		true => "True".to_string(),
		false => eq.join(" and "),
	};
	let _ = write!(
		out,
		"\n\tdef __eq__(self, other: _object) -> _bool:\n\t\tif not _isinstance(other, {name}) or other.__class__ is not self.__class__:\n\t\t\treturn _NotImplemented\n\t\treturn {eq}\n"
	);
	for m in members {
		let _ = write!(out, "\n{m}");
	}
	out
}

#[cfg(test)]
mod tests {
	use super::{DEPRECATOR, TAKEN, member, param_name, param_names, variant_classes};
	use crate::ir::{Param, Type};

	/// Every name the deprecation helper binds is taken, so no export shadows it.
	#[test]
	fn deprecator_names_taken() {
		for line in DEPRECATOR.lines() {
			let line = line.trim_start();
			let name = (line.strip_prefix("def ").map(|rest| rest.split('(').next().unwrap()))
				.or_else(|| line.split_once(" as ").map(|(_, name)| name))
				.or_else(|| line.split_once(" = ").map(|(name, _)| name));
			if let Some(name) = name {
				assert!(TAKEN.contains(&name), "the helper's `{name}` is not in TAKEN");
			}
		}
	}

	#[test]
	fn params_keep_apart() {
		let params = |names: &[&str]| {
			let params: Vec<Param> = names
				.iter()
				.map(|n| Param {
					name: n.to_string(),
					ty: Type::Bool,
					interned: false,
				})
				.collect();
			param_names(&params)
		};
		// `_any` is read only by `_load`, which has no parameter, so it keeps its name.
		assert_eq!(param_name("_any"), "_any");
		assert_eq!(param_name("_all"), "_all_");
		assert_eq!(params(&["_all", "_all_"]), ["_all_", "_all__"]);
		assert_eq!(params(&["_all_", "_all"]), ["_all_", "_all__"]);
		assert_eq!(params(&["a", "b"]), ["a", "b"]);
	}

	#[test]
	fn variant_classes_keep_apart() {
		let names = ["FooBar", "Foo_Bar", "None", "Late"];
		assert_eq!(
			variant_classes(names.into_iter()),
			["FooBar", "FooBar_", "None_", "Late"]
		);
	}

	#[test]
	fn member_keeps_clear_of_the_runtime_aliases() {
		assert_eq!(member("_int", &[]), "_int_");
		assert_eq!(member("_int", &["_int_"]), "_int__");
		assert_eq!(member("joined", &[]), "joined");
		assert_eq!(member("bytes", &[]), "bytes");
		assert_eq!(member("name", &["name"]), "name_");
		// A callback's `_int` then `_int_` keep apart: the second reserves the first's name.
		assert_eq!(member("_int_", &[member("_int", &[]).as_str()]), "_int__");
		// A callback's `async_` then `async` (raw `r#async`) keep apart: the keyword's `_` is added before the check.
		assert_eq!(member("async", &[member("async_", &[]).as_str()]), "async__");
		assert_eq!(member("async_", &[member("async", &[]).as_str()]), "async__");
	}
}
