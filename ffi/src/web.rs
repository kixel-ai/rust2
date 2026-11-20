//! TypeScript over the C ABI of a `wasm32-wasip1` build, for the web: `web/`, ES modules a
//! bundler takes as they are, over the `.wasm` that `cargo build --target wasm32-wasip1
//! --features web` makes of the crate, with no tool run on it after.
//!
//! The binding's API is rust2-ts's: each type is a module of its own, `web/<Type>.ts`,
//! exporting the type by its name and its functions, each taking `self` first, so a caller
//! writes `import * as Counter from "./web/Counter.ts"` and `Counter.get(c)`. `web/index.ts`
//! holds the free functions, the constants, and `load`, which instantiates the module before
//! any call. A record is a plain object, an enum without data a union of its variants' names,
//! an enum a union of objects tagged by variant, an object a handle disposed by
//! `[Symbol.dispose]` or freed by the garbage collector, a trait an interface JavaScript
//! implements, a `Result` an `rt.Result`, and an async function's value a promise that
//! cancels. Names are each item's C name without the prefix: a type's in Pascal case, and a
//! function's in camel case.
//!
//! Each call reads and writes linear memory at the layouts C's wasm32 ABI gives the C
//! header's structs, which [`Gen::layout`] computes, and passes each by that ABI: a struct
//! of one scalar as the scalar, any other through a pointer to a copy, and one returned
//! through a pointer the caller passes first. What JavaScript implements, a trait, a closure,
//! a waker, a future, or a reader, is a C function the module imports from `rust2`, whose
//! data is a token the binding finds its value by. The runtime they share is
//! [`runtime/rt.ts`](../runtime/rt.ts), and the module's std imports only web APIs: its
//! clock is `performance.now`, its randomness `crypto.getRandomValues`, and its output the
//! console.
//!
//! Beside each module it writes the module's JavaScript and declarations, which
//! [`crate::js`] makes, and a `package.json`, so `web/` is an npm package whose `load`
//! defaults to the `.wasm` beside it.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write;

use crate::abi::{self, Abi, Ty, pascal};
use crate::config::Package;
use crate::doc;
use crate::glue::rs;
use crate::ir::{Access, Closure, Family, Function, Int, Literal, Type};
use crate::kotlin::strip_refs;

const RUNTIME: &str = include_str!("../runtime/rt.ts");

/// The Rust runtime the crate's `web` feature builds.
pub(crate) const GLUE_RUNTIME: &str = include_str!("../runtime/web.rs");

/// The target the binding serves: wasm32, against the std that imports its platform, which
/// the binding implements with web APIs.
pub(crate) const TARGET: &str = "wasm32-wasip1";

/// The WASI functions the runtime implements with web APIs: each method of its `wasi`
/// object, so the export refuses what `load` would by the same list.
pub fn wasi() -> &'static [&'static str] {
	static WASI: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();
	WASI.get_or_init(|| {
		let start = RUNTIME
			.find("\nconst wasi = {\n")
			.expect("rt.ts declares its WASI functions");
		let body = &RUNTIME[start..];
		let body = &body[..body.find("\n};\n").expect("rt.ts closes its WASI functions")];
		// A method at the object's own level: one tab, its name, then its parameters.
		(body.lines())
			.filter_map(|line| line.strip_prefix('\t'))
			.filter_map(|line| line.split_once('(').map(|(name, _)| name))
			.filter(|name| !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
			.collect()
	})
}

/// What the generator writes: each TypeScript file by its name under `web/`, and the Rust
/// glue the crate's `web` feature builds.
pub(crate) struct Output {
	pub(crate) names: BTreeMap<String, String>,
	pub(crate) files: Vec<(String, String)>,
	pub(crate) glue: String,
}

/// Names a type may not take: a file beside it, case aside, as a case-insensitive file
/// system sees them, and the globals the generated modules name.
const TAKEN: &[&str] = &[
	"Index",
	"Rt",
	"Glue",
	"Array",
	"ArrayBuffer",
	"BigInt",
	"BigInt64Array",
	"BigUint64Array",
	"Boolean",
	"DataView",
	"Error",
	"Float32Array",
	"Float64Array",
	"Function",
	"Int8Array",
	"Int16Array",
	"Int32Array",
	"Map",
	"Math",
	"Number",
	"Object",
	"Promise",
	"RangeError",
	"Record",
	"Set",
	"String",
	"Symbol",
	"Uint8Array",
	"Uint16Array",
	"Uint32Array",
	"WebAssembly",
];

/// Words no parameter may be, which take a trailing `_`: TypeScript's reserved words and
/// the modules every generated file imports.
const RESERVED: &[&str] = &[
	"break",
	"case",
	"catch",
	"class",
	"const",
	"continue",
	"debugger",
	"default",
	"delete",
	"do",
	"else",
	"enum",
	"export",
	"extends",
	"false",
	"finally",
	"for",
	"function",
	"if",
	"import",
	"in",
	"instanceof",
	"new",
	"null",
	"return",
	"super",
	"switch",
	"this",
	"throw",
	"true",
	"try",
	"typeof",
	"var",
	"void",
	"while",
	"with",
	"let",
	"static",
	"yield",
	"await",
	"implements",
	"interface",
	"package",
	"private",
	"protected",
	"public",
	"arguments",
	"eval",
	"undefined",
	"rt",
	"g",
];

/// How a C value crosses a wasm call: as one scalar, or through a pointer to its memory.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Wasm {
	I32,
	I64,
	F32,
	F64,
}

/// A C type's size and alignment on wasm32.
#[derive(Clone, Copy, Debug)]
struct Layout {
	size: u32,
	align: u32,
}

fn round(n: u32, align: u32) -> u32 {
	n.div_ceil(align) * align
}

/// The TypeScript for `abi`'s items on [`TARGET`], whose header's contract is `contract`,
/// or none if the API does not serve it.
pub(crate) fn generate(abi: &Abi<'_>, facade: &Package, contract: u64) -> Result<Option<Output>, String> {
	let packages = &facade.name;
	let api = abi.api;
	let Some(web) = api.targets.iter().position(|t| t == TARGET) else {
		return Ok(None);
	};
	let ranges: BTreeSet<String> = (abi.ranges.keys())
		.map(|c| pascal(&c[abi.prefix.len() + 1..]))
		.collect();
	let taken = |name: &str| TAKEN.contains(&name) || ranges.contains(name);
	let mut g = Gen {
		abi,
		web,
		space: abi.space("web"),
		classes: abi.classes(|_| (String::new(), 0), taken),
		module: String::new(),
		modules: BTreeMap::new(),
		helpers: BTreeMap::new(),
		readers: HashMap::new(),
		writers: HashMap::new(),
		imports: BTreeMap::new(),
		rust: String::new(),
		relays: Vec::new(),
		trampolines: Vec::new(),
		names: BTreeMap::new(),
		locked: BTreeMap::new(),
		errors: Vec::new(),
	};
	g.locked = crate::lock::types(abi, &g.classes);
	g.items();
	if !g.errors.is_empty() {
		return Err(g.errors.join("\n"));
	}
	let p = &abi.prefix;
	let space = g.space.clone();
	let mut files = Vec::new();
	files.push(("rt.ts".to_string(), RUNTIME.to_string()));
	// The glue's helpers, which the modules import whole.
	let refs = g.modules.remove("glue").map(|m| m.refs).unwrap_or_default();
	let mut body = String::new();
	for code in g.helpers.values() {
		body.push_str(code);
	}
	// What the module imports from `rust2` for what JavaScript implements.
	let imports: Vec<String> = g
		.imports
		.iter()
		.map(|(name, code)| format!("\t{name}: {code},"))
		.collect();
	let _ = write!(
		body,
		"/** What the module imports for what JavaScript implements. */\nexport const imports = {{\n{}\n}};\n",
		imports.join("\n")
	);
	let glue = format!(
		"// Generated by rust2-ffi: what the modules of the pub API of {packages} share. Do not edit.\n\n{}{}\n{body}",
		module_imports(&body),
		type_imports(&refs, "glue")
	);
	files.push(("glue.ts".to_string(), glue));
	let mut index = g.modules.remove("index").unwrap_or_default();
	let mut out = format!(
		"// Generated by rust2-ffi: TypeScript over the C ABI of the pub API of {packages}, for the web. Do not edit.\n\n\
		import * as rt from \"./rt.ts\";\n\
		import * as g from \"./glue.ts\";\n"
	);
	out.push_str(&type_imports(&index.refs, "index"));
	if !index.renames.is_empty() {
		let renames: Vec<String> = (index.renames.iter())
			.map(|(local, name)| format!("{local} as {name}"))
			.collect();
		let _ = writeln!(index.body, "export {{ {} }};", renames.join(", "));
	}
	let _ = write!(
		out,
		"\n/**\n * Compile and instantiate the module `cargo build --target {TARGET} --features web` made of\n * the crate this binding was generated with, from `source`, before any other call: its\n * bytes, a `Response` such as `fetch` gives, a compiled `WebAssembly.Module`, or its URL.\n * By default it is `{p}.wasm` beside this module, as the package holds it, whose URL Vite\n * and webpack follow into the bundle. A runtime with source phase imports passes\n * `import source module from \"<package>/{p}.wasm\"`.\n */\nexport function load(source: rt.Source = new URL(\"./{p}.wasm\", import.meta.url)): Promise<void> {{\n\treturn rt.load(source, {{ prefix: \"{p}\", space: \"{space}\", file: \"index.ts\", contract: 0x{contract:016x}n, target: {}, imports: g.imports }});\n}}\n\n{}",
		g.web, index.body
	);
	files.push(("index.ts".to_string(), trim(out)));
	for (name, module) in std::mem::take(&mut g.modules) {
		let mut out = format!(
			"// Generated by rust2-ffi: TypeScript over the C ABI of the pub API of {packages}, for the web. Do not edit.\n\n{}",
			module_imports(&module.body)
		);
		out.push_str(&type_imports(&module.refs, &name));
		out.push('\n');
		out.push_str(&module.body);
		if !module.renames.is_empty() {
			let renames: Vec<String> = (module.renames.iter())
				.map(|(local, name)| format!("{local} as {name}"))
				.collect();
			let _ = writeln!(out, "export {{ {} }};", renames.join(", "));
		}
		files.push((format!("{name}.ts"), trim(out)));
	}
	// Each link once every name is final.
	let mut namespaces = HashMap::new();
	for (name, ts) in &mut files {
		let module = name.strip_suffix(".ts").unwrap_or(name);
		// A link names its own module's item as the module does, and another module's
		// through a type-only import of that module as a namespace, `<module>$`, which reaches
		// every export whether or not the module's code names it; either shows the name a
		// caller writes.
		let mut spaces = BTreeSet::new();
		let linked = doc::links(ts, abi.api, |link| {
			let (home, local, shown) = g.link(link.target)?;
			let target = match home == module {
				true => local,
				false => {
					let target = format!("{home}$.{local}");
					spaces.insert(home);
					target
				}
			};
			Some(match link.text.unwrap_or(&shown) {
				text if text == target => format!("{{@link {target}}}"),
				text => format!("{{@link {target} | {text}}}"),
			})
		});
		let imports: String = (spaces.iter())
			.map(|space| format!("import type * as {space}$ from \"./{space}.ts\";\n"))
			.collect();
		// After the header's line.
		let at = linked.find('\n').map_or(0, |at| at + 1);
		*ts = format!("{}{imports}{}", &linked[..at], &linked[at..]);
		if !imports.is_empty() {
			namespaces.insert(name.clone(), imports.replace(".ts\";", ".js\";"));
		}
	}
	let mut emitted = Vec::new();
	for (name, ts) in &files {
		let (js, mut dts) = crate::js::emit(name, ts)?;
		// Declarations keep no import only a doc names, which the links need.
		if let Some(imports) = namespaces.get(name) {
			let at = dts.find('\n').map_or(0, |at| at + 1);
			dts.insert_str(at, imports);
		}
		let stem = name.strip_suffix(".ts").expect("each module is a .ts file");
		emitted.push((format!("{stem}.js"), js));
		emitted.push((format!("{stem}.d.ts"), dts));
	}
	files.extend(emitted);
	files.push(("package.json".to_string(), package_json(facade, p)));
	let glue = format!(
		"//! Generated by rust2-ffi: what web/index.ts calls that the C ABI lacks. Do not edit.\n\n\
		mod runtime;\n\n\
		#[allow(unused_imports, reason = \"the C ABI's structs\")]\n\
		use crate::*;\n\n\
		use std::ffi::c_void;\n\n\
		use crate::rt;\n\n\
		#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_fns() -> *const runtime::Fns {{\n\t&runtime::FNS\n}}\n\n\
		#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_alloc(len: usize, align: usize) -> *mut u8 {{\n\truntime::alloc(len, align)\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_free(ptr: *mut u8, len: usize, align: usize) {{\n\tunsafe {{ runtime::dealloc(ptr, len, align) }}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_call(f: Option<unsafe extern \"C\" fn(*mut c_void)>, data: *mut c_void) {{\n\tif let Some(f) = f {{\n\t\tunsafe {{ f(data) }}\n\t}}\n}}\n\n\
		#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {space}_abort(ptr: *const u8, len: usize) {{\n\tlet text = unsafe {{ std::slice::from_raw_parts(ptr, len) }};\n\trt::fail(\"web\", &String::from_utf8_lossy(text))\n}}\n\n{}",
		g.rust
	);
	Ok(Some(Output {
		names: g.locked,
		files,
		glue,
	}))
}

/// The npm package the binding is, named and versioned as the facade: each module by its
/// name, the runtime's types as `<package>/rt`, and the module `cargo build` makes, which
/// the package holds beside them.
fn package_json(facade: &Package, lib: &str) -> String {
	// By hand, since a condition's order is its precedence: `types` before `default`.
	let json = |s: &str| serde_json::Value::from(s).to_string();
	let module = |path: &str| format!("{{ \"types\": \"./{path}.d.ts\", \"default\": \"./{path}.js\" }}");
	format!(
		"{{\n\t\"name\": {},\n\t\"version\": {},\n\t\"description\": {},\n\t\"type\": \"module\",\n\t\"sideEffects\": false,\n\t\"exports\": {{\n\t\t\".\": {},\n\t\t\"./{lib}.wasm\": \"./{lib}.wasm\",\n\t\t\"./*\": {}\n\t}},\n\t\"files\": [\"*.js\", \"*.d.ts\", \"{lib}.wasm\"]\n}}\n",
		json(&facade.name),
		json(&facade.version),
		json(&format!(
			"The pub API of {} for the web, generated by rust2-ffi",
			facade.name
		)),
		module("index"),
		module("*"),
	)
}

/// The imports of the runtime and the glue that `code` names.
fn module_imports(code: &str) -> String {
	let mut out = String::new();
	// Each use of either is a member's, which a field or parameter of that name is not.
	if uses(code, "rt.") {
		out.push_str("import * as rt from \"./rt.ts\";\n");
	}
	if uses(code, "g.") {
		out.push_str("import * as g from \"./glue.ts\";\n");
	}
	out
}

/// Whether `code` names `name` as a whole identifier, not a property, or for a `name`
/// ending in `.`, reads a member of it.
fn uses(code: &str, name: &str) -> bool {
	let ident = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '$';
	code.match_indices(name).any(|(at, _)| {
		let before = code[..at].chars().next_back();
		let after = code[at + name.len()..].chars().next();
		let whole = name.ends_with('.') || !after.is_some_and(ident);
		!before.is_some_and(|c| ident(c) || c == '.' || c == '"') && whole
	})
}

/// The type-only imports of `refs`, by the module each type is, but `module`'s own.
fn type_imports(refs: &BTreeSet<String>, module: &str) -> String {
	(refs.iter())
		.filter(|name| name.as_str() != module)
		.map(|name| format!("import type {{ {name} }} from \"./{name}.ts\";\n"))
		.collect()
}

/// Code ending in one newline.
fn trim(mut code: String) -> String {
	while code.ends_with("\n\n") {
		code.pop();
	}
	if !code.ends_with('\n') {
		code.push('\n');
	}
	code
}

/// A module's code, the types it names of other modules, and what it exports by another
/// name than its local one, as a reserved word must.
#[derive(Default)]
struct Module {
	refs: BTreeSet<String>,
	body: String,
	/// Each export by its local name.
	renames: BTreeMap<String, String>,
	/// The names it exports.
	exports: BTreeSet<String>,
}

struct Gen<'a> {
	abi: &'a Abi<'a>,
	/// The index of [`TARGET`] among the API's targets.
	web: usize,
	/// The namespace of the glue's C names, apart from the export's.
	space: String,
	/// Each exported type's name, by its C name.
	classes: HashMap<String, String>,
	/// The module being written, whose types its code names are imported.
	module: String,
	modules: BTreeMap<String, Module>,
	/// The glue's readers, writers, trampolines, and tables, by name.
	helpers: BTreeMap<String, String>,
	/// Each reader's name, by the type it reads.
	readers: HashMap<Type, String>,
	/// Each writer's name, by the type it writes.
	writers: HashMap<Type, String>,
	/// What the module imports from `rust2`, by name: the JavaScript calling what implements it.
	imports: BTreeMap<String, String>,
	/// The Rust glue: the imports and tables of what JavaScript implements.
	rust: String,
	/// Each C signature of a closure, whose import is `fn_<n>`.
	relays: Vec<Ty>,
	/// Each closure type, whose trampoline is `t<n>`.
	trampolines: Vec<Type>,
	/// Each name `index.ts` exports, with what claimed it.
	names: BTreeMap<String, String>,
	locked: BTreeMap<String, String>,
	errors: Vec<String>,
}

impl<'a> Gen<'a> {
	/// A C name without the prefix.
	fn bare<'n>(&self, c: &'n str) -> &'n str {
		&c[self.abi.prefix.len() + 1..]
	}

	/// The TypeScript name of the C type `c`.
	/// `text`, a function's docs without its sections, then its `# Errors`, naming the
	/// error its `Err` holds, and its `# Panics` as when it aborts.
	fn function_docs(&mut self, function: &Function, text: String) -> String {
		let sections = doc::sections(&function.docs);
		let mut text = text.trim().to_string();
		if let Some(errors) = &sections.errors {
			let what = match &function.err {
				Some(err) => format!("Its `Err` holds a `{}`: ", self.tt(err)),
				None => String::new(),
			};
			let _ = write!(text, "\n\n# Errors\n\n{what}{errors}");
		}
		if let Some(panics) = &sections.panics {
			let _ = write!(text, "\n\n# Aborts\n\n{panics}");
		}
		text.trim().to_string()
	}

	/// Where `target` is, if TypeScript has it: its module, the module's name for it, and
	/// the name a caller writes, who imports a type's module as the type's name. A variant is
	/// a tag of its union, which no declaration names.
	/// None where the binding lacks it, as for an item off the web's target.
	fn link(&self, target: doc::Target<'_>) -> Option<(String, String, String)> {
		let abi = self.abi;
		let api = abi.api;
		let on = |owner: doc::Owner| match owner {
			doc::Owner::Record(i) => api.records[i].on,
			doc::Owner::Enum(i) => api.enums[i].on,
			doc::Owner::Object(i) => api.objects[i].on,
			doc::Owner::Callback(i) => api.callbacks[i].on,
		};
		let here = match target {
			doc::Target::Record(i) | doc::Target::Field(i, _) => self.here(on(doc::Owner::Record(i))),
			doc::Target::Enum(i) => self.here(on(doc::Owner::Enum(i))),
			doc::Target::Variant(i, v) => self.here(on(doc::Owner::Enum(i))) && self.here(api.enums[i].variants[v].on),
			doc::Target::Object(i) => self.here(on(doc::Owner::Object(i))),
			doc::Target::Callback(i) => self.here(on(doc::Owner::Callback(i))),
			doc::Target::Constant(c) => self.here(c.on),
			doc::Target::Function(of, f) => self.here(f.on) && of.is_none_or(|of| self.here(on(of))),
		};
		if !here {
			return None;
		}
		let owner = |owner: doc::Owner| match owner {
			doc::Owner::Record(i) => self.class(&abi.record(i)),
			doc::Owner::Enum(i) => self.class(&abi.enumeration(i)),
			doc::Owner::Object(i) => self.class(&abi.object(i)),
			doc::Owner::Callback(i) => self.class(&abi.callback(i)),
		};
		let ty = |of: doc::Owner| {
			let class = owner(of);
			(class.clone(), class.clone(), class)
		};
		Some(match target {
			doc::Target::Record(i) => ty(doc::Owner::Record(i)),
			doc::Target::Enum(i) => ty(doc::Owner::Enum(i)),
			doc::Target::Object(i) => ty(doc::Owner::Object(i)),
			doc::Target::Callback(i) => ty(doc::Owner::Callback(i)),
			doc::Target::Variant(i, v) => {
				let class = owner(doc::Owner::Enum(i));
				let tag = &abi.api.enums[i].variants[v].name;
				(class.clone(), class.clone(), format!("{class} \"{tag}\""))
			}
			doc::Target::Field(i, n) => {
				let class = owner(doc::Owner::Record(i));
				let field = format!("{class}.{}", field_names(&abi.api.records[i].fields)[n]);
				(class, field.clone(), field)
			}
			doc::Target::Constant(c) => {
				let name = self.locked.get(&format!("const {}", c.path.join("::")))?;
				("index".to_string(), name.clone(), name.clone())
			}
			doc::Target::Function(of, f) => {
				let member = self.locked.get(&crate::lock::function_key(f))?;
				match of {
					// A callback's method is its interface's member, not an export.
					Some(of @ doc::Owner::Callback(_)) => {
						let class = owner(of);
						let member = format!("{class}.{member}");
						(class, member.clone(), member)
					}
					Some(of) => {
						let class = owner(of);
						(class.clone(), member.clone(), format!("{class}.{member}"))
					}
					None => ("index".to_string(), member.clone(), member.clone()),
				}
			}
		})
	}

	fn class(&self, c: &str) -> String {
		self.classes.get(c).cloned().unwrap_or_else(|| pascal(self.bare(c)))
	}

	/// Whether an item on `on` is on the binding's target.
	fn here(&self, on: crate::ir::On) -> bool {
		on.0 >> self.web & 1 == 1
	}

	fn out(&mut self) -> &mut Module {
		self.modules.entry(self.module.clone()).or_default()
	}

	/// The type `name`, named in the module being written.
	fn named(&mut self, name: String) -> String {
		let module = self.module.clone();
		self.modules.entry(module).or_default().refs.insert(name.clone());
		name
	}

	// Types.

	/// The TypeScript type of `ty`, as a value, a parameter, or a result.
	fn tt(&mut self, ty: &Type) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Unit => "void".to_string(),
			Type::Bool => "boolean".to_string(),
			Type::Int(Int::U64) | Type::NonZero(Int::U64) => "rt.u64".to_string(),
			Type::Int(Int::I64) | Type::NonZero(Int::I64) => "rt.i64".to_string(),
			Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64 | Type::Char => "number".to_string(),
			Type::U128 => "rt.u128".to_string(),
			Type::Str | Type::String => "string".to_string(),
			Type::Slice | Type::Vec | Type::Bytes | Type::Array(_) => "Uint8Array".to_string(),
			Type::Path | Type::PathBuf => "rt.PathBuf".to_string(),
			Type::Duration => "rt.Duration".to_string(),
			Type::SocketAddr(Family::Any) => "rt.SocketAddr".to_string(),
			Type::SocketAddr(Family::V4) => "rt.SocketAddrV4".to_string(),
			Type::SocketAddr(Family::V6) => "rt.SocketAddrV6".to_string(),
			Type::IpAddr(Family::Any) => "rt.IpAddr".to_string(),
			Type::IpAddr(Family::V4) => "rt.Ipv4Addr".to_string(),
			Type::IpAddr(Family::V6) => "rt.Ipv6Addr".to_string(),
			Type::List(..) if let Some(block) = ty.block() => typed_array(block).to_string(),
			Type::List(_, inner) => {
				let inner = self.tt(inner);
				match inner.contains(' ') {
					true => format!("({inner})[]"),
					false => format!("{inner}[]"),
				}
			}
			Type::Waker => "() => void".to_string(),
			Type::Reader => "rt.Reader".to_string(),
			Type::Future => "Promise<unknown>".to_string(),
			Type::Option(inner) => {
				let none = match undefinable(inner) {
					true => "rt.None",
					false => "undefined",
				};
				format!("{} | {none}", self.tt(inner))
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.tt(inner),
			Type::Record(i) => self.named(self.class(&abi.record(*i))),
			Type::Enum(i) => self.named(self.class(&abi.enumeration(*i))),
			Type::Object(i) => self.named(self.class(&abi.object(*i))),
			Type::Callback(i, _) => self.named(self.class(&abi.callback(*i))),
			Type::Closure(closure) => {
				let params: Vec<String> = (closure.params.iter().enumerate())
					.map(|(n, p)| format!("a{n}: {}", self.tt(p)))
					.collect();
				let ok = match &closure.ok {
					Type::Unit => "void".to_string(),
					ok => self.tt(ok),
				};
				let ret = match &closure.err {
					Some(err) => format!("rt.Result<{}, {}>", self.value(&closure.ok, &ok), self.tt(err)),
					None => ok,
				};
				format!("({}) => {ret}", params.join(", "))
			}
			Type::Range(int) => {
				let int = self.tt(&Type::Int(*int));
				format!("{{ start: {int}; end: {int} }}")
			}
			Type::Bound(int) => {
				let int = self.tt(&Type::Int(*int));
				format!(
					"{{ readonly tag: \"Included\"; readonly _0: {int} }} | {{ readonly tag: \"Excluded\"; readonly _0: {int} }} | {{ readonly tag: \"Unbounded\" }}"
				)
			}
			Type::Bounds(int) => {
				let bound = self.tt(&Type::Bound(*int));
				format!("{{ start: {bound}; end: {bound} }}")
			}
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.tt(e)).collect();
				format!("[{}]", elements.join(", "))
			}
		}
	}

	/// The type of a value `tt` gave `ty` as `t`: `undefined` for `()`, which is `void` returned.
	fn value(&mut self, ty: &Type, t: &str) -> String {
		match ty {
			Type::Unit => "undefined".to_string(),
			_ => t.to_string(),
		}
	}

	// The C ABI on wasm32.

	/// The size and alignment of a C type on wasm32.
	fn layout(&self, ty: &Ty) -> Layout {
		let l = |size, align| Layout { size, align };
		match ty {
			Ty::Void => l(0, 1),
			Ty::Bool => l(1, 1),
			Ty::Int(Int::U8 | Int::I8) => l(1, 1),
			Ty::Int(Int::U16 | Int::I16) => l(2, 2),
			Ty::Int(Int::U64 | Int::I64) => l(8, 8),
			Ty::Int(_) | Ty::F32 | Ty::Char | Ty::Enum(_) | Ty::Poll | Ty::Status => l(4, 4),
			Ty::F64 => l(8, 8),
			Ty::Handle { .. }
			| Ty::Future(_)
			| Ty::Out(_)
			| Ty::In(_)
			| Ty::InOut(_)
			| Ty::Fn(..)
			| Ty::Data
			| Ty::Jni(_) => l(4, 4),
			Ty::Buf | Ty::Waker | Ty::ForeignFuture | Ty::ForeignReader => l(12, 4),
			Ty::U128 => l(16, 8),
			Ty::Duration => l(16, 8),
			Ty::IpAddr(Family::V4) => l(4, 1),
			Ty::IpAddr(Family::V6) => l(16, 1),
			Ty::IpAddr(Family::Any) => l(17, 1),
			Ty::SocketAddr(Family::V4) => l(6, 2),
			Ty::SocketAddr(Family::V6) => l(28, 4),
			Ty::SocketAddr(Family::Any) => l(32, 4),
			Ty::Struct(name) => self.struct_layout(name),
		}
	}

	/// The size and alignment of a struct of `fields`, and each field's offset.
	fn place(&self, fields: &[Ty]) -> (Layout, Vec<u32>) {
		let mut at = 0;
		let mut align = 1;
		let mut offsets = Vec::new();
		for field in fields {
			let l = self.layout(field);
			at = round(at, l.align);
			offsets.push(at);
			at += l.size;
			align = align.max(l.align);
		}
		(
			Layout {
				size: round(at, align),
				align,
			},
			offsets,
		)
	}

	/// What the C struct `name` is.
	fn shape(&self, name: &str) -> Shape {
		let abi = self.abi;
		if let Some(n) = abi.arrays.get(name) {
			return Shape::Bytes(*n as u32);
		}
		if let Some(ty) = abi.ranges.get(name) {
			return Shape::Fields(match ty {
				Type::Range(int) => vec![Ty::Int(*int), Ty::Int(*int)],
				Type::Bound(int) => vec![Ty::Enum(abi.rt("bound")), Ty::Int(*int)],
				Type::Bounds(int) => {
					let bound = Ty::Struct(abi.range(&Type::Bound(*int)));
					vec![bound.clone(), bound]
				}
				other => unreachable!("{other:?} is no range"),
			});
		}
		if let Some(elem) = abi.lists.get(name) {
			return Shape::Fields(vec![Ty::In(Box::new(abi.ty(elem))), Ty::Int(Int::Usize), Ty::Data]);
		}
		if let Some(inner) = abi.options.get(name) {
			return Shape::Fields(vec![Ty::Bool, abi.ty(inner)]);
		}
		if let Some(elements) = abi.tuples.get(name) {
			return Shape::Fields(elements.iter().map(|e| abi.ty(e)).collect());
		}
		if let Some(i) = (0..abi.api.records.len()).find(|&i| abi.record(i) == name) {
			let record = &abi.api.records[i];
			if let Some(opaque) = &record.opaque {
				return Shape::Units(opaque.unit() as u32, opaque.units() as u32);
			}
			return Shape::Fields(record.fields.iter().map(|f| abi.ty(&f.ty)).collect());
		}
		if let Some(i) = (0..abi.api.enums.len()).find(|&i| abi.enumeration(i) == name) {
			let payloads = (abi.api.enums[i].variants.iter())
				.filter(|v| !v.fields.is_empty())
				.map(|v| v.fields.iter().map(|f| abi.ty(&f.ty)).collect())
				.collect();
			return Shape::Union(payloads);
		}
		if (0..abi.api.callbacks.len()).any(|i| abi.callback(i) == name) {
			return Shape::Fields(vec![Ty::Data, Ty::Data]);
		}
		unreachable!("`{name}` is no struct of the API")
	}

	fn struct_layout(&self, name: &str) -> Layout {
		match self.shape(name) {
			Shape::Bytes(n) => Layout { size: n, align: 1 },
			Shape::Units(unit, units) => Layout {
				size: unit * units,
				align: unit,
			},
			Shape::Fields(fields) => self.place(&fields).0,
			Shape::Union(payloads) => {
				let (union, _) = self.union(&payloads);
				let align = union.align.max(4);
				Layout {
					size: round(round(4, union.align) + union.size, align),
					align,
				}
			}
		}
	}

	/// The layout of a union of `payloads`, each a struct of its fields, and where the union
	/// sits after an `int` tag.
	fn union(&self, payloads: &[Vec<Ty>]) -> (Layout, u32) {
		let mut size = 0;
		let mut align = 1;
		for payload in payloads {
			let (l, _) = self.place(payload);
			size = size.max(l.size);
			align = align.max(l.align);
		}
		let l = Layout {
			size: round(size, align),
			align,
		};
		(l, round(4, align))
	}

	/// The scalar a C type crosses a wasm call as, if it is one, or a struct of exactly one.
	fn scalar(&self, ty: &Ty) -> Option<Wasm> {
		match ty {
			Ty::Void => None,
			Ty::Int(Int::U64 | Int::I64) => Some(Wasm::I64),
			Ty::F32 => Some(Wasm::F32),
			Ty::F64 => Some(Wasm::F64),
			Ty::Bool
			| Ty::Int(_)
			| Ty::Char
			| Ty::Enum(_)
			| Ty::Poll
			| Ty::Status
			| Ty::Handle { .. }
			| Ty::Future(_)
			| Ty::Out(_)
			| Ty::In(_)
			| Ty::InOut(_)
			| Ty::Fn(..)
			| Ty::Data
			| Ty::Jni(_) => Some(Wasm::I32),
			Ty::Buf | Ty::Waker | Ty::ForeignFuture | Ty::ForeignReader | Ty::Duration | Ty::U128 => None,
			Ty::IpAddr(_) | Ty::SocketAddr(_) => None,
			Ty::Struct(name) => match self.shape(name) {
				Shape::Bytes(1) => Some(Wasm::I32),
				Shape::Units(unit, 1) => Some(if unit == 8 { Wasm::I64 } else { Wasm::I32 }),
				Shape::Fields(fields) if fields.len() == 1 => self.scalar(&fields[0]),
				_ => None,
			},
		}
	}

	/// Whether a C type is a plain scalar, not a struct.
	fn plain(ty: &Ty) -> bool {
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

	/// The `DataView` method suffix reading or writing the scalar `w` a struct of one holds.
	fn word(w: Wasm) -> (&'static str, &'static str) {
		match w {
			Wasm::I32 => ("Int32", ""),
			Wasm::I64 => ("BigInt64", ""),
			Wasm::F32 => ("Float32", ""),
			Wasm::F64 => ("Float64", ""),
		}
	}

	// Reading and writing values in memory.

	/// The value of type `ty` at the address `p`, its C value laid out as C's: Rust's to free
	/// where `own`, else lent.
	fn get(&mut self, ty: &Type, p: &str, own: bool) -> String {
		let abi = self.abi;
		let o = if own { "true" } else { "false" };
		match ty {
			Type::Unit => "undefined".to_string(),
			Type::Bool => format!("rt.dv().getUint8({p}) !== 0"),
			Type::Int(int) | Type::NonZero(int) => match int {
				Int::U8 => format!("rt.dv().getUint8({p})"),
				Int::I8 => format!("rt.dv().getInt8({p})"),
				Int::U16 => format!("rt.dv().getUint16({p}, true)"),
				Int::I16 => format!("rt.dv().getInt16({p}, true)"),
				Int::U32 | Int::Usize => format!("rt.dv().getUint32({p}, true)"),
				Int::I32 | Int::Isize => format!("rt.dv().getInt32({p}, true)"),
				Int::U64 => format!("rt.u64Get({p})"),
				Int::I64 => format!("rt.i64Get({p})"),
			},
			Type::F32 => format!("rt.dv().getFloat32({p}, true)"),
			Type::F64 => format!("rt.dv().getFloat64({p}, true)"),
			Type::Char => format!("rt.dv().getUint32({p}, true)"),
			Type::U128 => format!("rt.u128Get({p})"),
			Type::Str | Type::String => match own {
				true => format!("rt.str({p})"),
				false => format!("rt.strLent({p})"),
			},
			Type::Slice | Type::Vec | Type::Bytes => match own {
				true => format!("rt.bytes_({p})"),
				false => format!("rt.bytesLent({p})"),
			},
			Type::Path | Type::PathBuf => match own {
				true => format!("{{ bytes: rt.bytes_({p}) }}"),
				false => format!("{{ bytes: rt.bytesLent({p}) }}"),
			},
			Type::Duration => format!("rt.durationGet({p})"),
			Type::Array(n) => format!("rt.mem().slice({p}, {p} + {n})"),
			Type::SocketAddr(Family::Any) => format!("rt.socketGet({p})"),
			Type::SocketAddr(Family::V4) => format!("rt.socketV4Get({p})"),
			Type::SocketAddr(Family::V6) => format!("rt.socketV6Get({p})"),
			Type::IpAddr(Family::Any) => format!("rt.ipGet({p})"),
			Type::IpAddr(Family::V4) => format!("rt.ipv4Get({p})"),
			Type::IpAddr(Family::V6) => format!("rt.ipv6Get({p})"),
			Type::Waker => format!("rt.wakerGet({p})"),
			Type::Object(_) => self.lift(ty, &format!("rt.dv().getUint32({p}, true)"), own),
			Type::Option(inner) if matches!(inner.base(), Type::Object(_)) => {
				self.lift(ty, &format!("rt.dv().getUint32({p}, true)"), own)
			}
			Type::Option(inner) if Abi::is_buf(inner) || matches!(inner.base(), Type::List(..)) => {
				let value = self.get(inner.base(), p, own);
				format!("(rt.isNone({p}) ? undefined : {value})")
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.get(inner, p, own),
			Type::Enum(i) if abi.api.enums[*i].plain() => {
				let lift = self.enum_lift(*i);
				format!("{lift}(rt.dv().getInt32({p}, true))")
			}
			Type::Record(_)
			| Type::Enum(_)
			| Type::Option(_)
			| Type::List(..)
			| Type::Tuple(_)
			| Type::Range(_)
			| Type::Bound(_)
			| Type::Bounds(_) => {
				let r = self.reader(ty);
				format!("{r}({p}, {o})")
			}
			other => unreachable!("{other:?} is never read"),
		}
	}

	/// Statements writing the value `e` of type `ty` at the address `p`: bytes and text as a
	/// view of a copy on the stack, unless `take`, where Rust takes a buffer of its own.
	fn put(&mut self, ty: &Type, e: &str, p: &str, take: bool) -> String {
		let abi = self.abi;
		match ty {
			Type::Unit => String::new(),
			Type::Bool => format!("rt.dv().setUint8({p}, {e} ? 1 : 0);"),
			Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64 | Type::Char => {
				let (set, lowered) = match ty {
					Type::Int(Int::U64) | Type::NonZero(Int::U64) => {
						return format!("rt.u64Put({p}, {});", self.checked(ty, e));
					}
					Type::Int(Int::I64) | Type::NonZero(Int::I64) => {
						return format!("rt.i64Put({p}, {});", self.checked(ty, e));
					}
					Type::Int(int) | Type::NonZero(int) => (int_view(*int), self.checked(ty, e)),
					Type::F32 => ("Float32", self.checked(ty, e)),
					Type::F64 => ("Float64", e.to_string()),
					_ => ("Uint32", self.checked(ty, e)),
				};
				match set {
					"Uint8" | "Int8" => format!("rt.dv().set{set}({p}, {lowered});"),
					_ => format!("rt.dv().set{set}({p}, {lowered}, true);"),
				}
			}
			Type::U128 => format!("rt.u128Put({p}, {e});"),
			Type::Str | Type::String => match take {
				true => format!("rt.copied({p}, {e});"),
				false => format!("rt.putStr({p}, {e});"),
			},
			Type::Slice | Type::Vec | Type::Bytes => match take {
				true => format!("rt.copiedBytes({p}, {e});"),
				false => format!("rt.putBytes({p}, {e});"),
			},
			Type::Path | Type::PathBuf => format!("rt.putBytes({p}, {e}.bytes);"),
			Type::Duration => format!("rt.durationPut({p}, {e});"),
			Type::Array(n) => format!("rt.mem().set(rt.array({e}, {n}), {p});"),
			Type::SocketAddr(Family::Any) => format!("rt.socketPut({p}, {e});"),
			Type::SocketAddr(Family::V4) => format!("rt.socketV4Put({p}, {e});"),
			Type::SocketAddr(Family::V6) => format!("rt.socketV6Put({p}, {e});"),
			Type::IpAddr(Family::Any) => format!("rt.ipPut({p}, {e});"),
			Type::IpAddr(Family::V4) => format!("rt.ipv4Put({p}, {e});"),
			Type::IpAddr(Family::V6) => format!("rt.ipv6Put({p}, {e});"),
			Type::Object(_) => format!("rt.dv().setUint32({p}, rt.hold({e}), true);"),
			Type::Option(inner) if matches!(inner.base(), Type::Object(_)) => {
				format!("rt.dv().setUint32({p}, rt.holdOr({e}), true);")
			}
			Type::Option(inner) if Abi::is_buf(inner) || matches!(inner.base(), Type::List(..)) => {
				let some = self.put(inner.base(), e, p, take);
				match matches!(inner.base(), Type::List(..)) {
					true => some,
					false => format!(
						"if ({e} === undefined) rt.putNone({p});\nelse {{\n{}\n}}",
						indent(&some)
					),
				}
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.put(inner, e, p, take),
			Type::Enum(i) if abi.api.enums[*i].plain() => {
				let lower = self.enum_lower(*i);
				format!("rt.dv().setInt32({p}, {lower}({e}), true);")
			}
			Type::Callback(i, _) => {
				let vtable = self.vtable(*i);
				format!(
					"rt.dv().setUint32({p}, {vtable}(), true);\nrt.dv().setUint32({p} + 4, rt.give({e}, () => {e}.close?.()), true);"
				)
			}
			Type::Waker => format!("rt.wakerOf({p}, {e});"),
			Type::Future => format!("rt.futureOf({p}, {e});"),
			Type::Reader => format!("rt.readerOf({p}, {e});"),
			Type::Record(_)
			| Type::Enum(_)
			| Type::Option(_)
			| Type::List(..)
			| Type::Tuple(_)
			| Type::Range(_)
			| Type::Bound(_)
			| Type::Bounds(_) => {
				let w = self.writer(ty, take);
				format!("{w}({p}, {e});")
			}
			other => unreachable!("{other:?} is never written"),
		}
	}

	/// `e`, a value of the scalar type `ty`, checked to be one of Rust's.
	fn checked(&self, ty: &Type, e: &str) -> String {
		match ty {
			Type::NonZero(int) => format!("rt.nonZero({})", self.checked(&Type::Int(*int), e)),
			Type::Int(int) => {
				let (min, max): (i64, i64) = match int {
					Int::U8 => (0, 0xff),
					Int::U16 => (0, 0xffff),
					Int::U32 | Int::Usize => (0, 0xffff_ffff),
					Int::I8 => (-0x80, 0x7f),
					Int::I16 => (-0x8000, 0x7fff),
					Int::I32 | Int::Isize => (-0x8000_0000, 0x7fff_ffff),
					Int::U64 | Int::I64 => return e.to_string(),
				};
				format!("rt.int({e}, {min}, {max}, \"{}\")", int.rust())
			}
			Type::F32 => format!("rt.f32({e})"),
			Type::Char => format!("rt.char({e})"),
			_ => e.to_string(),
		}
	}

	/// The wasm argument of the value `e` of type `ty`, whose C type is a plain scalar.
	fn lower(&mut self, ty: &Type, e: &str) -> String {
		match ty {
			Type::Bool => format!("({e} ? 1 : 0)"),
			Type::Int(Int::U64) | Type::NonZero(Int::U64) => format!("rt.u64Big({})", self.checked(ty, e)),
			Type::Int(Int::I64) | Type::NonZero(Int::I64) => format!("rt.i64Big({})", self.checked(ty, e)),
			Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::Char => self.checked(ty, e),
			Type::F64 => e.to_string(),
			Type::Enum(i) => {
				let lower = self.enum_lower(*i);
				format!("{lower}({e})")
			}
			Type::Object(_) => format!("rt.hold({e})"),
			Type::Option(inner) if matches!(inner.base(), Type::Object(_)) => format!("rt.holdOr({e})"),
			Type::Box(inner) | Type::Ref(_, inner) => self.lower(inner, e),
			other => unreachable!("{other:?} is no scalar"),
		}
	}

	/// The value of type `ty` of what a wasm call returned, `e`, whose C type is a plain
	/// scalar: a handle owned where `own`, else lent.
	fn lift(&mut self, ty: &Type, e: &str, own: bool) -> String {
		match ty {
			Type::Unit => "undefined".to_string(),
			Type::Bool => format!("{e} !== 0"),
			Type::Int(int) | Type::NonZero(int) => match int {
				Int::U8 => format!("{e} & 0xff"),
				Int::U16 => format!("{e} & 0xffff"),
				Int::U32 | Int::Usize => format!("{e} >>> 0"),
				Int::I8 => format!("({e} << 24) >> 24"),
				Int::I16 => format!("({e} << 16) >> 16"),
				Int::I32 | Int::Isize => format!("{e} | 0"),
				Int::U64 => format!("rt.u64OfBig({e})"),
				Int::I64 => format!("rt.i64OfBig({e})"),
			},
			Type::F32 | Type::F64 => e.to_string(),
			Type::Char => format!("{e} >>> 0"),
			Type::Enum(i) => {
				let lift = self.enum_lift(*i);
				format!("{lift}({e})")
			}
			Type::Object(i) => {
				let kind = self.kind(*i);
				let t = self.tt(ty);
				match own {
					true => format!("new rt.Handle({e}, {kind}, true) as {t}"),
					false => format!("rt.lentHandle({e}, {kind}) as {t}"),
				}
			}
			Type::Option(inner) if matches!(inner.base(), Type::Object(_)) => {
				let some = self.lift(inner.base(), "h$", own);
				format!("((h$) => h$ === 0 ? undefined : {some})({e})")
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.lift(inner, e, own),
			other => unreachable!("{other:?} is no scalar"),
		}
	}

	/// The kind of object `i`'s handles, defined once in the glue.
	fn kind(&mut self, i: usize) -> String {
		let abi = self.abi;
		let c = abi.object(i);
		let name = format!("k_{}", self.bare(&c));
		if !self.helpers.contains_key(&name) {
			let class = self.class(&c);
			let drop = match abi.checked[i] {
				true => format!(", drop: \"{c}_drop\""),
				false => String::new(),
			};
			self.helpers.insert(
				name.clone(),
				format!("export const {name}: rt.Kind = {{ name: \"{class}\", free: \"{c}_free\"{drop} }};\n\n"),
			);
		}
		format!("g.{name}")
	}

	/// The glue's function lowering a plain enum `i`'s value to its discriminant.
	fn enum_lower(&mut self, i: usize) -> String {
		let abi = self.abi;
		let c = abi.enumeration(i);
		let name = format!("e_{}", self.bare(&c));
		if !self.helpers.contains_key(&name) {
			let module = std::mem::replace(&mut self.module, "glue".to_string());
			let t = self.tt(&Type::Enum(i));
			self.module = module;
			let e = &abi.api.enums[i];
			let cases: Vec<String> = (e.variants.iter())
				.filter(|v| self.here(v.on))
				.map(|v| format!("\t\tcase \"{}\":\n\t\t\treturn {};", v.name, v.discriminant))
				.collect();
			self.helpers.insert(
				name.clone(),
				format!(
					"export function {name}(v: {t}): number {{\n\tswitch (v) {{\n{}\n\t\tdefault:\n\t\t\tthrow new RangeError(`${{String(v)}} is no {t}`);\n\t}}\n}}\n\n",
					cases.join("\n")
				),
			);
		}
		format!("g.{name}")
	}

	/// The glue's function lifting a plain enum `i`'s value from its discriminant.
	fn enum_lift(&mut self, i: usize) -> String {
		let abi = self.abi;
		let c = abi.enumeration(i);
		let name = format!("f_{}", self.bare(&c));
		if !self.helpers.contains_key(&name) {
			let module = std::mem::replace(&mut self.module, "glue".to_string());
			let t = self.tt(&Type::Enum(i));
			self.module = module;
			let e = &abi.api.enums[i];
			let cases: Vec<String> = (e.variants.iter())
				.filter(|v| self.here(v.on))
				.map(|v| format!("\t\tcase {}:\n\t\t\treturn \"{}\";", v.discriminant, v.name))
				.collect();
			self.helpers.insert(
				name.clone(),
				format!(
					"export function {name}(c: number): {t} {{\n\tswitch (c) {{\n{}\n\t\tdefault:\n\t\t\tthrow new RangeError(`{c}: an invalid value ${{c}}`);\n\t}}\n}}\n\n",
					cases.join("\n")
				),
			);
		}
		format!("g.{name}")
	}

	/// A helper's name for `ty`, `base` and a number apart from every other's.
	fn helper_name(&mut self, base: &str) -> String {
		let mut name = base.to_string();
		for k in 2.. {
			if !self.helpers.contains_key(&name) {
				break;
			}
			name = format!("{base}_{k}");
		}
		self.helpers.insert(name.clone(), String::new());
		name
	}

	/// The glue's function reading a value of type `ty`, a struct's, at a pointer.
	fn reader(&mut self, ty: &Type) -> String {
		if let Some(name) = self.readers.get(ty) {
			return format!("g.{name}");
		}
		let abi = self.abi;
		let c = match abi.ty(ty) {
			Ty::Struct(c) => c,
			other => unreachable!("{other:?} is no struct"),
		};
		let name = self.helper_name(&format!("r_{}", self.bare(&c)));
		self.readers.insert(ty.clone(), name.clone());
		let module = std::mem::replace(&mut self.module, "glue".to_string());
		let t = self.tt(ty);
		let body = self.read_body(ty, &c).replace("void o;\n\t", "");
		let body = match uses(&body, "o") {
			true => body,
			false => format!("void o;\n\t{body}"),
		};
		self.module = module;
		self.helpers.insert(
			name.clone(),
			format!(
				"/** A `{}` at `p`. */\nexport function {name}(p: number, o: boolean): {t} {{\n\t{body}\n}}\n\n",
				abi.spell(ty)
			),
		);
		format!("g.{name}")
	}

	fn read_body(&mut self, ty: &Type, c: &str) -> String {
		let abi = self.abi;
		let offsets = |g: &Self| match g.shape(c) {
			Shape::Fields(fields) => g.place(&fields).1,
			_ => Vec::new(),
		};
		match ty {
			Type::Record(i) if abi.api.records[*i].opaque.is_some() => {
				let size = self.layout(&Ty::Struct(c.to_string())).size;
				let t = self.tt(ty);
				format!("void o;\n\treturn rt.opaqueMade(rt.mem().slice(p, p + {size})) as {t};")
			}
			Type::Record(i) => {
				let record = &abi.api.records[*i];
				let at = offsets(self);
				let names = field_names(&record.fields);
				let fields: Vec<String> = (record.fields.iter().zip(at).zip(names))
					.map(|((f, at), name)| {
						let value = self.get_o(&f.ty, &format!("p + {at}"));
						format!("{name}: {value}")
					})
					.collect();
				format!("void o;\n\treturn {{ {} }};", fields.join(", "))
			}
			Type::Enum(i) => {
				let e = &abi.api.enums[*i];
				let Shape::Union(payloads) = self.shape(c) else {
					unreachable!("a data enum is a union")
				};
				let (_, u) = self.union(&payloads);
				let mut cases = Vec::new();
				for (n, variant) in e.variants.iter().enumerate() {
					// A variant the target lacks reads no fields, whose types it may lack too.
					if !self.here(variant.on) {
						continue;
					}
					let mut fields = vec![format!("tag: \"{}\"", variant.name)];
					if !variant.fields.is_empty() {
						let tys: Vec<Ty> = variant.fields.iter().map(|f| abi.ty(&f.ty)).collect();
						let (_, at) = self.place(&tys);
						for ((f, at), name) in variant.fields.iter().zip(at).zip(variant_fields(&variant.fields)) {
							let value = self.get_o(&f.ty, &format!("p + {}", u + at));
							fields.push(format!("{name}: {value}"));
						}
					}
					cases.push(format!("\t\tcase {n}:\n\t\t\treturn {{ {} }};", fields.join(", ")));
				}
				format!(
					"switch (rt.dv().getInt32(p, true)) {{\n{}\n\t\tdefault:\n\t\t\tthrow new RangeError(\"{c}: an invalid tag\");\n\t}}",
					cases.join("\n")
				)
			}
			Type::Option(inner) => {
				let at = offsets(self)[1];
				let value = self.get_o(inner, &format!("p + {at}"));
				let none = match undefinable(inner) {
					true => "rt.None",
					false => "undefined",
				};
				format!("return rt.dv().getUint8(p) !== 0 ? {value} : {none};")
			}
			Type::List(..) if let Some(block) = ty.block() => {
				let array = typed_array(block);
				let size = self.layout(&abi.ty(block)).size;
				format!(
					"const d = rt.dv();\n\tconst ptr = d.getUint32(p, true);\n\tconst len = d.getUint32(p + 4, true);\n\tconst out = new {array}(rt.mem().buffer.slice(ptr, ptr + len * {size}));\n\tif (o) rt.w.{c}_free(p);\n\treturn out;"
				)
			}
			Type::List(_, elem) => {
				let stride = {
					let l = self.layout(&abi.ty(elem));
					round(l.size, l.align)
				};
				let item = self.get_o(elem, &format!("ptr + i * {stride}"));
				let t = self.tt(elem);
				format!(
					"const d = rt.dv();\n\tconst ptr = d.getUint32(p, true);\n\tconst len = d.getUint32(p + 4, true);\n\tconst out: ({t})[] = [];\n\tfor (let i = 0; i < len; i++) out.push({item});\n\tif (o) {{\n\t\trt.dv().setUint32(p + 4, 0, true);\n\t\trt.w.{c}_free(p);\n\t}}\n\treturn out;"
				)
			}
			Type::Tuple(elements) => {
				let at = offsets(self);
				let values: Vec<String> = (elements.iter().zip(at))
					.map(|(e, at)| self.get_o(e, &format!("p + {at}")))
					.collect();
				format!("return [{}];", values.join(", "))
			}
			Type::Range(int) => {
				let at = offsets(self);
				let int = Type::Int(*int);
				let start = self.get(&int, &format!("p + {}", at[0]), true);
				let end = self.get(&int, &format!("p + {}", at[1]), true);
				format!("void o;\n\treturn {{ start: {start}, end: {end} }};")
			}
			Type::Bound(int) => {
				let at = offsets(self);
				let value = self.get(&Type::Int(*int), &format!("p + {}", at[1]), true);
				format!(
					"void o;\n\tswitch (rt.dv().getInt32(p, true)) {{\n\t\tcase 0:\n\t\t\treturn {{ tag: \"Unbounded\" }};\n\t\tcase 1:\n\t\t\treturn {{ tag: \"Included\", _0: {value} }};\n\t\tcase 2:\n\t\t\treturn {{ tag: \"Excluded\", _0: {value} }};\n\t\tdefault:\n\t\t\tthrow new RangeError(\"{c}: an invalid kind\");\n\t}}"
				)
			}
			Type::Bounds(int) => {
				let at = offsets(self);
				let bound = Type::Bound(*int);
				let start = self.get_o(&bound, &format!("p + {}", at[0]));
				let end = self.get_o(&bound, &format!("p + {}", at[1]));
				format!("return {{ start: {start}, end: {end} }};")
			}
			other => unreachable!("{other:?} has no reader"),
		}
	}

	/// [`Self::get`] of a value a reader reads, owned as its `o` says.
	fn get_o(&mut self, ty: &Type, p: &str) -> String {
		let owned = self.get(ty, p, true);
		let lent = self.get(ty, p, false);
		match owned == lent {
			true => owned,
			false => format!("(o ? {owned} : {lent})"),
		}
	}

	/// The glue's function writing a value of type `ty`, a struct's, at a pointer: buffers
	/// Rust takes where `take`.
	fn writer(&mut self, ty: &Type, take: bool) -> String {
		let key = match take {
			true => Type::Tuple(vec![ty.clone(), Type::Unit]),
			false => ty.clone(),
		};
		if let Some(name) = self.writers.get(&key) {
			return format!("g.{name}");
		}
		let abi = self.abi;
		let c = match abi.ty(ty) {
			Ty::Struct(c) => c,
			other => unreachable!("{other:?} is no struct"),
		};
		let base = match take {
			true => format!("t_{}", self.bare(&c)),
			false => format!("w_{}", self.bare(&c)),
		};
		let name = self.helper_name(&base);
		self.writers.insert(key, name.clone());
		let module = std::mem::replace(&mut self.module, "glue".to_string());
		let t = self.tt(ty);
		let body = self.write_body(ty, &c, take);
		self.module = module;
		self.helpers.insert(
			name.clone(),
			format!(
				"/** Write the `{}` `v` at `p`. */\nexport function {name}(p: number, v: {t}{}): void {{\n{}\n}}\n\n",
				abi.spell(ty),
				match ty {
					Type::List(..) => " | undefined",
					_ => "",
				},
				indent(&body)
			),
		);
		format!("g.{name}")
	}

	fn write_body(&mut self, ty: &Type, c: &str, take: bool) -> String {
		let abi = self.abi;
		let offsets = |g: &Self| match g.shape(c) {
			Shape::Fields(fields) => g.place(&fields).1,
			_ => Vec::new(),
		};
		match ty {
			Type::Record(i) if abi.api.records[*i].opaque.is_some() => {
				let size = self.layout(&Ty::Struct(c.to_string())).size;
				format!("rt.opaquePut(v, {size}, p);")
			}
			Type::Record(i) => {
				let record = &abi.api.records[*i];
				let at = offsets(self);
				let names = field_names(&record.fields);
				let lines: Vec<String> = (record.fields.iter().zip(at).zip(names))
					.map(|((f, at), name)| self.put(&f.ty, &format!("v.{name}"), &format!("p + {at}"), take))
					.collect();
				lines.join("\n")
			}
			Type::Enum(i) => {
				let e = &abi.api.enums[*i];
				let Shape::Union(payloads) = self.shape(c) else {
					unreachable!("a data enum is a union")
				};
				let (_, u) = self.union(&payloads);
				let mut cases = Vec::new();
				for (n, variant) in e.variants.iter().enumerate() {
					if !self.here(variant.on) {
						continue;
					}
					let mut body = vec![format!("rt.dv().setInt32(p, {n}, true);")];
					if !variant.fields.is_empty() {
						let tys: Vec<Ty> = variant.fields.iter().map(|f| abi.ty(&f.ty)).collect();
						let (_, at) = self.place(&tys);
						for ((f, at), name) in variant.fields.iter().zip(at).zip(variant_fields(&variant.fields)) {
							body.push(self.put(&f.ty, &format!("v.{name}"), &format!("p + {}", u + at), take));
						}
					}
					body.push("return;".to_string());
					cases.push(format!("case \"{}\":\n{}", variant.name, indent(&body.join("\n"))));
				}
				cases.push(format!(
					"default:\n\tthrow new RangeError(`${{String((v as {{ tag: unknown }}).tag)}} is no variant of {c}`);"
				));
				format!("switch (v.tag) {{\n{}\n}}", indent(&cases.join("\n")))
			}
			Type::Option(inner) => {
				let at = offsets(self)[1];
				let some = self.put(inner, "v", &format!("p + {at}"), take);
				let none = match undefinable(inner) {
					true => "rt.None",
					false => "undefined",
				};
				let narrowed = match undefinable(inner) {
					true => {
						let t = self.tt(inner);
						format!(
							"\nconst s = v as {t};\n{}",
							some.replace("(v", "(s").replace(" v,", " s,").replace(", v)", ", s)")
						)
					}
					false => format!("\n{some}"),
				};
				let some = indent(&format!("rt.dv().setUint8(p, 1);{narrowed}"));
				format!("if (v === {none}) rt.dv().setUint8(p, 0);\nelse {{\n{some}\n}}")
			}
			Type::List(..) if take => {
				self.errors.push(format!(
					"`{}` returned from JavaScript to Rust has no web form yet, which rust2-ffi should give it",
					abi.spell(ty)
				));
				String::new()
			}
			Type::List(..) if let Some(block) = ty.block() => {
				let size = self.layout(&abi.ty(block)).size;
				format!(
					"const d = rt.dv();\nif (v === undefined) {{\n\td.setUint32(p, 0, true);\n\td.setUint32(p + 4, 0, true);\n\td.setUint32(p + 8, 0, true);\n\treturn;\n}}\nconst ptr = rt.alloc(Math.max(v.length * {size}, 1), {size});\nrt.mem().set(new Uint8Array(v.buffer, v.byteOffset, v.length * {size}), ptr);\nconst e = rt.dv();\ne.setUint32(p, ptr, true);\ne.setUint32(p + 4, v.length, true);\ne.setUint32(p + 8, 0, true);"
				)
			}
			Type::List(_, elem) => {
				let l = self.layout(&abi.ty(elem));
				let stride = round(l.size, l.align);
				let item = self.put(elem, "v[i]", &format!("ptr + i * {stride}"), false);
				format!(
					"if (v === undefined) {{\n\tconst d = rt.dv();\n\td.setUint32(p, 0, true);\n\td.setUint32(p + 4, 0, true);\n\td.setUint32(p + 8, 0, true);\n\treturn;\n}}\nconst ptr = rt.alloc(Math.max(v.length * {stride}, 1), {});\nfor (let i = 0; i < v.length; i++) {{\n{}\n}}\nconst d = rt.dv();\nd.setUint32(p, ptr, true);\nd.setUint32(p + 4, v.length, true);\nd.setUint32(p + 8, 0, true);",
					l.align,
					indent(&item)
				)
			}
			Type::Tuple(elements) => {
				let at = offsets(self);
				let lines: Vec<String> = (elements.iter().enumerate().zip(at))
					.map(|((n, e), at)| self.put(e, &format!("v[{n}]"), &format!("p + {at}"), take))
					.collect();
				lines.join("\n")
			}
			Type::Range(int) => {
				let at = offsets(self);
				let int = Type::Int(*int);
				let start = self.put(&int, "v.start", &format!("p + {}", at[0]), take);
				let end = self.put(&int, "v.end", &format!("p + {}", at[1]), take);
				format!("{start}\n{end}")
			}
			Type::Bound(int) => {
				let at = offsets(self);
				let value = self.put(&Type::Int(*int), "v._0", &format!("p + {}", at[1]), take);
				format!(
					"switch (v.tag) {{\n\tcase \"Unbounded\":\n\t\trt.dv().setInt32(p, 0, true);\n\t\treturn;\n\tcase \"Included\":\n\t\trt.dv().setInt32(p, 1, true);\n\t\t{value}\n\t\treturn;\n\tcase \"Excluded\":\n\t\trt.dv().setInt32(p, 2, true);\n\t\t{value}\n\t\treturn;\n\tdefault:\n\t\tthrow new RangeError(\"a Bound is Included, Excluded, or Unbounded\");\n}}"
				)
			}
			Type::Bounds(int) => {
				let at = offsets(self);
				let bound = Type::Bound(*int);
				let start = self.put(&bound, "v.start", &format!("p + {}", at[0]), take);
				let end = self.put(&bound, "v.end", &format!("p + {}", at[1]), take);
				format!("{start}\n{end}")
			}
			other => unreachable!("{other:?} has no writer"),
		}
	}
}

/// What a C struct of the API is.
enum Shape {
	/// `[u8; N]`'s bytes.
	Bytes(u32),
	/// An opaque value's units, of their size each.
	Units(u32, u32),
	/// A struct of these fields.
	Fields(Vec<Ty>),
	/// A data enum: an `int` tag, then a union of each variant's fields.
	Union(Vec<Vec<Ty>>),
}

/// Whether `ty` is bytes, which a call returns as an `rt.Bytes`.
fn lends(ty: &Type) -> bool {
	matches!(ty, Type::Slice | Type::Vec | Type::Bytes)
}

/// Whether a value of `ty` can be `undefined`, so its `Option` keeps `None` apart as `rt.None`.
fn undefinable(ty: &Type) -> bool {
	matches!(ty.base(), Type::Unit | Type::Option(_))
}

/// The typed array holding a block's elements exactly.
fn typed_array(elem: &Type) -> &'static str {
	match elem {
		Type::Int(Int::U8) => "Uint8Array",
		Type::Int(Int::U16) => "Uint16Array",
		Type::Int(Int::U32) => "Uint32Array",
		Type::Int(Int::U64) => "BigUint64Array",
		Type::Int(Int::I8) => "Int8Array",
		Type::Int(Int::I16) => "Int16Array",
		Type::Int(Int::I32) => "Int32Array",
		Type::Int(Int::I64) => "BigInt64Array",
		Type::F32 => "Float32Array",
		Type::F64 => "Float64Array",
		other => unreachable!("{other:?} is no block"),
	}
}

/// The `DataView` accessor of an integer no wider than 32 bits.
fn int_view(int: Int) -> &'static str {
	match int {
		Int::U8 => "Uint8",
		Int::I8 => "Int8",
		Int::U16 => "Uint16",
		Int::I16 => "Int16",
		Int::U32 | Int::Usize => "Uint32",
		Int::I32 | Int::Isize => "Int32",
		Int::U64 | Int::I64 => unreachable!("a u64 is two words"),
	}
}

/// A snake-case name in camel case: `into_label` is `intoLabel`, and `0` is `_0`.
fn camel(name: &str) -> String {
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("_{name}");
	}
	let pascal = pascal(name);
	let mut chars = pascal.chars();
	match chars.next() {
		Some(first) => first.to_lowercase().chain(chars).collect(),
		None => name.to_string(),
	}
}

/// A field's name: in camel case, and `_0` for a positional one.
fn field_name(name: &str) -> String {
	camel(name)
}

/// The names of a record's fields, each a field's, and distinct: `foo` and `foo_`, both
/// `foo` in camel case, are `foo` and `foo2`.
fn field_names(fields: &[crate::ir::Field]) -> Vec<String> {
	abi::distinct(fields.iter().map(|f| field_name(&f.name)))
}

/// The names of a data enum variant's fields: a record's, but `tag$` for one named `tag`,
/// which names the variant.
fn variant_fields(fields: &[crate::ir::Field]) -> Vec<String> {
	abi::distinct(fields.iter().map(|f| match field_name(&f.name) {
		name if name == "tag" => "tag$".to_string(),
		name => name,
	}))
}

/// A parameter's name: in camel case, with `_` appended if a reserved word.
fn param_name(name: &str) -> String {
	let name = camel(name);
	match RESERVED.contains(&name.as_str()) {
		true => format!("{name}_"),
		false => name,
	}
}

/// JSDoc holding `docs`, each line indented by `indent`, or nothing for none, then
/// `@deprecated` with its note where `deprecated` has one.
fn docs(docs: &str, deprecated: Option<&str>, indent: &str) -> String {
	let docs = match deprecated {
		Some(note) => format!("{docs}\n\n@deprecated {note}"),
		None => docs.to_string(),
	};
	let docs = docs.trim();
	if docs.is_empty() {
		return String::new();
	}
	let lines: String = (docs.lines())
		.map(|line| match line.is_empty() {
			true => format!("{indent} *\n"),
			false => format!("{indent} * {}\n", line.replace("*/", "* /")),
		})
		.collect();
	format!("{indent}/**\n{lines}{indent} */\n")
}

/// Code indented by one tab more, but for blank lines.
fn indent(code: &str) -> String {
	let mut out: String = (code.lines())
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

/// What a call into the module returned: nothing, a scalar, or a value at a pointer.
enum Ret {
	None,
	Value(String),
	At(String),
}

impl Gen<'_> {
	/// Statements making `local` the wasm argument, of the C type `c`, of `e` of type `ty`:
	/// a struct is copied onto the stack, and passed there, or as its one scalar.
	fn pass(&mut self, ty: &Type, c: &Ty, e: &str, local: &str) -> String {
		if Self::plain(c) {
			return format!("const {local} = {};", self.lower(ty, e));
		}
		let l = self.layout(c);
		match self.scalar(c) {
			Some(w) => {
				let p = format!("{local}p");
				let put = self.put(ty, e, &p, false);
				format!(
					"const {p} = rt.alloc({}, {});\n{put}\nconst {local} = rt.dv().get{}({p}, true);",
					l.size.max(8),
					l.align.max(4),
					Self::word(w).0
				)
			}
			None => {
				let put = self.put(ty, e, local, false);
				format!("const {local} = rt.alloc({}, {});\n{put}", l.size.max(1), l.align)
			}
		}
	}

	/// Statements calling the C function `name` with `args`, returning `ret`, into `lines`: a
	/// struct it returns is written through a pointer passed first, and with `status`, a check
	/// it fails throws once it returns.
	fn invoke(&mut self, name: &str, mut args: Vec<String>, ret: &Ty, status: bool, lines: &mut Vec<String>) -> Ret {
		if status {
			lines.push("const s$ = rt.alloc(4, 4);\nrt.dv().setInt32(s$, 0, true);".to_string());
			args.push("s$".to_string());
		}
		let sret = !Self::plain(ret) && self.scalar(ret).is_none();
		if sret {
			let l = self.layout(ret);
			lines.push(format!("const rp$ = rt.alloc({}, {});", l.size, l.align));
			args.insert(0, "rp$".to_string());
		}
		let call = format!("rt.w.{name}({})", args.join(", "));
		let ret = match ret {
			Ty::Void => {
				lines.push(format!("rt.s.d++;\n{call};\nrt.s.d--;"));
				Ret::None
			}
			_ if sret => {
				lines.push(format!("rt.s.d++;\n{call};\nrt.s.d--;"));
				Ret::At("rp$".to_string())
			}
			ret if Self::plain(ret) => {
				lines.push(format!("rt.s.d++;\nconst r$ = {call};\nrt.s.d--;"));
				Ret::Value("r$".to_string())
			}
			ret => {
				// A struct of one scalar comes back as it, which is read as the struct once
				// written back.
				let l = self.layout(ret);
				let w = self.scalar(ret).expect("a struct of one scalar");
				lines.push(format!(
					"rt.s.d++;\nconst r$ = {call};\nrt.s.d--;\nconst rp$ = rt.alloc({}, {});\nrt.dv().set{}(rp$, r$, true);",
					l.size.max(8),
					l.align.max(4),
					Self::word(w).0
				));
				Ret::At("rp$".to_string())
			}
		};
		if status {
			lines.push(format!("rt.status(s$, \"{name}\");"));
		}
		ret
	}

	/// The value of type `ty` a call returned as `ret`, which the caller owns.
	fn result(&mut self, ty: &Type, ret: &Ret) -> String {
		match ret {
			Ret::None => "undefined".to_string(),
			Ret::Value(e) => self.lift(ty, e, true),
			Ret::At(p) => self.returned(ty, p),
		}
	}

	/// The value of type `ty` in the wasm parameter `e` of C type `c` of a call into
	/// JavaScript, which Rust lends if bytes or text, and else gives.
	fn incoming(&mut self, ty: &Type, c: &Ty, e: &str, lines: &mut Vec<String>) -> String {
		if Self::plain(c) {
			return self.lift(ty, e, true);
		}
		match self.scalar(c) {
			Some(w) => {
				let l = self.layout(c);
				let p = format!("{e}p");
				lines.push(format!(
					"const {p} = rt.alloc({}, {});\nrt.dv().set{}({p}, {e}, true);",
					l.size.max(8),
					l.align.max(4),
					Self::word(w).0
				));
				self.get_lent(ty, &p)
			}
			None => self.get_lent(ty, e),
		}
	}

	/// [`Self::get`] of what Rust passes a call into JavaScript: bytes and text lent, which
	/// it copies, and any other value given.
	fn get_lent(&mut self, ty: &Type, p: &str) -> String {
		match ty {
			Type::Ref(_, inner) => self.get_lent(inner, p),
			ty if Abi::is_buf(ty) => self.get(ty, p, false),
			Type::Option(inner) if Abi::is_buf(inner) => self.get(ty, p, false),
			ty => self.get(ty, p, true),
		}
	}

	/// Statements returning `e` of type `ty` from a call into JavaScript, as the C type `c`,
	/// which Rust then owns: through `sret` if a struct it returns that way.
	fn outgoing(&mut self, ty: &Type, c: &Ty, e: &str, lines: &mut Vec<String>) {
		if matches!(ty.base(), Type::Object(_)) {
			self.errors.push(format!(
				"`{}` returned from JavaScript to Rust has no web form yet, which rust2-ffi should give it",
				self.abi.spell(ty)
			));
			return;
		}
		match c {
			Ty::Void => lines.push(format!("{e};")),
			c if Self::plain(c) => lines.push(format!("return {};", self.lower(ty, e))),
			c => {
				// The implementation runs once, though a writer may read what it returns twice.
				lines.push(format!("const v$ = {e};"));
				match self.scalar(c) {
					Some(w) => {
						let l = self.layout(c);
						let put = self.put(ty, "v$", "o$", true);
						lines.push(format!(
							"const o$ = rt.alloc({}, {});\n{put}\nreturn rt.dv().get{}(o$, true);",
							l.size.max(8),
							l.align.max(4),
							Self::word(w).0
						));
					}
					None => lines.push(self.put(ty, "v$", "r$", true)),
				}
			}
		}
	}

	/// The wasm parameters of a C function JavaScript implements taking `params` after its
	/// data and returning `ret`: `r$` first for a struct it returns through a pointer, then
	/// `data`, then `p0` and on.
	fn imported(&self, params: &[Ty], ret: &Ty) -> Vec<String> {
		let mut names = Vec::new();
		if !Self::plain(ret) && self.scalar(ret).is_none() {
			names.push("r$".to_string());
		}
		names.push("data".to_string());
		names.extend((0..params.len()).map(|n| format!("p{n}")));
		names
	}

	/// The wasm type of what a C function JavaScript implements returns, as TypeScript.
	fn wasm_ret(&self, ret: &Ty) -> &'static str {
		match ret {
			Ty::Void => "void",
			ret => match self.scalar(ret) {
				Some(Wasm::I64) => "bigint",
				Some(_) => "number",
				None => "void",
			},
		}
	}

	/// The wasm type of a parameter of a C function JavaScript implements, as TypeScript.
	fn wasm_param(&self, ty: &Ty) -> &'static str {
		match self.scalar(ty) {
			Some(Wasm::I64) => "bigint",
			_ => "number",
		}
	}

	/// The Rust glue's declaration of `name`, a C function the module imports from `rust2`
	/// taking `params` after its data and returning `ret`.
	fn import_decl(&mut self, name: &str, params: &[Ty], ret: &Ty) {
		let decls: String = (params.iter().enumerate())
			.map(|(n, ty)| format!(", p{n}: {}", rs(ty)))
			.collect();
		let _ = write!(
			self.rust,
			"#[link(wasm_import_module = \"rust2\")]\nunsafe extern \"C\" {{\n\tfn {name}(data: *mut c_void{decls}) -> {};\n}}\n\n",
			rs(ret)
		);
	}

	/// The glue's function giving the table index of the C function the Rust glue's `export`
	/// returns, read once.
	fn index_fn(&mut self, local: &str, export: &str) -> String {
		if !self.helpers.contains_key(local) {
			self.helpers.insert(
				local.to_string(),
				format!(
					"let {local}$ = 0;\nexport function {local}(): number {{\n\tif ({local}$ === 0) {local}$ = rt.w.{export}();\n\treturn {local}$;\n}}\n\n"
				),
			);
		}
		format!("g.{local}")
	}

	/// The glue's function giving the table of the trait `i`, whose methods JavaScript
	/// implements, made once with the module's imports.
	fn vtable(&mut self, i: usize) -> String {
		let abi = self.abi;
		let c = abi.callback(i);
		let bare = self.bare(&c).to_string();
		let local = format!("vt_{bare}");
		if self.helpers.contains_key(&local) {
			return format!("g.{local}");
		}
		let space = self.space.clone();
		let callback = &abi.api.callbacks[i];
		let rust = callback.path.join("::");
		let class = self.class(&c);
		let mut fields = vec!["free: Some(runtime::FREE)".to_string()];
		for method in &callback.methods {
			let m = method.name();
			let name = format!("{space}_cb_{bare}_{m}");
			let params: Vec<Ty> = method.params.iter().map(|p| abi.ty(&p.ty)).collect();
			let ret = abi.ty(&method.ok);
			self.import_decl(&name, &params, &ret);
			fields.push(format!("{m}: Some({name})"));
			// The import, which calls the implementation the token names.
			let module = std::mem::replace(&mut self.module, "glue".to_string());
			let t = self.named(class.clone());
			let wasm = self.imported(&params, &ret);
			let mut lines = Vec::new();
			let mut lent = Vec::new();
			let mut args = Vec::new();
			for (n, (param, c)) in method.params.iter().zip(&params).enumerate() {
				args.push(self.arg_in(&param.ty, c, &format!("p{n}"), &mut lines, &mut lent));
			}
			let member = method_name(m);
			let call = format!("rt.find<{t}>(data).{member}({})", args.join(", "));
			let mut body = Vec::new();
			self.outgoing(&method.ok, &ret, &call, &mut body);
			self.module = module;
			let typed: Vec<String> = (wasm.iter())
				.map(|p| match p.strip_prefix('p').and_then(|n| n.parse::<usize>().ok()) {
					Some(n) => format!("{p}: {}", self.wasm_param(&params[n])),
					None => format!("{p}: number"),
				})
				.collect();
			let code = closed(&lines, &body, &lent);
			self.imports.insert(
				name,
				format!(
					"({}): {} =>\n\t\trt.out(\"{rust}::{m}\", () => {{\n{}\n\t\t}})",
					typed.join(", "),
					self.wasm_ret(&ret),
					indent(&indent(&indent(&code)))
				),
			);
		}
		let _ = write!(
			self.rust,
			"static {}: crate::{c}_vtable = crate::{c}_vtable {{\n\t{},\n}};\n\n#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_vtable_{bare}() -> *const crate::{c}_vtable {{\n\t&{}\n}}\n\n",
			format!("VTABLE_{bare}").to_uppercase(),
			fields.join(",\n\t"),
			format!("VTABLE_{bare}").to_uppercase(),
		);
		let export = format!("{space}_vtable_{bare}");
		self.index_fn(&local, &export)
	}

	/// The value a call into JavaScript passes its implementation for the parameter `e` of
	/// type `ty` and C type `c`: an object Rust lends for the call is closed after it.
	fn arg_in(&mut self, ty: &Type, c: &Ty, e: &str, lines: &mut Vec<String>, lent: &mut Vec<String>) -> String {
		if let Type::Ref(_, inner) = ty
			&& let Type::Object(i) = **inner
		{
			let kind = self.kind(i);
			let t = self.tt(inner);
			let l = format!("l{}", lent.len());
			lines.push(format!("const {l} = rt.lentHandle({e}, {kind}) as {t};"));
			lent.push(l.clone());
			return l;
		}
		self.incoming(ty, c, e, lines)
	}

	/// The trampoline of the closure type `ty`, `g.t<n>`, and the glue's function giving the
	/// table index of the C function of its signature, which calls it.
	fn closure(&mut self, ty: &Type, closure: &Closure) -> (String, String) {
		let abi = self.abi;
		let space = self.space.clone();
		let sig = abi.closure(closure);
		let Ty::Fn(c_params, c_ret) = &sig else {
			unreachable!("a closure is a function")
		};
		let (c_params, c_ret) = (c_params.clone(), (**c_ret).clone());
		let wasm = self.imported(&c_params, &c_ret);
		let k = match self.relays.iter().position(|r| *r == sig) {
			Some(k) => k,
			None => {
				self.relays.push(sig.clone());
				let k = self.relays.len() - 1;
				let name = format!("{space}_i_fn_{k}");
				self.import_decl(&name, &c_params, &c_ret);
				let fn_ty = format!(
					"unsafe extern \"C\" fn(*mut c_void{}) -> {}",
					c_params.iter().map(|ty| format!(", {}", rs(ty))).collect::<String>(),
					rs(&c_ret)
				);
				let _ = write!(
					self.rust,
					"#[unsafe(no_mangle)]\npub extern \"C\" fn {space}_fn_{k}() -> usize {{\n\t{name} as {fn_ty} as usize\n}}\n\n"
				);
				// The import, which calls the trampoline the token names with the closure.
				let typed: Vec<String> = (wasm.iter())
					.map(|p| match p.strip_prefix('p').and_then(|n| n.parse::<usize>().ok()) {
						Some(n) => format!("{p}: {}", self.wasm_param(&c_params[n])),
						None => format!("{p}: number"),
					})
					.collect();
				let rest: Vec<&String> = wasm.iter().filter(|p| *p != "data").collect();
				let rest: String = rest.iter().map(|p| format!(", {p}")).collect();
				self.imports.insert(
					name,
					format!(
						"({}): {} => {{\n\t\tconst [t, f] = rt.find<[(...a: any[]) => any, unknown]>(data);\n\t\treturn t(f{rest});\n\t}}",
						typed.join(", "),
						self.wasm_ret(&c_ret)
					),
				);
				k
			}
		};
		let relay = self.index_fn(&format!("f{k}"), &format!("{space}_fn_{k}"));
		// The trampoline, per closure type, since two of one C signature may lift apart.
		let t = match self.trampolines.iter().position(|t| t == ty) {
			Some(t) => t,
			None => {
				self.trampolines.push(ty.clone());
				let t = self.trampolines.len() - 1;
				let module = std::mem::replace(&mut self.module, "glue".to_string());
				let tt = self.tt(ty);
				let mut lines = Vec::new();
				let mut lent = Vec::new();
				let mut args = Vec::new();
				for (n, (param, c)) in closure.params.iter().zip(&c_params).enumerate() {
					args.push(self.arg_in(param, c, &format!("p{n}"), &mut lines, &mut lent));
				}
				let call = format!("f({})", args.join(", "));
				let mut body = Vec::new();
				match &closure.err {
					None => self.outgoing(&closure.ok, &c_ret, &call, &mut body),
					// Its value goes to the out before last, and its error to the last.
					Some(err) => {
						let at = closure.params.len();
						body.push(format!("const v$ = {call};"));
						let mut ok = String::new();
						if closure.ok != Type::Unit {
							ok = format!("{}\n", self.put(&closure.ok, "v$.value", &format!("p{at}"), true));
						}
						let err_at = c_params.len() - 1;
						let error = self.put(err, "v$.error", &format!("p{err_at}"), true);
						let ok = indent(&format!("{ok}return 1;"));
						body.push(format!("if (v$.ok) {{\n{ok}\n}}\n{error}\nreturn 0;"));
					}
				}
				self.module = module;
				let typed: Vec<String> = std::iter::once(format!("f: {tt}"))
					.chain(wasm.iter().filter(|p| *p != "data").map(|p| {
						match p.strip_prefix('p').and_then(|n| n.parse::<usize>().ok()) {
							Some(n) => format!("{p}: {}", self.wasm_param(&c_params[n])),
							None => format!("{p}: number"),
						}
					}))
					.collect();
				let ret = match &closure.err {
					Some(_) => "number",
					None => self.wasm_ret(&c_ret),
				};
				let code = closed(&lines, &body, &lent);
				self.helpers.insert(
					format!("t{t}"),
					format!(
						"export function t{t}({}): {ret} {{\n\treturn rt.out(\"a closure\", () => {{\n{}\n\t}});\n}}\n\n",
						typed.join(", "),
						indent(&indent(&code))
					),
				);
				t
			}
		};
		(relay, format!("g.t{t}"))
	}
}

/// Statements running `lines`, then `body`, closing each handle of `lent` after `body`.
fn closed(lines: &[String], body: &[String], lent: &[String]) -> String {
	let mut code: Vec<String> = lines.to_vec();
	match lent.is_empty() {
		true => code.extend(body.iter().cloned()),
		false => {
			let closes: Vec<String> = lent.iter().map(|l| format!("{l}[Symbol.dispose]();")).collect();
			code.push(format!(
				"try {{\n{}\n}} finally {{\n\t{}\n}}",
				indent(&body.join("\n")),
				closes.join("\n\t")
			));
		}
	}
	code.join("\n")
}

/// A constant's local name in `index.ts`: its own, but `URL`'s, which would shadow the global
/// `load` takes the module's URL by, so it is exported from `URL$`. Bundlers find the
/// module by `new URL(...)` written so, which `globalThis.URL` would hide from them.
fn constant_local(name: &str) -> String {
	match name {
		"URL" => format!("{name}$"),
		_ => name.to_string(),
	}
}

/// A trait method's name in camel case, with `$` appended if a reserved word or `close`.
fn method_name(name: &str) -> String {
	let name = camel(name);
	// `close` is the hook Rust calls once it drops an implementation.
	match RESERVED.contains(&name.as_str()) || name == "close" {
		true => format!("{name}$"),
		false => name,
	}
}

impl Gen<'_> {
	fn items(&mut self) {
		let abi = self.abi;
		let api = abi.api;
		self.names.insert("load".to_string(), "the binding's load".to_string());
		self.module = "index".to_string();
		for constant in &api.constants {
			if !self.here(constant.on) {
				continue;
			}
			let name = abi::snake(self.bare(&abi.name(&constant.path))).to_uppercase();
			let name = abi::claim(
				&mut self.names,
				|n| RESERVED.contains(&n),
				&name,
				format!("const `{}`", constant.path.join("::")),
			);
			self.locked
				.insert(format!("const {}", constant.path.join("::")), name.clone());
			let value = match constant.value {
				Literal::Bool(b) => b.to_string(),
				// Past a safe integer, the type's constructor of its halves, by the type and
				// not the sign: `i64::MAX` is an `rt.i64`.
				Literal::Int(int @ (Int::U64 | Int::I64), v) if v.unsigned_abs() > (1 << 53) - 1 => {
					let bits = v as u64;
					match int {
						Int::I64 => format!("rt.i64Of({}, {})", (bits >> 32) as u32 as i32, bits as u32),
						_ => format!("rt.u64Of({}, {})", bits >> 32, bits as u32),
					}
				}
				Literal::Int(_, v) => v.to_string(),
			};
			let ty = match constant.value {
				Literal::Bool(_) => "boolean",
				Literal::Int(Int::U64, _) => "rt.u64",
				Literal::Int(Int::I64, _) => "rt.i64",
				Literal::Int(..) => "number",
			};
			let local = constant_local(&name);
			if local != name {
				self.out().renames.insert(local.clone(), name.clone());
			}
			let code = format!(
				"{}export const {local}: {ty} = {value};\n\n",
				docs(&constant.docs, constant.deprecated.as_deref(), "")
			);
			self.out().body.push_str(&code);
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
		self.module = "index".to_string();
		let mut taken = BTreeSet::new();
		for function in &api.functions {
			if let Some(code) = self.function(function, None, None, &mut taken) {
				self.module = "index".to_string();
				self.out().body.push_str(&code);
				self.out().body.push('\n');
			}
		}
	}

	/// Start the module of the type `name`, writing its declaration.
	fn start(&mut self, name: &str, declaration: String) {
		self.module = name.to_string();
		let module = self.out();
		module.exports.insert(name.to_string());
		module.body.push_str(&declaration);
	}

	/// Each method of a type, into its module.
	fn methods(&mut self, methods: &[Function], owner: &Ty, ty: &Type, taken: &mut BTreeSet<String>) {
		let module = self.module.clone();
		for method in methods {
			if let Some(code) = self.function(method, Some(owner), Some(ty), taken) {
				self.module = module.clone();
				self.out().body.push_str(&code);
				self.out().body.push('\n');
			}
		}
		self.module = module;
	}

	fn record(&mut self, i: usize) {
		let abi = self.abi;
		let record = &abi.api.records[i];
		if !self.here(record.on) {
			return;
		}
		let c = abi.record(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&record.path, &record.args);
		let ty = Type::Record(i);
		self.module = name.clone();
		let declaration = match &record.opaque {
			Some(_) => {
				let text = format!(
					"{rust}\n\nIts bytes, which only Rust makes or reads: copy it, but never make one.\n\n{}",
					record.docs
				);
				format!(
					"{}export type {name} = rt.Opaque<\"{name}\">;\n\n",
					docs(&text, record.deprecated.as_deref(), "")
				)
			}
			None => {
				let fields: Vec<String> = (record.fields.iter().zip(field_names(&record.fields)))
					.map(|(f, name)| {
						format!(
							"{}\t{name}: {};\n",
							docs(&f.docs, f.deprecated.as_deref(), "\t"),
							self.tt(&f.ty)
						)
					})
					.collect();
				let text = format!("{rust}\n\n{}", record.docs);
				format!(
					"{}export type {name} = {{\n{}}};\n\n",
					docs(&text, record.deprecated.as_deref(), ""),
					fields.concat()
				)
			}
		};
		self.start(&name, declaration);
		let mut taken = BTreeSet::new();
		self.traits(&ty, &c, record.traits, &mut taken);
		self.methods(&record.methods, &Ty::Struct(c.clone()), &ty, &mut taken);
	}

	fn enumeration(&mut self, i: usize) {
		let abi = self.abi;
		let e = &abi.api.enums[i];
		if !self.here(e.on) {
			return;
		}
		let c = abi.enumeration(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&e.path, &e.args);
		let ty = Type::Enum(i);
		self.module = name.clone();
		let variants: Vec<String> = match e.plain() {
			true => (e.variants.iter())
				.filter(|v| self.here(v.on))
				.map(|v| format!("\"{}\"", v.name))
				.collect(),
			false => {
				let mut cases = Vec::new();
				let here: Vec<_> = e.variants.iter().filter(|v| self.here(v.on)).collect();
				for variant in here {
					let mut fields = vec![format!("tag: \"{}\"", variant.name)];
					for (field, name) in variant.fields.iter().zip(variant_fields(&variant.fields)) {
						fields.push(format!("{name}: {}", self.tt(&field.ty)));
					}
					cases.push(format!("{{ {} }}", fields.join("; ")));
				}
				cases
			}
		};
		let union = match variants.is_empty() {
			true => "never".to_string(),
			false => format!("\n\t| {}", variants.join("\n\t| ")),
		};
		let text = format!("{rust}\n\n{}", e.docs);
		self.start(
			&name,
			format!(
				"{}export type {name} = {union};\n\n",
				docs(&text, e.deprecated.as_deref(), "")
			),
		);
		let mut taken = BTreeSet::new();
		self.traits(&ty, &c, e.traits, &mut taken);
		self.methods(&e.methods, &abi.ty(&ty), &ty, &mut taken);
	}

	fn object(&mut self, i: usize) {
		let abi = self.abi;
		let object = &abi.api.objects[i];
		if !self.here(object.on) {
			return;
		}
		let c = abi.object(i);
		let name = self.class(&c);
		let rust = abi.api.rust(&object.path, &object.args);
		let ty = Type::Object(i);
		self.module = name.clone();
		let text = format!(
			"{rust}\n\nA handle to it: `[Symbol.dispose]` drops it, as does the garbage collector one never disposed.\n\n{}",
			object.docs
		);
		self.start(
			&name,
			format!(
				"{}export type {name} = rt.Handle<\"{name}\">;\n\n",
				docs(&text, object.deprecated.as_deref(), "")
			),
		);
		let owner = Ty::Handle {
			name: c.clone(),
			constant: false,
		};
		let mut taken = BTreeSet::new();
		self.traits(&ty, &c, object.traits, &mut taken);
		self.methods(&object.methods, &owner, &ty, &mut taken);
	}

	/// The TypeScript type of what a function returns: a waker Rust returns is one
	/// JavaScript can wake and drop, unlike the function a parameter takes.
	fn returned_t(&mut self, ty: &Type) -> String {
		match ty {
			Type::Waker => "rt.Waker".to_string(),
			ty if lends(ty) => "rt.Bytes".to_string(),
			Type::Option(inner) if lends(inner) => "rt.Bytes | undefined".to_string(),
			_ => self.tt(ty),
		}
	}

	/// The value of type `ty` a call returns at `p`: bytes as an `rt.Bytes`, which may lend
	/// Rust's buffer, as nothing inside another value does, since only a caller holding the
	/// bytes alone disposes them.
	fn returned(&mut self, ty: &Type, p: &str) -> String {
		match ty {
			ty if lends(ty) => format!("rt.returnedBytes({p})"),
			Type::Option(inner) if lends(inner) => format!("(rt.isNone({p}) ? undefined : rt.returnedBytes({p}))"),
			_ => self.get(ty, p, true),
		}
	}

	/// The TypeScript type of a parameter: bytes may also be an `rt.Bytes` a call returned,
	/// which passes back with no copy.
	fn param_t(&mut self, ty: &Type) -> String {
		match ty {
			ty if lends(ty) => "Uint8Array | rt.Bytes".to_string(),
			Type::Option(inner) if lends(inner.base()) => "Uint8Array | rt.Bytes | undefined".to_string(),
			Type::Box(inner) | Type::Ref(_, inner) => self.param_t(inner),
			_ => self.tt(ty),
		}
	}

	fn callback(&mut self, i: usize) {
		let abi = self.abi;
		let callback = &abi.api.callbacks[i];
		if !self.here(callback.on) {
			return;
		}
		let c = abi.callback(i);
		let name = self.class(&c);
		let rust = callback.path.join("::");
		self.module = name.clone();
		let mut members = Vec::new();
		for method in &callback.methods {
			let member = method_name(method.name());
			crate::lock::function(&mut self.locked, method, &member);
			let params: Vec<String> = (method.params.iter())
				.map(|p| format!("{}: {}", param_name(&p.name), self.tt(&p.ty)))
				.collect();
			let ok = self.tt(&method.ok);
			members.push(format!(
				"{}\t{member}({}): {ok};\n",
				docs(
					&self.function_docs(method, doc::sections(&method.docs).body),
					method.deprecated.as_deref(),
					"\t"
				),
				params.join(", ")
			));
		}
		members.push(
			"\t/** Called once Rust drops this implementation, to free what it holds. */\n\tclose?(): void;\n"
				.to_string(),
		);
		let text = format!("{rust}: implemented in JavaScript.\n\n{}", callback.docs);
		self.start(
			&name,
			format!(
				"{}export interface {name} {{\n{}}}\n\n",
				docs(&text, callback.deprecated.as_deref(), ""),
				members.join("\n")
			),
		);
	}

	/// The functions a type's std traits export, into its module: `Display` is `toString`,
	/// `Debug` `debug`, `Clone` an object's `clone`, `PartialEq` `eq`, with `Hash` `hash`,
	/// and `Default` `default`.
	fn traits(&mut self, ty: &Type, c: &str, t: crate::ir::Traits, taken: &mut BTreeSet<String>) {
		let abi = self.abi;
		let name = self.tt(ty);
		let object = matches!(ty, Type::Object(_));
		let c_ty = abi.ty(ty);
		let recv = match object {
			true => Ty::Handle {
				name: c.to_string(),
				constant: true,
			},
			false => Ty::In(Box::new(c_ty.clone())),
		};
		let status = abi.traits_status_of(ty);
		// The statements passing `e` as the receiver, making `local`.
		let this = |g: &mut Self, e: &str, local: &str| -> String {
			match object {
				true => format!("const {local} = rt.hold({e});"),
				false => {
					let l = g.layout(&c_ty);
					let put = g.put(ty, e, local, false);
					format!("const {local} = rt.alloc({}, {});\n{put}", l.size.max(1), l.align)
				}
			}
		};
		let mut emit = |g: &mut Self, member: &str, doc: &str, params: &str, ret_t: &str, body: Vec<String>| {
			taken.insert(member.to_string());
			let (local, rename) = match RESERVED.contains(&member) {
				true => (format!("{member}$"), true),
				false => (member.to_string(), false),
			};
			let code = format!(
				"/** {doc} */\nexport function {local}({params}): {ret_t} {{\n{}\n}}\n\n",
				indent(&wrap(&body))
			);
			let module = g.out();
			module.body.push_str(&code);
			if rename {
				module.renames.insert(local, member.to_string());
			}
		};
		let text = |g: &mut Self, what: &str| -> Vec<String> {
			let mut lines = vec![this(g, "self", "a$")];
			let ret = g.invoke(
				&format!("{c}_{what}"),
				vec!["a$".to_string()],
				&Ty::Buf,
				status,
				&mut lines,
			);
			let value = g.result(&Type::String, &ret);
			lines.push(format!("return {value};"));
			lines
		};
		if t.display {
			let body = text(self, "to_string");
			emit(
				self,
				"toString",
				"Rust's `Display` form.",
				&format!("self: {name}"),
				"string",
				body,
			);
		}
		if t.debug {
			let body = text(self, "debug");
			emit(
				self,
				"debug",
				"Rust's `Debug` form.",
				&format!("self: {name}"),
				"string",
				body,
			);
		}
		if object && t.clone {
			let mut lines = vec![this(self, "self", "a$")];
			let ret = self.invoke(&format!("{c}_clone"), vec!["a$".to_string()], &recv, status, &mut lines);
			let value = self.result(ty, &ret);
			lines.push(format!("return {value};"));
			emit(
				self,
				"clone",
				"A new handle to a clone.",
				&format!("self: {name}"),
				&name,
				lines,
			);
		}
		let plain = matches!(ty, Type::Enum(i) if abi.api.enums[*i].plain());
		if t.eq && !plain {
			let mut lines = vec![this(self, "a", "a$"), this(self, "b", "b$")];
			let args = vec!["a$".to_string(), "b$".to_string()];
			let ret = self.invoke(&format!("{c}_eq"), args, &Ty::Bool, status, &mut lines);
			let value = self.result(&Type::Bool, &ret);
			lines.push(format!("return {value};"));
			emit(
				self,
				"eq",
				"Rust's `PartialEq`.",
				&format!("a: {name}, b: {name}"),
				"boolean",
				lines,
			);
			if t.hash {
				let mut lines = vec![this(self, "self", "a$")];
				let u64 = Ty::Int(Int::U64);
				let ret = self.invoke(&format!("{c}_hash"), vec!["a$".to_string()], &u64, status, &mut lines);
				let value = self.result(&Type::Int(Int::U64), &ret);
				lines.push(format!("return {value};"));
				emit(
					self,
					"hash",
					"Rust's `Hash`, which values Rust calls equal share.",
					&format!("self: {name}"),
					"rt.u64",
					lines,
				);
			}
		}
		if t.default {
			let ret_c = match object {
				true => Ty::Handle {
					name: c.to_string(),
					constant: false,
				},
				false => c_ty.clone(),
			};
			let mut lines = Vec::new();
			let ret = self.invoke(&format!("{c}_default"), Vec::new(), &ret_c, false, &mut lines);
			let value = self.result(ty, &ret);
			lines.push(format!("return {value};"));
			emit(self, "default", "Rust's `Default`.", "", &name, lines);
		}
	}
}

/// Statements running `lines` in a call's frame, which gives back what it took.
fn wrap(lines: &[String]) -> String {
	format!(
		"const m$ = rt.enter();\ntry {{\n{}\n}} finally {{\n\trt.leave(m$);\n}}",
		indent(&lines.join("\n"))
	)
}

impl Gen<'_> {
	/// `function`'s TypeScript, a function of the module of its type `owner_ty` if it has
	/// one, whose C type is `owner`, named apart from `taken`; none if not on the target.
	fn function(
		&mut self,
		function: &Function,
		owner: Option<&Ty>,
		owner_ty: Option<&Type>,
		taken: &mut BTreeSet<String>,
	) -> Option<String> {
		let abi = self.abi;
		if !self.here(function.on) {
			return None;
		}
		let module = self.module.clone();
		// A value is copied into JavaScript, so its `&mut self` method returns it as the call
		// left it, calling the C function in place on a copy the glue hands back.
		let handed = (owner.zip(owner_ty)).and_then(|(owner, ty)| abi.handed_back(function, owner, ty));
		let (handed, export, call_name) = match handed {
			Some((back, export, handed)) => {
				let name = format!("{}_h_{}", self.space, self.bare(&export.call.name));
				let decls: Vec<String> = (export.call.params.iter().enumerate())
					.map(|(n, (_, ty))| format!("p{n}: {}", rs(ty)))
					.collect();
				let args: Vec<String> = (0..export.call.params.len()).map(|n| format!("p{n}")).collect();
				let _ = write!(
					self.rust,
					"#[unsafe(no_mangle)]\npub unsafe extern \"C\" fn {name}({}) -> {} {{\n\t{}\n}}\n\n",
					decls.join(", "),
					rs(&export.call.ret),
					handed.call(&args)
				);
				(Some(back), export, name)
			}
			None => {
				let export = abi.export(function, owner);
				let name = export.call.name.clone();
				(None, export, name)
			}
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
		let base = match function.access {
			Access::Const => abi::snake(snake).to_uppercase(),
			_ => camel(snake),
		};
		let member = match owner {
			None => abi::claim(
				&mut self.names,
				|_| false,
				&base,
				format!("fn `{}`", function.path.join("::")),
			),
			Some(_) => {
				let mut member = base;
				while taken.contains(&member) {
					member.push('_');
				}
				member
			}
		};
		taken.insert(member.clone());
		crate::lock::function(&mut self.locked, function, &member);
		let local = match RESERVED.contains(&member.as_str()) {
			true => format!("{member}$"),
			false => member.clone(),
		};

		let receiver = function.receiver.filter(|_| owner.is_some());
		let mut params = Vec::new();
		let mut lines = Vec::new();
		let mut gives = Vec::new();
		let mut args = Vec::new();
		let mut c_params = export.call.params.iter();
		if receiver.is_some() {
			let (_, c) = c_params.next().expect("a method has its receiver");
			let ty = owner_ty.expect("a method has its type").clone();
			let t = self.tt(&ty);
			params.push(format!("self: {t}"));
			match c {
				Ty::Handle { .. } => lines.push("const self$ = rt.hold(self);".to_string()),
				Ty::In(inner) => {
					let l = self.layout(inner);
					let put = self.put(&ty, "self", "self$", false);
					lines.push(format!(
						"const self$ = rt.alloc({}, {});\n{put}",
						l.size.max(1),
						l.align
					));
				}
				other => unreachable!("a receiver is {other:?}"),
			}
			args.push("self$".to_string());
		}
		// Distinct from each other, and from the receiver's `self`.
		let receiving = receiver.is_some() as usize;
		let names = (function.params.iter()).map(|param| param_name(&param.name));
		let names = abi::distinct(["self".to_string()].into_iter().take(receiving).chain(names));
		let names = names.into_iter().skip(receiving);
		let mut writes = Vec::new();
		for (k, (param, name)) in function.params.iter().zip(names).enumerate() {
			let (_, c) = c_params.next().expect("each parameter has its C one");
			let a = format!("a{k}$");
			// A value passed for `&mut`, in a place that takes what Rust writes back.
			if param.ty.written() {
				let inner = param.ty.base();
				let t = self.tt(inner);
				params.push(format!("{name}: rt.Place<{t}>"));
				let value = format!("{name}.v");
				lines.push(self.pass(inner, c, &value, &a));
				args.push(a);
				let out = Abi::written(function, &abi::escape(&param.name));
				let read = self.get(inner, &format!("o_{out}"), true);
				writes.push(format!("{value} = {read};"));
				continue;
			}
			let t = self.param_t(&param.ty);
			params.push(format!("{name}: {t}"));
			match (&param.ty, c) {
				(Type::Closure(closure), Ty::Fn(..)) => {
					c_params.next().expect("a closure has its data");
					let (relay, trampoline) = self.closure(&param.ty, closure);
					args.push(format!("{relay}()"));
					if closure.kept {
						// Given to Rust, which frees it through `free`.
						c_params.next().expect("a kept closure has its free");
						gives.push(format!("const {a} = rt.give([{trampoline}, {name}]);"));
						args.push(a);
						args.push("rt.fn.free".to_string());
					} else {
						lines.push(format!("const {a} = rt.lend([{trampoline}, {name}]);"));
						args.push(a);
					}
				}
				(_, Ty::ForeignReader) => {
					lines.push(format!("const {a} = rt.alloc(12, 4);\nrt.readerOf({a}, {name});"));
					args.push(a);
				}
				(Type::Callback(..), c) | (Type::Box(_), c @ Ty::Struct(_))
					if matches!(param.ty.base(), Type::Callback(..)) =>
				{
					let l = self.layout(c);
					let put = self.put(param.ty.base(), &name, &a, false);
					gives.push(format!("const {a} = rt.alloc({}, {});\n{put}", l.size, l.align));
					args.push(a);
				}
				(Type::Waker, _) => {
					gives.push(format!("const {a} = rt.alloc(12, 4);\nrt.wakerOf({a}, {name});"));
					args.push(a);
				}
				(Type::Future, _) => {
					gives.push(format!("const {a} = rt.alloc(12, 4);\nrt.futureOf({a}, {name});"));
					args.push(a);
				}
				(ty, c) => {
					lines.push(self.pass(ty, c, &name, &a));
					// Rust takes `Bytes` by value: lent bytes become a second owner of its
					// buffer once nothing left to lower can throw, so none is given in vain.
					if matches!(ty, Type::Bytes) || matches!(ty, Type::Option(inner) if **inner == Type::Bytes) {
						gives.push(format!("rt.shared({a}, {name});"));
					}
					args.push(a);
				}
			}
		}
		let outs: Vec<(String, Ty)> = c_params.filter(|(_, ty)| !abi::is_status(ty)).cloned().collect();
		let status = export.call.status().is_some();
		lines.extend(gives);

		let ok = strip_refs(&function.ok);
		let err = function.err.as_ref().map(strip_refs);
		let ok_t = self.returned_t(&ok);
		let value_t = match &err {
			Some(err) => {
				let err_t = self.tt(err);
				format!("rt.Result<{}, {err_t}>", self.value(&ok, &ok_t))
			}
			None => ok_t.clone(),
		};
		let ret_t;
		if let Some(abi::Future { start, poll, free, .. }) = &export.future {
			ret_t = format!("rt.Call<{value_t}>");
			// Its outputs, without the thread check a future that is not `Send` polls with.
			let poll_outs: Vec<(String, Ty)> = poll.params[2..]
				.iter()
				.filter(|(_, ty)| !abi::is_status(ty))
				.cloned()
				.collect();
			// The poll, which completes the future.
			let poll_name = format!("p_{}", self.bare(&c_name));
			let mut poll_lines = Vec::new();
			let mut poll_args = vec!["f".to_string(), "w$".to_string()];
			for (n, ty) in &poll_outs {
				poll_lines.push(out_decl(self, n, ty));
				poll_args.push(format!("o_{n}"));
			}
			let ready = match &ok {
				Type::Unit => "undefined".to_string(),
				_ => self.lift_outs(&function.ok, &ok),
			};
			let ready = match &err {
				Some(_) => format!("rt.Ok({ready})"),
				None => ready,
			};
			let failed = err
				.as_ref()
				.map(|err| format!("rt.Err({})", self.get(err, "o_err", true)));
			let ret = self.invoke(
				&poll.name,
				poll_args,
				&poll.ret,
				poll.status().is_some(),
				&mut poll_lines,
			);
			let Ret::Value(r) = ret else {
				unreachable!("a poll returns its state")
			};
			let mut cases = vec![format!("case 1:\n\treturn [true, {ready}];")];
			if let Some(failed) = &failed {
				cases.push(format!("case 2:\n\treturn [true, {failed}];"));
			}
			cases.push("default:\n\treturn [false, undefined as never];".to_string());
			poll_lines.push(format!("switch ({r}) {{\n{}\n}}", indent(&cases.join("\n"))));
			let module_ = std::mem::replace(&mut self.module, "glue".to_string());
			let value_g = match &err {
				Some(err) => {
					let err_t = self.tt(err);
					let ok_t = self.returned_t(&ok);
					format!("rt.Result<{}, {err_t}>", self.value(&ok, &ok_t))
				}
				None => self.returned_t(&ok),
			};
			self.module = module_;
			self.helpers.insert(
				poll_name.clone(),
				format!(
					"export function {poll_name}(f: number, w$: number): [boolean, {value_g}] {{\n{}\n}}\n\n",
					indent(&poll_lines.join("\n"))
				),
			);
			// The call and its first poll in one, which completes the call if ready, or else
			// leaves it the future to poll.
			for (n, ty) in &poll_outs {
				lines.push(out_decl(self, n, ty));
				args.push(format!("o_{n}"));
			}
			lines.push("const of$ = rt.alloc(4, 4);".to_string());
			args.push("of$".to_string());
			let started = self.invoke(&start.name, args, &start.ret, status, &mut lines);
			let Ret::Value(r) = started else {
				unreachable!("a start returns its state")
			};
			let mut cases = vec![format!("case 1:\n\treturn rt.ready({ready});")];
			if let Some(failed) = &failed {
				cases.push(format!("case 2:\n\treturn rt.ready({failed});"));
			}
			cases.push(format!(
				"default:\n\treturn rt.pending(rt.dv().getUint32(of$, true), g.{poll_name}, \"{}\");",
				free.name
			));
			lines.push(format!("switch ({r}) {{\n{}\n}}", indent(&cases.join("\n"))));
		} else {
			ret_t = value_t;
			for (n, ty) in &outs {
				lines.push(out_decl(self, n, ty));
				args.push(format!("o_{n}"));
			}
			let ret = self.invoke(&call_name, args, &export.call.ret, status, &mut lines);
			lines.extend(writes.iter().cloned());
			match &err {
				Some(err) => {
					let Ret::Value(r) = &ret else {
						unreachable!("a call that fails returns whether it did")
					};
					let value = match &ok {
						Type::Unit => "undefined".to_string(),
						_ => self.lift_outs(&function.ok, &ok),
					};
					let error = self.get(err, "o_err", true);
					lines.push(format!("return {r} !== 0 ? rt.Ok({value}) : rt.Err({error});"));
				}
				None if ok == Type::Unit => {}
				None if outs.len() > writes.len() => {
					let value = self.lift_outs(&function.ok, &ok);
					lines.push(format!("return {value};"));
				}
				None => {
					let value = match ok {
						Type::Waker => self.result(&Type::Waker, &ret),
						_ => self.result(&ok, &ret),
					};
					lines.push(format!("return {value};"));
				}
			}
		}
		self.module = module;

		let mut text = doc::sections(&function.docs).body;
		if handed.is_some() {
			text.push_str(
				"\n\nRust changes a copy of this value, which it returns first, as the call left it: this one never changes.",
			);
		}
		for param in &function.params {
			let name = param_name(&param.name);
			if let Type::Closure(closure) = &param.ty {
				text.push_str(&match closure.kept {
					true => format!("\n\nRust keeps `{name}` until it drops it."),
					false => format!("\n\nRust calls `{name}` only during the call."),
				});
				if closure.params.iter().any(|p| matches!(p.base(), Type::Object(_))) {
					text.push_str(&format!(
						"\nAn object it lends lives only for that call of `{name}`: keep none past it."
					));
				}
				if closure.err.is_some() {
					text.push_str("\nIt fails by returning an `Err`; anything it throws aborts.");
				}
			}
		}
		if function.future {
			text.push_str("\n\nCancelling it cancels it in Rust, losing nothing.");
		}
		let ret_t = match ok {
			Type::Unit if err.is_none() && !function.future => "void".to_string(),
			_ => ret_t,
		};
		if local != member {
			self.out().renames.insert(local.clone(), member.clone());
		}
		let text = self.function_docs(function, text);
		Some(format!(
			"{}export function {local}({}): {ret_t} {{\n{}\n}}\n",
			docs(&text, function.deprecated.as_deref(), ""),
			params.join(", "),
			indent(&wrap(&lines))
		))
	}

	/// The value of a call's outs, which return `ok`, or each element of a tuple.
	fn lift_outs(&mut self, declared: &Type, ok: &Type) -> String {
		match declared {
			Type::Tuple(elements) => {
				let lifted: Vec<String> = (elements.iter().enumerate())
					.map(|(n, e)| self.get(&strip_refs(e), &format!("o_out{n}"), true))
					.collect();
				format!("[{}]", lifted.join(", "))
			}
			_ => self.returned(ok, "o_out"),
		}
	}
}

/// The declaration of the local the call writes the out `name` of C type `ty` to.
fn out_decl(g: &Gen<'_>, name: &str, ty: &Ty) -> String {
	let Ty::Out(inner) = ty else {
		unreachable!("an out parameter is a pointer")
	};
	let l = g.layout(inner);
	format!("const o_{name} = rt.alloc({}, {});", l.size.max(1), l.align)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A field or parameter named as a namespace imports nothing; a member read imports it.
	#[test]
	fn imports_follow_member_reads() {
		assert_eq!(
			module_imports("type Rgb = { r: number; g: number };\nfunction f(rt: number) {}"),
			""
		);
		assert_eq!(
			module_imports("return g.f(rt.alloc(4, 4));"),
			"import * as rt from \"./rt.ts\";\nimport * as g from \"./glue.ts\";\n"
		);
		assert!(!uses("x.rt.y", "rt."));
	}

	fn fields(names: &[&str]) -> Vec<crate::ir::Field> {
		(names.iter())
			.map(|name| crate::ir::Field {
				name: name.to_string(),
				docs: String::new(),
				deprecated: None,
				ty: Type::Unit,
			})
			.collect()
	}

	/// A variant's field named `tag` keeps clear of the tag naming the variant, and fields
	/// alike in camel case keep apart.
	#[test]
	fn field_names_distinct() {
		assert_eq!(variant_fields(&fields(&["tag", "tag_name"])), ["tag$", "tagName"]);
		assert_eq!(field_names(&fields(&["tag", "foo", "foo_"])), ["tag", "foo", "foo2"]);
	}

	/// A constant named `URL` keeps clear of the global `load` constructs.
	#[test]
	fn constant_url() {
		assert_eq!(constant_local("URL"), "URL$");
		assert_eq!(constant_local("LIMIT"), "LIMIT");
	}
}
