//! A call of a function `#[wasm_bindgen]` made around an import, replaced whole by the JS
//! call the import stands for, as the wasm-bindgen CLI's JS glue makes it, and the other
//! glue of wasm-bindgen that is the identity once its types are JS values.

use std::rc::Rc;

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, Instance, Ty};
use rustc_span::Span;

use super::convert::{self, global};
use super::{Glue, Import, Kind, MethodKind, Module, Op, PLACEHOLDER};
use crate::lower::body::Body;
use crate::lower::library::Std;
use crate::ts::{Expr, Key, Param, Stmt, StmtKind, Type};

/// A call of `instance`, which is wasm-bindgen's `glue`, given `args`.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	instance: Instance<'tcx>,
	glue: Glue<'tcx>,
	mut args: Vec<Expr>,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Expr {
	let tcx = b.t.tcx;
	let path = tcx.def_path_str(instance.def_id());
	let lowered = match glue {
		Glue::Identity => Ok(args.swap_remove(0)),
		Glue::Nothing => Ok(Expr::undefined()),
		Glue::Force => Ok(args.swap_remove(0).member("force").call(Vec::new())),
		Glue::Throw { error } => {
			let value = args.swap_remove(0);
			let thrown = match error {
				true => Expr::New(Box::new(global("Error")), vec![value]),
				false => value,
			};
			Ok(Expr::rs("jsThrow", vec![thrown]))
		}
		Glue::Cast { from, to } => {
			convert::to_js(b, from, args.swap_remove(0), span).and_then(|js| convert::from_js(b, to, js, span))
		}
		Glue::Import(Err(why)) => Err(why),
		Glue::Import(Ok(import)) => lower(b, instance, &import, args, span, out),
		Glue::Closure { ty, owned } => convert::closure(b, ty, args.swap_remove(0), owned, span),
		Glue::Dispose => Ok(Expr::rs("drop", vec![args.swap_remove(0)])),
		Glue::SpawnLocal => Ok(Expr::rs("jsSpawnLocal", vec![args.swap_remove(0)])),
		Glue::Refused(why) => Err(why.into()),
	};
	match lowered {
		Ok(value) => value,
		Err(why) => b.err(span, format!("`{path}`, of wasm-bindgen: {why}")),
	}
}

/// A constant `JsValue` of a slot wasm-bindgen reserves, `idx`: `undefined`, `null`,
/// `true`, or `false`.
pub fn constant(idx: u32) -> Option<Expr> {
	// `JSIDX_OFFSET` and the reserved slots after it, as wasm-bindgen's `__rt` numbers them.
	Some(Expr::lit(match idx.checked_sub(1024)? {
		0 => "undefined",
		1 => "null",
		2 => "true",
		3 => "false",
		_ => return None,
	}))
}

fn lower<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	instance: Instance<'tcx>,
	import: &Rc<Import>,
	args: Vec<Expr>,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Result<Expr, String> {
	let is_intrinsic = match &import.module {
		None => false,
		Some(Module::Raw(module)) if module == PLACEHOLDER => true,
		Some(Module::Raw(module)) => {
			return Err(format!(
				"its JS module `{module}`, custom JS, which translation refuses"
			));
		}
		// The program names it by the crate's name and a hash of where it is, then its path.
		Some(Module::Named(module)) => {
			let file = module.split_once('/').map_or(module.as_str(), |(_, file)| file);
			return Err(format!("its JS file `{file}`, custom JS, which translation refuses"));
		}
		Some(Module::Inline) => return Err("its `inline_js`, custom JS, which translation refuses".into()),
	};
	if let Kind::Function(f) = &import.kind {
		if f.suspending {
			return Err("`suspending`, which suspends the WASM stack on a promise, as no JS call can".into());
		}
		if f.generic_per_mono {
			return Err(
				"`experimental_generic_mono`, which the CLI makes JS of per instance, as translation does not".into(),
			);
		}
		if f.asyncness || f.jspi {
			return Err("an `async` import, whose promise translation does not yet await".into());
		}
	}
	let sig = b.t.sig(instance.def_id(), instance.args);
	let inputs = sig.inputs().to_vec();
	let output = sig.output();
	let (catch, variadic) = match &import.kind {
		Kind::Function(f) => (f.catch, f.variadic),
		_ => (false, false),
	};
	// What the call returns before `catch` wraps it in a `Result`.
	let ret = match catch {
		true => result_ok(b, output).ok_or("`catch` on an import that returns no `Result`")?,
		false => output,
	};
	let mut values = Vec::with_capacity(args.len());
	let mut after = Vec::new();
	let mut borrowed = Vec::new();
	for (ty, value) in inputs.iter().zip(args) {
		let arg = convert::arg(b, *ty, value, span, out)?;
		values.push(arg.value);
		after.extend(arg.after);
		borrowed.extend(arg.borrowed);
	}
	let call = match is_intrinsic {
		true => {
			let Kind::Function(f) = &import.kind else {
				return Err("an intrinsic that is not a function".into());
			};
			intrinsic(&f.name, values)
				.ok_or_else(|| format!("wasm-bindgen's intrinsic `{}`, which translation refuses", f.name))?
		}
		false => shape(b, import, &inputs, values, variadic, output)?,
	};
	// The closures it borrows end as it returns or throws.
	let call = match borrowed.is_empty() {
		true => call,
		false => Expr::rs(
			"jsLending",
			vec![Expr::Array(borrowed), Expr::Lambda(Vec::new(), Box::new(call))],
		),
	};
	let value = match catch {
		true => {
			let ty::Adt(_, result_args) = output.kind() else {
				unreachable!("a `Result`")
			};
			let err = result_args.type_at(1);
			if !convert::represented(b, err) {
				return Err(format!("`catch` into `{err}`, which is not a JS value"));
			}
			let from = convert::from_lambda(b, ret, span)?;
			let glue = Expr::lit(convert::converted_in_glue(b, ret).to_string());
			let caught = Expr::rs("jsCatch", vec![Expr::Lambda(Vec::new(), Box::new(call)), from, glue]);
			let ts = b.t.ty(output, span);
			Expr::As(Box::new(caught), ts)
		}
		false if ret.is_unit() || ret.is_never() => {
			out.push(StmtKind::Expr(call).at(None));
			out.extend(after);
			return Ok(Expr::undefined());
		}
		false if after.is_empty() => convert::from_js(b, ret, call, span)?,
		false => {
			let got = b.fresh("got");
			out.push(StmtKind::decl(false, got.clone(), None, Some(call)).at(None));
			out.extend(after);
			return convert::from_js(b, ret, Expr::ident(got), span);
		}
	};
	if !after.is_empty() {
		let got = b.fresh("got");
		out.push(StmtKind::decl(false, got.clone(), None, Some(value)).at(None));
		out.extend(after);
		return Ok(Expr::ident(got));
	}
	Ok(value)
}

/// The `Ok` type of `ty`, a `Result`.
fn result_ok<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match (b.t.std_of(ty), ty.kind()) {
		(Some(Std::Result), ty::Adt(_, args)) => Some(args.type_at(0)),
		_ => None,
	}
}

/// The JS call of `import` with `values`, as the CLI's glue makes it.
fn shape<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	import: &Import,
	inputs: &[Ty<'tcx>],
	mut values: Vec<Expr>,
	variadic: bool,
	output: Ty<'tcx>,
) -> Result<Expr, String> {
	let _ = inputs;
	Ok(match &import.kind {
		Kind::String(string) => Expr::Str(string.clone()),
		Kind::Static { name } => {
			let optional = b.t.std_of(output) == Some(Std::Option);
			let path = resolve(import, name)?;
			match optional {
				true => Expr::rs("jsOptionalPath", path_parts(&path)),
				false => path.expr(),
			}
		}
		Kind::Instanceof { class } => {
			let class = resolve(import, class)?.expr();
			let value = values.pop().ok_or("an `instanceof` check of nothing")?;
			Expr::rs("jsInstanceof", vec![value, Expr::Lambda(Vec::new(), Box::new(class))])
		}
		Kind::Function(f) => {
			let Some(method) = &f.method else {
				let callee = resolve(import, &f.name)?.expr();
				return Ok(apply(callee, None, values, variadic));
			};
			let class = || resolve(import, &method.class).map(|c| c.expr());
			match &method.kind {
				MethodKind::Constructor => match variadic {
					true => {
						let (last, rest) = split_last(values)?;
						global("Reflect")
							.member("construct")
							.call(vec![class()?, spread(rest, last)])
					}
					false => Expr::New(Box::new(class()?), values),
				},
				MethodKind::Operation { is_static, op } => {
					let is_static = *is_static;
					let receiver = |values: &mut Vec<Expr>| match values.is_empty() {
						true => Err("a method without its receiver".to_string()),
						false => Ok(js_any(values.remove(0))),
					};
					match op {
						Op::RegularThis => return Err("`this` of an export, on an import".into()),
						Op::Regular if is_static => apply(class()?, Some(&f.name), values, variadic),
						Op::Regular if f.structural => {
							let this = receiver(&mut values)?;
							apply(this, Some(&f.name), values, variadic)
						}
						Op::Regular => {
							let method = js_any(path(class()?.member("prototype"), &f.name));
							let this = receiver(&mut values)?;
							match variadic {
								true => {
									let (last, rest) = split_last(values)?;
									method.member("apply").call(vec![this, spread(rest, last)])
								}
								false => method.member("call").call([vec![this], values].concat()),
							}
						}
						Op::Getter(field) if f.structural => match is_static {
							true => path(class()?, field),
							false => path(receiver(&mut values)?, field),
						},
						Op::Setter(field) if f.structural => {
							let value = values.pop().ok_or("a setter without its value")?;
							let target = match is_static {
								true => path(class()?, field),
								false => path(receiver(&mut values)?, field),
							};
							Expr::Assign(Box::new(target), Box::new(value))
						}
						Op::Getter(field) | Op::Setter(field) => {
							let anchor = match is_static {
								true => class()?,
								false => class()?.member("prototype"),
							};
							let which = if matches!(op, Op::Getter(_)) { "get" } else { "set" };
							let descriptor = Expr::rs("jsDescriptor", vec![anchor, Expr::Str(field.clone())]);
							js_any(descriptor.member(which)).member("call").call(values)
						}
						Op::IndexingGetter | Op::IndexingSetter | Op::IndexingDeleter if !f.structural => {
							return Err("a non-structural indexing operation, which the CLI refuses".into());
						}
						Op::IndexingGetter => {
							let target = match is_static {
								true => class()?,
								false => receiver(&mut values)?,
							};
							let index = values.pop().ok_or("an index of nothing")?;
							target.index(index)
						}
						Op::IndexingSetter => {
							let target = match is_static {
								true => class()?,
								false => receiver(&mut values)?,
							};
							let value = values.pop().ok_or("an indexing setter without its value")?;
							let index = values.pop().ok_or("an index of nothing")?;
							Expr::Assign(Box::new(target.index(index)), Box::new(value))
						}
						Op::IndexingDeleter => {
							let target = match is_static {
								true => class()?,
								false => receiver(&mut values)?,
							};
							let index = values.pop().ok_or("an index of nothing")?;
							Expr::Unary("delete ", Box::new(target.index(index)))
						}
					}
				}
			}
		}
	})
}

/// `f(...)` or `obj.name(...)`, its last value spread if `variadic`.
fn apply(target: Expr, name: Option<&str>, values: Vec<Expr>, variadic: bool) -> Expr {
	let target = js_any(target);
	let callee = match name {
		Some(name) => path(target, name),
		None => target,
	};
	if !variadic {
		return callee.call(values);
	}
	let Ok((last, rest)) = split_last(values) else {
		return callee.call(Vec::new());
	};
	// As a spread call: what it is a method of read once, the function read from it, and
	// then the arguments spread, a namespace's function called on its namespace.
	match callee {
		Expr::Member(base, Key::Name(key)) => {
			Expr::rs("jsSpreadMethod", vec![*base, Expr::Str(key), Expr::Array(rest), last])
		}
		Expr::Index(base, key) => Expr::rs("jsSpreadMethod", vec![*base, *key, Expr::Array(rest), last]),
		callee => Expr::rs("jsSpreadCall", vec![callee, Expr::undefined(), Expr::Array(rest), last]),
	}
}

fn split_last(mut values: Vec<Expr>) -> Result<(Expr, Vec<Expr>), String> {
	let last = values.pop().ok_or("a variadic import without arguments")?;
	Ok((last, values))
}

/// `[...rest, ...last]`, the arguments of a variadic call: `last` by its iterator, as a
/// spread reads it, which throws where it has none.
fn spread(rest: Vec<Expr>, last: Expr) -> Expr {
	let last = Expr::rs("jsSpread", vec![last]);
	match rest.is_empty() {
		true => last,
		false => Expr::Array(rest).member("concat").call(vec![last]),
	}
}

/// `value` as TypeScript's `any`, which a JS value's property is read from: as it is where
/// it is already a global's, of `rs.global`, or `rs.jsAny`'s.
fn js_any(value: Expr) -> Expr {
	fn typed_any(e: &Expr) -> bool {
		match e {
			Expr::Rs(name) => name == "global",
			Expr::Member(base, _) | Expr::Index(base, _) => typed_any(base),
			Expr::Call(f, _) => {
				matches!(&**f, Expr::Rs(name) if ["jsAny", "jsGlobal", "jsVendored"].contains(&name.as_str()))
			}
			_ => false,
		}
	}
	match typed_any(&value) {
		true => value,
		false => Expr::rs("jsAny", vec![value]),
	}
}

/// `base` then each of `name`'s properties, as the CLI reads a dotted JS name.
fn path(base: Expr, name: &str) -> Expr {
	if let Some(inner) = name.strip_prefix('[').and_then(|n| n.strip_suffix(']')) {
		let mut key = inner.split('.');
		let mut index = global(key.next().unwrap_or_default());
		for part in key {
			index = index.member(part);
		}
		return base.index(index);
	}
	name.split('.')
		.filter(|part| !part.is_empty())
		.fold(base, |base, part| match is_ident(part) {
			true => base.member(part),
			false => base.index(Expr::Str(part.into())),
		})
}

fn is_ident(name: &str) -> bool {
	let mut chars = name.chars();
	chars
		.next()
		.is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
		&& chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// Where a global an import names is: under the global object by its namespace, or the
/// first of its vendor-prefixed names that is defined.
enum Path {
	Vendored(String, Vec<String>),
	Global(Vec<String>),
}

impl Path {
	fn expr(&self) -> Expr {
		match self {
			Path::Vendored(name, prefixes) => Expr::rs(
				"jsVendored",
				vec![
					Expr::Str(name.clone()),
					Expr::Array(prefixes.iter().map(|p| Expr::Str(p.clone())).collect()),
				],
			),
			Path::Global(parts) => {
				// A global the JS glue names as an identifier, which throws where none is defined.
				let (first, rest) = parts.split_first().expect("a global's name");
				path(Expr::rs("jsGlobal", vec![Expr::Str(first.clone())]), &rest.join("."))
			}
		}
	}
}

/// `rs.jsOptionalPath`'s arguments for `path`: the global's root, then each property.
fn path_parts(path: &Path) -> Vec<Expr> {
	match path {
		Path::Vendored(..) => vec![path.expr()],
		Path::Global(parts) => parts.iter().map(|p| Expr::Str(p.clone())).collect(),
	}
}

/// Where `item` is, as the CLI's `determine_import` resolves it.
fn resolve(import: &Import, item: &str) -> Result<Path, String> {
	if !import.vendor_prefixes.is_empty() {
		if import.namespace.is_some() {
			return Err("vendor prefixes through a namespace, which the CLI refuses".into());
		}
		return Ok(Path::Vendored(item.into(), import.vendor_prefixes.clone()));
	}
	let mut parts = import.namespace.clone().unwrap_or_default();
	parts.extend(item.split('.').map(str::to_string));
	match parts.is_empty() || parts.iter().any(String::is_empty) {
		true => Err(format!("the JS name `{item}`, which names no global")),
		false => Ok(Path::Global(parts)),
	}
}

/// The JS body of wasm-bindgen's intrinsic `name` of `args`, as the CLI gives it, if
/// translation has one: what reads the WASM instance, such as its memory, has none.
fn intrinsic(name: &str, mut args: Vec<Expr>) -> Option<Expr> {
	let any = |e: Expr| Expr::As(Box::new(e), Type::Builtin("any"));
	let one = |args: &mut Vec<Expr>| args.pop();
	let typeof_is = |v: Expr, t: &str| Expr::Unary("typeof ", Box::new(v)).binary("===", Expr::Str(t.into()));
	let binary = |args: &mut Vec<Expr>, op: &'static str| {
		let b = args.pop()?;
		let a = args.pop()?;
		Some(any(a).binary(op, any(b)))
	};
	let got = |v: Expr, t: &str| {
		// The value where it is of type `t`, else `undefined`, for an `Option` to read.
		let checked = Expr::Lambda(
			vec![Param {
				name: "v".into(),
				ty: Some(Type::Builtin("unknown")),
			}],
			Box::new(Expr::Cond(
				Box::new(typeof_is(Expr::ident("v"), t)),
				Box::new(Expr::ident("v")),
				Box::new(Expr::undefined()),
			)),
		);
		checked.call(vec![v])
	};
	Some(match name {
		"__wbindgen_is_null" => one(&mut args)?.binary("===", Expr::lit("null")),
		"__wbindgen_is_undefined" => one(&mut args)?.binary("===", Expr::undefined()),
		"__wbindgen_is_null_or_undefined" => Expr::rs("jsIsNone", vec![one(&mut args)?]),
		"__wbindgen_is_symbol" => typeof_is(one(&mut args)?, "symbol"),
		"__wbindgen_is_object" => Expr::rs("jsIsObject", vec![one(&mut args)?]),
		"__wbindgen_is_function" => typeof_is(one(&mut args)?, "function"),
		"__wbindgen_is_string" => typeof_is(one(&mut args)?, "string"),
		"__wbindgen_is_bigint" => typeof_is(one(&mut args)?, "bigint"),
		"__wbindgen_typeof" => Expr::Unary("typeof ", Box::new(one(&mut args)?)),
		"__wbindgen_in" => binary(&mut args, "in")?,
		"__wbindgen_is_falsy" => Expr::Unary("!", Box::new(one(&mut args)?)),
		"__wbindgen_try_into_number" => Expr::rs("jsTryIntoNumber", vec![one(&mut args)?]),
		"__wbindgen_neg" => Expr::Unary("-", Box::new(any(one(&mut args)?))),
		"__wbindgen_bit_and" => binary(&mut args, "&")?,
		"__wbindgen_bit_or" => binary(&mut args, "|")?,
		"__wbindgen_bit_xor" => binary(&mut args, "^")?,
		"__wbindgen_bit_not" => Expr::Unary("~", Box::new(any(one(&mut args)?))),
		"__wbindgen_shl" => binary(&mut args, "<<")?,
		"__wbindgen_shr" => binary(&mut args, ">>")?,
		"__wbindgen_unsigned_shr" => binary(&mut args, ">>>")?,
		"__wbindgen_add" => binary(&mut args, "+")?,
		"__wbindgen_sub" => binary(&mut args, "-")?,
		"__wbindgen_div" => binary(&mut args, "/")?,
		"__wbindgen_checked_div" => {
			let b = args.pop()?;
			Expr::rs("jsCheckedDiv", vec![args.pop()?, b])
		}
		"__wbindgen_mul" => binary(&mut args, "*")?,
		"__wbindgen_rem" => binary(&mut args, "%")?,
		"__wbindgen_pow" => binary(&mut args, "**")?,
		"__wbindgen_lt" => binary(&mut args, "<")?,
		"__wbindgen_le" => binary(&mut args, "<=")?,
		"__wbindgen_ge" => binary(&mut args, ">=")?,
		"__wbindgen_gt" => binary(&mut args, ">")?,
		"__wbindgen_number_get" => got(one(&mut args)?, "number"),
		"__wbindgen_boolean_get" => got(one(&mut args)?, "boolean"),
		"__wbindgen_string_get" => got(one(&mut args)?, "string"),
		"__wbindgen_bigint_get_as_i64" => got(one(&mut args)?, "bigint"),
		"__wbindgen_string_from_utf8_lossy" => Expr::rs("jsUtf8Lossy", vec![one(&mut args)?]),
		"__wbindgen_debug_string" => Expr::rs("jsDebugString", vec![one(&mut args)?]),
		"__wbindgen_throw" => {
			let error = Expr::New(Box::new(global("Error")), vec![one(&mut args)?]);
			Expr::rs("jsThrow", vec![error])
		}
		"__wbindgen_rethrow" => Expr::rs("jsThrow", vec![one(&mut args)?]),
		"__wbindgen_jsval_eq" => {
			let b = args.pop()?;
			args.pop()?.binary("===", b)
		}
		"__wbindgen_jsval_loose_eq" => binary(&mut args, "==")?,
		"__wbindgen_copy_to_typed_array" => {
			let js = args.pop()?;
			Expr::rs("jsCopyToTypedArray", vec![args.pop()?, js])
		}
		_ => return None,
	})
}

/// The string enum `did` is, as the JS strings of its variants, if a program declares it.
pub fn string_enum(b: &Body<'_, '_, '_>, did: DefId) -> Option<Expr> {
	let tcx = b.t.tcx;
	let adt = tcx.adt_def(did);
	let last = adt.variants().iter().last()?;
	if !adt.is_enum() || last.name.as_str() != "__Invalid" {
		return None;
	}
	let found = b.t.programs().string_enum(tcx, did)?;
	(found.values.len() + 1 == adt.variants().len())
		.then(|| Expr::Array(found.values.iter().map(|v| Expr::Str(v.clone())).collect()))
}
