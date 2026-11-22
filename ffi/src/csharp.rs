//! C# over the C ABI: `csharp/<prefix>.cs`, one file that .NET, NativeAOT included, and
//! Unity's IL2CPP each compile, at C# 9 against .NET Standard 2.1, as Unity does.
//!
//! Each C function is a blittable `DllImport`, so no marshalling stub runs: a `bool` is a
//! byte, a handle an `IntPtr`, and each C struct a sequential struct of its C layout. Each
//! name is its C name without the prefix, in Pascal case: `moq_track_Producer` is
//! `TrackProducer`, and its method `moq_track_Producer_append_group` is
//! `TrackProducer.AppendGroup`. A free function or constant is a static member of the
//! static class `Lib`. An object is a sealed class owning its handle, `IDisposable`, freed
//! once disposed and no call holds it, or by the garbage collector; a record a struct; a
//! plain enum a C# enum, whose functions are extension methods of `<Enum>Extensions`; an
//! enum with data an abstract class of a sealed class per variant; a tuple a `ValueTuple`;
//! a trait an interface; a closure a `Func` or `Action`; a reader a `Stream`; a future a
//! `ValueTask`; an error a thrown `<E>Exception`. The runtime they share,
//! [`runtime/rt.cs`](../runtime/rt.cs), holds the handles, text, bytes, and std values,
//! and the source each future completes from.
//!
//! A callback Rust calls into C# is an `UnmanagedCallersOnly` method's address on .NET,
//! and under Unity (`UNITY_5_3_OR_NEWER`), whose IL2CPP has none, a delegate's, kept alive,
//! whose method is `MonoPInvokeCallback`. On iOS, where Unity links the library
//! statically, each function binds to `__Internal`.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;

use crate::abi::{self, Abi, Sig, Ty, pascal};
use crate::ir::{Access, Closure, Family, Function, Int, Literal, Receiver, Type};

const RUNTIME: &str = include_str!("../runtime/rt.cs");

/// What the generator writes: the C# file, and its public names for the API lock.
pub(crate) struct Output {
	pub(crate) names: BTreeMap<String, String>,
	pub(crate) csharp: String,
}

/// The runtime's names and the standard library's the generated code names unqualified,
/// which an exported type would shadow inside the namespace: of the framework's, each that
/// the api fixture's and moq's assemblies reference, by their metadata, and the attributes'
/// short names.
const TAKEN: &[&str] = &[
	"Action",
	"Arena",
	"ArgumentException",
	"ArgumentNullException",
	"ArgumentOutOfRangeException",
	"Array",
	"Bound",
	"Buffer",
	"Bytes",
	"CallConvCdecl",
	"Callbacks",
	"CallingConvention",
	"CancellationToken",
	"CancellationTokenRegistration",
	"CancellationTokenSource",
	"Console",
	"DllImport",
	"DllImportAttribute",
	"Duration",
	"Environment",
	"Exception",
	"FieldOffset",
	"FieldOffsetAttribute",
	"ForeignFuture",
	"FormatException",
	"Func",
	"GC",
	"GCHandle",
	"IAsyncEnumerable",
	"IAsyncEnumerator",
	"IComparable",
	"IDisposable",
	"IEquatable",
	"IOException",
	"ISeekableReader",
	"IValueTaskSource",
	"IWakeable",
	"IntPtr",
	"Interlocked",
	"InvalidOperationException",
	"IpAddr",
	"Ipv4Addr",
	"Ipv6Addr",
	"LayoutKind",
	"Lease",
	"Lib",
	"List",
	"ManualResetValueTaskSourceCore",
	"Marshal",
	"Math",
	"Memory",
	"MemoryExtensions",
	"MemoryHandle",
	"MemoryManager",
	"MemoryMarshal",
	"MethodImpl",
	"MethodImplAttribute",
	"MethodImplOptions",
	"ModuleInitializer",
	"ModuleInitializerAttribute",
	"Native",
	"Object",
	"ObjectDisposedException",
	"OperationCanceledException",
	"PathBuf",
	"Pending",
	"PlatformNotSupportedException",
	"ReadOnlyMemory",
	"ReadOnlySpan",
	"RustException",
	"RustObject",
	"Scalar",
	"SeekOrigin",
	"SeekableStream",
	"SocketAddr",
	"SocketAddrV4",
	"SocketAddrV6",
	"Span",
	"Str",
	"Stream",
	"Streams",
	"String",
	"StructLayout",
	"StructLayoutAttribute",
	"SynchronizationContext",
	"System",
	"Task",
	"TaskScheduler",
	"TaskStatus",
	"Thread",
	"ThreadPool",
	"TimeSpan",
	"U128",
	"UInt128",
	"UnmanagedCallersOnly",
	"UnmanagedCallersOnlyAttribute",
	"UnmanagedFunctionPointer",
	"UnmanagedFunctionPointerAttribute",
	"ValueTask",
	"ValueTaskSourceOnCompletedFlags",
	"ValueTaskSourceStatus",
	"ValueTuple",
	"Volatile",
	"Waker",
];

/// C#'s keywords, which a parameter or field takes with `@`.
const KEYWORDS: &[&str] = &[
	"abstract",
	"as",
	"base",
	"bool",
	"break",
	"byte",
	"case",
	"catch",
	"char",
	"checked",
	"class",
	"const",
	"continue",
	"decimal",
	"default",
	"delegate",
	"do",
	"double",
	"else",
	"enum",
	"event",
	"explicit",
	"extern",
	"false",
	"finally",
	"fixed",
	"float",
	"for",
	"foreach",
	"goto",
	"if",
	"implicit",
	"in",
	"int",
	"interface",
	"internal",
	"is",
	"lock",
	"long",
	"namespace",
	"new",
	"null",
	"object",
	"operator",
	"out",
	"override",
	"params",
	"private",
	"protected",
	"public",
	"readonly",
	"ref",
	"return",
	"sbyte",
	"sealed",
	"short",
	"sizeof",
	"stackalloc",
	"static",
	"string",
	"struct",
	"switch",
	"this",
	"throw",
	"true",
	"try",
	"typeof",
	"uint",
	"ulong",
	"unchecked",
	"unsafe",
	"ushort",
	"using",
	"virtual",
	"void",
	"volatile",
	"while",
];

/// The members every object has, which an exported method's name keeps apart from.
const OBJECT_MEMBERS: &[&str] = &[
	"Dispose",
	"IsClosed",
	"Equals",
	"GetHashCode",
	"ToString",
	"GetType",
	"Finalize",
	"MemberwiseClone",
	"ReferenceEquals",
];

/// What generated code names in an expression, where a member of the name, `Lib`'s too,
/// would shadow it.
const QUALIFIERS: &[&str] = &["Native", "Callbacks", "IntPtr", "GCHandle", "Task", "Marshal"];

/// The instance methods `System.Enum` declares besides `object`'s, which a plain enum's
/// extension method of the same name never reaches.
const ENUM_MEMBERS: &[&str] = &["CompareTo", "HasFlag", "GetTypeCode"];

/// The C# for `abi`, whose header's contract is `contract`, or why it cannot be written.
pub(crate) fn generate(abi: &Abi<'_>, packages: &str, contract: u64) -> Result<Output, String> {
	let ranges: BTreeSet<String> = (abi.ranges.keys())
		.map(|c| pascal(&c[abi.prefix.len() + 1..]))
		.chain(abi.arrays.values().map(|n| Abi::array_class(*n)))
		.collect();
	let nonzero: BTreeSet<String> = ["U8", "U16", "U32", "U64", "Usize", "I8", "I16", "I32", "I64", "Isize"]
		.iter()
		.map(|n| format!("NonZero{n}"))
		.collect();
	let classes = abi.classes(
		|_| (String::new(), 0),
		|name| TAKEN.contains(&name) || nonzero.contains(name) || name.ends_with("Exception"),
	);
	let mut g = Gen {
		abi,
		classes,
		locked: BTreeMap::new(),
		types: String::new(),
		natives: String::new(),
		callbacks: String::new(),
		lib: String::new(),
		declared: BTreeSet::new(),
		structs: BTreeSet::new(),
		helpers: BTreeSet::new(),
		trampolines: HashMap::new(),
		exceptions: BTreeMap::new(),
		nonzero: BTreeSet::new(),
		futures: BTreeSet::new(),
		vtables: BTreeSet::new(),
		fresh: 0,
		errors: Default::default(),
		nonzero_named: Default::default(),
		top: BTreeMap::new(),
		record_fields: Vec::new(),
		variant_names: Vec::new(),
		variant_fields: Vec::new(),
		ns: namespace(&abi.prefix),
		inside: BTreeSet::new(),
	};
	g.locked = crate::lock::types(abi, &g.classes);
	for name in g.classes.values() {
		g.top.insert(name.clone(), "a type".to_string());
	}
	for name in &ranges {
		g.top.insert(name.clone(), "the runtime".to_string());
	}
	g.plan();
	g.write();
	let errors = g.errors.take();
	if !errors.is_empty() {
		return Err(errors.join("\n"));
	}
	let p = &abi.prefix;
	let ns = namespace(p);
	let mut cs = format!(
		"// Generated by rust2-ffi: C# over the C ABI of the pub API of {packages}. Do not edit.\n\
		// <auto-generated/>\n\
		#nullable enable\n\
		#pragma warning disable CS1591, CS8500, CS0649\n\n\
		namespace {ns}\n{{\n\
		using System;\n\
		using System.Buffers;\n\
		using System.Collections.Generic;\n\
		using System.IO;\n\
		using System.Runtime.CompilerServices;\n\
		using System.Runtime.InteropServices;\n\
		using System.Threading;\n\
		using System.Threading.Tasks;\n\
		using System.Threading.Tasks.Sources;\n\n"
	);
	cs.push_str(RUNTIME);
	cs.push('\n');
	// The C functions and the runtime's, each blittable.
	let unity_lib = "__Internal";
	let _ = write!(
		cs,
		"internal static unsafe partial class Native\n{{\n\
		\t/// The library: on iOS, where Unity links it statically, the program itself.\n\
		#if UNITY_IOS && !UNITY_EDITOR\n\tconst string Lib = \"{unity_lib}\";\n#else\n\tconst string Lib = \"{p}\";\n#endif\n\n\
		\t/// The contract of the header this file was generated with.\n\
		\tinternal const ulong Contract = 0x{contract:016x};\n\n\
		\t/// Checks, before any other call, that the library is the one this file was generated with:\n\
		\t/// under Unity, which has no module initializer, from this type's constructor, which runs\n\
		\t/// before any use of it, and whose throw every later use rethrows.\n\
		#if UNITY_5_3_OR_NEWER\n\
		\tstatic Native()\n\t{{\n\t\tLoad();\n\t\tTarget = (int)target();\n\t}}\n\n\
		#else\n\
		\t[ModuleInitializer]\n\
		#endif\n\
		\tinternal static void Load()\n\
		\t{{\n\t\tif (contract() != Contract)\n\t\t{{\n\t\t\tthrow new InvalidOperationException(\"{p}.cs and the library are from different builds\");\n\t\t}}\n\t}}\n\n\
		\tinternal static int HashBytes(ReadOnlySpan<byte> bytes)\n\t{{\n\t\tunchecked\n\t\t{{\n\t\t\tvar h = (int)2166136261;\n\t\t\tforeach (var b in bytes)\n\t\t\t{{\n\t\t\t\th = (h ^ b) * 16777619;\n\t\t\t}}\n\t\t\treturn h;\n\t\t}}\n\t}}\n\n"
	);
	for (name, params, ret) in [
		("contract", "", "ulong"),
		("target", "", "uint"),
		("buf_free", "Buf buf", "void"),
		("buf_copy", "Buf view", "Buf"),
		("buf_clone", "Buf buf", "Buf"),
		("status_message", "int status", "byte*"),
		("close_drain", "IntPtr handle", "byte"),
	] {
		let _ = write!(
			cs,
			"\t[DllImport(Lib, EntryPoint = \"{p}_{name}\", CallingConvention = CallingConvention.Cdecl)]\n\tinternal static extern {ret} {name}({params});\n\n"
		);
	}
	cs.push_str(&g.natives);
	cs.push_str("}\n\n");
	if !g.callbacks.is_empty() {
		cs.push_str("internal static unsafe partial class Callbacks\n{\n");
		cs.push_str(&g.callbacks);
		cs.push_str("}\n\n");
	}
	cs.push_str(&g.types);
	cs.push_str("/// The crate's free functions and constants.\npublic static unsafe partial class Lib\n{\n");
	cs.push_str(&g.lib);
	cs.push_str("}\n");
	cs.push_str("}\n");
	let cs = tidy(&cs);
	Ok(Output {
		names: g.locked,
		csharp: cs,
	})
}

/// The namespace of the library `prefix`, in Pascal case: `api` is `Api`.
pub(crate) fn namespace(prefix: &str) -> String {
	pascal(prefix)
}

/// `code` indented by its braces, each on a line of its own as the generator writes them,
/// and a `switch`'s `case` bodies under it, with no run of blank lines, nor one before a
/// closing brace.
fn tidy(code: &str) -> String {
	let mut out = String::with_capacity(code.len());
	let mut blank = false;
	let lines: Vec<&str> = code.lines().map(str::trim).collect();
	// Each open brace's depth, and whether a `case` label indents what follows it.
	let mut depth = 0usize;
	let mut cases: Vec<bool> = Vec::new();
	for (k, line) in lines.iter().enumerate() {
		if line.is_empty() {
			let next = lines.get(k + 1).copied().unwrap_or("");
			if !blank && !next.starts_with('}') && k > 0 {
				out.push('\n');
			}
			blank = true;
			continue;
		}
		blank = false;
		let label = line.starts_with("case ") && line.ends_with(':') || *line == "default:";
		if (line.starts_with('}') || label) && cases.last() == Some(&true) {
			cases.pop();
			cases.push(false);
			depth -= 1;
		}
		if line.starts_with('}') {
			cases.pop();
			depth = depth.saturating_sub(1);
		}
		if !line.starts_with('#') {
			out.push_str(&"\t".repeat(depth));
		}
		out.push_str(line);
		out.push('\n');
		if label {
			*cases.last_mut().expect("a case in a switch") = true;
			depth += 1;
		}
		if line.ends_with('{') && !line.starts_with("///") && !line.starts_with("//") {
			cases.push(false);
			depth += 1;
		}
	}
	out
}

struct Gen<'a> {
	abi: &'a Abi<'a>,
	/// Each exported type's C# name, by its C name.
	classes: HashMap<String, String>,
	locked: BTreeMap<String, String>,
	/// The public types.
	types: String,
	/// `Native`'s generated half: the C functions, the C structs, and how each lowers and lifts.
	natives: String,
	/// `Callbacks`' generated half: each closure's and trait method's trampoline.
	callbacks: String,
	/// The free functions and constants.
	lib: String,
	/// The C functions declared, by name.
	declared: BTreeSet<String>,
	/// The C structs declared, by name.
	structs: BTreeSet<String>,
	/// The lower and lift helpers written, by C name.
	helpers: BTreeSet<String>,
	/// Each trampoline's field, by its signature.
	trampolines: HashMap<String, String>,
	/// Each exception class, by name, with the error type it holds.
	exceptions: BTreeMap<String, Type>,
	/// The `NonZero` types written.
	nonzero: BTreeSet<&'static str>,
	/// The future classes written, by C name.
	futures: BTreeSet<String>,
	/// The trait tables written, by C name.
	vtables: BTreeSet<String>,
	fresh: usize,
	errors: std::cell::RefCell<Vec<String>>,
	/// Each `NonZero` a C# type names, which `write` defines once it wrote the rest.
	nonzero_named: std::cell::RefCell<Vec<Int>>,
	/// The namespace's names, with what claimed each.
	top: BTreeMap<String, String>,
	/// Each record's fields' C# names.
	record_fields: Vec<Vec<String>>,
	/// Each enum's variants' C# names.
	variant_names: Vec<Vec<String>>,
	/// Each enum's variants' fields' C# names.
	variant_fields: Vec<Vec<Vec<String>>>,
	/// The namespace.
	ns: String,
	/// The variants of the data enum being written, whose classes shadow a type of their name.
	inside: BTreeSet<String>,
}

/// The owner of a function: none, or a type and how C passes it.
#[derive(Clone)]
struct Owner {
	ty: Type,
	c: Ty,
	/// Whether its members are extension methods of a plain enum.
	extension: bool,
}

/// How a parameter passes: what opens and closes around the call, its C arguments, and
/// what runs once it ran.
#[derive(Default)]
struct Arg {
	opens: Vec<(String, String)>,
	exprs: Vec<String>,
	after: Vec<String>,
	/// Checks that throw before the call allocates anything.
	checks: Vec<String>,
	/// Locals of data the call hands Rust, made after every other argument, so nothing
	/// that throws runs between one and the call that takes it.
	gives: Vec<String>,
}

impl<'a> Gen<'a> {
	fn fresh(&mut self, base: &str) -> String {
		self.fresh += 1;
		format!("{base}{}_", self.fresh)
	}

	/// The C# name of the exported type of C name `c`.
	fn class(&self, c: &str) -> String {
		self.classes
			.get(c)
			.cloned()
			.unwrap_or_else(|| pascal(&c[self.abi.prefix.len() + 1..]))
	}

	// ------------------------------------------------------------------------------------
	// Types.

	/// The C# type of `ty`, as a value, a field, or a result: by its full name where a
	/// variant's class of its name shadows it.
	fn cs(&self, ty: &Type) -> String {
		let name = self.cs_(ty);
		match self.inside.contains(&name) {
			true => format!("global::{}.{name}", self.ns),
			false => name,
		}
	}

	fn cs_(&self, ty: &Type) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Unit => "void".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) => cs_int(*int).to_string(),
			Type::NonZero(int) => {
				self.nonzero_named.borrow_mut().push(*int);
				nonzero_name(*int)
			}
			Type::F32 => "float".to_string(),
			Type::F64 => "double".to_string(),
			Type::Char => "Scalar".to_string(),
			Type::Str | Type::String => "Str".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "Bytes".to_string(),
			Type::Path | Type::PathBuf => "PathBuf".to_string(),
			Type::U128 => "U128".to_string(),
			Type::Duration => "Duration".to_string(),
			Type::Array(n) => Abi::array_class(*n),
			Type::SocketAddr(Family::Any) => "SocketAddr".to_string(),
			Type::SocketAddr(Family::V4) => "SocketAddrV4".to_string(),
			Type::SocketAddr(Family::V6) => "SocketAddrV6".to_string(),
			Type::IpAddr(Family::Any) => "IpAddr".to_string(),
			Type::IpAddr(Family::V4) => "Ipv4Addr".to_string(),
			Type::IpAddr(Family::V6) => "Ipv6Addr".to_string(),
			Type::List(_, inner) => format!("{}[]", self.cs(inner)),
			Type::Waker => "Waker".to_string(),
			Type::Future => "Func<CancellationToken, Task>".to_string(),
			Type::Reader => "ISeekableReader".to_string(),
			Type::Option(inner) => {
				if matches!(inner.base(), Type::Option(_)) {
					let inner = self.cs(inner.base());
					(self.errors.borrow_mut())
						.push(format!("`Option<Option<{inner}>>` has no C# form, which has one null"));
				}
				format!("{}?", self.cs(inner))
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.cs(inner),
			Type::Record(i) => self.class(&abi.record(*i)),
			Type::Enum(i) => self.class(&abi.enumeration(*i)),
			Type::Object(i) => self.class(&abi.object(*i)),
			Type::Callback(i, _) => self.class(&abi.callback(*i)),
			Type::Closure(closure) => self.delegate(closure),
			Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => pascal(&abi.range(ty)[abi.prefix.len() + 1..]),
			// A tuple of one is C#'s `ValueTuple<T>`, so `Option<(Option<T>,)>` keeps `None` apart from `Some((None,))`.
			Type::Tuple(elements) => match &elements[..] {
				[element] => format!("ValueTuple<{}>", self.cs(element)),
				elements => {
					let elements: Vec<String> = elements.iter().map(|e| self.cs(e)).collect();
					format!("({})", elements.join(", "))
				}
			},
		}
	}

	/// The C# type of a parameter of type `ty`: a slice Rust reads at the call's top level is
	/// a span of C#'s memory, and a waker an `Action`.
	fn param(&self, ty: &Type) -> String {
		match ty.base() {
			Type::Slice | Type::Vec | Type::Bytes => "ReadOnlySpan<byte>".to_string(),
			list @ Type::List(_, elem) if list.block().is_some() => format!("ReadOnlySpan<{}>", self.cs(elem)),
			Type::Waker => "Action".to_string(),
			_ => self.cs(ty),
		}
	}

	/// The delegate type of a closure.
	fn delegate(&self, closure: &Closure) -> String {
		let params: Vec<String> = closure.params.iter().map(|p| self.lent_cs(p)).collect();
		match &closure.ok {
			Type::Unit if params.is_empty() => "Action".to_string(),
			Type::Unit => format!("Action<{}>", params.join(", ")),
			ok => {
				let mut all = params;
				all.push(self.cs(ok));
				format!("Func<{}>", all.join(", "))
			}
		}
	}

	/// The C# type of a value Rust lends a closure or a trait's method.
	fn lent_cs(&self, ty: &Type) -> String {
		self.cs(ty)
	}

	/// The raw C# type of a C type, as a `DllImport` or a C struct's field takes it.
	fn raw(&self, ty: &Ty) -> String {
		match ty {
			Ty::Void => "void".to_string(),
			Ty::Bool => "byte".to_string(),
			Ty::Int(int) => cs_int(*int).to_string(),
			Ty::F32 => "float".to_string(),
			Ty::F64 => "double".to_string(),
			Ty::Char => "uint".to_string(),
			Ty::Buf => "Native.Buf".to_string(),
			Ty::U128 => "Native.CU128".to_string(),
			Ty::Duration => "Native.CDuration".to_string(),
			Ty::SocketAddr(Family::Any) => "Native.CSocketAddr".to_string(),
			Ty::SocketAddr(Family::V4) => "Native.CSocketAddrV4".to_string(),
			Ty::SocketAddr(Family::V6) => "Native.CSocketAddrV6".to_string(),
			Ty::IpAddr(Family::Any) => "Native.CIpAddr".to_string(),
			Ty::IpAddr(Family::V4) => "Native.CIpv4Addr".to_string(),
			Ty::IpAddr(Family::V6) => "Native.CIpv6Addr".to_string(),
			Ty::Struct(name) => format!("Native.{name}"),
			Ty::Enum(_) => "int".to_string(),
			Ty::Handle { .. } | Ty::Future(_) => "IntPtr".to_string(),
			Ty::Waker => "Native.CWaker".to_string(),
			Ty::ForeignFuture => "Native.CForeignFuture".to_string(),
			Ty::ForeignReader => "Native.CForeignReader".to_string(),
			Ty::Poll | Ty::Status => "int".to_string(),
			Ty::Out(inner) | Ty::In(inner) | Ty::InOut(inner) => format!("{}*", self.raw(inner)),
			Ty::Fn(..) => "IntPtr".to_string(),
			Ty::Data => "void*".to_string(),
			Ty::Jni(_) => unreachable!("only Kotlin passes a JNI value"),
		}
	}

	/// The raw C# type of a value of `ty` as C holds it.
	fn raw_of(&self, ty: &Type) -> String {
		self.raw(&self.abi.ty(ty))
	}

	// ------------------------------------------------------------------------------------
	// C functions and structs.

	/// Declares the C function `sig`, once.
	fn declare(&mut self, sig: &Sig) {
		if !self.declared.insert(sig.name.clone()) {
			return;
		}
		let params: Vec<String> = (sig.params.iter())
			.map(|(name, ty)| format!("{} {}", self.raw(ty), field_name(name)))
			.collect();
		let _ = write!(
			self.natives,
			"\t[DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]\n\tinternal static extern {} {}({});\n\n",
			self.raw(&sig.ret),
			sig.name,
			params.join(", ")
		);
	}

	/// Declares a C function of a name and raw signature, once.
	fn declare_raw(&mut self, name: &str, params: &[(&str, String)], ret: &str) {
		if !self.declared.insert(name.to_string()) {
			return;
		}
		let params: Vec<String> = params.iter().map(|(n, t)| format!("{t} {}", field_name(n))).collect();
		let _ = write!(
			self.natives,
			"\t[DllImport(Lib, CallingConvention = CallingConvention.Cdecl)]\n\tinternal static extern {ret} {name}({});\n\n",
			params.join(", ")
		);
	}

	/// Declares the C struct of `ty`, if it is one, after those it holds.
	fn c_struct(&mut self, ty: &Ty) {
		let abi = self.abi;
		let Ty::Struct(name) = ty else {
			if let Ty::Out(inner) | Ty::In(inner) | Ty::InOut(inner) = ty {
				self.c_struct(inner);
			}
			return;
		};
		if !self.structs.insert(name.clone()) {
			return;
		}
		let mut fields: Vec<String> = Vec::new();
		let mut extra = String::new();
		if let Some(ty) = abi.ranges.get(name) {
			match ty {
				Type::Range(int) => {
					fields.push(format!("public {} start;", cs_int(*int)));
					fields.push(format!("public {} end;", cs_int(*int)));
				}
				Type::Bound(int) => {
					fields.push("public int kind;".to_string());
					fields.push(format!("public {} value;", cs_int(*int)));
				}
				Type::Bounds(int) => {
					let bound = abi.range(&Type::Bound(*int));
					self.c_struct(&Ty::Struct(bound.clone()));
					fields.push(format!("public {bound} start;"));
					fields.push(format!("public {bound} end;"));
				}
				other => unreachable!("{other:?} is no range"),
			}
		} else if let Some(n) = abi.arrays.get(name) {
			fields.push(format!("public fixed byte items[{n}];"));
		} else if let Some(elem) = abi.lists.get(name).cloned() {
			let free = abi::join(name, "free");
			self.declare_raw(&free, &[("vec", name.clone())], "void");
			self.c_struct(&abi.ty(&elem));
			fields.push("public void* ptr;".to_string());
			fields.push("public nuint len;".to_string());
			fields.push("public void* owner;".to_string());
		} else if let Some(inner) = abi.options.get(name).cloned() {
			let inner = abi.ty(&inner);
			self.c_struct(&inner);
			fields.push("public byte some;".to_string());
			fields.push(format!("public {} value;", self.raw(&inner)));
		} else if let Some(elements) = abi.tuples.get(name).cloned() {
			for (n, element) in elements.iter().enumerate() {
				let element = abi.ty(element);
				self.c_struct(&element);
				fields.push(format!("public {} _{n};", self.raw(&element)));
			}
		} else if let Some(i) = (0..abi.api.records.len()).find(|&i| &abi.record(i) == name) {
			let record = &abi.api.records[i];
			if let Some(opaque) = &record.opaque {
				let unit = match opaque.unit() {
					1 => "byte",
					2 => "ushort",
					4 => "uint",
					_ => "ulong",
				};
				fields.push(format!("public fixed {unit} opaque[{}];", opaque.units()));
			} else {
				for f in &record.fields {
					let ty = abi.ty(&f.ty);
					self.c_struct(&ty);
					fields.push(format!(
						"public {} {};",
						self.raw(&ty),
						field_name(&abi::escape(&f.name))
					));
				}
			}
		} else if let Some(i) = (0..abi.api.enums.len()).find(|&i| &abi.enumeration(i) == name) {
			let e = &abi.api.enums[i];
			fields.push("public int tag;".to_string());
			let union = format!("{name}_union_");
			let mut members = Vec::new();
			for variant in e.variants.iter().filter(|v| !v.fields.is_empty()) {
				let payload = abi::payload(name, &variant.name);
				let mut decls = Vec::new();
				for f in &variant.fields {
					let ty = abi.ty(&f.ty);
					self.c_struct(&ty);
					decls.push(format!(
						"\t\tpublic {} {};",
						self.raw(&ty),
						field_name(&abi::escape(&f.name))
					));
				}
				let _ = write!(
					extra,
					"\t[StructLayout(LayoutKind.Sequential)]\n\tinternal struct {payload}\n\t{{\n{}\n\t}}\n\n",
					decls.join("\n")
				);
				members.push(format!(
					"\t\t[FieldOffset(0)] public {payload} {};",
					field_name(&abi::variant_member(&variant.name))
				));
			}
			if !members.is_empty() {
				let _ = write!(
					extra,
					"\t[StructLayout(LayoutKind.Explicit)]\n\tinternal struct {union}\n\t{{\n{}\n\t}}\n\n",
					members.join("\n")
				);
				fields.push(format!("public {union} u;"));
			}
		} else if let Some(i) = (0..abi.api.callbacks.len()).find(|&i| &abi.callback(i) == name) {
			let _ = i;
			fields.push("public IntPtr vtable;".to_string());
			fields.push("public void* data;".to_string());
		} else {
			self.errors.borrow_mut().push(format!("no C struct is named {name}"));
			return;
		}
		let fields: Vec<String> = fields.iter().map(|f| format!("\t\t{f}")).collect();
		let _ = write!(
			self.natives,
			"{extra}\t[StructLayout(LayoutKind.Sequential)]\n\tinternal struct {name}\n\t{{\n{}\n\t}}\n\n",
			fields.join("\n")
		);
	}

	// ------------------------------------------------------------------------------------
	// Lowering and lifting.

	/// Whether lowering `ty` needs the call's arena: to pin bytes, build an array, or hold a
	/// handle.
	fn arena(&self, ty: &Type) -> bool {
		let api = self.abi.api;
		match ty {
			ty if Abi::is_buf(ty) => true,
			Type::List(..) | Type::Object(_) => true,
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => self.arena(inner),
			Type::Tuple(elements) => elements.iter().any(|e| self.arena(e)),
			Type::Record(i) => api.records[*i].fields.iter().any(|f| self.arena(&f.ty)),
			Type::Enum(i) => (api.enums[*i].variants.iter()).any(|v| v.fields.iter().any(|f| self.arena(&f.ty))),
			_ => false,
		}
	}

	/// The C value of the C# value `e` of `ty`, through the arena `a_` where it needs one.
	fn lower(&mut self, ty: &Type, e: &str) -> String {
		let abi = self.abi;
		match ty {
			Type::Unit => unreachable!("no value is unit"),
			Type::Bool => format!("({e} ? (byte)1 : (byte)0)"),
			Type::Int(_) | Type::F32 | Type::F64 => e.to_string(),
			Type::NonZero(_) => format!("{e}.Value"),
			Type::Char => format!("{e}.Value"),
			Type::U128 => format!("Native.U128({e})"),
			Type::Duration => format!("Native.Duration({e})"),
			Type::SocketAddr(_) | Type::IpAddr(_) => format!("Native.Addr({e})"),
			Type::Array(_) => format!("{e}.c_"),
			Type::Str | Type::String | Type::Path | Type::PathBuf | Type::Slice | Type::Vec | Type::Bytes => {
				format!("a_.View({e})")
			}
			// Rust copies a `Copy` value out of its handle, which stays the caller's.
			Type::Object(i) if abi.api.objects[*i].copy => format!("a_.Hold({e}, false)"),
			Type::Object(_) => format!("a_.Hold({e}, a_.Take)"),
			Type::Box(inner) | Type::Ref(_, inner) => self.lower(inner, e),
			Type::Option(inner) if Abi::nullable(inner) => {
				let v = self.fresh("v");
				let some = self.lower(inner, &v);
				let none = match inner.base() {
					Type::Object(_) => "IntPtr.Zero".to_string(),
					_ => "default".to_string(),
				};
				format!("({e} is {{}} {v} ? {some} : {none})")
			}
			Type::Enum(i) if abi.api.enums[*i].plain() => {
				self.helper(ty);
				format!("Native.Lower_{}({e})", abi.enumeration(*i))
			}
			Type::Waker => format!("{e}.Lower()"),
			Type::Future => format!("Native.Future({e})"),
			Type::Callback(i, _) => {
				self.vtable(*i);
				format!("Native.Lower_{}({e})", abi.callback(*i))
			}
			Type::Closure(_) | Type::Reader | Type::Jni(_) | Type::Const(_) => {
				unreachable!("{ty:?} lowers as a parameter")
			}
			_ => {
				let c = abi.ty(ty);
				let Ty::Struct(name) = &c else {
					unreachable!("{ty:?} lowers to a struct")
				};
				let name = name.clone();
				self.helper(ty);
				let arena = match self.arena(ty) {
					true => "a_",
					false => "null!",
				};
				format!("Native.Lower_{name}({e}, {arena})")
			}
		}
	}

	/// The C# value of the C value `e` of `ty`, whose bytes, handles, and arrays it takes.
	fn lift(&mut self, ty: &Type, e: &str) -> String {
		let abi = self.abi;
		match ty {
			Type::Unit => "default".to_string(),
			Type::Bool => format!("({e} != 0)"),
			Type::Int(_) | Type::F32 | Type::F64 => e.to_string(),
			Type::NonZero(int) => {
				self.nonzero(*int);
				format!("new {}({e}, true)", nonzero_name(*int))
			}
			Type::Char => format!("new Scalar({e}, true)"),
			Type::U128 => format!("Native.U128({e})"),
			Type::Duration => format!("Native.Duration({e})"),
			Type::SocketAddr(_) | Type::IpAddr(_) => format!("Native.Addr({e})"),
			Type::Array(n) => format!("new {}({e})", Abi::array_class(*n)),
			Type::Str | Type::String => format!("Native.String({e})"),
			Type::Path | Type::PathBuf => format!("Native.Path({e})"),
			Type::Slice | Type::Vec | Type::Bytes => format!("Native.TakeBytes({e})"),
			Type::Object(_) => format!("new {}({e})", self.cs(ty)),
			Type::Box(inner) | Type::Ref(_, inner) => self.lift(inner, e),
			Type::Waker => format!("new Waker({e})"),
			Type::Option(inner) if Abi::nullable(inner) => match inner.base() {
				Type::Object(_) => format!("({e} == IntPtr.Zero ? null : {})", self.lift(inner, e)),
				Type::Str | Type::String => format!("Native.OptionString({e})"),
				Type::Path | Type::PathBuf => format!("Native.OptionPath({e})"),
				Type::Slice | Type::Vec | Type::Bytes => format!("Native.OptionBytes({e})"),
				_ => format!("({e}.ptr == null ? null : {})", self.lift(inner, e)),
			},
			Type::Enum(i) if abi.api.enums[*i].plain() => format!("({}){e}", self.cs(ty)),
			_ => {
				let c = abi.ty(ty);
				let Ty::Struct(name) = &c else {
					unreachable!("{ty:?} lifts from a struct")
				};
				let name = name.clone();
				self.helper(ty);
				format!("Native.Lift_{name}({e})")
			}
		}
	}

	/// The C# value of the C value `e` of `ty`, which Rust lends a closure or trait method for
	/// its call: bytes and text copied, an object lent, and anything else as it lifts.
	fn lift_lent(&mut self, ty: &Type, e: &str) -> String {
		match ty {
			Type::Str | Type::String => format!("new Str(Native.Copy({e}))"),
			Type::Path | Type::PathBuf => format!("new PathBuf(Native.Copy({e}), true)"),
			Type::Slice | Type::Vec | Type::Bytes => format!("new Bytes(Native.Copy({e}))"),
			Type::Option(inner) if Abi::is_buf(inner) => {
				let lifted = self.lift_lent(inner, e);
				format!("({e}.ptr == null ? null : ({}?){lifted})", self.cs(inner))
			}
			Type::Ref(_, inner) => self.lift_lent(inner, e),
			ty => self.lift(ty, e),
		}
	}

	/// Writes the lower and lift helpers of a C struct type, once.
	fn helper(&mut self, ty: &Type) {
		let abi = self.abi;
		let c = abi.ty(ty);
		self.c_struct(&c);
		let name = match &c {
			Ty::Struct(name) | Ty::Enum(name) => name.clone(),
			_ => return,
		};
		if !self.helpers.insert(name.clone()) {
			return;
		}
		let cs = self.cs(ty);
		let (lower, lift) = match ty {
			Type::Enum(i) if abi.api.enums[*i].plain() => {
				let e = &abi.api.enums[*i];
				// A variant the library's target lacks is refused as it crosses.
				let mut cases = String::new();
				let mut everywhere = Vec::new();
				for v in &e.variants {
					let case = format!("\t\t\tcase {cs}.{}:\n", self.variant_name(*i, &v.name));
					match abi.api.everywhere(v.on) {
						true => everywhere.push(case),
						false => {
							let _ = write!(
								cases,
								"{case}\t\t\t\tNative.On({}, \"{name}.{}\");\n\t\t\t\treturn (int)v;\n",
								v.on.0 & abi.api.all().0,
								v.name
							);
						}
					}
				}
				if !everywhere.is_empty() {
					cases = format!("{}\t\t\t\treturn (int)v;\n{cases}", everywhere.concat());
				}
				let lower = format!(
					"\tinternal static int Lower_{name}({cs} v)\n\t{{\n\t\tswitch (v)\n\t\t{{\n{cases}\t\t\tdefault:\n\t\t\t\tthrow new ArgumentException(\"{cs} holds no variant of \" + (int)v);\n\t\t}}\n\t}}\n\n"
				);
				let _ = write!(self.natives, "{lower}");
				return;
			}
			Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => self.range_helpers(ty, &name, &cs),
			Type::Array(_) => return,
			Type::Tuple(elements) => {
				let mut lowers = Vec::new();
				let mut lifts = Vec::new();
				for (n, element) in elements.iter().enumerate() {
					let item = format!("v.Item{}", n + 1);
					let lowered = self.lower(element, &item);
					lowers.push(format!("_{n} = {lowered}"));
					lifts.push(self.lift(element, &format!("c._{n}")));
				}
				let lift = match elements.len() {
					1 => format!("new {cs}({})", lifts[0]),
					_ => format!("({})", lifts.join(", ")),
				};
				(format!("new {name} {{ {} }}", lowers.join(", ")), lift)
			}
			Type::Option(inner) => {
				let v = "x";
				let lowered = self.lower(inner, v);
				let lifted = self.lift(inner, "c.value");
				(
					format!("v is {{}} {v} ? new {name} {{ some = 1, value = {lowered} }} : default"),
					format!("c.some != 0 ? ({cs}){lifted} : null"),
				)
			}
			Type::List(_, elem) => {
				let raw = self.raw_of(elem);
				let elem_cs = self.cs(elem);
				let lower = match ty.block() {
					Some(_) => format!(
						"{{\n\t\tvar p = Native.Array<{raw}>(v.Length, a_);\n\t\tnew ReadOnlySpan<{raw}>(v).CopyTo(new Span<{raw}>(p, v.Length));\n\t\treturn new {name} {{ ptr = p, len = (nuint)v.Length }};\n\t}}"
					),
					None => {
						let item = self.lower(elem, "v[k]");
						format!(
							"{{\n\t\tvar p = Native.Array<{raw}>(v.Length, a_);\n\t\tfor (var k = 0; k < v.Length; k++)\n\t\t{{\n\t\t\tp[k] = {item};\n\t\t}}\n\t\treturn new {name} {{ ptr = p, len = (nuint)v.Length }};\n\t}}"
						)
					}
				};
				let free = abi::join(&name, "free");
				let lift = match ty.block() {
					Some(_) => format!(
						"{{\n\t\tvar r = Native.Block<{raw}>(c.ptr, c.len);\n\t\t{free}(c);\n\t\treturn r;\n\t}}"
					),
					None => {
						let item = self.lift(elem, &format!("(({raw}*)c.ptr)[k]"));
						format!(
							"{{\n\t\tvar n = checked((int)c.len);\n\t\tvar r = NewArray<{elem_cs}>(n);\n\t\tfor (var k = 0; k < n; k++)\n\t\t{{\n\t\t\tr[k] = {item};\n\t\t}}\n\t\tc.len = 0;\n\t\t{free}(c);\n\t\treturn r;\n\t}}"
						)
					}
				};
				let _ = write!(
					self.natives,
					"\tinternal static {name} Lower_{name}({cs} v, Arena a_)\n\t{lower}\n\n\tinternal static {cs} Lift_{name}({name} c)\n\t{lift}\n\n"
				);
				return;
			}
			Type::Record(i) => {
				let record = &abi.api.records[*i];
				if record.opaque.is_some() {
					("v.c_".to_string(), format!("new {cs}(c)"))
				} else {
					let fields = record.fields.clone();
					let mut lowers = Vec::new();
					let mut lifts = Vec::new();
					for f in &fields {
						let member = self.field_member(*i, &f.name);
						lowers.push(format!(
							"{} = {}",
							field_name(&abi::escape(&f.name)),
							self.lower(&f.ty, &format!("v.{member}"))
						));
						lifts.push(self.lift(&f.ty, &format!("c.{}", field_name(&abi::escape(&f.name)))));
					}
					(
						format!("new {name} {{ {} }}", lowers.join(", ")),
						format!("new {cs}({})", lifts.join(", ")),
					)
				}
			}
			Type::Enum(i) => {
				let e = &abi.api.enums[*i];
				let mut lower_cases = Vec::new();
				let mut lift_cases = Vec::new();
				for (tag, variant) in e.variants.iter().enumerate() {
					let class = format!("{cs}.{}", self.variant_name(*i, &variant.name));
					let member = field_name(&abi::variant_member(&variant.name));
					let mut sets = Vec::new();
					let mut gets = Vec::new();
					for f in &variant.fields {
						let fname = self.variant_field(*i, &variant.name, &f.name);
						let c_field = field_name(&abi::escape(&f.name));
						sets.push(format!(
							"c.u.{member}.{c_field} = {};",
							self.lower(&f.ty, &format!("x.{fname}"))
						));
						gets.push(self.lift(&f.ty, &format!("c.u.{member}.{c_field}")));
					}
					// A variant the library's target lacks is refused as it crosses.
					let on = match abi.api.everywhere(variant.on) {
						true => String::new(),
						false => format!(
							"\t\t\t\tNative.On({}, \"{name}.{}\");\n",
							variant.on.0 & abi.api.all().0,
							variant.name
						),
					};
					lower_cases.push(format!(
						"\t\t\tcase {class} x:\n{on}\t\t\t\tc.tag = {tag};\n{}\t\t\t\treturn c;",
						sets.iter().map(|s| format!("\t\t\t\t{s}\n")).collect::<String>()
					));
					lift_cases.push(format!("\t\t\tcase {tag}: return new {class}({});", gets.join(", ")));
				}
				let _ = write!(
					self.natives,
					"\tinternal static {name} Lower_{name}({cs} v, Arena a_)\n\t{{\n\t\tvar c = default({name});\n\t\tswitch (v)\n\t\t{{\n{}\n\t\t\tdefault:\n\t\t\t\tthrow new ArgumentNullException(nameof(v));\n\t\t}}\n\t}}\n\n\tinternal static {cs} Lift_{name}({name} c)\n\t{{\n\t\tswitch (c.tag)\n\t\t{{\n{}\n\t\t\tdefault: throw new InvalidOperationException(\"{cs}: no variant \" + c.tag);\n\t\t}}\n\t}}\n\n",
					lower_cases.join("\n"),
					lift_cases.join("\n")
				);
				return;
			}
			Type::Box(inner) | Type::Ref(_, inner) => {
				self.helper(inner);
				return;
			}
			_ => return,
		};
		let _ = write!(
			self.natives,
			"\tinternal static {name} Lower_{name}({cs} v, Arena a_) => {lower};\n\n\tinternal static {cs} Lift_{name}({name} c) => {lift};\n\n"
		);
	}

	/// The lower and lift of a range, bound, or pair of bounds.
	fn range_helpers(&mut self, ty: &Type, _name: &str, cs: &str) -> (String, String) {
		let abi = self.abi;
		match ty {
			Type::Range(_) => (
				format!("new {} {{ start = v.Start, end = v.End }}", abi.range(ty)),
				format!("new {cs}(c.start, c.end)"),
			),
			Type::Bound(_) => (
				format!("new {} {{ kind = (int)v.Kind, value = v.Value }}", abi.range(ty)),
				format!("new {cs}((Bound)c.kind, c.value)"),
			),
			Type::Bounds(int) => {
				let bound = Type::Bound(*int);
				self.helper(&bound);
				let b = abi.range(&bound);
				(
					format!(
						"new {} {{ start = Lower_{b}(v.Start, a_), end = Lower_{b}(v.End, a_) }}",
						abi.range(ty)
					),
					format!("new {cs}(Lift_{b}(c.start), Lift_{b}(c.end))"),
				)
			}
			_ => unreachable!(),
		}
	}

	fn nonzero(&mut self, int: Int) {
		let name = match int {
			Int::U8 => "NonZeroU8",
			Int::U16 => "NonZeroU16",
			Int::U32 => "NonZeroU32",
			Int::U64 => "NonZeroU64",
			Int::Usize => "NonZeroUsize",
			Int::I8 => "NonZeroI8",
			Int::I16 => "NonZeroI16",
			Int::I32 => "NonZeroI32",
			Int::I64 => "NonZeroI64",
			Int::Isize => "NonZeroIsize",
		};
		if !self.nonzero.insert(name) {
			return;
		}
		let t = cs_int(int);
		let _ = write!(
			self.types,
			"/// A Rust `NonZero<{rust}>`: a `{t}` that is never zero, which the constructor checks, and\n/// which `default` is one of.\npublic readonly struct {name} : IEquatable<{name}>, IComparable<{name}>\n{{\n\
			\t/// One less than the value, so the struct's zero bits are one.\n\
			\treadonly {t} less;\n\n\
			\tpublic {t} Value => unchecked(({t})(less + 1));\n\n\
			\t/// `value`, throwing an `ArgumentOutOfRangeException` for zero.\n\
			\tpublic {name}({t} value)\n\t{{\n\t\tif (value == 0)\n\t\t{{\n\t\t\tthrow new ArgumentOutOfRangeException(nameof(value), \"zero is no NonZero\");\n\t\t}}\n\t\tless = unchecked(({t})(value - 1));\n\t}}\n\n\
			\tinternal {name}({t} value, bool unchecked_)\n\t{{\n\t\tless = unchecked(({t})(value - 1));\n\t}}\n\n\
			\tpublic static explicit operator {name}({t} value) => new {name}(value);\n\n\
			\tpublic static implicit operator {t}({name} value) => value.Value;\n\n\
			\tpublic override string ToString() => Value.ToString();\n\n\
			\tpublic bool Equals({name} other) => Value == other.Value;\n\n\
			\tpublic override bool Equals(object? obj) => obj is {name} other && Equals(other);\n\n\
			\tpublic override int GetHashCode() => Value.GetHashCode();\n\n\
			\tpublic int CompareTo({name} other) => Value < other.Value ? -1 : Value > other.Value ? 1 : 0;\n}}\n\n",
			rust = int.rust()
		);
	}

	// ------------------------------------------------------------------------------------
	// Names.

	/// The C# name of the variant `variant` of enum `i`.
	fn variant_name(&self, i: usize, variant: &str) -> String {
		let e = &self.abi.api.enums[i];
		let k = e.variants.iter().position(|v| v.name == variant).expect("a variant");
		self.variant_names[i][k].clone()
	}

	/// The C# name of the field `field` of record `i`.
	fn field_member(&self, i: usize, field: &str) -> String {
		let k = (self.abi.api.records[i].fields.iter())
			.position(|f| f.name == field)
			.expect("a field");
		self.record_fields[i][k].clone()
	}

	/// The C# name of the field `field` of variant `variant` of enum `i`.
	fn variant_field(&self, i: usize, variant: &str, field: &str) -> String {
		let e = &self.abi.api.enums[i];
		let v = e.variants.iter().position(|v| v.name == variant).expect("a variant");
		let k = e.variants[v]
			.fields
			.iter()
			.position(|f| f.name == field)
			.expect("a field");
		self.variant_fields[i][v][k].clone()
	}

	/// Names each record's fields and each enum's variants and their fields, apart from the
	/// members beside them: a data enum's variant is a class nested in, and deriving from, the
	/// enum's own, whose members its fields keep apart from.
	fn plan(&mut self) {
		let abi = self.abi;
		let api = abi.api;
		let apart = |rust: Vec<&str>, taken: &[String]| -> Vec<String> {
			let spelled = rust.iter().map(|r| member_name(r)).collect();
			abi::named(
				&rust,
				spelled,
				|rust, name| member_name(rust) == name,
				|_, name| taken.iter().any(|t| t == name),
				abi::underscored,
			)
		};
		for (i, record) in api.records.iter().enumerate() {
			let name = self.class(&abi.record(i));
			let taken = self.base_members(&Type::Record(i), &name);
			let rust = record.fields.iter().map(|f| f.name.as_str()).collect();
			self.record_fields.push(apart(rust, &taken));
		}
		for (i, e) in api.enums.iter().enumerate() {
			let name = self.class(&abi.enumeration(i));
			let rust: Vec<&str> = e.variants.iter().map(|v| v.name.as_str()).collect();
			if e.plain() {
				self.variant_names.push(apart(rust, &[]));
				self.variant_fields.push(Vec::new());
				continue;
			}
			// A variant's class is nested in the enum's, so it shadows a type of its name
			// there: the runtime's and the standard library's take a trailing `_` instead,
			// and an exported type's is written by its full name.
			let mut taken = self.base_members(&Type::Enum(i), &name);
			let reserved: Vec<String> = taken
				.iter()
				.cloned()
				.chain(TAKEN.iter().map(|s| s.to_string()))
				.collect();
			let variants = apart(rust, &reserved);
			taken.extend(variants.iter().cloned());
			let owner = Owner {
				ty: Type::Enum(i),
				c: Ty::Struct(abi.enumeration(i)),
				extension: false,
			};
			let refs: Vec<&Function> = e.methods.iter().collect();
			taken.extend(self.names(&refs, Some(&owner), &taken));
			let mut fields = Vec::new();
			for (v, vname) in e.variants.iter().zip(&variants) {
				let mut taken = taken.clone();
				taken.push(vname.clone());
				fields.push(apart(v.fields.iter().map(|f| f.name.as_str()).collect(), &taken));
			}
			self.variant_names.push(variants);
			self.variant_fields.push(fields);
		}
	}

	/// The members a type's own declare before its methods: those every object has, its own
	/// name, which no member may take, and its std traits'.
	fn base_members(&self, ty: &Type, name: &str) -> Vec<String> {
		let api = self.abi.api;
		let mut members: Vec<String> = OBJECT_MEMBERS.iter().chain(QUALIFIERS).map(|s| s.to_string()).collect();
		members.push(name.to_string());
		let (traits, plain) = match ty {
			Type::Record(i) => (api.records[*i].traits, false),
			Type::Enum(i) => (api.enums[*i].traits, api.enums[*i].plain()),
			Type::Object(i) => (api.objects[*i].traits, false),
			_ => unreachable!(),
		};
		if traits.clone && matches!(ty, Type::Object(_)) {
			members.push("Clone".to_string());
		}
		if let (Type::Enum(i), true) = (ty, plain) {
			members.extend(self.variant_names[*i].iter().cloned());
			members.extend(ENUM_MEMBERS.iter().map(|s| s.to_string()));
		}
		let named = |base: &str| self.trait_name(ty, base);
		match traits.ord {
			crate::ir::Order::None => {}
			crate::ir::Order::Partial => members.push(named("PartialCompareTo")),
			crate::ir::Order::Total => members.extend([named("PartialCompareTo"), named("CompareTo")]),
		}
		if traits.display && plain {
			members.push(named("Display"));
		}
		if traits.debug {
			members.push(named("Debug"));
		}
		if traits.default {
			members.push("Default".to_string());
		}
		if let Type::Object(i) = ty
			&& api.objects[*i].methods.iter().any(|f| f.stream().is_some())
		{
			members.push("GetAsyncEnumerator".to_string());
		}
		members
	}

	/// The name of `ty`'s std trait member `base`. A plain enum's is an extension method,
	/// which C# finds only where no member of the enum's has its name, so it takes Rust's
	/// shorter names, apart from its variants and `System.Enum`'s `CompareTo(object)`.
	fn trait_name(&self, ty: &Type, base: &str) -> String {
		let Type::Enum(i) = ty else { return base.to_string() };
		if !self.abi.api.enums[*i].plain() {
			return base.to_string();
		}
		let mut name = match base {
			"PartialCompareTo" => "PartialCmp".to_string(),
			"CompareTo" => "Cmp".to_string(),
			base => base.to_string(),
		};
		while self.variant_names[*i].contains(&name) || ENUM_MEMBERS.contains(&name.as_str()) {
			name.push('_');
		}
		name
	}

	/// The Rust name `function` spells, a member of `owner` if it has one: its C name past
	/// its owner's, as no clash renames it.
	fn rust_name(&self, function: &Function, owner: Option<&Owner>) -> String {
		let export = self.abi.export(function, owner.map(|o| &o.c));
		let own = match owner.map(|o| &o.c) {
			Some(Ty::Struct(o) | Ty::Enum(o) | Ty::Handle { name: o, .. }) => {
				export.named.strip_prefix(o.as_str()).and_then(|r| r.strip_prefix('_'))
			}
			_ => None,
		};
		let prefix = format!("{}_", self.abi.prefix);
		own.map(str::to_string)
			.unwrap_or_else(|| export.named.strip_prefix(&prefix).unwrap_or(&export.named).to_string())
	}

	/// The C# spelling of a function's Rust name: Pascal case, or a constant's own.
	fn spell(function: &Function, rust: &str) -> String {
		let spelled = match function.access {
			Access::Const => rust.to_string(),
			_ => pascal(rust),
		};
		// A tuple struct's field accessor, `0_get`, starts with a digit, which no name may.
		match spelled.starts_with(|c: char| c.is_ascii_digit()) {
			true => format!("_{spelled}"),
			false => spelled,
		}
	}

	/// The names of `functions`, members of one scope, kept apart from each other and from `taken`.
	/// A field's getter and setter, each a property's accessor, share its field's name.
	fn names(&self, functions: &[&Function], owner: Option<&Owner>, taken: &[String]) -> Vec<String> {
		let accessors: Vec<bool> = functions.iter().map(|f| self.accessor(f, owner)).collect();
		let rust: Vec<String> = (functions.iter().zip(&accessors))
			.map(|(f, &accessor)| match accessor {
				true => f.name().to_string(),
				false => self.rust_name(f, owner),
			})
			.collect();
		let spelled: Vec<String> = (functions.iter().zip(&rust)).map(|(f, r)| Self::spell(f, r)).collect();
		let rust: Vec<&str> = rust.iter().map(String::as_str).collect();
		let field = |k: usize| accessors[k].then(|| functions[k].name());
		abi::named_apart(
			&rust,
			spelled,
			|rust, name| pascal(rust) == name || rust == name,
			|_, name| taken.iter().any(|t| t == name),
			abi::underscored,
			|k, a, j, b| a == b && (field(k).is_none() || field(k) != field(j)),
		)
	}

	/// A top-level name, claimed for `what`.
	fn claim(&mut self, name: &str, what: String) -> String {
		let taken = |n: &str| TAKEN.contains(&n);
		abi::claim(&mut self.top, taken, name, what)
	}

	// ------------------------------------------------------------------------------------
	// Items.

	fn write(&mut self) {
		let abi = self.abi;
		let api = abi.api;
		for (c, ty) in &abi.ranges.clone() {
			self.range(c, ty);
		}
		for n in abi.arrays.values().copied().collect::<BTreeSet<_>>() {
			self.array(n);
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
		// Free functions and constants, as `Lib`'s members, none named as `Lib` itself or a
		// type its bodies name.
		let mut taken: Vec<String> = QUALIFIERS.iter().map(|s| s.to_string()).collect();
		taken.push("Lib".to_string());
		for constant in &api.constants {
			// Its C name without the prefix, which keeps two modules' constants apart.
			let name = abi.constant(&constant.path)[abi.prefix.len() + 1..].to_string();
			let (ty, value) = match constant.value {
				Literal::Bool(b) => ("bool".to_string(), b.to_string()),
				Literal::Int(int, v) => (cs_int(int).to_string(), int_literal(int, v)),
			};
			// C# takes a native-sized `const` only within 32 bits, on every platform.
			let wide = |int, v: i128| match int {
				Int::Usize => u32::try_from(v).is_err(),
				Int::Isize => i32::try_from(v).is_err(),
				_ => false,
			};
			let key = format!("const {}", constant.path.join("::"));
			self.locked.insert(key, name.clone());
			let declaration = match constant.value {
				Literal::Int(int, v) if wide(int, v) => {
					let wide = match int {
						Int::Usize => "ulong",
						_ => "long",
					};
					format!("static readonly {ty} {name} = checked(({ty})({wide}){v});")
				}
				_ => format!("const {ty} {name} = {value};"),
			};
			let _ = write!(self.lib, "{}\tpublic {declaration}\n\n", docs(&constant.docs, "\t"));
			taken.push(name);
		}
		let functions: Vec<&Function> = api.functions.iter().collect();
		let names = self.names(&functions, None, &taken);
		for (function, name) in functions.iter().zip(names) {
			let code = self.function(function, None, &name);
			self.lib.push_str(&code);
		}
		// Each exception class.
		for (name, err) in self.exceptions.clone() {
			let cs = self.cs(&err);
			let message = self.message(&err, "error");
			let _ = write!(
				self.types,
				"/// What a Rust call throws for an error of `{cs}`, which `Error` holds.\npublic sealed class {name} : RustException\n{{\n\tpublic {cs} Error {{ get; }}\n\n\tpublic {name}({cs} error) : base({message})\n\t{{\n\t\tError = error;\n\t}}\n}}\n\n"
			);
		}
		// Each `NonZero` named, by a parameter alone too.
		for int in self.nonzero_named.take() {
			self.nonzero(int);
		}
	}

	/// The message an exception holding `e`, an error of `ty`, shows: its `Display`, else its
	/// `Debug`, else its type.
	fn message(&self, ty: &Type, e: &str) -> String {
		let api = self.abi.api;
		let traits = match ty.base() {
			Type::Record(i) => Some(api.records[*i].traits),
			Type::Enum(i) => Some(api.enums[*i].traits),
			Type::Object(i) => Some(api.objects[*i].traits),
			Type::Str | Type::String | Type::PathBuf | Type::Path => return format!("{e}.ToString()"),
			_ => None,
		};
		let plain = matches!(ty.base(), Type::Enum(i) if api.enums[*i].plain());
		// By the names the plan gave the trait members, which a variant's may have moved.
		let base = ty.base().clone();
		match traits {
			Some(t) if t.display && plain => format!("{e}.{}()", self.trait_name(&base, "Display")),
			Some(t) if t.display => format!("{e}.ToString()"),
			Some(t) if t.debug => format!("{e}.{}()", self.trait_name(&base, "Debug")),
			_ if matches!(ty.base(), Type::List(..)) => format!("\"[\" + string.Join(\", \", {e}) + \"]\""),
			_ => format!("{e}.ToString() ?? \"\""),
		}
	}

	/// The exception class an error of `ty` throws as.
	fn exception(&mut self, ty: &Type) -> String {
		let ty = crate::kotlin::strip_refs(ty);
		let spelled = match &ty {
			Type::Str | Type::String => "String".to_string(),
			ty => pascal(&self.abi.spell(ty)),
		};
		let mut name = format!("{spelled}Exception");
		// One class per type C# holds the error as, so `&str`'s and `String`'s share one.
		let native = self.cs(&ty);
		while let Some(held) = self.exceptions.get(&name) {
			if self.cs(held) == native {
				break;
			}
			name.push('_');
		}
		if !self.exceptions.contains_key(&name) {
			name = self.claim(&name, format!("the exception of {}", self.abi.spell(&ty)));
			self.exceptions.insert(name.clone(), ty.clone());
			crate::lock::exception(&mut self.locked, self.abi, &ty, &native, &name);
		}
		name
	}

	fn range(&mut self, c: &str, ty: &Type) {
		let abi = self.abi;
		let name = pascal(&c[abi.prefix.len() + 1..]);
		let code = match ty {
			Type::Range(int) => {
				let t = cs_int(*int);
				format!(
					"/// A Rust `Range<{rust}>`: from `Start`, up to but not including `End`.\npublic struct {name} : IEquatable<{name}>\n{{\n\tpublic {t} Start;\n\tpublic {t} End;\n\n\tpublic {name}({t} start, {t} end)\n\t{{\n\t\tStart = start;\n\t\tEnd = end;\n\t}}\n\n\tpublic bool Equals({name} other) => Start == other.Start && End == other.End;\n\n\tpublic override bool Equals(object? obj) => obj is {name} other && Equals(other);\n\n\tpublic override int GetHashCode() => (Start, End).GetHashCode();\n}}\n\n",
					rust = int.rust()
				)
			}
			Type::Bound(int) => {
				let t = cs_int(*int);
				if !self.top.contains_key("Bound") {
					self.top.insert("Bound".to_string(), "the runtime".to_string());
					self.types.push_str("/// What a range's bound does: nothing, include its value, or exclude it.\npublic enum Bound\n{\n\tUnbounded = 0,\n\tIncluded = 1,\n\tExcluded = 2,\n}\n\n");
				}
				format!(
					"/// A Rust `Bound<{rust}>`: its kind, and the value it includes or excludes.\npublic struct {name} : IEquatable<{name}>\n{{\n\tpublic Bound Kind;\n\tpublic {t} Value;\n\n\tpublic {name}(Bound kind, {t} value = 0)\n\t{{\n\t\tKind = kind;\n\t\tValue = kind == Bound.Unbounded ? ({t})0 : value;\n\t}}\n\n\tpublic static {name} Unbounded => new {name}(Bound.Unbounded);\n\n\tpublic static {name} Included({t} value) => new {name}(Bound.Included, value);\n\n\tpublic static {name} Excluded({t} value) => new {name}(Bound.Excluded, value);\n\n\tpublic bool Equals({name} other) => Kind == other.Kind && Value == other.Value;\n\n\tpublic override bool Equals(object? obj) => obj is {name} other && Equals(other);\n\n\tpublic override int GetHashCode() => (Kind, Value).GetHashCode();\n\n\tpublic override string ToString() => Kind switch {{ Bound.Included => \"Included(\" + Value + \")\", Bound.Excluded => \"Excluded(\" + Value + \")\", _ => \"Unbounded\" }};\n}}\n\n",
					rust = int.rust()
				)
			}
			Type::Bounds(int) => {
				let bound = pascal(&abi.range(&Type::Bound(*int))[abi.prefix.len() + 1..]);
				format!(
					"/// A Rust `RangeBounds<{rust}>`: its start and end bounds.\npublic struct {name} : IEquatable<{name}>\n{{\n\tpublic {bound} Start;\n\tpublic {bound} End;\n\n\tpublic {name}({bound} start, {bound} end)\n\t{{\n\t\tStart = start;\n\t\tEnd = end;\n\t}}\n\n\tpublic bool Equals({name} other) => Start.Equals(other.Start) && End.Equals(other.End);\n\n\tpublic override bool Equals(object? obj) => obj is {name} other && Equals(other);\n\n\tpublic override int GetHashCode() => (Start, End).GetHashCode();\n}}\n\n",
					rust = int.rust()
				)
			}
			_ => unreachable!(),
		};
		self.types.push_str(&code);
	}

	fn array(&mut self, n: usize) {
		let name = Abi::array_class(n);
		let c = self.abi.array(n);
		self.c_struct(&Ty::Struct(c.clone()));
		let _ = write!(
			self.types,
			"/// A Rust `[u8; {n}]`: exactly {n} bytes, held inline.\npublic unsafe struct {name} : IEquatable<{name}>\n{{\n\tinternal Native.{c} c_;\n\n\tinternal {name}(Native.{c} c)\n\t{{\n\t\tc_ = c;\n\t}}\n\n\
			\t/// A copy of `bytes`, throwing an `ArgumentException` unless it holds exactly {n}.\n\tpublic {name}(ReadOnlySpan<byte> bytes)\n\t{{\n\t\tif (bytes.Length != {n})\n\t\t{{\n\t\t\tthrow new ArgumentException(\"a [u8; {n}] holds exactly {n} bytes\", nameof(bytes));\n\t\t}}\n\t\tc_ = default;\n\t\tfixed (byte* p = c_.items)\n\t\t{{\n\t\t\tbytes.CopyTo(new Span<byte>(p, {n}));\n\t\t}}\n\t}}\n\n\
			\tpublic byte this[int index]\n\t{{\n\t\tget\n\t\t{{\n\t\t\tif ((uint)index >= {n})\n\t\t\t{{\n\t\t\t\tthrow new IndexOutOfRangeException();\n\t\t\t}}\n\t\t\treturn c_.items[index];\n\t\t}}\n\t}}\n\n\
			\tpublic int Length => {n};\n\n\
			\tpublic byte[] ToArray()\n\t{{\n\t\tvar bytes = new byte[{n}];\n\t\tfor (var k = 0; k < {n}; k++)\n\t\t{{\n\t\t\tbytes[k] = c_.items[k];\n\t\t}}\n\t\treturn bytes;\n\t}}\n\n\
			\tpublic bool Equals({name} other) => ToArray().AsSpan().SequenceEqual(other.ToArray());\n\n\tpublic override bool Equals(object? obj) => obj is {name} other && Equals(other);\n\n\tpublic override int GetHashCode() => Native.HashBytes(ToArray());\n}}\n\n"
		);
	}

	/// The body of a std trait's call of the C function `f` on `operands`, C# values of `ty`
	/// each passed behind a pointer, which returns the raw `ret` and then `finish` of it, `r_`.
	fn trait_body(&mut self, ty: &Type, f: &str, operands: &[&str], ret: &str, finish: &str) -> String {
		let abi = self.abi;
		let guard = self.on_guard(ty, f);
		let status = match ty {
			Type::Object(i) => abi.traits_status(*i),
			ty => abi.value_status(ty),
		};
		let plain = matches!(ty, Type::Enum(i) if abi.api.enums[*i].plain());
		let arena = !plain && !matches!(ty, Type::Object(_)) && self.arena(ty);
		let mut opens: Vec<(String, String)> = Vec::new();
		let mut args = Vec::new();
		for (k, e) in operands.iter().enumerate() {
			match ty {
				Type::Object(_) => {
					opens.push((
						format!("var h{k}_ = {e}.enter(out var o{k}_);\n\t\ttry\n\t\t{{\n"),
						format!("\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\t{e}.exit(o{k}_);\n\t\t}}\n"),
					));
					args.push(format!("h{k}_"));
				}
				Type::Enum(i) if plain => {
					opens.push((
						format!("var c{k}_ = Native.Lower_{}({e});\n", abi.enumeration(*i)),
						String::new(),
					));
					args.push(format!("&c{k}_"));
				}
				ty => {
					let lowered = self.lower(ty, e);
					opens.push((format!("var c{k}_ = {lowered};\n"), String::new()));
					args.push(format!("&c{k}_"));
				}
			}
		}
		if status {
			args.push("&s_".to_string());
		}
		let mut code = format!("\t\tr_ = Native.{f}({});\n", args.join(", "));
		for (open, close) in opens.iter().rev() {
			code = format!("\t\t{open}{code}{close}");
		}
		if arena {
			code = format!(
				"\t\tvar a_ = new Arena();\n\t\ttry\n\t\t{{\n\t\t\ta_.Take = false;\n{code}\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\ta_.Dispose();\n\t\t}}\n"
			);
		}
		let mut body = format!("\t{{\n{guard}\t\t{ret} r_;\n");
		if status {
			body.push_str("\t\tint s_ = 0;\n");
		}
		body.push_str(&code);
		if status {
			let _ = writeln!(body, "\t\tNative.Check(s_, \"{f}\");");
		}
		let _ = write!(body, "\t\treturn {finish};\n\t}}");
		body
	}

	/// The check refusing a call of `f`, a C function of the type `ty`, where the library's
	/// target lacks the type, as a statement of a body; none where every target has it.
	fn on_guard(&self, ty: &Type, f: &str) -> String {
		let api = self.abi.api;
		let on = match ty {
			Type::Record(i) => api.records[*i].on,
			Type::Enum(i) => api.enums[*i].on,
			Type::Object(i) => api.objects[*i].on,
			_ => return String::new(),
		};
		match api.everywhere(on) {
			true => String::new(),
			false => format!("\t\tNative.On({}, \"{f}\");\n", on.0 & api.all().0),
		}
	}

	/// The std trait members of a type, as C# spells them, each among the type's base
	/// members, which its methods keep apart from.
	fn traits(&mut self, ty: &Type, owner: &Owner, members: &mut Vec<String>) -> String {
		let abi = self.abi;
		let api = abi.api;
		let (traits, c_name, plain) = match ty {
			Type::Record(i) => (api.records[*i].traits, abi.record(*i), false),
			Type::Enum(i) => (api.enums[*i].traits, abi.enumeration(*i), api.enums[*i].plain()),
			Type::Object(i) => (api.objects[*i].traits, abi.object(*i), false),
			_ => unreachable!(),
		};
		let shown = api.shown(ty).clone();
		let cs = self.cs(ty);
		let status = match ty {
			Type::Object(i) => abi.traits_status(*i),
			ty => abi.value_status(ty),
		};
		let c = abi.ty(ty);
		let object = matches!(ty, Type::Object(_));
		// What each C function takes the type as: a handle, a plain enum's value, or a pointer.
		let ptr = match ty {
			Type::Object(_) => "IntPtr".to_string(),
			_ if plain => "int*".to_string(),
			_ => format!("{}*", self.raw(&c)),
		};
		let declare = |g: &mut Gen<'_>, name: &str, params: &[(&str, String)], ret: &str| {
			let mut params: Vec<(&str, String)> = params.to_vec();
			if status {
				params.push(("status", "int*".to_string()));
			}
			g.declare_raw(name, &params, ret);
		};
		let this = match plain {
			true => "self",
			false => "this",
		};
		// A member of a plain enum is an extension method; any other is an instance's.
		let head = |name: &str, ret: &str| match owner.extension {
			true => format!("\tpublic static {ret} {name}(this {cs} self)"),
			false => format!("\tpublic {ret} {name}()"),
		};
		let nullable = match self.is_value(ty) {
			true => "",
			false => "?",
		};
		let mut out = String::new();
		if traits.clone && object {
			let f = abi::join(&c_name, "clone");
			declare(self, &f, &[("self", "IntPtr".to_string())], "IntPtr");
			members.push("Clone".to_string());
			crate::lock::trait_member(&mut self.locked, &shown, "Clone", "Clone");
			let body = self.trait_body(ty, &f, &["this"], "IntPtr", &format!("new {cs}(r_)"));
			let _ = write!(
				out,
				"\t/// A clone of its value, as Rust's `Clone`.\n\tpublic {cs} Clone()\n{body}\n\n"
			);
		}
		// A plain enum's `==` is C#'s own, by its value.
		if traits.eq && !plain {
			let f = abi::join(&c_name, "eq");
			declare(self, &f, &[("a", ptr.clone()), ("b", ptr.clone())], "byte");
			crate::lock::trait_member(&mut self.locked, &shown, "PartialEq", "Equals");
			let body = self.trait_body(ty, &f, &["this", "other"], "byte", "r_ != 0");
			let check = match nullable {
				"?" => {
					// No shortcut for itself: Rust's `==` decides, as a NaN it holds is unequal.
					"\t\tif (other is null)\n\t\t{\n\t\t\treturn false;\n\t\t}\n"
				}
				_ => "",
			};
			let body = body.replacen("\t{\n", &format!("\t{{\n{check}"), 1);
			let _ = write!(
				out,
				"\t/// Whether it equals `other`, as Rust's `PartialEq`.\n\tpublic bool Equals({cs}{nullable} other)\n{body}\n\n\tpublic override bool Equals(object? obj) => obj is {cs} other && Equals(other);\n\n"
			);
			let hash = match traits.hash {
				true => {
					let h = abi::join(&c_name, "hash");
					declare(self, &h, &[("self", ptr.clone())], "ulong");
					crate::lock::trait_member(&mut self.locked, &shown, "Hash", "GetHashCode");
					self.trait_body(ty, &h, &["this"], "ulong", "(int)(r_ ^ (r_ >> 32))")
				}
				// No hash C# computes agrees with Rust's `==`.
				false => "\t\t=> 0;".to_string(),
			};
			let _ = write!(
				out,
				"\t/// Rust's `Hash`, folded to 32 bits, or 0 without one, as no hash C# computes agrees with Rust's `==`.\n\tpublic override int GetHashCode()\n{hash}\n\n"
			);
			let equals = match nullable {
				"?" => "a is null ? b is null : a.Equals(b)",
				_ => "a.Equals(b)",
			};
			let _ = write!(
				out,
				"\tpublic static bool operator ==({cs}{nullable} a, {cs}{nullable} b) => {equals};\n\n\tpublic static bool operator !=({cs}{nullable} a, {cs}{nullable} b) => !(a == b);\n\n"
			);
		}
		for (t, suffix) in crate::c::comparisons(traits.ord) {
			let f = abi::join(&c_name, suffix);
			declare(self, &f, &[("a", ptr.clone()), ("b", ptr.clone())], "sbyte");
			let (name, helper) = match *t {
				"PartialOrd" => (self.trait_name(ty, "PartialCompareTo"), "partialCmp_"),
				_ => (self.trait_name(ty, "CompareTo"), "cmp_"),
			};
			members.push(name.clone());
			crate::lock::trait_member(&mut self.locked, &shown, t, &name);
			let body = self.trait_body(ty, &f, &["a", "b"], "sbyte", "r_");
			let _ = write!(out, "\tstatic int {helper}({cs} a, {cs} b)\n{body}\n\n");
			match (*t, owner.extension) {
				("PartialOrd", true) => {
					let _ = write!(
						out,
						"\t/// Rust's `PartialOrd`: -1, 0, 1, or null where the two are unordered.\n\tpublic static int? {name}(this {cs} self, {cs} other)\n\t{{\n\t\tvar r_ = partialCmp_(self, other);\n\t\treturn r_ == 2 ? null : r_;\n\t}}\n\n"
					);
				}
				("PartialOrd", false) => {
					let _ = write!(
						out,
						"\t/// Rust's `PartialOrd`: -1, 0, 1, or null where the two are unordered.\n\tpublic int? PartialCompareTo({cs} other)\n\t{{\n\t\tvar r_ = partialCmp_(this, other);\n\t\treturn r_ == 2 ? null : r_;\n\t}}\n\n"
					);
					for (op, test) in [
						("<", "r_ == -1"),
						("<=", "r_ == -1 || r_ == 0"),
						(">", "r_ == 1"),
						(">=", "r_ == 1 || r_ == 0"),
					] {
						let _ = write!(
							out,
							"\tpublic static bool operator {op}({cs} a, {cs} b)\n\t{{\n\t\tvar r_ = partialCmp_(a, b);\n\t\treturn {test};\n\t}}\n\n"
						);
					}
				}
				(_, true) => {
					let _ = write!(
						out,
						"\t/// Rust's `Ord`: -1, 0, or 1.\n\tpublic static int {name}(this {cs} self, {cs} other) => cmp_(self, other);\n\n"
					);
				}
				(_, false) => {
					let compare = match nullable {
						"?" => "other is null ? 1 : cmp_(this, other)",
						_ => "cmp_(this, other)",
					};
					let _ = write!(
						out,
						"\t/// Rust's `Ord`: -1, 0, or 1, and 1 against null, which `IComparable` orders first.\n\tpublic int CompareTo({cs}{nullable} other) => {compare};\n\n"
					);
				}
			}
		}
		if traits.display {
			let f = abi::join(&c_name, "to_string");
			declare(self, &f, &[("self", ptr.clone())], "Native.Buf");
			let name = match plain {
				true => self.trait_name(ty, "Display"),
				false => "ToString".to_string(),
			};
			members.push(name.clone());
			crate::lock::trait_member(&mut self.locked, &shown, "Display", &name);
			let body = self.trait_body(ty, &f, &[this], "Native.Buf", "Native.String(r_).ToString()");
			let head = match plain {
				true => head(&name, "string"),
				false => "\tpublic override string ToString()".to_string(),
			};
			let _ = write!(out, "\t/// Its text, as Rust's `Display`.\n{head}\n{body}\n\n");
		}
		if traits.debug {
			let f = abi::join(&c_name, "debug");
			declare(self, &f, &[("self", ptr.clone())], "Native.Buf");
			let name = self.trait_name(ty, "Debug");
			members.push(name.clone());
			crate::lock::trait_member(&mut self.locked, &shown, "Debug", &name);
			let body = self.trait_body(ty, &f, &[this], "Native.Buf", "Native.String(r_).ToString()");
			let _ = write!(
				out,
				"\t/// Its text, as Rust's `Debug`.\n{}\n{body}\n\n",
				head(&name, "string")
			);
		}
		if traits.default {
			let f = abi::join(&c_name, "default");
			let ret = match ty {
				Type::Object(_) => "IntPtr".to_string(),
				_ if plain => "int".to_string(),
				_ => self.raw(&c),
			};
			self.declare_raw(&f, &[], &ret);
			members.push("Default".to_string());
			crate::lock::trait_member(&mut self.locked, &shown, "Default", "Default");
			let lifted = self.lift(ty, &format!("Native.{f}()"));
			let guard = self.on_guard(ty, &f);
			let _ = write!(
				out,
				"\t/// Its default, as Rust's `Default`.\n\tpublic static {cs} Default()\n\t{{\n{guard}\t\treturn {lifted};\n\t}}\n\n"
			);
		}
		out
	}

	/// The members of `functions`, of `owner`: a field's getter and setter one property.
	fn methods(&mut self, functions: &[Function], owner: &Owner, members: &mut Vec<String>) -> String {
		let refs: Vec<&Function> = functions.iter().collect();
		let names = self.names(&refs, Some(owner), members);
		let mut out = String::new();
		for (k, (function, name)) in functions.iter().zip(&names).enumerate() {
			members.push(name.clone());
			if !self.accessor(function, Some(owner)) {
				out.push_str(&self.function(function, Some(owner), name));
				continue;
			}
			let set = matches!(function.access, Access::Field { set: true, .. });
			// The other accessor of its field, which the property holds too.
			let pair = (functions.iter().zip(&names).enumerate()).find(|(j, (f, n))| {
				*j != k && *n == name && self.accessor(f, Some(owner)) && f.name() == function.name()
			});
			if set && pair.is_some() {
				continue;
			}
			let member = self.member(function, Some(owner), name);
			let other = pair.map(|(_, (f, _))| self.member(f, Some(owner), name));
			let (get, set) = match set {
				true => (None, Some(&member)),
				false => (Some(&member), other.as_ref()),
			};
			let property = Member {
				docs: get.or(set).map(|m| m.docs.clone()).unwrap_or_default(),
				..member.clone()
			};
			out.push_str(&property.property(get.map(|m| &m.body[..]), set.map(|m| &m.body[..])));
		}
		out
	}

	fn record(&mut self, i: usize) {
		let abi = self.abi;
		let record = &abi.api.records[i];
		let c_name = abi.record(i);
		let name = self.class(&c_name);
		let owner = Owner {
			ty: Type::Record(i),
			c: Ty::Struct(c_name.clone()),
			extension: false,
		};
		self.c_struct(&owner.c);
		let mut members = self.base_members(&owner.ty, &name);
		let mut body = String::new();
		if record.opaque.is_some() {
			let _ = write!(
				body,
				"\tinternal Native.{c_name} c_;\n\n\tinternal {name}(Native.{c_name} c)\n\t{{\n\t\tc_ = c;\n\t}}\n\n"
			);
		} else {
			let mut params = Vec::new();
			let mut sets = Vec::new();
			let names = ctor_params(record.fields.iter().map(|f| f.name.as_str()));
			for (f, param) in record.fields.iter().zip(names) {
				let member = self.field_member(i, &f.name);
				crate::lock::field(&mut self.locked, &record.shown, &f.name, &member);
				members.push(member.clone());
				let _ = writeln!(body, "{}\tpublic {} {member};", docs(&f.docs, "\t"), self.cs(&f.ty));
				params.push(format!("{} {param}", self.cs(&f.ty)));
				sets.push(format!("\t\tthis.{member} = {param};"));
			}
			if !record.fields.is_empty() {
				let _ = write!(
					body,
					"\n\tpublic {name}({})\n\t{{\n{}\n\t}}\n\n",
					params.join(", "),
					sets.join("\n")
				);
			}
		}
		let traits = self.traits(&Type::Record(i), &owner, &mut members);
		let methods = self.methods(&record.methods, &owner, &mut members);
		let ifaces = self.interfaces(&Type::Record(i), &name);
		let _ = write!(
			self.types,
			"{}public unsafe partial struct {name}{ifaces}\n{{\n{body}{traits}{methods}}}\n\n",
			docs(&record.docs, "")
		);
	}

	/// The interfaces a type implements for its std traits.
	fn interfaces(&self, ty: &Type, name: &str) -> String {
		let api = self.abi.api;
		let traits = match ty {
			Type::Record(i) => api.records[*i].traits,
			Type::Enum(i) => api.enums[*i].traits,
			Type::Object(i) => api.objects[*i].traits,
			_ => return String::new(),
		};
		let mut list = Vec::new();
		if traits.eq {
			list.push(format!("IEquatable<{name}>"));
		}
		if traits.ord == crate::ir::Order::Total {
			list.push(format!("IComparable<{name}>"));
		}
		match list.is_empty() {
			true => String::new(),
			false => format!(" : {}", list.join(", ")),
		}
	}

	fn enumeration(&mut self, i: usize) {
		let abi = self.abi;
		let e = &abi.api.enums[i];
		let c_name = abi.enumeration(i);
		let name = self.class(&c_name);
		for v in &e.variants {
			let vname = self.variant_name(i, &v.name);
			crate::lock::variant(&mut self.locked, &e.shown, &v.name, &vname);
		}
		if e.plain() {
			let variants: Vec<String> = (e.variants.iter())
				.map(|v| format!("\t{} = {},", self.variant_name(i, &v.name), v.discriminant))
				.collect();
			let _ = write!(
				self.types,
				"{}public enum {name}\n{{\n{}\n}}\n\n",
				docs(&e.docs, ""),
				variants.join("\n")
			);
			let t = e.traits;
			let has = t.debug || t.display || t.ord != crate::ir::Order::None || t.default || t.hash;
			if e.methods.is_empty() && !has {
				return;
			}
			let class = self.claim(&format!("{name}Extensions"), format!("the functions of `{name}`"));
			let owner = Owner {
				ty: Type::Enum(i),
				c: Ty::Enum(c_name.clone()),
				extension: true,
			};
			self.helper(&Type::Enum(i));
			let mut members = self.base_members(&owner.ty, &class);
			let traits = self.traits(&Type::Enum(i), &owner, &mut members);
			let methods = self.methods(&e.methods, &owner, &mut members);
			let _ = write!(
				self.types,
				"/// The functions and std traits of `{name}`.\npublic static unsafe partial class {class}\n{{\n{traits}{methods}}}\n\n"
			);
			return;
		}
		let owner = Owner {
			ty: Type::Enum(i),
			c: Ty::Struct(c_name.clone()),
			extension: false,
		};
		self.c_struct(&owner.c);
		let mut members = self.base_members(&owner.ty, &name);
		self.inside = self.variant_names[i].iter().cloned().collect();
		let mut variants = String::new();
		for v in &e.variants {
			let vname = self.variant_name(i, &v.name);
			members.push(vname.clone());
			let mut fields = String::new();
			let mut params = Vec::new();
			let mut sets = Vec::new();
			let names = ctor_params(v.fields.iter().map(|f| f.name.as_str()));
			for (f, param) in v.fields.iter().zip(names) {
				let member = self.variant_field(i, &v.name, &f.name);
				crate::lock::variant_field(&mut self.locked, &e.shown, &v.name, &f.name, &member);
				let _ = writeln!(fields, "\t\tpublic {} {member};", self.cs(&f.ty));
				params.push(format!("{} {param}", self.cs(&f.ty)));
				sets.push(format!("\t\t\tthis.{member} = {param};"));
			}
			let ctor = match v.fields.is_empty() {
				true => String::new(),
				false => format!(
					"\n\t\tpublic {vname}({})\n\t\t{{\n{}\n\t\t}}\n",
					params.join(", "),
					sets.join("\n")
				),
			};
			let _ = write!(
				variants,
				"\tpublic sealed class {vname} : {name}\n\t{{\n{fields}{ctor}\t}}\n\n"
			);
		}
		let traits = self.traits(&Type::Enum(i), &owner, &mut members);
		let methods = self.methods(&e.methods, &owner, &mut members);
		let ifaces = self.interfaces(&Type::Enum(i), &name);
		let _ = write!(
			self.types,
			"{}public abstract unsafe partial class {name}{ifaces}\n{{\n\tprivate protected {name}()\n\t{{\n\t}}\n\n{variants}{traits}{methods}}}\n\n",
			docs(&e.docs, "")
		);
		self.inside.clear();
	}

	fn object(&mut self, i: usize) {
		let abi = self.abi;
		let object = &abi.api.objects[i];
		let c_name = abi.object(i);
		let name = self.class(&c_name);
		let owner = Owner {
			ty: Type::Object(i),
			c: Ty::Handle {
				name: c_name.clone(),
				constant: false,
			},
			extension: false,
		};
		let free = abi::join(&c_name, "free");
		self.declare_raw(&free, &[("self", "IntPtr".to_string())], "void");
		let mut members = self.base_members(&owner.ty, &name);
		crate::lock::trait_member(&mut self.locked, &object.shown, "Drop", "Dispose");
		let mut body = format!(
			"\tinternal {name}(IntPtr raw, bool lent = false) : base(raw, lent)\n\t{{\n\t}}\n\n\tinternal override void free(IntPtr raw) => Native.{free}(raw);\n\n"
		);
		if abi.checked[i] {
			// The header leaves `_drop_signal` out: C#'s dispose passes `commit`.
			let drop = abi::join(&c_name, "drop_signal");
			self.declare_raw(
				&drop,
				&[
					("self", "IntPtr".to_string()),
					("status", "int*".to_string()),
					("commit", "IntPtr".to_string()),
					("data", "long*".to_string()),
				],
				"void",
			);
			let _ = write!(
				body,
				"\tinternal override bool dropped => true;\n\n\tinternal override void drop(IntPtr raw, int* status, IntPtr commit, long* state) => Native.{drop}(raw, status, commit, state);\n\n"
			);
		}
		let traits = self.traits(&Type::Object(i), &owner, &mut members);
		let refs: Vec<&Function> = object.methods.iter().collect();
		let names = self.names(&refs, Some(&owner), &members);
		let methods = self.methods(&object.methods, &owner, &mut members);
		let mut ifaces = self.interfaces(&Type::Object(i), &name);
		// A stream iterates in C#'s own protocol.
		let mut stream = String::new();
		if let Some(next) = object.methods.iter().find(|f| f.stream().is_some()) {
			let item = next.stream().expect("a stream").clone();
			let item_cs = self.cs(&item);
			let at = object
				.methods
				.iter()
				.position(|f| std::ptr::eq(f, next))
				.expect("found");
			let next_name = &names[at];
			let each = match self.is_value(&item) {
				true => "Structs",
				false => "Classes",
			};
			ifaces = match ifaces.is_empty() {
				true => format!(" : IAsyncEnumerable<{item_cs}>"),
				false => format!("{ifaces}, IAsyncEnumerable<{item_cs}>"),
			};
			let _ = write!(
				stream,
				"\t/// Each item `{next_name}` yields, one call per item, until it yields none or throws.\n\tpublic IAsyncEnumerator<{item_cs}> GetAsyncEnumerator(CancellationToken cancellationToken = default) =>\n\t\tStreams.{each}<{item_cs}>({next_name}, cancellationToken);\n\n"
			);
		}
		ifaces = match ifaces.is_empty() {
			true => " : RustObject".to_string(),
			false => format!(" : RustObject, {}", &ifaces[3..]),
		};
		let _ = write!(
			self.types,
			"{}public sealed unsafe partial class {name}{ifaces}\n{{\n{body}{traits}{methods}{stream}}}\n\n",
			docs(&object.docs, "")
		);
	}

	/// Whether C# holds a value of `ty` as a value type, whose option is a `Nullable`.
	fn is_value(&self, ty: &Type) -> bool {
		match ty {
			Type::Object(_) | Type::List(..) | Type::Callback(..) | Type::Closure(_) | Type::Future | Type::Reader => {
				false
			}
			Type::Waker => false,
			Type::Enum(i) => self.abi.api.enums[*i].plain(),
			Type::Box(inner) | Type::Ref(_, inner) => self.is_value(inner),
			_ => true,
		}
	}

	fn callback(&mut self, i: usize) {
		let abi = self.abi;
		let callback = &abi.api.callbacks[i];
		let c_name = abi.callback(i);
		let name = self.class(&c_name);
		let mut methods = String::new();
		let refs: Vec<&Function> = callback.methods.iter().collect();
		let rust: Vec<String> = refs.iter().map(|f| f.name().to_string()).collect();
		let spelled: Vec<String> = rust.iter().map(|r| pascal(r)).collect();
		let rust_refs: Vec<&str> = rust.iter().map(String::as_str).collect();
		let names = abi::named(
			&rust_refs,
			spelled,
			|rust, name| pascal(rust) == name,
			|_, name| OBJECT_MEMBERS.contains(&name) || QUALIFIERS.contains(&name),
			abi::underscored,
		);
		for (method, mname) in callback.methods.iter().zip(&names) {
			crate::lock::function(&mut self.locked, method, mname);
			let locals = abi::distinct(&method.params, param_name, |_| false);
			let params: Vec<String> = (method.params.iter().zip(&locals))
				.map(|(p, local)| format!("{} {local}", self.lent_cs(&p.ty)))
				.collect();
			let _ = write!(
				methods,
				"{}\t{} {mname}({});\n\n",
				docs(&method.docs, "\t"),
				self.cs(&method.ok),
				params.join(", ")
			);
		}
		let _ = write!(
			self.types,
			"{}public interface {name}\n{{\n{methods}}}\n\n",
			docs(&callback.docs, "")
		);
		let _ = names;
	}

	/// Writes trait `i`'s table and its lowering, once: a table of C functions Rust calls,
	/// each a trampoline into the implementation its data holds.
	fn vtable(&mut self, i: usize) {
		let abi = self.abi;
		let c_name = abi.callback(i);
		if !self.vtables.insert(c_name.clone()) {
			return;
		}
		self.c_struct(&Ty::Struct(c_name.clone()));
		let callback = &abi.api.callbacks[i];
		let iface = self.class(&c_name);
		let rust: Vec<String> = callback.methods.iter().map(|f| f.name().to_string()).collect();
		let spelled: Vec<String> = rust.iter().map(|r| pascal(r)).collect();
		let rust_refs: Vec<&str> = rust.iter().map(String::as_str).collect();
		let names = abi::named(
			&rust_refs,
			spelled,
			|rust, name| pascal(rust) == name,
			|_, name| OBJECT_MEMBERS.contains(&name) || QUALIFIERS.contains(&name),
			abi::underscored,
		);
		let mut slots = vec!["Callbacks.Release".to_string()];
		for (method, mname) in callback.methods.iter().zip(&names) {
			let ps: Vec<Ty> = method.params.iter().map(|p| abi.ty(&p.ty)).collect();
			let ret = abi.ty(&method.ok);
			let field = self.fresh("Trait");
			let mut raw_params = vec!["void* data".to_string()];
			let mut lifted = Vec::new();
			for (k, (p, c)) in method.params.iter().zip(&ps).enumerate() {
				raw_params.push(format!("{} p{k}", self.raw(c)));
				lifted.push(self.lift_lent(&p.ty, &format!("p{k}")));
			}
			let call = format!("Native.Held<{iface}>(data).{mname}({})", lifted.join(", "));
			let raw_ret = self.raw(&ret);
			let body = match method.ok {
				Type::Unit => format!("{call};"),
				ref ok => {
					let lowered = self.returned(ok, "r_");
					format!("var r_ = {call};\n\t\t\treturn {lowered};")
				}
			};
			let what = format!("{}::{}", callback.path.join("::"), method.name());
			self.trampoline(&field, &raw_params, &raw_ret, &body, &what);
			slots.push(format!("Callbacks.{field}"));
		}
		let table = format!("{c_name}_table_");
		let _ = write!(
			self.natives,
			"\t/// `{iface}`'s table of C functions, which every implementation shares.\n\tstatic readonly IntPtr {table} = Table_{c_name}();\n\n\tstatic IntPtr Table_{c_name}()\n\t{{\n\t\tvar t = (IntPtr*)Marshal.AllocHGlobal({n} * sizeof(IntPtr));\n{}\n\t\treturn (IntPtr)t;\n\t}}\n\n\tinternal static {c_name} Lower_{c_name}({iface} v) => new {c_name} {{ vtable = {table}, data = Give(v ?? throw new ArgumentNullException(nameof(v))) }};\n\n",
			slots
				.iter()
				.enumerate()
				.map(|(k, s)| format!("\t\tt[{k}] = {s};"))
				.collect::<Vec<_>>()
				.join("\n"),
			n = slots.len()
		);
	}

	/// The C value a closure or trait method returns Rust, which Rust owns: text and bytes in
	/// a buffer `buf_copy` made, anything else as it lowers, with no arena.
	fn returned(&mut self, ty: &Type, e: &str) -> String {
		match ty {
			Type::Str | Type::String => format!("Native.Owned({e}.Utf8.Span)"),
			Type::PathBuf | Type::Path => format!("Native.Owned({e}.Bytes.Span)"),
			Type::Vec | Type::Bytes | Type::Slice => format!("Native.OwnedBytes({e})"),
			Type::Option(inner) if Abi::is_buf(inner) => {
				let v = self.fresh("v");
				let some = self.returned(inner, &v);
				format!("({e} is {{}} {v} ? {some} : default)")
			}
			// What else a closure or callback returns, a scalar or an opaque value, needs no arena.
			ty => {
				assert!(!self.arena(ty), "rust2-ffi lets no closure return {ty:?}");
				self.lower(ty, e)
			}
		}
	}

	/// Writes a trampoline: a static method Rust calls through `Callbacks.<field>`, which
	/// aborts on any exception, as a callback has no caller to throw to.
	fn trampoline(&mut self, field: &str, params: &[String], ret: &str, body: &str, what: &str) {
		let types: Vec<String> = params
			.iter()
			.map(|p| p.rsplit_once(' ').expect("typed").0.to_string())
			.collect();
		let mut sig = types.clone();
		sig.push(ret.to_string());
		let fallback = match ret {
			"void" => String::new(),
			_ => "\n\t\t\treturn default;".to_string(),
		};
		let what = what.replace('"', "\\\"");
		let _ = write!(
			self.callbacks,
			"#if UNITY_5_3_OR_NEWER\n\t[UnmanagedFunctionPointer(CallingConvention.Cdecl)]\n\tinternal delegate {ret} {field}Fn({});\n\tstatic readonly {field}Fn {field}Keep = {field}Impl;\n\tinternal static readonly IntPtr {field} = Marshal.GetFunctionPointerForDelegate({field}Keep);\n\n\t[AOT.MonoPInvokeCallback(typeof({field}Fn))]\n#else\n\tinternal static readonly IntPtr {field} = (IntPtr)(delegate* unmanaged[Cdecl]<{}>)&{field}Impl;\n\n\t[UnmanagedCallersOnly(CallConvs = new[] {{ typeof(CallConvCdecl) }})]\n#endif\n\tstatic {ret} {field}Impl({})\n\t{{\n\t\ttry\n\t\t{{\n\t\t\t{body}\n\t\t}}\n\t\tcatch (Exception e)\n\t\t{{\n\t\t\tNative.Abort(\"{what}: a call into C# threw\", e);{fallback}\n\t\t}}\n\t}}\n\n",
			params.join(", "),
			sig.join(", "),
			params.join(", ")
		);
	}

	/// The trampoline of a closure type, once per signature: it lends each object for the
	/// call, and hands Rust what the closure returns, or the error it throws.
	fn closure_trampoline(&mut self, closure: &Closure) -> String {
		let abi = self.abi;
		let key = format!("{closure:?}");
		if let Some(field) = self.trampolines.get(&key) {
			return field.clone();
		}
		let field = self.fresh("Closure");
		self.trampolines.insert(key, field.clone());
		let delegate = self.delegate(closure);
		let Ty::Fn(c_params, c_ret) = abi.closure(closure) else {
			unreachable!("a closure is a C function")
		};
		let mut raw_params = vec!["void* data".to_string()];
		let mut args = Vec::new();
		let mut pre = Vec::new();
		let mut ends = Vec::new();
		for (k, ty) in closure.params.iter().enumerate() {
			raw_params.push(format!("{} p{k}", self.raw(&c_params[k])));
			match ty {
				Type::Ref(_, inner) if matches!(**inner, Type::Object(_)) => {
					pre.push(format!("var l{k} = new {}(p{k}, true);", self.cs(inner)));
					ends.push(format!("l{k}.end();"));
					args.push(format!("l{k}"));
				}
				ty => args.push(self.lift_lent(ty, &format!("p{k}"))),
			}
		}
		let call = format!("Native.Held<{delegate}>(data)({})", args.join(", "));
		let mut body = pre;
		let ret = self.raw(&c_ret);
		match &closure.err {
			Some(err) => {
				let mut n = closure.params.len();
				let ok_out = match closure.ok {
					Type::Unit => None,
					_ => {
						raw_params.push(format!("{} ok_", self.raw(&c_params[n])));
						n += 1;
						Some("ok_")
					}
				};
				raw_params.push(format!("{} err_", self.raw(&c_params[n])));
				let exception = self.exception(err);
				let ok_set = match ok_out {
					Some(out) => {
						let lowered = self.returned(&closure.ok, "r_");
						format!("var r_ = {call};\n\t\t\t\t*{out} = {lowered};")
					}
					None => format!("{call};"),
				};
				let err_value = self.returned(err, "e.Error");
				body.push(format!(
					"try\n\t\t\t{{\n\t\t\t\t{ok_set}\n\t\t\t\treturn 1;\n\t\t\t}}\n\t\t\tcatch ({exception} e)\n\t\t\t{{\n\t\t\t\t*err_ = {err_value};\n\t\t\t\treturn 0;\n\t\t\t}}\n\t\t\tfinally\n\t\t\t{{\n\t\t\t\t{}\n\t\t\t}}",
					ends.join(" ")
				));
			}
			None => match &closure.ok {
				Type::Unit => {
					body.push(format!(
						"try\n\t\t\t{{\n\t\t\t\t{call};\n\t\t\t}}\n\t\t\tfinally\n\t\t\t{{\n\t\t\t\t{}\n\t\t\t}}",
						ends.join(" ")
					));
				}
				ok => {
					let lowered = self.returned(ok, "r_");
					body.push(format!(
						"try\n\t\t\t{{\n\t\t\t\tvar r_ = {call};\n\t\t\t\treturn {lowered};\n\t\t\t}}\n\t\t\tfinally\n\t\t\t{{\n\t\t\t\t{}\n\t\t\t}}",
						ends.join(" ")
					));
				}
			},
		}
		let body = body.join("\n\t\t\t");
		self.trampoline(&field, &raw_params, &ret, &body, "a closure");
		field
	}

	// ------------------------------------------------------------------------------------
	// Functions.

	/// Whether passing `ty` at a call's top level moves an object's value out.
	fn moved(&self, ty: &Type) -> bool {
		match ty {
			Type::Object(i) => !self.abi.api.objects[*i].copy,
			Type::Option(inner) | Type::Box(inner) => self.moved(inner),
			_ => false,
		}
	}

	/// How a call passes the parameter `ty`, whose C# value is `e`, the `k`th, as the C
	/// parameter `c`.
	fn arg(&mut self, ty: &Type, c: &Ty, e: &str, k: usize) -> (Arg, bool) {
		let abi = self.abi;
		let mut arg = Arg::default();
		match (ty, c) {
			(Type::Closure(closure), Ty::Fn(..)) => {
				let field = self.closure_trampoline(closure);
				arg.exprs.push(format!("Callbacks.{field}"));
				match closure.kept {
					true => {
						let d = format!("d{k}_");
						arg.checks.push(null_check(e));
						arg.gives.push(format!("var {d} = Native.Give({e});"));
						arg.exprs.push(d);
						arg.exprs.push("Callbacks.Release".to_string());
					}
					false => {
						let g = format!("g{k}_");
						arg.opens.push((
							format!(
								"var {g} = GCHandle.Alloc({e} ?? throw new ArgumentNullException(nameof({e})));\n\t\ttry\n\t\t{{\n"
							),
							format!("\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\t{g}.Free();\n\t\t}}\n"),
						));
						arg.exprs.push(format!("(void*)GCHandle.ToIntPtr({g})"));
					}
				}
				(arg, false)
			}
			(Type::Ref(_, inner), Ty::ForeignReader) if **inner == Type::Reader => {
				let r = format!("r{k}_");
				arg.opens.push((
					format!("var {r} = Native.Reader({e});\n\t\ttry\n\t\t{{\n"),
					format!("\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\tNative.Release({r}.data);\n\t\t}}\n"),
				));
				arg.exprs.push(r);
				(arg, false)
			}
			(Type::Waker | Type::Future | Type::Callback(..), _) => {
				let give = match ty {
					Type::Waker => format!("Native.Waker({e})"),
					Type::Future => format!("Native.Future({e})"),
					Type::Callback(i, _) => {
						self.vtable(*i);
						format!("Native.Lower_{}({e})", abi.callback(*i))
					}
					_ => unreachable!(),
				};
				let d = format!("d{k}_");
				arg.checks.push(null_check(e));
				arg.gives.push(format!("var {d} = {give};"));
				arg.exprs.push(d);
				(arg, false)
			}
			// Bytes or text at the top level are a view of C#'s memory, pinned for the call.
			(ty, Ty::Buf) if Abi::is_buf(ty) => {
				let b = format!("b{k}_");
				let span = match ty.base() {
					Type::Str | Type::String => format!("{e}.Utf8.Span"),
					Type::Path | Type::PathBuf => format!("{e}.Bytes.Span"),
					_ => e.to_string(),
				};
				arg.opens
					.push((format!("fixed (byte* {b} = {span})\n\t\t{{\n"), "\t\t}\n".to_string()));
				arg.exprs.push(format!("Native.View({span}, {b})"));
				(arg, false)
			}
			// A block at the top level is a view of C#'s array, pinned for the call.
			(ty, Ty::Struct(list)) if ty.base().block().is_some() && !matches!(ty, Type::Option(_)) => {
				let Type::List(_, elem) = ty.base() else { unreachable!() };
				let raw = self.raw_of(elem);
				let b = format!("b{k}_");
				arg.opens
					.push((format!("fixed ({raw}* {b} = {e})\n\t\t{{\n"), "\t\t}\n".to_string()));
				self.c_struct(&Ty::Struct(list.clone()));
				arg.exprs
					.push(format!("new Native.{list} {{ ptr = {b}, len = (nuint){e}.Length }}"));
				(arg, false)
			}
			// An object at the top level is held for the call, and closed once its value moved.
			(ty, Ty::Handle { .. })
				if matches!(ty.base(), Type::Object(_))
					|| matches!(ty, Type::Option(inner) if matches!(inner.base(), Type::Object(_))) =>
			{
				let h = format!("h{k}_");
				let optional = matches!(ty, Type::Option(_));
				match optional {
					true => {
						arg.opens.push((
							format!(
								"var o{k}_ = -1;\n\t\tvar {h} = {e} != null ? {e}.enter(out o{k}_) : IntPtr.Zero;\n\t\ttry\n\t\t{{\n"
							),
							format!("\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\t{e}?.exit(o{k}_);\n\t\t}}\n"),
						));
					}
					false => {
						arg.opens.push((
							format!("var {h} = {e}.enter(out var o{k}_);\n\t\ttry\n\t\t{{\n"),
							format!("\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\t{e}.exit(o{k}_);\n\t\t}}\n"),
						));
					}
				}
				arg.exprs.push(h);
				if self.moved(ty) {
					arg.after.push(match optional {
						true => format!("{e}?.moved();"),
						false => format!("{e}.moved();"),
					});
				}
				(arg, false)
			}
			(ty, _) => {
				let arena = self.arena(ty);
				let lowered = self.lower(ty, e);
				let take = !matches!(ty, Type::Ref(..));
				// A value C reads behind a pointer is a local the call points at.
				let behind = matches!(c, Ty::In(_) | Ty::InOut(_));
				let lowered = match arena || behind {
					true => {
						let v = format!("c{k}_");
						let take = match arena {
							true => format!("a_.Take = {take};\n\t\t"),
							false => String::new(),
						};
						arg.opens.push((format!("{take}var {v} = {lowered};\n"), String::new()));
						match behind {
							true => format!("&{v}"),
							false => v,
						}
					}
					false => lowered,
				};
				arg.exprs.push(lowered);
				(arg, arena)
			}
		}
	}

	/// `function`'s C# member, named `name`, a member of `owner` if it has one.
	fn function(&mut self, function: &Function, owner: Option<&Owner>, name: &str) -> String {
		let member = self.member(function, owner, name);
		let property = function.access == Access::Const && function.params.is_empty() && !function.future;
		match property {
			true => member.property(Some(&member.body), None),
			false => member.method(),
		}
	}

	/// Whether `function`, of `owner`, is a field's getter or setter C# writes as a property.
	fn accessor(&self, function: &Function, owner: Option<&Owner>) -> bool {
		let Access::Field { set, .. } = function.access else {
			return false;
		};
		let Some(owner) = owner else { return false };
		let value = matches!(owner.c, Ty::Struct(_));
		!function.future
			&& !owner.extension
			&& match set {
				true => function.params.len() == 1,
				false => function.params.is_empty() && !(value && function.receiver == Some(Receiver::Mut)),
			}
	}

	/// `function`'s C# member, named `name`, a member of `owner` if it has one: its parts.
	fn member(&mut self, function: &Function, owner: Option<&Owner>, name: &str) -> Member {
		let abi = self.abi;
		let mut export = abi.export(function, owner.map(|o| &o.c));
		// A future that is not `Send` made from none lives on a home, so a continuation on any
		// thread polls it.
		if function.future && export.homed {
			export = export.on_home();
		}
		crate::lock::function(&mut self.locked, function, name);
		let c_name = export.call.name.clone();
		self.declare(&export.call);
		if let Some(future) = &export.future {
			self.declare(&future.start.clone());
			self.declare(&future.poll.clone());
			self.declare(&future.free.clone());
		}
		for (_, ty) in &export.call.params {
			self.c_struct(ty);
		}
		let ret_ty = export.call.ret.clone();
		self.c_struct(&ret_ty);

		let receiver = function.receiver.filter(|_| owner.is_some());
		let is_static = receiver.is_none() && owner.is_some_and(|o| !o.extension) || owner.is_none();
		let mut pre: Vec<String> = Vec::new();
		let everywhere = abi.api.everywhere(function.on);
		if !everywhere {
			pre.push(format!("Native.On({}, \"{c_name}\");", function.on.0 & abi.api.all().0));
		}
		let mut arena = false;
		let mut args: Vec<Arg> = Vec::new();
		let mut c_params = export.call.params.iter();
		let mut post: Vec<String> = Vec::new();
		// A data enum is a class, whose `this` no method assigns: one Rust replaces through
		// `&mut self` is static, taking the caller's variable by `ref`.
		let replaced = receiver == Some(Receiver::Mut)
			&& owner.is_some_and(|o| matches!(o.ty, Type::Enum(i) if !abi.api.enums[i].plain()))
			&& matches!(export.call.params.first(), Some((_, Ty::InOut(_))));
		let is_static = is_static || replaced;
		let this = match owner {
			Some(o) if o.extension => "self",
			_ if replaced => "self",
			_ => "this",
		};
		if let (Some(receiver), Some(owner)) = (receiver, owner) {
			let (_, c) = c_params.next().expect("a method has its receiver");
			let mut arg = Arg::default();
			match c {
				Ty::Handle { .. } => {
					arg.opens.push((
						"var h_ = enter(out var o_);\n\t\ttry\n\t\t{\n".to_string(),
						"\t\t}\n\t\tfinally\n\t\t{\n\t\t\texit(o_);\n\t\t}\n".to_string(),
					));
					arg.exprs.push("h_".to_string());
					let copy = matches!(owner.ty, Type::Object(i) if abi.api.objects[i].copy);
					if receiver == Receiver::Owned && !copy {
						arg.after.push("moved();".to_string());
					}
				}
				_ => {
					arena |= self.arena(&owner.ty);
					let lowered = match &owner.ty {
						Type::Enum(i) if abi.api.enums[*i].plain() => {
							self.helper(&owner.ty);
							format!("Native.Lower_{}({this})", abi.enumeration(*i))
						}
						ty => self.lower(ty, this),
					};
					let take = match self.arena(&owner.ty) {
						true => format!("a_.Take = {};\n\t\t", receiver == Receiver::Owned),
						false => String::new(),
					};
					arg.opens
						.push((format!("{take}var c_self_ = {lowered};\n"), String::new()));
					arg.exprs.push("&c_self_".to_string());
					if matches!(c, Ty::InOut(_)) {
						let lifted = match &owner.ty {
							Type::Enum(i) if abi.api.enums[*i].plain() => format!("({})c_self_", self.cs(&owner.ty)),
							ty => self.lift(ty, "c_self_"),
						};
						post.push(format!("{this} = {lifted};"));
					}
				}
			}
			args.push(arg);
		}
		// The parameters' C# names, apart from each other and the call's own locals, which end in `_`.
		let locals = match self.accessor(function, owner) {
			// A property's setter takes its value as `value`.
			true => vec!["value".to_string(); function.params.len()],
			false => abi::distinct(&function.params, param_name, |n| {
				n == "cancellationToken" || this == "self" && n == "self"
			}),
		};
		let mut params = Vec::new();
		let mut written = Vec::new();
		for (k, (param, local)) in function.params.iter().zip(&locals).enumerate() {
			let (_, c) = c_params.next().expect("each parameter has its C one");
			if let Type::Closure(closure) = &param.ty {
				c_params.next().expect("a closure has its data");
				if closure.kept {
					c_params.next().expect("a kept closure has its free");
				}
			}
			let written_ = param.ty.written();
			let decl = match written_ {
				true => format!("ref {} {local}", self.cs(param.ty.base())),
				false => format!("{} {local}", self.param(&param.ty)),
			};
			params.push(decl);
			let (arg, needs) = match written_ {
				true => {
					let base = param.ty.base().clone();
					let needs = self.arena(&base);
					let lowered = self.lower(&base, local);
					let mut arg = Arg::default();
					let v = format!("c{k}_");
					let take = match needs {
						true => "a_.Take = true;\n\t\t",
						false => "",
					};
					arg.opens.push((format!("{take}var {v} = {lowered};\n"), String::new()));
					arg.exprs.push(v);
					(arg, needs)
				}
				// A property's `value` is of its own type, `Bytes` for bytes, which the arena
				// holds for the call, taking no view of it.
				false
					if self.accessor(function, owner)
						&& matches!(param.ty.base(), Type::Slice | Type::Vec | Type::Bytes) =>
				{
					let mut arg = Arg::default();
					arg.exprs.push(self.lower(param.ty.base(), local));
					(arg, true)
				}
				false => self.arg(&param.ty, c, local, k),
			};
			arena |= needs;
			args.push(arg);
			if written_ {
				written.push((param.clone(), local.clone()));
			}
		}
		let outs: Vec<(String, Ty)> = c_params.filter(|(_, ty)| !abi::is_status(ty)).cloned().collect();
		let status = export.call.status().is_some();
		let mut decls: Vec<String> = Vec::new();
		if status {
			decls.push("int s_ = 0;".to_string());
		}
		// Each value Rust writes back for a `&mut`, once the call ran.
		let mut writebacks = Vec::new();
		let mut written_outs = Vec::new();
		for (param, local) in &written {
			let out = Abi::written(function, &abi::escape(&param.name));
			let (_, ty) = outs
				.iter()
				.find(|(n, _)| *n == out)
				.expect("a written parameter has its out");
			decls.push(self.out_decl(&out, ty));
			let lifted = self.lift(param.ty.base(), &format!("o_{out}"));
			writebacks.push(format!("{local} = {lifted};"));
			written_outs.push(out);
		}
		let exprs: Vec<String> = args.iter().flat_map(|a| a.exprs.clone()).collect();
		let afters: Vec<String> = args.iter().flat_map(|a| a.after.clone()).collect();
		let ok = crate::kotlin::strip_refs(&function.ok);
		let err = function.err.as_ref().map(crate::kotlin::strip_refs);
		let exception = err.as_ref().map(|e| self.exception(e));

		// The call itself, its result, and what follows it.
		let mut inner: Vec<String> = Vec::new();
		let mut result: Vec<String> = Vec::new();
		let ret_cs;
		if let Some(future) = &export.future {
			let poll_status = future.poll.status().is_some();
			let n_outs = future.poll.params.len() - 2 - usize::from(poll_status);
			let poll_outs: Vec<(String, Ty)> = future.poll.params[2..2 + n_outs].to_vec();
			for (n, ty) in &poll_outs {
				decls.push(self.out_decl(n, ty));
			}
			decls.push("IntPtr f_ = IntPtr.Zero;".to_string());
			decls.push("int p_;".to_string());
			let mut start_args = exprs.clone();
			start_args.extend(poll_outs.iter().map(|(n, _)| format!("&o_{n}")));
			start_args.push("&f_".to_string());
			if status {
				start_args.push("&s_".to_string());
			}
			inner.push(format!("p_ = Native.{}({});", future.start.name, start_args.join(", ")));
			let output = match ok {
				Type::Unit => None,
				ref ok => Some(self.cs(ok)),
			};
			let ready = match &ok {
				Type::Unit => String::new(),
				ok => self.lift_outs(&function.ok, ok),
			};
			let class = self.future_class(&export, function, &ok, err.as_ref(), exception.as_deref(), &poll_outs);
			let error = match (&err, &exception) {
				(Some(err), Some(exception)) => {
					let lifted = self.lift(err, "o_err");
					format!("new {exception}({lifted})")
				}
				_ => "new InvalidOperationException()".to_string(),
			};
			match &output {
				Some(output) => {
					ret_cs = format!("ValueTask<{output}>");
					result.push(format!(
						"if (p_ == Native.PollReady)\n\t\t{{\n\t\t\treturn new ValueTask<{output}>({ready});\n\t\t}}"
					));
					if err.is_some() {
						result.push(format!(
							"if (p_ == Native.PollError)\n\t\t{{\n\t\t\treturn new ValueTask<{output}>(Task.FromException<{output}>({error}));\n\t\t}}"
						));
					}
					result.push(format!("return new Native.{class}().Start(f_, cancellationToken);"));
				}
				None => {
					ret_cs = "ValueTask".to_string();
					result.push("if (p_ == Native.PollReady)\n\t\t{\n\t\t\treturn default;\n\t\t}".to_string());
					if err.is_some() {
						result.push(format!(
							"if (p_ == Native.PollError)\n\t\t{{\n\t\t\treturn new ValueTask(Task.FromException({error}));\n\t\t}}"
						));
					}
					result.push(format!("return new Native.{class}().StartVoid(f_, cancellationToken);"));
				}
			}
			params.push("CancellationToken cancellationToken = default".to_string());
		} else {
			for (n, ty) in outs.iter().filter(|(n, _)| !written_outs.contains(n)) {
				decls.push(self.out_decl(n, ty));
			}
			let mut call_args = exprs.clone();
			call_args.extend(outs.iter().map(|(n, _)| format!("&o_{n}")));
			if status {
				call_args.push("&s_".to_string());
			}
			let call = format!("Native.{c_name}({})", call_args.join(", "));
			let has_outs = outs.iter().any(|(n, _)| !written_outs.contains(n));
			ret_cs = match ok {
				Type::Unit => "void".to_string(),
				ref ok => self.cs(ok),
			};
			match (&err, &ok) {
				(Some(err), ok) => {
					decls.push("byte r_;".to_string());
					inner.push(format!("r_ = {call};"));
					let lifted = self.lift(err, "o_err");
					result.push(format!(
						"if (r_ == 0)\n\t\t{{\n\t\t\tthrow new {}({lifted});\n\t\t}}",
						exception.clone().expect("an error throws")
					));
					if *ok != Type::Unit {
						result.push(format!("return {};", self.lift_outs(&function.ok, ok)));
					}
				}
				(None, Type::Unit) => inner.push(format!("{call};")),
				(None, ok) if has_outs => {
					inner.push(format!("{call};"));
					result.push(format!("return {};", self.lift_outs(&function.ok, ok)));
				}
				(None, ok) => {
					let raw = self.raw(&ret_ty);
					decls.push(format!("{raw} r_;"));
					inner.push(format!("r_ = {call};"));
					result.push(format!("return {};", self.lift(ok, "r_")));
				}
			}
		}
		// Whether the call took what it moves, before a write back that may throw, so the
		// arena closes what Rust took either way.
		if arena && status {
			inner.push("a_.Ran = s_ == 0;".to_string());
		} else if arena {
			inner.push("a_.Ran = true;".to_string());
		}
		// What Rust wrote back through a value's `&mut self`, while its C copy is in scope.
		for line in &post {
			inner.push(match status {
				true => format!("if (s_ == 0)\n\t\t{{\n\t\t\t{line}\n\t\t}}"),
				false => line.clone(),
			});
		}

		// Nest the call in each argument's opens, the first outermost, with the data it gives
		// Rust made last, after every open that may throw.
		let gives = args.iter().flat_map(|a| a.gives.iter());
		let mut code = gives.chain(&inner).map(|l| format!("\t\t{l}\n")).collect::<String>();
		for arg in args.iter().rev() {
			for (open, close) in arg.opens.iter().rev() {
				code = format!("\t\t{open}{code}{close}");
			}
		}
		if arena {
			code = format!(
				"\t\tvar a_ = new Arena();\n\t\ttry\n\t\t{{\n{code}\t\t}}\n\t\tfinally\n\t\t{{\n\t\t\ta_.Dispose();\n\t\t}}\n"
			);
		}
		let mut body = String::new();
		let checks = args.iter().flat_map(|a| a.checks.iter());
		for line in checks.chain(&pre).chain(&decls) {
			let _ = writeln!(body, "\t\t{line}");
		}
		body.push_str(&code);
		if status {
			let _ = writeln!(body, "\t\tNative.Check(s_, \"{c_name}\");");
		}
		for line in afters.iter().chain(&writebacks) {
			let _ = writeln!(body, "\t\t{line}");
		}
		for line in &result {
			let _ = writeln!(body, "\t\t{line}");
		}
		let modifiers = match (owner, is_static) {
			(Some(o), _) if o.extension => "public static ",
			(_, true) => "public static ",
			(_, false) => "public ",
		};
		let mut all_params = params;
		if let Some(o) = owner
			&& o.extension
			&& receiver.is_some()
		{
			let r = match receiver {
				Some(Receiver::Mut) => "ref ",
				_ => "",
			};
			all_params.insert(0, format!("this {r}{} self", self.cs(&o.ty)));
		}
		if let Some(o) = owner
			&& replaced
		{
			all_params.insert(0, format!("ref {} self", self.cs(&o.ty)));
		}
		let mut text = crate::doc::sections(&function.docs).body;
		if function.future {
			text.push_str("\n\nCancelling the token frees the future, which cancels it in Rust and loses nothing.");
		}
		Member {
			docs: docs(&text, "\t"),
			modifiers: modifiers.to_string(),
			ret: ret_cs,
			name: name.to_string(),
			params: all_params,
			body,
		}
	}

	/// The declaration of the local a call writes its out value `name`, of C type `ty`, to.
	fn out_decl(&mut self, name: &str, ty: &Ty) -> String {
		let Ty::Out(inner) = ty else {
			unreachable!("an out parameter is a pointer")
		};
		self.c_struct(inner);
		format!("{} o_{} = default;", self.raw(inner), name)
	}

	/// The C# value of a call's outs, which return `ok`, or each element of a tuple.
	fn lift_outs(&mut self, declared: &Type, ok: &Type) -> String {
		match declared {
			Type::Tuple(elements) if elements.len() == 1 => {
				let lifted = self.lift(&crate::kotlin::strip_refs(&elements[0]), "o_out0");
				format!("new {}({lifted})", self.cs(ok))
			}
			Type::Tuple(elements) => {
				let lifted: Vec<String> = (elements.iter().enumerate())
					.map(|(n, e)| self.lift(&crate::kotlin::strip_refs(e), &format!("o_out{n}")))
					.collect();
				format!("({})", lifted.join(", "))
			}
			_ => self.lift(ok, "o_out"),
		}
	}

	/// The class a pending future of `function` completes from, once.
	fn future_class(
		&mut self,
		export: &abi::Export,
		function: &Function,
		ok: &Type,
		err: Option<&Type>,
		exception: Option<&str>,
		outs: &[(String, Ty)],
	) -> String {
		let future = export.future.as_ref().expect("a future");
		let class = format!("Future_{}", export.call.name);
		if !self.futures.insert(class.clone()) {
			return class;
		}
		let output = match ok {
			Type::Unit => "byte".to_string(),
			ok => self.cs(ok),
		};
		let poll_status = future.poll.status().is_some();
		let mut decls = Vec::new();
		for (n, ty) in outs {
			decls.push(self.out_decl(n, ty));
		}
		let mut args = vec!["f".to_string(), "w".to_string()];
		args.extend(outs.iter().map(|(n, _)| format!("&o_{n}")));
		if poll_status {
			decls.push("int s_ = 0;".to_string());
			args.push("&s_".to_string());
		}
		let check = match poll_status {
			true => format!("\n\t\t\tNative.Check(s_, \"{}\");", future.poll.name),
			false => String::new(),
		};
		let ready = match ok {
			Type::Unit => "0".to_string(),
			ok => self.lift_outs(&function.ok, ok),
		};
		let error = match (err, exception) {
			(Some(err), Some(exception)) => {
				let lifted = self.lift(err, "o_err");
				format!("\n\t\t\tif (p == PollError)\n\t\t\t{{\n\t\t\t\terror = new {exception}({lifted});\n\t\t\t}}")
			}
			_ => String::new(),
		};
		let _ = write!(
			self.natives,
			"\tinternal sealed class {class} : Pending<{output}>\n\t{{\n\t\tprotected override int Poll(IntPtr f, CWaker w, out {output} value, out Exception? error)\n\t\t{{\n{}\t\t\tvar p = {}({});{check}\n\t\t\tvalue = default!;\n\t\t\terror = null;\n\t\t\tif (p == PollReady)\n\t\t\t{{\n\t\t\t\tvalue = {ready};\n\t\t\t}}{error}\n\t\t\treturn p;\n\t\t}}\n\n\t\tprotected override void Free(IntPtr f) => {}(f);\n\t}}\n\n",
			decls.iter().map(|d| format!("\t\t\t{d}\n")).collect::<String>(),
			future.poll.name,
			args.join(", "),
			future.free.name,
		);
		class
	}
}

/// A member's parts, which a method or a property's accessors write.
#[derive(Clone)]
struct Member {
	docs: String,
	modifiers: String,
	ret: String,
	name: String,
	params: Vec<String>,
	body: String,
}

impl Member {
	fn method(&self) -> String {
		format!(
			"{}\t{}{} {}({})\n\t{{\n{}\t}}\n\n",
			self.docs,
			self.modifiers,
			self.ret,
			self.name,
			self.params.join(", "),
			self.body
		)
	}

	/// A property of a getter's body, a setter's, or both.
	fn property(&self, get: Option<&str>, set: Option<&str>) -> String {
		let ret = match get {
			Some(_) => &self.ret,
			// A setter's own return is `void`: the property's type is its value's.
			None => self.params[0].rsplit_once(' ').map_or("", |(ty, _)| ty),
		};
		let mut out = format!("{}\t{}{ret} {}\n\t{{\n", self.docs, self.modifiers, self.name);
		for (accessor, body) in [("get", get), ("set", set)] {
			if let Some(body) = body {
				let _ = write!(out, "\t\t{accessor}\n\t\t{{\n{body}\t\t}}\n");
			}
		}
		out.push_str("\t}\n\n");
		out
	}
}

/// C#'s integer type of `int`.
fn cs_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "byte",
		Int::U16 => "ushort",
		Int::U32 => "uint",
		Int::U64 => "ulong",
		Int::Usize => "nuint",
		Int::I8 => "sbyte",
		Int::I16 => "short",
		Int::I32 => "int",
		Int::I64 => "long",
		Int::Isize => "nint",
	}
}

fn nonzero_name(int: Int) -> String {
	let suffix = match int {
		Int::U8 => "U8",
		Int::U16 => "U16",
		Int::U32 => "U32",
		Int::U64 => "U64",
		Int::Usize => "Usize",
		Int::I8 => "I8",
		Int::I16 => "I16",
		Int::I32 => "I32",
		Int::I64 => "I64",
		Int::Isize => "Isize",
	};
	format!("NonZero{suffix}")
}

/// An integer constant as C# spells it.
fn int_literal(int: Int, v: i128) -> String {
	match int {
		Int::U64 | Int::Usize if v > i64::MAX as i128 => format!("{v}UL"),
		Int::U32 | Int::U64 | Int::Usize => format!("{v}U"),
		Int::I64 if v == i64::MIN as i128 => "long.MinValue".to_string(),
		Int::I32 if v == i32::MIN as i128 => "int.MinValue".to_string(),
		_ => v.to_string(),
	}
}

/// A C name as a C# identifier: a keyword takes `@`.
fn field_name(name: &str) -> String {
	match KEYWORDS.contains(&name) {
		true => format!("@{name}"),
		false => name.to_string(),
	}
}

/// A parameter's C# name: camel case, a keyword with `@`.
fn param_name(name: &str) -> String {
	let pascal = pascal(name);
	let mut chars = pascal.chars();
	let camel = match chars.next() {
		Some(first) => first.to_lowercase().chain(chars).collect(),
		None => name.to_string(),
	};
	let camel = match camel.parse::<usize>() {
		Ok(n) => format!("_{n}"),
		Err(_) => camel,
	};
	field_name(&camel)
}

/// A constructor's parameters' names, one per field of Rust name in `fields`, kept apart.
fn ctor_params<'f>(fields: impl Iterator<Item = &'f str>) -> Vec<String> {
	let rust: Vec<&str> = fields.collect();
	let spelled = rust.iter().map(|r| param_name(r)).collect();
	abi::named(
		&rust,
		spelled,
		|rust, name| param_name(rust) == name,
		|_, _| false,
		abi::numbered,
	)
}

/// A statement throwing `ArgumentNullException` where `e` is null.
fn null_check(e: &str) -> String {
	format!("if ({e} == null)\n\t\t{{\n\t\t\tthrow new ArgumentNullException(nameof({e}));\n\t\t}}")
}

/// A field's or member's C# name: Pascal case.
fn member_name(name: &str) -> String {
	match name.parse::<usize>() {
		Ok(n) => format!("_{n}"),
		Err(_) => pascal(name),
	}
}

/// Doc comments of `text`, indented by `indent`.
fn docs(text: &str, indent: &str) -> String {
	let text = text.trim();
	if text.is_empty() {
		return String::new();
	}
	let escaped = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
	let mut out = format!("{indent}/// <summary>\n");
	for line in escaped.lines() {
		let line = line.trim_end();
		match line.is_empty() {
			true => {
				let _ = writeln!(out, "{indent}/// <para/>");
			}
			false => {
				let _ = writeln!(out, "{indent}/// {line}");
			}
		}
	}
	let _ = writeln!(out, "{indent}/// </summary>");
	out
}
