//! What wasm-bindgen's JS glue passes for each Rust type at an import, and what it reads
//! back, from the value translation holds: numbers as JS numbers, 64- and 128-bit integers
//! as BigInts, strings as JS strings, `Option`'s `None` as `undefined`, byte slices as
//! views of their bytes, other slices and vectors as typed arrays or arrays, string enums as
//! their strings, and JS values as themselves. What comes back is coerced as the
//! WebAssembly ABI coerces it, then read as wasm-bindgen's `from_abi` reads it.

use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use super::super::body::Body;
use super::super::library::Std;
use crate::ts::{Expr, Param, Stmt, StmtKind, Type};

/// A converted argument, with what writes back what JS wrote into its copy after the call.
pub struct Arg {
	pub value: Expr,
	pub after: Vec<Stmt>,
}

/// The typed array JS views a slice of `elem` as, and whether its items are BigInts.
fn typed(elem: Ty<'_>) -> Option<(&'static str, bool)> {
	Some(match elem.kind() {
		ty::Int(ty::IntTy::I8) => ("Int8Array", false),
		ty::Uint(ty::UintTy::U8) => ("Uint8Array", false),
		ty::Int(ty::IntTy::I16) => ("Int16Array", false),
		ty::Uint(ty::UintTy::U16) => ("Uint16Array", false),
		ty::Int(ty::IntTy::I32 | ty::IntTy::Isize) => ("Int32Array", false),
		ty::Uint(ty::UintTy::U32 | ty::UintTy::Usize) => ("Uint32Array", false),
		ty::Int(ty::IntTy::I64) => ("BigInt64Array", true),
		ty::Uint(ty::UintTy::U64) => ("BigUint64Array", true),
		ty::Float(ty::FloatTy::F32) => ("Float32Array", false),
		ty::Float(ty::FloatTy::F64) => ("Float64Array", false),
		_ => return None,
	})
}

/// Why translation refuses an import's `ty`, which its JS glue converts in a way translation
/// doesn't.
fn refuse(ty: Ty<'_>, at: &str) -> String {
	format!("an import's {at} of `{ty}`, which wasm-bindgen converts in a way translation does not")
}

/// The JS value the glue passes for an argument `value` of `ty`.
pub fn arg<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	value: Expr,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Result<Arg, String> {
	if let ty::Ref(_, inner, ty::Mutability::Mut) = ty.kind()
		&& let ty::Slice(elem) = inner.kind()
		&& let Some((class, big)) = typed(*elem)
		&& !super::super::byte(*elem)
	{
		// A copy JS writes into, written back after, as it writes the slice in memory.
		let copy = b.fresh("view");
		let made = Expr::rs(
			"jsTyped",
			vec![global(class), value.clone(), Expr::lit(big.to_string())],
		);
		let back = from_lambda(b, *elem, span)?;
		let after =
			vec![StmtKind::Expr(Expr::rs("jsTypedBack", vec![value, Expr::ident(copy.clone()), back])).at(None)];
		out.push(StmtKind::decl(false, copy.clone(), None, Some(made)).at(None));
		return Ok(Arg {
			value: Expr::ident(copy),
			after,
		});
	}
	Ok(Arg {
		value: to_js(b, ty, value, span)?,
		after: Vec::new(),
	})
}

/// `global.<name>`, a property of the global object.
pub fn global(name: &str) -> Expr {
	Expr::Rs("global".into()).member(name)
}

/// The JS value of `value`, a `ty`.
pub fn to_js<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Result<Expr, String> {
	Ok(match ty.kind() {
		ty::Bool | ty::Float(_) => value,
		ty::Int(i) if !matches!(i, ty::IntTy::I64 | ty::IntTy::I128) => value,
		ty::Uint(u) if !matches!(u, ty::UintTy::U64 | ty::UintTy::U128) => value,
		ty::Int(_) | ty::Uint(_) => Expr::rs("jsBigInt", vec![value]),
		ty::Char => Expr::rs("jsChar", vec![value]),
		ty::Tuple(tys) if tys.is_empty() => value,
		ty::Ref(_, inner, _) => match inner.kind() {
			ty::Str => value,
			ty::Slice(elem) if super::super::byte(*elem) => Expr::rs("byteSliceView", vec![value]),
			// What JS writes into one is written back only where it is the argument itself.
			ty::Slice(_) if matches!(ty.kind(), ty::Ref(_, _, ty::Mutability::Mut)) => {
				return Err(refuse(ty, "argument"));
			}
			ty::Slice(elem) if let Some((class, big)) = typed(*elem) => {
				Expr::rs("jsTyped", vec![global(class), value, Expr::lit(big.to_string())])
			}
			ty::Slice(elem) => {
				let to = to_lambda(b, *elem, span)?;
				Expr::rs("jsArray", vec![value, to])
			}
			_ if represented(b, *inner) => value,
			// A reference to a string enum or a number, which the glue passes as the value.
			_ if matches!(ty.kind(), ty::Ref(_, _, ty::Mutability::Not)) && b.t.is_copy(*inner) => {
				to_js(b, *inner, value, span)?
			}
			_ => return Err(refuse(ty, "argument")),
		},
		ty::Adt(adt, args) => match b.t.std_of(ty) {
			Some(Std::Js) => value,
			Some(Std::String) => Expr::rs("jsString", vec![value]),
			Some(Std::Option) => {
				let inner = args.type_at(0);
				let some = b.fresh("some");
				let none = b.t.none(inner);
				let converted = to_js(b, inner, Expr::ident(some.clone()), span)?;
				// Its value once, then `undefined` for `None`.
				Expr::Call(
					Box::new(Expr::Lambda(
						vec![Param {
							name: some.clone(),
							ty: Some(b.t.ty(ty, span)),
						}],
						Box::new(Expr::Cond(
							Box::new(Expr::ident(some).binary("===", none)),
							Box::new(Expr::undefined()),
							Box::new(converted),
						)),
					)),
					vec![value],
				)
			}
			Some(Std::Vec | Std::Box) => match boxed_slice(b, ty) {
				Some(elem) if super::super::byte(elem) => match b.t.std_of(ty) {
					Some(Std::Vec) => Expr::rs("jsByteVec", vec![value]),
					_ => Expr::rs("byteSliceView", vec![value]).member("slice").call(vec![]),
				},
				Some(elem) if let Some((class, big)) = typed(elem) => Expr::rs(
					"jsTyped",
					vec![global(class), slice_of(b, ty, value), Expr::lit(big.to_string())],
				),
				Some(elem) => {
					let to = to_lambda(b, elem, span)?;
					Expr::rs("jsArray", vec![value, to])
				}
				None => return Err(refuse(ty, "argument")),
			},
			None if let Some(values) = super::lower::string_enum(b, adt.did()) => {
				let names = enum_names(b, adt.did());
				Expr::rs("jsEnum", vec![names, values, value])
			}
			_ => return Err(refuse(ty, "argument")),
		},
		_ => return Err(refuse(ty, "argument")),
	})
}

/// The Rust value of `value`, what an import returned, as a `ty`.
pub fn from_js<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Result<Expr, String> {
	let int = |name: &str| Expr::rs(name, vec![value.clone()]);
	Ok(match ty.kind() {
		ty::Bool => int("jsToBool"),
		ty::Uint(ty::UintTy::U8) => int("jsToU8"),
		ty::Int(ty::IntTy::I8) => int("jsToI8"),
		ty::Uint(ty::UintTy::U16) => int("jsToU16"),
		ty::Int(ty::IntTy::I16) => int("jsToI16"),
		ty::Uint(ty::UintTy::U32) => int("jsToU32"),
		ty::Int(ty::IntTy::I32) => int("jsToI32"),
		ty::Uint(ty::UintTy::Usize) => int("jsToUsize"),
		ty::Int(ty::IntTy::Isize) => int("jsToIsize"),
		ty::Float(ty::FloatTy::F64) => int("jsToF64"),
		ty::Float(ty::FloatTy::F32) => int("jsToF32"),
		ty::Int(i) => {
			let bits = if *i == ty::IntTy::I64 { 64 } else { 128 };
			let wide = Expr::rs("jsToWide", vec![value, Expr::lit(bits.to_string()), Expr::lit("true")]);
			Expr::As(Box::new(wide), b.t.ty(ty, span))
		}
		ty::Uint(u) => {
			let bits = if *u == ty::UintTy::U64 { 64 } else { 128 };
			let wide = Expr::rs("jsToWide", vec![value, Expr::lit(bits.to_string()), Expr::lit("false")]);
			Expr::As(Box::new(wide), b.t.ty(ty, span))
		}
		ty::Char => int("jsToChar"),
		ty::Tuple(tys) if tys.is_empty() => Expr::undefined(),
		ty::Adt(adt, args) => match b.t.std_of(ty) {
			Some(Std::Js) => Expr::As(Box::new(value), b.t.ty(ty, span)),
			Some(Std::String) => int("jsToString"),
			Some(Std::Option) => {
				let inner = args.type_at(0);
				let some = b.fresh("got");
				let none = b.t.none(inner);
				// The JS glue reads an optional `bool` by its truthiness, not through the ABI's `i32`.
				let converted = match inner.kind() {
					ty::Bool => Expr::Unary("!!", Box::new(Expr::ident(some.clone()))),
					_ => from_js(b, inner, Expr::ident(some.clone()), span)?,
				};
				// An optional small integer crosses the ABI's `i32` as it is, its `0xFFFFFF` `None`.
				let (value, is_none) = match small(inner) {
					true => (
						Expr::rs("jsToOptionSmall", vec![value]),
						Expr::ident(some.clone()).binary("===", Expr::undefined()),
					),
					false => (value, Expr::rs("jsIsNone", vec![Expr::ident(some.clone())])),
				};
				let ret = b.t.ty(ty, span);
				Expr::Call(
					Box::new(Expr::Arrow(
						vec![Param {
							name: some.clone(),
							ty: Some(Type::Builtin("unknown")),
						}],
						ret,
						vec![
							StmtKind::Return(Some(Expr::Cond(Box::new(is_none), Box::new(none), Box::new(converted))))
								.at(None),
						],
					)),
					vec![value],
				)
			}
			Some(Std::Vec | Std::Box) => match boxed_slice(b, ty) {
				Some(elem) if super::super::byte(elem) => match b.t.std_of(ty) {
					Some(Std::Vec) => int("jsToByteVec"),
					_ => int("jsToByteSlice"),
				},
				Some(elem) => {
					let from = from_lambda(b, elem, span)?;
					let vec = Expr::rs("jsToVec", vec![value, from]);
					match b.t.std_of(ty) {
						Some(Std::Vec) => vec,
						_ => Expr::rs("vecSlice", vec![vec]),
					}
				}
				None => return Err(refuse(ty, "result")),
			},
			None if let Some(values) = super::lower::string_enum(b, adt.did()) => {
				let names = enum_names(b, adt.did());
				let ts = b.t.ty(ty, span);
				Expr::As(Box::new(Expr::rs("jsToEnum", vec![names, values, value])), ts)
			}
			_ => return Err(refuse(ty, "result")),
		},
		_ => return Err(refuse(ty, "result")),
	})
}

/// Whether the JS glue converts an import's result of `ty`, so that a `catch` import catches
/// what converting it throws, rather than the WebAssembly ABI, after the glue has returned.
pub fn converted_in_glue<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		// A 128-bit integer, which the glue splits into halves, and no other number.
		ty::Bool | ty::Float(_) | ty::Int(_) | ty::Uint(_) => {
			matches!(ty.kind(), ty::Int(ty::IntTy::I128) | ty::Uint(ty::UintTy::U128))
		}
		ty::Tuple(tys) => !tys.is_empty(),
		ty::Adt(_, args) if b.t.std_of(ty) == Some(Std::Option) => !small(args.type_at(0)),
		_ => b.t.std_of(ty) != Some(Std::Js),
	}
}

/// Whether `ty` is an integer the ABI's `i32` carries as an `Option`'s payload: the JS glue
/// passes its value as is, and its `0xFFFFFF` is `None`.
fn small(ty: Ty<'_>) -> bool {
	matches!(
		ty.kind(),
		ty::Int(ty::IntTy::I8 | ty::IntTy::I16) | ty::Uint(ty::UintTy::U8 | ty::UintTy::U16)
	)
}

/// `(v) => <v converted to JS>`, for each item of a slice or vector of `elem`.
fn to_lambda<'tcx>(b: &mut Body<'_, '_, 'tcx>, elem: Ty<'tcx>, span: Span) -> Result<Expr, String> {
	let item = b.fresh("item");
	let ty = b.t.ty(elem, span);
	let body = to_js(b, elem, Expr::ident(item.clone()), span)?;
	Ok(Expr::Lambda(
		vec![Param {
			name: item,
			ty: Some(ty),
		}],
		Box::new(body),
	))
}

/// `(v: unknown) => <v converted from JS>`, for each item an import returns of `elem`.
pub fn from_lambda<'tcx>(b: &mut Body<'_, '_, 'tcx>, elem: Ty<'tcx>, span: Span) -> Result<Expr, String> {
	let item = b.fresh("item");
	let body = from_js(b, elem, Expr::ident(item.clone()), span)?;
	Ok(Expr::Lambda(
		vec![Param {
			name: item,
			ty: Some(Type::Builtin("unknown")),
		}],
		Box::new(body),
	))
}

/// The slice element type of `ty`, a `Vec<T>` or a `Box<[T]>`.
fn boxed_slice<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let ty::Adt(_, args) = ty.kind() else { return None };
	match b.t.std_of(ty)? {
		Std::Vec => Some(args.type_at(0)),
		Std::Box => match args.type_at(0).kind() {
			ty::Slice(elem) => Some(*elem),
			_ => None,
		},
		_ => None,
	}
}

/// The `rs.Slice` of `value`, a `Vec` or a `Box<[T]>` of `ty`.
fn slice_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr) -> Expr {
	match b.t.std_of(ty) {
		Some(Std::Vec) => Expr::rs("vecSlice", vec![value]),
		_ => value,
	}
}

/// Whether `ty` is JS-represented, seen through references.
pub fn represented<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	b.t.std_of(ty.peel_refs()) == Some(Std::Js)
}

/// The names translation holds a string enum's variants by, in order.
fn enum_names(b: &Body<'_, '_, '_>, did: rustc_hir::def_id::DefId) -> Expr {
	let names = (b.t.tcx.adt_def(did).variants().iter())
		.map(|v| Expr::Str(v.name.to_string()))
		.collect();
	Expr::Array(names)
}
